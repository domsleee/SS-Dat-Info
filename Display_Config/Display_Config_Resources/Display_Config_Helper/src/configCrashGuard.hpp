#pragma once
#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "log.hpp"

// srConfig hash-node unlink crash guard ("Village Hard is very crashy").
//
// Windows crash history shows repeated faults at sr.dll+0x13568 / +0x1357f
// (e.g. 3 crashes in 12 minutes on 2026-08-23, plus 2026-08-22 and a June
// occurrence) - all inside sr.dll's internal hash-node unlink (sr+0x13550,
// the helper right after srConfig::append), reached from srConfig::removeAll
// (level teardown) and srConfig::set (key replacement, e.g. saving a high
// time). The faulting instructions:
//
//     mov eax, [esi+8]        ; node->prev   <- crashes when node is garbage
//     mov [eax+0xc], ecx      ; prev->next = next
//     ...
//     mov [eax+8], edx        ; next->prev   <- crashes when ->next is garbage
//
// i.e. the list is already corrupted (use-after-free / wild write elsewhere)
// and the game dies at save/exit time - eating the run. Until the corruptor
// is found, this guard validates the node and its neighbours with SEH-probed
// reads before letting the original unlink run; a corrupt node is skipped
// (one leaked node vs a hard crash) and logged so we learn the frequency.
namespace configguard {

inline SafetyHookInline g_unlinkHook{};
inline volatile long g_skipped = 0;

using Unlink_t = void(__thiscall*)(void* self, void* node);

inline bool probeReadable(const void* p) {
    if ((uintptr_t)p < 0x10000) return false;
    __try {
        volatile char c = *(const volatile char*)p;
        (void)c;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Node layout (from the disassembly): +0 key/hash-links, +8 prev, +0xC next.
inline bool nodeLooksSane(void* node) {
    if (!probeReadable(node) || !probeReadable((char*)node + 0xC)) return false;
    void* prev = *(void**)((char*)node + 0x8);
    void* next = *(void**)((char*)node + 0xC);
    // prev/next may legitimately be null (head/tail); non-null must be readable
    // at the offsets the unlink writes to (prev+0xC, next+0x8).
    if (prev && !probeReadable((char*)prev + 0xC)) return false;
    if (next && !probeReadable((char*)next + 0x8)) return false;
    return true;
}

inline void __fastcall UnlinkDetour(void* self, void* /*edx*/, void* node) {
    if (!nodeLooksSane(node)) {
        long n = InterlockedIncrement(&g_skipped);
        Log(std::format("ConfigCrashGuard: SKIPPED corrupt unlink #{} (node={:p}) - this would have crashed",
                        n, node));
        return;  // leak the node instead of dying
    }
    g_unlinkHook.thiscall<void>(self, node);
}

inline void DoConfigCrashGuard() {
    auto module = GetModuleHandleA("sr.dll");
    if (!module) {
        Log("ConfigCrashGuard: sr.dll not found");
        return;
    }
    // sr+0x13550 prologue: sub esp,8; push ebx; push esi; mov esi,[esp+14];
    // test esi,esi; push edi; mov edi,ecx; mov [esp+0xc],edi
    std::uint8_t* fn = Memory::PatternScan(module,
        "83 EC 08 53 56 8B 74 24 14 85 F6 57 8B F9 89 7C 24 0C");
    if (!fn) {
        Log("ConfigCrashGuard: pattern not found (sr.dll version mismatch?) - not installed");
        return;
    }
    g_unlinkHook = safetyhook::create_inline((void*)fn, (void*)UnlinkDetour);
    Log(g_unlinkHook
            ? std::format("ConfigCrashGuard: installed at sr.dll+{:x}", fn - (std::uint8_t*)module)
            : "ConfigCrashGuard: hook failed");
}

}  // namespace configguard

inline void DoConfigCrashGuard() { configguard::DoConfigCrashGuard(); }
