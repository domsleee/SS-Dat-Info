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
// clock by ticks * per-tick advance (EXE+0x46DB08, normally 0.01 s). Scaling
// the advance by 1/speed sets the speed (0.005 = 2x). Scaling ESI does
// nothing: the next frame recomputes it from wall time. See DESIGN.md
// "Speed control".

inline TasSharedState* g_tickCaveState = nullptr;
static SafetyHookMid tickCaveHook{};

// The advance the tick cave writes: g_privateTickAdvance, or the game's
// constant in the fallback.
static float* g_tickAdvancePtr = nullptr;

// EXE+0x46DB08 is read by sixteen `fmul dword ptr [0x46db08]`: four in the
// game cycle, twelve in the menu (Menu::Paint's animation dt). The tick cave
// doesn't run in the menu, so a scaled shared value would run the menu video
// fast. The four in-game operands are pointed here instead (same-length
// rewrite of a 4-byte absolute address).
// inline, not static: the patched instructions hold one fixed address.
inline float g_privateTickAdvance = 0.01f;

// RVAs of the operand in each in-game fmul (the instruction starts 2 bytes
// earlier):
//   +0x25CE0  fmul -> per-tick timestamp offset   (i * advance)
//   +0x25D89  fmul -> Time::Add(clock,  ticks_run * advance)
//   +0x25DB2  fmul -> Time::Add(clock2, ticks_demanded * advance)
//   +0x25E2B  fmul -> game-object time [ebp+0x0C] += ticks_demanded * advance
//                     (not the clock tick demand is computed from)
static constexpr uint32_t TICK_ADVANCE_OPERANDS[] = {
    0x25CE2, 0x25D8B, 0x25DB4, 0x25E2D,
};

// Original protection of every .text page the tick cave patches, recorded on
// first sight. Shared by all patches because they overlap: three fmul operands
// and the clamp byte are on page 0x425000, and a second VirtualProtect there
// would report our own RWX as the "old" protection.
struct TickCaveCodePages {
    static constexpr size_t kMax = 8;
    uintptr_t base[kMax] = {};
    DWORD original[kMax] = {};
    size_t count = 0;
    size_t pageSize = 0;

    size_t PageSize() {
        if (!pageSize) {
            SYSTEM_INFO si{};
            GetSystemInfo(&si);
            pageSize = si.dwPageSize ? si.dwPageSize : 0x1000;
        }
        return pageSize;
    }

    // Make [addr, addr+len) RWX.
    bool Open(void* addr, size_t len) {
        const size_t ps = PageSize();
        uintptr_t first = (uintptr_t)addr & ~(uintptr_t)(ps - 1);
        uintptr_t last = ((uintptr_t)addr + len - 1) & ~(uintptr_t)(ps - 1);
        for (uintptr_t pg = first; pg <= last; pg += ps) {
            bool seen = false;
            for (size_t i = 0; i < count; ++i) {
                if (base[i] == pg) { seen = true; break; }
            }
            if (!seen && count >= kMax) return false;
            DWORD old = 0;
            if (!VirtualProtect((void*)pg, ps, PAGE_EXECUTE_READWRITE, &old)) return false;
            if (!seen) {
                base[count] = pg;
                original[count] = old;
                ++count;
            }
        }
        return true;
    }

    // Flush the icache (VirtualProtect does not) and put every page back.
    void CloseAll() {
        const size_t ps = PageSize();
        for (size_t i = 0; i < count; ++i) {
            FlushInstructionCache(GetCurrentProcess(), (void*)base[i], ps);
            DWORD ignored = 0;
            VirtualProtect((void*)base[i], ps, original[i], &ignored);
        }
        count = 0;
    }
};

inline TickCaveCodePages g_tickCaveCodePages{};

// The redirected operands point into this DLL, so they must be restored
// before it could unload.
inline bool g_tickCaveRedirectApplied = false;
inline uint8_t* g_tickCaveExeBase = nullptr;
inline bool g_tickCaveClampApplied = false;
inline bool g_tickAdvanceProtectionChanged = false;
inline DWORD g_tickAdvanceOriginalProtect = 0;

static constexpr float TICK_ADVANCE_DEFAULT = 0.01f;

// The game's own advance, read at install. 1x and OFF restore this.
static float g_nativeTickAdvance = TICK_ADVANCE_DEFAULT;

// Per-frame tick ceiling after raising the game's clamp from 20. Above 64 the
// per-frame overhead dominates.
static constexpr int32_t TICK_CAVE_PER_FRAME_CAP = 64;

// The game's own clamp (cmp esi, 14h), still enforced at 1x and below so
// normal play behaves like the unpatched game.
static constexpr int32_t NATIVE_GAME_CLAMP_AT_1X = 20;

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
            s->continue_from_frame, s->gate_index, s->gate_align_rec);
        splice_parked = s->playback_pos >= park_at;
    }
    return splice_parked;
}

