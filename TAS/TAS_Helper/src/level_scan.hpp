#pragma once
#include <windows.h>
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

inline volatile bool g_stop = false;

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
        if (s->game_in_game) {
            int32_t id = scanLevelId();
            s->level_id = (id >= 0) ? (uint32_t)id : 0xFFFFFFFFu;
        } else {
            s->level_id = 0xFFFFFFFFu;
        }
        // Re-scan periodically: game_in_game stays 1 across the post-race
        // submenus, so the 0->1 edge alone would miss track changes within a
        // session. ~1.5 s keeps the chip fresh at negligible cost.
        Sleep(1500);
    }
    return 0;
}

// Spawn the detection thread. Safe to call once during DLL init.
inline void Start(TasSharedState* s) {
    if (!s) return;
    s->level_id = 0xFFFFFFFFu;
    g_stop = false;
    HANDLE h = CreateThread(nullptr, 0, threadProc, s, 0, nullptr);
    if (h) CloseHandle(h);
}

inline void Stop() { g_stop = true; }

} // namespace levelscan
