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

// In-process level detection, plus the DLL's background housekeeping.
//
// The level identity is read from the GAME-SETUP OBJECT (setup_object.hpp): the
// menu's selection, reached through the executable's own config pointer chain.
// It names the area AND the difficulty as plain strings (and so settles
// Village Easy vs Village Hard, which share a path asset). The level-path
// string below is the level-CHANGE event and an area cross-check. Runs on a
// background thread, publishing into s->level_id only while a level is running
// (game_in_game + cycle heartbeat):
//   0..9 = area*3 + difficulty  (area 0=Forest,1=Alpine,2=Village,3=Practice;
//                                 diff 0=Easy,1=Medium,2=Hard; Practice has
//                                 only Easy on disk, so only 9 occurs)
//   0xFFFFFFFF = unknown / in the menu.
//
// The same worker is the out-of-cycle STOP consumer, retires a stale
// cont_suppress_input, refreshes the renderer/rider stamps, runs the menu
// housekeeping and flushes cave2's deferred log lines.
namespace levelscan {

inline std::atomic<bool> g_stop{ false };
inline HANDLE g_thread = nullptr;

// LEVEL CONTEXT = the engine's own level-path string.
//
// `[SG+0x1D3304]` points at the CURRENT level's resource path (verified on four
// tracks). The string changes the instant a level loads, so a change IS the
// level-change event — direct, immediate, and requiring no inference. (The
// root pointer [SG+0x1D5450] does NOT change on a track switch, and the
// level's files are not opened through CreateFile after injection, so neither
// of those can serve as the event.)
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
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// frame_count at the last context change. Supreme::Cycle is FROZEN for the whole
// level load, so the cycle ticking again is proof the load FINISHED — and
// identifying only after that removes the mid-load window in which the OLD
// level's state is still what a read would return.
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

// Steady-state re-identification period while resolved. Every way into a level
// (a load) and out of one (the menu, the pause menu) freezes Supreme::Cycle,
// and the worker unresolves on the freeze and re-identifies on the resume, so
// this is only a safety net against a missed edge. The read costs ~0 ms.
static const uint32_t STEADY_PERIOD_MS = 60000;

// A cycle frozen this long with cont_suppress_input still set means the flag is
// STALE: a CONT's reload freeze lasts a second or two, so a judged cycle that
// never tore down (crashed harness, session end, quit-to-menu mid-cycle) left
// it behind. Left alone, the input gate swallows every non-ESC key until
// reinjection.
static const uint32_t STALE_SUPPRESS_MS = 5000;

// Identify the level from the game-setup object (setup_object.hpp) - the
// authoritative menu selection, read through the executable config pointer
// chain. The AREA comes from the path (areaHint, reliable); the game-setup
// object supplies the DIFFICULTY, which the path cannot (it points at the
// shared easy/ shadow asset). If the object's area disagrees with the path we
// are mid-switch, so nothing is reported. Practice resolves from the path
// alone - it skips the menu screen that writes the setup object, so the object
// holds a stale Arcade selection there (the case levelpath::LevelIdFrom
// guards; see its unit tests). Returns area*3+diff (Practice = 9) or -1.
static int32_t scanLevelId(int areaHint) {
    if (areaHint < 0) return -1;   // no area from the path => nothing to identify
    gamesetup::Setup setup;
    // Practice (area 3) needs no setup object; every other area needs its
    // difficulty. A failed read leaves empty strings, which LevelIdFrom rejects
    // for the non-practice areas and ignores for practice.
    gamesetup::Read(&setup);
    return levelpath::LevelIdFrom(areaHint, setup.area, setup.difficulty);
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
            if (s->cont_suppress_input && (GetTickCount() - *g_cycleMs) > STALE_SUPPRESS_MS) {
                s->cont_suppress_input = 0;
                Log("Level scan: retired a stale cont_suppress_input (cycle frozen >5 s)");
            }
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
        int areaHint = g_lastPath[0] ? areaFromPath(g_lastPath) : -1;
        int32_t id = mayScan ? scanLevelId(areaHint) : -1;
        if (mayScan && s_rescanOnResume) {
            s_rescanOnResume = false;
            Log(std::format("Level scan: cycle resumed after a {} ms freeze - re-identified (id {})",
                            GetTickCount64() - s_frozenSinceMs, id));
        }
        // ...and again AFTER, so a level swapped underneath the read is not
        // published as if the result described the new context.
        pollLevelContext(s);

        bool settled = (id >= 0);

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
        // else: same context, nothing identified. The level cannot have changed
        // without the path changing, so keep the last known level rather than
        // flapping to unknown.

        // Cadence: the safety-net period while resolved, 200 ms while
        // unresolved (a read is ~free, and this is when latency matters). The
        // sleep is EDGE-AWARE: a context change (new level path) or a cycle
        // freeze (load / menu) ends it at once. Sliced so Stop() joins promptly.
        bool resolved = (s->level_scan_epoch == s->level_epoch);
        const int slices = resolved ? (int)(STEADY_PERIOD_MS / 100u) : 2;
        const uint32_t epochAtSleep = s->level_epoch;
        for (int i = 0; i < slices && !g_stop.load(std::memory_order_relaxed); i++) {
            Sleep(100);
            FlushPendingLog();           // cave2's mode-transition lines (queued inside the hook)
            pollLevelContext(s);
            renderer::Refresh(s);
            rider::Refresh(s);
            menustate::Housekeeping();   // menu doc: clear when no menu executes, expire stale commands (no UI access)
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
                  volatile uint32_t* cycleMs) {
    g_levelPathPtrAddr = levelPathPtrAddr;
    g_readPtr = readPtr;
    g_cycleMs = cycleMs;
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