// End the catch-up batch exactly on the splice so no catch-up tick spills
// into REC.
static int32_t LimitTicksToSplice(TasSharedState* s, int32_t realTick) {
    uint32_t aligned_splice = GateAlignedSplicePos(
        s->continue_from_frame, s->gate_index, s->gate_align_rec);
    if (s->continue_from_frame > 0 && s->mode == MODE_PLAY) {
        int32_t remaining = (int32_t)ContinueSpliceTickLimit(
            s->playback_pos, aligned_splice,
            s->gate_align_rec == 0 || s->cont_splice_approved != 0,
            s->gate_align_rec);
        if (realTick > remaining) realTick = remaining;
    }
    return realTick;
}

static void ChooseTickCount(SafetyHookContext& ctx, TasSharedState* s, int32_t realTick,
                            bool catchup_drain, bool splice_parked) {
    if (catchup_drain) {
        // One tick drains the whole wall-time gap.
        if (g_tickAdvancePtr) {
            *g_tickAdvancePtr = (float)realTick * g_nativeTickAdvance;
        }
        ctx.esi = 1;
    } else {
        // Clamp __ftol garbage to the patched cap.
        if (realTick < 0) realTick = 0;
        if (realTick > TICK_CAVE_PER_FRAME_CAP) realTick = TICK_CAVE_PER_FRAME_CAP;

        if (s->playback_speed <= 1.0f && realTick > NATIVE_GAME_CLAMP_AT_1X) {
            realTick = NATIVE_GAME_CLAMP_AT_1X;
        }

        if (splice_parked) realTick = 0;

        ctx.esi = (uintptr_t)realTick;
    }
}

