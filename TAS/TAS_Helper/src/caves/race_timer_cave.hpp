#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../fpu_safe_hook.hpp"
#include "../shared_state.hpp"
#include "menu_cave.hpp"

// Race timer: publishes the game's own race clock (DESIGN.md "Race time")
// from the clock-tick hook, Supreme_Game+0xB4B80 (inc word [SG+0x1D5334];
// 100/sec while a level runs). race_time_cs, race_start_ts and race_ab_* are
// no longer written; they keep their reset values.

namespace racetimer {

inline TasSharedState* g_state = nullptr;

static SafetyHookMid g_tickHook{};

// The game's own race clock: [player+0xB8] is the player's timer object;
// +0x0C its elapsed seconds (float, +0.01 per Player update), +0x10 started,
// +0x11 finished. The HUD formats +0x0C.
static bool ReadRaceClock(uint32_t player, uint32_t* bits, uint32_t* flags) {
    if (player < 0x10000) return false;
    __try {
        const uint32_t timer = *(uint32_t*)(player + GameAddresses::PLAYER_RACE_TIMER_OFFSET);
        if (timer < 0x10000) return false;
        *bits = *(uint32_t*)(timer + 0x0C);
        *flags = (*(uint8_t*)(timer + 0x10) ? TAS_RACE_CLOCK_STARTED : 0u) |
                 (*(uint8_t*)(timer + 0x11) ? TAS_RACE_CLOCK_FINISHED : 0u);
        return true;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Writes the clock pair under race_seq. Game thread only.
static void PublishClock(bool inGame) {
    uint32_t bits = 0xFFFFFFFFu, flags = 0;
    if (!inGame || !ReadRaceClock(g_state->player_ptr, &bits, &flags)) {
        bits = 0xFFFFFFFFu;
        flags = 0;
    }
    if (g_state->race_clock_bits == bits && g_state->race_clock_flags == flags) return;
    InterlockedIncrement((volatile LONG*)&g_state->race_seq);   // odd: writing
    g_state->race_clock_bits = bits;
    g_state->race_clock_flags = flags;
    InterlockedIncrement((volatile LONG*)&g_state->race_seq);   // even: stable
}

// Once per clock tick.
static void TickCb(SafetyHookContext&) {
    if (!g_state) return;
    // The clock only ticks while a level runs, so no menu is on screen.
    if (g_state->game_in_game) menustate::ClearForLevel();
    PublishClock(g_state->game_in_game != 0);
}

inline bool Install(GameAddresses& addr, TasSharedState* state) {
    if (!addr.sg) { Log("Race timer: no SG base"); return false; }
    auto sg = (uint8_t*)addr.sg;
    // On-disk bytes; the imm32 at +3 is compared after rebasing (ValidateCodeAbs).
    static constexpr uint8_t kRaceTick[] =                    // inc word [SG+0x1D5334]; ret
        { 0x66, 0xFF, 0x05, 0x34, 0x53, 0x1D, 0x10, 0xC3 };
    // Optional feature: a mismatch disables it rather than failing TAS_Initialize.
    if (!GameAddresses::ValidateCodeAbs<3>("Supreme_Game.dll+0xB4B80", sg + 0xB4B80,
                                           kRaceTick, sg, 0x1D5334)) {
        Log("Race timer: unavailable - site validation failed");
        return false;
    }

    g_state = state;
    // Shared memory survives reinjection: an odd race_seq left by a DLL killed
    // mid-publish would block readers forever.
    if (g_state->race_seq & 1) InterlockedIncrement((volatile LONG*)&g_state->race_seq);
    g_tickHook = CreateMidHook<TickCb>(sg + 0xB4B80);  // clock tick (100/sec)
    if (!g_tickHook) g_state = nullptr;
    Log(std::format("Race timer: tick={}", (bool)g_tickHook));
    return (bool)g_tickHook;
}

// Initialization rollback.
inline void Uninstall() {
    g_tickHook = {};
    g_state = nullptr;
}

} // namespace racetimer
