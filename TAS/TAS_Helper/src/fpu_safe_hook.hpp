#pragma once
#include <cstdint>
#include <cstring>
#include <intrin.h>
#include <safetyhook.hpp>

// Every mid-hook body runs between FSAVE and FRSTOR, because SafetyHook saves
// the general registers but not the x87 state (DESIGN.md "Pitfalls"). Always
// install through CreateMidHook.
//
// FSAVE re-initialises the FPU (control word 0x037F, 64-bit precision), so hook
// bodies run at that precision. g_hookFpuControlWord holds the game's control
// word for the current call. SafetyHook does not save MXCSR either, so it is
// saved and restored too; g_hookMxcsr holds the game's value for the call.
inline volatile uint16_t g_hookFpuControlWord = 0;
inline volatile uint32_t g_hookMxcsr = 0;

template <void (*Body)(SafetyHookContext&)>
static void FpuSafeHook(SafetyHookContext& ctx) {
    uint8_t fpu[108];
    const uint32_t csr = _mm_getcsr();
    __asm { fsave [fpu] }
    uint16_t cw;
    std::memcpy(&cw, fpu, sizeof(cw));   // protected-mode FSAVE image starts with the control word
    g_hookFpuControlWord = cw;
    g_hookMxcsr = csr;
    Body(ctx);
    __asm { frstor [fpu] }
    _mm_setcsr(csr);
}

template <void (*Body)(SafetyHookContext&)>
inline SafetyHookMid CreateMidHook(void* site) {
    return safetyhook::create_mid(site, FpuSafeHook<Body>);
}
