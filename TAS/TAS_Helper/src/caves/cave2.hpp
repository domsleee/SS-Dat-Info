#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"
#include "snapshot.hpp"

// Cave 2: Supreme::Cycle hook (SG+0x13FE40)
// Main REC/PLAY engine. Fires every render frame during gameplay.
//
// FPU PRESERVATION:
// SafetyHookMid does NOT save x87 FPU state. The game does fld/fmul
// immediately after the hook site. ALL float operations in this callback
// MUST be avoided — use integer-width memcpy for coordinate capture.
// Drift computation is deferred to the Rust test harness post-playback.
//
// REC (inject_mode=6, source=0):
//   1. Sample DI buffer -> build 6-bit input mask
//   2. Write action_state from mask
//   3. Call BB3B10 on transitions (with cave2_injecting=1)
//   4. Store mask in input_log[index]
//   5. Capture player coordinates (integer-width copy)
//   6. Increment recorded_count
//
// PLAY (inject_mode=6, force_direct=2):
//   1. Read mask from input_log[playback_pos]
//   2. Write DI buffer from mask
//   3. Write action_state from mask
//   4. Call BB3B10 on transitions (with cave2_injecting=1)
//   5. Capture player coordinates (integer-width copy)
//   6. Increment playback_pos (stop at recorded_count)

// Globals accessed by hook callback
inline TasSharedState* g_cave2State = nullptr;
inline GameAddresses* g_cave2Addr = nullptr;
static SafetyHookMid cave2Hook{};

// Hand-rolled hex for hook-context RDIAG logs (no CRT/format in a hook).
static inline void DiagHexU32(char* dst, uint32_t v) {
    static const char H[] = "0123456789ABCDEF";
    for (int i = 0; i < 8; ++i) dst[i] = H[(v >> ((7 - i) * 4)) & 0xF];
}

// BB3B10 function type: __thiscall with 4 stack args
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

// Helper: sample DI buffer into a 6-bit mask (SEH-protected)
static uint8_t SampleDIBuffer(uint32_t buffer) {
    if (!buffer) return 0;
    __try {
        uint8_t mask = 0;
        auto buf = (uint8_t*)buffer;
        if (buf[GameAddresses::KEY_LEFT])   mask |= INPUT_LEFT;
        if (buf[GameAddresses::KEY_RIGHT])  mask |= INPUT_RIGHT;
        if (buf[GameAddresses::KEY_UP])     mask |= INPUT_UP;
        if (buf[GameAddresses::KEY_DOWN])   mask |= INPUT_DOWN;
        if (buf[GameAddresses::KEY_JUMP] || buf[GameAddresses::KEY_JUMP2])  mask |= INPUT_JUMP;
        if (buf[GameAddresses::KEY_SHIFT] || buf[GameAddresses::KEY_SHIFT2]) mask |= INPUT_SHIFT;
        return mask;
    } __except(EXCEPTION_EXECUTE_HANDLER) { return 0; }
}

// Helper: write DI buffer from mask (SEH-protected)
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

// Helper: write action_state bytes from mask (SEH-protected)
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

// BB3B10's 4th argument is NOT a constant: it drifts with game/session state
// (live-captured 0x96 on 2026-06-03, observed 0x8F on 2026-06-10), and the
// game silently DISCARDS injected calls whose arg4 doesn't match — the same
// silent-steering-no-op failure mode as the original 0x588 bug. So we
// self-calibrate: cave1d updates this from every REAL BB3B10 call it passes
// through (menu keys, F5, any keypress), and all injection sites use the
// captured value. Falls back to the last-known-good constant until the first
// real call is seen.
inline volatile uint32_t g_bb3b10Arg4 = GameAddresses::BB3B10_ARG4;

// RDIAG: one-shot guard so we log only the FIRST injection's arg4 per arm.
// Reset at ARM_REC / ARM_PLAY (see ProcessCommand).
inline volatile uint32_t g_diagInjectLogged = 0;

