#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../gate_alignment.hpp"
#include "../game_addresses.hpp"
#include <safetyhook.hpp>

// Cave 5: Fixed tick override with variable speed via time-advance scaling.
// Hook at Supreme.exe+25C81 (after __ftol call and mov esi, eax).
//
// At this point:
//   ESI = tick count from __ftol (may be garbage for large floats)
//   Game's next instruction: cmp esi, 14h; ja -> mov esi, 14h (clamp to 20)
//
// Speed scaling approach:
//   The game maintains an accumulator: prev_time += ticks * [EXE+0x25E2B const].
//   The per-tick time advance constant at EXE+0x46DB08 is normally 0.01 (= 1/100).
//   Scaling this constant by 1/speed changes how fast the game "consumes" time:
//     - 0.25x speed: per_tick = 0.04 → game advances time 4x per tick → 25 ticks/s
//     - 2.0x speed:  per_tick = 0.005 → game advances time 0.5x per tick → 200 ticks/s
//   This avoids the auto-compensation problem where scaling ESI directly had no
//   net effect because the game recomputes ticks from accumulated wall time.
//
// Priority:
//   1. force_fixed_tick > 0: force that exact value (typically 2, deterministic)
//   2. playback_speed != 1.0: write 0.01/speed to per-tick advance constant
//   3. Otherwise: clamp to [0, 20] (fix __ftol garbage)

inline TasSharedState* g_cave5State = nullptr;
static SafetyHookMid cave5Hook{};

// CONT clock-backlog reset: cave2 sets it at the splice (and at a judged
// PLAY's speed handover), cave5 consumes it on the next tick by advancing the
// game's time accumulator to "now" without processing the backlog ticks.
// DLL-private: both hooks run on the game thread.
inline volatile uint32_t g_contResetPending = 0;

// Pointer to the game's per-tick time advance constant (EXE + 0x46DB08).
// VirtualProtect'd to PAGE_READWRITE during init so we can write it.
static float* g_tickAdvancePtr = nullptr;

// The game keeps its per-tick time advance in ONE read-only float at
// EXE+0x46DB08, and sixteen `fmul dword ptr [0x46db08]` instructions read it.
// Four of them are in the game-cycle function this cave hooks; the other
// twelve are the menu. Nothing in the game ever WRITES it - it is a constant.
//
// cave5 scales it for speed playback and for the catch-up drain, and relies
// on its next call to put it back. On the quit-to-level->menu transition there
// is no next call: cave5 drains once on the transition and is then never
// called again while the menu is up, so the scaled value stays behind, the
// twelve menu instructions read it, Menu::Paint derives its animation dt from
// it, and the menu background video runs fast.
//
// So: give the four in-game readers a private copy and never touch the game's
// constant. The operand is a 4-byte absolute address inside a 6-byte
// instruction, so this is a same-length rewrite of four immediates - no
// relocation, no trampoline. In-game behaviour is bit-identical; the menu
// becomes structurally incapable of seeing the scaled value.
// inline, NOT static: the game code is patched to ONE fixed address, so a second
// translation unit getting its own copy would mean cave5 updating a different
// object than the one the patched instructions read.
inline float g_privateTickAdvance = 0.01f;

// RVAs of the 4-byte operand inside each in-game `fmul dword ptr [0x46db08]`
// (the instruction itself starts 2 bytes earlier):
//   +0x25CE0  fmul -> per-tick timestamp offset   (i * advance)
//   +0x25D89  fmul -> Time::Add(clock,  ticks_run * advance)
//   +0x25DB2  fmul -> Time::Add(clock2, ticks_demanded * advance)
//   +0x25E2B  fmul -> prev_time += ticks_demanded * advance   ([ebp+0x0C])
static constexpr uint32_t CAVE5_TICK_ADVANCE_OPERANDS[] = {
    0x25CE2, 0x25D8B, 0x25DB4, 0x25E2D,
};

// Tracks the ORIGINAL protection of every .text page InstallCave5 opens, so it
// can hand each one back exactly as it found it.
//
// One shared record for ALL of cave5's code patches, because they overlap:
// three of the four fmul operands and the `cmp esi,14h` clamp byte all live on
// page 0x425000. VirtualProtect reports the protection AT THE TIME OF THE CALL,
// so the second call on a page reports back the PAGE_EXECUTE_READWRITE the first
// one installed — only the first observation of a page is the truth.
//
// Restoring properly matters: PAGE_READWRITE drops EXECUTE, and a code page
// left that way only survives because a 1999 binary has no /NXCOMPAT bit and
// so runs with DEP off.
struct Cave5CodePages {
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

