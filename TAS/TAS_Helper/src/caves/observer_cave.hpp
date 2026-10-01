#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include "key_handler_cave.hpp"
#include <safetyhook.hpp>

// The observer cave: gate on BB3B10 (HMG+3B10), the keyboard observer
// notification that fires on key-state changes. Steering needs it.
// BB3B10 is __thiscall with 4 stack args (ret 0010):
//   ecx = keyboard+0x18, args: keyIndex, pressed, Time.lo, Time.hi
// The __fastcall detour takes a dummy EDX.
//
// Injected calls always pass. Real calls are blocked during a CONT; ESC always
// passes. (PLAY's block is in the key-handler cave; REC blocks nothing.)

inline TasSharedState* g_observerCaveState = nullptr;
static SafetyHookInline observerHook{};

inline void UninstallObserverCave() {
    observerHook = {};
    if (g_observerCaveState) g_observerCaveState->observer_cave_hooked = 0;
    g_observerCaveState = nullptr;
}

void __fastcall Observer_Detour(void* ecx, void* edx, uint32_t keyIndex,
                                     uint32_t pressed, uint32_t unk, uint32_t arg4) {
    auto* s = g_observerCaveState;
    if (s) {
        s->bb3b10_call_count++;

        // Calibrate the fallback stamp (see g_bb3b10Arg4).
        if (!IsTasInjectionThread() && !s->test_arg4_override && arg4 != g_bb3b10Arg4) {
            g_bb3b10Arg4 = arg4;
        }

        if (IsTasInjectionThread()) {
            observerHook.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
            return;
        }

        // Backstop to the key-handler cave's CONT block. Not pause-exempt:
        // the F5 reload stalls the cycle.
        if (s->cont_suppress_input && keyIndex != GameAddresses::KEY_ESC) {
            s->bb3b10_block_count++;
            return;
        }

        // A running PLAY lets through the release of a key a take cannot
        // hold (its press may have come in a menu). Queued, it would sit
        // among the replay's events and, one event per Update, push one of
        // them a tick later; applied here it leaves the queue alone. Not
        // Escape: its press always gets through (pause, abort), and the
        // pause menu must see it come up or the next Escape cannot resume.
        if (s->mode == MODE_PLAY && (pressed & 0xFF) == 0 && keyIndex != GameAddresses::KEY_ESC &&
            !heldkeys::IsTasCode(keyIndex) && !MenuPauseNow()) {
            ClearHeldCode(keyIndex);
            s->bb3b10_block_count++;
            return;
        }
    }

    observerHook.thiscall<void>(ecx, keyIndex, pressed, unk, arg4);
}

bool InstallObserverCave(GameAddresses& addr, TasSharedState* state) {
    if (!addr.bb3b10) {
        Log("Observer cave: hook site not resolved");
        return false;
    }

    g_observerCaveState = state;
    Log(std::format("Observer cave: hooking BB3B10 at {:p} (HMG+3B10)", (void*)addr.bb3b10));

    observerHook = safetyhook::create_inline(addr.bb3b10, Observer_Detour);
    if (!observerHook) {
        Log("Observer cave: SafetyHook create_inline FAILED on BB3B10 (+3B10)");
        g_observerCaveState = nullptr;
        return false;
    }

    state->observer_cave_hooked = 1;
    Log("Observer cave: inline hook installed successfully");
    return true;
}
