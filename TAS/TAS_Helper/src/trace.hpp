#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "safe_read.hpp"
#include "fpu_safe_hook.hpp"
#include "renderer_info.hpp"
#include "ssb_hook.hpp"
#include <bcrypt.h>
#include <intrin.h>
#include <psapi.h>
#include <cstdio>
#include <map>
#include <vector>
#pragma comment(lib, "bcrypt.lib")

// Per-tick state trace for the bit-exact rewrite (ssb-recomp re/harness.md;
// the reader and diff tool are ssb-recomp crates/ssb-trace). Opt-in: on only
// when ssb_trace_regions.txt sits next to TAS_Helper.dll. Every PLAY arm (a
// CONT prefix too) writes traces\<unix ms>-g<arm generation>.ssbtrace beside it:
//   "SSBTRACE 1\n", u32 header length, header text ("key value" lines, then a
//   "regions" line and the regions file verbatim), then per PLAY tick one
//   TickRecord followed by every region's bytes in file order.
// Region line: name point chain len [skip]. point: pre = the cycle cave, after
// PLAY wrote this tick's input and before its physics; post = the human
// Player tick (SG+0x849E0) returning, at SG+0x84C43. chain: base[>off]...[+off],
// base `player`, `exe` or a module name; each >off follows the pointer at +off, a
// final +off is added. skip (off:len,...) is read by the diff tool only.
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
    uint8_t reserved;
    uint32_t hook_calls;        // ssb-hook detour calls so far (0 without it)
    uint32_t region_ok;         // bit i: region i was readable
};
#pragma pack(pop)
static_assert(sizeof(TickRecord) == 32);
constexpr uint8_t FLAG_POST = 1;   // the post point was captured
// The x87 control word must not change during a trace: it is the renderer's
// (DirectX 0x007F = 24-bit, OpenGL 0x027F = 53-bit; header fpu_cw), and a
// driver changing it mid-run changes the physics.
constexpr uint32_t EXPECTED_MXCSR = 0x1F80;   // all masked, no DAZ/FTZ, nearest
constexpr uint32_t MXCSR_CONTROL = 0xFFC0;    // the control bits; the low 6 are sticky flags

struct Region {
    std::string name;
    bool post = false;
    std::string base;   // "player" or a module name
    std::vector<uint32_t> derefs;
    uint32_t add = 0;
    uint32_t len = 0;
    uint32_t at = 0;    // byte offset in the tick's region block
    uint32_t base_va = 0;
};

inline TasSharedState* g_state = nullptr;
inline std::vector<Region> g_regions;
inline std::string g_regionsText;
inline uint32_t g_regionBytes = 0;
inline std::string g_dir;
inline FILE* g_file = nullptr;
inline uint32_t g_gen = 0;
inline std::vector<uint8_t> g_rec;  // TickRecord + regions, being filled
inline bool g_pending = false;      // g_rec holds a pre not yet written
inline uint32_t g_ticks = 0, g_noPost = 0, g_badCw = 0, g_badCsr = 0, g_maxOthers = 0;
inline uint16_t g_traceCw = 0;   // the control word at the trace's first tick
static SafetyHookMid g_postHook{};

inline TickRecord& Rec() { return *reinterpret_cast<TickRecord*>(g_rec.data()); }

static uint32_t ParseNum(const std::string& s) { return (uint32_t)strtoul(s.c_str(), nullptr, 0); }

// One region line, or false.
static bool ParseRegion(const std::string& line, Region* r) {
    char name[64], point[8], chain[256];
    char len[32];
    if (sscanf_s(line.c_str(), "%63s %7s %255s %31s", name, (unsigned)sizeof name, point,
                 (unsigned)sizeof point, chain, (unsigned)sizeof chain, len, (unsigned)sizeof len) != 4) {
        return false;
    }
    r->name = name;
    if (!strcmp(point, "post")) r->post = true;
    else if (strcmp(point, "pre")) return false;
    std::string c = chain;
    size_t i = c.find_first_of(">+");
    r->base = c.substr(0, i);
    while (i != std::string::npos) {
        const char op = c[i];
        const size_t j = c.find_first_of(">+", i + 1);
        const uint32_t v = ParseNum(c.substr(i + 1, j == std::string::npos ? j : j - i - 1));
        if (op == '>') {
            if (r->add) return false;   // +off comes last
            r->derefs.push_back(v);
        } else {
            r->add = v;
        }
        i = j;
    }
    r->len = ParseNum(len);
    return r->len > 0 && !r->base.empty();
}