// Set by cave1c/cave1d the moment a REAL handler call updates g_bb3b10Arg4 to a
// new value while recording/playing. The injected arg4 is dynamic and only
// learnable from a real keypress, so the very first injected transition can
// race ahead of calibration and be silently dropped (the "dead until warmup"
// bug). When this fires, cave2 re-asserts the currently-held input with the
// now-correct arg4 on its next tick, so the dropped press lands within a frame
// — deterministically, no warmup. Cleared by cave2 after the re-inject.
inline volatile uint32_t g_arg4Recalibrated = 0;

// Helper: call BB3B10 for each changed bit (transitions only)
static void CallBB3B10OnTransitions(TasSharedState* s, GameAddresses* addr,
                                     uint32_t kbobj, uint8_t mask, uint8_t transitions) {
    if (!transitions || !kbobj) return;

    auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
    void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);

    // RDIAG: log the arg4 value the FIRST injection uses per arm — shows
    // whether REC injects with the live race value (0x01) or the stale
    // default (0x96 → silently discarded → cold steering dead).
    if (!g_diagInjectLogged) {
        g_diagInjectLogged = 1;
        char buf[48]; int p = 0;
        auto put = [&](const char* t){ while (*t && p < 36) buf[p++] = *t++; };
        put("RDIAG inject m="); DiagHexU32(buf + p, s->mode); p += 8;
        put(" a4="); DiagHexU32(buf + p, g_bb3b10Arg4); p += 8; buf[p] = '\0';
        LogRing(s, LOG_INFO, buf);
    }

    // Set cave2_injecting so Cave 1C/1D pass through
    s->cave2_injecting = 1;

    if (transitions & INPUT_LEFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_LEFT, (mask & INPUT_LEFT) ? 1 : 0,
               0, g_bb3b10Arg4);
    }
    if (transitions & INPUT_RIGHT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_RIGHT, (mask & INPUT_RIGHT) ? 1 : 0,
               0, g_bb3b10Arg4);
    }
    if (transitions & INPUT_UP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_UP, (mask & INPUT_UP) ? 1 : 0,
               0, g_bb3b10Arg4);
    }
    if (transitions & INPUT_DOWN) {
        bb3b10(thisPtr, GameAddresses::BB3B10_DOWN, (mask & INPUT_DOWN) ? 1 : 0,
               0, g_bb3b10Arg4);
    }
    if (transitions & INPUT_JUMP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_JUMP, (mask & INPUT_JUMP) ? 1 : 0,
               0, g_bb3b10Arg4);
    }
    if (transitions & INPUT_SHIFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_SHIFT, (mask & INPUT_SHIFT) ? 1 : 0,
               0, g_bb3b10Arg4);
    }

    s->cave2_injecting = 0;
    s->bb3b10_call_count++;
}

// Helper: capture player coordinates (SEH-protected, NO FLOAT OPS)
// Uses integer-width memcpy to avoid corrupting x87 FPU state.
// Drift computation is deferred to Rust test harness post-playback.
static void CapturePlayerCoords(TasSharedState* s, uint32_t index, bool isRec) {
    if (!s->player_ptr) return;

    uint32_t raw[3];
    uint32_t rot_raw[9];
    bool got_rot = false;
    __try {
        auto player = (uint8_t*)s->player_ptr;
        memcpy(&raw[0], player + GameAddresses::PLAYER_X, 4);
        memcpy(&raw[1], player + GameAddresses::PLAYER_Y, 4);
        memcpy(&raw[2], player + GameAddresses::PLAYER_Z, 4);
        // Read rotation from physics sub-object
        uint32_t physics = 0;
        memcpy(&physics, player + GameAddresses::PLAYER_PHYSICS, 4);
        if (physics) {
            memcpy(rot_raw, (uint8_t*)physics + GameAddresses::PHYSICS_ROT, 36);
            got_rot = true;
        }
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        return;
    }

    // Update live position (integer-width copy, no float ops)
    memcpy(&s->player_x, &raw[0], 4);
    memcpy(&s->player_y, &raw[1], 4);
    memcpy(&s->player_z, &raw[2], 4);
    // Update rotation matrix (integer-width copy, no float ops)
    if (got_rot) {
        memcpy(s->rotation_matrix, rot_raw, 36);
    }

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
    }
}

