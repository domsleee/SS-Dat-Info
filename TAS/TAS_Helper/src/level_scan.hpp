#pragma once
#include <windows.h>
#include <atomic>
#include <cstdint>
#include <cstddef>
#include "shared_state.hpp"
#include "caves/cave2.hpp"
#include "renderer_info.hpp"
#include "caves/menu_state.hpp"
#include "setup_object.hpp"
#include "rider_identity.hpp"
#include "level_path_parse.hpp"
#include <cstring>
#include <climits>

// In-process level detection.
//
// The level identity is read from the GAME-SETUP OBJECT (setup_object.hpp): the
// menu's selection, reached through the stable [[player_base]+0x540] chain
// cave2 already uses for input. It names the area AND the difficulty as plain
// strings, so there is NO heap scan any more (it used to walk tens of MiB per
// scan, about 165 ms, and only ever answered the difficulty; the object
// answers both in ~0 ms and settles Village Easy vs Village Hard, which the
// shared path asset cannot). The level-path string below is still the
// level-CHANGE event and an area cross-check. Runs on a background thread,
// publishing into s->level_id only while a level is running (game_in_game +
// cycle heartbeat):
//   0..9 = area*3 + difficulty  (area 0=Forest,1=Alpine,2=Village,3=Practice;
//          Practice has only Easy on disk, so of the practice ids only 9 can
//          ever tally - 10/11 have no strings to match;
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
//   the PATH gives the change event and an area cross-check (areaFromPath),
//   the game-setup object gives the AUTHORITATIVE area + difficulty.
// If the two areas disagree we are mid-switch, so nothing is published.
//
// KNOWN LIMIT: because some tracks share a path, switching BETWEEN two such
// tracks (e.g. Village Easy <-> Village Hard, both ".../village/Tracks/easy/")
// produces NO string change and therefore no change event. The periodic rescan
// still corrects level_id within a cadence, but no "resolving" state is entered
// for that transition. Detecting it needs a per-track signal the path cannot
// give.
inline uint32_t g_levelPathPtrAddr = 0;
// GameAddresses::player_base (SG+0x1D5450): head of the setup-object chain
// [[player_base]+0x540]. Set by Start(); the difficulty and the rider stance
// are read from that object instead of scanning the heap.
inline uint32_t g_playerBaseAddr = 0;
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

// ENGINE-CYCLE HEARTBEAT (cave2's g_lastCycleMs, passed in by main.cc so this
// header stays free of the cave/safetyhook includes).
//
// This is the signal the whole design was missing, and it took a live probe at
// the menu to see it. Returning to the menu does NOT tear the level down:
// measured at the Arcade menu after quitting Village Easy, the level-path
// pointer still read ".../village/Tracks/easy/...", the engine root was still
// non-NULL, and the game's own "in a level" flag still read 1. Nothing we were
// watching changed, so the UI went on confidently asserting the track the player
// had already left — which is the bug this whole line of work started from.
//
// What DOES change is that Supreme::Cycle STOPS at static menus, at the pause
// menu, and for the whole of a level load. So a frozen cycle means "the engine
// is not running a level right now", and that is exactly when our identification
// must not be trusted: the heap still holds the old track's strings, so even a
// rescan would confirm the wrong answer.
//
// 400ms matches tas_ui's "In Game" chip, which asks the same question of the
// same heartbeat. Active gameplay ticks every ~7ms, so the margin is enormous.
inline volatile uint32_t* g_cycleMs = nullptr;
static const uint32_t CYCLE_FROZEN_MS = 400;

