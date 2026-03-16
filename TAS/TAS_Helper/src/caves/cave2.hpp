#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 2: Supreme::Cycle hook (SG+0x13FE40)
// Main REC/PLAY engine. During REC: samples DI buffer, records mask.
// During PLAY: reads input log, writes DI buffer + action_state, calls BB3B10.

// Global pointer accessed by non-capturing lambda
inline TasSharedState* g_cave2State = nullptr;
static SafetyHookMid cave2Hook{};

bool InstallCave2(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave2_site) {
        Log("Cave 2: hook site not resolved");
        return false;
    }

    g_cave2State = state;
    Log(std::format("Cave 2: hooking Supreme::Cycle at {:p}", (void*)addr.cave2_site));

    cave2Hook = safetyhook::create_mid(addr.cave2_site, [](SafetyHookContext& ctx) {
        auto* s = g_cave2State;
        if (!s) return;

        // Increment frame counter for liveness
        s->frame_count++;

        // Phase 1: just prove the hook fires.
        // Full REC/PLAY logic will be ported in Phase 2.
    });

    if (!cave2Hook) {
        Log("Cave 2: SafetyHook create_mid FAILED");
        return false;
    }

    state->cave2_hooked = 1;
    Log("Cave 2: hook installed successfully");
    return true;
}