// Scale the advance during REC/PLAY at a non-1x speed, otherwise restore the
// native value. Skipped on a catch-up drain frame, which set its own value.
static void ApplyPlaybackSpeed(TasSharedState* s, bool catchup_drain) {
    if (g_tickAdvancePtr && !catchup_drain) {
        if (s->mode != MODE_OFF && s->playback_speed > 0.0f && s->playback_speed != 1.0f) {
            *g_tickAdvancePtr = g_nativeTickAdvance / s->playback_speed;
        } else {
            *g_tickAdvancePtr = g_nativeTickAdvance;
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
        // after an engine freeze.
        const int32_t CATCHUP_THRESHOLD = 50;

        bool resumed_from_freeze = ResumedFromFreeze();

        bool splice_parked = IsSpliceParked(s);

        bool catchup_drain =
            realTick > CATCHUP_THRESHOLD
            && !splice_parked
            && (s->playback_speed == 1.0f || resumed_from_freeze);

        realTick = LimitTicksToSplice(s, realTick);

        ChooseTickCount(ctx, s, realTick, catchup_drain, splice_parked);

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

    g_tickCaveState = state;

    auto* exeBase = (uint8_t*)addr.exe;
    g_tickCaveExeBase = exeBase;
    g_tickAdvancePtr = (float*)(exeBase + 0x6DB08);

    g_nativeTickAdvance = *g_tickAdvancePtr;
    Log(std::format("Tick cave: native tick advance constant = {}", g_nativeTickAdvance));

    // Redirect the four in-game readers, all or none: after a partial
    // redirect the fallback would leave some sites reading a float nothing
    // updates. kFmulTickAdvance = fmul dword ptr [0x46DB08].
    g_privateTickAdvance = g_nativeTickAdvance;
    static const uint8_t kFmulTickAdvance[6] = { 0xD8, 0x0D, 0x08, 0xDB, 0x46, 0x00 };
    bool redirected = true;

    const size_t kSiteCount = sizeof(TICK_ADVANCE_OPERANDS) / sizeof(uint32_t);
    for (size_t i = 0; i < kSiteCount && redirected; ++i) {
        uint32_t rva = TICK_ADVANCE_OPERANDS[i];
        uint8_t* insn = exeBase + rva - 2;
        if (!g_tickCaveCodePages.Open(insn, sizeof(kFmulTickAdvance))) {
            Log(std::format("Tick cave: VirtualProtect on fmul at EXE+0x{:X} FAILED (err={})",
                            rva - 2, GetLastError()));
            redirected = false;
            break;
        }
        if (memcmp(insn, kFmulTickAdvance, sizeof(kFmulTickAdvance)) != 0) {
            Log(std::format("Tick cave: fmul at EXE+0x{:X} is not the expected instruction; not redirecting",
                            rva - 2));
            redirected = false;
            break;
        }
    }

    // The game thread may be running these instructions. The operands are
    // unaligned, so use a locked exchange, which is atomic at any alignment
    // (none crosses a cache line). Old and new addresses hold the same value.
    if (redirected) {
        LONG target = (LONG)(uintptr_t)&g_privateTickAdvance;
        for (uint32_t rva : TICK_ADVANCE_OPERANDS) {
            InterlockedExchange((volatile LONG*)(exeBase + rva), target);
        }
        g_tickCaveRedirectApplied = true;
    }

    if (redirected) {
        g_tickAdvancePtr = &g_privateTickAdvance;
        Log(std::format("Tick cave: {} in-game tick-advance readers redirected to DLL float at {:p} "
                        "(game constant at EXE+0x6DB08 left untouched for the menu)",
                        (int)(sizeof(TICK_ADVANCE_OPERANDS) / sizeof(uint32_t)),
                        (void*)&g_privateTickAdvance));
    } else {
        DWORD oldProtect = 0;
        if (!VirtualProtect(g_tickAdvancePtr, sizeof(float), PAGE_READWRITE, &oldProtect)) {
            Log(std::format("Tick cave: VirtualProtect on tick advance constant FAILED (err={})", GetLastError()));
            // Non-fatal: no speed scaling. Drop the pointer so the callback
            // doesn't write to a read-only page.
            g_tickAdvancePtr = nullptr;
        } else {
            g_tickAdvanceOriginalProtect = oldProtect;
            g_tickAdvanceProtectionChanged = true;
            Log(std::format("Tick cave: FALLBACK - writing the game's shared constant at {:p} (was 0x{:X}); "
                            "the menu video will speed up after a level", (void*)g_tickAdvancePtr, oldProtect));
        }
    }

    // Raise the game's per-frame tick clamp from 14h (20) to 40h (64) so fast
    // speeds aren't capped by it:
    //   EXE+0x25C83: immediate of `cmp esi, 14h`
    //   EXE+0x26001: immediate of `mov ebx, 14h`
    // Before the mid-hook, so SafetyHook's trampoline copies the new cmp.
    {
        uint8_t* cmp_imm = exeBase + 0x25C83;
        uint8_t* mov_imm = exeBase + 0x26001;
        bool cmp_ok = g_tickCaveCodePages.Open(cmp_imm, 1);
        bool mov_ok = g_tickCaveCodePages.Open(mov_imm, 1);
        if (cmp_ok && mov_ok) {
            if (*cmp_imm == 0x14 && *mov_imm == 0x14) {
                *cmp_imm = (uint8_t)TICK_CAVE_PER_FRAME_CAP;
                *mov_imm = (uint8_t)TICK_CAVE_PER_FRAME_CAP;
                g_tickCaveClampApplied = true;
                Log(std::format(
                    "Tick cave: raised tick clamp 0x14 -> 0x{:02X} at EXE+0x25C83 and EXE+0x26001",
                    TICK_CAVE_PER_FRAME_CAP));
            } else {
                Log(std::format(
                    "Tick cave: tick clamp bytes unexpected (cmp_imm=0x{:02X} mov_imm=0x{:02X}); not patching",
                    *cmp_imm, *mov_imm));
            }
        } else {
            Log(std::format(
                "Tick cave: VirtualProtect on tick clamp bytes FAILED (cmp_ok={} mov_ok={} err={})",
                cmp_ok, mov_ok, GetLastError()));
        }
    }

    // Restore page protection before SafetyHook does its own patching.
    g_tickCaveCodePages.CloseAll();

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

    auto* gameTickAdvance = (float*)(g_tickCaveExeBase + 0x6DB08);
    g_tickAdvancePtr = gameTickAdvance;

    if (g_tickCaveRedirectApplied) {
        const LONG original = (LONG)(uintptr_t)gameTickAdvance;
        for (uint32_t rva : TICK_ADVANCE_OPERANDS) {
            uint8_t* operand = g_tickCaveExeBase + rva;
            if (!g_tickCaveCodePages.Open(operand, sizeof(LONG))) continue;
            InterlockedExchange((volatile LONG*)operand, original);
        }
        g_tickCaveRedirectApplied = false;
    }

    if (g_tickCaveClampApplied) {
        uint8_t* cmp_imm = g_tickCaveExeBase + 0x25C83;
        uint8_t* mov_imm = g_tickCaveExeBase + 0x26001;
        if (g_tickCaveCodePages.Open(cmp_imm, 1) &&
            g_tickCaveCodePages.Open(mov_imm, 1)) {
            if (*cmp_imm == (uint8_t)TICK_CAVE_PER_FRAME_CAP) *cmp_imm = 0x14;
            if (*mov_imm == (uint8_t)TICK_CAVE_PER_FRAME_CAP) *mov_imm = 0x14;
        }
        g_tickCaveClampApplied = false;
    }

    g_tickCaveCodePages.CloseAll();

    if (g_tickAdvanceProtectionChanged) {
        *gameTickAdvance = g_nativeTickAdvance;
        DWORD ignored = 0;
        VirtualProtect(gameTickAdvance, sizeof(float),
                       g_tickAdvanceOriginalProtect, &ignored);
        g_tickAdvanceProtectionChanged = false;
    }
    if (g_tickCaveState) g_tickCaveState->tick_cave_hooked = 0;
    g_tickCaveState = nullptr;
    Log("Tick cave: hook and all supporting patches removed");
}
