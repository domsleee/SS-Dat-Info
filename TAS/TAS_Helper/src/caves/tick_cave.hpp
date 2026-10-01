#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../gate_alignment.hpp"
#include "../game_addresses.hpp"
#include "../fpu_safe_hook.hpp"

// The tick cave: per-frame tick count and playback speed.
// Hook at Supreme.exe+25C81, after the __ftol call and `mov esi, eax`.
// ESI = ticks owed this frame (garbage for large floats); the game's next
// instruction is `cmp esi, 14h` (clamp to 20).
//
// Each frame the game owes (now - its clock) * 100 ticks and advances its
// clock by ticks * per-tick advance (0x46DB08 = EXE+0x6DB08, normally 0.01 s). Scaling
// the advance by 1/speed sets the speed (0.005 = 2x). Scaling ESI does
// nothing: the next frame recomputes it from wall time. See DESIGN.md
// "Speed control".

inline TasSharedState* g_tickCaveState = nullptr;
static SafetyHookMid tickCaveHook{};

// The advance the tick cave writes. 0x46DB08 is read by sixteen
// `fmul dword ptr [0x46db08]`: four in the game cycle, twelve in the menu
// (Menu::Paint's animation dt). The tick cave doesn't run in the menu, so a
// scaled shared value would run the menu video fast. The four in-game
// operands are pointed here instead (same-length rewrite of a 4-byte absolute
// address); the install fails if they cannot be.
// inline, not static: the patched instructions hold one fixed address.
inline float g_privateTickAdvance = 0.01f;

// RVAs of the operand in each in-game fmul (the instruction starts 2 bytes
// earlier). C1 [esp+0x40] drives tick demand and the keyboard observer's event
// gate; C2 [esp+0x60] goes to Supreme::Cycle, which ignores it. Physics steps
// a constant dt, so a catch-up drain tick is physics-neutral.
//   +0x25CE0  fmul -> i * advance, added to C1 and C2 for tick i
//   +0x25D89  fmul -> C2 += ticks_run * advance
//   +0x25DB2  fmul -> C1 += ticks_demanded * advance
//   +0x25E2B  fmul -> [ebp+0x0C] += ticks_demanded * advance (results timer)
static constexpr uint32_t TICK_ADVANCE_OPERANDS[] = {
    0x25CE2, 0x25D8B, 0x25DB4, 0x25E2D,
};

// Every patch (the clamp immediates and the four fmul operands) lies in
// EXE+0x25C83..0x26002, all on .text pages of one protection.
static constexpr uint32_t TICK_CAVE_PATCH_FIRST = 0x25C83;
static constexpr uint32_t TICK_CAVE_PATCH_END = 0x26002;

// Runs patch() with the span writable, then flushes the icache (VirtualProtect
// does not) and restores the protection.
template <typename Patch>
static bool PatchTickCaveCode(uint8_t* exeBase, Patch patch) {
    void* span = exeBase + TICK_CAVE_PATCH_FIRST;
    const size_t len = TICK_CAVE_PATCH_END - TICK_CAVE_PATCH_FIRST;
    DWORD old = 0;
    if (!VirtualProtect(span, len, PAGE_EXECUTE_READWRITE, &old)) {
        Log(std::format("Tick cave: VirtualProtect on EXE+0x{:X}..0x{:X} FAILED (err={})",
                        TICK_CAVE_PATCH_FIRST, TICK_CAVE_PATCH_END, GetLastError()));
        return false;
    }
    patch();
    FlushInstructionCache(GetCurrentProcess(), span, len);
    VirtualProtect(span, len, old, &old);
    return true;
}

// The redirected operands point into this DLL, so they must be restored
// before it could unload.
inline bool g_tickCaveRedirectApplied = false;
inline uint8_t* g_tickCaveExeBase = nullptr;
inline bool g_tickCaveClampApplied = false;

static constexpr float TICK_ADVANCE_DEFAULT = 0.01f;

// The game's own advance, read at install. 1x and OFF restore this.
static float g_nativeTickAdvance = TICK_ADVANCE_DEFAULT;

// Per-frame tick ceiling after raising the game's clamp from 20. Above 64 the
// per-frame overhead dominates.
static constexpr int32_t TICK_CAVE_PER_FRAME_CAP = 64;

// The game's own clamp (cmp esi, 14h), still enforced at 1x and below so
// normal play behaves like the unpatched game.
static constexpr int32_t NATIVE_GAME_CLAMP_AT_1X = 20;

// Set by the cycle cave on a CONT splice: the catch-up leaves the game clock
// behind wall time, and the next frame drains that backlog whatever its size
// or the resume speed, so the new take starts at its own pace. Game thread.
inline bool g_spliceDrainPending = false;

// Gate predicted from the rider's countdown at the start of an aligned
// replay (cycle cave); 0 until then. It indexes the replay from its first
// tick, so no hold window or pending splice is needed.
inline uint32_t g_predictedGate = 0;

inline uint32_t LiveGate(const TasSharedState* s) {
    return g_predictedGate ? g_predictedGate : s->gate_index;
}

