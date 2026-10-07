#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "safe_read.hpp"
#include "fpu_safe_hook.hpp"
#include "renderer_info.hpp"
#include "ssb_hook.hpp"
#include "trace_provenance.hpp"
#include "census.hpp"
#include "readset.hpp"
#include <intrin.h>
#include <psapi.h>
#include <cstdio>
#include <map>
#include <vector>
#pragma comment(lib, "version.lib")

// Per-tick state trace for the bit-exact rewrite (ssb-recomp re/harness.md;
// the reader, checker and diff tool are ssb-recomp crates/ssb-trace). Opt-in:
// on only when ssb_trace_regions.txt sits next to TAS_Helper.dll, and then
// ssb_trace_env.txt must too (the expected FPU environment, optional forced
// rand seed). Every PLAY arm (a CONT prefix too) writes
// traces\<unix ms>-g<arm generation>.ssbtrace beside it:
//   "SSBTRACE 3\n", u32 header length, header text ("key value" lines, then a
//   "regions" line and the regions file verbatim), then per PLAY tick one
//   TickRecord, u32 base address per region (0 = unreadable) and every
//   region's bytes in file order, then, once the arm ends cleanly, the late
//   file section ("SSBLATE\n", u32 length, a "file <sha256> <record index>
//   <path>" line per identity first opened after the header) and the trailer.
// Region line: name point chain len [classes...]. point: pre = the cycle cave,
// after PLAY wrote this tick's input and before its physics; post = the human
// Player tick (SG+0x849E0) returning, at SG+0x84C43; entry = the human Body's
// physics step (SG+0x98360) entered. chain: base[>off]...[+off], base
// `player`, `exe` or a module name; each >off follows the pointer at +off, a
// final +off is added. The byte classes after len are read by the diff only.
// With ssb_census.txt too, the pointer census and physics read-set sidecars
// (<stem>.census, <stem>.readset) are written beside each trace; see the
// "census" section below. Without it nothing here changes.
namespace trace {

#pragma pack(push, 1)
struct TickRecord {
    uint32_t pos;               // playback index
    int32_t rec_tick;           // recording index played: pos - live gate + recording gate
    uint16_t cw_pre, cw_post;   // the game's x87 control word, from the FSAVE image
    uint32_t mxcsr_pre, mxcsr_post;
    uint8_t input;              // mask PLAY wrote this tick
    uint8_t others;             // other riders' player ticks this cycle (ghosts, AI)
    uint8_t flags;              // FLAG_*
    uint8_t cheat;              // Cheat_Flags::game_cheat [SG+0x1D5394] at pre
    uint32_t hook_calls;        // ssb-hook detour calls so far (0 without it)
    uint32_t region_ok;         // bit i: region i was readable
    uint32_t player;            // the human Player at pre ([replayObj+0x84])
    uint8_t human_ticks;        // human Player ticks (post hook hits) this cycle
    uint8_t entry_calls;        // SG+0x98360 entries with the human Body this cycle
    uint8_t sse2_math;          // msvcrt __use_sse2_mathfcns at pre (0xFF unreadable)
    uint8_t reserved0;
    uint16_t cw_entry;
    uint16_t reserved1;
    uint32_t mxcsr_entry;
    uint32_t holdrand_pre;      // the game thread's msvcrt rand state at pre
    uint32_t holdrand_post;     // at post
    uint32_t hook_fpu_bad;      // ssb-hook detour entries off the expected FPU environment, so far
    uint32_t srand_calls;       // msvcrt srand calls since injection
};
#pragma pack(pop)
static_assert(sizeof(TickRecord) == 64);
// The Trailer, "SSBTEND\n", u32 records, u32 status (0 = every write
// succeeded), is written by traceprovenance::OpenedFiles::CloseAndPublish.
constexpr uint8_t FLAG_POST = 1;            // the post point was captured
constexpr uint8_t FLAG_ENTRY = 2;           // the entry point was captured
constexpr uint8_t FLAG_PLAYER_FREED = 4;    // the trace's Player was destroyed (SG+0x84160) by now
constexpr uint8_t FLAG_PLAYER_CHANGED = 8;  // [replayObj+0x84] is not the trace's first Player
enum Point : uint8_t { PRE, POST, ENTRY };
constexpr uint32_t MXCSR_CONTROL = 0xFFC0;  // the control bits; the low 6 are sticky flags

struct Region {
    std::string name;
    Point point = PRE;
    std::string base;   // "player", "exe" or a module name
    bool player = false;
    std::vector<uint32_t> derefs;
    uint32_t add = 0;
    uint32_t len = 0;
    uint32_t at = 0;    // byte offset in the tick's region block
    uint32_t base_va = 0;
};

// ssb_trace_env.txt: the environment the corpus was recorded in, passed on to
// ssb-hook. `cw 0x027f`, `mxcsr 0x1f80`, optional `srand <seed>`.
struct Env {
    bool have = false;
    uint16_t cw = 0;
    uint32_t mxcsr = 0;
    bool force_seed = false;
    uint32_t seed = 0;
};
inline Env g_env;

inline TasSharedState* g_state = nullptr;
inline uint8_t* g_sg = nullptr;
inline bool g_on = false;
inline std::vector<Region> g_regions;
inline std::string g_regionsText;
inline std::string g_dir;
inline FILE* g_file = nullptr;
inline std::string g_tracePath, g_pendingPath;
inline uint32_t g_gen = 0, g_triedGen = 0xFFFFFFFFu;
inline std::vector<uint8_t> g_rec;  // TickRecord + bases + regions, being filled
inline bool g_pending = false;      // g_rec holds a pre not yet written
inline uint32_t g_ticks = 0, g_noPost = 0, g_maxOthers = 0;
inline uint32_t g_writeStatus = 0;
inline uint32_t g_tracePlayer = 0;  // the human Player at the trace's first tick
inline volatile bool g_playerFreed = false;
inline volatile uint32_t g_srandCalls = 0;
inline std::vector<std::string> g_srandLog;
static SafetyHookMid g_postHook{}, g_entryHook{}, g_dtorHook{}, g_srandHook{};
static SafetyHookInline g_openA{}, g_openW{};

inline TickRecord& Rec() { return *reinterpret_cast<TickRecord*>(g_rec.data()); }
inline uint32_t* Bases() { return reinterpret_cast<uint32_t*>(g_rec.data() + sizeof(TickRecord)); }
inline uint8_t* Block() { return g_rec.data() + sizeof(TickRecord) + 4 * g_regions.size(); }

static uint32_t ParseNum(const std::string& s) { return (uint32_t)strtoul(s.c_str(), nullptr, 0); }

// ---- msvcrt: the game thread's rand state and the SSE2 math switch --------
// rand (system msvcrt): call _getptd; imul ecx,[eax+14h],343FDh; ...; so
// _holdrand is _getptd()+0x14. _set_SSE2_enable: ... and eax,[x]; mov
// [__use_sse2_mathfcns],eax.
using GetPtdFn = uint8_t*(__cdecl*)();
inline GetPtdFn g_getptd = nullptr;
inline uint32_t g_sse2Flag = 0;
inline uint8_t* g_srand = nullptr;

static bool ResolveMsvcrt() {
    HMODULE m = GetModuleHandleA("msvcrt.dll");
    auto rnd = m ? (uint8_t*)GetProcAddress(m, "rand") : nullptr;
    auto sse = m ? (uint8_t*)GetProcAddress(m, "_set_SSE2_enable") : nullptr;
    g_srand = m ? (uint8_t*)GetProcAddress(m, "srand") : nullptr;
    static constexpr uint8_t kRand[] = {0x69, 0x48, 0x14, 0xFD, 0x43, 0x03, 0x00};
    static constexpr uint8_t kSse[] = {0x8B, 0xFF, 0x55, 0x8B, 0xEC, 0x8B, 0x45, 0x08,
                                       0xF7, 0xD8, 0x1B, 0xC0, 0x23, 0x05};
    static constexpr uint8_t kSrand[] = {0x8B, 0xFF, 0x55, 0x8B, 0xEC, 0xE8};
    if (!rnd || !sse || !g_srand || rnd[0] != 0xE8 || memcmp(rnd + 5, kRand, sizeof kRand) ||
        memcmp(sse, kSse, sizeof kSse) || sse[18] != 0xA3 || memcmp(g_srand, kSrand, sizeof kSrand)) {
        Log("trace: msvcrt.dll rand/srand/_set_SSE2_enable are not the expected code");
        return false;
    }
    g_getptd = (GetPtdFn)(rnd + 5 + *(int32_t*)(rnd + 1));
    g_sse2Flag = *(uint32_t*)(sse + 19);
    return true;
}

inline uint32_t* HoldRand() { return (uint32_t*)(g_getptd() + 0x14); }

// ---- checked writes --------------------------------------------------------
static void Put(const void* p, size_t n) {
    if (g_file && fwrite(p, 1, n, g_file) != n) {
        if (!g_writeStatus) Log("trace: WRITE FAILED; the trace will be rejected");
        g_writeStatus |= 1;
    }
}

static void WritePending() {
    if (!g_pending || !g_file) return;
    g_pending = false;
    if (!(Rec().flags & FLAG_POST)) g_noPost++;
    Put(g_rec.data(), g_rec.size());
    g_ticks++;
}

// For ssb-hook's panic hook too: the tick it died in becomes the last record.
static void CloseWindow();
static void CloseHw();
static void CloseProbe();
inline void __cdecl Flush() {
    CloseWindow();
    CloseHw();
    CloseProbe();
    WritePending();
    if (g_file && fflush(g_file)) g_writeStatus |= 2;
}

// ---- provenance --------------------------------------------------------------
using traceprovenance::Sha256Bytes;
using traceprovenance::Sha256File;

static std::wstring Widen(const std::string& s) {
    std::wstring w(MAX_PATH * 2, L'\0');
    w.resize(MultiByteToWideChar(CP_ACP, 0, s.c_str(), -1, w.data(), (int)w.size()) - 1);
    return w;
}
static std::string Utf8(const std::wstring& w) {
    std::string s(w.size() * 3 + 1, '\0');
    s.resize(WideCharToMultiByte(CP_UTF8, 0, w.c_str(), -1, s.data(), (int)s.size(), nullptr, nullptr) - 1);
    return s;
}

static std::wstring GameRoot() {
    wchar_t exe[MAX_PATH] = {};
    GetModuleFileNameW(nullptr, exe, MAX_PATH);
    std::wstring r(exe);
    return r.substr(0, r.find_last_of(L"\\/") + 1);
}

// Files the game opened for reading under its folder since injection, with
// the content hash at each open (tuning, terrain, meshes, ...).
inline traceprovenance::OpenedFiles g_opened;
inline thread_local bool t_inOpen = false;

static void NoteOpened(const std::wstring& name) {
    wchar_t full[MAX_PATH * 2];
    const DWORD n = GetFullPathNameW(name.c_str(), MAX_PATH * 2, full, nullptr);
    static const std::wstring root = GameRoot();
    if (!n || n >= MAX_PATH * 2 || n <= root.size() || _wcsnicmp(full, root.c_str(), root.size())) return;
    const std::string rel = Utf8(full + root.size());
    const std::string sha = Sha256File(full);
    // A new identity while a trace is open goes into its late section. `at`:
    // the record being filled (after its PRE), or the record count once the
    // last record is written.
    const uint32_t at = g_ticks;
    if (g_opened.Note(rel, sha, at) && g_file)
        Log("trace: late file at record " + std::to_string(at) + ": " + sha + " " + rel);
}

constexpr DWORD kWriteAccess = GENERIC_WRITE | GENERIC_ALL | FILE_WRITE_DATA | FILE_APPEND_DATA;

// The hash runs under FXSAVE: library code must not leave the caller's x87 or
// SSE state changed, and GetLastError is the caller's.
static void AfterOpen(HANDLE h, const std::wstring& name, DWORD access, DWORD flags) {
    if (h == INVALID_HANDLE_VALUE || (access & kWriteAccess) || (flags & FILE_FLAG_BACKUP_SEMANTICS)) return;
    const DWORD err = GetLastError();
    alignas(16) uint8_t fx[512];
    _fxsave(fx);
    NoteOpened(name);
    _fxrstor(fx);
    SetLastError(err);
}

static HANDLE WINAPI OpenAHook(LPCSTR name, DWORD access, DWORD share, LPSECURITY_ATTRIBUTES sa, DWORD disp,
                               DWORD flags, HANDLE tmpl) {
    const bool outer = !t_inOpen;
    t_inOpen = true;
    HANDLE h = g_openA.stdcall<HANDLE>(name, access, share, sa, disp, flags, tmpl);
    if (outer && name) AfterOpen(h, Widen(name), access, flags);
    if (outer) t_inOpen = false;
    return h;
}

static HANDLE WINAPI OpenWHook(LPCWSTR name, DWORD access, DWORD share, LPSECURITY_ATTRIBUTES sa, DWORD disp,
                               DWORD flags, HANDLE tmpl) {
    const bool outer = !t_inOpen;
    t_inOpen = true;
    HANDLE h = g_openW.stdcall<HANDLE>(name, access, share, sa, disp, flags, tmpl);
    if (outer && name) AfterOpen(h, name, access, flags);
    if (outer) t_inOpen = false;
    return h;
}

static std::string CpuName() {
    int r[4];
    char vendor[13] = {};
    __cpuid(r, 0);
    memcpy(vendor, &r[1], 4);
    memcpy(vendor + 4, &r[3], 4);
    memcpy(vendor + 8, &r[2], 4);
    char brand[49] = {};
    for (int i = 0; i < 3; i++) {
        __cpuid(r, 0x80000002 + i);
        memcpy(brand + 16 * i, r, 16);
    }
    return std::format("{} | {}", vendor, brand);
}

static std::string FileVersion(const wchar_t* path) {
    DWORD h = 0;
    const DWORD n = GetFileVersionInfoSizeW(path, &h);
    std::vector<uint8_t> buf(n);
    VS_FIXEDFILEINFO* fi = nullptr;
    UINT len = 0;
    if (!n || !GetFileVersionInfoW(path, 0, n, buf.data()) ||
        !VerQueryValueW(buf.data(), L"\\", (void**)&fi, &len) || !fi)
        return "unknown";
    return std::format("{}.{}.{}.{}", HIWORD(fi->dwFileVersionMS), LOWORD(fi->dwFileVersionMS),
                       HIWORD(fi->dwFileVersionLS), LOWORD(fi->dwFileVersionLS));
}

// Every loaded module: base, size, SHA-256 (game-folder modules and msvcrt;
// "-" for other system DLLs), path (relative to the game folder when inside).
static std::string ModuleLines() {
    static std::map<std::wstring, std::string> cache;  // the files don't change under a running game
    static const std::wstring root = GameRoot();
    HMODULE mods[1024];
    DWORD needed = 0;
    std::string out;
    if (!K32EnumProcessModules(GetCurrentProcess(), mods, sizeof mods, &needed)) return out;
    HMODULE crt = GetModuleHandleA("msvcrt.dll");
    for (DWORD i = 0; i < needed / sizeof(HMODULE) && i < 1024; i++) {
        wchar_t path[MAX_PATH] = {};
        GetModuleFileNameW(mods[i], path, MAX_PATH);
        MODULEINFO mi{};
        K32GetModuleInformation(GetCurrentProcess(), mods[i], &mi, sizeof mi);
        const bool game = _wcsnicmp(path, root.c_str(), root.size()) == 0;
        std::string sha = "-";
        if (game || mods[i] == crt) {
            auto it = cache.find(path);
            sha = it != cache.end() ? it->second : (cache[path] = Sha256File(path));
        }
        out += std::format("module {:#010x} {:#x} {} {}\n", (uint32_t)(uintptr_t)mods[i], mi.SizeOfImage, sha,
                           Utf8(game ? path + root.size() : path));
        if (mods[i] == crt) out += std::format("msvcrt {} {} {}\n", sha, FileVersion(path), Utf8(path));
    }
    return out;
}

// The region's first byte now, or 0.
static uint32_t RegionAddr(const Region& r, uint32_t player) {
    uint32_t p = r.player ? player : r.base_va;
    for (uint32_t d : r.derefs) p = p ? SafeRead32(p + d) : 0;
    return p ? p + r.add : 0;
}

// ---- census (census.hpp, readset.hpp) ------------------------------------------
// ssb_census.txt turns it on. Census: at record 0, every `every`th record and
// the first tick the human Body steps, each point's regions as captured go
// through census::Scan into <stem>.census. Read-set: on the selected physics
// entries, the heap blocks the regions point at (live, at ENTRY), minus blocks
// holding a captured region, plus `guard` ranges, are page-guarded until the
// human Player tick returns (or, failing that, the next cycle); `hw` watches
// do the same with data breakpoints on `hw_ticks`. Output: <stem>.readset.
inline census::Options g_census;
inline bool g_censusWanted = false;
inline census::Sidecar g_censusFile, g_readsetFile;
inline census::HeapIndex g_heaps;
inline uint32_t g_firstEntry = UINT32_MAX, g_entries = 0, g_samples = 0, g_ptrs = 0, g_windows = 0;
inline bool g_window = false, g_hwWindow = false;
inline uint32_t g_windowRecord = 0;
inline int32_t g_windowTick = 0;
inline std::string g_windowText, g_hwText;
inline LARGE_INTEGER g_windowStart{};
inline uint64_t g_windowSetupUs = 0;
inline readset::ArmResult g_armResult;
constexpr const char* kPointName[] = {"pre", "post", "entry"};

static uint64_t MicrosSince(LARGE_INTEGER from) {
    static LARGE_INTEGER f{};
    if (!f.QuadPart) QueryPerformanceFrequency(&f);
    LARGE_INTEGER now;
    QueryPerformanceCounter(&now);
    return (uint64_t)((now.QuadPart - from.QuadPart) * 1000000 / f.QuadPart);
}

static void CensusSample(Point point) {
    LARGE_INTEGER t0;
    QueryPerformanceCounter(&t0);
    g_heaps.Build();
    const uint64_t walk = MicrosSince(t0);
    std::vector<census::View> views;
    for (size_t i = 0; i < g_regions.size(); i++) {
        const Region& r = g_regions[i];
        if (r.point == point && (Rec().region_ok >> i & 1)) views.push_back({r.name.c_str(), Block() + r.at, r.len});
    }
    const std::string tag = std::format("{} {} {}", g_ticks, Rec().rec_tick, kPointName[point]);
    const uint32_t n = census::Scan(g_censusFile, g_heaps, tag, views);
    g_censusFile.Line(std::format("sample {} regions {} heap_blocks {}{} walk_us {} us {} ptrs {}", tag, views.size(),
                                  g_heaps.blocks.size(), g_heaps.truncated ? " truncated" : "", walk,
                                  MicrosSince(t0), n));
    g_samples++;
    g_ptrs += n;
}

// The base of a `guard`/`hw` chain.
static uint32_t SpecBase(const census::Spec& s, uint32_t body, uint32_t player) {
    if (s.chain.base == "body") return body;
    if (s.chain.base == "player") return player;
    return (uint32_t)(uintptr_t)GetModuleHandleA(s.chain.base == "exe" ? nullptr : s.chain.base.c_str());
}

// At the human Body's physics step entry, on the game thread.
static void OpenWindow(uint32_t body, uint32_t player) {
    QueryPerformanceCounter(&g_windowStart);
    g_windowRecord = g_ticks;
    g_windowTick = Rec().rec_tick;
    g_heaps.Build();
    std::vector<std::pair<uint32_t, uint32_t>> captured;   // [a, b) of every region now
    for (const Region& r : g_regions)
        if (const uint32_t a = RegionAddr(r, player)) captured.push_back({a, a + r.len});
    std::vector<readset::Span> spans;
    std::map<uint32_t, uint32_t> spanOf;   // block base -> span
    std::string text;
    const census::LiveMem mem;
    std::vector<uint8_t> buf;
    for (const Region& r : g_regions) {
        const uint32_t a = RegionAddr(r, player);
        buf.assign(r.len, 0);
        if (!a || !SafeCopy(a, buf.data(), r.len)) continue;
        for (uint32_t off = 0; off + 4 <= r.len; off += 4) {
            uint32_t v;
            memcpy(&v, &buf[off], 4);
            const census::Block* b = v >= 0x10000 ? g_heaps.Find(v) : nullptr;
            if (!b) continue;
            const uint32_t end = b->base + (b->size ? b->size : 1);
            bool holdsRegion = false;
            for (const auto& [lo, hi] : captured) holdsRegion |= lo < end && b->base < hi;
            if (holdsRegion) continue;   // its target is captured: nothing to learn
            auto [it, fresh] = spanOf.try_emplace(b->base, (uint32_t)spans.size());
            if (fresh) {
                spans.push_back({b->base, end});
                const std::string cls = census::ObjectClass(mem, b->base);
                text += std::format("span {} {:#x} {:#x} {} {}\n", it->second, b->base, b->size, g_heaps.Name(b->heap),
                                    cls.empty() ? "-" : cls);
            }
            text += std::format("ref {} {} {:#x} {:#x}\n", it->second, r.name, off, v);
        }
    }
    for (const census::Spec& g : g_census.guards) {
        const uint32_t a = census::Resolve(g.chain, SpecBase(g, body, player));
        if (!a) {
            text += "guard_unresolved " + g.text + "\n";
            continue;
        }
        text += std::format("span {} {:#x} {:#x} guard {}\n", spans.size(), a, g.len, g.text);
        spans.push_back({a, a + g.len});
    }
    g_windowText = text;
    g_windowSetupUs = MicrosSince(g_windowStart);
    QueryPerformanceCounter(&g_windowStart);
    g_armResult = readset::Arm(spans, g_census.max_exceptions);
    g_window = true;
}

// At the human Player tick's return, or on any other way out. Never leaves a
// page guarded.
static void CloseWindow() {
    if (!g_window) return;
    readset::Disarm();
    g_window = false;
    const uint64_t us = MicrosSince(g_windowStart);
    g_windows++;
    g_readsetFile.Line(std::format(
        "window {} {} pages {} skipped {}{} exceptions {} hits {} ignored {} other_thread {} site_overflow {} "
        "bailed {} setup_us {} us {}",
        g_windowRecord, g_windowTick, g_armResult.pages, g_armResult.skipped,
        g_armResult.truncated ? " truncated" : "", (uint32_t)readset::g_exceptions, (uint32_t)readset::g_hits,
        (uint32_t)readset::g_ignored, (uint32_t)readset::g_other, (uint32_t)readset::g_siteOverflow,
        (uint32_t)readset::g_bailed, g_windowSetupUs, us));
    g_readsetFile.Put(g_windowText);
    for (const readset::Site& s : readset::Sites()) {
        g_readsetFile.Line(std::format("site {} {} {} {} {} {:#x} {}", s.span,
                                       s.tid == readset::g_gameTid ? "game" : "other", s.tid,
                                       census::ModRva(s.eip), s.write ? "w" : "r", s.addr, s.count));
    }
    if (readset::g_bailed)
        Log(std::format("trace: read-set window at record {} bailed out after {} guard exceptions", g_windowRecord,
                        (uint32_t)readset::g_exceptions));
}

static void OpenHw(uint32_t body, uint32_t player) {
    uint32_t addrs[4] = {};
    const uint32_t n = (uint32_t)g_census.hw.size();
    std::string text;
    bool ok = true;
    for (uint32_t i = 0; i < n; i++) {
        const census::Spec& s = g_census.hw[i];
        addrs[i] = census::Resolve(s.chain, SpecBase(s, body, player));
        ok &= addrs[i] && !(addrs[i] & 3);
        text += std::format("hwwatch {} {} {:#x}\n", i, s.text, addrs[i]);
    }
    g_hwWindow = ok && readset::HwArm(addrs, n);
    g_hwText = std::format("hwwindow {} {} armed {}\n", g_ticks, Rec().rec_tick, g_hwWindow ? 1 : 0) + text;
    if (!g_hwWindow) {
        g_readsetFile.Put(g_hwText);
        Log(std::format("trace: data breakpoints not armed at record {}", g_ticks));
    }
}

static void CloseHw() {
    if (!g_hwWindow) return;
    readset::HwDisarm();
    g_hwWindow = false;
    g_readsetFile.Put(g_hwText);
    for (const readset::HwHit& h : readset::HwHits()) {
        g_readsetFile.Line(std::format("hw {} {} {} {:#x} {:#x} {}", h.dr, h.tid, census::ModRva(h.eip), h.addr,
                                       h.value, h.unread ? "unread" : h.changed ? "changed" : "same"));
    }
    if (readset::g_hwOverflow) g_readsetFile.Line(std::format("hw_overflow {}", (uint32_t)readset::g_hwOverflow));
}

// ssb_census.txt; false when present but malformed.
static bool LoadCensus() {
    FILE* f = nullptr;
    g_censusWanted = !fopen_s(&f, (ssbhook::DllDir() + "ssb_census.txt").c_str(), "rb") && f;
    if (!g_censusWanted) return true;
    std::string text;
    char buf[4096];
    size_t n;
    while ((n = fread(buf, 1, sizeof buf, f))) text.append(buf, n);
    fclose(f);
    std::string err;
    if (!census::ParseOptions(text, &g_census, &err)) {
        Log("trace: ssb_census.txt: " + err);
        return false;
    }
    if ((g_census.readset || !g_census.hw.empty()) && !readset::Install()) {
        Log("trace: the read-set exception handler could not be installed");
        return false;
    }
    return true;
}

// ---- contact probe (`probe_ticks`) -----------------------------------------------
// On the listed rec_ticks, from the human Body's step entry to the Player
// tick's return, the step's obstacle path is logged into <stem>.readset (all
// floats as IEEE hex, bit-exact). ebx is the Body throughout the step
// SG+0x98360. Listing: capstone of ssb-decomp/bin/Supreme_Game.dll.
//   SG+0x9877D/0x9959B/0x99D68, after each `call SG+0x9BEF0` (candidate query;
//     eax = the Body's candidate vector [Body+0x1E4]): `probe_query` with the
//     finder [Body+0x4DC] (query count +0x28, re-query count +0x24, cached box
//     +0x8, result vector [+0x4]), the octree's visit epoch [[finder]+0x2C]
//     and both vectors' entries entry:object:mark(+0xC):dynamic(+0x10):class.
//   SG+0x9972E, `call SG+0x3570`: `probe_call` with Body+0x10, the candidate
//     ([ebp-0x24] iterator), the current slot [esp] and previous slot [esp+0xC],
//     8 points (24 floats) each.
//   SG+0x99733, after it: `probe_box`, the swept box [ebp-0xC90] (6 floats).
//   SG+0x9974D, SG+0x4010 returned nonzero: `probe_broadphase_hit`.
//   SG+0x9976D, after `call SG+0x3A40`: `probe_contact` point [ebp-0x384],
//     normal [ebp-0x1D8], depth [ebp-0x1CC].
inline uint32_t g_probeBody = 0;
inline std::string g_probeText;
static SafetyHookMid g_probeHooks[7]{};

static std::string Words(uint32_t a, uint32_t n) {
    std::string s;
    for (uint32_t i = 0; i < n; i++) {
        uint32_t v = 0;
        s += SafeCopy(a + 4 * i, &v, 4) ? std::format("{}{:08x}", i ? "," : "", v) : std::string(i ? ",?" : "?");
    }
    return s;
}

static std::string ClassOf(uint32_t obj) {
    const std::string c = census::ObjectClass(census::LiveMem{}, obj);
    return c.empty() ? "-" : c;
}

// A std::vector of octree entries (begin +4, end +8).
static std::string Entries(uint32_t vec) {
    const uint32_t b = SafeRead32(vec + 4), e = SafeRead32(vec + 8);
    if (e < b || e - b > 4 * 1024 || (e - b) & 3) return "?";
    std::string s;
    for (uint32_t p = b; p < e; p += 4) {
        const uint32_t entry = SafeRead32(p), obj = SafeRead32(entry + 8);
        uint8_t dyn = 0xFF;
        SafeCopy(entry + 0x10, &dyn, 1);
        s += std::format("{}{:#x}:{:#x}:{}:{}:{}", p == b ? "" : " ", entry, obj, SafeRead32(entry + 0xC), dyn,
                         ClassOf(obj));
    }
    return s;
}

template <uint32_t Site>
static void ProbeQuery(SafetyHookContext& ctx) {
    if (!g_probeBody || ctx.ebx != g_probeBody) return;
    const uint32_t finder = SafeRead32(ctx.ebx + 0x4DC);
    g_probeText += std::format(
        "probe_query {:#x} epoch {} queries {} requeries {} finder_box {} finder [{}] body [{}]\n", Site,
        SafeRead32(SafeRead32(finder) + 0x2C), SafeRead32(finder + 0x28), SafeRead32(finder + 0x24),
        Words(finder + 8, 6), Entries(SafeRead32(finder + 4)), Entries((uint32_t)ctx.eax));
}

static void ProbeCall(SafetyHookContext& ctx) {
    if (!g_probeBody || ctx.ebx != g_probeBody) return;
    const uint32_t cur = SafeRead32((uint32_t)ctx.esp), prev = SafeRead32((uint32_t)ctx.esp + 0xC);
    const uint32_t entry = SafeRead32(SafeRead32((uint32_t)ctx.ebp - 0x24)), obj = SafeRead32(entry + 8);
    g_probeText += std::format("probe_call f10 {} entry {:#x} obj {:#x} class {} cur {:#x} {} prev {:#x} {}\n",
                               SafeRead32(ctx.ebx + 0x10), entry, obj, ClassOf(obj), cur, Words(cur, 24), prev,
                               Words(prev, 24));
}

static void ProbeBox(SafetyHookContext& ctx) {
    if (g_probeBody && ctx.ebx == g_probeBody)
        g_probeText += "probe_box " + Words((uint32_t)ctx.ebp - 0xC90, 6) + "\n";
}

static void ProbeHit(SafetyHookContext& ctx) {
    if (g_probeBody && ctx.ebx == g_probeBody) g_probeText += "probe_broadphase_hit\n";
}

static void ProbeContact(SafetyHookContext& ctx) {
    if (!g_probeBody || ctx.ebx != g_probeBody) return;
    const uint32_t bp = (uint32_t)ctx.ebp;
    g_probeText += std::format("probe_contact point {} normal {} depth {}\n", Words(bp - 0x384, 3),
                               Words(bp - 0x1D8, 3), Words(bp - 0x1CC, 1));
}

static void OpenProbe(uint32_t body) {
    g_probeBody = body;
    g_probeText = std::format("probe_window {} {} body {:#x}\n", g_ticks, Rec().rec_tick, body);
}

static void CloseProbe() {
    if (!g_probeBody) return;
    g_probeBody = 0;
    g_readsetFile.Put(g_probeText);
    g_probeText.clear();
}

static bool InstallProbe(uint8_t* sg) {
    static constexpr uint8_t kQ1[] = {0x50, 0x8B, 0xCB, 0xE8};               // push eax; mov ecx,ebx; call
    static constexpr uint8_t kQ2[] = {0x8B, 0xC8, 0x89, 0x85};               // mov ecx,eax; mov [ebp-..],eax
    static constexpr uint8_t kQ3[] = {0xC7, 0x83, 0x74, 0x03, 0x00, 0x00};   // mov dword [ebx+0x374],0
    static constexpr uint8_t kCall[] = {0xE8, 0x3D, 0x9E, 0xF6, 0xFF};       // call SG+0x3570
    static constexpr uint8_t kBox[] = {0x8B, 0x55, 0xEC, 0x8B, 0x4A, 0x08};  // mov edx,[ebp-14h]; mov ecx,[edx+8]
    static constexpr uint8_t kHit[] = {0x8D, 0x8D, 0x34, 0xFE, 0xFF, 0xFF};  // lea ecx,[ebp-1CCh]
    static constexpr uint8_t kCt[] = {0x8B, 0x8D, 0x28, 0xFE, 0xFF, 0xFF};   // mov ecx,[ebp-1D8h]
    if (!GameAddresses::ValidateCode("Supreme_Game.dll+0x9877D (probe)", sg + 0x9877D, kQ1) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x9959B (probe)", sg + 0x9959B, kQ2) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x99D68 (probe)", sg + 0x99D68, kQ3) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x9972E (probe)", sg + 0x9972E, kCall) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x99733 (probe)", sg + 0x99733, kBox) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x9974D (probe)", sg + 0x9974D, kHit) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x9976D (probe)", sg + 0x9976D, kCt))
        return false;
    g_probeHooks[0] = CreateMidHook<ProbeQuery<0x9877D>>(sg + 0x9877D);
    g_probeHooks[1] = CreateMidHook<ProbeQuery<0x9959B>>(sg + 0x9959B);
    g_probeHooks[2] = CreateMidHook<ProbeQuery<0x99D68>>(sg + 0x99D68);
    g_probeHooks[3] = CreateMidHook<ProbeCall>(sg + 0x9972E);
    g_probeHooks[4] = CreateMidHook<ProbeBox>(sg + 0x99733);
    g_probeHooks[5] = CreateMidHook<ProbeHit>(sg + 0x9974D);
    g_probeHooks[6] = CreateMidHook<ProbeContact>(sg + 0x9976D);
    for (auto& h : g_probeHooks)
        if (!h) return false;
    return true;
}

