#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../input_gate.hpp"
#include "../external/safetyhook.hpp"

// Cave 1C: Handler gate hooks at HMG+3940 (keyDown) and HMG+3980 (keyUp).
//
// During REC (mode=1) and PLAY (mode=2): blocks external handler calls
//   unless the current thread is inside a Cave 2 injection scope.
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

inline void UninstallCave1C() {
    cave1cUpInline = {};
    cave1cDownInline = {};
    if (g_cave1cState) g_cave1cState->cave1c_hooked = 0;
    g_cave1cState = nullptr;
}

// Handler signature emulated via __fastcall:
//   ecx = this, edx = unused, stack: arg1, arg2, arg3
void __fastcall Cave1C_DownDetour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    uint64_t t0 = __rdtsc();
    auto* s = g_cave1cState;
    // Block external handler during REC and PLAY (symmetric).
    // Cave 2 writes the buffer and calls BB3B10 directly in both modes.
    // Only pass through on Cave 2's injection thread or when mode is IDLE.
    // Fallback calibration: a3 is the hi dword of the Kernel::Time the event
    // was stamped with (the handler forwards its Time args verbatim to
    // BB3B10). Injection normally stamps with Kernel::Time::Current() and
    // never reads this; the observed value only backs up injection if that
    // export ever fails to resolve.
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // ESC passthrough: cave1c's REC/PLAY block exists to keep GAMEPLAY input
    // symmetric between REC and PLAY (cave2 owns the DI buffer + BB3B10).
    // ESC is not a gameplay key — the boarder never reads it — but blocking
    // it made the pause menu unreachable during REC ("Escape key does not
    // work with record"). Let the real handler process it; a1 is the Win32 VK
    // (the dispatcher forwards wParam — see the Translate RE).
    //
    // Gate the real key event (policy in input_gate.hpp — unit-tested).
    if (s && ShouldBlockRealInput({
            s->mode,
            s->cont_suppress_input != 0,
            IsTasInjectionThread(),
            (GetTickCount() - g_lastCycleMs) > 250,
            a1 == VK_ESCAPE,
        })) {
        s->handler_block_count++;
        PerfSample(s->perf_cave1c_down, __rdtsc() - t0);
        return;
    }
    cave1cDownInline.thiscall<void>(ecx, a1, a2, a3);
    if (s) {
        PerfSample(s->perf_cave1c_down, __rdtsc() - t0);
    }
}

void __fastcall Cave1C_UpDetour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    uint64_t t0 = __rdtsc();
    auto* s = g_cave1cState;
    // Keep the fallback arg4 fresh from real keyUp calls too (see DownDetour).
    if (s && !IsTasInjectionThread() && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // Keep down/up symmetric (see DownDetour) — same gate policy. A key
    // RELEASED during a CONT must be blocked too, else a press blocked on the
    // way down but released after the flag clears lands an unbalanced up event.
    if (s && ShouldBlockRealInput({
            s->mode,
            s->cont_suppress_input != 0,
            IsTasInjectionThread(),
            (GetTickCount() - g_lastCycleMs) > 250,
            a1 == VK_ESCAPE,
        })) {
        s->handler_block_count++;
        PerfSample(s->perf_cave1c_up, __rdtsc() - t0);
        return;
    }
    cave1cUpInline.thiscall<void>(ecx, a1, a2, a3);
    if (s) {
        PerfSample(s->perf_cave1c_up, __rdtsc() - t0);
    }
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
        g_cave1cState = nullptr;
        return false;
    }

    cave1cUpInline = safetyhook::create_inline(addr.cave1c_up, Cave1C_UpDetour);
    if (!cave1cUpInline) {
        Log("Cave 1C: SafetyHook create_inline FAILED on keyUp (+3980)");
        // All-or-none: leaving keyDown intercepted without the matching keyUp
        // path creates stuck/asymmetric input while cave1c_hooked still says no.
        UninstallCave1C();
        return false;
    }

    state->cave1c_hooked = 1;
    Log("Cave 1C: both inline hooks installed successfully");
    return true;
}