// Was the engine frozen (dialog, menu, load) since the last call? The backlog
// after a freeze is dropped at any speed. CONT catch-up is not a freeze.
static bool ResumedFromFreeze() {
    static uint32_t s_lastRunMs = 0;
    uint32_t nowMs = GetTickCount();
    bool resumed_from_freeze =
        s_lastRunMs != 0 && (nowMs - s_lastRunMs) > 250;
    s_lastRunMs = nowMs;
    return resumed_from_freeze;
}

// Aligned-CONT splice interlock: once playback reaches the splice, emit zero
// ticks until the controller's watcher sets cont_splice_approved. The splice
// is destructive and the watcher can lag. STOP or RESTART lifts the park.
static bool IsSpliceParked(TasSharedState* s) {
    bool splice_parked = false;
    if (s->continue_from_frame > 0 && s->mode == MODE_PLAY
        && s->gate_align_rec != 0 && s->cont_splice_approved == 0) {
        uint32_t park_at = GateAlignedSplicePos(
            s->continue_from_frame, LiveGate(s), s->gate_align_rec);
        splice_parked = s->playback_pos >= park_at;
    }
    return splice_parked;
}

// End the catch-up batch exactly on the splice so no catch-up tick spills
// into REC.
static int32_t LimitTicksToSplice(TasSharedState* s, int32_t realTick) {
    uint32_t aligned_splice = GateAlignedSplicePos(
        s->continue_from_frame, LiveGate(s), s->gate_align_rec);
    if (s->continue_from_frame > 0 && s->mode == MODE_PLAY) {
        int32_t remaining = (int32_t)ContinueSpliceTickLimit(
            s->playback_pos, aligned_splice,
            s->gate_align_rec == 0 || s->cont_splice_approved != 0,
            s->gate_align_rec);
        if (realTick > remaining) realTick = remaining;
    }
    return realTick;
}

static void ChooseTickCount(SafetyHookContext& ctx, TasSharedState* s, int32_t owed,
                            int32_t realTick, bool catchup_drain, bool splice_parked) {
    if (catchup_drain) {
        // One tick drains the whole wall-time gap, even when the splice
        // limited this frame's ticks.
        g_privateTickAdvance = (float)owed * g_nativeTickAdvance;
        ctx.esi = 1;
    } else {
        // Clamp __ftol garbage to the patched cap.
        if (realTick < 0) realTick = 0;
        if (realTick > TICK_CAVE_PER_FRAME_CAP) realTick = TICK_CAVE_PER_FRAME_CAP;

        // OFF keeps the native clamp whatever speed was left in shared memory.
        if ((s->mode == MODE_OFF || s->playback_speed <= 1.0f) && realTick > NATIVE_GAME_CLAMP_AT_1X) {
            realTick = NATIVE_GAME_CLAMP_AT_1X;
        }

        if (splice_parked) realTick = 0;

        ctx.esi = (uintptr_t)realTick;
    }
}

// Scale the advance during REC/PLAY at a non-1x speed, otherwise restore the
// native value. Skipped on a catch-up drain frame, which set its own value.
static void ApplyPlaybackSpeed(TasSharedState* s, bool catchup_drain) {
    if (!catchup_drain) {
        if (s->mode != MODE_OFF && s->playback_speed > 0.0f && s->playback_speed != 1.0f) {
            g_privateTickAdvance = g_nativeTickAdvance / s->playback_speed;
        } else {
            g_privateTickAdvance = g_nativeTickAdvance;
        }
    }
}

// Publish the ticks simulated (not rendered frames). Counts what the game
// runs: its loop is bounded by ebx, capped at TICK_CAVE_PER_FRAME_CAP.
static void PublishTickCount(SafetyHookContext& ctx) {
    if (auto* sp = g_tickCaveState) {
        uint32_t emitted = (uint32_t)ctx.esi;
        if (emitted > (uint32_t)TICK_CAVE_PER_FRAME_CAP) {
            emitted = (uint32_t)TICK_CAVE_PER_FRAME_CAP;
        }
        sp->tick_count += emitted;
    }
}

static void TickCave_Callback(SafetyHookContext& ctx) {
    auto* s = g_tickCaveState;
    if (s) {
        int32_t realTick = (int32_t)ctx.esi;

        // After a pause, __ftol hands over the whole gap as ticks (20 s =
        // 2000), which the native clamp spreads into a visible speedup.
        // Drain it in one tick instead: at 1x in any mode, or at any speed
        // after an engine freeze or a CONT splice.
        const int32_t CATCHUP_THRESHOLD = 50;

        bool resumed_from_freeze = ResumedFromFreeze();

        bool splice_parked = IsSpliceParked(s);

        bool catchup_drain =
            !splice_parked
            && ((realTick > CATCHUP_THRESHOLD
                    && (s->playback_speed == 1.0f || resumed_from_freeze))
                || (g_spliceDrainPending && realTick > 1));
        g_spliceDrainPending = false;

        const int32_t owed = realTick;
        realTick = LimitTicksToSplice(s, realTick);

        ChooseTickCount(ctx, s, owed, realTick, catchup_drain, splice_parked);

        ApplyPlaybackSpeed(s, catchup_drain);
    }

    PublishTickCount(ctx);
}

