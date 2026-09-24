#pragma once
#include <cstdint>
#include <cstring>
#include <safetyhook.hpp>

// Every mid-hook body runs between FSAVE and FRSTOR.
//
// SafetyHook mid-hooks save the general registers but not the x87 state, and a
// mid-hook can sit in the middle of the game's physics with live values on the
// FPU stack. A float, a formatted log line or a CRT conversion inside the hook
// would silently change them, and replays would stop matching with nothing
// crashing. So no hook installs its callback directly: CreateMidHook wraps it.
//
// FSAVE re-initialises the FPU (control word 0x037F, 64-bit precision), so code
// inside a hook runs at that precision; FRSTOR puts the game's own state back.
// The game's control word at the hook is kept in g_hookFpuControlWord for the
// one reader that needs it (the cycle cave publishes the physics precision from inside
// its body, so the value is always the current call's).
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