// ---- the trace file ----------------------------------------------------------
static void Close() {
    if (!g_file) return;
    CloseWindow();
    CloseHw();
    CloseProbe();
    WritePending();
    ssbhook::Arm(nullptr);
    // The late file section and the trailer, then checked publication.
    g_writeStatus =
        g_opened.CloseAndPublish(g_file, g_writeStatus, g_ticks, g_pendingPath.c_str(), g_tracePath.c_str());
    g_file = nullptr;
    Log(std::format("trace: closed g{}: {} ticks, {} without the post point, max {} other rider(s), "
                    "ssb-hook calls {}, FPU violations {}, write status {}{}",
                    g_gen, g_ticks, g_noPost, g_maxOthers, ssbhook::Calls(), ssbhook::FpuViolations(),
                    g_writeStatus, g_writeStatus ? " (FAILED: the trace is incomplete)" : ""));
    if (g_censusFile.on() || g_readsetFile.on()) {
        const uint32_t cs = g_censusFile.Close(), rs = g_readsetFile.Close();
        Log(std::format("trace: census {} samples, {} pointers, status {}; read-set {} windows, status {}", g_samples,
                        g_ptrs, cs, g_windows, rs));
    }
}

static void Open(TasSharedState* s, uint16_t cw, uint32_t firstPreCalls) {
    g_gen = (uint32_t)s->arm_generation;
    g_triedGen = g_gen;
    g_ticks = g_noPost = g_maxOthers = g_writeStatus = 0;
    g_pending = false;
    g_tracePlayer = s->player_ptr;
    g_playerFreed = false;
    g_firstEntry = UINT32_MAX;
    g_entries = g_samples = g_ptrs = g_windows = 0;
    for (Region& r : g_regions) {
        r.base_va = r.player ? 0
                  : (uint32_t)(uintptr_t)GetModuleHandleA(r.base == "exe" ? nullptr : r.base.c_str());
    }
    CreateDirectoryA((g_dir + "traces").c_str(), nullptr);
    FILETIME ft;
    GetSystemTimeAsFileTime(&ft);
    const uint64_t ms = ((uint64_t(ft.dwHighDateTime) << 32 | ft.dwLowDateTime) - 116444736000000000ull) / 10000;
    const std::string path = std::format("{}traces\\{}-g{}.ssbtrace", g_dir, ms, g_gen);
    g_tracePath = path;
    g_pendingPath = path + ".pending";
    if (fopen_s(&g_file, g_pendingPath.c_str(), "wb") || !g_file) {
        g_file = nullptr;
        Log("trace: FAILED to create " + g_pendingPath);
        return;
    }
    setvbuf(g_file, nullptr, _IOFBF, 1 << 22);
    if (g_env.force_seed) *HoldRand() = g_env.seed;   // the arm starts from a known rand state
    const uint32_t count = (uint32_t)s->recorded_count;
    std::string h;
    h += std::format("record_size {}\n", g_rec.size());
    h += std::format("tick_size {}\n", sizeof(TickRecord));
    h += std::format("hook_calls_before_first_record {}\n", firstPreCalls);
    h += std::format("created_unix_ms {}\n", ms);
    h += std::format("arm_generation {}\n", g_gen);
    h += std::format("recorded_count {}\n", count);
    h += std::format("inputs_sha256 {}\n", Sha256Bytes((const void*)s->input_log, count <= TAS_MAX_TICKS ? count : 0));
    h += std::format("gate_align_rec {}\n", (uint32_t)s->gate_align_rec);
    h += std::format("input_model {}\n", (uint32_t)s->input_model);
    h += std::format("renderer {}\n", renderer::Name(renderer::Detect(true)));
    h += std::format("fpu_cw {:#06x}\n", cw);
    h += std::format("expect_cw {:#06x}\n", g_env.cw);
    h += std::format("expect_mxcsr {:#06x}\n", g_env.mxcsr);
    h += g_env.force_seed ? std::format("srand_forced {}\n", g_env.seed) : std::string("srand_forced none\n");
    for (auto& l : g_srandLog) h += "srand_call " + l + "\n";
    h += std::format("sg_base {:#010x}\n", (uint32_t)(uintptr_t)g_sg);
    h += std::format("cpu {}\n", CpuName());
    h += std::format("ssb_hook {}\n", ssbhook::g_summary);
    h += ModuleLines();
    h += g_opened.Snapshot();
    if (g_censusWanted) {
        const std::string stem = path.substr(0, path.size() - strlen(".ssbtrace"));
        const std::string meta = std::format("trace {}\ncensus_options {}\n", path.substr(path.find_last_of('\\') + 1),
                                             g_census.summary) + h;
        if (!g_censusFile.Open(stem + ".census", "SSBCENSUS 1\n" + meta)) Log("trace: FAILED to create the census file");
        if (census::WantsReadset(g_census) && !g_readsetFile.Open(stem + ".readset", "SSBREADSET 1\n" + meta))
            Log("trace: FAILED to create the read-set file");
    }
    h += "regions\n" + g_regionsText;
    const uint32_t n = (uint32_t)h.size();
    Put("SSBTRACE 3\n", 11);
    Put(&n, 4);
    Put(h.data(), n);
    Log("trace: writing " + g_pendingPath);
    // ssb-hook's per-call files follow the trace: <stem>.ssbcalls.
    ssbhook::Arm((path.substr(0, path.size() - strlen(".ssbtrace")) + ".ssbcalls").c_str());
}

