#pragma once
#include "../stdafx.h"
#include <atomic>
#include "../cycle_stop_gate.hpp"
#include "../log.hpp"
#include "../gate_alignment.hpp"
#include "../input_gate.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../restart_release.hpp"
#include <safetyhook.hpp>
#include "cave5.hpp"   // g_contResetPending (cave2 sets at the splice, cave5 consumes)

// Cave 2: Supreme::Cycle hook (SG+0x13FE40)
// Main REC/PLAY engine. Fires every render frame during gameplay.
//
// FPU PRESERVATION:
// SafetyHookMid does NOT save x87 FPU state. The game does fld/fmul
// immediately after the hook site. Cave2_MidCallback wraps the logic in
// FSAVE/FRSTOR; the command/capture helpers below still avoid float ops and
// copy coordinates as integers so they stay safe wherever they are called.
//
// REC: sample the keyboard, write the DI buffer + action state, notify the
//      observer (BB3B10) on transitions, log the mask, capture coordinates.
// PLAY: read the logged mask (gate-aligned when armed), write it into the
//      game the same way, capture coordinates, splice to REC at the CONT
//      point.

inline TasSharedState* g_cave2State = nullptr;
inline std::atomic_flag g_cycleStopGate = ATOMIC_FLAG_INIT;
inline GameAddresses* g_cave2Addr = nullptr;
static SafetyHookMid cave2Hook{};

inline void UninstallCave2() {
    cave2Hook = {};
    if (g_cave2State) g_cave2State->cave2_hooked = 0;
    g_cave2State = nullptr;
    g_cave2Addr = nullptr;
}

typedef void(__thiscall* BB3B10Fn)(void* thisPtr, uint32_t keyIndex, uint32_t pressed,
                                    uint32_t unk, uint32_t arg4);

