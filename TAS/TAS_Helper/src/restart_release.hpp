#pragma once
#include <windows.h>
#include <cstdint>
#include "game_addresses.hpp"
#include "log.hpp"
#include "restart_release_policy.hpp"

// Release of F5 the moment the game has acted on it.
//
// The game restarts the level on every input poll that sees F5 down - about
// one poll every 20-25 ms - and a restart itself takes only ~20 ms, so a key
// held for any length of time restarts the level again and again (measured
// 2026-09-09: a physical tap held 15 ms = 1-2 restarts, 50 ms = 2-3,
// 150 ms = 6-7; Cave 2's old 10-cycle hold = 4 per arm). Every re-entry tears
// the level down while the previous rebuild is still in flight; the crashes
// seen at the next save/teardown (sr.dll+0x13568 in the srConfig unlink, a
// use-after-free in the text-input callback) are the debt that leaves.
//
// So the key goes back up the moment the game has ACTED on it: the level
// rebuild creates a new player recorder, and the replay-capture hook that
// adopts it runs inside that rebuild, before the level's next input poll.
// That hook clears the DI byte here - for Cave 2's own press AND for a
// physical tap. For Cave 2's press a timed thread also clears the byte after
// a short cap, in case a second poll lands before the rebuild reaches the
// recorder; Cave 2 finishes the sequence (observer "up", restart_state) on
// its next cycle either way.
//
// A physical key held past the keyboard's repeat delay would set the byte
// again on every autorepeat, so cave1c's handler gate also swallows F5 downs
// arriving within F5_MIN_SPACING_MS of the last accepted one (and their
// matching ups).
namespace restartrelease {

inline uint32_t g_diBuffer = 0;        // DI buffer Cave 2's press was written to
inline uint32_t g_knownBuffer = 0;     // last DI buffer Cave 2 saw (for physical taps)
inline DWORD g_pressMs = 0;            // GetTickCount at Cave 2's press
inline volatile LONG g_pending = 0;    // 1 while Cave 2's F5 byte is still down
inline volatile LONG g_releasedBy = 0; // diagnostic: 1 = restart hook, 2 = timed cap, 3 = cycle cap
inline volatile LONG g_physicalClears = 0;  // restarts that took a physical F5 back up
inline volatile LONG g_repeatsSwallowed = 0;  // physical F5 downs refused as autorepeat/bounce
inline HANDLE g_thread = nullptr;
inline volatile LONG g_stop = 0;

// Handler-gate state (game thread only).
inline DWORD g_lastAcceptedF5Ms = 0;
inline uint32_t g_f5DownsSwallowed = 0;

// Timed cap for Cave 2's press: longer than one poll period so the game is
// sure to have seen the key, shorter than a second poll.
inline constexpr DWORD MAX_HOLD_MS = 25;

inline void SafeWriteF5(uint32_t buffer, bool pressed) {
    __try {
        ((uint8_t*)buffer)[GameAddresses::KEY_F5] = pressed ? 0x01 : 0x00;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}

inline bool SafeReadF5(uint32_t buffer, uint8_t* out) {
    __try {
        *out = ((uint8_t*)buffer)[GameAddresses::KEY_F5];
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Cave 2 calls this every cycle so a physical restart can be cut short too.
inline void NoteBuffer(uint32_t buffer) {
    if (buffer) g_knownBuffer = buffer;
}

// Diagnostics for the worker's log: how long the byte was down and who
// took it up. Written by the releasing thread, read by the worker.
inline volatile LONG g_lastHoldMs = -1;
inline volatile LONG g_lastBy = 0;
inline volatile LONG g_releases = 0;
inline LONG g_releasesLogged = 0;

// Clears Cave 2's byte if its press is still down. Idempotent, any thread.
inline bool ReleaseByteNow(LONG by) {
    if (InterlockedExchange(&g_pending, 0) == 0) return false;
    if (g_diBuffer) SafeWriteF5(g_diBuffer, false);
    InterlockedExchange(&g_releasedBy, by);
    InterlockedExchange(&g_lastHoldMs, (LONG)(GetTickCount() - g_pressMs));
    InterlockedExchange(&g_lastBy, by);
    InterlockedIncrement(&g_releases);
    return true;
}

// Worker-thread side: one line per release (no I/O on the game thread).
inline void FlushLog() {
    LONG releases = g_releases;
    if (releases == g_releasesLogged) return;
    g_releasesLogged = releases;
    const char* by = g_lastBy == 1 ? "restart hook" : g_lastBy == 2 ? "25 ms cap" : "cycle cap";
    Log(std::format("Restart F5: byte up after {} ms ({}); physical taps cut short {}, repeats swallowed {}",
                    (long)g_lastHoldMs, by, (long)g_physicalClears, (long)g_repeatsSwallowed));
}

inline DWORD WINAPI ReleaseThread(LPVOID) {
    while (!g_stop) {
        Sleep(5);
        if (g_pending && GetTickCount() - g_pressMs >= MAX_HOLD_MS) ReleaseByteNow(2);
    }
    return 0;
}

// Cave 2 pressed F5 (game thread).
inline void Pressed(uint32_t buffer) {
    DWORD now = GetTickCount();
    g_diBuffer = buffer;
    g_pressMs = now;
    g_lastAcceptedF5Ms = now ? now : 1;   // a physical tap right behind an arm is a repeat too
    InterlockedExchange(&g_releasedBy, 0);
    InterlockedExchange(&g_pending, 1);
    if (!g_thread) g_thread = CreateThread(nullptr, 0, ReleaseThread, nullptr, 0, nullptr);
}

// Stops the timed-release thread. The DLL has no unload path today; this
// exists so one can be added without leaving a thread running in freed code.
inline void Shutdown() {
    InterlockedExchange(&g_stop, 1);
    if (g_thread) {
        WaitForSingleObject(g_thread, 200);
        CloseHandle(g_thread);
        g_thread = nullptr;
    }
}

// The level restarted (a new human recorder was adopted): the game has seen
// the key, take it away before the rebuilt level polls again - whoever
// pressed it.
inline void OnLevelRestartObserved() {
    if (ReleaseByteNow(1)) return;
    uint8_t down = 0;
    if (g_knownBuffer && SafeReadF5(g_knownBuffer, &down) && down) {
        SafeWriteF5(g_knownBuffer, false);
        InterlockedIncrement(&g_physicalClears);
    }
}

// cave1c handler gate: a physical F5 down may reach the game only
// F5_MIN_SPACING_MS after the last one it saw. Game thread only.
inline bool AcceptPhysicalF5Down() {
    DWORD now = GetTickCount();
    if (!now) now = 1;
    if (!F5DownAccepted(now, g_lastAcceptedF5Ms, F5_MIN_SPACING_MS)) {
        g_f5DownsSwallowed++;
        InterlockedIncrement(&g_repeatsSwallowed);
        return false;
    }
    g_lastAcceptedF5Ms = now;
    return true;
}

// The up that pairs with a swallowed down is swallowed too.
inline bool SwallowPhysicalF5Up() { return F5UpSwallowed(g_f5DownsSwallowed); }

inline bool Pending() { return g_pending != 0; }

}  // namespace restartrelease
