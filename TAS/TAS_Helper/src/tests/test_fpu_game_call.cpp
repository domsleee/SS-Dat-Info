// fpu_safe_hook.hpp GameFpuScope: game code a hook body calls sees the control
// word and MXCSR the game had at hook entry (not FSAVE's 0x037F), with an
// empty x87 stack, and the body gets its own state back afterwards. A nested
// hook entered at another control word must not change what the outer body's
// next call sees.
#define FPU_SAFE_HOOK_NO_SAFETYHOOK
#include "../fpu_safe_hook.hpp"
#include "check.hpp"

struct Ctx {};

struct Seen { uint16_t cw, tags; uint32_t csr; };
static Seen g_probe[2];
static int g_probeCalls = 0;
static uint16_t g_bodyCwBefore, g_bodyCwAfter;
static uint32_t g_bodyCsrBefore, g_bodyCsrAfter;
static uint16_t g_nestedSeenCw;

static void Nested(Ctx&) { g_nestedSeenCw = g_hookFpuControlWord; }

// Stands in for game code: records its environment, then, like the game's own
// fldcw save/modify/restore around a PC24 section, enters a nested hook at
// another control word.
static void __declspec(noinline) Probe() {
    uint16_t cw, pc24 = 0x007F;
    uint8_t env[28];
    const uint32_t csr = _mm_getcsr();
    __asm {
        fnstcw cw
        fnstenv env
        fldcw cw            ; fnstenv masked every exception; put the control word back
    }
    Seen& s = g_probe[g_probeCalls++ & 1];
    s.cw = cw;
    s.csr = csr;
    std::memcpy(&s.tags, env + 8, 2);
    Ctx inner{};
    __asm fldcw pc24
    FpuSafeCall<Ctx, Nested>(inner);
    __asm fldcw cw
}

static uint16_t Cw() {
    uint16_t cw;
    __asm fnstcw cw
    return cw;
}

static void Body(Ctx&) {
    g_bodyCwBefore = Cw();
    g_bodyCsrBefore = _mm_getcsr();
    {
        GameFpuScope fpu;
        Probe();
    }
    {
        GameFpuScope fpu;
        Probe();
    }
    g_bodyCwAfter = Cw();
    g_bodyCsrAfter = _mm_getcsr();
}

// The game's environment at hook entry: `cw`, `csr`, with values on the stack.
static void Enter(uint16_t cw, uint32_t csr) {
    static const double kVals[2] = {1.5, -2.25};
    Ctx ctx{};
    void* pctx = &ctx;
    void (*fn)(Ctx&) = &FpuSafeCall<Ctx, Body>;
    __asm {
        fninit
        fldcw cw
        fld qword ptr [kVals]
        fld qword ptr [kVals + 8]
        ldmxcsr csr
        push pctx
        call fn
        add esp, 4
        fninit
    }
    _mm_setcsr(0x1F80);
}

static void Case(const char* name, uint16_t cw, uint32_t csr) {
    std::printf(" %s (CW %#06x, MXCSR %#06x):\n", name, cw, csr);
    g_probeCalls = 0;
    Enter(cw, csr);
    check(g_probeCalls == 2, "probe_ran_twice");
    for (int i = 0; i < 2; i++) {
        check(g_probe[i].cw == cw, i ? "second_call_sees_the_entry_cw_after_a_nested_hook"
                                     : "game_call_sees_the_entry_cw");
        check(g_probe[i].csr == csr, i ? "second_call_sees_the_entry_mxcsr" : "game_call_sees_the_entry_mxcsr");
        check(g_probe[i].tags == 0xFFFF, "game_call_sees_an_empty_x87_stack");
    }
    check(g_nestedSeenCw == 0x007F, "nested_hook_sees_its_own_entry_cw");
    check(g_bodyCwBefore == 0x037F, "body_runs_at_fsave_default");
    check(g_bodyCwAfter == g_bodyCwBefore, "body_cw_restored");
    check(g_bodyCsrAfter == g_bodyCsrBefore, "body_mxcsr_restored");
    check(g_hookFpuControlWord == 0 && g_hookMxcsr == 0, "globals_cleared_after_the_outermost_hook");
}

int main() {
    std::printf("fpu game-call tests:\n");
    Case("game PC53 (OpenGL)", 0x027F, 0x1F80 | 0x20);
    Case("game PC24, round down, FTZ", 0x047F, 0x1F80 | 0x2000 | 0x8000);
    return FinishTests();
}
