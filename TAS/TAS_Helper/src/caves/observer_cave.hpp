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
//
// The key-queue hook (EXE+0x10950) keeps a real ESC out of TC_Kbd_Impl's queue
// during PLAY: the queue is insertion-ordered and applies one event per Update,
// so an ESC queued among an injected take's events pushed one of them a tick
// later (a CONT diverged when paused during its catch-up). The pause menu hears
// ESC through its own listener (0x453760), which BB3B10 still calls.

inline TasSharedState* g_observerCaveState = nullptr;
static SafetyHookInline observerHook{};
static SafetyHookInline keyQueueHook{};

inline void UninstallObserverCave() {
    keyQueueHook = {};
    observerHook = {};
    if (g_observerCaveState) g_observerCaveState->observer_cave_hooked = 0;
    g_observerCaveState = nullptr;
}

void __fastcall Observer_Detour(void* ecx, void* edx, uint32_t keyIndex,
                                     uint32_t pressed, uint32_t unk, uint32_t arg4) {
    auto* s = g_observerCaveState;
    if (s) {
        s->bb3b10_call_count++;

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

// __fastcall(this, &{keyIndex, pressed}, &stamp), ret 4.
void __fastcall KeyQueue_Detour(void* ecx, const uint32_t* event, const void* stamp) {
    auto* s = g_observerCaveState;
    if (s && s->mode == MODE_PLAY && event[0] == GameAddresses::KEY_ESC && !IsTasInjectionThread()) {
        const uint8_t pressed = (uint8_t)event[1];
        WithHeld([=](volatile uint8_t* held) { held[GameAddresses::KEY_ESC] = pressed; });
        return;
    }
    keyQueueHook.fastcall<void>(ecx, event, stamp);
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

    keyQueueHook = safetyhook::create_inline(addr.key_queue_site, KeyQueue_Detour);
    if (!keyQueueHook) {
        Log("Observer cave: SafetyHook create_inline FAILED on the key queue (EXE+0x10950)");
        UninstallObserverCave();
        return false;
    }

    state->observer_cave_hooked = 1;
    Log("Observer cave: inline hook installed successfully");
    return true;
}
