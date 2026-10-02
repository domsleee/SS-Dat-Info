#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../safe_read.hpp"
#include <Zydis.h>
#include <algorithm>
#include <sstream>
#include <vector>

// Opt-in research mode: records where every listed indirect call in
// Supreme_Game.dll goes. Enabled only when call_trace_sites.txt sits next to
// TAS_Helper.dll (one site RVA per line, first hex token; '#' lines ignored).
// Writes call_trace.txt beside it every 5 s and at exit:
//   site_rva module target_rva count     (hex RVAs, count = sampled hits)
//
// Each site gets a one-byte INT3 rather than a 5-byte mid-hook: most of these
// calls are 2-3 bytes and are often followed by a branch target (the end of an
// `if (p) p->f();`), so a jmp would break code that jumps into it. A vectored
// handler records the target and resumes in a per-site thunk holding a copy of
// the call plus a jmp back. Registers are untouched and the handler body runs
// between FSAVE and FRSTOR like every other hook (fpu_safe_hook.hpp).
// A trap costs ~7 us under WOW64 and one site alone fired 130k/s (SG+0x24CC,
// the vector constructor iterator), so each site traps at most kBudget times
// per second: it disarms itself at zero and the flush thread re-arms it.
// ponytail: hot polymorphic sites are sampled (kBudget consecutive calls per
// second, at a wall-clock phase); a target used only between samples is missed.
// Promote hot sites to mid-hooks if that matters.
namespace calltrace {

constexpr int kSlots = 16;
constexpr LONG kBudget = 100;
struct Slot {
    volatile LONG target;
    volatile LONG count;
    uint32_t base;  // allocation base of the target's image, 0 if not an image
};
struct Site {
    uint32_t va;
    uint8_t len;
    bool mem;
    int8_t base_reg, index_reg;  // 0..7 = eax ecx edx ebx esp ebp esi edi, -1 none
    uint8_t scale;
    int32_t disp;
    uint8_t* thunk;
    uint8_t orig;              // the call's first byte, restored while disarmed
    volatile LONG budget;      // traps left before disarming
    Slot slots[kSlots];
    volatile LONG overflow;
};

inline Site* g_sites = nullptr;
inline size_t g_count = 0;
inline uint32_t g_sg = 0;
inline std::string g_outPath;
inline volatile LONG64 g_hits = 0;

struct Mod { uint32_t base; char name[64]; };
inline Mod g_mods[128];
inline LONG g_modCount = 0;  // flush side only: no locks or loader calls in the handler

static int8_t RegIndex(ZydisRegister r) {
    if (r >= ZYDIS_REGISTER_EAX && r <= ZYDIS_REGISTER_EDI) return (int8_t)(r - ZYDIS_REGISTER_EAX);
    return -1;
}

static uint32_t ImageBase(uint32_t va) {
    MEMORY_BASIC_INFORMATION mbi{};
    if (!VirtualQuery((void*)(uintptr_t)va, &mbi, sizeof mbi) || mbi.Type != MEM_IMAGE) return 0;
    return (uint32_t)(uintptr_t)mbi.AllocationBase;
}

// Flush side: names each module base once. A module that unloads before the
// next pass (1 s) is written as "?".
static void NoteModules() {
    for (size_t i = 0; i < g_count; i++) {
        for (const Slot& sl : g_sites[i].slots) {
            const uint32_t base = sl.base;
            if (!sl.target || !base) continue;
            bool known = false;
            for (LONG m = 0; m < g_modCount && !known; m++) known = g_mods[m].base == base;
            if (known || g_modCount >= (LONG)std::size(g_mods)) continue;
            Mod& mod = g_mods[g_modCount];
            char path[MAX_PATH] = "?";
            GetModuleFileNameA((HMODULE)(uintptr_t)base, path, MAX_PATH);
            const char* slash = strrchr(path, '\\');
            strncpy_s(mod.name, slash ? slash + 1 : path, _TRUNCATE);
            mod.base = base;
            g_modCount++;
        }
    }
}

static void Record(Site& s, uint32_t t) {
    InterlockedIncrement64(&g_hits);
    for (Slot& sl : s.slots) {
        LONG cur = sl.target;
        if (cur == 0) {
            cur = InterlockedCompareExchange(&sl.target, (LONG)t, 0);
            if (cur == 0) {
                sl.base = ImageBase(t);
                cur = (LONG)t;
            }
        }
        if ((uint32_t)cur == t) { InterlockedIncrement(&sl.count); return; }
    }
    InterlockedIncrement(&s.overflow);
}

// regs: eax ecx edx ebx esp ebp esi edi at the call.
static bool Target(const Site& s, const uint32_t* regs, uint32_t* out) {
    if (!s.mem) { *out = regs[s.base_reg]; return true; }
    uint32_t a = (uint32_t)s.disp;
    if (s.base_reg >= 0) a += regs[s.base_reg];
    if (s.index_reg >= 0) a += regs[s.index_reg] * s.scale;
    return SafeCopy(a, out, 4);
}

static Site* Find(uint32_t va) {
    size_t lo = 0, hi = g_count;
    while (lo < hi) {
        size_t mid = (lo + hi) / 2;
        if (g_sites[mid].va < va) lo = mid + 1; else hi = mid;
    }
    return lo < g_count && g_sites[lo].va == va ? &g_sites[lo] : nullptr;
}

static void Body(CONTEXT* c, Site* s) {
    const uint32_t regs[8] = { c->Eax, c->Ecx, c->Edx, c->Ebx, c->Esp, c->Ebp, c->Esi, c->Edi };
    uint32_t t;
    if (Target(*s, regs, &t)) Record(*s, t);
    if (InterlockedDecrement(&s->budget) == 0) *(volatile uint8_t*)(uintptr_t)s->va = s->orig;
}

static LONG CALLBACK OnException(EXCEPTION_POINTERS* ep) {
    const DWORD code = ep->ExceptionRecord->ExceptionCode;
    if (code != EXCEPTION_BREAKPOINT && code != 0x4000001F /* STATUS_WX86_BREAKPOINT */) {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    const uint32_t at = (uint32_t)(uintptr_t)ep->ExceptionRecord->ExceptionAddress;
    Site* s = Find(at);
    if (!s) return EXCEPTION_CONTINUE_SEARCH;
    // Continuation restores the CONTEXT, not the TEB's last error.
    const DWORD lastError = GetLastError();
    uint8_t fpu[108];
    __asm { fsave [fpu] }
    Body(ep->ContextRecord, s);
    __asm { frstor [fpu] }
    SetLastError(lastError);
    ep->ContextRecord->Eip = (DWORD)(uintptr_t)s->thunk;  // the call, then back to site+len
    return EXCEPTION_CONTINUE_EXECUTION;
}

inline void Flush() {
    if (!g_sites) return;
    NoteModules();
    std::ostringstream out;
    size_t hit = 0;
    for (size_t i = 0; i < g_count; i++) {
        const Site& s = g_sites[i];
        if (s.slots[0].target) hit++;
        for (const Slot& sl : s.slots) {
            if (!sl.target) break;
            const uint32_t t = (uint32_t)sl.target;
            std::string name = "?";
            for (LONG m = 0; m < g_modCount; m++) if (g_mods[m].base == sl.base) name = g_mods[m].name;
            out << std::format("{:x} {} {:x} {}\n", s.va - g_sg, name, t - sl.base, (uint32_t)sl.count);
        }
        if (s.overflow) out << std::format("{:x} OVERFLOW 0 {}\n", s.va - g_sg, (uint32_t)s.overflow);
    }
    const std::string tmp = g_outPath + ".tmp";
    {
        std::ofstream f(tmp, std::ios::binary | std::ios::trunc);
        f << std::format("# sites {} hit {} traps {}\n", g_count, hit, (long long)g_hits) << out.str();
        if (!f) return;
    }
    MoveFileExA(tmp.c_str(), g_outPath.c_str(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH);
}

static DWORD WINAPI FlushThread(LPVOID) {
    for (int n = 1;; n++) {
        Sleep(1000);
        for (size_t i = 0; i < g_count; i++) {
            Site& s = g_sites[i];
            if (s.budget > 0) continue;
            s.budget = kBudget;
            MemoryBarrier();
            *(volatile uint8_t*)(uintptr_t)s.va = 0xCC;
        }
        if (n % 5) NoteModules(); else Flush();
    }
}

inline void Install(const GameAddresses& addr) {
    char dll[MAX_PATH];
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                       (LPCSTR)&Install, &self);
    GetModuleFileNameA(self, dll, MAX_PATH);
    const std::string dir = std::string(dll).substr(0, std::string(dll).find_last_of("\\/") + 1);
    std::ifstream list(dir + "call_trace_sites.txt");
    if (!list) return;  // off: normal TAS use

    g_sg = (uint32_t)(uintptr_t)addr.sg;
    g_outPath = dir + "call_trace.txt";

    // The pristine file image (sections at their RVAs, no relocations): a site
    // already patched by another hook must not be touched.
    char sgPath[MAX_PATH];
    GetModuleFileNameA(addr.sg, sgPath, MAX_PATH);
    HMODULE fileImg = LoadLibraryExA(sgPath, nullptr, LOAD_LIBRARY_AS_IMAGE_RESOURCE | LOAD_LIBRARY_AS_DATAFILE);
    if (!fileImg) { Log("Call trace: cannot map Supreme_Game.dll from disk; off"); return; }
    auto* disk = (uint8_t*)((uintptr_t)fileImg & ~(uintptr_t)3);
    const uint32_t imageSize = ((PIMAGE_NT_HEADERS)(disk + ((PIMAGE_DOS_HEADER)disk)->e_lfanew))
                                   ->OptionalHeader.SizeOfImage;

    std::vector<uint32_t> rvas;
    for (std::string line; std::getline(list, line);) {
        if (line.empty() || line[0] == '#') continue;
        rvas.push_back(std::stoul(line, nullptr, 16));
    }
    std::sort(rvas.begin(), rvas.end());
    rvas.erase(std::unique(rvas.begin(), rvas.end()), rvas.end());

    ZydisDecoder dec;
    ZydisDecoderInit(&dec, ZYDIS_MACHINE_MODE_LEGACY_32, ZYDIS_STACK_WIDTH_32);
    g_sites = new Site[rvas.size()]{};
    auto* thunks = (uint8_t*)VirtualAlloc(nullptr, rvas.size() * 16, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    if (!thunks) { Log("Call trace: VirtualAlloc failed; off"); FreeLibrary(fileImg); return; }
    size_t skipped = 0;
    for (uint32_t rva : rvas) {
        if (rva + 16 > imageSize) { skipped++; continue; }
        uint8_t* live = (uint8_t*)addr.sg + rva;
        ZydisDecodedInstruction ins;
        ZydisDecodedOperand ops[ZYDIS_MAX_OPERAND_COUNT];
        if (!ZYAN_SUCCESS(ZydisDecoderDecodeFull(&dec, live, 15, &ins, ops)) ||
            ins.mnemonic != ZYDIS_MNEMONIC_CALL || std::memcmp(live, disk + rva, ins.length) != 0) {
            Log(std::format("Call trace: skip SG+0x{:X} (not an unpatched call)", rva));
            skipped++;
            continue;
        }
        const ZydisDecodedOperand& op = ops[0];
        Site& s = g_sites[g_count];
        s.va = (uint32_t)(uintptr_t)live;
        s.len = ins.length;
        if (op.type == ZYDIS_OPERAND_TYPE_REGISTER && RegIndex(op.reg.value) >= 0) {
            s.mem = false;
            s.base_reg = RegIndex(op.reg.value);
            s.index_reg = -1;
        } else if (op.type == ZYDIS_OPERAND_TYPE_MEMORY && op.mem.segment != ZYDIS_REGISTER_FS &&
                   (op.mem.base == ZYDIS_REGISTER_NONE || RegIndex(op.mem.base) >= 0) &&
                   (op.mem.index == ZYDIS_REGISTER_NONE || RegIndex(op.mem.index) >= 0)) {
            s.mem = true;
            s.base_reg = op.mem.base == ZYDIS_REGISTER_NONE ? -1 : RegIndex(op.mem.base);
            s.index_reg = op.mem.index == ZYDIS_REGISTER_NONE ? -1 : RegIndex(op.mem.index);
            s.scale = op.mem.scale;
            s.disp = op.mem.disp.has_displacement ? (int32_t)op.mem.disp.value : 0;
        } else {
            Log(std::format("Call trace: skip SG+0x{:X} (operand not register-based)", rva));
            skipped++;
            continue;
        }
        // Thunk: the original call (position independent), then jmp site+len.
        s.thunk = thunks + g_count * 16;
        std::memcpy(s.thunk, live, s.len);
        s.thunk[s.len] = 0xE9;
        const int32_t rel = (int32_t)(s.va + s.len - ((uint32_t)(uintptr_t)s.thunk + s.len + 5));
        std::memcpy(s.thunk + s.len + 1, &rel, 4);
        g_count++;
    }
    FreeLibrary(fileImg);
    FlushInstructionCache(GetCurrentProcess(), thunks, rvas.size() * 16);

    if (!AddVectoredExceptionHandler(1, &OnException)) { Log("Call trace: no VEH; off"); return; }
    size_t armed = 0;
    for (size_t i = 0; i < g_count; i++) {
        auto* p = (uint8_t*)(uintptr_t)g_sites[i].va;
        DWORD old;
        // Left writable: arming and disarming are then plain byte stores.
        if (!VirtualProtect(p, 1, PAGE_EXECUTE_READWRITE, &old)) continue;
        g_sites[i].orig = *p;
        g_sites[i].budget = kBudget;
        *p = 0xCC;
        armed++;
    }
    FlushInstructionCache(GetCurrentProcess(), nullptr, 0);
    CloseHandle(CreateThread(nullptr, 0, &FlushThread, nullptr, 0, nullptr));
    Log(std::format("Call trace: armed {} of {} sites ({} skipped), writing {}", armed, rvas.size(),
                    skipped, g_outPath));
}

}  // namespace calltrace
