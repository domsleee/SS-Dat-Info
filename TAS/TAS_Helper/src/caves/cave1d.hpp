#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 1D: BB3B10 observer notification gate at HMG+3B10.
// This is the observer notification function that the game calls on key state changes.
// During PLAY: Cave 2 calls this directly on transitions for steering parity.
// This hook monitors calls for diagnostics.

inline TasSharedState* g_cave1dState = nullptr;
static SafetyHookMid cave1dHook{};

bool InstallCave1D(GameAddresses& addr, TasSharedState* state) {
    if (!addr.bb3b10) {
        Log("Cave 1D: hook site not resolved");
        return false;
    }

    g_cave1dState = state;
    Log(std::format("Cave 1D: hooking BB3B10 at {:p}", (void*)addr.bb3b10));

    cave1dHook = safetyhook::create_mid(addr.bb3b10, [](SafetyHookContext& ctx) {
        auto* s = g_cave1dState;
        if (!s) return;
        s->bb3b10_call_count++;
    });

    if (!cave1dHook) {
        Log("Cave 1D: SafetyHook create_mid FAILED on BB3B10 (+3B10)");
        return false;
    }

    state->cave1d_hooked = 1;
    Log("Cave 1D: hook installed successfully");
    return true;
}
