#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../fpu_safe_hook.hpp"

// In-process F5 restart, driven by the game's own F5 handling.
//
// Once per outer race-loop iteration, before the tick batch, the EXE reads F5
// from the Win32 keyboard's key buffer (Supreme.exe+0x25C11). If it is down and
// the race accepts a restart, it calls the race controller's restart at
// +0x25C3F, which rebuilds the level synchronously: Supreme::Set_Game_Mode
// recreates the human player and its recorder, and its mode initialisation
// returns at Supreme_Game+0x14199F.
//
// So a restart is: the cycle cave holds F5 down; the ACCEPT hook takes it back up the
// moment the game acts on it (any F5, a real key too, so one press is one
// restart - the game restarts again on every poll that still sees the key);
// the DONE hook marks the rebuilt level. Until DONE, the cycle cave keeps the key down,
// so a restart the game refused is simply taken on its next poll. A STOP lets go.
namespace f5restart {

inline GameAddresses* g_addr = nullptr;

// One request at a time; generations keep a stale DONE from answering a newer
// request. Written by the cycle cave and STOP (request/cancel) and by the two hooks,
// all on the game thread except a STOP consumed while the loop is frozen.
inline volatile LONG g_request = 0;   // current request's generation, 0 = none
inline volatile LONG g_accepted = 0;  // generation whose F5 the game acted on
inline volatile LONG g_done = 0;      // generation whose level finished rebuilding
inline LONG g_nextGeneration = 0;

inline SafetyHookMid g_acceptHook{};
inline SafetyHookMid g_doneHook{};

static uint32_t ReadU32(uint32_t addr) {
    if (!addr) return 0;
    __try {
        return *(volatile uint32_t*)addr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// The key buffer the game polls, resolved fresh on every use: [[[SG+1D5450]+0x530]+0x30].
static uint32_t KeyBuffer() {
    if (!g_addr) return 0;
    const uint32_t root = ReadU32((uint32_t)(uintptr_t)g_addr->player_base);
    const uint32_t keyboard = ReadU32(root ? root + GameAddresses::KEYBOARD_OBJ_OFFSET : 0);
    return ReadU32(keyboard ? keyboard + GameAddresses::DI_BUFFER_PTR_OFFSET : 0);
}

static void WriteF5(bool down) {
    const uint32_t buffer = KeyBuffer();
    if (!buffer) return;
    __try {
        ((volatile uint8_t*)buffer)[GameAddresses::KEY_F5] = down ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
}

// Supreme.exe+0x25C3F: F5 is down and the race is about to restart.
static void OnAccept(SafetyHookContext&) {
    WriteF5(false);
    const LONG request = g_request;
    if (request) InterlockedExchange(&g_accepted, request);
}

// Supreme_Game+0x14199F: Set_Game_Mode's mode initialisation returned. It also
// runs on a race entered from the menu, so it counts only after an accept.
static void OnDone(SafetyHookContext&) {
    const LONG request = g_request;
    if (request && g_accepted == request) InterlockedExchange(&g_done, request);
}

// The cycle cave (game thread): start a restart.
inline void Request() {
    InterlockedExchange(&g_request, ++g_nextGeneration);
    WriteF5(true);
}

// The cycle cave, every cycle of a request: true once the level has rebuilt; until
// then the key stays down (a real key-up could have cleared it).
inline bool Step() {
    const LONG request = g_request;
    if (!request) return false;
    if (g_done == request) {
        InterlockedExchange(&g_request, 0);
        return true;
    }
    if (g_accepted != request) WriteF5(true);
    return false;
}

// STOP or a new request over an unfinished one: let go of the key. Any thread.
inline void Cancel() {
    const LONG request = InterlockedExchange(&g_request, 0);
    if (request && g_accepted != request) WriteF5(false);
}

inline bool Install(GameAddresses& addr) {
    g_addr = &addr;
    g_acceptHook = CreateMidHook<OnAccept>(addr.f5_accept_site);
    g_doneHook = CreateMidHook<OnDone>(addr.f5_done_site);
    const bool ok = g_acceptHook && g_doneHook;
    Log(ok ? std::format("F5 restart: hooked accept {:p} (EXE+0x25C3F), done {:p} (SG+0x14199F)",
                         (void*)addr.f5_accept_site, (void*)addr.f5_done_site)
           : std::string("F5 restart: hook installation FAILED"));
    return ok;
}

inline void Uninstall() {
    g_doneHook = {};
    g_acceptHook = {};
    g_request = 0;
}

}  // namespace f5restart