inline void UninstallTickCave();

bool InstallTickCave(GameAddresses& addr, TasSharedState* state) {
    if (!addr.tick_cave_site) {
        Log("Tick cave: hook site not resolved");
        return false;
    }

    auto* exeBase = (uint8_t*)addr.exe;
    g_nativeTickAdvance = *(float*)(exeBase + 0x6DB08);
    g_privateTickAdvance = g_nativeTickAdvance;
    Log(std::format("Tick cave: native tick advance constant = {}", g_nativeTickAdvance));

    // Redirect the four in-game readers, all or none, else install fails.
    // kFmulTickAdvance = fmul dword ptr [0x46DB08].
    static const uint8_t kFmulTickAdvance[6] = { 0xD8, 0x0D, 0x08, 0xDB, 0x46, 0x00 };
    for (uint32_t rva : TICK_ADVANCE_OPERANDS) {
        if (memcmp(exeBase + rva - 2, kFmulTickAdvance, sizeof(kFmulTickAdvance)) != 0) {
            Log(std::format("Tick cave: fmul at EXE+0x{:X} is not the expected instruction", rva - 2));
            return false;
        }
    }

    // Raise the game's per-frame tick clamp from 14h (20) to 40h (64) so fast
    // speeds aren't capped by it:
    //   EXE+0x25C83: immediate of `cmp esi, 14h`
    //   EXE+0x26001: immediate of `mov ebx, 14h`
    // Before the mid-hook, so SafetyHook's trampoline copies the new cmp.
    uint8_t* cmp_imm = exeBase + 0x25C83;
    uint8_t* mov_imm = exeBase + 0x26001;
    const bool clampOk = *cmp_imm == 0x14 && *mov_imm == 0x14;
    if (!clampOk) {
        Log(std::format("Tick cave: tick clamp bytes unexpected (cmp_imm=0x{:02X} mov_imm=0x{:02X}); not patching",
                        *cmp_imm, *mov_imm));
    }

    // The game thread may be running these instructions. The operands are
    // unaligned, so use a locked exchange, which is atomic at any alignment
    // (none crosses a cache line). Old and new addresses hold the same value.
    g_tickCaveExeBase = exeBase;
    if (!PatchTickCaveCode(exeBase, [&] {
            const LONG target = (LONG)(uintptr_t)&g_privateTickAdvance;
            for (uint32_t rva : TICK_ADVANCE_OPERANDS) {
                InterlockedExchange((volatile LONG*)(exeBase + rva), target);
            }
            g_tickCaveRedirectApplied = true;
            if (clampOk) {
                *cmp_imm = (uint8_t)TICK_CAVE_PER_FRAME_CAP;
                *mov_imm = (uint8_t)TICK_CAVE_PER_FRAME_CAP;
                g_tickCaveClampApplied = true;
            }
        })) {
        return false;
    }
    Log(std::format("Tick cave: {} in-game tick-advance readers redirected to DLL float at {:p}{}",
                    (int)(sizeof(TICK_ADVANCE_OPERANDS) / sizeof(uint32_t)), (void*)&g_privateTickAdvance,
                    clampOk ? std::format("; tick clamp 0x14 -> 0x{:02X}", TICK_CAVE_PER_FRAME_CAP) : ""));

    g_tickCaveState = state;

    Log(std::format("Tick cave: hooking tick override at {:p} (EXE+0x25C81)", (void*)addr.tick_cave_site));

    tickCaveHook = CreateMidHook<TickCave_Callback>(addr.tick_cave_site);

    if (!tickCaveHook) {
        Log("Tick cave: SafetyHook create_mid FAILED");
        UninstallTickCave();
        return false;
    }

    state->tick_cave_hooked = 1;
    Log("Tick cave: hook installed successfully");
    return true;
}

// Undo every patch. Used to roll back a failed initialization; a successful
// one pins the DLL.
inline void UninstallTickCave() {
    tickCaveHook = {};
    if (!g_tickCaveExeBase) return;

    uint8_t* exeBase = g_tickCaveExeBase;
    PatchTickCaveCode(exeBase, [&] {
        if (g_tickCaveRedirectApplied) {
            const LONG original = (LONG)(uintptr_t)(exeBase + 0x6DB08);
            for (uint32_t rva : TICK_ADVANCE_OPERANDS) {
                InterlockedExchange((volatile LONG*)(exeBase + rva), original);
            }
            g_tickCaveRedirectApplied = false;
        }
        if (g_tickCaveClampApplied) {
            exeBase[0x25C83] = 0x14;
            exeBase[0x26001] = 0x14;
            g_tickCaveClampApplied = false;
        }
    });

    if (g_tickCaveState) g_tickCaveState->tick_cave_hooked = 0;
    g_tickCaveState = nullptr;
    Log("Tick cave: hook and all supporting patches removed");
}
