// F5 restart debounce: make a held F5 perform ONE restart instead of a
// restart every engine tick.
//
// The game's restart is LEVEL-triggered: the key handler in
// HMG_Cetsup_Win32.dll (+0x3940 down / +0x3980 up, thiscall(this, VK,
// timeLo, timeHi)) writes a pressed/released byte into the input buffer and
// the restart logic re-fires every tick while the F5 byte reads pressed.
// Holding F5 therefore keeps the engine inside its multi-frame restart
// forever, which on 600m/detail-4 is visible as serrated half-built terrain.
//
// Hook the down handler; when a physical F5 press arrives, pass it through
// (the restart fires once) and 150ms later deliver the matching keyUp
// through the real handler (which performs the correct buffer write). The
// delay guarantees the game's poll sees the press; the release then lets the
// restart's scene rebuild finish cleanly. Pressing F5 again restarts again.
//
// TAS safety: TAS gameplay/restart injection writes the buffer and calls the
// BB3B10 observer directly (cave2), bypassing these handlers entirely.
#pragma once

#include <windows.h>
#include <cstdint>
#include <format>

#include "log.hpp"
#include "external/safetyhook.hpp"

namespace f5debounce {

typedef void(__fastcall* Handler_t)(void* ecx, void* edx, uint32_t vk, uint32_t tlo, uint32_t thi);

inline SafetyHookInline g_downHook{};
inline SafetyHookInline g_upHook{};
inline SafetyHookInline g_swapHook{};
inline uint8_t* g_upHandler = nullptr;   // real +0x3980 entry

// After an accepted restart the scene rebuild takes ~400ms and renders
// half-built terrain. Because the accept moment is known exactly, presents
// are skipped for a fixed window after it - the screen holds the last frame,
// then cuts to the completed scene. Purely time-bounded: no heuristics, ends
// unconditionally, cannot freeze anything.
constexpr DWORD REBUILD_HIDE_MS = 450;
inline volatile DWORD g_hidePresentsUntil = 0;

inline int __stdcall SwapDetour(void* hdc) {
    DWORD until = g_hidePresentsUntil;
    if (until && (GetTickCount() - until) & 0x80000000) {   // now < until
        return 1;   // report success without presenting the rebuild frame
    }
    return g_swapHook.stdcall<int, void*>(hdc);
}

inline volatile LONG g_pendingRelease = 0;
inline void* g_this = nullptr;
inline uint32_t g_tlo = 0, g_thi = 0;
inline DWORD g_downTick = 0;
inline DWORD g_lastAccepted = 0;

// The restart's scene rebuild takes ~0.4s; accepting presses faster than that
// (spamming taps) re-enters the restart mid-rebuild and shows half-built
// terrain again. Swallow any F5 press within this window of the last one.
constexpr DWORD MIN_RESTART_SPACING_MS = 700;

inline void __fastcall DownDetour(void* ecx, void* edx, uint32_t vk, uint32_t tlo, uint32_t thi) {
    if (vk == VK_F5) {
        DWORD now = GetTickCount();
        if (now - g_lastAccepted < MIN_RESTART_SPACING_MS)
            return;   // swallow: previous restart is still rebuilding its scene
        g_lastAccepted = now;
        g_hidePresentsUntil = now + REBUILD_HIDE_MS;
        g_this = ecx;
        g_tlo = tlo;
        g_thi = thi;
        g_downTick = now;
        InterlockedExchange(&g_pendingRelease, 1);
    }
    g_downHook.thiscall<void>(ecx, vk, tlo, thi);
}

inline void __fastcall UpDetour(void* ecx, void* edx, uint32_t vk, uint32_t tlo, uint32_t thi) {
    if (vk == VK_F5) InterlockedExchange(&g_pendingRelease, 0);
    g_upHook.thiscall<void>(ecx, vk, tlo, thi);
}

inline DWORD WINAPI ReleaseThread(LPVOID) {
    for (;;) {
        Sleep(50);
        if (g_pendingRelease && g_this && (GetTickCount() - g_downTick) >= 150) {
            if (InterlockedExchange(&g_pendingRelease, 0)) {
                // Deliver the release through the hook chain so the handler's
                // own buffer write runs (the stamp may be discarded by the
                // observer after the restart epoch change - harmless, the
                // buffer write is what clears the level-trigger).
                UpDetour(g_this, nullptr, VK_F5, g_tlo, g_thi);
            }
        }
    }
    return 0;
}

inline DWORD WINAPI InstallThread(LPVOID) {
    HMODULE hmg = nullptr;
    for (int i = 0; i < 600 && !hmg; i++) {
        hmg = GetModuleHandleA("HMG_Cetsup_Win32.dll");
        if (!hmg) Sleep(100);
    }
    if (!hmg) {
        Log("F5Debounce: HMG_Cetsup_Win32.dll never loaded - not installed");
        return 0;
    }
    // Present hook for the post-restart rebuild hide (opengl32 loads lazily).
    HMODULE gl = nullptr;
    for (int i = 0; i < 600 && !gl; i++) {
        gl = GetModuleHandleA("opengl32.dll");
        if (!gl) Sleep(100);
    }
    if (gl) {
        if (auto* swap = GetProcAddress(gl, "wglSwapBuffers")) {
            g_swapHook = safetyhook::create_inline((void*)swap, (void*)SwapDetour);
            Log(g_swapHook ? "F5Debounce: rebuild-hide present hook installed"
                           : "F5Debounce: wglSwapBuffers hook FAILED (rebuild stays visible)");
        }
    }
    uint8_t* down = (uint8_t*)hmg + 0x3940;
    g_upHandler = (uint8_t*)hmg + 0x3980;
    g_downHook = safetyhook::create_inline((void*)down, (void*)DownDetour);
    g_upHook = safetyhook::create_inline((void*)g_upHandler, (void*)UpDetour);
    if (g_downHook && g_upHook) {
        HANDLE t = CreateThread(nullptr, 0, ReleaseThread, nullptr, 0, nullptr);
        if (t) CloseHandle(t);
        Log("F5Debounce: installed (held F5 = one restart)");
    } else {
        Log("F5Debounce: handler hooks FAILED - not installed");
    }
    return 0;
}

}  // namespace f5debounce

inline void DoF5Debounce() {
    HANDLE t = CreateThread(nullptr, 0, f5debounce::InstallThread, nullptr, 0, nullptr);
    if (t) CloseHandle(t);
    else Log("F5Debounce: failed to start install thread");
}
