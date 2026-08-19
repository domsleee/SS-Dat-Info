#pragma once
#include <windows.h>
#include <atomic>
#include <cstdint>
#include <cstddef>
#include "shared_state.hpp"
#include "level_path_parse.hpp"
#include <cstring>

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

// LEVEL CONTEXT = the engine's own level-path string.
//
// `[SG+0x1D3304]` points at the CURRENT level's resource path; found by
// differential RE (`tas_test level-hunt ptr`) and verified on four tracks. The
// string changes the instant a level loads, so a change IS the level-change
// event — direct, immediate, and requiring no inference.
//
// This replaces two mechanisms that were MEASURED NOT TO FIRE on a real track
// switch, rather than assumed to work:
//   * the root-pointer epoch ([SG+0x1D5450]) — epoch stayed 0 across a switch,
//     despite cave2's comment claiming the root is reallocated on track changes;
//   * a CreateFileA/W hook — 305 file opens intercepted, none under Data/Levels,
//     because the level's resources are not opened via CreateFile after the DLL
//     is injected.
// With them went the null-root threshold, the ABA reasoning and the settle gate.
//
// The path is RELIABLE FOR AREA but NOT FOR DIFFICULTY — some tracks share the
// easy/ shadow asset, so Village Hard reads ".../Tracks/easy/...". So:
//   the PATH gives the change event AND CONSTRAINS THE AREA (areaFromPath),
//   the majority-voted heap scan picks the DIFFICULTY within that area.
// Both halves are used; the area constraint is not advisory.
//
// KNOWN LIMIT: because some tracks share a path, switching BETWEEN two such
// tracks (e.g. Village Easy <-> Village Hard, both ".../village/Tracks/easy/")
// produces NO string change and therefore no change event. The periodic rescan
// still corrects level_id within a cadence, but no "resolving" state is entered
// for that transition. Detecting it needs a per-track signal the path cannot
// give.
inline uint32_t g_levelPathPtrAddr = 0;
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// frame_count at the last context change. Supreme::Cycle is FROZEN for the whole
// level load (frame_limit.hpp:43-47), so the cycle ticking again is proof the
// load FINISHED — and scanning only after that removes the mid-load window in
// which the OLD track's strings are still resident and dominant. No threshold
// can catch that case: the reading is stale, not noisy.
inline uint32_t g_frameAtEpochBump = 0;
inline bool g_awaitingCycleTick = false;

// Last path we published, for change detection.
inline char g_lastPath[TAS_LEVEL_PATH_MAX] = { 0 };
// True while no level path is available — i.e. no level is loaded, so any track
// strings in the heap are residue from the one we left.
inline bool g_noLevelPath = true;



// Parsing lives in level_path_parse.hpp so it can be unit-tested WITHOUT the
// game. Aliased so the shipped code and the tested code are the same code.
using levelpath::AREAS;
using levelpath::DIFFS;
static int matchOne(const char* p, size_t n, const char* const* table, int count) {
    return levelpath::MatchOne(p, n, table, count);
}

