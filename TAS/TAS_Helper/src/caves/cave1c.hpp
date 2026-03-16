#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 1C: Handler gate hooks at HMG+3940 (keyDown) and HMG+3980 (keyUp).
// During PLAY (mode=2): blocks external handler calls.
// During REC (mode=1): passes through for Pico HID -> BB3B10 observer parity.
// Also passes when cave2_injecting=1 (playback injection active).

inline TasSharedState* g_cave1cState = nullptr;
static SafetyHookMid cave1cDownHook{};
static SafetyHookMid cave1cUpHook{};

bool InstallCave1C(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave1c_down || !addr.cave1c_up) {
        Log("Cave 1C: hook sites not resolved");
        return false;
    }

    g_cave1cState = state;
    Log(std::format("Cave 1C: hooking keyDown at {:p}", (void*)addr.cave1c_down));
    Log(std::format("Cave 1C: hooking keyUp at {:p}", (void*)addr.cave1c_up));

    cave1cDownHook = safetyhook::create_mid(addr.cave1c_down, [](SafetyHookContext& ctx) {
        auto* s = g_cave1cState;
        if (!s) return;
        s->handler_block_count++;
        // Full blocking logic in Phase 2:
        // if mode == PLAY && !cave2_injecting: block by modifying return
    });

    cave1cUpHook = safetyhook::create_mid(addr.cave1c_up, [](SafetyHookContext& ctx) {
        auto* s = g_cave1cState;
        if (!s) return;
        s->handler_block_count++;
    });

    if (!cave1cDownHook) {
        Log("Cave 1C: SafetyHook create_mid FAILED on keyDown (+3940)");
        return false;
    }
    if (!cave1cUpHook) {
        Log("Cave 1C: SafetyHook create_mid FAILED on keyUp (+3980)");
        return false;
    }

    state->cave1c_hooked = 1;
    Log("Cave 1C: both hooks installed successfully");
    return true;
}