static void Capture(Point point, uint32_t player) {
    TickRecord& t = Rec();
    uint32_t* bases = Bases();
    uint8_t* block = Block();
    for (size_t i = 0; i < g_regions.size(); i++) {
        const Region& r = g_regions[i];
        if (r.point != point) continue;
        const uint32_t p = RegionAddr(r, player);
        uint8_t* dst = block + r.at;
        if (p && SafeCopy(p, dst, r.len)) {
            t.region_ok |= 1u << i;
            bases[i] = p;
        } else {
            memset(dst, 0, r.len);
            bases[i] = 0;
        }
    }
    if (g_censusFile.on() && census::CensusAt(g_census, g_ticks, g_firstEntry, point == PRE)) CensusSample(point);
}

// The cycle cave, every tick, last. `played`: this cycle ran a PLAY tick.
inline void OnCycle(TasSharedState* s, bool played, uint32_t pos, int32_t recTick, uint8_t mask,
                    uint16_t cw, uint32_t csr) {
    if (!g_on) return;
    CloseWindow();   // the post point did not come
    CloseHw();
    CloseProbe();
    if (!played || (g_file && g_gen != s->arm_generation)) Close();
    if (!played) return;
    // A real cumulative sample at this arm's first PRE, before Player work.
    // Do not reset the DLL counter or manufacture a preceding physics record.
    if (!g_file && g_triedGen != s->arm_generation) Open(s, cw, ssbhook::Calls());
    if (!g_file) return;
    WritePending();   // the previous tick's record
    ssbhook::Tick(g_ticks);   // this cycle's record index
    memset(g_rec.data(), 0, g_rec.size());
    TickRecord& t = Rec();
    t.pos = pos;
    t.rec_tick = recTick;
    t.cw_pre = cw;
    t.mxcsr_pre = csr;
    t.input = mask;
    t.player = s->player_ptr;
    if (t.player != g_tracePlayer) t.flags |= FLAG_PLAYER_CHANGED;
    if (g_playerFreed) t.flags |= FLAG_PLAYER_FREED;
    uint8_t b = 0;
    t.cheat = SafeCopy((uint32_t)(uintptr_t)(g_sg + 0x1D5394), &b, 1) ? b : 0xFF;
    t.sse2_math = SafeCopy(g_sse2Flag, &b, 1) ? b : 0xFF;
    t.holdrand_pre = *HoldRand();
    t.hook_calls = ssbhook::Calls();
    t.hook_fpu_bad = ssbhook::FpuViolations();
    t.srand_calls = g_srandCalls;
    Capture(PRE, t.player);
    g_pending = true;
}

