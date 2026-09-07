#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include <format>

// Renderer / x87-precision awareness.
//
// The physics are not renderer-independent: Supreme.exe sets 24-bit x87
// precision at startup (_controlfp(_PC_24, _MCW_PC)), DirectX 6/7 leave it
// there (control word 0x007F), but the OpenGL/Software2 path runs the game
// thread at 53-bit (0x027F). Every x87 multiply in the sim then rounds
// differently, so a recording made under one renderer does not replay
// bit-exact under the other (see the wiki page "Why are replays sometimes
// 0.01s shorter than expected?").
//
// The DLL publishes both facts so tas_ui can show them, stamp them into
// recordings and history entries, and warn on a mismatch:
//   - fpu_control_word: sampled on the game thread by cave2 every cycle.
//   - renderer_id: which srDD_*.dll sr.dll loaded, refreshed here.
namespace renderer {

inline uint32_t Detect() {
    // The plugin cannot change at runtime: resolve it once instead of taking
    // the loader lock five times every 100 ms from the worker.
    static uint32_t s_cached = TAS_RENDERER_UNKNOWN;
    if (s_cached != TAS_RENDERER_UNKNOWN) return s_cached;
    static const char* const kModules[] = {
        "srDD_DirectX6.dll",  // TAS_RENDERER_DIRECTX6
        "srDD_DirectX7.dll",  // TAS_RENDERER_DIRECTX7
        "srDD_OpenGL.dll",    // TAS_RENDERER_OPENGL
        "srDD_Glide3x.dll",   // TAS_RENDERER_GLIDE3X
        "srDD_Software2.dll", // TAS_RENDERER_SOFTWARE2
    };
    for (uint32_t i = 0; i < 5; i++) {
        if (GetModuleHandleA(kModules[i])) {
            s_cached = i + 1;
            return s_cached;
        }
    }
    return TAS_RENDERER_UNKNOWN;
}

inline const char* Name(uint32_t id) {
    switch (id) {
        case TAS_RENDERER_DIRECTX6:  return "DirectX6";
        case TAS_RENDERER_DIRECTX7:  return "DirectX7";
        case TAS_RENDERER_OPENGL:    return "OpenGL";
        case TAS_RENDERER_GLIDE3X:   return "Glide3x";
        case TAS_RENDERER_SOFTWARE2: return "Software2";
        default:                     return "unknown";
    }
}

// Publish renderer_id; log whenever the renderer or the game thread's control
// word changes. Called from the level-scan worker (~10 Hz), never from a hook.
inline void Refresh(TasSharedState* s) {
    static uint32_t lastRenderer = 0xFFFFFFFFu;
    static uint32_t lastCw = 0xFFFFFFFFu;
    const uint32_t id = Detect();
    s->renderer_id = id;
    const uint32_t cw = s->fpu_control_word;
    if (cw == 0) return;  // no Supreme::Cycle yet: nothing sampled
    if (id == lastRenderer && cw == lastCw) return;
    lastRenderer = id;
    lastCw = cw;
    const uint32_t pc = (cw >> 8) & 3;   // precision-control field: 0 = 24-bit, 2 = 53, 3 = 64
    Log(std::format("Renderer: {} (srDD id {}), game-thread x87 control word 0x{:04X} = {}-bit precision",
                    Name(id), id, cw, pc == 0 ? 24 : pc == 2 ? 53 : pc == 3 ? 64 : 0));
}

} // namespace renderer