// In-process F5 restart constants
static constexpr uint32_t RESTART_F5_HOLD_FRAMES = 10;  // Hold F5 for 10 frames

// Helper: press or release F5 in the DI buffer + notify BB3B10
static void InjectF5(TasSharedState* s, GameAddresses* addr, uint32_t kbobj, bool pressed) {
    uint32_t buffer = GetDIBuffer(kbobj);
    if (buffer) {
        __try {
            ((uint8_t*)buffer)[GameAddresses::KEY_F5] = pressed ? 0x01 : 0x00;
        } __except(EXCEPTION_EXECUTE_HANDLER) {}
    }

    // Notify BB3B10 of the F5 state change
    if (kbobj) {
        auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
        void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);
        s->cave2_injecting = 1;
        bb3b10(thisPtr, GameAddresses::BB3B10_F5, pressed ? 1 : 0,
               0, g_bb3b10Arg4);
        s->cave2_injecting = 0;
    }
}

// Deferred log messages — set in callback, logged outside callback
// Restart/root diagnostics (RDIAG): the visible-steering-dies-after-REC bug
// tracks to the game's root player pointer ([SG+1D5450]) dangling across the
// in-process restart. These hook-safe ring logs capture the root, a vtable
// readability probe, and the resolved keyboard object at each restart stage so
// one repro shows exactly where the chain breaks. No CRT formatting (hook
// context) — hand-rolled hex.
static uint32_t g_diagRootAtRestartCmd = 0;

static void LogRootDiag(TasSharedState* s, const char* stage) {
    auto* addr = g_cave2Addr;
    if (!s || !addr) return;
    uint32_t root = SafeReadPtr((uint32_t)addr->player_base);
    uint32_t vt   = root ? SafeReadPtr(root) : 0;   // 0 = root dangles
    uint32_t kb   = root ? SafeReadPtr(root + GameAddresses::KEYBOARD_OBJ_OFFSET) : 0;
    char buf[112];
    int p = 0;
    auto put = [&](const char* t) { while (*t && p < 100) buf[p++] = *t++; };
    put("RDIAG ");
    put(stage);
    put(" root=");  DiagHexU32(buf + p, root); p += 8;
    put(" vt=");    DiagHexU32(buf + p, vt);   p += 8;
    put(" kb=");    DiagHexU32(buf + p, kb);   p += 8;
    put(" prev=");  DiagHexU32(buf + p, g_diagRootAtRestartCmd); p += 8;
    buf[p] = '\0';
    LogRing(s, LOG_INFO, buf);
}

static volatile uint32_t g_cave2_pendingLog = 0;  // 0=none, 1=REC, 2=PLAY, 3=STOP, 4=playback_done
static volatile uint32_t g_cave2_logParam = 0;

// Auto-calibration pulse. arg4 is dynamic and only learnable from a REAL
// handler call; arming REC/PLAY we may hold a stale (menu-context) value, and
// whether the handler fires again during REC is unreliable. So before
// recording/replaying we spend a few OFF-mode ticks toggling a throwaway key
// into the DI buffer — the game dispatches its real keyDown handler (+3940),
// which cave1c calibrates g_bb3b10Arg4 from, picking up the current race-context
// value. Runs in the post-restart countdown where the boarder can't move, so
// the pulse is invisible and unrecorded. Then we enter the staged mode.
static volatile uint32_t g_calibPhase = 0;       // OFF ticks remaining (0 = idle)
static volatile uint32_t g_calibThenMode = 0;    // MODE_REC / MODE_PLAY to enter after
static constexpr uint32_t CALIB_TICKS = 8;

// Splice gate: 1 only between a SUCCESSFUL CMD_ARM_CONTINUE and its splice
// (or any stop/re-arm). The PLAY handler's splice check requires this flag,
// so a `continue_from_frame` that appears in shared memory by any other
// route (UI bug, stray writer, stale value) can NEVER convert a plain
// replay into REC. Deliberately a cave2-private static, not a shared-state
// field — external processes must not be able to set it.
static volatile uint32_t g_cave2_contArmed = 0;

