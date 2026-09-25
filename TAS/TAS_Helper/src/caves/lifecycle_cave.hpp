#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../shared_state.hpp"
#include "../level_context.hpp"
#include "cycle_cave.hpp"
#include "menu_cave.hpp"
#include "../fpu_safe_hook.hpp"

// The race lifecycle hooks (DESIGN.md "The race lifecycle"), all on the game
// thread:
//
//   LAUNCH  Supreme.exe+0x25BD7 - the race loop has entered the level and set
//           the game mode; the race is about to run.
//   STOP    Supreme_Game+0x1408F0 (Supreme::Stop) - the race is being left,
//           while the level still exists.
//   PUMP    Supreme.exe+0x55920 - the message pump. Runs in every state,
//           including menus and dialogs where Supreme::Cycle does not.
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

// A CONT whose controller died leaves the live-input block set. A restart
// freezes the cycle for a second or two; longer than this, nothing will
// clear it.
static constexpr uint32_t STALE_SUPPRESS_MS = 5000;

static void OnLaunch(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    g_sawLaunch = true;
    levelcontext::PublishRunning(s);
    s->game_in_game = 1;
    g_refreshStamps = REFRESH_STAMP_TICKS;
}

static void OnStop(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    StopForLeftRace(s);
    levelcontext::PublishLeft(s);
    s->game_in_game = 0;
}

static void OnPump(SafetyHookContext&) {
    auto* s = g_state;
    if (!s) return;
    TryProcessStopCommand(s);
    if (!g_sawLaunch && s->frame_count != g_frameAtInstall) {
        g_sawLaunch = true;
        levelcontext::PublishRunning(s);
        s->game_in_game = 1;
        g_refreshStamps = REFRESH_STAMP_TICKS;
    }
    // The setup object may lag the launch, so retry identification.
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
}

inline bool Install(GameAddresses& addr, TasSharedState* s) {
    g_state = s;
    g_frameAtInstall = s->frame_count;
    levelcontext::Init(s, (uint32_t)(uintptr_t)addr.level_path_ptr, &SafeReadPtr);
    s->game_in_game = 0;
    g_launchHook = CreateMidHook<OnLaunch>(addr.launch_site);
    g_stopHook = CreateMidHook<OnStop>(addr.stop_site);
    g_pumpHook = CreateMidHook<OnPump>(addr.pump_site);
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
