#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include <safetyhook.hpp>

// Cave 1D: BB3B10 observer notification gate at HMG+3B10.
//
// BB3B10 is the keyboard observer notification function. It fires when
// key state changes, and is REQUIRED for steering to work.
//
// Gate logic:
//   - current thread is in a TAS injection scope: ALWAYS pass through
//   - a CONT is in flight (cont_suppress_input): BLOCK, ESC exempt
//   - MODE_REC: BLOCK (Cave 2 calls BB3B10 directly on transitions for
//     timing symmetry with PLAY), ESC and paused-game exempt
//   - All other cases: pass through
//
// Uses SafetyHookInline for clean function interception.
// BB3B10 is __thiscall with 4 stack args (ret 0010):
//   ecx = keyboard+0x18, args: keyIndex, pressed, Time.lo, Time.hi
// Detour uses __fastcall with dummy EDX to emulate thiscall.

inline TasSharedState* g_cave1dState = nullptr;
static SafetyHookInline cave1dInline{};

inline void UninstallCave1D() {
    cave1dInline = {};
    if (g_cave1dState) g_cave1dState->cave1d_hooked = 0;
    g_cave1dState = nullptr;
}

void __fastcall Cave1D_BB3B10Detour(void* ecx, void* edx, uint32_t keyIndex,
                                     uint32_t pressed, uint32_t unk, uint32_t arg4) {
    auto* s = g_cave1dState;
    if (s) {
        s->bb3b10_call_count++;

        // Fallback calibration from real BB3B10 calls (arg4 = Time.hi of the
        // event stamp). Injection normally uses Kernel::Time::Current() and
        // never reads this; belt-and-braces alongside cave1c's calibration.
        if (!IsTasInjectionThread() && !s->test_arg4_override && arg4 != g_bb3b10Arg4) {
            g_bb3b10Arg4 = arg4;
        }

        // Allow through only on the thread Cave 2 is actively injecting from.
        if (IsTasInjectionThread()) {
            cave1dInline.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
            return;
        }

        bool gamePaused = (GetTickCount() - g_lastCycleMs) > 250;
        // CONT block, a backstop to cave1c (the +3940 handler is the only
        // path real input takes to BB3B10): drop any real observer call so it
        // cannot perturb the spawn. ESC exempt; NOT pause-exempt, because the
        // F5 reload stalls the cycle.
        if (s->cont_suppress_input && keyIndex != GameAddresses::KEY_ESC) {
            s->bb3b10_block_count++;
            return;
        }

        // Block during REC: Cave 2 calls BB3B10 directly on transitions
        // for symmetric timing between REC and PLAY. Exempt:
        //  - ESC (ki 0x48): not a gameplay key; the pause menu subscribes to
        //    its broadcast.
        //  - while PAUSED (cycle heartbeat stale): the sim is not running, and
        //    the pause menu's own navigation (arrows/enter) is observer-driven.
        if (s->mode == MODE_REC
            && keyIndex != GameAddresses::KEY_ESC && !gamePaused) {
            s->bb3b10_block_count++;
            return;
        }
    }

    // Pass through in all other cases (IDLE, PLAY)
    cave1dInline.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
}

bool InstallCave1D(GameAddresses& addr, TasSharedState* state) {
    if (!addr.bb3b10) {
        Log("Cave 1D: hook site not resolved");
        return false;
    }

    g_cave1dState = state;
    Log(std::format("Cave 1D: hooking BB3B10 at {:p} (HMG+3B10)", (void*)addr.bb3b10));

    cave1dInline = safetyhook::create_inline(addr.bb3b10, Cave1D_BB3B10Detour);
    if (!cave1dInline) {
        Log("Cave 1D: SafetyHook create_inline FAILED on BB3B10 (+3B10)");
        g_cave1dState = nullptr;
        return false;
    }

    state->cave1d_hooked = 1;
    Log("Cave 1D: inline hook installed successfully");
    return true;
}