static void Capture(bool post, uint32_t player) {
    TickRecord& t = Rec();
    for (size_t i = 0; i < g_regions.size(); i++) {
        const Region& r = g_regions[i];
        if (r.post != post) continue;
        uint32_t p = r.base == "player" ? player : r.base_va;
        for (uint32_t d : r.derefs) p = p ? SafeRead32(p + d) : 0;
        uint8_t* dst = g_rec.data() + sizeof(TickRecord) + r.at;
        if (p && SafeCopy(p + r.add, dst, r.len)) {
            t.region_ok |= 1u << i;
        } else {
            memset(dst, 0, r.len);
        }
    }
}

static void CheckFpu(uint16_t cw, uint32_t csr) {
    if (cw != g_traceCw && g_badCw++ == 0)
        LogRing(g_state, LOG_ERROR, "trace: the x87 control word changed during the trace");
    if ((csr & MXCSR_CONTROL) != EXPECTED_MXCSR && g_badCsr++ == 0)
        LogRing(g_state, LOG_ERROR, "trace: MXCSR is not 0x1F80 on hook entry");
}

static void WritePending() {
    if (!g_pending || !g_file) return;
    g_pending = false;
    if (!(Rec().flags & FLAG_POST)) g_noPost++;
    fwrite(g_rec.data(), 1, g_rec.size(), g_file);
    g_ticks++;
}

// For ssb-hook's panic hook too: the tick it died in becomes the last record.
inline void __cdecl Flush() {
    WritePending();
    if (g_file) fflush(g_file);
}