// SG+0x84C43, the player tick's epilogue: ebx = the Player. The record is
// written at the next cycle, so riders ticking after the human still count.
static void PostHook(SafetyHookContext& ctx) {
    if (!g_pending) return;
    TickRecord& t = Rec();
    if (!t.player || ctx.ebx != t.player) {
        if (++t.others > g_maxOthers) g_maxOthers = t.others;
        return;
    }
    if (t.human_ticks < 255) t.human_ticks++;
    CloseWindow();
    CloseHw();
    CloseProbe();
    if (t.flags & FLAG_POST) return;   // a second human tick in one cycle: the checker rejects it
    t.flags |= FLAG_POST;
    t.cw_post = g_hookFpuControlWord;
    t.mxcsr_post = g_hookMxcsr;
    t.holdrand_post = *HoldRand();
    t.hook_calls = ssbhook::Calls();
    t.hook_fpu_bad = ssbhook::FpuViolations();
    Capture(POST, (uint32_t)ctx.ebx);
}

// SG+0x98360 entry, the physics step (thiscall, ecx = Body, ret 8): the
// consumption boundary of the Body's lazily filled contact buffers.
static void EntryHook(SafetyHookContext& ctx) {
    if (!g_pending) return;
    TickRecord& t = Rec();
    if (!t.player || ctx.ecx != SafeRead32(t.player + 0x110)) return;   // other riders: the post hook counts them
    if (t.entry_calls < 255) t.entry_calls++;
    if (t.flags & FLAG_ENTRY) return;
    t.flags |= FLAG_ENTRY;
    t.cw_entry = g_hookFpuControlWord;
    t.mxcsr_entry = g_hookMxcsr;
    if (g_firstEntry == UINT32_MAX) g_firstEntry = g_ticks;
    Capture(ENTRY, t.player);
    const uint32_t entry = g_entries++;
    if (g_readsetFile.on() && census::ReadsetAt(g_census, entry, t.rec_tick)) OpenWindow(ctx.ecx, t.player);
    if (g_readsetFile.on() && census::InRanges(g_census.hw_ticks, t.rec_tick)) OpenHw(ctx.ecx, t.player);
    if (g_readsetFile.on() && census::InRanges(g_census.probe_ticks, t.rec_tick)) OpenProbe(ctx.ecx);
}

