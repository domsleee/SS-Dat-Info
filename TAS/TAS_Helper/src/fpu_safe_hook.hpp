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
// Both are per thread and restored when a nested hook's body returns.
// SafetyHook's mid-hook stub saves XMM0-7 itself (safetyhook.hpp Context32).
inline thread_local volatile uint16_t g_hookFpuControlWord = 0;
inline thread_local volatile uint32_t g_hookMxcsr = 0;

template <typename Ctx, void (*Body)(Ctx&)>
static void FpuSafeCall(Ctx& ctx) {
    uint8_t fpu[108];
    const uint32_t csr = _mm_getcsr();
    __asm { fsave [fpu] }
    const uint16_t outerCw = g_hookFpuControlWord;
    const uint32_t outerCsr = g_hookMxcsr;
    uint16_t cw;
    std::memcpy(&cw, fpu, sizeof(cw));   // protected-mode FSAVE image starts with the control word
    g_hookFpuControlWord = cw;
    g_hookMxcsr = csr;
    Body(ctx);
    g_hookFpuControlWord = outerCw;
    g_hookMxcsr = outerCsr;
    __asm { frstor [fpu] }
    _mm_setcsr(csr);
}

// Game code called from a hook body (menu commands, Kernel::Time::Current,
// BB3B10, ...) runs at the control word and MXCSR the game had at hook entry,
// as it would without the hook, not at FSAVE's 0x037F (docs/fpu-cw-menu-channel.md).
// The x87 stack is empty, as at any call: FSAVE emptied it and compiled code
// keeps it empty across calls. The body's own control word and MXCSR come back
// at scope exit. Only inside a CreateMidHook body.
class GameFpuScope {
public:
    GameFpuScope() {
        uint16_t own, game = g_hookFpuControlWord;
        __asm {
            fnstcw own
            fldcw game
        }
        ownCw_ = own;
        ownCsr_ = _mm_getcsr();
        _mm_setcsr(g_hookMxcsr);
    }
    ~GameFpuScope() {
        uint16_t own = ownCw_;
        __asm { fldcw own }
        _mm_setcsr(ownCsr_);
    }
    GameFpuScope(const GameFpuScope&) = delete;
    GameFpuScope& operator=(const GameFpuScope&) = delete;

private:
    uint16_t ownCw_;
    uint32_t ownCsr_;
};

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
