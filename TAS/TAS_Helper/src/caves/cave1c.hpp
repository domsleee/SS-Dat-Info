#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

// Cave 1C: Handler gate hooks at HMG+3940 (keyDown) and HMG+3980 (keyUp).
//
// During REC (mode=1) and PLAY (mode=2): blocks external handler calls
//   UNLESS cave2_injecting=1 (Cave 2 direct call passthrough).
//   Symmetric blocking ensures +3940 never writes the buffer outside Cave 2,
//   eliminating the one-frame timing difference between REC and PLAY.
// During IDLE (mode=0): passes through.
//
// Uses SafetyHookInline so we can skip the original function entirely when blocking.
// The handlers are __thiscall with 3 stack args (ret 000C).
// Detours use __fastcall with dummy EDX to emulate thiscall calling convention.

inline TasSharedState* g_cave1cState = nullptr;

// Inline hooks for keyDown (+3940) and keyUp (+3980)
static SafetyHookInline cave1cDownInline{};
static SafetyHookInline cave1cUpInline{};

// Handler signature emulated via __fastcall:
//   ecx = this, edx = unused, stack: arg1, arg2, arg3
void __fastcall Cave1C_DownDetour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    auto* s = g_cave1cState;
    // Block external handler during REC and PLAY (symmetric).
    // Cave 2 writes the buffer and calls BB3B10 directly in both modes.
    // Only pass through when cave2_injecting=1 (Cave 2's own BB3B10 calls)
    // or when mode is IDLE.
    if (s && s->mode != MODE_OFF && !s->cave2_injecting) {
        s->handler_block_count++;
        return;
    }
    cave1cDownInline.thiscall<void>(ecx, a1, a2, a3);
}

void __fastcall Cave1C_UpDetour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    auto* s = g_cave1cState;
    if (s && s->mode != MODE_OFF && !s->cave2_injecting) {
        s->handler_block_count++;
        return;
    }
    cave1cUpInline.thiscall<void>(ecx, a1, a2, a3);
}

bool InstallCave1C(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave1c_down || !addr.cave1c_up) {
        Log("Cave 1C: hook sites not resolved");
        return false;
    }

    g_cave1cState = state;
    Log(std::format("Cave 1C: hooking keyDown at {:p} (HMG+3940)", (void*)addr.cave1c_down));
    Log(std::format("Cave 1C: hooking keyUp at {:p} (HMG+3980)", (void*)addr.cave1c_up));

    cave1cDownInline = safetyhook::create_inline(addr.cave1c_down, Cave1C_DownDetour);
    if (!cave1cDownInline) {
        Log("Cave 1C: SafetyHook create_inline FAILED on keyDown (+3940)");
        return false;
    }

    cave1cUpInline = safetyhook::create_inline(addr.cave1c_up, Cave1C_UpDetour);
    if (!cave1cUpInline) {
        Log("Cave 1C: SafetyHook create_inline FAILED on keyUp (+3980)");
        return false;
    }

    state->cave1c_hooked = 1;
    Log("Cave 1C: both inline hooks installed successfully");
    return true;
}
