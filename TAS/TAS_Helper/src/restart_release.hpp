#pragma once
#include <windows.h>
#include <cstdint>
#include "game_addresses.hpp"
#include "log.hpp"

// Release of F5 the moment the game has acted on it.
//
// The game restarts the level on every input poll that sees F5 down - about
// one poll every 20-25 ms - and a restart itself takes only ~20 ms, so a held
// key restarts the level again and again (a 150 ms tap restarts it 6-7
// times). Each re-entry tears the level down while the previous rebuild is
// still in flight, and that corrupts the heap: the game crashes later, at the
// next save or teardown (e.g. sr.dll+0x13568).
//
// So the key goes back up the moment the game has ACTED on it: the level
// rebuild creates a new player recorder, and the replay-capture hook that
// adopts it runs inside that rebuild, before the level's next input poll.
// That hook clears the DI byte here - for Cave 2's own press AND for a
// physical tap. For Cave 2's press a timed thread also clears the byte after
// a short cap, in case a second poll lands before the rebuild reaches the
// recorder; Cave 2 finishes the sequence (observer "up", restart_state) on
// its next cycle either way. A single Supreme::Cycle is not a usable hold:
// the poll runs less often than the cycle, so a one-cycle press can be missed.
//
// Nothing here filters keyboard autorepeat, because the game already does:
// Win32_Driver::Translate (HMG_Cetsup_Win32) forwards WM_KEYDOWN to the key
// handler at +0x3940 only when `(lParam & 0x40000000)` is clear - bit 30, "key
// was already down" - or when Win32_Keyboard's Key_Repeat flag (+0x34) is set.
// Supreme.exe touches that flag only through the wrapper at 0x004558d0, in
// pairs around the main menu's modal loops: `push 1` in (0x423a09, 0x4288d2),
// `push ebx` with ebx zeroed out (0x423a35, 0x42890a). So repeats reach the
// handler only in menu dialogs, where no level is running and F5 restarts
// nothing. Do not add a repeat gate: it cannot fire where F5 restarts, and a
// spacing window there would refuse restarts the player asked for.
namespace restartrelease {

inline uint32_t g_diBuffer = 0;        // DI buffer Cave 2's press was written to
inline uint32_t g_knownBuffer = 0;     // last DI buffer Cave 2 saw (for physical taps)
inline DWORD g_pressMs = 0;            // GetTickCount at Cave 2's press
inline volatile LONG g_pending = 0;    // 1 while Cave 2's F5 byte is still down
inline volatile LONG g_physicalClears = 0;  // restarts that took a physical F5 back up
inline HANDLE g_thread = nullptr;

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
    Log(std::format("Restart F5: byte up after {} ms ({}); physical taps cut short {}",
                    (long)g_lastHoldMs, by, (long)g_physicalClears));
}

// Runs for the life of the process: main.cc pins TAS_Helper.dll precisely so
// its code cannot be unloaded out from under the game, so there is no unload
// path for this thread to be stopped on.
inline DWORD WINAPI ReleaseThread(LPVOID) {
    for (;;) {
        Sleep(5);
        if (g_pending && GetTickCount() - g_pressMs >= MAX_HOLD_MS) ReleaseByteNow(2);
    }
}

// Cave 2 pressed F5 (game thread).
inline void Pressed(uint32_t buffer) {
    DWORD now = GetTickCount();
    g_diBuffer = buffer;
    g_pressMs = now;
    InterlockedExchange(&g_pending, 1);
    if (!g_thread) g_thread = CreateThread(nullptr, 0, ReleaseThread, nullptr, 0, nullptr);
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

inline bool Pending() { return g_pending != 0; }

}  // namespace restartrelease
