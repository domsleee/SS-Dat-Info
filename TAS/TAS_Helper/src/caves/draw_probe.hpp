#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../external/safetyhook.hpp"

// ---------------------------------------------------------------------------
// DIAGNOSTIC ONLY (env-gated TAS_DRAWPROBE=1) — glDrawElements logger.
//
// The "missing far triangles at 600m detail 4" bug: srDD_OpenGL draws
// GL_UNSIGNED_INT (32-bit) yet geometry wraps at 65,536 vertices. This hook
// answers the decisive question WITHOUT patching anything: what count/type
// actually reaches glDrawElements, and does terrain submit one >65535 call
// (index-content truncation) or many sub-batches (vertex aliasing)?
//
// It publishes per-frame maxima into spare shared fields so an out-of-process
// reader (a Python script) can sample them at 450 vs 600 without a debugger.
// Pure observation: no behaviour change, safe on a live game.
// ---------------------------------------------------------------------------

namespace drawprobe {

inline SafetyHookInline g_hook{};
inline TasSharedState*  g_state = nullptr;

// GL constants (avoid pulling in gl.h).
inline constexpr unsigned GL_UNSIGNED_BYTE  = 0x1401;
inline constexpr unsigned GL_UNSIGNED_SHORT = 0x1403;
inline constexpr unsigned GL_UNSIGNED_INT   = 0x1405;

// Per-frame accumulators (reset by cave2's cycle each frame is overkill; we
// just publish running maxima and let the reader diff across a fresh restart).
inline volatile uint32_t g_maxCount = 0;   // largest `count` seen
inline volatile uint32_t g_lastType = 0;   // most recent index type
inline volatile uint32_t g_callTally = 0;  // total glDrawElements calls
inline volatile uint32_t g_over64k = 0;    // calls with count > 65535
inline volatile uint32_t g_maxIndex = 0;   // largest INDEX VALUE seen (the wrap tell)

typedef void(__stdcall* glDrawElements_t)(unsigned mode, int count, unsigned type,
                                          const void* indices);
typedef void(__stdcall* glVertexPointer_t)(int size, unsigned type, int stride,
                                           const void* ptr);

inline SafetyHookInline g_vpHook{};
// Correlate the LAST glVertexPointer base with the NEXT draw: only draws with a
// terrain-sized count (>1000) count toward the terrain pool span, filtering out
// HUD/boarder meshes that also call glVertexPointer.
inline volatile uintptr_t g_lastVpBase = 0;
inline volatile uint32_t  g_vpStride = 0;
inline volatile uintptr_t g_terMin = (uintptr_t)-1;
inline volatile uintptr_t g_terMax = 0;
inline volatile uint32_t  g_vpSpanVerts = 0;

inline void __stdcall VpDetour(int size, unsigned type, int stride, const void* ptr) {
    uintptr_t p = (uintptr_t)ptr;
    if (p > 0x10000) {
        g_lastVpBase = p;
        if (stride > 0) g_vpStride = (uint32_t)stride;
    }
    g_vpHook.stdcall<void, int, unsigned, int, const void*>(size, type, stride, ptr);
}

inline void __stdcall Detour(unsigned mode, int count, unsigned type,
                             const void* indices) {
    if (count > 0) {
        uint32_t c = (uint32_t)count;
        if (c > g_maxCount) g_maxCount = c;
        g_lastType = type;
        g_callTally++;
        if (c > 65535u) g_over64k++;
        // Max INDEX VALUE: if a small batch's indices point past 65535 into a
        // large shared vertex pool, THAT is where a u16 slot would wrap.
        // Old GL: `indices` is a client pointer (no VBO in srDD_OpenGL). Guard
        // the deref and only sample first + last (O(1)) — the terrain strips
        // are ~monotonic, so those bracket the range.
        // Terrain-sized batch: fold its vertex-array base into the terrain pool
        // span. The base+localMaxIndex reaches the highest pool slot touched;
        // if that exceeds 65535 at 600 but not 450, the pool overflowed.
        if (c > 1000u && g_lastVpBase && g_vpStride) {
            uintptr_t b = g_lastVpBase;
            if (b < g_terMin) g_terMin = b;
            if (b > g_terMax) g_terMax = b;
            g_vpSpanVerts = (uint32_t)((g_terMax - g_terMin) / (uintptr_t)g_vpStride) + c;
        }
        uintptr_t p = (uintptr_t)indices;
        if (p > 0x10000 && type == GL_UNSIGNED_INT) {
            const uint32_t* idx = (const uint32_t*)indices;
            uint32_t a = idx[0];
            uint32_t b = idx[c - 1];
            uint32_t m = a > b ? a : b;
            if (m > g_maxIndex && m < 0x0FFFFFFFu) g_maxIndex = m;
        } else if (p > 0x10000 && type == GL_UNSIGNED_SHORT) {
            const uint16_t* idx = (const uint16_t*)indices;
            uint32_t a = idx[0];
            uint32_t b = idx[c - 1];
            uint32_t m = a > b ? a : b;
            if (m > g_maxIndex) g_maxIndex = m;
        }
        if (g_state) {
            // Publish into objsnap_arm_player[] — a research buffer that is idle
            // except during snapshot experiments, so nothing clobbers us (the
            // diag_* fields are rewritten by cave5 every frame).
            g_state->objsnap_arm_player[0] = 0xD4A3B0BEu;  // locator sentinel
            g_state->objsnap_arm_player[1] = g_maxCount;
            g_state->objsnap_arm_player[2] = g_lastType;
            g_state->objsnap_arm_player[3] = g_callTally;
            g_state->objsnap_arm_player[4] = g_over64k;
            g_state->objsnap_arm_player[5] = g_maxIndex;
            g_state->objsnap_arm_player[6] = g_vpSpanVerts;  // vertices spanned by the pool
            g_state->objsnap_arm_player[7] = g_vpStride;
        }
    }
    g_hook.stdcall<void, unsigned, int, unsigned, const void*>(mode, count, type, indices);
}

inline DWORD WINAPI InstallThread(LPVOID param) {
    g_state = (TasSharedState*)param;
    // opengl32 loads well after DLL init (the game brings the renderer up
    // lazily). Poll for it, then hook glDrawElements.
    for (int i = 0; i < 600; i++) {  // up to ~60s
        HMODULE gl = GetModuleHandleA("opengl32.dll");
        if (gl) {
            auto* fn = GetProcAddress(gl, "glDrawElements");
            if (fn) {
                g_hook = safetyhook::create_inline((void*)fn, (void*)Detour);
                if (g_hook) {
                    Log(std::format("DrawProbe: opengl32!glDrawElements hooked at {:p}", (void*)fn));
                } else {
                    Log("DrawProbe: SafetyHook on glDrawElements FAILED");
                }
                if (auto* vp = GetProcAddress(gl, "glVertexPointer")) {
                    g_vpHook = safetyhook::create_inline((void*)vp, (void*)VpDetour);
                    Log(g_vpHook ? "DrawProbe: glVertexPointer hooked"
                                 : "DrawProbe: glVertexPointer hook FAILED");
                }
                return 0;
            }
        }
        Sleep(100);
    }
    Log("DrawProbe: opengl32.dll never appeared");
    return 0;
}

inline bool Install(TasSharedState* state) {
    CreateThread(nullptr, 0, InstallThread, state, 0, nullptr);
    return true;
}

}  // namespace drawprobe
