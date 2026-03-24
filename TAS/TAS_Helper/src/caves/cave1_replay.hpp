#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Replay object capture hook at SG+0x9E8F0.
// Original instruction: sub esp, 00000080 (6 bytes).
// At this site, ECX holds the replay/Supreme object pointer.
// We capture it so Cave 2 can derive the player pointer from [replayObj+0x84].
//
// Ported from CE "replayCave": keeps the replay pointer stable while
// REC/PLAY is active (doesn't overwrite mid-run).

inline TasSharedState* g_replayState = nullptr;
static SafetyHookMid replayCaptureHook{};

bool InstallReplayCapture(GameAddresses& addr, TasSharedState* state) {
    if (!addr.replay_capture_site) {
        Log("Replay capture: hook site not resolved");
        return false;
    }

    g_replayState = state;
    Log(std::format("Replay capture: hooking at {:p} (SG+0x9E8F0)", (void*)addr.replay_capture_site));

    replayCaptureHook = safetyhook::create_mid(addr.replay_capture_site, [](SafetyHookContext& ctx) {
        auto* s = g_replayState;
        if (!s) return;

        // ECX holds the replay object at this hook site.
        auto newPtr = (uint32_t)ctx.ecx;

        // Keep replay pointer stable while REC/PLAY is active.
        // Only update when idle or when we don't have a pointer yet.
        if (s->mode == MODE_OFF || s->replay_ptr == 0) {
            s->replay_ptr = newPtr;
        }
    });

    if (!replayCaptureHook) {
        Log("Replay capture: SafetyHook create_mid FAILED");
        return false;
    }

    state->replay_capture_hooked = 1;
    Log("Replay capture: hook installed successfully");
    return true;
}
