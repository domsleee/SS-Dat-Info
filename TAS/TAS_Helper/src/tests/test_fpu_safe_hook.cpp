// fpu_safe_hook.hpp: a hook body that wrecks the x87 state and MXCSR must
// leave the game's state exactly as it was: register contents, tags, status
// word (TOP and sticky flags), control word and the whole MXCSR. XMM0-7 are
// SafetyHook's job (its stub saves them) and are not checked here.
#define FPU_SAFE_HOOK_NO_SAFETYHOOK
#include "../fpu_safe_hook.hpp"
#include "check.hpp"

struct Ctx { int calls; uint16_t seen_cw; uint32_t seen_csr; };

static void Wreck(Ctx& c) {
    c.calls++;
    c.seen_cw = g_hookFpuControlWord;
    c.seen_csr = g_hookMxcsr;
    __asm {
        fninit
        fldpi
        fldz
        fdiv st(0), st(0)   ; invalid -> sticky IE, NaN
        fld1
        fldz
        fdivp st(1), st     ; zero divide -> sticky ZE
    }
    _mm_setcsr(0xFFBF);     // FTZ, DAZ, round-to-zero, all flags
}

alignas(16) static uint8_t g_before[512], g_after[512];
static const double kVals[3] = {1.5, 3.0e300, -2.25};
static uint16_t kGameCw = 0x027F;
static uint32_t kGameCsr = 0x1F80 | 0x20;   // OpenGL's environment plus a sticky PE

static bool Same(int from, int to) { return std::memcmp(g_before + from, g_after + from, to - from) == 0; }

int main() {
    std::printf("fpu_safe_hook tests:\n");
    Ctx ctx{};
    void* pctx = &ctx;
    void (*fn)(Ctx&) = &FpuSafeCall<Ctx, Wreck>;
    __asm {
        fninit
        fldcw kGameCw
        ldmxcsr kGameCsr
        fld qword ptr [kVals]
        fld qword ptr [kVals + 8]
        fld qword ptr [kVals + 16]
        fsqrt               ; sqrt(-2.25) -> sticky IE, NaN on top
        fxsave [g_before]
        push pctx
        call fn
        add esp, 4
        fxsave [g_after]
        fninit
    }
    _mm_setcsr(0x1F80);
    check(ctx.calls == 1, "body_ran");
    check(ctx.seen_cw == 0x027F, "body_sees_the_game_cw");
    check((ctx.seen_csr & 0xFFC0) == 0x1F80, "body_sees_the_game_mxcsr");
    check(Same(0, 2), "control_word_restored");
    check(Same(2, 4), "status_word_restored");   // TOP and the sticky flags
    check(Same(4, 5), "tags_restored");
    check(Same(24, 28), "mxcsr_restored_with_flags");
    bool regs = true;
    for (int i = 0; i < 8; i++) regs = regs && Same(32 + 16 * i, 32 + 16 * i + 10);
    check(regs, "x87_registers_restored");
    uint16_t fsw;
    std::memcpy(&fsw, g_before + 2, 2);
    check(((fsw >> 11) & 7) == 5 && (fsw & 0x01), "setup_had_three_values_and_a_sticky_flag");
    return FinishTests();
}
