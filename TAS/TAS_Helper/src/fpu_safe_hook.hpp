#pragma once
#include <cstdint>
#include <cstring>
#include <safetyhook.hpp>

// Every mid-hook body runs between FSAVE and FRSTOR, because SafetyHook saves
// the general registers but not the x87 state (DESIGN.md "Pitfalls"). Always
// install through CreateMidHook.
//
// FSAVE re-initialises the FPU (control word 0x037F, 64-bit precision), so hook
// bodies run at that precision. g_hookFpuControlWord holds the game's control
// word for the current call.
inline volatile uint16_t g_hookFpuControlWord = 0;

template <void (*Body)(SafetyHookContext&)>
static void FpuSafeHook(SafetyHookContext& ctx) {
    uint8_t fpu[108];
    __asm { fsave [fpu] }
    uint16_t cw;
    std::memcpy(&cw, fpu, sizeof(cw));   // protected-mode FSAVE image starts with the control word
    g_hookFpuControlWord = cw;
    Body(ctx);
    __asm { frstor [fpu] }
}

template <void (*Body)(SafetyHookContext&)>
inline SafetyHookMid CreateMidHook(void* site) {
    return safetyhook::create_mid(site, FpuSafeHook<Body>);
}
