#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include <safetyhook.hpp>

// The key-handler cave: Handler gate hooks at HMG+3940 (keyDown) and HMG+3980 (keyUp).
//
// During REC (mode=1) and PLAY (mode=2): blocks external handler calls
//   unless the current thread is inside the cycle cave's injection scope.
//   Symmetric blocking ensures +3940 never writes the buffer outside the cycle cave,
//   eliminating the one-frame timing difference between REC and PLAY.
// During IDLE (mode=0): passes through.
//
// Uses SafetyHookInline so we can skip the original function entirely when blocking.
// The handlers are __thiscall with 3 stack args (ret 000C).
// Detours use __fastcall with dummy EDX to emulate thiscall calling convention.

inline TasSharedState* g_keyHandlerCaveState = nullptr;

// Inline hooks for keyDown (+3940) and keyUp (+3980)
static SafetyHookInline keyDownHook{};
static SafetyHookInline keyUpHook{};

inline void UninstallKeyHandlerCave() {
    keyUpHook = {};
    keyDownHook = {};
    if (g_keyHandlerCaveState) g_keyHandlerCaveState->key_handler_cave_hooked = 0;
    g_keyHandlerCaveState = nullptr;
}

// Handler signature emulated via __fastcall:
//   ecx = this, edx = unused, stack: arg1, arg2, arg3
void __fastcall KeyDown_Detour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    auto* s = g_keyHandlerCaveState;
    // Block external handler during REC and PLAY (symmetric).
    // The cycle cave writes the buffer and calls BB3B10 directly in both modes.
    // Only pass through on the cycle cave's injection thread or when mode is IDLE.
    // Fallback calibration: a3 is the hi dword of the Kernel::Time the event
    // was stamped with (the handler forwards its Time args verbatim to
    // BB3B10). Injection normally stamps with Kernel::Time::Current() and
    // never reads this; the observed value only backs up injection if that
    // export ever fails to resolve.
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // Gate the real key event (policy in input_gate.hpp, unit-tested). ESC
    // always passes: it is not a gameplay key, and the pause menu needs it
    // during REC. a1 is the Win32 VK (the dispatcher forwards wParam).
    if (s && ShouldBlockRealInput({
            s->mode,
            s->cont_suppress_input != 0,
            IsTasInjectionThread(),
            (GetTickCount() - g_lastCycleMs) > 250,
            a1 == VK_ESCAPE,
        })) {
        s->handler_block_count++;
        return;
    }
    keyDownHook.thiscall<void>(ecx, a1, a2, a3);
}

void __fastcall KeyUp_Detour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    auto* s = g_keyHandlerCaveState;
    // Keep the fallback arg4 fresh from real keyUp calls too (see DownDetour).
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // Same gate policy as DownDetour. A release during a CONT is blocked too,
    // or a press blocked on the way down would land an unbalanced up event.
    if (s && ShouldBlockRealInput({
            s->mode,
            s->cont_suppress_input != 0,
            IsTasInjectionThread(),
            (GetTickCount() - g_lastCycleMs) > 250,
            a1 == VK_ESCAPE,
        })) {
        s->handler_block_count++;
        return;
    }
    keyUpHook.thiscall<void>(ecx, a1, a2, a3);
}

bool InstallKeyHandlerCave(GameAddresses& addr, TasSharedState* state) {
    if (!addr.key_down_site || !addr.key_up_site) {
        Log("Key-handler cave: hook sites not resolved");
        return false;
    }

    g_keyHandlerCaveState = state;
    Log(std::format("Key-handler cave: hooking keyDown at {:p} (HMG+3940)", (void*)addr.key_down_site));
    Log(std::format("Key-handler cave: hooking keyUp at {:p} (HMG+3980)", (void*)addr.key_up_site));

    keyDownHook = safetyhook::create_inline(addr.key_down_site, KeyDown_Detour);
    if (!keyDownHook) {
        Log("Key-handler cave: SafetyHook create_inline FAILED on keyDown (+3940)");
        g_keyHandlerCaveState = nullptr;
        return false;
    }

    keyUpHook = safetyhook::create_inline(addr.key_up_site, KeyUp_Detour);
    if (!keyUpHook) {
        Log("Key-handler cave: SafetyHook create_inline FAILED on keyUp (+3980)");
        // All-or-none: leaving keyDown intercepted without the matching keyUp
        // path creates stuck/asymmetric input while key_handler_cave_hooked still says no.
        UninstallKeyHandlerCave();
        return false;
    }

    state->key_handler_cave_hooked = 1;
    Log("Key-handler cave: both inline hooks installed successfully");
    return true;
}