// SG+0x84160, Player::~Player (stores the Player vtables first): ecx = this.
static void DtorHook(SafetyHookContext& ctx) {
    if (!g_tracePlayer || ctx.ecx != g_tracePlayer) return;
    g_playerFreed = true;
    if (g_pending) Rec().flags |= FLAG_PLAYER_FREED;
    Log(std::format("trace: the traced Player {:#010x} is being destroyed", g_tracePlayer));
}

// msvcrt!srand entry: [esp] = return address, [esp+4] = seed.
static void SrandHook(SafetyHookContext& ctx) {
    g_srandCalls++;
    auto* seed = (uint32_t*)(ctx.esp + 4);
    const uint32_t ret = *(uint32_t*)ctx.esp;
    HMODULE m = nullptr;
    char name[MAX_PATH] = "?";
    if (GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                           (LPCSTR)(uintptr_t)ret, &m)) {
        GetModuleFileNameA(m, name, MAX_PATH);
    }
    const char* base = strrchr(name, '\\');
    std::string line = std::format("{}+{:#x} thread {} seed {:#x}{}", base ? base + 1 : name,
                                   ret - (uint32_t)(uintptr_t)m, GetCurrentThreadId(), *seed,
                                   g_env.force_seed ? std::format(" forced {:#x}", g_env.seed) : "");
    if (g_env.force_seed) *seed = g_env.seed;
    if (g_srandLog.size() < 16) g_srandLog.push_back(line);
    Log("trace: srand " + line);
}