// Read the current level path into `out`. Returns false when unavailable OR not
// a plausible level path.
//
// SEH only contains crashes; it does not give a coherent snapshot. Freed-but-
// still-committed heap does not fault, and an in-place rewrite between the
// length walk and the copy yields a hybrid string. So:
//   * require a NUL WITHIN bounds (127 non-NUL bytes is not a valid path, it is
//     a garbage buffer that happened to be readable);
//   * require the path grammar we actually depend on ("levels" and "tracks"),
//     which rejects transient garbage and unrelated strings;
//   * sample TWICE and require the two to agree, which rejects a torn read.
static bool readLevelPathOnce(char* out, size_t cap) {
    if (!g_levelPathPtrAddr || !g_readPtr) return false;
    uint32_t p = g_readPtr(g_levelPathPtrAddr);
    if (!p) return false;
    __try {
        const char* src = (const char*)p;
        size_t n = 0;
        while (n < cap && src[n]) n++;
        if (n == 0 || n >= cap) return false;   // no NUL in bounds => not a path
        for (size_t i = 0; i < n; i++) out[i] = src[i];
        out[n] = '\0';
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    // Grammar check — see levelpath::IsPlausible, unit-tested standalone.
    return levelpath::IsPlausible(out);
}

static bool readLevelPath(char* out, size_t cap) {
    char a[TAS_LEVEL_PATH_MAX];
    if (!readLevelPathOnce(a, sizeof(a))) return false;
    char b[TAS_LEVEL_PATH_MAX];
    if (!readLevelPathOnce(b, sizeof(b))) return false;
    if (std::strcmp(a, b) != 0) return false;   // torn / mid-rewrite
    size_t n = 0;
    while (n < cap - 1 && a[n]) { out[n] = a[n]; n++; }
    out[n] = '\0';
    return true;
}

// Parsing lives in level_path_parse.hpp so it can be unit-tested WITHOUT the
// game (tests/test_level_path.cpp, run by `just test_dll`). Aliased here so the
// shipped code and the tested code are the same code.
static int areaFromPath(const char* path) { return levelpath::AreaFrom(path); }

// Seqlock writer. Marks the group as being mutated, applies `fn`, then commits.
//
// InterlockedIncrement rather than `++` so the sequence transitions are real
// atomic release/acquire points and cannot be reordered around the payload by
// either the compiler or the store buffer — which a plain store plus one
// MemoryBarrier did not guarantee for a 128-byte array read from another
// process.
// MUST NOT NEST: an inner pair would drive the sequence back to EVEN halfway
// through the outer write, publishing a torn group as if it were stable. Hence
// the *Locked helpers below rather than self-synchronising mutators.
template <typename F>
static void publishContext(TasSharedState* s, F&& fn) {
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> odd: writing
    fn();
    InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);   // -> even: stable
}

// Invalidate the current identification: the context we identified is gone.
// `Locked` = the caller must already be inside publishContext(). The suffix is
// there so a naked call reads as the bug it would be.
static void invalidateContextLocked(TasSharedState* s) {
    s->level_path[0] = '\0';
    s->level_path_gen++;
    // No MemoryBarrier between the path and the epoch any more: the closing
    // InterlockedIncrement is the release for the whole group, and ordering
    // WITHIN the group no longer matters because a reader either sees all of it
    // or rejects the read.
    s->level_epoch++;
}

