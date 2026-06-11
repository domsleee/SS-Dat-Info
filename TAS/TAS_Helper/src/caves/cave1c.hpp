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
// RDIAG: last `this` seen by the REAL native keyDown handler while in OFF mode
// — the object the game actually routes live keyboard input through. Compared
// against the kbobj cave2 resolves via the root chain ([SG+1D5450]->+0x530) to
// find where the two diverge after an in-process restart (the
// steering-dies-after-REC investigation).
inline volatile uint32_t g_lastRealHandlerThis = 0;

void __fastcall Cave1C_DownDetour(void* ecx, void* edx, uint32_t a1, uint32_t a2, uint32_t a3) {
    uint64_t t0 = __rdtsc();
    auto* s = g_cave1cState;
    // Block external handler during REC and PLAY (symmetric).
    // Cave 2 writes the buffer and calls BB3B10 directly in both modes.
    // Only pass through when cave2_injecting=1 (Cave 2's own BB3B10 calls)
    // or when mode is IDLE.
    // Fallback calibration: a3 is the hi dword of the Kernel::Time the event
    // was stamped with (the handler forwards its Time args verbatim to
    // BB3B10). Injection normally stamps with Kernel::Time::Current() and
    // never reads this; the observed value only backs up injection if that
    // export ever fails to resolve.
    if (s && !s->cave2_injecting && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // ESC passthrough: cave1c's REC/PLAY block exists to keep GAMEPLAY input
    // symmetric between REC and PLAY (cave2 owns the DI buffer + BB3B10).
    // ESC is not a gameplay key — the boarder never reads it — but blocking
    // it made the pause menu unreachable during REC ("Escape key does not
    // work with record"). Let the real handler process it; a1 is the Win32 VK
    // (the dispatcher forwards wParam — see the Translate RE).
    //
    // Pause passthrough: when Supreme::Cycle hasn't ticked recently the game
    // is paused (pause menu / dialog / static menu) — the sim isn't running,
    // so blocking buys no REC/PLAY symmetry and only makes the pause menu
    // (and a stuck-mode session) unnavigable. Pass everything through.
    //
    // CONT countdown coverage: cont_suppress_input extends the block into OFF
    // mode while a Continue is in flight — the post-F5 spawn countdown is OFF,
    // and a stray live keypress there perturbs the spawn so the bucket matches
    // first-moving but diverges right after the judge window. Live input must
    // be inert from CONT start until the splice; the UI clears the flag the
    // instant the bucket aligns, so post-splice REC still records live input.
    bool gamePaused = (GetTickCount() - g_lastCycleMs) > 250;
    bool contCountdown = s && s->cont_suppress_input;
    if (s && (s->mode != MODE_OFF || contCountdown)
        && !s->cave2_injecting && a1 != VK_ESCAPE && !gamePaused) {
        s->handler_block_count++;
        PerfSample(s->perf_cave1c_down, __rdtsc() - t0);
        return;
    }
    // RDIAG: record the real handler's `this` AND raw args on OFF-mode
    // passthroughs, ring a log when (this,a1) changes (hand-rolled hex — hook
    // context). a1/a2/a3 are compared against the arg4 the real BB3B10 call
    // carries (cave1d log) — the handler forwards one of its own args as arg4,
    // and identifying which one lets us calibrate the injected arg4 even from
    // BLOCKED keypresses during REC (the handler detour still sees the args).
    if (s && s->mode == MODE_OFF && !s->cave2_injecting) {
        uint32_t cur = ((uint32_t)(uintptr_t)ecx) ^ (a1 << 1);
        if (cur != g_lastRealHandlerThis) {
            g_lastRealHandlerThis = cur;
            char buf[112];
            int p = 0;
            auto put = [&](const char* t) { while (*t && p < 100) buf[p++] = *t++; };
            put("RDIAG real-h this=");
            DiagHexU32(buf + p, (uint32_t)(uintptr_t)ecx); p += 8;
            put(" a1=");  DiagHexU32(buf + p, a1); p += 8;
            put(" a2=");  DiagHexU32(buf + p, a2); p += 8;
            put(" a3=");  DiagHexU32(buf + p, a3); p += 8;
            // The CALLER of +3940 — disassembling around this return address
            // shows where a3 (the dynamic arg4 counter) is LOADED from, which
            // is the memory location cave2 should read live instead of
            // calibrating from keypresses.
            put(" ret=");
            DiagHexU32(buf + p, (uint32_t)(uintptr_t)_ReturnAddress()); p += 8;
            buf[p] = '\0';
            LogRing(s, LOG_INFO, buf);
        }
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
    if (s && !s->cave2_injecting && !s->test_arg4_override && a3 != g_bb3b10Arg4) {
        g_bb3b10Arg4 = a3;
    }
    // ESC + pause passthrough + CONT-countdown block — keep down/up symmetric
    // (see DownDetour). A key RELEASED during the countdown must be blocked
    // too, else a press blocked on the way down but released after the flag
    // clears would land an unbalanced up event on the spawn.
    bool gamePaused = (GetTickCount() - g_lastCycleMs) > 250;
    bool contCountdown = s && s->cont_suppress_input;
    if (s && (s->mode != MODE_OFF || contCountdown)
        && !s->cave2_injecting && a1 != VK_ESCAPE && !gamePaused) {
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
