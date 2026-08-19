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
// frame_count captured when the context last changed. Supreme::Cycle is FROZEN
// throughout a level load (frame_limit.hpp:43-47), so the cycle ticking again is
// proof the load FINISHED. Scanning before that is what let a mid-load scan see
// the OLD track still dominant and publish it under the NEW epoch — and no
// threshold can catch that, because the reading is stale rather than noisy.
// Measured on this build: stale FE reads 55 while a freshly loaded track reads
// 23, so 55 >= 2*23 passes any dominance ratio. Waiting for the tick removes the
// window instead of trying to out-threshold it.
inline uint32_t g_frameAtEpochBump = 0;
inline bool g_awaitingCycleTick = false;

inline uint32_t g_rootPtrAddr = 0;
inline uint32_t g_lastRoot = 0;
// SEH-guarded pointer read, supplied by the caller (cave2's SafeReadPtr) so this
// header does not depend on the cave headers.
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// An F5 restart passes through root==0 transiently, but a level TEARDOWN leaves
// it null — and at a static menu it can stay null indefinitely. So a null root
// is news once it PERSISTS past anything a restart could produce.
//
// Measured in WALL TIME, not poll counts: polls are not evenly spaced (one
// before the scan, one after, one per 100 ms sleep slice), so at a loop boundary
// several land back-to-back, while a long scanLevelId() can block this thread
// well past 100 ms. Counting polls would make the threshold anywhere from ~200 ms
// to seconds depending on scan timing.
//
// THRESHOLD SET FROM MEASUREMENT, via last_null_root_ms on this build:
//   level load  -> 875 ms of null root
//   F5 restart  -> no null gap observed AT ALL (x3; the value never moved off
//                  the load's 875, so a restart's transient is shorter than the
//                  100 ms poll interval)
// The two are cleanly separated rather than marginally, so this sits well below
// the load and far above anything a restart produces. 500 rather than a value
// hugging 875 specifically to catch a SHORTER-than-measured load: a review
// scenario of "null for 600 ms then the root is reused at the same address"
// would slip past a 750 ms threshold and leave the stale track resolved (the
// ABA hole), and 500 closes it while keeping ~375 ms of margin under the
// measured load.
inline uint32_t g_nullRootSinceMs = 0;   // GetTickCount when the root went null
inline bool     g_nullRootReported = false;
static const uint32_t NULL_ROOT_INVALIDATE_MS = 500;

// True while the engine has no root at all — i.e. no level is loaded. Any track
// strings still in the heap are residue from the level we LEFT, so an
// identification made now would be confidently wrong.
inline bool g_rootIsNull = false;