static void pollLevelContext(TasSharedState* s) {
    char cur[TAS_LEVEL_PATH_MAX];
    if (!readLevelPath(cur, sizeof(cur))) {
        // No level path => no level. This MUST invalidate: leaving the old id
        // resolved through the menu and the load is the original reported bug,
        // and it is the same hole the root-based version had when root==0.
        if (!g_noLevelPath || g_lastPath[0]) {
            g_noLevelPath = true;
            if (g_lastPath[0]) {
                g_lastPath[0] = '\0';
                publishContext(s, [&] { invalidateContextLocked(s); });
                g_frameAtEpochBump = s->frame_count;
                g_awaitingCycleTick = true;
            }
        }
        return;
    }
    g_noLevelPath = false;
    if (std::strcmp(cur, g_lastPath) == 0) return;   // same level, nothing to say

    // The level changed. Publish the new path and the epoch that invalidates the
    // old identification as ONE seqlock write: the path is 128 bytes copied a
    // byte at a time, so a reader without the sequence can catch it spliced
    // between two tracks. Only the shared-memory stores go inside the window —
    // the g_* locals are DLL-private and no reader can see them.
    size_t n = 0;
    while (n < TAS_LEVEL_PATH_MAX - 1 && cur[n]) { g_lastPath[n] = cur[n]; n++; }
    g_lastPath[n] = '\0';
    publishContext(s, [&] {
        for (size_t i = 0; i <= n; i++) s->level_path[i] = g_lastPath[i];
        s->level_path_gen++;
        s->level_epoch++;
    });
    g_frameAtEpochBump = s->frame_count;
    g_awaitingCycleTick = true;
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

// `areaHint` (0..2, or -1) CONSTRAINS the winner to that area.
//
// This is the division of labour the RE actually established: the level-path
// pointer answers AREA reliably, the heap tally answers DIFFICULTY. Previously
// the path was only a change signal and the tally still chose freely among all
// nine ids, so the path's reliable half was being thrown away — and a residue
// winner from a DIFFERENT AREA could beat the real track. Restricting the tally
// to the known area makes that impossible by construction.
static int32_t scanLevelId(int* outBest, int* outSecond, int areaHint) {
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
        // Outside the known area? Cannot be the current track.
        if (areaHint >= 0 && i / 3 != areaHint) continue;
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
        bool mayScan = s->game_in_game && !g_noLevelPath && !g_awaitingCycleTick;
        int scanBest = 0, scanSecond = 0;
        int areaHint = g_lastPath[0] ? areaFromPath(g_lastPath) : -1;
        int32_t id = mayScan ? scanLevelId(&scanBest, &scanSecond, areaHint) : -1;
        // Deliberately OUTSIDE the seqlock. These are diagnostics — how strong
        // the last scan's evidence was — not part of the identity group, and no
        // decision is made from them. Putting them in the window would widen it
        // for nothing. `tas_test status` prints them raw and is the only reader.
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
        // No settle gate any more. It existed to survive a mid-load scan, and the
        // post-tick gate above already removes that window by refusing to scan
        // until the load has demonstrably finished. Requiring a second confirming
        // scan only cost the user another ~1.5s of "resolving".
        bool settled = (id >= 0) && confidentEnough(scanBest, scanSecond);

        // Every one of these mutates the identity half of the group, so every
        // one goes through the seqlock. The pair (level_id, level_scan_epoch) is
        // the whole point: a reader that sees the validating epoch without the
        // id it validates gets "resolved" plus the PREVIOUS track, which is
        // worse than unresolved because it looks trustworthy.
        //
        // Each also publishes only when it would actually CHANGE something. The
        // steady state re-derives the same id every 1.5s, and republishing it
        // opened a write window — and made a reader retry — for no new
        // information. Skipping an identical write is exactly equivalent, and it
        // buys a real invariant: the sequence advances if and only if the level
        // context changed. (We are the sole writer, so reading these fields back
        // to compare is not itself racy.)
        if (s->level_epoch != epochAtScan) {
            // The context moved under the scan: whatever we found describes the
            // level we just left. Discard it and stay unresolved — level_scan_epoch
            // is deliberately NOT advanced, so resolved stays false until a scan
            // completes entirely inside one context.
            if (s->level_id != 0xFFFFFFFFu) {
                publishContext(s, [&] { s->level_id = 0xFFFFFFFFu; });
            }
        } else if (settled && id >= 0) {
            if (s->level_id != (uint32_t)id || s->level_scan_epoch != epochAtScan) {
                publishContext(s, [&] {
                    s->level_id = (uint32_t)id;
                    s->level_scan_epoch = epochAtScan;
                });
            }
        } else if (epochAtScan != s->level_scan_epoch) {
            // A context we have not identified yet — stay explicitly unknown.
            if (s->level_id != 0xFFFFFFFFu) {
                publishContext(s, [&] { s->level_id = 0xFFFFFFFFu; });
            }
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
inline void Start(TasSharedState* s, uint32_t levelPathPtrAddr, uint32_t (*readPtr)(uint32_t)) {
    g_levelPathPtrAddr = levelPathPtrAddr;
    g_readPtr = readPtr;
    g_lastPath[0] = 0;
    if (!s) return;
    // Shared memory SURVIVES reinjection, so a previous DLL instance killed
    // between the two increments leaves the sequence ODD — and every future read
    // in every process would be rejected forever, permanently "resolving". We
    // are the sole writer and nothing is in flight here, so this is the one
    // place that can honestly re-establish an even sequence.
    if (InterlockedOr((volatile LONG*)&s->level_ctx_seq, 0) & 1) {
        InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);
    }
    publishContext(s, [&] {
        s->level_id = 0xFFFFFFFFu;
        // Reinjection can happen while a level is already loaded — so a stale
        // level_scan_epoch could equal level_epoch and make the cleared level_id
        // read as a trustworthy "we are at the menu". Force the pair unequal so
        // this reads as "not identified in this context YET", which is the truth
        // until the first scan of this DLL instance completes.
        s->level_scan_epoch = s->level_epoch - 1u;
    });
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
