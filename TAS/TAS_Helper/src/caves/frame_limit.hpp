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

// Defined below (it needs g_qpcFreq); declared here so the detour can call it.
inline void PreciseWaitUntil(LONGLONG targetQpc);

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
        // ...but a CONT's reload freeze lasts a second or two at most. A
        // cycle frozen for many seconds with the flag still up means the flag
        // is STALE — a judged cycle that never tore down (crashed harness,
        // session end, quit-to-menu mid-cycle). Left alone, a stale flag
        // disables this cap forever, and the menu video then runs uncapped:
        // half-speed dips on a fresh menu, ~3x after a level round-trip —
        // the reported "menu is sometimes slow and sometimes fast".
        if (contInFlight && (GetTickCount() - g_lastCycleMs) > 5000) {
            contInFlight = false;
        }

        if (cap > 0 && cycleFrozen && !contInFlight && g_qpcFreq.QuadPart) {
            const LONGLONG minTicks = g_qpcFreq.QuadPart / cap;
            LARGE_INTEGER now;
            QueryPerformanceCounter(&now);
            LONGLONG elapsed = now.QuadPart - g_lastPresentQpc;
            if (g_lastPresentQpc != 0 && elapsed >= 0 && elapsed < minTicks) {
                PreciseWaitUntil(g_lastPresentQpc + minTicks);
                QueryPerformanceCounter(&now);
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

// The throttle Sleeps, so it is only as accurate as the system timer. At the
// default ~15.6 ms granularity every wait rounds UP to the next quantum, and the
// cap lands nowhere near its target: cap=20 (50 ms) measured 62.4 ms = 4 quanta
// = 16 fps, cap=34 (29.4 ms) measured 31.2 ms = 2 quanta. Every observed present
// interval was a multiple of 15.6 ms, which is the signature.
//
// Raising the resolution to 1 ms makes Sleep accurate and the cap mean what it
// says. This must be done from INSIDE the game process: Windows 10 2004+/11
// made timeBeginPeriod PER-PROCESS, which is also why tas_ui raising it no
// longer affects the game (and why the original 2x stopped reproducing).
//
// Loaded dynamically so the project needs no new link dependency.
inline HMODULE g_winmm = nullptr;
inline void RaiseTimerResolution() {
    if (g_winmm) return;
    g_winmm = LoadLibraryA("winmm.dll");
    if (!g_winmm) return;
    using TimeFn = MMRESULT(WINAPI*)(UINT);
    auto begin = (TimeFn)GetProcAddress(g_winmm, "timeBeginPeriod");
    if (begin) {
        begin(1);
        Log("FrameLimit: timer resolution raised to 1ms (fallback wait path)");
    }
}

// WHY THE CAP VALUE MATTERS SO MUCH: THE MENU DECODER HAS A 40 ms GATE.
//
// The menu background is a decoded video, not a render, and Main_Menu.dll drives
// it as:
//     Menu::Cycle  -> ++tick_count
//     Menu::Paint  -> dt = tick_count * 0.01; tick_count = 0
//     Anim_Player::Cycle(dt) -> decode AT MOST ONE frame per Paint,
//                               and only once >= 0.04 s has accumulated
//
// So video speed is a STEP FUNCTION of the present interval, not proportional to
// it. Measured by sweeping the cap on a fresh menu (screen fps vs presents/s):
//
//     cap=18  55.6ms/Paint   18.0 presents  18.0 fps   <- 1:1, above the gate
//     cap=22  45.5ms         22.1           19.8
//     cap=25  40.0ms         25.0           15.6       <- falls off AT the gate
//     cap=28  35.7ms         28.0           17.2
//     cap=32  31.2ms         32.0           16.9       <- plateau ~17 fps
//     cap=40  25.0ms         40.1           17.1          regardless of presents
//
// Above ~40 ms per Paint every Paint crosses the gate and decodes; below it, the
// gate is crossed only every other Paint and the video runs at roughly half
// speed no matter how fast we present. That is why cap=34 (29.4 ms) looked
// "1.6x fast" in one state and slow in another, and why 20 (50 ms) lands
// cleanly above the gate and matches native in both. Do not raise the cap past
// ~24 without re-measuring: 25 is already the cliff edge.
//
// (Mechanism identified by codex from the retail binary; the sweep above is the
// hook-free confirmation. The remaining state-dependence — the same 31.2 ms
// Paint giving 16.9 fps fresh but 32.1 fps after a level round-trip — is
// consistent with the post-level dispatcher feeding MORE menu ticks per Paint,
// pushing dt over 0.04 every Paint. Not yet confirmed directly.)

// PREFERRED WAIT: a high-resolution waitable timer, NOT timeBeginPeriod.
//
// Accuracy here was a local problem — one Sleep in one hook — and raising the
// process-wide timer resolution is a global answer to it. That global answer is
// uncomfortably close to the original bug: a 1 ms timer speeding sr.dll's own
// Sleep-based limiter is what made the menu video run 2x in the first place. Fix
// it that way and the menu is only correct because the cap happens to clamp it —
// with the cap off after a level round-trip it measures 64 fps against a native
// 20, i.e. the old bug, still there, held back by a throttle.
//
// CREATE_WAITABLE_TIMER_HIGH_RESOLUTION (Win10 1803+) gives sub-millisecond
// waits WITHOUT changing anything process-wide, so the game's Sleep behaviour
// stays exactly as it is un-injected. timeBeginPeriod remains only as the
// fallback for an OS that lacks the flag.
#ifndef CREATE_WAITABLE_TIMER_HIGH_RESOLUTION
#define CREATE_WAITABLE_TIMER_HIGH_RESOLUTION 0x00000002
#endif

inline HANDLE g_waitTimer = nullptr;
inline bool   g_waitTimerHighRes = false;

inline void InitPrecisionWait() {
    if (g_waitTimer) return;
    g_waitTimer = CreateWaitableTimerExW(nullptr, nullptr,
                                         CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                                         TIMER_ALL_ACCESS);
    if (g_waitTimer) {
        g_waitTimerHighRes = true;
        Log("FrameLimit: high-resolution waitable timer (process timer untouched)");
        return;
    }
    // No high-res flag on this OS — fall back to the coarse timer plus a raised
    // process resolution, which is what makes Sleep usable at all.
    Log("FrameLimit: no high-res timer; falling back to timeBeginPeriod(1)");
    RaiseTimerResolution();
}

/// Block until QPC reaches `targetQpc`.
///
/// Waits for the bulk minus a margin, then spins the remainder. The margin is
/// what makes this correct at ANY timer granularity: the old code slept
/// `remain - 1ms`, which at the default ~15.6 ms quantum ROUNDS UP past the
/// target, so the following spin exited immediately and every interval landed on
/// a multiple of 15.6 ms.
inline void PreciseWaitUntil(LONGLONG targetQpc) {
    LARGE_INTEGER now;
    QueryPerformanceCounter(&now);
    if (now.QuadPart >= targetQpc) return;

    if (g_waitTimer) {
        LONGLONG remain = targetQpc - now.QuadPart;
        // QPC ticks -> 100ns units. remain is at most one frame, so no overflow.
        LONGLONG remain100ns = (remain * 10000000LL) / g_qpcFreq.QuadPart;
        // Leave enough slack to absorb wake-up jitter, then spin the rest.
        const LONGLONG margin100ns = g_waitTimerHighRes ? 5000LL     // 0.5 ms
                                                        : 20000LL;   // 2 ms
        if (remain100ns > margin100ns) {
            LARGE_INTEGER due;
            due.QuadPart = -(remain100ns - margin100ns);  // negative = relative
            if (SetWaitableTimer(g_waitTimer, &due, 0, nullptr, nullptr, FALSE)) {
                WaitForSingleObject(g_waitTimer, INFINITE);
            }
        }
    }
    do {
        QueryPerformanceCounter(&now);
    } while (now.QuadPart < targetQpc);
}

inline bool InstallFrameLimit(TasSharedState* state) {
    g_flState = state;
    QueryPerformanceFrequency(&g_qpcFreq);
    InitPrecisionWait();

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

// Install on FIRST USE rather than at DLL init.
//
// Measured: the hook costs ~12 ms per frame on the FRESH main menu merely by
// existing (48.5 ms without it, 59.9 ms with it even at cap=0, so it is not the
// throttle). But it is genuinely needed AFTER a level round-trip, where the menu
// video becomes present-locked and runs at 64 fps against a native 20.
//
// Those two facts only reconcile if the hook is absent until a level has
// actually been loaded. Before that it can do nothing useful, and it demonstrably
// does harm. `levelscan` calls this the first time it identifies a track.
inline bool g_frameLimitInstalled = false;
inline void EnsureFrameLimitInstalled(TasSharedState* state) {
    if (g_frameLimitInstalled) return;
    g_frameLimitInstalled = true;   // set first: never retry a failed install
    if (InstallFrameLimit(state)) {
        Log("FrameLimit: installed on first level load (menu video is present-locked after this)");
    } else {
        Log("FrameLimit: first-level-load install FAILED");
    }
}