    // Make [addr, addr+len) writable+executable, remembering each page's
    // protection the first time we see it.
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

inline Cave5CodePages g_cave5CodePages{};

// Set once the operand redirect is actually live, so UninstallCave5 knows
// whether there is anything to undo. The patched instructions hold the absolute
// address of a float inside THIS DLL: if the DLL were ever unloaded without
// putting the original operand back, four instructions in the game's hot loop
// would dereference freed memory on the very next frame.
inline bool g_cave5RedirectApplied = false;
inline uint8_t* g_cave5ExeBase = nullptr;
inline bool g_cave5ClampApplied = false;
inline bool g_tickAdvanceProtectionChanged = false;
inline DWORD g_tickAdvanceOriginalProtect = 0;

// Documented default per-tick time advance — used only as a fallback if we
// somehow can't read the live value during init.
static constexpr float TICK_ADVANCE_DEFAULT = 0.01f;

// Native value of the time advance constant, captured during InstallCave5
// before we ever modify it. At 1x speed (or in OFF mode) we restore this
// rather than overwriting with a hardcoded guess; if the game's actual
// per-tick advance differs from 0.01 then forcing 0.01 silently changes
// the game's effective speed and (critically) leaves a stale value when
// the game pauses, producing a fast-forward on resume.
static float g_nativeTickAdvance = TICK_ADVANCE_DEFAULT;

// Per-frame ticks-per-cycle ceiling after raising the game's clamp (the game
// clamps esi to 0x14 = 20; the bytes are patched at install time). 64 is at
// the practical ceiling: per-frame game overhead dominates above it and
// reliability at the 64x setting degraded when it was raised further.
static constexpr int32_t CAVE5_PER_FRAME_TICK_CAP = 64;

// Original game clamp value (cmp esi, 14h). Cave5 enforces this in software
// during normal 1× gameplay so the game's defensive smooth-catchup behaviour
// is preserved — the larger CAVE5_PER_FRAME_TICK_CAP only kicks in for
// scripted fast-forward (playback_speed > 1) or the catchup-drain path.
static constexpr int32_t NATIVE_GAME_CLAMP_AT_1X = 20;

// Cave 5 callback with FPU preservation
static void Cave5_MidCallback(SafetyHookContext& ctx) {
    uint64_t t0 = __rdtsc();
    uint8_t fpu_buf[108];
    __asm { fsave [fpu_buf] }

    auto* s = g_cave5State;
    if (s) {
        int32_t realTick = (int32_t)ctx.esi;

        // Pause-resume catchup detection: when the game is unpaused after a
        // pause (Escape, or the "save replay" dialog at end of a run), __ftol
        // computes (wall_time - prev_time) / tick_advance and hands a huge
        // tick count (e.g. 20s pause @ 0.01s/tick = 2000) to the physics
        // loop. The original clamp at 20 just spreads the burst over many
        // frames (visible as a ~2× speedup for a second or so).
        //
        // Drain the accumulator in a single frame instead: set this frame's
        // tick_advance to realTick * native so 1 physics tick consumes the
        // entire wall-time gap. The game advances 1 physics tick (snowboarder
        // barely moves), prev_time catches up to wall_time, next frame is
        // back to a normal tick count under native tick_advance.
        //
        // Fires in any mode (OFF/REC/PLAY) at 1x, and at any speed when the
        // engine itself was frozen. Gated to force_fixed_tick == 0 so the
        // deterministic regression-suite path is untouched.
        const int32_t CATCHUP_THRESHOLD = 50;

        // Was the ENGINE itself frozen (dialog, menu, load) since our last
        // run? A backlog that appears after a freeze is wall-clock debt, not
        // simulation the user asked for — and it must be dropped at ANY
        // playback speed (a race finished at 2x otherwise replays the whole
        // save-dialog idle time on dismiss). Deliberate catch-up (CONT at
        // 256x) is not a freeze: cave5 runs every frame there, so the gap
        // stays ~7-16ms and this never fires on it.
        static uint32_t s_lastRunMs = 0;
        uint32_t nowMs = GetTickCount();
        bool resumed_from_freeze =
            s_lastRunMs != 0 && (nowMs - s_lastRunMs) > 250;
        s_lastRunMs = nowMs;

        // PARK — the aligned-CONT splice interlock. The splice is DESTRUCTIVE
        // (truncates recorded_count, flips PLAY to REC), and the gate-relative
        // watcher that validates the prefix lives in the controller process.
        // Its verdict normally lands thousands of ticks before the splice, but
        // nothing FORCED that order — a starved controller could let the
        // catch-up reach the splice unjudged. So an aligned CONT emits ZERO
        // ticks from the moment playback reaches its splice until the
        // controller writes cont_splice_approved. 0-tick frames are routine:
        // the sim and playback_pos freeze in place, the renderer keeps
        // presenting. If the controller dies parked, STOP or RESTART clears
        // the alignment and lifts the park. Unaligned CONT (gate_align_rec ==
        // 0) never parks.
        bool splice_parked = false;
        if (s->continue_from_frame > 0 && s->mode == MODE_PLAY
            && s->gate_align_rec != 0 && s->cont_splice_approved == 0) {
            uint32_t park_at = GateAlignedSplicePos(
                s->continue_from_frame, s->gate_index, s->gate_align_rec);
            splice_parked = s->playback_pos >= park_at;
        }

        bool catchup_drain =
            realTick > CATCHUP_THRESHOLD
            && s->force_fixed_tick == 0
            && !splice_parked  // a parked splice must not leak a drain tick
            && (s->playback_speed == 1.0f || resumed_from_freeze);

        // CONT clock-backlog reset (zero-cost, frame-exact at full speed).
        // When cave2 flags a splice, advance the game's time accumulator to
        // "now" so the resumed REC runs real-time from the splice frame, with
        // NO end-of-replay deceleration. The accumulator is the clock object's
        // +0x0C float, and at THIS hook site ctx.ebp IS that object (verified:
        // ebp=clock this, [ebp+0x0C]=seconds accumulator). raw demand
        // (realTick) = elapsed*[0x46DB0C](=100) = (now-prev)/native, so
        // realTick * native(0.01) = (now-prev) seconds → prev jumps to now. We
        // use NATIVE (not the speed-scaled tick_advance) so it's
        // speed-independent, and we do NOT process the backlog ticks (ctx.esi
        // stays capped below), so there's no recorded burst and no huge-dt jump.
        bool did_reset = false;
        if (g_contResetPending && !splice_parked) {  // parked: keep it pending, no tick may leak
            if (ctx.ebp) {
                float* prev_time = (float*)(uintptr_t)(ctx.ebp + 0x0C);
                *prev_time += (float)realTick * g_nativeTickAdvance;
            }
            g_contResetPending = 0;
            did_reset = true;
        }

        // Land the catch-up batch EXACTLY on the splice frame. cave5 sets the
        // per-frame tick count; capping it to the ticks-until-splice makes the
        // batch end on continue_from_frame so the PLAY→REC switch happens on the
        // LAST tick of the batch — no leftover catch-up ticks spill into the
        // resumed REC. Free: the batch was already ≤ cap; only the final replay
        // frame shortens.
        {
            uint32_t aligned_splice = GateAlignedSplicePos(
                s->continue_from_frame, s->gate_index, s->gate_align_rec);
            if (s->continue_from_frame > 0 && s->mode == MODE_PLAY
                && s->playback_pos < aligned_splice) {
                int32_t remaining = (int32_t)(aligned_splice - s->playback_pos);
                if (realTick > remaining) realTick = remaining;
            }
        }

        // Same treatment for a PLAY speed handover: land the batch exactly ON the
        // handoff position so the speed changes at that tick and not up to a whole
        // batch late. Without this the catch-up would routinely overshoot by tens
        // of ticks, which for a judged PLAY means skipping the start of the very
        // run the user asked to watch.
        if (s->speed_handoff_pos > 0 && s->mode == MODE_PLAY
            && s->playback_pos < s->speed_handoff_pos) {
            int32_t remaining = (int32_t)(s->speed_handoff_pos - s->playback_pos);
            if (realTick > remaining) realTick = remaining;
        }

        if (s->force_fixed_tick > 0) {
            // Deterministic mode: exact tick count per frame (0 while parked)
            ctx.esi = splice_parked ? 0 : (uintptr_t)s->force_fixed_tick;
        } else if (did_reset) {
            // Splice frame's backlog was just zeroed (prev → now); process a
            // single resume tick so the first REC frame doesn't re-burst.
            ctx.esi = 1;
        } else if (catchup_drain) {
            // Set tick_advance large enough that 1 tick drains the whole
            // wall-time gap (gap ≈ realTick * native because __ftol used
            // tick_advance = native last frame).
            if (g_tickAdvancePtr) {
                *g_tickAdvancePtr = (float)realTick * g_nativeTickAdvance;
            }
            ctx.esi = 1;
        } else {
            // Clamp raw tick first (fix __ftol garbage). The game's own
            // clamp (cmp esi, 14h / mov ebx, 14h) is patched at install
            // time to use 40h instead, so we match that here.
            if (realTick < 0) realTick = 0;
            if (realTick > CAVE5_PER_FRAME_TICK_CAP) realTick = CAVE5_PER_FRAME_TICK_CAP;

            // At normal playback speed (1×) preserve the game's original
            // defensive clamp at 20 ticks/frame. Only relax the cap when the
            // user has explicitly requested fast-forward via playback_speed >
            // 1. This keeps non-fast-forward play behaviorally identical to the
            // unpatched game — including its spiral-of-death protection for
            // moderate stalls. The catchup-drain branch above handles long
            // pauses (> 50 ticks accumulated) for 1× play, so this lower cap is
            // only the ceiling for "normal stutter recovery" at 1×.
            if (s->playback_speed <= 1.0f && realTick > NATIVE_GAME_CLAMP_AT_1X) {
                realTick = NATIVE_GAME_CLAMP_AT_1X;
            }

            if (splice_parked) realTick = 0;  // hold AT the splice until approved

            ctx.esi = (uintptr_t)realTick;
        }

        // Variable speed: only scale the time advance constant during active
        // REC/PLAY at a non-1x speed. At 1x, in OFF mode, or with invalid
        // speed, write the captured native value so the game runs at its
        // own natural speed. Writing the native value (vs not writing at
        // all) restores correct state when transitioning down from 2x→1x.
        // Skipped during catchup_drain — that path wrote a temporary large
        // value to tick_advance that the game must read next frame.
        if (g_tickAdvancePtr && !catchup_drain) {
            if (s->mode != MODE_OFF && s->playback_speed > 0.0f && s->playback_speed != 1.0f) {
                *g_tickAdvancePtr = g_nativeTickAdvance / s->playback_speed;
            } else {
                *g_tickAdvancePtr = g_nativeTickAdvance;
            }
        }
    }

    // Publish the per-frame tick count: the only external signal of how fast
    // the game is SIMULATING rather than rendering, which is exactly what a
    // fast-forward regression changes and a frame counter cannot see.
    //
    // Count what the game will actually RUN, not what we asked for: the tick
    // loop bounds itself with ebx, which the clamp caps at CAVE5_PER_FRAME_TICK_CAP.
    // esi above that is demand the game discards (reachable only via
    // force_fixed_tick > 64), and counting it would overstate the rate.
    if (auto* sp = g_cave5State) {
        uint32_t emitted = (uint32_t)ctx.esi;
        if (emitted > (uint32_t)CAVE5_PER_FRAME_TICK_CAP) {
            emitted = (uint32_t)CAVE5_PER_FRAME_TICK_CAP;
        }
        sp->tick_count += emitted;
    }

    __asm { frstor [fpu_buf] }
    auto* s2 = g_cave5State;
    if (s2) {
        PerfSample(s2->perf_cave5, __rdtsc() - t0);
    }
}

inline void UninstallCave5();

bool InstallCave5(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave5_site) {
        Log("Cave 5: hook site not resolved");
        return false;
    }