// Handle command transitions
// WARNING: NO Log/format/float calls — runs inside SafetyHookMid (x87 FPU not saved).
static void ProcessCommand(TasSharedState* s) {
    uint32_t cmd = s->command;
    if (cmd == CMD_IDLE) return;

    // Integer zero for clearing float fields without x87 instructions
    static constexpr uint32_t ZERO_BITS = 0;

    switch (cmd) {
        case CMD_ARM_REC:
            s->recorded_count = 0;
            s->playback_pos = 0;
            s->prev_mask = 0;
            memcpy(&s->max_drift_x, &ZERO_BITS, 4);
            memcpy(&s->max_drift_z, &ZERO_BITS, 4);
            s->bb3b10_call_count = 0;
            s->handler_block_count = 0;
            s->bb3b10_block_count = 0;
            // Reset segment tracking for fresh recording
            s->segment_index = 0;
            s->segment_count = 1;
            s->segment_start_frame = 0;
            memset(s->segment_boundaries, 0, sizeof(s->segment_boundaries));
            s->segment_boundaries[0].frame = 0;
            s->segment_boundaries[0].input_log_offset = 0;
            // Calibrate arg4 in OFF first, THEN flip to REC (see g_calibPhase).
            s->mode = MODE_OFF;
            g_calibThenMode = MODE_REC;
            g_calibPhase = CALIB_TICKS;
            g_cave2_contArmed = 0;
            g_diagInjectLogged = 0;
            LogRootDiag(s, "arm-rec");
            break;

        case CMD_ARM_PLAY:
            g_cave2_logParam = s->recorded_count;
            s->playback_pos = 0;
            s->prev_mask = 0;
            memcpy(&s->max_drift_x, &ZERO_BITS, 4);
            memcpy(&s->max_drift_z, &ZERO_BITS, 4);
            s->bb3b10_call_count = 0;
            s->handler_block_count = 0;
            s->bb3b10_block_count = 0;

            // Clear any leftover CONT splice marker. The PLAY handler treats
            // any non-zero `continue_from_frame` as a splice point and
            // auto-switches PLAY→REC at that frame; if a prior CONT was
            // refused or stopped before the splice fired, the field stays
            // non-zero in shared memory. Without this explicit reset, a
            // subsequent plain PLAY silently hijacks itself into a CONT at
            // the stale frame and truncates/overwrites the recording.
            s->continue_from_frame = 0;
            g_cave2_contArmed = 0;
            LogRootDiag(s, "arm-play");

            // No position forcing — F5 matching must happen naturally.
            // Position forcing (even velocity-preserving) creates physics state
            // inconsistency: position says rc0 but terrain/rotation/angular state
            // is from wherever F5 actually spawned. This causes drift with steering.

            s->mode = MODE_PLAY;
            g_cave2_pendingLog = 2;
            break;

        case CMD_ARM_CONTINUE:
            // Continue Record: PLAY frames 0..continue_from_frame, then auto-switch to REC
            // Validate splice point is within recorded range
            if (s->continue_from_frame == 0 || s->continue_from_frame > s->recorded_count) {
                LogRing(s, LOG_ERROR, "ARM_CONTINUE: invalid splice point");
                s->mode = MODE_OFF;
                s->continue_from_frame = 0;  // refused — don't leave a stale marker armed
                g_cave2_contArmed = 0;
                g_cave2_pendingLog = 3;  // "stopped"
                break;
            }
            // Refuse ARM_CONTINUE if we're already in REC/PLAY: the prefix
            // playback assumes the snowboarder is at rec_coords[0]'s state,
            // which is only true right after an F5 restart. Firing CONT
            // mid-run injects the recorded inputs against whatever state
            // the player happens to be in, producing drift or worse. The
            // tas_ui CONT button uses RestartThen(ArmContinue), which goes
            // through CMD_RESTART first — that's the only safe entry. Any
            // direct CMD_ARM_CONTINUE from mid-run (e.g. a segments-panel
            // "redo from frame" without an explicit restart) lands here and
            // gets bounced.
            if (s->mode != MODE_OFF) {
                LogRing(s, LOG_ERROR,
                    "ARM_CONTINUE: refused — game is REC/PLAY; CONT requires a fresh restart first");
                s->mode = MODE_OFF;
                s->continue_from_frame = 0;  // refused — don't leave a stale marker armed
                g_cave2_contArmed = 0;
                g_cave2_pendingLog = 3;  // "stopped"
                break;
            }
            g_cave2_logParam = s->continue_from_frame;
            s->playback_pos = 0;
            s->prev_mask = 0;
            memcpy(&s->max_drift_x, &ZERO_BITS, 4);
            memcpy(&s->max_drift_z, &ZERO_BITS, 4);
            s->bb3b10_call_count = 0;
            s->handler_block_count = 0;
            s->bb3b10_block_count = 0;
            // Segment tracking: keep existing segment_count, we'll add one at splice
            s->mode = MODE_PLAY;  // Start as PLAY, will auto-switch in PLAY handler
            g_cave2_contArmed = 1;  // the ONLY place the splice gate opens
            g_cave2_pendingLog = 5;
            break;

        case CMD_STOP:
            s->mode = MODE_OFF;
            s->cave2_injecting = 0;
            // Defense-in-depth: a CONT that was stopped before its splice fired
            // must not leave a live splice marker behind. Every armer re-writes
            // the marker immediately before CMD_ARM_CONTINUE, so clearing here
            // can't break a legitimate cycle.
            s->continue_from_frame = 0;
            g_cave2_contArmed = 0;
            g_cave2_pendingLog = 3;
            break;

        case CMD_RESTART:
            // Begin in-process F5 restart sequence
            s->restart_state = 1;
            s->restart_frames_held = 0;
            // Clock-phase pin: restart the canonical [1,1,0] tick cycle here so
            // every in-process restart replays the same settle schedule (the
            // F5 bucket). See cave5's pin block.
            s->clock_pin_phase = 0;
            g_diagRootAtRestartCmd = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
            LogRootDiag(s, "restart-cmd");
            g_cave2_pendingLog = 7;  // "restart initiated"
            break;

        case CMD_SNAPSHOT: {
            // PROTOTYPE: capture writable memory at this frame boundary.
            // bytes -> snapshot_size, microseconds -> snapshot_flags (reused).
            uint64_t us = 0;
            uint32_t bytes = SnapshotCapture(s, &us);
            s->snapshot_flags = (uint32_t)us;
            // Record the player coords (raw bits) at the snapshot instant for the
            // frame-exact rewind proof at restore time.
            g_snapPlayerValid = SnapReadPlayerBits(s, g_snapPlayerBits);
            SnapTrajStart(1); // record traj A for the next N frames
            g_cave2_logParam = bytes;
            g_cave2_pendingLog = 9;  // "snapshot captured"
            break;
        }

        case CMD_SNAPSHOT_AT_SPAWN:
            // Arm a capture at the next PLAY frame-0 — see the MODE_PLAY handler.
            g_snapAtSpawn = true;
            break;

        case CMD_RESTORE: {
            // PROTOTYPE: restore the last snapshot (instant rewind) + clock reset.
            uint64_t us = 0;
            uint32_t bytes = SnapshotRestore(s, &us);
            // Frame-exact proof: re-read the player coords NOW (same hook call,
            // zero frames advanced) and compare bit-for-bit to the snapshot.
            uint32_t match = SnapshotRevertMatch(s); // 0..3, or 0xFF if no ptr
            s->snapshot_size = bytes;
            s->snapshot_flags = (uint32_t)us;
            s->snapshot_buffer_capacity = match; // probe reads: 3 = bit-exact revert
            // Region accounting for the long-window diagnostic: high16=skipped,
            // low16=faulted (regions that changed shape since the snapshot).
            s->snapshot_buffer_ptr = (g_snapLastSkipped << 16) | (g_snapLastFaulted & 0xFFFF);
            SnapTrajStart(2); // record traj B for the next N frames, then compare
            g_cave2_logParam = match;
            g_cave2_pendingLog = 10; // "snapshot restored"
            break;
        }
    }

    s->command = CMD_IDLE;
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
        case 9: Log(std::format("Cave 2: [snap] captured {} bytes", param)); break;
        case 10: Log(std::format("Cave 2: [snap] restored; player revert match {}/3 (3=bit-exact)", param)); break;
    }
}