inline void Uninstall() {
    g_postHook = {};
    g_entryHook = {};
    g_dtorHook = {};
    g_srandHook = {};
    for (auto& h : g_probeHooks) h = {};
    g_openA = {};
    g_openW = {};
    Close();
    readset::Uninstall();
    g_on = false;
    g_regions.clear();
    g_state = nullptr;
}

// One region line, or false.
static bool ParseRegion(const std::string& line, Region* r) {
    char name[64], point[8], chain[256];
    char len[32];
    if (sscanf_s(line.c_str(), "%63s %7s %255s %31s", name, (unsigned)sizeof name, point,
                 (unsigned)sizeof point, chain, (unsigned)sizeof chain, len, (unsigned)sizeof len) != 4) {
        return false;
    }
    r->name = name;
    if (!strcmp(point, "pre")) r->point = PRE;
    else if (!strcmp(point, "post")) r->point = POST;
    else if (!strcmp(point, "entry")) r->point = ENTRY;
    else return false;
    census::Chain c;
    if (!census::ParseChain(chain, &c)) return false;   // +off comes last
    r->base = c.base;
    r->player = r->base == "player";
    r->derefs = c.derefs;
    r->add = c.add;
    r->len = ParseNum(len);
    return r->len > 0 && !r->base.empty();
}