    g_cave5State = state;

    // Resolve the per-tick time advance constant at EXE+0x46DB08. This float
    // controls how much game-time each physics tick consumes from the
    // accumulator; scaling it gives variable speed playback.
    auto* exeBase = (uint8_t*)addr.exe;
    g_cave5ExeBase = exeBase;
    g_tickAdvancePtr = (float*)(exeBase + 0x6DB08);

    // Capture the native value before we ever modify the constant. The page
    // is readable even before VirtualProtect (it's part of the loaded image),
    // so this read is safe regardless of whether VirtualProtect succeeds.
    g_nativeTickAdvance = *g_tickAdvancePtr;
    Log(std::format("Cave 5: native tick advance constant = {}", g_nativeTickAdvance));

    // Point the four in-game readers at our own float so that scaling it can
    // never reach the menu.
    //
    // Verify EVERY site before writing ANY of them. A half-applied redirect is
    // worse than none: the fallback below goes back to writing the game's shared
    // constant, and any site already redirected would then be stuck reading a
    // private float that nothing updates again - a frozen tick advance for part
    // of the tick loop. All four or none.
    g_privateTickAdvance = g_nativeTickAdvance;
    static const uint8_t kFmulTickAdvance[6] = { 0xD8, 0x0D, 0x08, 0xDB, 0x46, 0x00 };
    bool redirected = true;

