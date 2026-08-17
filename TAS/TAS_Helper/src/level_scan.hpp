#pragma once
#include <windows.h>
#include <atomic>
#include <cstdint>
#include <cstddef>
#include "shared_state.hpp"

// In-process level detection.
//
// There is no stable fixed-address anchor for the current track (the engine's
// level identity is heap-only and the obvious pointers are transient/reused —
// see the long RE notes). What IS reliable: while a level is loaded, the heap
// holds resource path strings ".../<Area>/Tracks/<Difficulty>/..." with the
// correct area AND difficulty. So we scan the heap and majority-vote, which
// shrugs off outliers like the shared shadow.qua that always lives under easy/.
//
// Unlike the tas_ui version, this runs INSIDE the game process, so reads are
// direct (no ReadProcessMemory) — much cheaper. It runs on a background thread
// (the scan walks tens of MiB; must not block the game's main thread) and only
// while game_in_game == 1, publishing the result into s->level_id:
//   0..8 = area*3 + difficulty  (area 0=Forest,1=Alpine,2=Village;
//                                 diff 0=Easy,1=Medium,2=Hard)
//   0xFFFFFFFF = unknown / in the menu.
namespace levelscan {

inline std::atomic<bool> g_stop{ false };
inline HANDLE g_thread = nullptr;

// Address of the engine's root pointer ([SG+0x1D5450]) and the last value seen,
// for the level-context epoch. Polled HERE rather than from the Cave2 hook
// because Supreme::Cycle FREEZES at static menus, dialogs and LEVEL LOADS
// (frame_limit.hpp:43-47) — i.e. exactly across the transition we need to
// detect. A cycle-driven bump would not land until the new level's first tick,
// leaving the old track asserted for the whole menu + load. This thread runs
// independently of the cycle, so it sees the swap as it happens.
inline uint32_t g_rootPtrAddr = 0;
inline uint32_t g_lastRoot = 0;
// SEH-guarded pointer read, supplied by the caller (cave2's SafeReadPtr) so this
// header does not depend on the cave headers.
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// Bump the context epoch if the root was reallocated. root==0 is mid-teardown
// and is NOT a change (restarts pass through it transiently) — same rule the
// armed-mode auto-stop uses.
static void pollLevelContext(TasSharedState* s) {
    if (!g_rootPtrAddr || !g_readPtr) return;
    uint32_t cur = g_readPtr(g_rootPtrAddr);
    if (!cur) return;
    if (g_lastRoot && cur != g_lastRoot) {
        s->level_epoch++;
    }
    g_lastRoot = cur;
}

static const char* AREAS[3] = { "forest", "alpine", "village" };
static const char* DIFFS[3] = { "easy", "medium", "hard" };

// Case-insensitive match of [p, p+n) against one of `count` lowercase needles.
static int matchOne(const char* p, size_t n, const char* const* table, int count) {
    for (int t = 0; t < count; t++) {
        const char* w = table[t];
        size_t wl = 0;
        while (w[wl]) wl++;
        if (wl != n) continue;
        bool ok = true;
        for (size_t i = 0; i < n; i++) {
            char c = p[i];
            if (c >= 'A' && c <= 'Z') c = (char)(c + 32);
            if (c != w[i]) { ok = false; break; }
        }
        if (ok) return t;
    }
    return -1;
}

// Tally "<area>/Tracks/<diff>" occurrences. Anchors on "racks/" (the always-
// lowercase core of Tracks/tracks), then case-folds the short area/diff segments.
static void scanRegion(const uint8_t* p, size_t n, int tally[9]) {
    if (n < 8) return;
    for (size_t i = 0; i + 6 < n; i++) {
        if (p[i] == 'r' && p[i + 1] == 'a' && p[i + 2] == 'c' &&
            p[i + 3] == 'k' && p[i + 4] == 's' && p[i + 5] == '/') {
            if (i < 2) continue;
            char tc = (char)p[i - 1];
            if (tc != 'T' && tc != 't') continue;
            if (p[i - 2] != '/' && p[i - 2] != '\\') continue;
            size_t aend = i - 2;
            size_t astart = 0;
            for (size_t j = aend; j > 0; j--) {
                if (p[j - 1] == '/' || p[j - 1] == '\\') { astart = j; break; }
            }
            size_t dstart = i + 6;
            size_t dend = n;
            for (size_t j = dstart; j < n; j++) {
                if (p[j] == '/' || p[j] == '\\') { dend = j; break; }
            }
            int ai = matchOne((const char*)(p + astart), aend - astart, AREAS, 3);
            int di = matchOne((const char*)(p + dstart), dend - dstart, DIFFS, 3);
            if (ai >= 0 && di >= 0) tally[ai * 3 + di]++;
        }
    }
}

// Walk private committed regions <=4 MiB (path strings live in the small-block
// heap; the big texture/geometry buffers hold no paths), SEH-guarded. Returns
// the majority track index 0..8, or -1 if nothing found.
static int32_t scanLevelId() {
    int tally[9] = { 0 };
    uint8_t* addr = nullptr;
    const uint8_t* MAXADDR = (const uint8_t*)0x7FFF0000u;
    MEMORY_BASIC_INFORMATION mbi;
    while (addr < MAXADDR) {
        if (VirtualQuery(addr, &mbi, sizeof(mbi)) == 0) break;
        uint8_t* next = (uint8_t*)mbi.BaseAddress + mbi.RegionSize;
        bool readable = mbi.State == MEM_COMMIT
            && mbi.Type == MEM_PRIVATE
            && mbi.RegionSize <= (4u << 20)
            && !(mbi.Protect & PAGE_GUARD)
            && !(mbi.Protect & PAGE_NOACCESS);
        if (readable && mbi.RegionSize > 0) {
            __try {
                scanRegion((const uint8_t*)mbi.BaseAddress, mbi.RegionSize, tally);
            } __except (EXCEPTION_EXECUTE_HANDLER) {}
        }
        if (next <= addr) break;
        addr = next;
    }
    int best = -1, bestc = 0;
    for (int i = 0; i < 9; i++) {
        if (tally[i] > bestc) { bestc = tally[i]; best = i; }
    }
    return (bestc > 0) ? best : -1;
}

static DWORD WINAPI threadProc(LPVOID param) {
    TasSharedState* s = (TasSharedState*)param;
    while (!g_stop) {
        // Sample the context epoch BEFORE scanning: if the level is swapped
        // mid-scan, the result belongs to the old context and must not be
        // published as if it described the new one.
        uint32_t epochAtScan = s->level_epoch;
        int32_t id = s->game_in_game ? scanLevelId() : -1;

        if (id >= 0) {
            s->level_id = (uint32_t)id;
            s->level_scan_epoch = epochAtScan;
        } else if (epochAtScan != s->level_scan_epoch) {
            // A context we have not identified yet — stay explicitly unknown.
            s->level_id = 0xFFFFFFFFu;
        }
        // else: same context, scan found nothing. The level CANNOT have changed
        // without the root changing, so this is a transient miss (a >4 MiB or
        // guarded region skipped, a faulting region swallowed, a VirtualQuery
        // failure ending the walk early). Keep the last known level rather than
        // flapping to unknown — the whole point of the epoch is that only a root
        // change may invalidate an identification.
        // Re-scan periodically: game_in_game stays 1 across the post-race
        // submenus, so the 0->1 edge alone would miss track changes within a
        // session. ~1.5 s keeps the chip fresh at negligible cost. Sleep in
        // short slices so Stop() can join promptly (else detach waits ~1.5s).
        // Poll the level context on every 100 ms slice, not once per scan: the
        // whole point is to notice a swap DURING the menu/load, and the scan
        // itself only runs every ~1.5s.
        for (int i = 0; i < 15 && !g_stop.load(std::memory_order_relaxed); i++) {
            Sleep(100);
            pollLevelContext(s);
        }
    }
    return 0;
}

// Spawn the detection thread. Safe to call once during DLL init.
inline void Start(TasSharedState* s, uint32_t rootPtrAddr, uint32_t (*readPtr)(uint32_t)) {
    g_rootPtrAddr = rootPtrAddr;
    g_readPtr = readPtr;
    g_lastRoot = 0;
    if (!s) return;
    s->level_id = 0xFFFFFFFFu;
    // Reinjection can happen while a level is already loaded, and shared memory
    // survives it — so a stale level_scan_epoch could equal level_epoch and make
    // the cleared level_id read as a trustworthy "we are at the menu". Force the
    // pair unequal so this reads as "not identified in this context YET", which
    // is the truth until the first scan of this DLL instance completes.
    s->level_scan_epoch = s->level_epoch - 1u;
    g_stop.store(false, std::memory_order_relaxed);
    g_thread = CreateThread(nullptr, 0, threadProc, s, 0, nullptr);
}

// Signal the worker and JOIN it before the caller tears down shared memory.
// Without the join the worker could write s->level_id through a pointer that
// DLL_PROCESS_DETACH has already unmapped (use-after-free), or resume into the
// unloading DLL's code. Bounded wait so a wedged scan can't hang detach.
inline void Stop() {
    g_stop.store(true, std::memory_order_relaxed);
    if (g_thread) {
        WaitForSingleObject(g_thread, 3000);
        CloseHandle(g_thread);
        g_thread = nullptr;
    }
}

} // namespace levelscan
