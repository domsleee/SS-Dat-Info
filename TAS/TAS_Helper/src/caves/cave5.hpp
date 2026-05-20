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

// Documented default per-tick time advance — used only as a fallback if we
// somehow can't read the live value during init.
static constexpr float TICK_ADVANCE_DEFAULT = 0.01f;

// Native value of the time advance constant, captured during InstallCave5
// before we ever modify it. At 1x speed (or in OFF mode) we restore this
// rather than overwriting with a hardcoded guess; if the game's actual
// per-tick advance differs from 0.01 then forcing 0.01 silently changes
// the game's effective speed and (critically) leaves a stale value when
// the game pauses, producing a fast-forward on resume.
static float g_nativeTickAdvance = TICK_ADVANCE_DEFAULT;

// Cave 5 callback with FPU preservation
static void Cave5_MidCallback(SafetyHookContext& ctx) {
    uint64_t t0 = __rdtsc();
    uint8_t fpu_buf[108];
    __asm { fsave [fpu_buf] }

    auto* s = g_cave5State;
    if (s) {
        int32_t realTick = (int32_t)ctx.esi;

        // Pause-resume catchup detection: when the game is unpaused after a
        // pause, __ftol computes (wall_time - prev_time) / tick_advance and
        // hands a huge tick count (e.g. 20s pause @ 0.01s/tick = 2000) to
        // the physics loop. The original clamp at 20 just spreads the burst
        // over many frames (visible as a ~2× speedup for a second or so).
        // Drain the accumulator in a single frame instead: set this frame's
        // tick_advance to realTick * native so 1 physics tick consumes the
        // entire wall-time gap. The game advances 1 physics tick (snowboarder
        // barely moves), prev_time catches up to wall_time, next frame is
        // back to a normal tick count under native tick_advance.
        //
        // Only fires in OFF mode at 1x speed — REC/PLAY runs must stay
        // deterministic (force_fixed_tick path) and speed-scaled playback
        // owns the tick_advance constant.
        const int32_t CATCHUP_THRESHOLD = 50;
        bool catchup_drain =
            realTick > CATCHUP_THRESHOLD
            && s->force_fixed_tick == 0
            && s->mode == MODE_OFF
            && s->playback_speed == 1.0f;

        if (s->force_fixed_tick > 0) {
            // Deterministic mode: exact tick count per frame
            ctx.esi = s->force_fixed_tick;
        } else if (catchup_drain) {
            // Set tick_advance large enough that 1 tick drains the whole
            // wall-time gap (gap ≈ realTick * native because __ftol used
            // tick_advance = native last frame).
            if (g_tickAdvancePtr) {
                *g_tickAdvancePtr = (float)realTick * g_nativeTickAdvance;
            }
            ctx.esi = 1;
        } else {
            // Clamp raw tick first (fix __ftol garbage)
            if (realTick < 0) realTick = 0;
            if (realTick > 20) realTick = 20;
            ctx.esi = (uintptr_t)realTick;
        }

        // Variable speed: only scale the time advance constant during active
        // REC/PLAY at a non-1x speed. At 1x, in OFF mode, or with invalid
        // speed, write the captured native value so the game runs at its
        // own natural speed. Writing the native value (vs not writing at
        // all) restores correct state when transitioning down from 2x→1x.
        // Skipped during catchup_drain — that path wrote a temporary large
        // value to tick_advance that the game must read next frame.
        if (g_tickAdvancePtr && !catchup_drain) {
            if (s->mode != MODE_OFF && s->playback_speed > 0.0f && s->playback_speed != 1.0f) {
                *g_tickAdvancePtr = g_nativeTickAdvance / s->playback_speed;
            } else {
                *g_tickAdvancePtr = g_nativeTickAdvance;
            }
        }
    }

    __asm { frstor [fpu_buf] }
    auto* s2 = g_cave5State;
    if (s2) {
        PerfSample(s2->perf_cave5, __rdtsc() - t0);
    }
}

bool InstallCave5(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave5_site) {
        Log("Cave 5: hook site not resolved");
        return false;
    }

    g_cave5State = state;

    // Resolve and unprotect the per-tick time advance constant at EXE+0x46DB08.
    // This float controls how much game-time each physics tick consumes from
    // the accumulator. We modify it for variable speed playback.
    auto* exeBase = (uint8_t*)addr.exe;
    g_tickAdvancePtr = (float*)(exeBase + 0x6DB08);

    // Capture the native value before we ever modify the constant. The page
    // is readable even before VirtualProtect (it's part of the loaded image),
    // so this read is safe regardless of whether VirtualProtect succeeds.
    g_nativeTickAdvance = *g_tickAdvancePtr;
    Log(std::format("Cave 5: native tick advance constant = {}", g_nativeTickAdvance));

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
