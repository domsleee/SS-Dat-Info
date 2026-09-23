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
// A background thread publishes s->level_id only while a level is running
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

inline HANDLE g_thread = nullptr;

// LEVEL CONTEXT = the engine's own level-path string.
//
// `[SG+0x1D3304]` points at the CURRENT level's resource path. The string
// changes the instant a level loads, so a change IS the level-change event.
// (The root pointer [SG+0x1D5450] does not change on a track switch.)
//
// The path is reliable for AREA but not for DIFFICULTY: some tracks share the
// easy/ asset, so Village Hard reads ".../Tracks/easy/...". So the path gives
// the change event and the area; the game-setup object gives the difficulty.
//
// Known limit: switching between two tracks that share a path (Village Easy <->
// Village Hard) changes no string. The engine-cycle freeze of the switch
// still unresolves and rescans (see below).
inline uint32_t g_levelPathPtrAddr = 0;
inline uint32_t (*g_readPtr)(uint32_t) = nullptr;

// frame_count at the last context change. Supreme::Cycle is frozen for the
// whole level load, so a tick after this proves the load finished; reads
// before it can still return the old level's state.
inline uint32_t g_frameAtEpochBump = 0;
inline bool g_awaitingCycleTick = false;

// Last path we published, for change detection.
inline char g_lastPath[TAS_LEVEL_PATH_MAX] = { 0 };
// True while no level path is available — i.e. no level is loaded, so any track
// strings in the heap are residue from the one we left.
inline bool g_noLevelPath = true;

// ENGINE-CYCLE HEARTBEAT (cave2's g_lastCycleMs, passed in by main.cc).
//
// Returning to the menu does NOT tear the level down: the level path, the
// engine root and the game's "in a level" flag all still describe the track
// just left. What does change is that Supreme::Cycle stops at static menus,
// the pause menu and for a whole level load. So a frozen cycle means "no
// level is running", and the identification must not be trusted then.
//
// 400 ms matches tas_ui's "In Game" chip, which reads the same heartbeat.
// Active gameplay ticks every ~7 ms.
inline volatile uint32_t* g_cycleMs = nullptr;
static const uint32_t CYCLE_FROZEN_MS = 400;

// Is the engine cycle stopped? False when no heartbeat was wired.
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

// Parsing lives in level_path_parse.hpp so it can be unit-tested without the
// game (tests/test_level_path.cpp).
static int areaFromPath(const char* path) { return levelpath::AreaFrom(path); }

// Seqlock writer. Marks the group as being mutated, applies `fn`, then commits.
//
// InterlockedIncrement rather than `++` so the sequence transitions are full
// barriers that neither the compiler nor the CPU can reorder around the
// payload.
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
    // Order within the group does not matter: a reader sees all of it or
    // rejects the read.
    s->level_epoch++;
}

static void pollLevelContext(TasSharedState* s) {
    char cur[TAS_LEVEL_PATH_MAX];
    if (!readLevelPath(cur, sizeof(cur))) {
        // No level path => no level, so the old id must not stay resolved.
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
// stale: a restart's reload freeze lasts a second or two, so a controller that
// never cleared it (crash, quit to menu mid-restart) left it behind. Left
// alone, the input gate swallows every non-ESC key.
static const uint32_t STALE_SUPPRESS_MS = 5000;

// Area from the path, difficulty from the setup object (see the header).
// Disagreeing areas mean a mid-switch, so nothing is reported. Practice
// resolves from the path alone: it skips the menu screen that writes the setup
// object, so the object holds a stale Arcade selection there (guarded by
// levelpath::LevelIdFrom; see its unit tests).
// Returns area*3+diff (Practice = 9) or -1.
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
    for (;;) {
        // Poll BEFORE sampling the epoch, so a swap that already happened is
        // reflected in the epoch this scan will be stamped with.
        pollLevelContext(s);
        uint32_t epochAtScan = s->level_epoch;
        // Wait for a cycle tick after a context change before scanning (see
        // g_frameAtEpochBump).
        if (g_awaitingCycleTick && s->frame_count != g_frameAtEpochBump) {
            g_awaitingCycleTick = false;
        }

        // A frozen cycle means static menu, pause or load: invalidate the
        // identity and do not scan. It is the only signal for a return to the
        // menu and for a switch between two tracks that share a path.
        bool frozen = cycleFrozen();
        if (frozen) {
            // Out-of-cycle STOP consumer: cave2 cannot consume a STOP while
            // the cycle is frozen. Observer callbacks are deferred to cave2.
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
            // Require a fresh tick before believing a scan again.
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

        // Publish id and validating epoch together through the seqlock, or a
        // reader can take the new epoch with the previous id and believe it.
        // Skip identical publications so the sequence advances only on a real
        // context change (sole writer, so reading back to compare is safe).
        if (s->level_epoch != epochAtScan) {
            // The context moved under the scan, so the result may describe the
            // level just left. Discard it; level_scan_epoch is not advanced, so
            // resolved stays false until a scan completes inside one context.
            if (s->level_id != 0xFFFFFFFFu) {
                publishContext(s, [&] { s->level_id = 0xFFFFFFFFu; });
            }
        } else if (settled && id >= 0) {
            bool alreadyResolved = (s->level_scan_epoch == epochAtScan);
            if (alreadyResolved && s->level_id != (uint32_t)id) {
                // Same context, contradicting ids: two tracks sharing one path
                // or a bad scan. Go unresolved and let the next scan (~200 ms)
                // decide; stepping level_scan_epoch back leaves the pair
                // unequal = unresolved.
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
        // freeze (load / menu) ends it at once.
        bool resolved = (s->level_scan_epoch == s->level_epoch);
        const int slices = resolved ? (int)(STEADY_PERIOD_MS / 100u) : 2;
        const uint32_t epochAtSleep = s->level_epoch;
        for (int i = 0; i < slices; i++) {
            Sleep(100);
            FlushPendingLog();           // cave2's mode-transition lines (queued inside the hook)
            restartrelease::FlushLog();  // F5 hold timing per restart (restart_release.hpp)
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
    // Shared memory survives reinjection, so a previous DLL instance killed
    // mid-write can leave the sequence ODD, which rejects every read forever.
    // Nothing is in flight here, so re-establish an even sequence.
    if (InterlockedOr((volatile LONG*)&s->level_ctx_seq, 0) & 1) {
        InterlockedIncrement((volatile LONG*)&s->level_ctx_seq);
    }
    publishContext(s, [&] {
        s->level_id = 0xFFFFFFFFu;
        // Force the pair unequal ("not identified yet"): a stale equal pair
        // would make the cleared level_id read as a resolved "at the menu".
        s->level_scan_epoch = s->level_epoch - 1u;
    });
    g_thread = CreateThread(nullptr, 0, threadProc, s, 0, nullptr);
    if (!g_thread) {
        // Leave the epochs UNEQUAL: with no scanner the track is unknowable,
        // and an equal pair would read as "resolved, at the menu" forever.
        // The caller logs the failure.
    }
}

} // namespace levelscan