// ssb_trace_env.txt. False when it exists but is malformed; g_env.have says
// whether it exists.
inline bool LoadEnv() {
    g_env = {};
    FILE* f = nullptr;
    if (fopen_s(&f, (ssbhook::DllDir() + "ssb_trace_env.txt").c_str(), "rb") || !f) return true;
    char line[256];
    bool cw = false, csr = false, ok = true;
    while (fgets(line, sizeof line, f)) {
        char key[32], val[64];
        if (line[0] == '#' || sscanf_s(line, "%31s %63s", key, (unsigned)sizeof key, val, (unsigned)sizeof val) != 2)
            continue;
        const uint32_t v = ParseNum(val);
        if (!strcmp(key, "cw")) { g_env.cw = (uint16_t)v; cw = true; }
        else if (!strcmp(key, "mxcsr")) { g_env.mxcsr = v; csr = true; }
        else if (!strcmp(key, "srand")) { g_env.seed = v; g_env.force_seed = true; }
        else ok = false;
    }
    fclose(f);
    g_env.have = ok && cw && csr;
    if (!g_env.have) Log("trace: ssb_trace_env.txt needs `cw <x87 CW>` and `mxcsr <MXCSR>` lines and nothing unknown");
    return g_env.have;
}

// Off unless ssb_trace_regions.txt exists (true). False when it exists and
// tracing could not be installed: the caller fails the injection.
inline bool Install(GameAddresses& addr, TasSharedState* state) {
    g_dir = ssbhook::DllDir();
    FILE* f = nullptr;
    if (fopen_s(&f, (g_dir + "ssb_trace_regions.txt").c_str(), "rb") || !f) {
        if (GetFileAttributesA((g_dir + "ssb_census.txt").c_str()) == INVALID_FILE_ATTRIBUTES) return true;
        Log("trace: ssb_census.txt needs ssb_trace_regions.txt");
        return false;
    }
    char line[512];
    std::vector<Region> regions;
    std::string text;
    uint32_t at = 0;
    bool ok = true;
    while (fgets(line, sizeof line, f)) {
        text += line;
        std::string l = line;
        const size_t first = l.find_first_not_of(" \t\r\n");
        if (first == std::string::npos || l[first] == '#' || first > 0) continue;   // indented = class lines
        Region r;
        if (!ParseRegion(l, &r)) {
            Log("trace: bad region line: " + l);
            ok = false;
            continue;
        }
        r.at = at;
        at += r.len;
        regions.push_back(r);
    }
    fclose(f);
    if (!ok || regions.empty() || regions.size() > 32) {
        Log("trace: ssb_trace_regions.txt unusable (bad lines, none, or over 32)");
        return false;
    }
    if (!g_env.have) {
        Log("trace: ssb_trace_env.txt (the expected FPU environment) is missing");
        return false;
    }
    if (!ResolveMsvcrt() || !LoadCensus()) return false;
    auto sg = (uint8_t*)addr.sg;
    static constexpr uint8_t kEpilogue[] = {0x5F, 0x5E, 0x5B, 0x83, 0xC4, 0x18, 0xC3};   // pop edi/esi/ebx; add esp,18; ret
    static constexpr uint8_t kStep[] = {0x55, 0x8B, 0xEC, 0x6A, 0xFF, 0x68};             // push ebp; mov ebp,esp; push -1; push imm
    static constexpr uint8_t kDtor[] = {0x6A, 0xFF, 0x68};                               // push -1; push imm
    static constexpr uint8_t kDtorTail[] = {0x64, 0xA1, 0x00, 0x00, 0x00, 0x00, 0x50};   // mov eax,fs:[0]; push eax
    if (!GameAddresses::ValidateCode("Supreme_Game.dll+0x84C43 (player tick epilogue)", sg + 0x84C43, kEpilogue) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x98360 (physics step)", sg + 0x98360, kStep) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x84160 (Player::~Player)", sg + 0x84160, kDtor) ||
        !GameAddresses::ValidateCode("Supreme_Game.dll+0x84167 (Player::~Player)", sg + 0x84167, kDtorTail)) {
        return false;
    }
    g_state = state;
    g_sg = sg;
    g_regions = std::move(regions);
    g_regionsText = text;
    g_rec.assign(sizeof(TickRecord) + 4 * g_regions.size() + at, 0);
    ModuleLines();   // hash what is loaded now, off the game thread
    HMODULE kb = GetModuleHandleA("kernelbase.dll");
    auto openA = kb ? (void*)GetProcAddress(kb, "CreateFileA") : nullptr;
    auto openW = kb ? (void*)GetProcAddress(kb, "CreateFileW") : nullptr;
    g_postHook = CreateMidHook<PostHook>(sg + 0x84C43);
    g_entryHook = CreateMidHook<EntryHook>(sg + 0x98360);
    g_dtorHook = CreateMidHook<DtorHook>(sg + 0x84160);
    g_srandHook = CreateMidHook<SrandHook>(g_srand);
    if (openA && openW) {
        g_openA = safetyhook::create_inline(openA, (void*)&OpenAHook);
        g_openW = safetyhook::create_inline(openW, (void*)&OpenWHook);
    }
    if (!g_census.probe_ticks.empty() && !InstallProbe(sg)) {
        Log("trace: the contact probe could not be installed");
        Uninstall();
        return false;
    }
    if (!g_postHook || !g_entryHook || !g_dtorHook || !g_srandHook || !g_openA || !g_openW) {
        Log("trace: a trace hook could not be installed");
        Uninstall();
        return false;
    }
    g_on = true;
    if (g_censusWanted) Log("trace: census on: " + g_census.summary);
    Log(std::format("trace: on, {} regions, {} bytes per tick, expect CW {:#06x} MXCSR {:#06x}, srand {}, into {}traces",
                    g_regions.size(), g_rec.size(), g_env.cw, g_env.mxcsr,
                    g_env.force_seed ? std::format("forced {}", g_env.seed) : std::string("free"), g_dir));
    return true;
}

} // namespace trace