    // Pass 1: unprotect and verify.
    const size_t kSiteCount = sizeof(CAVE5_TICK_ADVANCE_OPERANDS) / sizeof(uint32_t);
    for (size_t i = 0; i < kSiteCount && redirected; ++i) {
        uint32_t rva = CAVE5_TICK_ADVANCE_OPERANDS[i];
        uint8_t* insn = exeBase + rva - 2;
        if (!g_cave5CodePages.Open(insn, sizeof(kFmulTickAdvance))) {
            Log(std::format("Cave 5: VirtualProtect on fmul at EXE+0x{:X} FAILED (err={})",
                            rva - 2, GetLastError()));
            redirected = false;
            break;
        }
        if (memcmp(insn, kFmulTickAdvance, sizeof(kFmulTickAdvance)) != 0) {
            Log(std::format("Cave 5: fmul at EXE+0x{:X} is not the expected instruction; not redirecting",
                            rva - 2));
            redirected = false;
            break;
        }
    }

    // Pass 2: commit.
    //
    // The game loop runs on another thread and may be executing these very
    // instructions right now. The operands are unaligned (2/3/0/1 mod 4), so a
    // plain store is not guaranteed to be observed as one write — and a torn
    // value here is half of one address and half of another, i.e. a garbage
    // pointer the fmul would dereference. A lock-prefixed exchange removes the
    // question: MSVC emits `lock xchg`, which x86 performs atomically at any
    // alignment (none of the four crosses a cache line). Every reader sees
    // either the old address or the new one, and at this instant both hold
    // 0.01, because g_privateTickAdvance was seeded from the game's own value.
    if (redirected) {
        LONG target = (LONG)(uintptr_t)&g_privateTickAdvance;
        for (uint32_t rva : CAVE5_TICK_ADVANCE_OPERANDS) {
            InterlockedExchange((volatile LONG*)(exeBase + rva), target);
        }
        g_cave5RedirectApplied = true;
    }