// Helper: safe pointer read with SEH protection against dangling pointers
// Returns 0 on access violation (freed memory during F5 restart)
static inline uint32_t SafeReadPtr(uint32_t addr) {
    if (!addr) return 0;
    __try {
        return *(uint32_t*)addr;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// Helper: read the keyboard object pointer chain
// root = [SG+1D5450], kbobj = [root+0x530]
static inline uint32_t GetKeyboardObject(GameAddresses* addr) {
    uint32_t root = SafeReadPtr((uint32_t)addr->player_base);
    if (!root) return 0;
    return SafeReadPtr(root + GameAddresses::KEYBOARD_OBJ_OFFSET);
}

// Helper: read the DI buffer pointer: [kbobj+0x30]
static inline uint32_t GetDIBuffer(uint32_t kbobj) {
    if (!kbobj) return 0;
    return SafeReadPtr(kbobj + GameAddresses::DI_BUFFER_PTR_OFFSET);
}

// Helper: sample input via GetAsyncKeyState (for symmetric REC path)
// Uses VK codes — works with Pico HID since it's a real USB HID device.
// No FPU operations; safe inside FSAVE/FRSTOR wrapper.
static uint8_t SampleGAKS() {
    uint8_t mask = 0;
    if (GetAsyncKeyState(VK_LEFT) & 0x8000)    mask |= INPUT_LEFT;
    if (GetAsyncKeyState(VK_RIGHT) & 0x8000)   mask |= INPUT_RIGHT;
    if (GetAsyncKeyState(VK_UP) & 0x8000)       mask |= INPUT_UP;
    if (GetAsyncKeyState(VK_DOWN) & 0x8000)     mask |= INPUT_DOWN;
    if (GetAsyncKeyState(VK_CONTROL) & 0x8000)  mask |= INPUT_JUMP;
    if (GetAsyncKeyState(VK_SHIFT) & 0x8000)    mask |= INPUT_SHIFT;
    return mask;
}

static void WriteDIBuffer(uint32_t buffer, uint8_t mask) {
    if (!buffer) return;
    __try {
        auto buf = (uint8_t*)buffer;
        // +3940 writes 0x01 for pressed, 0x00 for released — match that exactly
        buf[GameAddresses::KEY_LEFT]   = (mask & INPUT_LEFT)  ? 0x01 : 0x00;
        buf[GameAddresses::KEY_RIGHT]  = (mask & INPUT_RIGHT) ? 0x01 : 0x00;
        buf[GameAddresses::KEY_UP]     = (mask & INPUT_UP)    ? 0x01 : 0x00;
        buf[GameAddresses::KEY_DOWN]   = (mask & INPUT_DOWN)  ? 0x01 : 0x00;
        buf[GameAddresses::KEY_JUMP]   = (mask & INPUT_JUMP)  ? 0x01 : 0x00;
        buf[GameAddresses::KEY_JUMP2]  = (mask & INPUT_JUMP)  ? 0x01 : 0x00;
        buf[GameAddresses::KEY_SHIFT]  = (mask & INPUT_SHIFT) ? 0x01 : 0x00;
        buf[GameAddresses::KEY_SHIFT2] = (mask & INPUT_SHIFT) ? 0x01 : 0x00;
    } __except(EXCEPTION_EXECUTE_HANDLER) {}
}

static void WriteActionState(uint32_t kbobj, uint8_t mask) {
    if (!kbobj) return;
    __try {
        auto kb = (uint8_t*)kbobj;
        kb[GameAddresses::AS_LEFT]  = (mask & INPUT_LEFT)  ? 1 : 0;
        kb[GameAddresses::AS_RIGHT] = (mask & INPUT_RIGHT) ? 1 : 0;
        kb[GameAddresses::AS_UP]    = (mask & INPUT_UP)    ? 1 : 0;
        kb[GameAddresses::AS_DOWN]  = (mask & INPUT_DOWN)  ? 1 : 0;
        kb[GameAddresses::AS_JUMP]  = (mask & INPUT_JUMP)  ? 1 : 0;
        kb[GameAddresses::AS_SHIFT] = (mask & INPUT_SHIFT) ? 1 : 0;
    } __except(EXCEPTION_EXECUTE_HANDLER) {}
}

// BB3B10's 3rd+4th arguments are the {lo, hi} dwords of the 64-bit
// Kernel::Time the event was stamped with at message-pump dispatch (RE'd from
// the exported Win32_Driver::Translate(tagMSG&, Kernel::Time) chain: +3940
// forwards its Time args verbatim to BB3B10). The observer silently DISCARDS
// events whose Time predates the current race context. Injections are stamped
// with the game's own Kernel::Time::Current() (see CallBB3B10OnTransitions);
// this calibrated fallback value (Time.hi observed from real keypresses via
// cave1c/cave1d) only matters if the Kernel export ever fails to resolve.
inline volatile uint32_t g_bb3b10Arg4 = GameAddresses::BB3B10_ARG4;

// GetTickCount() stamped on every Supreme::Cycle tick. The message-pump-driven
// hooks (cave1c) use it to detect "the game is PAUSED / at a non-ticking
// screen" (pause menu, dialogs, static main menu): when the cycle hasn't
// ticked recently, the input gate passes ALL keys through so the user can
// operate the pause menu / dialogs even while a TAS mode is armed.
inline volatile uint32_t g_lastCycleMs = 0;

// Last injected input mask (REC and PLAY), for transition detection.
static uint8_t g_prevMask = 0;

class ScopedTasInjection {
public:
    ScopedTasInjection() { ++g_tasInjectionDepth; }
    ~ScopedTasInjection() {
        if (g_tasInjectionDepth > 0) --g_tasInjectionDepth;
    }
    ScopedTasInjection(const ScopedTasInjection&) = delete;
    ScopedTasInjection& operator=(const ScopedTasInjection&) = delete;
};

// Helper: get the current Kernel::Time from the game's own clock.
// SEH-protected; returns false if the export is unresolved or the call faults.
// The callee is x87-balanced (verified by disasm) and we already run game code
// (BB3B10 → observers) from this same hook context.
static bool GetKernelTimeNow(GameAddresses* addr, KernelTime* out) {
    if (!addr->time_current) return false;
    __try {
        addr->time_current(out, nullptr);
        return true;
    } __except(EXCEPTION_EXECUTE_HANDLER) { return false; }
}

// Helper: call BB3B10 for each changed bit (transitions only)
static void CallBB3B10OnTransitions(TasSharedState* s, GameAddresses* addr,
                                     uint32_t kbobj, uint8_t mask, uint8_t transitions) {
    if (!transitions || !kbobj) return;

    auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
    void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);

    // Stamp the injected events with the game's own current Kernel::Time —
    // exactly what a real keypress carries — so the observer never discards
    // them as stale. Priority: test override (forced wrong value, steer-impact
    // regression hook) > live Time::Current > keypress-calibrated fallback.
    KernelTime t = { 0, 0 };
    if (s->test_arg4_override) {
        t.lo = 0;
        t.hi = s->test_arg4_override;
        s->arg4_source = ARG4_SOURCE_OVERRIDE;
    } else if (GetKernelTimeNow(addr, &t)) {
        // Floor the lo dword: the stamp stays in-window for the observer's
        // event-time gate (hi is what the gate checks), and a FLOORED stamp is
        // DETERMINISTIC for every event in the same ~7-min hi-window, where the
        // live lo differs between a REC and its replay.
        t.lo = 0;
        s->arg4_source = ARG4_SOURCE_TIME_CURRENT;
    } else {
        t.lo = 0;
        t.hi = g_bb3b10Arg4;
        s->arg4_source = ARG4_SOURCE_CALIBRATED;
    }

    ScopedTasInjection injectionScope;

    if (transitions & INPUT_LEFT) {
        bb3b10(thisPtr, GameAddresses::KEY_LEFT, (mask & INPUT_LEFT) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_RIGHT) {
        bb3b10(thisPtr, GameAddresses::KEY_RIGHT, (mask & INPUT_RIGHT) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_UP) {
        bb3b10(thisPtr, GameAddresses::KEY_UP, (mask & INPUT_UP) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_DOWN) {
        bb3b10(thisPtr, GameAddresses::KEY_DOWN, (mask & INPUT_DOWN) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_JUMP) {
        bb3b10(thisPtr, GameAddresses::KEY_JUMP, (mask & INPUT_JUMP) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_SHIFT) {
        bb3b10(thisPtr, GameAddresses::KEY_SHIFT, (mask & INPUT_SHIFT) ? 1 : 0,
               t.lo, t.hi);
    }

    s->bb3b10_call_count++;
}

// Helper: capture player coordinates (SEH-protected, NO FLOAT OPS)
// Uses integer-width memcpy to avoid corrupting x87 FPU state.
static void CapturePlayerCoords(TasSharedState* s, uint32_t index, bool isRec) {
    // A capture that writes nothing still lets the caller advance the index,
    // leaving a STALE coordinate inside the current prefix. Anything scanning
    // that prefix for first movement can read the hole as movement, so the
    // failure has to be visible rather than silent.
    if (!s->player_ptr) {
        s->capture_ok = 0;
        return;
    }

    uint32_t raw[3];
    __try {
        auto player = (uint8_t*)s->player_ptr;
        memcpy(&raw[0], player + GameAddresses::PLAYER_X, 4);
        memcpy(&raw[1], player + GameAddresses::PLAYER_Y, 4);
        memcpy(&raw[2], player + GameAddresses::PLAYER_Z, 4);
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        s->capture_ok = 0;
        return;
    }

    // Update live position (integer-width copy, no float ops)
    memcpy(&s->player_x, &raw[0], 4);
    memcpy(&s->player_y, &raw[1], 4);
    memcpy(&s->player_z, &raw[2], 4);

    if (index < TAS_MAX_TICKS) {
        if (isRec) {
            memcpy(&s->rec_coords[index][0], &raw[0], 4);
            memcpy(&s->rec_coords[index][1], &raw[1], 4);
            memcpy(&s->rec_coords[index][2], &raw[2], 4);
        } else {
            memcpy(&s->play_coords[index][0], &raw[0], 4);
            memcpy(&s->play_coords[index][1], &raw[1], 4);
            memcpy(&s->play_coords[index][2], &raw[2], 4);
        }

        // Stamp the countdown gate: the first frame of this session whose
        // position differs from frame 0. Integer compare on the raw bits.
        // capture_ok also means frame 0 of THIS session was really written;
        // without it the comparison below is against the previous session's
        // frame 0 and can stamp a gate that never happened.
        if (s->capture_ok && s->gate_index == 0 && index > 0) {
            const uint32_t* z = isRec ? (const uint32_t*)&s->rec_coords[0][0]
                                      : (const uint32_t*)&s->play_coords[0][0];
            if (raw[0] != z[0] || raw[1] != z[1] || raw[2] != z[2]) {
                s->gate_index = index;
            }
        }
    }
}

// In-process F5 restart. restart_release.hpp owns the F5 byte and explains
// why it must go up as soon as the game has acted on it. This cycle cap only
// bounds the observer "up" if neither of its releases fires.
static constexpr uint32_t RESTART_F5_MAX_HOLD_FRAMES = 30;
static uint32_t g_restartFramesHeld = 0;                    // Cycles since the press

// Press or release F5 in the DI buffer and notify BB3B10.
static void InjectF5(GameAddresses* addr, uint32_t kbobj, bool pressed) {
    uint32_t buffer = GetDIBuffer(kbobj);
    if (buffer) {
        // The SEH write stays in a leaf function: MSVC rejects __try in a
        // function that also holds the ScopedTasInjection below (C2712).
        restartrelease::SafeWriteF5(buffer, pressed);
    }

    if (kbobj) {
        auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
        void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);
        ScopedTasInjection injectionScope;
        // Stamp F5 with the live Kernel::Time too. The restart itself is
        // driven by the DI-buffer write (the observer call is auxiliary), but
        // a wrong stamp here POISONS the observer's event-time window: a stamp
        // in the future makes later steering events look out-of-order and get
        // silently dropped — dead steering right after every injected F5.
        KernelTime ft = { 0, GameAddresses::BB3B10_ARG4 };
        GetKernelTimeNow(addr, &ft);
        ft.lo = 0;  // floored like the steering stamp (see CallBB3B10OnTransitions)
        bb3b10(thisPtr, GameAddresses::KEY_F5, pressed ? 1 : 0,
               ft.lo, ft.hi);
    }
}

// Clear the raw input state and return the mask that still needs observer UP
// notifications. The direct writes are safe from an out-of-cycle caller;
// calling game code from that potentially different thread is not.
static uint8_t ClearTasInputState(GameAddresses* addr) {
    uint8_t held = g_prevMask;
    g_prevMask = 0;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return held;
    WriteDIBuffer(GetDIBuffer(kbobj), 0);
    WriteActionState(kbobj, 0);
    return held;
}

// Release every input the TAS session injected: zero the DI buffer + action
// states and notify the observer that held keys went UP (live Time stamp).
// MUST run on every TAS→OFF transition — without it a key held by the replay
// at STOP stays "down" and the boarder keeps steering. Resolves the
// kbobj fresh, so after an auto-stop (level swapped) it clears the NEW
// level's state — release events for keys the new observer never saw pressed
// are no-ops, same as a real keyUp without a down.
static void ReleaseTasInput(TasSharedState* s, GameAddresses* addr) {
    uint8_t held = ClearTasInputState(addr);
    if (!held) return;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;
    CallBB3B10OnTransitions(s, addr, kbobj, 0, held);
}

// A STOP consumed out-of-cycle (the level-scan worker) clears the memory
// immediately, then defers observer callbacks until Supreme::Cycle is running
// on its normal hook thread again. Multiple frozen stops simply merge their
// held masks.
static volatile LONG g_deferredInputReleaseMask = 0;

static void DeferTasInputRelease(GameAddresses* addr) {
    uint8_t held = ClearTasInputState(addr);
    if (held) InterlockedOr(&g_deferredInputReleaseMask, (LONG)held);
}

static void FlushDeferredTasInputRelease(TasSharedState* s, GameAddresses* addr) {
    LONG held = InterlockedExchange(&g_deferredInputReleaseMask, 0);
    if (!held) return;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) {
        InterlockedOr(&g_deferredInputReleaseMask, held);
        return;
    }
    CallBB3B10OnTransitions(s, addr, kbobj, 0, (uint8_t)held);
}

// Drop any gate-relative input alignment. gate_align_rec is persistent shared
// memory that the controller stages right before each arm, and a leftover
// value would silently re-index a later replay and move its splice point. It
// is cleared on STOP, a refused CONT, restart, ARM_REC, a level change and the
// end of PLAY; the controller also clears it before every restart.
static inline void ClearGateAlign(TasSharedState* s) {
    s->gate_align_rec = 0;
    s->cont_splice_approved = 0;  // no alignment, nothing to approve
}

static volatile uint32_t g_cave2_pendingLog = 0;  // 0=none, 1=REC, 2=PLAY, 3=STOP, 4=playback_done
static volatile uint32_t g_cave2_logParam = 0;

// Root player pointer ([SG+1D5450]) captured at ARM time. It survives F5
// restarts but is reallocated when the level is torn down (quit to menu, the
// attract demo loading, a track switch). A mode left armed across that would
// record the demo, fast-forward the menu video and eat native keys, so Cave 2
// auto-stops when the live root no longer matches.
inline volatile uint32_t g_armedRoot = 0;

// Splice gate: 1 only between a SUCCESSFUL CMD_ARM_CONTINUE and its splice
// (or any stop/re-arm). The PLAY handler's splice check requires this flag,
// so a `continue_from_frame` that appears in shared memory by any other
// route (UI bug, stray writer, stale value) can NEVER convert a plain
// replay into REC. Deliberately a cave2-private static, not a shared-state
// field — external processes must not be able to set it.
static volatile uint32_t g_cave2_contArmed = 0;

// STOP also has to work while Supreme::Cycle is frozen. That is not an edge
// case: leaving a level is detected precisely because the Cycle hook stopped
// running. A STOP left for ProcessCommand() could therefore remain pending
// forever, leaving MODE_PLAY/MODE_REC asserted. The level-scan worker
// (level_scan.hpp) calls TryProcessStopCommand() while the cycle is frozen,
// so every consumer needs an atomic claim word.
static constexpr LONG CAVE2_CMD_CLAIMED_STOP = -1;

// Apply the complete TAS -> OFF transition. No logging/formatting/float
// arithmetic: this is called from the Cycle mid-hook and from the
// out-of-cycle consumer above.
static void ApplyStopTransition(TasSharedState* s, bool notifyObserverNow, bool protectRestart) {
    // Install protection before publishing OFF so real keys cannot enter the
    // restart window. Ordinary STOP still releases it for menu navigation.
    s->cont_suppress_input = protectRestart ? 1u : 0u;
    s->mode = MODE_OFF;
    if (notifyObserverNow) {
        ReleaseTasInput(s, g_cave2Addr);
    } else {
        DeferTasInputRelease(g_cave2Addr);
    }
    s->continue_from_frame = 0;
    g_cave2_contArmed = 0;
    ClearGateAlign(s);
    g_cave2_pendingLog = 3;
}

// Atomically claim and acknowledge a pending STOP. Returning command to IDLE
// is the completion publication and therefore happens only after every status
// and cleanup write. Compare-exchange on the final step avoids erasing a newer
// command if a misbehaving writer ignored the non-idle slot while cleanup ran.
static bool TryProcessStopCommand(TasSharedState* s, bool notifyObserverNow) {
    CycleStopGuard guard(g_cycleStopGate, !notifyObserverNow);
    if (!guard) return false;
    LONG expected = InterlockedCompareExchange((volatile LONG*)&s->command, CMD_IDLE, CMD_IDLE);
    if (expected != CMD_STOP && expected != CMD_STOP_FOR_RESTART) return false;
    LONG previous = InterlockedCompareExchange(
        (volatile LONG*)&s->command, CAVE2_CMD_CLAIMED_STOP, expected);
    if (previous != expected) return false;

    ApplyStopTransition(s, notifyObserverNow, expected == CMD_STOP_FOR_RESTART);
    InterlockedCompareExchange(
        (volatile LONG*)&s->command, CMD_IDLE, CAVE2_CMD_CLAIMED_STOP);
    return true;
}

// Reset the playback position, held-key mask and injection counters. Shared
// by every arm (REC, PLAY, CONT).
static void ResetArmCounters(TasSharedState* s) {
    s->playback_pos = 0;
    g_prevMask = 0;
    s->bb3b10_call_count = 0;
    s->handler_block_count = 0;
    s->bb3b10_block_count = 0;
}

static void ArmRec(TasSharedState* s) {
    s->recorded_count = 0;
    ResetArmCounters(s);
    s->segment_count = 1;
    s->segment_start_frame = 0;
    memset(s->segment_boundaries, 0, sizeof(s->segment_boundaries));
    s->segment_boundaries[0].frame = 0;
    s->mode = MODE_REC;
    g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
    g_cave2_contArmed = 0;
    // REC never replays; make sure it cannot inherit alignment.
    ClearGateAlign(s);
    g_cave2_pendingLog = 1;
}

static void ArmPlay(TasSharedState* s) {
    g_cave2_logParam = s->recorded_count;
    ResetArmCounters(s);

    // A plain PLAY never splices: clear any splice marker a refused or
    // stopped CONT left in shared memory.
    s->continue_from_frame = 0;
    g_cave2_contArmed = 0;

    // Do not force the spawn position: the rest of the physics state
    // (rotation, terrain contact) would still be the real spawn's, and
    // the mismatch drifts once steering starts.

    s->mode = MODE_PLAY;
    g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
    g_cave2_pendingLog = 2;
}

// Refuse an ARM_CONTINUE: log why and leave the mode OFF.
static void RefuseArmContinue(TasSharedState* s, const char* reason) {
    LogRing(s, LOG_ERROR, reason);
    s->mode = MODE_OFF;
    s->continue_from_frame = 0;  // refused — don't leave a stale marker armed
    g_cave2_contArmed = 0;
    ClearGateAlign(s);  // refusal must not leave alignment armed for a later replay
    g_cave2_pendingLog = 3;  // "stopped"
}

static void ArmContinue(TasSharedState* s) {
    // Continue Record: PLAY frames 0..continue_from_frame, then auto-switch to REC
    // Validate splice point is within recorded range
    if (s->continue_from_frame == 0 || s->continue_from_frame > s->recorded_count) {
        RefuseArmContinue(s, "ARM_CONTINUE: invalid splice point");
        return;
    }
    // Refuse ARM_CONTINUE while REC/PLAY is running: the prefix
    // replay assumes the boarder is at rec_coords[0]'s state, which
    // holds only right after an F5 restart. tas_ui always sends
    // CMD_RESTART first (RestartThen(ArmContinue)).
    if (s->mode != MODE_OFF) {
        RefuseArmContinue(s,
            "ARM_CONTINUE: refused — game is REC/PLAY; CONT requires a fresh restart first");
        return;
    }
    g_cave2_logParam = s->continue_from_frame;
    ResetArmCounters(s);
    // Segment tracking: keep existing segment_count, we'll add one at splice
    s->mode = MODE_PLAY;  // Start as PLAY, will auto-switch in PLAY handler
    g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
    g_cave2_contArmed = 1;  // the ONLY place the splice gate opens
    s->cont_splice_approved = 0;  // aligned attempts start unapproved (splice interlock)
    // Keep gate_align_rec: the controller staged it right before this
    // arm. The splice fires at the aligned play-index while the
    // recording stays in rec-index space (CompleteContinueSplice).
    g_cave2_pendingLog = 5;
}

// Begin in-process F5 restart sequence
static void BeginRestart(TasSharedState* s) {
    s->restart_state = 1;
    g_restartFramesHeld = 0;
    // The controller stages alignment AFTER the restart; anything
    // older is stale.
    ClearGateAlign(s);
    g_cave2_pendingLog = 7;  // "restart initiated"
}

// Handle command transitions
// WARNING: NO Log/format/float calls — runs inside SafetyHookMid (x87 FPU not saved).
// Returns false when the out-of-cycle worker currently owns STOP cleanup. The
// caller must skip REC/PLAY work for that cycle rather than race the cleanup.
static bool ProcessCommand(TasSharedState* s) {
    // Cross-process acquire for the command publication word. The Rust writer
    // stages every payload field first and stores command with Release.
    uint32_t cmd = (uint32_t)InterlockedCompareExchange(
        (volatile LONG*)&s->command, CMD_IDLE, CMD_IDLE);
    if (cmd == CMD_IDLE) return true;
    if ((LONG)cmd == CAVE2_CMD_CLAIMED_STOP) return false;
    if (cmd == CMD_STOP || cmd == CMD_STOP_FOR_RESTART) {
        return TryProcessStopCommand(s, true);
    }

    switch (cmd) {
        case CMD_ARM_REC:      ArmRec(s);       break;
        case CMD_ARM_PLAY:     ArmPlay(s);      break;
        case CMD_ARM_CONTINUE: ArmContinue(s);  break;
        case CMD_RESTART:      BeginRestart(s); break;
    }

    // arm_generation is the controller's "the arm landed" signal, so it is
    // bumped after the switch. It must be the LAST store of the arm: x86 keeps
    // store order, so a reader that sees it move also sees the mode and
    // position the arm wrote. It also counts REFUSED arms (the early breaks
    // above), which otherwise leave nothing to wait on but a mode that stays OFF.
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE || cmd == CMD_ARM_REC) {
        // Fresh session: the gate has not fired yet, and no capture has failed.
        s->gate_index = 0;
        s->capture_ok = 1;
    }
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE) {
        s->arm_generation++;
    }

    // Publish command completion after all resulting mode/status writes.
    InterlockedExchange((volatile LONG*)&s->command, CMD_IDLE);
    return true;
}

// Flush deferred log (call from OUTSIDE the SafetyHookMid callback)
static void FlushPendingLog() {
    uint32_t log = g_cave2_pendingLog;
    if (!log) return;
    g_cave2_pendingLog = 0;
    uint32_t param = g_cave2_logParam;
    switch (log) {
        case 1: Log("Cave 2: entering REC mode"); break;
        case 2: Log(std::format("Cave 2: entering PLAY mode ({} ticks recorded)", param)); break;
        case 3: Log("Cave 2: stopped"); break;
        case 4: Log(std::format("Cave 2: playback complete at frame {}", param)); break;
        case 5: Log(std::format("Cave 2: continue record (PLAY until frame {})", param)); break;
        case 6: Log(std::format("Cave 2: spliced to REC at frame {}", param)); break;
        case 7: Log("Cave 2: in-process F5 restart initiated"); break;
        case 8: Log("Cave 2: F5 released, restart complete"); break;
    }
}

// Complete before processing a cycle as well as after the last prefix tick.
// The controller may only approve after cave5 has parked at the splice;
// processing another PLAY tick on resume duplicates that tick in the saved run.
static void CompleteContinueSplice(TasSharedState* s) {
    uint32_t aligned_splice = GateAlignedSplicePos(
        s->continue_from_frame, s->gate_index, s->gate_align_rec);
    // Aligned splice interlock: only a watcher-approved prefix may be
    // spliced. cave5 parks playback AT the splice while unapproved.
    if (g_cave2_contArmed && s->continue_from_frame > 0
            && s->playback_pos >= aligned_splice
            && (s->gate_align_rec == 0 || s->cont_splice_approved != 0)) {
        uint32_t rec_splice = s->continue_from_frame;
        s->recorded_count = rec_splice;
        // Switch to the user's resume speed at the exact splice tick; waiting
        // for the UI to notice PLAY->REC would record the start of the take at
        // the catch-up rate.
        if (s->cont_resume_speed > 0.0f) {
            s->playback_speed = s->cont_resume_speed;
        }
        // Tell cave5 to discard the catch-up clock backlog so REC starts
        // frame-exact at the resume speed.
        g_contResetPending = 1;

        uint32_t segIdx = s->segment_count;
        if (segIdx < TAS_MAX_SEGMENTS) {
            s->segment_boundaries[segIdx].frame = rec_splice;
            s->segment_count = segIdx + 1;
        }
        s->segment_start_frame = rec_splice;

        s->mode = MODE_REC;
        s->continue_from_frame = 0;  // Clear splice marker
        g_cave2_contArmed = 0;       // splice consumed — close the gate
        g_cave2_logParam = rec_splice;
        g_cave2_pendingLog = 6;
    }
}

// Publish the live position, velocity and rotation in every mode.
// Inside FSAVE/FRSTOR, so float ops are safe here. Holds the __try, so it
// must not own any C++ object needing unwinding (C2712).
static void PublishLivePosition(TasSharedState* s) {
    if (s->player_ptr) {
        __try {
            auto player = (uint8_t*)s->player_ptr;
            float new_x, new_y, new_z;
            memcpy(&new_x, player + GameAddresses::PLAYER_X, 4);
            memcpy(&new_y, player + GameAddresses::PLAYER_Y, 4);
            memcpy(&new_z, player + GameAddresses::PLAYER_Z, 4);

            s->velocity_x = new_x - s->player_x;
            s->velocity_y = new_y - s->player_y;
            s->velocity_z = new_z - s->player_z;

            s->player_x = new_x;
            s->player_y = new_y;
            s->player_z = new_z;
        } __except(EXCEPTION_EXECUTE_HANDLER) {}
    }
}

// In-process F5 restart state machine (runs regardless of mode)
static void StepInProcessRestart(TasSharedState* s, GameAddresses* addr) {
    if (s->restart_state == 1) {
        uint32_t kbobj = GetKeyboardObject(addr);
        if (kbobj) {
            if (g_restartFramesHeld == 0) {
                // Press: the byte AND the observer. restart_release.hpp
                // takes the byte back the moment the restart is observed.
                restartrelease::Pressed(GetDIBuffer(kbobj));
                InjectF5(addr, kbobj, true);
            }
            g_restartFramesHeld++;
            // Finish once the byte is up (the restart hook or the worker's
            // wall-clock cap released it): observer "up" + done. The cycle
            // cap only bounds a run where neither ever fires.
            if (!restartrelease::Pending() || g_restartFramesHeld >= RESTART_F5_MAX_HOLD_FRAMES) {
                restartrelease::ReleaseByteNow(3);
                InjectF5(addr, kbobj, false);
                s->restart_state = 2;  // Done
                g_cave2_pendingLog = 8;
            }
        }
    }
}

// Auto-stop when the level is swapped out under an armed mode (see
// g_armedRoot). root==0 is not a trigger: restarts pass through it.
// Returns true when it stopped the session.
static bool AutoStopIfLevelChanged(TasSharedState* s, GameAddresses* addr) {
    uint32_t curRoot = SafeReadPtr((uint32_t)addr->player_base);
    if (curRoot && g_armedRoot && curRoot != g_armedRoot) {
        s->mode = MODE_OFF;
        ReleaseTasInput(s, addr);  // clears the NEW level's input state
        s->continue_from_frame = 0;
        g_cave2_contArmed = 0;
        ClearGateAlign(s);
        // The level is gone, so no restart is in flight: release the
        // live-input block.
        s->cont_suppress_input = 0;
        g_armedRoot = 0;
        LogRing(s, LOG_WARN,
            "TAS auto-stopped: level context changed (left the race / menu demo loaded)");
        g_cave2_pendingLog = 3;  // "stopped"
        return true;
    }
    return false;
}

static void RecTick(TasSharedState* s, GameAddresses* addr, uint32_t kbobj) {
    uint32_t index = s->recorded_count;
    if (index >= TAS_MAX_TICKS) {
        // The buffer holds 10 min 55 s at 100 ticks/s. Say so, rather than
        // ending the take indistinguishably from a user STOP.
        LogRing(s, LOG_WARN, "REC stopped: recording buffer full (TAS_MAX_TICKS)");
        s->mode = MODE_OFF;
        ReleaseTasInput(s, addr);
        g_cave2_pendingLog = 3;
        return;
    }

    // Sample input via GAKS (Cave 1C blocks +3940, so game buffer is empty).
    // This makes REC symmetric with PLAY: both write buffer + action_state
    // + BB3B10 at the same point in Supreme::Cycle.
    uint8_t mask = SampleGAKS();
    uint8_t transitions = mask ^ g_prevMask;

    uint32_t buffer = GetDIBuffer(kbobj);
    WriteDIBuffer(buffer, mask);

    WriteActionState(kbobj, mask);

    if (transitions) {
        CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
    }

    s->input_log[index] = mask;
    g_prevMask = mask;

    CapturePlayerCoords(s, index, true);

    s->recorded_count = index + 1;
}

static void PlayTick(TasSharedState* s, GameAddresses* addr, uint32_t kbobj) {
    uint32_t pos = s->playback_pos;

    // With alignment the end is gate-relative too, so a replay whose gate
    // landed early still plays every recorded input. recorded_count is
    // written by the UI process; cap it so it can never index past
    // input_log.
    const uint32_t rec_count =
        s->recorded_count < TAS_MAX_TICKS ? s->recorded_count : TAS_MAX_TICKS;
    uint32_t play_end = rec_count;
    if (s->gate_align_rec > 0 && s->gate_index > 0 && rec_count > s->gate_align_rec) {
        play_end = s->gate_index + (rec_count - s->gate_align_rec);
    }
    if (pos >= play_end) {
        s->mode = MODE_OFF;
        ClearGateAlign(s);  // aligned PLAY finished — don't leave it armed
        ReleaseTasInput(s, addr);  // replay done — un-stick its held keys
        g_cave2_contArmed = 0;  // hygiene — an armed CONT always splices before here
        g_cave2_logParam = pos;
        g_cave2_pendingLog = 4;
        return;
    }

    // Gate-relative input alignment (gate_alignment.hpp). Input is indexed
    // from the gate, not the arm, so a countdown a tick longer or shorter
    // than the recording's does not shift the run's input timing. The
    // controller's trajectory watcher rejects a differing spawn state.
    //
    // Before the live gate, replay the recording's input, then HOLD its
    // gate mask from the pre-gate lead on. gate_index is stamped at the
    // END of the cycle whose position first differs, so that cycle's mask
    // is chosen before the gate is known; injecting nothing there loses a
    // cycle of input (a constant 0.389 drift). The boarder cannot move
    // before the gate, so the held mask is inert until it is the right
    // one. Earlier transitions are kept because the input observer sees
    // them.
    uint32_t src = pos;
    if (s->gate_align_rec > 0) {
        src = GateAlignedInputSource(pos, s->gate_index, s->gate_align_rec, rec_count);
    }
    uint8_t mask = (src == GATE_ALIGN_INVALID_SOURCE) ? (uint8_t)0 : s->input_log[src];

    uint32_t buffer = GetDIBuffer(kbobj);
    WriteDIBuffer(buffer, mask);

    WriteActionState(kbobj, mask);

    uint8_t transitions = mask ^ g_prevMask;
    if (transitions) {
        CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
    }

    g_prevMask = mask;

    CapturePlayerCoords(s, pos, false);

    s->playback_pos = pos + 1;

    CompleteContinueSplice(s);
}

// Cave 2 callback logic — called with FPU state saved/restored.
// Separated from the FSAVE wrapper because MSVC forbids __asm in functions with SEH.
static void __declspec(noinline) Cave2_Logic() {
    auto* s = g_cave2State;
    auto* addr = g_cave2Addr;
    if (!s || !addr) return;

    // Finish any observer notifications deferred by a frozen-cycle STOP. Raw
    // buffers were already cleared out-of-cycle; this only publishes key-up
    // transitions from the game thread once it exists again.
    FlushDeferredTasInputRelease(s, addr);

    s->frame_count++;
    g_lastCycleMs = GetTickCount();  // pause detector heartbeat (see cave1c)

    // Game-state awareness: publish exe+0x8895C (0=menu, 1=in-game) so the UI
    // knows the state. Integer read — FPU-safe.
    if (addr->is_in_game) {
        s->game_in_game = *(volatile uint32_t*)addr->is_in_game;
    }

    if (s->replay_ptr) {
        s->player_ptr = SafeReadPtr(s->replay_ptr + GameAddresses::REPLAY_PLAYER_OFFSET);
    }

    PublishLivePosition(s);

    if (!ProcessCommand(s)) return;

    // The restart hook cuts a PHYSICAL F5 short too (restart_release.hpp);
    // it needs the key buffer, which only this thread can resolve.
    if (uint32_t kb = GetKeyboardObject(addr)) restartrelease::NoteBuffer(GetDIBuffer(kb));

    StepInProcessRestart(s, addr);

    // The level-context epoch is bumped by level_scan.hpp's worker, not here:
    // Supreme::Cycle is frozen during the menus and loads it must detect.

    if (s->mode == MODE_OFF) return;

    if (AutoStopIfLevelChanged(s, addr)) return;

    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;

    if (s->mode == MODE_PLAY) CompleteContinueSplice(s);

    if (s->mode == MODE_REC) {
        RecTick(s, addr, kbobj);
    } else if (s->mode == MODE_PLAY) {
        PlayTick(s, addr, kbobj);
    }
}

// SafetyHookMid callback with explicit x87 FPU preservation.
// FSAVE saves all 8 ST registers + control/status (108 bytes) and reinits FPU.
// FRSTOR restores everything before returning to game code.
static void Cave2_MidCallback(SafetyHookContext& ctx) {
    uint8_t fpu_buf[108];
    __asm { fsave [fpu_buf] }
    auto* s = g_cave2State;
    if (s) {
        // The game thread's x87 control word = the precision the renderer left
        // the physics running at (0x007F DirectX 6/7 = 24-bit, OpenGL = 53/64).
        // It is per-thread state, and FSAVE above re-initialises the FPU
        // (an fnstcw inside Cave2_Logic reads the post-init 0x037F), so take
        // it from the saved image: the FSAVE protected-mode layout starts
        // with the control word.
        uint16_t cw;
        memcpy(&cw, fpu_buf, sizeof(cw));
        s->fpu_control_word = cw;
    }
    {
        CycleStopGuard guard(g_cycleStopGate);
        if (guard) Cave2_Logic();
    }
    __asm { frstor [fpu_buf] }
}

bool InstallCave2(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cave2_site) {
        Log("Cave 2: hook site not resolved");
        return false;
    }

    g_cave2State = state;
    g_cave2Addr = &addr;
    Log(std::format("Cave 2: hooking Supreme::Cycle at {:p}", (void*)addr.cave2_site));

    cave2Hook = safetyhook::create_mid(addr.cave2_site, Cave2_MidCallback);

    if (!cave2Hook) {
        Log("Cave 2: SafetyHook create_mid FAILED");
        g_cave2State = nullptr;
        g_cave2Addr = nullptr;
        return false;
    }

    state->cave2_hooked = 1;
    Log("Cave 2: hook installed successfully");
    return true;
}
