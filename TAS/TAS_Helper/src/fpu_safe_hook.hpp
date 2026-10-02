#pragma once
#include <cstdint>
#include <cstring>
#include <intrin.h>
#ifndef FPU_SAFE_HOOK_NO_SAFETYHOOK  // tests/test_fpu_safe_hook.cpp builds without SafetyHook
#include <safetyhook.hpp>
#endif

// Every mid-hook body runs between FSAVE and FRSTOR, because SafetyHook saves
// the general registers but not the x87 state (DESIGN.md "Pitfalls"). Always
// install through CreateMidHook.
//
// FSAVE re-initialises the FPU (control word 0x037F, 64-bit precision), so hook
// bodies run at that precision. g_hookFpuControlWord holds the game's control
// word for the current call. SafetyHook does not save MXCSR either, so it is
// saved and restored too; g_hookMxcsr holds the game's value for the call.
// SafetyHook's mid-hook stub saves XMM0-7 itself (safetyhook.hpp Context32).
inline volatile uint16_t g_hookFpuControlWord = 0;
inline volatile uint32_t g_hookMxcsr = 0;

template <typename Ctx, void (*Body)(Ctx&)>
static void FpuSafeCall(Ctx& ctx) {
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

#ifndef FPU_SAFE_HOOK_NO_SAFETYHOOK
template <void (*Body)(SafetyHookContext&)>
static void FpuSafeHook(SafetyHookContext& ctx) {
    FpuSafeCall<SafetyHookContext, Body>(ctx);
}

template <void (*Body)(SafetyHookContext&)>
inline SafetyHookMid CreateMidHook(void* site) {
    return safetyhook::create_mid(site, FpuSafeHook<Body>);
}
#endif