static void pollLevelContext(TasSharedState* s) {
    if (!g_rootPtrAddr || !g_readPtr) return;
    uint32_t cur = g_readPtr(g_rootPtrAddr);

    if (!cur) {
        g_rootIsNull = true;
        uint32_t now = GetTickCount();
        if (!g_nullRootSinceMs) g_nullRootSinceMs = now ? now : 1;
        if (!g_nullRootReported &&
            (now - g_nullRootSinceMs) >= NULL_ROOT_INVALIDATE_MS && g_lastRoot) {
            // Sustained teardown: we are no longer in the context we identified.
            s->level_epoch++;
            g_frameAtEpochBump = s->frame_count;
            g_awaitingCycleTick = true;
            g_nullRootReported = true;
            // NOTE: g_lastRoot is deliberately NOT cleared. Clearing it made the
            // `g_lastRoot &&` guard below false, so the NEXT root — the new
            // level — produced no bump at all, and a track identified during the
            // null window stayed formally resolved into the new level. Keeping
            // the old value means the new root still reads as a change. A second
            // bump is harmless; the epoch is a change counter, not a sequence.
        }
        return;
    }

    g_rootIsNull = false;
    if (g_nullRootSinceMs) {
        s->last_null_root_ms = GetTickCount() - g_nullRootSinceMs;
    }
    g_nullRootSinceMs = 0;
    g_nullRootReported = false;
    if (g_lastRoot && cur != g_lastRoot) {
        s->level_epoch++;
        g_frameAtEpochBump = s->frame_count;
        g_awaitingCycleTick = true;
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
// Detection confidence floor. A loaded track measures 23-24 hits with a
// runner-up of 0 (sampled repeatedly on Forest Easy, stable across restarts).
// Residue from a level already left is far sparser, so a floor well under the
// observed legitimate count rejects it without risking a real detection.
static const int MIN_TRACK_HITS = 8;
// ...and the winner must clearly dominate. Guards the mid-load window where the
// OLD track's strings are still resident alongside the new one's.
static const int MIN_DOMINANCE_NUM = 2;

static bool confidentEnough(int best, int second) {
    return best >= MIN_TRACK_HITS && best >= second * MIN_DOMINANCE_NUM;
}

// Require the SAME id from two consecutive scans in the SAME context before
// publishing it.
//
// A confidence floor alone cannot fix the worst case: during a level load the
// root for the new level already exists (so the epoch has moved and we are
// scanning) while the heap still holds the OLD track's strings in force. That
// scan legitimately sees the old track with a high, dominant count and would
// publish it stamped with the NEW epoch — confidently wrong, and no amount of
// thresholding detects it because the reading is not noisy, just stale.
//
// Waiting for the reading to SETTLE does detect it: across a load the tally
// shifts old -> new, so two consecutive scans disagree until the new level's
// resources are actually resident. Costs one scan interval (~1.5s) of
// "resolving" after a change, which is the honest answer during a load anyway.
inline int32_t g_pendingId = -1;
inline uint32_t g_pendingEpoch = 0;

static bool confirms(int32_t id, uint32_t epoch) {
    bool agrees = (id == g_pendingId && epoch == g_pendingEpoch);
    g_pendingId = id;
    g_pendingEpoch = epoch;
    return agrees;
}

static int32_t scanLevelId(int* outBest, int* outSecond) {
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
    // Confidence, not just a winner. The old code took the largest tally with
    // NO minimum and NO margin, so a SINGLE residual string from a level we had
    // already left could win and be published as the current track — "fresh but
    // wrong", which the epoch cannot detect because the epoch only certifies
    // WHEN a scan ran, never WHETHER it was right.
    //
    // A genuinely loaded level references its resource paths pervasively; heap
    // residue from a previous one is sparse. So report the top two counts and
    // let the caller apply the thresholds.
    int best = -1, bestc = 0, secondc = 0;
    for (int i = 0; i < 9; i++) {
        if (tally[i] > bestc) { secondc = bestc; bestc = tally[i]; best = i; }
        else if (tally[i] > secondc) { secondc = tally[i]; }
    }
    if (outBest) *outBest = bestc;
    if (outSecond) *outSecond = secondc;
    return (bestc > 0) ? best : -1;
}

static DWORD WINAPI threadProc(LPVOID param) {
    TasSharedState* s = (TasSharedState*)param;
    while (!g_stop) {
        // Poll BEFORE sampling the epoch, so a swap that already happened is
        // reflected in the epoch this scan will be stamped with.
        pollLevelContext(s);
        uint32_t epochAtScan = s->level_epoch;
        // Refuse to identify anything while there is NO ROOT. game_in_game is
        // not enough: it is only written by the Cycle hook, so it stays stale at
        // 1 through a menu/teardown. With no root there is no level, and the
        // track strings still in the heap are residue from the one we LEFT — a
        // scan here would publish the OLD track stamped with the NEW epoch, i.e.
        // confidently wrong, which is worse than unresolved.
        // Wait for the cycle to tick AFTER a context change before scanning at
        // all: the cycle is frozen for the whole load, so a tick proves the load
        // finished and the new track's resources are resident. Without this the
        // scan runs mid-load, when the OLD track's strings are still dominant.
        if (g_awaitingCycleTick && s->frame_count != g_frameAtEpochBump) {
            g_awaitingCycleTick = false;
        }
        bool mayScan = s->game_in_game && !g_rootIsNull && !g_awaitingCycleTick;
        int scanBest = 0, scanSecond = 0;
        int32_t id = mayScan ? scanLevelId(&scanBest, &scanSecond) : -1;
        s->level_scan_best_hits = (uint32_t)scanBest;
        s->level_scan_second_hits = (uint32_t)scanSecond;
        // ...and again AFTER, because scanLevelId() walks tens of MiB and the
        // level can be swapped underneath it. Without this the result of a scan
        // that straddled the swap would be published as if it described the new
        // context — stale id, fresh epoch, and the UI would trust it.
        pollLevelContext(s);

        // Update the pending candidate UNCONDITIONALLY. Folding this into the
        // publish condition with && short-circuited it: a low-confidence scan
        // never reset the pending id, so stale -> unconfident -> stale counted as
        // two "consecutive" agreeing scans when it was nothing of the sort.
        bool confident = (id >= 0) && confidentEnough(scanBest, scanSecond);
        bool settled = confirms(confident ? id : -1, epochAtScan) && confident;

        if (s->level_epoch != epochAtScan) {
            // The context moved under the scan: whatever we found describes the
            // level we just left. Discard it and stay unresolved — level_scan_epoch
            // is deliberately NOT advanced, so resolved stays false until a scan
            // completes entirely inside one context.
            s->level_id = 0xFFFFFFFFu;
        } else if (settled && id >= 0) {
            // Publish the id BEFORE the marker that validates it: a reader that
            // sees the new scan epoch must already be able to see the id it
            // describes, never the previous one.
            //
            // The barrier is load-bearing, not decoration. These are plain u32s
            // in a shared mapping; without it the compiler (and, in principle,
            // the store buffer) may make the marker visible to the OTHER PROCESS
            // before the id it validates, handing the reader "resolved" plus the
            // previous track. Volatile reads on the Rust side prevent load
            // elision but establish no ordering with this writer — the release
            // has to come from here.
            s->level_id = (uint32_t)id;
            MemoryBarrier();
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
        // ADAPTIVE CADENCE. 1.5s is right for steady state — the track cannot
        // change without us noticing via the root — but it is the wrong latency
        // while we do not know where we are. Unresolved cost the user ~3s after
        // a level finished loading: up to 1.5s to reach the next loop, then
        // another 1.5s for the second scan the settle gate needs.
        //
        // Polling fast while unresolved is close to free: scanLevelId only runs
        // when mayScan holds (root present, cycle ticked), so at a menu or
        // mid-teardown this just re-reads one pointer. The expensive heap walk
        // happens only when a level is actually there to identify — precisely
        // when the latency matters.
        bool resolved = (s->level_scan_epoch == s->level_epoch);
        int slices = resolved ? 15 : 2;   // 1.5s steady, 200ms while resolving
        for (int i = 0; i < slices && !g_stop.load(std::memory_order_relaxed); i++) {
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
    if (!g_thread) {
        // Leave the epochs UNEQUAL. With no scanner the track is genuinely
        // unknowable, and unresolved is the honest encoding of that. Restoring
        // equality here (an earlier attempt) made resolved_level_id() return
        // Some(0xFFFFFFFF) — "resolved, at the menu" — forever, which is a
        // confident lie. The caller logs the failure.
    }
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
