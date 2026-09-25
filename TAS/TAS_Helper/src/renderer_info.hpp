#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "shared_state.hpp"
#include <format>

// Renderer / x87 precision. Supreme.exe sets 24-bit precision at startup and
// DirectX 6/7 keep it (CW 0x007F), but OpenGL/Software2 run the game thread at
// 53-bit (0x027F), so recordings do not replay bit-exact across them. tas_ui
// stamps both into recordings and warns on a mismatch. The cycle cave samples
// fpu_control_word; renderer_id (which srDD_*.dll sr.dll loaded) is set here.
namespace renderer {

inline uint32_t Detect() {
    // The plugin cannot change at runtime; cache it to avoid the loader lock.
    static uint32_t s_cached = TAS_RENDERER_UNKNOWN;
    if (s_cached != TAS_RENDERER_UNKNOWN) return s_cached;
    static const char* const kModules[] = {
        "srDD_DirectX6.dll",  // index + 1 = TasRendererId
        "srDD_DirectX7.dll",
        "srDD_OpenGL.dll",
        "srDD_Glide3x.dll",
        "srDD_Software2.dll",
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

// Publish renderer_id; log when it or the control word changes. Called by the
// cycle cave on each race's first tick, inside its FSAVE/FRSTOR.
inline void Refresh(TasSharedState* s) {
    static uint32_t lastRenderer = 0xFFFFFFFFu;
    static uint32_t lastCw = 0xFFFFFFFFu;
    const uint32_t id = Detect();
    s->renderer_id = id;
    const uint32_t cw = s->fpu_control_word;
    if (cw == 0) return;  // not sampled yet
    if (id == lastRenderer && cw == lastCw) return;
    lastRenderer = id;
    lastCw = cw;
    const uint32_t pc = (cw >> 8) & 3;   // precision-control field: 0 = 24-bit, 2 = 53, 3 = 64
    Log(std::format("Renderer: {} (srDD id {}), game-thread x87 control word 0x{:04X} = {}-bit precision",
                    Name(id), id, cw, pc == 0 ? 24 : pc == 2 ? 53 : pc == 3 ? 64 : 0));
}

} // namespace renderer