    if (redirected) {
        g_tickAdvancePtr = &g_privateTickAdvance;
        Log(std::format("Cave 5: {} in-game tick-advance readers redirected to DLL float at {:p} "
                        "(game constant at EXE+0x6DB08 left untouched for the menu)",
                        (int)(sizeof(CAVE5_TICK_ADVANCE_OPERANDS) / sizeof(uint32_t)),
                        (void*)&g_privateTickAdvance));
    } else {
        DWORD oldProtect = 0;
        if (!VirtualProtect(g_tickAdvancePtr, sizeof(float), PAGE_READWRITE, &oldProtect)) {
            Log(std::format("Cave 5: VirtualProtect on tick advance constant FAILED (err={})", GetLastError()));
            // Non-fatal: speed scaling won't work but fixed tick still does
        } else {
            g_tickAdvanceOriginalProtect = oldProtect;
            g_tickAdvanceProtectionChanged = true;
            Log(std::format("Cave 5: FALLBACK - writing the game's shared constant at {:p} (was 0x{:X}); "
                            "the menu video will speed up after a level", (void*)g_tickAdvancePtr, oldProtect));
        }
    }

    // Raise the game's per-frame tick clamp from 14h (20) to 40h (64) so
    // playback_speed > 12× actually delivers higher catch-up rates instead
    // of being bottlenecked by the game's own cmp/clamp pair. Two bytes:
    //   EXE+0x25C83: immediate of `cmp esi, 14h` (the comparison)
    //   EXE+0x26001: immediate of `mov ebx, 14h`  (the clamp value)
    // Patches must happen BEFORE installing the SafetyHook mid-hook —
    // SafetyHook captures the bytes at the hook site into its trampoline,
    // and we want that trampoline copy to use the bumped immediate.
    {
        uint8_t* cmp_imm = exeBase + 0x25C83;
        uint8_t* mov_imm = exeBase + 0x26001;
        // Same page record as the operand redirect above: cmp_imm shares page
        // 0x425000 with three of the four fmuls, so its own VirtualProtect would
        // report back OUR RWX rather than the game's original protection.
        bool cmp_ok = g_cave5CodePages.Open(cmp_imm, 1);
        bool mov_ok = g_cave5CodePages.Open(mov_imm, 1);
        if (cmp_ok && mov_ok) {
            // Sanity-check current values before clobbering — refuse to patch
            // if the game's bytes drifted from what we expect (defends against
            // wrong-build EXEs).
            if (*cmp_imm == 0x14 && *mov_imm == 0x14) {
                *cmp_imm = (uint8_t)CAVE5_PER_FRAME_TICK_CAP;
                *mov_imm = (uint8_t)CAVE5_PER_FRAME_TICK_CAP;
                g_cave5ClampApplied = true;
                Log(std::format(
                    "Cave 5: raised tick clamp 0x14 -> 0x{:02X} at EXE+0x25C83 and EXE+0x26001",
                    CAVE5_PER_FRAME_TICK_CAP));
            } else {
                Log(std::format(
                    "Cave 5: tick clamp bytes unexpected (cmp_imm=0x{:02X} mov_imm=0x{:02X}); not patching",
                    *cmp_imm, *mov_imm));
            }
        } else {
            Log(std::format(
                "Cave 5: VirtualProtect on tick clamp bytes FAILED (cmp_ok={} mov_ok={} err={})",
                cmp_ok, mov_ok, GetLastError()));
        }
    }