// Is the engine cycle stopped? False when no heartbeat was wired, so a build
// that does not pass one behaves exactly as before rather than seizing up.
static bool cycleFrozen() {
    if (!g_cycleMs) return false;
    return (GetTickCount() - *g_cycleMs) > CYCLE_FROZEN_MS;
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
// Steady-state scan period (ms). The heap walk touches tens of MiB; on the
// 2026-09-02 measurement the game thread used ~6x more CPU while this worker
// walked the heap every 1.5 s (cache thrash), so the cadence is tunable:
// TAS_LEVELSCAN_PERIOD_MS (0 = edge-only, else clamped 500..600000). Measured 2026-09-02 on a 60 s
// coast: 1.5 s cadence = game thread 9.0% of a core, worker 1.5%; 10 s cadence
// = game thread 1.6% (the scan-off baseline is 1.5%), worker 1.3%. Each
// scan is ~165 ms wall. Level changes are still caught fast: the epoch /
// root change drops to the 200 ms cadence until resolved.
// Every way into a level (a load) and out of one (the menu, the pause menu)
// freezes Supreme::Cycle, and the worker already unresolves on the freeze and
// rescans on the resume - so in steady state the heap walk is only a safety
// net against a wrong or missed edge. 0 = edge-only (never rescan while
// resolved); default 60 s.
static uint32_t steadyPeriodMs() {
    static uint32_t s_period = 0xFFFFFFFFu;
    if (s_period == 0xFFFFFFFFu) {
        // Parse strictly: digits only, else the default. A too-long value
        // leaves `buf` empty (GetEnvironmentVariable returns the needed size)
        // and a negative one used to wrap to the maximum - both now fall back.
        char buf[16] = {};
        uint32_t v = 60000;
        const DWORD n = GetEnvironmentVariableA("TAS_LEVELSCAN_PERIOD_MS", buf, sizeof buf);
        if (n > 0 && n < sizeof buf) {
            char* end = nullptr;
            const unsigned long parsed = strtoul(buf, &end, 10);
            if (end && end != buf && *end == 0 && parsed <= 0xFFFFFFFFul) v = (uint32_t)parsed;
            else Log("Level scan: TAS_LEVELSCAN_PERIOD_MS is not a number - using the default");
        }
        if (v != 0 && v < 500) v = 500;
        if (v > 600000) v = 600000;
        s_period = v;
        if (v) Log(std::format("Level scan: steady-state safety-net period {} ms (rescans also on every cycle freeze/resume edge)", v));
        else   Log("Level scan: edge-only (TAS_LEVELSCAN_PERIOD_MS=0) - rescans only on a cycle freeze/resume edge");
    }
    return s_period;
}

// Cumulative scan cost, logged about once a minute.
static void noteScanCost(LARGE_INTEGER t0, LARGE_INTEGER t1) {
    static LARGE_INTEGER freq = {};
    static double totalMs = 0, maxMs = 0;
    static uint32_t scans = 0;
    static ULONGLONG lastLog = 0;
    if (!freq.QuadPart) QueryPerformanceFrequency(&freq);
    const double ms = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
    totalMs += ms;
    if (ms > maxMs) maxMs = ms;
    scans++;
    const ULONGLONG now = GetTickCount64();
    if (now - lastLog >= 60000) {
        lastLog = now;
        Log(std::format("Level scan: {} scans so far, avg {:.1f} ms, max {:.1f} ms each (worker thread)", scans, totalMs / scans, maxMs));
    }
}

// Identify the level from the game-setup object (setup_object.hpp) - the
// authoritative menu selection, read through the stable [[player_base]+0x540]
// chain with NO heap walk. It names the area AND the difficulty, so it settles
// the one pair the path asset cannot (Village Easy vs Village Hard both load
// ".../village/Tracks/easy/"). `areaHint` (from the reliable path) is a
// cross-check: if the object disagrees with it we are mid-switch, so report
// nothing rather than a torn answer.
//
// The AREA comes from the path (areaHint, reliable); the game-setup object
// supplies the DIFFICULTY, which the path cannot (it points at the shared
// easy/ shadow asset). Practice resolves from the path alone - it skips the
// menu screen that writes the setup object, so the object holds a stale Arcade
// selection there (this is the case the pure LevelIdFrom guards; see it and
// its unit tests). Keeps scanLevelId's contract: returns area*3+diff
// (Practice = 9) or -1, with *outBest set so confidentEnough() passes.
static int32_t scanLevelId(int* outBest, int* outSecond, int areaHint) {
    if (outBest) *outBest = 0;
    if (outSecond) *outSecond = 0;
    if (areaHint < 0) return -1;   // no area from the path => nothing to identify
    gamesetup::Setup setup;
    // Practice (area 3) needs no setup object; every other area needs its
    // difficulty. A failed read leaves empty strings, which LevelIdFrom rejects
    // for the non-practice areas and ignores for practice.
    gamesetup::Read(g_playerBaseAddr, &setup);
    const int id = levelpath::LevelIdFrom(areaHint, setup.area, setup.difficulty);
    if (id < 0) return -1;
    if (outBest) *outBest = MIN_TRACK_HITS;  // a clean read is fully confident
    return id;
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

        // THE ENGINE IS NOT RUNNING A LEVEL. Static menu, pause menu, or a load
        // in progress — see cycleFrozen(). Give up the identification for as
        // long as it lasts.
        //
        // This is the only mechanism that catches a return to the menu, because
        // NOTHING ELSE CHANGES there: probed live, the level-path pointer, the
        // engine root and the game's own in-a-level flag all still describe the
        // track the player just left. It is also the only thing that catches a
        // switch between two tracks that SHARE a path (Village Easy and Village
        // Hard both load ".../village/Tracks/easy/..."), since for those the
        // path never changes at all and there is no other event to hang off.
        //
        // Scanning is suppressed too, not just publication. At a menu the old
        // level's strings are still resident and dominant, so a scan here would
        // "confirm" the track we just left and immediately undo this.
        bool frozen = cycleFrozen();
        if (frozen) {
            // STOP must have an out-of-cycle consumer: leaving a level freezes
            // Supreme::Cycle, so waiting for cave2 to consume the command is a
            // deadlock. This worker remains alive at menus and applies only
            // raw/shared cleanup; observer callbacks are deferred to cave2.
            TryProcessStopCommand(s, false);
            if (s->level_scan_epoch == s->level_epoch) {
                publishContext(s, [&] {
                    s->level_id = 0xFFFFFFFFu;
                    s->level_scan_epoch = s->level_epoch - 1u;   // -> unresolved
                });
            }
            // Require a fresh tick before believing a scan again: the cycle
            // resuming is what proves a level is actually running (and, after a
            // load, that it finished).
            g_awaitingCycleTick = true;
            g_frameAtEpochBump = s->frame_count;
        }
        // EDGE: the cycle resuming after a freeze (a level load, a menu trip)
        // is what triggers the rescan - the periodic rescan is only a safety net.
        static bool s_wasFrozen = false;
        static ULONGLONG s_frozenSinceMs = 0;
        if (frozen && !s_wasFrozen) s_frozenSinceMs = GetTickCount64();
        static bool s_rescanOnResume = false;
        if (s_wasFrozen && !frozen) s_rescanOnResume = true;
        s_wasFrozen = frozen;

        bool mayScan = s->game_in_game && !g_noLevelPath && !g_awaitingCycleTick && !frozen;
        int scanBest = 0, scanSecond = 0;
        int areaHint = g_lastPath[0] ? areaFromPath(g_lastPath) : -1;
        LARGE_INTEGER scanT0, scanT1;
        QueryPerformanceCounter(&scanT0);
        int32_t id = mayScan ? scanLevelId(&scanBest, &scanSecond, areaHint) : -1;
        QueryPerformanceCounter(&scanT1);
        if (mayScan) noteScanCost(scanT0, scanT1);
        if (mayScan && s_rescanOnResume) {
            s_rescanOnResume = false;
            Log(std::format("Level scan: cycle resumed after a {} ms freeze - rescanned (id {}, hits {}/{})",
                            GetTickCount64() - s_frozenSinceMs, id, scanBest, scanSecond));
        }
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

        // No settle gate (no "wait for two agreeing scans"). It existed to
        // survive a mid-load scan, and the post-tick gate above already removes
        // that window by refusing to scan until the load has demonstrably
        // finished; keeping it only cost the user another ~1.5s of "resolving".
        // The one case that still demands a second opinion — a scan that
        // contradicts what we already published — is handled below, where we
        // know it IS a contradiction rather than guessing that every scan might
        // be one.
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
            bool alreadyResolved = (s->level_scan_epoch == epochAtScan);
            if (alreadyResolved && s->level_id != (uint32_t)id) {
                // SAME CONTEXT, DIFFERENT TRACK. The path did not change, so
                // this is either two tracks sharing one path (Village Easy and
                // Village Hard both load ".../village/Tracks/easy/...") or a
                // scan that was wrong — and from here the two are
                // indistinguishable. What is certain is that the id we are about
                // to publish CONTRADICTS the one we already published, so at
                // least one of them is false.
                //
                // Go unresolved and let the next scan, taken entirely inside
                // this context, decide. Overwriting one confident answer with
                // another would hand the UI a fresh wrong track just as readily
                // as a fresh right one; unknown is the only honest state when
                // the evidence disagrees with itself. Costs ~200ms (the
                // unresolved cadence) in the rare case, and nothing otherwise.
                // epochAtScan == s->level_epoch here (that is what the outer
                // branch established, and this thread is the only writer), so
                // this makes the pair unequal = unresolved.
                publishContext(s, [&] {
                    s->level_id = 0xFFFFFFFFu;
                    s->level_scan_epoch = epochAtScan - 1u;
                });
            } else if (s->level_id != (uint32_t)id || s->level_scan_epoch != epochAtScan) {
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
        // Steady cadence = the safety-net period (0 = sleep until an edge);
        // 200 ms while unresolved. The sleep is EDGE-AWARE: a context change
        // (new level path) or a cycle freeze (load / menu) ends it at once, so
        // the period bounds only the safety-net rescan, never the latency of a
        // real level change.
        const uint32_t period = steadyPeriodMs();
        const int slices = resolved ? (period ? (int)(period / 100u) : INT_MAX) : 2;
        const uint32_t epochAtSleep = s->level_epoch;
        for (int i = 0; i < slices && !g_stop.load(std::memory_order_relaxed); i++) {
            Sleep(100);
            pollLevelContext(s);
            renderer::Refresh(s);
            rider::Refresh(s, g_playerBaseAddr);
            menustate::RefreshMenu();   // menu cursor + item document (no-op unless at a menu)
            if (cycleFrozen()) { TryProcessStopCommand(s, false); break; }
            if (s->level_epoch != epochAtSleep) break;
        }
    }
    return 0;
}

// Spawn the detection thread. Safe to call once during DLL init.
// `cycleMs` is cave2's engine-cycle heartbeat (`&g_lastCycleMs`). Pass nullptr
// only in a diagnostic build; freeze detection and out-of-cycle STOP handling
// then cannot run.
inline void Start(TasSharedState* s, uint32_t levelPathPtrAddr, uint32_t (*readPtr)(uint32_t),
                  volatile uint32_t* cycleMs, uint32_t playerBaseAddr) {
    g_levelPathPtrAddr = levelPathPtrAddr;
    g_readPtr = readPtr;
    g_cycleMs = cycleMs;
    g_playerBaseAddr = playerBaseAddr;
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
// unloading DLL's code. A timeout is a failed stop, not permission to discard
// the only handle that can prove the worker is gone.
inline bool Stop(DWORD timeoutMs = 3000) {
    g_stop.store(true, std::memory_order_relaxed);
    if (g_thread) {
        DWORD wait = WaitForSingleObject(g_thread, timeoutMs);
        if (wait != WAIT_OBJECT_0) {
            Log(std::format("Level scan: worker did not stop (wait={}, error={}); "
                            "retaining thread handle and shared state",
                            wait, wait == WAIT_FAILED ? GetLastError() : 0));
            return false;
        }
        CloseHandle(g_thread);
        g_thread = nullptr;
    }
    return true;
}

} // namespace levelscan