// Cave 2 callback logic — called with FPU state saved/restored.
// Separated from the FSAVE wrapper because MSVC forbids __asm in functions with SEH.
static void __declspec(noinline) Cave2_Logic() {
    auto* s = g_cave2State;
    auto* addr = g_cave2Addr;
    if (!s || !addr) return;

    s->frame_count++;

    // Game-state awareness: publish exe+0x8895C (0=menu, 1=in-game) so the UI
    // knows the state. Integer read — FPU-safe.
    if (addr->is_in_game) {
        s->game_in_game = *(volatile uint32_t*)addr->is_in_game;
    }

    if (s->replay_ptr) {
        uint32_t playerPtr = SafeReadPtr(s->replay_ptr + GameAddresses::REPLAY_PLAYER_OFFSET);
        s->player_ptr = playerPtr;
    }

    // Always update live position (for F5 stabilization position matching).
    // Inside FSAVE/FRSTOR so float ops are safe here.
    if (s->player_ptr) {
        __try {
            auto player = (uint8_t*)s->player_ptr;
            float new_x, new_y, new_z;
            memcpy(&new_x, player + GameAddresses::PLAYER_X, 4);
            memcpy(&new_y, player + GameAddresses::PLAYER_Y, 4);
            memcpy(&new_z, player + GameAddresses::PLAYER_Z, 4);

            // Compute velocity (current - previous)
            s->velocity_x = new_x - s->player_x;
            s->velocity_y = new_y - s->player_y;
            s->velocity_z = new_z - s->player_z;
            float vxz2 = s->velocity_x * s->velocity_x + s->velocity_z * s->velocity_z;
            // Approximate sqrt via integer hack (no x87 fsqrt needed)
            // Just store squared speed; UI can sqrt if needed
            s->speed = vxz2;

            // Update previous position
            s->prev_player_x = s->player_x;
            s->prev_player_y = s->player_y;
            s->prev_player_z = s->player_z;

            // Update current position
            s->player_x = new_x;
            s->player_y = new_y;
            s->player_z = new_z;

            // Update rotation matrix from physics sub-object
            uint32_t physics = 0;
            memcpy(&physics, player + GameAddresses::PLAYER_PHYSICS, 4);
            if (physics) {
                memcpy(s->rotation_matrix, (uint8_t*)physics + GameAddresses::PHYSICS_ROT, 36);
            }
        } __except(EXCEPTION_EXECUTE_HANDLER) {}
    }

    ProcessCommand(s);

    // PROTOTYPE: record the post-snapshot / post-restore player trajectory for
    // the frame-exact determinism check (no-op unless a snapshot/restore armed it).
    SnapTrajTick(s);

    // In-process F5 restart state machine (runs regardless of mode)
    if (s->restart_state == 1) {
        uint32_t kbobj = GetKeyboardObject(addr);
        if (kbobj) {
            if (s->restart_frames_held == 0) {
                // First frame: press F5
                InjectF5(s, addr, kbobj, true);
            }
            s->restart_frames_held++;
            if (s->restart_frames_held >= RESTART_F5_HOLD_FRAMES) {
                // Release F5 after holding long enough
                InjectF5(s, addr, kbobj, false);
                s->restart_state = 2;  // Done
                LogRootDiag(s, "f5-released");
                g_cave2_pendingLog = 8;
            }
        }
    }

    // Auto-calibration pulse (OFF, post-arm): force the game's real keyDown
    // handler to fire so cave1c calibrates the dynamic arg4 to the current race
    // context BEFORE we record/replay. Toggle a throwaway key in the DI buffer;
    // the game dispatches +3940 on the edge → cave1c (OFF, not injecting) reads
    // the live a3. The boarder can't move during the post-restart countdown, so
    // this is invisible. When the window closes, clear the key and enter the
    // staged mode (the recording/replay starts clean here, AFTER calibration).
    if (g_calibPhase > 0) {
        uint32_t ckb = GetKeyboardObject(addr);
        if (ckb) {
            uint32_t cbuf = GetDIBuffer(ckb);
            uint8_t cmask = (g_calibPhase & 1) ? (uint8_t)INPUT_LEFT : (uint8_t)0;
            WriteDIBuffer(cbuf, cmask);
        }
        g_calibPhase--;
        if (g_calibPhase == 0) {
            uint32_t ckb2 = GetKeyboardObject(addr);
            if (ckb2) WriteDIBuffer(GetDIBuffer(ckb2), 0);  // clear the calib key
            s->prev_mask = 0;
            s->mode = g_calibThenMode;
            g_arg4Recalibrated = 1;  // re-assert held input on the first real tick
            g_cave2_pendingLog = (g_calibThenMode == MODE_REC) ? 1 : 5;
            g_calibThenMode = 0;
        }
        return;  // hold off normal REC/PLAY processing until calibrated
    }

    if (s->mode == MODE_OFF) return;

    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;

    if (s->mode == MODE_REC) {
        uint32_t index = s->recorded_count;
        if (index >= TAS_MAX_TICKS) {
            s->mode = MODE_OFF;
            return;
        }

        // Sample input via GAKS (Cave 1C blocks +3940, so game buffer is empty).
        // This makes REC symmetric with PLAY: both write buffer + action_state
        // + BB3B10 at the same point in Supreme::Cycle.
        uint8_t mask = SampleGAKS();
        uint8_t transitions = mask ^ (uint8_t)s->prev_mask;

        // arg4 just got (re)calibrated from a real keypress: re-assert every
        // currently-held key so a first-press dropped under the stale value
        // lands now with the correct arg4 (kills the "dead until warmup" race).
        if (g_arg4Recalibrated) {
            transitions |= mask;
            g_arg4Recalibrated = 0;
        }

        uint32_t buffer = GetDIBuffer(kbobj);
        WriteDIBuffer(buffer, mask);

        WriteActionState(kbobj, mask);

        if (s->inject_mode == 6 && transitions) {
            CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
        }

        s->input_log[index] = mask;
        s->prev_mask = mask;

        CapturePlayerCoords(s, index, true);

        s->recorded_count = index + 1;

        // RDIAG: periodic root/kbobj probe during REC — shows whether the
        // chain the input writes go through still matches the live session
        // (the steering-dies-after-REC investigation).
        if ((index & 0xFF) == 0) {
            LogRootDiag(s, "rec");
        }

    } else if (s->mode == MODE_PLAY) {
        uint32_t pos = s->playback_pos;

        // CONT replay starting (first replay tick): stamp frame_count so the
        // harness can measure how many game-frames the catch-up replay takes to
        // reach the splice (the "resume off by a few frames" skew).
        if (pos == 0 && s->continue_from_frame > 0) {
            s->cont_replay_start_fc = s->frame_count;
        }

        // PROTOTYPE: snapshot the exact spawn at frame-0 of the replay (armed via
        // CMD_SNAPSHOT_AT_SPAWN). Captured BEFORE this frame's input is injected,
        // so it's the pure spawn the replay starts from — restoring it later
        // reproduces this bucket bit-exactly (no F5 lottery). Done here (not from
        // the harness) so the arm timing that picks the bucket isn't disturbed by
        // the ~200ms capture.
        if (pos == 0 && g_snapAtSpawn) {
            uint64_t us = 0;
            uint32_t bytes = SnapshotCapture(s, &us);
            g_snapPlayerValid = SnapReadPlayerBits(s, g_snapPlayerBits);
            s->snapshot_size = bytes;
            s->snapshot_flags = (uint32_t)us;
            g_snapAtSpawn = false;
        }

        if (pos >= s->recorded_count) {
            s->mode = MODE_OFF;
            g_cave2_contArmed = 0;  // hygiene — an armed CONT always splices before here
            g_cave2_logParam = pos;
            g_cave2_pendingLog = 4;
            return;
        }

        uint8_t mask = s->input_log[pos];

        uint32_t buffer = GetDIBuffer(kbobj);
        WriteDIBuffer(buffer, mask);

        WriteActionState(kbobj, mask);

        uint8_t transitions = mask ^ (uint8_t)s->prev_mask;
        // Re-assert held keys when arg4 was just (re)calibrated — same
        // first-press race fix as the REC path (replay/CONT steering dropped
        // under a stale arg4 = the deterministic "goes straight at first input"
        // divergence).
        if (g_arg4Recalibrated) {
            transitions |= mask;
            g_arg4Recalibrated = 0;
        }
        if (s->force_direct == 2 && transitions) {
            CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
        }

        s->prev_mask = mask;

        CapturePlayerCoords(s, pos, false);

        s->playback_pos = pos + 1;

        // Continue Record: auto-switch to REC AFTER processing the splice frame.
        // This ensures the splice frame gets normal PLAY processing (input injection
        // + coordinate capture), maintaining symmetry with the final PLAY phase.
        // Gated on g_cave2_contArmed: only a PLAY entered via CMD_ARM_CONTINUE may
        // splice. A marker that lands in shared memory by any other route (stray
        // writer mid-replay, stale value, UI setting continue_from during PLAY)
        // must never hijack a plain replay into REC.
        if (g_cave2_contArmed && s->continue_from_frame > 0
                && s->playback_pos >= s->continue_from_frame) {
            uint32_t splice_pos = s->playback_pos;
            s->recorded_count = splice_pos;

            // Stamp the splice instant. (cont_splice_fc - cont_replay_start_fc)
            // is the game-frames the replay took to reach the splice — the
            // diagnostic for "resume yields a few frames early/late".
            s->cont_splice_fc = s->frame_count;

            // Problem B fix: drop to the user's resume speed ATOMICALLY here, at
            // the exact splice tick. Otherwise the recording keeps fast-
            // forwarding at the catch-up rate (e.g. 64x) for the whole window
            // until the UI polls, sees PLAY->REC, and restores the speed — a
            // variable post-splice overshoot. Cave5 picks this up next tick.
            if (s->cont_resume_speed > 0.0f) {
                s->playback_speed = s->cont_resume_speed;
            }
            // Signal cave5 to clear the catch-up clock backlog on its next tick
            // (advance the game-time accumulator to "now" without processing the
            // backlog ticks) so the resume is frame-exact at full speed — no
            // end-of-replay deceleration needed.
            s->cont_reset_pending = 1;

            // Record new segment boundary
            uint32_t segIdx = s->segment_count;
            if (segIdx < TAS_MAX_SEGMENTS) {
                s->segment_boundaries[segIdx].frame = splice_pos;
                s->segment_boundaries[segIdx].input_log_offset = splice_pos;
                s->segment_count = segIdx + 1;
                s->segment_index = segIdx;
            }
            s->segment_start_frame = splice_pos;

            s->mode = MODE_REC;
            s->continue_from_frame = 0;  // Clear splice marker
            g_cave2_contArmed = 0;       // splice consumed — close the gate
            g_cave2_logParam = splice_pos;
            g_cave2_pendingLog = 6;
        }
    }
}

// SafetyHookMid callback with explicit x87 FPU preservation.
// FSAVE saves all 8 ST registers + control/status (108 bytes) and reinits FPU.
// FRSTOR restores everything before returning to game code.
static void Cave2_MidCallback(SafetyHookContext& ctx) {
    uint64_t t0 = __rdtsc();
    uint8_t fpu_buf[108];
    __asm { fsave [fpu_buf] }
    Cave2_Logic();
    __asm { frstor [fpu_buf] }
    auto* s = g_cave2State;
    if (s) {
        PerfSample(s->perf_cave2, __rdtsc() - t0);
    }
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
        return false;
    }

    state->cave2_hooked = 1;
    Log("Cave 2: hook installed successfully");
    return true;
}
