// Present gate: stop the game showing its own unfinished frames.
//
// Root cause of "missing terrain when holding F5 / fast-forwarding a replay"
// at high render distance: during restart processing Supreme PRESENTS back
// buffers that are still sky-cleared or only partially drawn. Measured with
// glReadPixels on presented frames: settled frames read depth ~0.95 and snow
// colors at fixed probe points; the torn frames read depth 1.0 / pure black -
// the buffer was presented before the ground was emitted. The geometry
// pipeline itself is fine (extents, strips and submission were all verified);
// the serrated "missing terrain" is simply a half-drawn frame on screen.
//
// Fix: hook wglSwapBuffers and probe three points on the centre column of the
// LOWER half of the frame (the engine emits ground far-to-near, so unfinished
// frames are always still sky-cleared low even when the far slope is drawn;
// settled gameplay always has ground or the rider there). If all three points
// are still cleared, the present is skipped and the screen keeps the last
// complete frame.
#pragma once

#include <windows.h>
#include <cstdint>
#include <format>

#include "log.hpp"
#include "external/safetyhook.hpp"

namespace presentgate {

typedef void(__stdcall* glReadPixels_t)(int, int, int, int, unsigned, unsigned, void*);
typedef void(__stdcall* glGetIntegerv_t)(unsigned, int*);
constexpr unsigned GL_VIEWPORT_E = 0x0BA2;
constexpr unsigned GL_DEPTH_COMPONENT_E = 0x1902;
constexpr unsigned GL_RGB_E = 0x1907;
constexpr unsigned GL_FLOAT_E = 0x1406;
constexpr unsigned GL_UNSIGNED_BYTE_E = 0x1401;

inline glReadPixels_t  g_readPixels = nullptr;
inline glGetIntegerv_t g_getIntegerv = nullptr;
inline SafetyHookInline g_swapHook{};

inline LONG g_totalSkips = 0;

inline bool frameLooksUnfinished(void* hdc) {
    // Probe positions must come from the WINDOW size, not GL_VIEWPORT: the
    // viewport at present time is whatever the last render pass set (HUD or
    // sub-passes leave small rectangles behind), which would relocate the
    // probes unpredictably. The window client rect is stable.
    HWND wnd = WindowFromDC((HDC)hdc);
    RECT rc = {};
    if (!wnd || !GetClientRect(wnd, &rc)) return false;
    const int w = rc.right - rc.left, h = rc.bottom - rc.top;
    if (w < 64 || h < 64) return false;
    // 8x8 blocks (not single pixels): snowflake sprites, stars and overlay
    // glyphs can cover an individual pixel of empty sky, so a point only
    // counts as drawn if a meaningful part of its block is drawn. A block is
    // "still cleared" when >=52 of its 64 pixels are black AND far-depth.
    const int xs = w / 2 - 4;
    const int ys[3] = { h * 10 / 100, h * 25 / 100, h * 45 / 100 };
    for (int i = 0; i < 3; i++) {
        float d[64] = {};
        unsigned char rgb[64 * 4] = {};
        g_readPixels(xs, ys[i], 8, 8, GL_DEPTH_COMPONENT_E, GL_FLOAT_E, d);
        g_readPixels(xs, ys[i], 8, 8, GL_RGB_E, GL_UNSIGNED_BYTE_E, rgb);
        int cleared = 0;
        for (int p = 0; p < 64; p++) {
            if (d[p] >= 0.99999f && rgb[p * 3] < 8 && rgb[p * 3 + 1] < 8 && rgb[p * 3 + 2] < 8)
                cleared++;
        }
        if (cleared < 52) return false;   // this block has real content (not just sprites)
    }
    return true;
}

// No skip cap: a cleared-and-undrawn buffer is the ONLY state that matches the
// probe (legitimate dark frames - fades, night sky shots, menus - all carry
// scene depth < 1.0 or non-black pixels), so holding a restart for any length
// of time simply keeps the last complete frame on screen. A cap was tried and
// regressed: it eventually presented raw cleared buffers during long holds.
inline int __stdcall SwapDetour(void* hdc) {
    if (frameLooksUnfinished(hdc)) {
        g_totalSkips++;
        return 1;   // report success without presenting the unfinished frame
    }
    return g_swapHook.stdcall<int, void*>(hdc);
}

inline DWORD WINAPI InstallThread(LPVOID) {
    // opengl32 loads lazily well after injection - poll for it.
    HMODULE gl = nullptr;
    for (int i = 0; i < 600 && !gl; i++) {   // up to ~60s
        gl = GetModuleHandleA("opengl32.dll");
        if (!gl) Sleep(100);
    }
    if (!gl) {
        Log("PresentGate: opengl32.dll never loaded - not installed");
        return 0;
    }
    g_readPixels  = (glReadPixels_t)GetProcAddress(gl, "glReadPixels");
    g_getIntegerv = (glGetIntegerv_t)GetProcAddress(gl, "glGetIntegerv");
    auto* swap = GetProcAddress(gl, "wglSwapBuffers");
    if (!swap || !g_readPixels || !g_getIntegerv) {
        Log("PresentGate: missing GL entry points - not installed");
        return 0;
    }
    g_swapHook = safetyhook::create_inline((void*)swap, (void*)SwapDetour);
    Log(g_swapHook ? "PresentGate: installed (unfinished mid-restart frames are no longer presented)"
                   : "PresentGate: wglSwapBuffers hook FAILED - not installed");
    return 0;
}

}  // namespace presentgate

inline void DoPresentGate() {
    HANDLE t = CreateThread(nullptr, 0, presentgate::InstallThread, nullptr, 0, nullptr);
    if (t) CloseHandle(t);
    else Log("PresentGate: failed to start install thread");
}