static std::string Sha256File(const std::string& path) {
    static std::map<std::string, std::string> cache;  // the files don't change under a running game
    if (auto it = cache.find(path); it != cache.end()) return it->second;
    std::string out = "unreadable";
    FILE* f = nullptr;
    if (!fopen_s(&f, path.c_str(), "rb") && f) {
        std::vector<uint8_t> data;
        uint8_t buf[65536];
        size_t n;
        while ((n = fread(buf, 1, sizeof buf, f)) > 0) data.insert(data.end(), buf, buf + n);
        fclose(f);
        uint8_t h[32];
        if (BCRYPT_SUCCESS(BCryptHash(BCRYPT_SHA256_ALG_HANDLE, nullptr, 0, data.data(),
                                      (ULONG)data.size(), h, sizeof h))) {
            out.clear();
            for (uint8_t b : h) out += std::format("{:02x}", b);
        }
    }
    return cache[path] = out;
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

// Every module loaded from the game folder (the game, its plugins, this DLL,
// ssb_hook.dll): name, base, SHA-256.
static std::string ModuleLines() {
    char exe[MAX_PATH] = {};
    GetModuleFileNameA(nullptr, exe, MAX_PATH);
    std::string root(exe);
    root = root.substr(0, root.find_last_of("\\/") + 1);
    HMODULE mods[512];
    DWORD needed = 0;
    std::string out;
    if (!K32EnumProcessModules(GetCurrentProcess(), mods, sizeof mods, &needed)) return out;
    for (DWORD i = 0; i < needed / sizeof(HMODULE) && i < 512; i++) {
        char path[MAX_PATH] = {};
        GetModuleFileNameA(mods[i], path, MAX_PATH);
        if (_strnicmp(path, root.c_str(), root.size()) != 0) continue;
        out += std::format("module {} {:#010x} {}\n", path + root.size(),
                           (uint32_t)(uintptr_t)mods[i], Sha256File(path));
    }
    return out;
}

static void Close() {
    if (!g_file) return;
    WritePending();
    fclose(g_file);
    g_file = nullptr;
    Log(std::format("trace: closed g{}: {} ticks, {} without the post point, {} bad CW, {} bad MXCSR, "
                    "max {} other rider(s), ssb-hook calls {}",
                    g_gen, g_ticks, g_noPost, g_badCw, g_badCsr, g_maxOthers, ssbhook::Calls()));
}

static void Open(TasSharedState* s, uint16_t cw) {
    g_gen = (uint32_t)s->arm_generation;
    g_ticks = g_noPost = g_badCw = g_badCsr = g_maxOthers = 0;
    g_pending = false;
    g_traceCw = cw;
    for (Region& r : g_regions) {
        r.base_va = r.base == "player" ? 0
                  : (uint32_t)(uintptr_t)GetModuleHandleA(r.base == "exe" ? nullptr : r.base.c_str());
    }
    CreateDirectoryA((g_dir + "traces").c_str(), nullptr);
    FILETIME ft;
    GetSystemTimeAsFileTime(&ft);
    const uint64_t ms = ((uint64_t(ft.dwHighDateTime) << 32 | ft.dwLowDateTime) - 116444736000000000ull) / 10000;
    const std::string path = std::format("{}traces\\{}-g{}.ssbtrace", g_dir, ms, g_gen);
    if (fopen_s(&g_file, path.c_str(), "wb") || !g_file) {
        g_file = nullptr;
        Log("trace: FAILED to create " + path);
        return;
    }
    setvbuf(g_file, nullptr, _IOFBF, 1 << 22);
    std::string h;
    h += std::format("record_size {}\n", g_rec.size());
    h += std::format("created_unix_ms {}\n", ms);
    h += std::format("arm_generation {}\n", g_gen);
    h += std::format("recorded_count {}\n", (uint32_t)s->recorded_count);
    h += std::format("gate_align_rec {}\n", (uint32_t)s->gate_align_rec);
    h += std::format("input_model {}\n", (uint32_t)s->input_model);
    h += std::format("renderer {}\n", renderer::Name(renderer::Detect(true)));
    h += std::format("fpu_cw {:#06x}\n", cw);
    h += std::format("cpu {}\n", CpuName());
    h += std::format("ssb_hook {}\n", ssbhook::g_summary);
    h += ModuleLines();
    h += "regions\n" + g_regionsText;
    const uint32_t n = (uint32_t)h.size();
    fwrite("SSBTRACE 1\n", 1, 11, g_file);
    fwrite(&n, 4, 1, g_file);
    fwrite(h.data(), 1, n, g_file);
    Log("trace: writing " + path);
}

// The cycle cave, every tick, last. `played`: this cycle ran a PLAY tick.
inline void OnCycle(TasSharedState* s, bool played, uint32_t pos, int32_t recTick, uint8_t mask,
                    uint16_t cw, uint32_t csr) {
    if (g_regions.empty()) return;
    if (!played || (g_file && g_gen != s->arm_generation)) Close();
    if (!played) return;
    if (!g_file) Open(s, cw);
    if (!g_file) return;
    WritePending();   // the previous tick's record
    memset(g_rec.data(), 0, g_rec.size());
    TickRecord& t = Rec();
    t.pos = pos;
    t.rec_tick = recTick;
    t.cw_pre = cw;
    t.mxcsr_pre = csr;
    t.input = mask;
    t.hook_calls = ssbhook::Calls();
    CheckFpu(cw, t.mxcsr_pre);
    Capture(false, s->player_ptr);
    g_pending = true;
}

// SG+0x84C43, the player tick's epilogue: ebx = the Player. The record is
// written at the next cycle, so riders ticking after the human still count.
static void PostHook(SafetyHookContext& ctx) {
    if (!g_pending || !g_state) return;
    TickRecord& t = Rec();
    if (ctx.ebx != g_state->player_ptr || (t.flags & FLAG_POST)) {
        if (++t.others > g_maxOthers) g_maxOthers = t.others;
        return;
    }
    t.flags |= FLAG_POST;
    t.cw_post = g_hookFpuControlWord;
    t.mxcsr_post = g_hookMxcsr;
    t.hook_calls = ssbhook::Calls();
    CheckFpu(t.cw_post, t.mxcsr_post);
    Capture(true, (uint32_t)ctx.ebx);
}

inline void Uninstall() {
    g_postHook = {};
    Close();
    g_regions.clear();
    g_state = nullptr;
}

// Off unless ssb_trace_regions.txt exists. Optional: a failure only logs.
inline bool Install(GameAddresses& addr, TasSharedState* state) {
    g_dir = ssbhook::DllDir();
    FILE* f = nullptr;
    if (fopen_s(&f, (g_dir + "ssb_trace_regions.txt").c_str(), "rb") || !f) return true;
    char line[512];
    std::vector<Region> regions;
    std::string text;
    uint32_t at = 0;
    bool ok = true;
    while (fgets(line, sizeof line, f)) {
        text += line;
        std::string l = line;
        if (l.find_first_not_of(" \t\r\n") == std::string::npos || l[l.find_first_not_of(" \t")] == '#') continue;
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
        Log("trace: ssb_trace_regions.txt unusable (bad lines, none, or over 32); tracing off");
        return false;
    }
    static constexpr uint8_t kEpilogue[] = { 0x5F, 0x5E, 0x5B, 0x83, 0xC4, 0x18, 0xC3 };  // pop edi/esi/ebx; add esp,18; ret
    auto site = (uint8_t*)addr.sg + 0x84C43;
    if (!GameAddresses::ValidateCode("Supreme_Game.dll+0x84C43 (player tick epilogue)", site, kEpilogue)) return false;
    g_state = state;
    g_regions = std::move(regions);
    g_regionsText = text;
    g_regionBytes = at;
    g_rec.assign(sizeof(TickRecord) + at, 0);
    ModuleLines();   // hash what is loaded now, off the game thread
    g_postHook = CreateMidHook<PostHook>(site);
    if (!g_postHook) {
        Log("trace: post hook FAILED; tracing off");
        g_regions.clear();
        return false;
    }
    Log(std::format("trace: on, {} regions, {} bytes per tick, into {}traces", g_regions.size(),
                    g_rec.size(), g_dir));
    return true;
}

} // namespace trace
