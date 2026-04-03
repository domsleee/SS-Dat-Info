#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 1D: BB3B10 observer notification gate at HMG+3B10.
//
// BB3B10 is the keyboard observer notification function. It fires when
// key state changes, and is REQUIRED for steering to work.
//
// Gate logic:
//   - cave2_injecting=1: ALWAYS pass through (Cave 2 direct call)
//   - MODE_REC + inject_mode=6: BLOCK (Cave 2 will call BB3B10 directly
//     on transitions for timing symmetry with PLAY)
//   - All other cases: pass through
//
// Uses SafetyHookInline for clean function interception.
// BB3B10 is __thiscall with 4 stack args (ret 0010):
//   ecx = keyboard+0x18, args: keyIndex, pressed, unk(0), 0x588
// Detour uses __fastcall with dummy EDX to emulate thiscall.

inline TasSharedState* g_cave1dState = nullptr;
static SafetyHookInline cave1dInline{};

void __fastcall Cave1D_BB3B10Detour(void* ecx, void* edx, uint32_t keyIndex,
                                     uint32_t pressed, uint32_t unk, uint32_t arg4) {
    uint64_t t0 = __rdtsc();
    auto* s = g_cave1dState;
    if (s) {
        s->bb3b10_call_count++;

        // Allow through if Cave 2 is actively injecting
        if (s->cave2_injecting) {
            cave1dInline.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
            PerfSample(s->perf_cave1d, __rdtsc() - t0);
            return;
        }

        // Block during REC mode 6: Cave 2 will call BB3B10 directly on
        // transitions for symmetric timing between REC and PLAY.
        if (s->mode == MODE_REC && s->inject_mode == 6) {
            s->bb3b10_block_count++;
            PerfSample(s->perf_cave1d, __rdtsc() - t0);
            return;
        }
    }

    // Pass through in all other cases (IDLE, PLAY, non-mode-6)
    cave1dInline.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
    if (s) {
        PerfSample(s->perf_cave1d, __rdtsc() - t0);
    }
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
        return false;
    }

    state->cave1d_hooked = 1;
    Log("Cave 1D: inline hook installed successfully");
    return true;
}
