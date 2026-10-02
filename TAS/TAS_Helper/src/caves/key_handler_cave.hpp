#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include "cycle_cave.hpp"
#include "menu_cave.hpp"
#include <safetyhook.hpp>

// The key-handler cave: inline hooks on HMG+3940 (keyDown) and HMG+3980 (keyUp).
// During PLAY and a CONT, real key events are blocked (input_gate.hpp) so only
// the cycle cave drives the keys. REC lets the six recorded keys through and
// records the result; other keys stay blocked, as they would not be in the take.
// The handlers are __thiscall with 3 stack args (ret 000C); the __fastcall
// detours take a dummy EDX.

inline TasSharedState* g_keyHandlerCaveState = nullptr;

// Shared with the observer cave; see IsMenuPause.
inline bool MenuPauseNow() {
    const uint64_t last_menu = menustate::g_lastExecuteMs;
    return IsMenuPause(GetTickCount() - g_lastCycleMs, last_menu != 0,
                       GetTickCount64() - last_menu);
}

static SafetyHookInline keyDownHook{};
static SafetyHookInline keyUpHook{};

inline void UninstallKeyHandlerCave() {
    keyUpHook = {};
    keyDownHook = {};
    if (g_keyHandlerCaveState) g_keyHandlerCaveState->key_handler_cave_hooked = 0;
    g_keyHandlerCaveState = nullptr;
}

// True (and counted) when a real key event for `vk` must not reach the game.
// Releases are gated too, so a blocked press never gets an unbalanced up.
static bool BlockReal(uint32_t vk, bool release) {
    auto* s = g_keyHandlerCaveState;
    if (!s || !ShouldBlockRealInput({
            s->mode,
            s->cont_suppress_input != 0,
            IsTasInjectionThread(),
            MenuPauseNow(),
            vk == VK_ESCAPE,
            IsTasVirtualKey(vk),
            release,
        })) {
        return false;
    }
    s->handler_block_count++;
    return true;
}

// a1 = Win32 VK (wParam), a3 = Kernel::Time.hi of the event stamp.
void __fastcall KeyDown_Detour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    if (!BlockReal(a1, false)) keyDownHook.thiscall<void>(ecx, a1, a2, a3);
}

void __fastcall KeyUp_Detour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    if (!BlockReal(a1, true)) keyUpHook.thiscall<void>(ecx, a1, a2, a3);
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
