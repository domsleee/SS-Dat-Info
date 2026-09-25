#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include <safetyhook.hpp>

// The key-handler cave: inline hooks on HMG+3940 (keyDown) and HMG+3980 (keyUp).
// During REC and PLAY, real key events are blocked (input_gate.hpp) so only the
// cycle cave writes the buffer, and REC and PLAY see input on the same tick.
// The handlers are __thiscall with 3 stack args (ret 000C); the __fastcall
// detours take a dummy EDX.

inline TasSharedState* g_keyHandlerCaveState = nullptr;

static SafetyHookInline keyDownHook{};
static SafetyHookInline keyUpHook{};

inline void UninstallKeyHandlerCave() {
    keyUpHook = {};
    keyDownHook = {};
    if (g_keyHandlerCaveState) g_keyHandlerCaveState->key_handler_cave_hooked = 0;
    g_keyHandlerCaveState = nullptr;
}

// a1 = Win32 VK (wParam), a3 = Kernel::Time.hi of the event stamp.
void __fastcall KeyDown_Detour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    auto* s = g_keyHandlerCaveState;
    // Calibrate the fallback stamp (see g_bb3b10Arg4).
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
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
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // Releases are gated too, so a blocked press never gets an unbalanced up.
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
        // All or none: keyDown hooked without keyUp would leave keys stuck.
        UninstallKeyHandlerCave();
        return false;
    }

    state->key_handler_cave_hooked = 1;
    Log("Key-handler cave: both inline hooks installed successfully");
    return true;
}
