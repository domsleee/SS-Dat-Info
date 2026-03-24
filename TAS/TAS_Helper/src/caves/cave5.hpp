#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 5: Fixed tick override with variable speed via time-advance scaling.
// Hook at Supreme.exe+25C81 (after __ftol call and mov esi, eax).
//
// At this point:
//   ESI = tick count from __ftol (may be garbage for large floats)
//   Game's next instruction: cmp esi, 14h; ja -> mov esi, 14h (clamp to 20)
//
// Speed scaling approach:
//   The game maintains an accumulator: prev_time += ticks * [EXE+0x25E2B const].
//   The per-tick time advance constant at EXE+0x46DB08 is normally 0.01 (= 1/100).
//   Scaling this constant by 1/speed changes how fast the game "consumes" time:
//     - 0.25x speed: per_tick = 0.04 → game advances time 4x per tick → 25 ticks/s
//     - 2.0x speed:  per_tick = 0.005 → game advances time 0.5x per tick → 200 ticks/s
//   This avoids the auto-compensation problem where scaling ESI directly had no
//   net effect because the game recomputes ticks from accumulated wall time.
//
// Priority:
//   1. force_fixed_tick > 0: force that exact value (typically 2, deterministic)
//   2. playback_speed != 1.0: write 0.01/speed to per-tick advance constant
//   3. Otherwise: clamp to [0, 20] (fix __ftol garbage)

inline TasSharedState* g_cave5State = nullptr;
static SafetyHookMid cave5Hook{};

// Pointer to the game's per-tick time advance constant (EXE + 0x46DB08).
// VirtualProtect'd to PAGE_READWRITE during init so we can write it.
static float* g_tickAdvancePtr = nullptr;

// Default per-tick time advance (1/100 second per tick at normal speed)
static constexpr float TICK_ADVANCE_BASE = 0.01f;

// Cave 5 callback with FPU preservation
static void Cave5_MidCallback(SafetyHookContext& ctx) {
    uint8_t fpu_buf[108];
    __asm { fsave [fpu_buf] }

    auto* s = g_cave5State;
    if (s) {
        int32_t realTick = (int32_t)ctx.esi;

        if (s->force_fixed_tick > 0) {
            // Deterministic mode: exact tick count per frame
            ctx.esi = s->force_fixed_tick;
        } else {
            // Clamp raw tick first (fix __ftol garbage)
            if (realTick < 0) realTick = 0;
            if (realTick > 20) realTick = 20;
            ctx.esi = (uintptr_t)realTick;
        }

        // Variable speed: only scale time advance constant during active REC/PLAY.
        // In OFF mode, always restore the base constant so the game runs at
        // normal speed (fixes buttons/menus broken after 2x playback).
        if (g_tickAdvancePtr) {
            if (s->mode != MODE_OFF && s->playback_speed > 0.0f) {
                *g_tickAdvancePtr = TICK_ADVANCE_BASE / s->playback_speed;
            } else {
                *g_tickAdvancePtr = TICK_ADVANCE_BASE;
            }
        }
    }

    __asm { frstor [fpu_buf] }
}

bool InstallCave5(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave5_site) {
        Log("Cave 5: hook site not resolved");
        return false;
    }

    g_cave5State = state;

    // Resolve and unprotect the per-tick time advance constant at EXE+0x46DB08.
    // This float (normally 0.01) controls how much game-time each physics tick
    // consumes from the accumulator. We modify it for variable speed playback.
    auto* exeBase = (uint8_t*)addr.exe;
    g_tickAdvancePtr = (float*)(exeBase + 0x6DB08);

    DWORD oldProtect = 0;
    if (!VirtualProtect(g_tickAdvancePtr, sizeof(float), PAGE_READWRITE, &oldProtect)) {
        Log(std::format("Cave 5: VirtualProtect on tick advance constant FAILED (err={})", GetLastError()));
        // Non-fatal: speed scaling won't work but fixed tick still does
    } else {
        Log(std::format("Cave 5: tick advance constant at {:p} unprotected (was 0x{:X})", (void*)g_tickAdvancePtr, oldProtect));
    }

    Log(std::format("Cave 5: hooking tick override at {:p} (EXE+0x25C81)", (void*)addr.cave5_site));

    cave5Hook = safetyhook::create_mid(addr.cave5_site, Cave5_MidCallback);

    if (!cave5Hook) {
        Log("Cave 5: SafetyHook create_mid FAILED");
        return false;
    }

    state->cave5_hooked = 1;
    Log("Cave 5: hook installed successfully");
    return true;
}
