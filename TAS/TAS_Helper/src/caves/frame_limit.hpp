#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../external/safetyhook.hpp"
#include "cave2.hpp"  // g_lastCycleMs (engine-cycle heartbeat)

// ---------------------------------------------------------------------------
// Frame-limit cave — menu present-rate cap.
//
// The renderer (srDD_OpenGL.DLL) presents each frame via gdi32!SwapBuffers.
// The game runs WINDOWED, so DWM vsyncs every present to the desktop refresh
// (e.g. 165 Hz). Gameplay is unaffected — its physics run on a fixed-timestep
// accumulator (cave5 disasm: cmp esi,14h tick clamp), which is refresh-
// independent. But the MAIN-MENU attract demo advances one video frame per
// present, so at 165 Hz it plays ~2.7x too fast. Neither TAS_Helper nor
// Display_Config_Helper hooks the present path, so this is purely the windowed
// high-refresh present rate (proven: at the menu cave2/cave5 counters are
// frozen, yet the video animates).
//
// Fix: hook SwapBuffers and, ONLY while the engine cycle is frozen (menu /
// pause — Supreme::Cycle hasn't ticked in >250 ms, same heartbeat cave1c/1d
// use), throttle the present rate to menu_fps_cap. During gameplay the cycle
// ticks every frame, so the heartbeat is fresh and we never throttle — full
// refresh-rate rendering is preserved. During a CONT catch-up the cycle also
// ticks (fast), so catch-up is untouched.
//
// present_count is incremented on every present (diagnostic: lets a poller
// measure the live present rate at the menu vs in-game). menu_fps_cap is a
// UI/config field: 0 = OFF (no throttle, pure measurement), N = cap to N fps.
// ---------------------------------------------------------------------------

inline TasSharedState*  g_flState = nullptr;
inline SafetyHookInline g_swapHook{};
inline LARGE_INTEGER    g_qpcFreq{};
inline LONGLONG         g_lastPresentQpc = 0;

inline BOOL WINAPI SwapBuffers_Detour(HDC hdc) {
    auto* s = g_flState;
    if (s) {
        s->present_count++;

        uint32_t cap = s->menu_fps_cap;
        // Menu / pause = engine cycle frozen. The Supreme::Cycle hook stamps
        // g_lastCycleMs every gameplay tick; >250 ms stale means no gameplay
        // is advancing (main menu, pause menu, dialog, level load).
        bool cycleFrozen = (GetTickCount() - g_lastCycleMs) > 250;

        // NEVER throttle during a CONT. cont_suppress_input is set for the
        // whole Continue cycle (before the F5 restart through the splice). A
        // CONT's F5 reload briefly freezes the cycle, and the F5 spawn bucket
        // is sub-tick timing sensitive — an added present-Sleep in that window
        // could shift the lottery. Gating on !cont_suppress_input makes the
        // throttle provably CONT-safe (catch-up replay ticks fast anyway, so it
        // wouldn't trip cycleFrozen, but the reload gap could — this closes it).
        bool contInFlight = s->cont_suppress_input != 0;

        if (cap > 0 && cycleFrozen && !contInFlight && g_qpcFreq.QuadPart) {
            const LONGLONG minTicks = g_qpcFreq.QuadPart / cap;
            LARGE_INTEGER now;
            QueryPerformanceCounter(&now);
            LONGLONG elapsed = now.QuadPart - g_lastPresentQpc;
            if (g_lastPresentQpc != 0 && elapsed >= 0 && elapsed < minTicks) {
                // Sleep the bulk (coarse), then spin the final <1 ms for accuracy.
                LONGLONG remainMs = ((minTicks - elapsed) * 1000) / g_qpcFreq.QuadPart;
                if (remainMs > 1) Sleep((DWORD)(remainMs - 1));
                do {
                    QueryPerformanceCounter(&now);
                } while (now.QuadPart - g_lastPresentQpc < minTicks);
            }
            g_lastPresentQpc = now.QuadPart;
        } else {
            LARGE_INTEGER now;
            QueryPerformanceCounter(&now);
            g_lastPresentQpc = now.QuadPart;
        }
    }
    return g_swapHook.stdcall<BOOL, HDC>(hdc);
}

inline bool InstallFrameLimit(TasSharedState* state) {
    g_flState = state;
    QueryPerformanceFrequency(&g_qpcFreq);

    HMODULE gdi = GetModuleHandleA("gdi32.dll");
    if (!gdi) {
        Log("FrameLimit: gdi32.dll not loaded; SwapBuffers hook skipped");
        return false;
    }
    auto* fn = GetProcAddress(gdi, "SwapBuffers");
    if (!fn) {
        Log("FrameLimit: gdi32!SwapBuffers not found");
        return false;
    }

    g_swapHook = safetyhook::create_inline((void*)fn, (void*)SwapBuffers_Detour);
    if (!g_swapHook) {
        Log("FrameLimit: SafetyHook create_inline on SwapBuffers FAILED");
        return false;
    }
    Log(std::format("FrameLimit: gdi32!SwapBuffers hooked at {:p} (menu_fps_cap={})",
        (void*)fn, state->menu_fps_cap));
    return true;
}