    // Every code patch is written; hand all touched pages back exactly as we
    // found them, icache flushed. Done BEFORE create_mid so SafetyHook does its
    // own protect/patch/restore on normally-protected pages, as it expects to.
    g_cave5CodePages.CloseAll();

    Log(std::format("Cave 5: hooking tick override at {:p} (EXE+0x25C81)", (void*)addr.cave5_site));

    cave5Hook = safetyhook::create_mid(addr.cave5_site, Cave5_MidCallback);

    if (!cave5Hook) {
        Log("Cave 5: SafetyHook create_mid FAILED");
        UninstallCave5();
        return false;
    }

    state->cave5_hooked = 1;
    Log("Cave 5: hook installed successfully");
    return true;
}

// Put the game's own operand back, so the redirect cannot outlive this DLL.
//
// The four patched instructions hold the absolute address of a float in our
// image. Unloading without undoing that leaves the game's hot loop dereferencing
// freed memory on its very next frame — a guaranteed crash, and one that would
// look like it came from anywhere but here. Successful initialization pins the
// DLL, but failed initialization uses this rollback before reporting failure.
inline void UninstallCave5() {
    // Stop callbacks before restoring anything they read or write.
    cave5Hook = {};
    if (!g_cave5ExeBase) return;

    auto* gameTickAdvance = (float*)(g_cave5ExeBase + 0x6DB08);
    g_tickAdvancePtr = gameTickAdvance;

    if (g_cave5RedirectApplied) {
        const LONG original = (LONG)(uintptr_t)gameTickAdvance;
        for (uint32_t rva : CAVE5_TICK_ADVANCE_OPERANDS) {
            uint8_t* operand = g_cave5ExeBase + rva;
            if (!g_cave5CodePages.Open(operand, sizeof(LONG))) continue;
            InterlockedExchange((volatile LONG*)operand, original);
        }
        g_cave5RedirectApplied = false;
    }

    if (g_cave5ClampApplied) {
        uint8_t* cmp_imm = g_cave5ExeBase + 0x25C83;
        uint8_t* mov_imm = g_cave5ExeBase + 0x26001;
        if (g_cave5CodePages.Open(cmp_imm, 1) &&
            g_cave5CodePages.Open(mov_imm, 1)) {
            if (*cmp_imm == (uint8_t)CAVE5_PER_FRAME_TICK_CAP) *cmp_imm = 0x14;
            if (*mov_imm == (uint8_t)CAVE5_PER_FRAME_TICK_CAP) *mov_imm = 0x14;
        }
        g_cave5ClampApplied = false;
    }

    g_cave5CodePages.CloseAll();

    if (g_tickAdvanceProtectionChanged) {
        *gameTickAdvance = g_nativeTickAdvance;
        DWORD ignored = 0;
        VirtualProtect(gameTickAdvance, sizeof(float),
                       g_tickAdvanceOriginalProtect, &ignored);
        g_tickAdvanceProtectionChanged = false;
    }
    if (g_cave5State) g_cave5State->cave5_hooked = 0;
    g_cave5State = nullptr;
    Log("Cave 5: hook and all supporting patches removed");
}
