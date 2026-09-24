#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../shared_state.hpp"
#include "../level_context.hpp"
#include "cave2.hpp"
#include "menu_state.hpp"
#include <safetyhook.hpp>

// The game's race lifecycle, observed at its own transition points instead of
// inferred from a polling thread:
//
//   LAUNCH  Supreme.exe+0x25BD7 - the race loop has entered the level (a new
//           load or the same track re-entered) and set the game mode; the race
//           is about to run. Publishes the level and game_in_game = 1.
//   STOP    Supreme_Game+0x1408F0 (Supreme::Stop) - the race is being left
//           (quit to the menu, before a track switch). Stops an armed TAS mode
//           while the level still exists, publishes "no race", game_in_game = 0.
//   PUMP    Supreme.exe+0x55920 - the game's message pump. It runs on the game
//           thread in every state: racing, the menus, the pause menu and
//           dialogs, where Supreme::Cycle does not run. Consumes STOP there and
//           does the DLL's housekeeping.
//
// All three run on the game thread, like Cave 2, so no DLL state is shared
// with another thread.
namespace lifecycle {

inline TasSharedState* g_state = nullptr;
inline SafetyHookMid g_launchHook{};
inline SafetyHookMid g_stopHook{};
inline SafetyHookMid g_pumpHook{};

// Injected into a race already running: no LAUNCH will come for it, so the
// first Supreme::Cycle tick after install identifies it once.
inline bool g_sawLaunch = false;
inline uint32_t g_frameAtInstall = 0;
inline uint32_t g_frameAtLastIdentify = 0;

// A Continue whose controller died leaves the live-input block set, and the
// input gate then swallows every key but Escape. A restart's reload freezes
// the cycle for a second or two; frozen this long with the block still set,
// nothing is going to clear it.
static constexpr uint32_t STALE_SUPPRESS_MS = 5000;

static void OnLaunch(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    uint8_t fpu[108];
    __asm { fsave [fpu] }
    g_sawLaunch = true;
    levelcontext::PublishRunning(s);
    s->game_in_game = 1;
    g_refreshStamps = REFRESH_STAMP_TICKS;   // rider and renderer, over the race's first ticks (cave2)
    __asm { frstor [fpu] }
}

static void OnStop(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    uint8_t fpu[108];
    __asm { fsave [fpu] }
    StopForLeftRace(s);
    levelcontext::PublishLeft(s);
    s->game_in_game = 0;
    __asm { frstor [fpu] }
}

static void OnPump(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    uint8_t fpu[108];
    __asm { fsave [fpu] }
    TryProcessStopCommand(s);
    if (!g_sawLaunch && s->frame_count != g_frameAtInstall) {
        g_sawLaunch = true;
        levelcontext::PublishRunning(s);
        s->game_in_game = 1;
        g_refreshStamps = REFRESH_STAMP_TICKS;
    }
    // A race whose track could not be identified at launch stays unresolved;
    // try again once it has ticked (the setup object may lag the launch).
    if (s->game_in_game && s->frame_count != g_frameAtLastIdentify) {
        g_frameAtLastIdentify = s->frame_count;
        levelcontext::RetryIfUnresolved(s);
    }
    if (s->cont_suppress_input && GetTickCount() - g_lastCycleMs > STALE_SUPPRESS_MS) {
        s->cont_suppress_input = 0;
        LogRing(s, LOG_WARN, "Cleared a stale live-input block (game loop frozen over 5 s)");
    }
    menustate::Housekeeping();
    FlushPendingLog();
    __asm { frstor [fpu] }
}

inline bool Install(GameAddresses& addr, TasSharedState* s) {
    g_state = s;
    g_frameAtInstall = s->frame_count;
    levelcontext::Init(s, (uint32_t)(uintptr_t)addr.level_path_ptr, &SafeReadPtr);
    s->game_in_game = 0;
    g_launchHook = safetyhook::create_mid(addr.launch_site, OnLaunch);
    g_stopHook = safetyhook::create_mid(addr.stop_site, OnStop);
    g_pumpHook = safetyhook::create_mid(addr.pump_site, OnPump);
    const bool ok = g_launchHook && g_stopHook && g_pumpHook;
    Log(ok ? std::format("Lifecycle: hooked launch {:p} (EXE+0x25BD7), stop {:p} (SG+0x1408F0), pump {:p} (EXE+0x55920)",
                         (void*)addr.launch_site, (void*)addr.stop_site, (void*)addr.pump_site)
           : std::string("Lifecycle: hook installation FAILED"));
    return ok;
}

inline void Uninstall() {
    g_pumpHook = {};
    g_stopHook = {};
    g_launchHook = {};
    g_state = nullptr;
}

}  // namespace lifecycle
