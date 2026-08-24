#pragma once
#include "../stdafx.h"
#include <atomic>
#include "../log.hpp"
#include "../helper.hpp"
#include "../gate_alignment.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"
#include "snapshot.hpp"
#include "race_timer.hpp"   // racetimer::ReadClk() — the GAME's own clock

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

// BB3B10's 3rd+4th arguments are the {lo, hi} dwords of the 64-bit
// Kernel::Time the event was stamped with at message-pump dispatch (RE'd from
// the exported Win32_Driver::Translate(tagMSG&, Kernel::Time) chain: +3940
// forwards its Time args verbatim to BB3B10). The observer silently DISCARDS
// events whose Time predates the current race context — the "dynamic arg4"
// that made stale injection a silent no-op (dead REC steering, straight-line
// replays). The primary fix is stamping injections with the game's own
// Kernel::Time::Current() (see CallBB3B10OnTransitions); this calibrated
// fallback value (Time.hi observed from real keypresses via cave1c/cave1d)
// only matters if the Kernel export ever fails to resolve.
inline volatile uint32_t g_bb3b10Arg4 = GameAddresses::BB3B10_ARG4;

// RDIAG: one-shot guard so we log only the FIRST injection's arg4 per arm.
// Reset at ARM_REC / ARM_PLAY (see ProcessCommand).
inline volatile uint32_t g_diagInjectLogged = 0;

// GetTickCount() stamped on every Supreme::Cycle tick. The message-pump-driven
// hooks (cave1c) use it to detect "the game is PAUSED / at a non-ticking
// screen" (pause menu, dialogs, static main menu): when the cycle hasn't
// ticked recently, the input gate passes ALL keys through so the user can
// operate the pause menu / dialogs even while a TAS mode is armed.
inline volatile uint32_t g_lastCycleMs = 0;

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
        // event-time gate (hi is what the gate checks — proven live), but a
        // FLOORED stamp is DETERMINISTIC for every event in the same ~7-min
        // hi-window, where the live lo differs between a REC and its replay.
        // The bit-exact zero-drift era injected a constant stamp; keeping the
        // stamp constant-per-window preserves that determinism in case the
        // game uses the stamp beyond the gate (sub-tick input timing).
        t.lo = 0;
        s->arg4_source = ARG4_SOURCE_TIME_CURRENT;
    } else {
        t.lo = 0;
        t.hi = g_bb3b10Arg4;
        s->arg4_source = ARG4_SOURCE_CALIBRATED;
    }

    // RDIAG: log the Time the FIRST injection uses per arm — shows whether the
    // injection is stamped live (source=1) or fell back (source=2).
    if (!g_diagInjectLogged) {
        g_diagInjectLogged = 1;
        char buf[80]; int p = 0;
        auto put = [&](const char* t2){ while (*t2 && p < 64) buf[p++] = *t2++; };
        put("RDIAG inject m="); DiagHexU32(buf + p, s->mode); p += 8;
        put(" hi="); DiagHexU32(buf + p, t.hi); p += 8;
        put(" lo="); DiagHexU32(buf + p, t.lo); p += 8;
        put(" src="); DiagHexU32(buf + p, s->arg4_source); p += 8; buf[p] = '\0';
        LogRing(s, LOG_INFO, buf);
    }

    // Set cave2_injecting so Cave 1C/1D pass through
    s->cave2_injecting = 1;

    if (transitions & INPUT_LEFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_LEFT, (mask & INPUT_LEFT) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_RIGHT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_RIGHT, (mask & INPUT_RIGHT) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_UP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_UP, (mask & INPUT_UP) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_DOWN) {
        bb3b10(thisPtr, GameAddresses::BB3B10_DOWN, (mask & INPUT_DOWN) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_JUMP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_JUMP, (mask & INPUT_JUMP) ? 1 : 0,
               t.lo, t.hi);
    }
    if (transitions & INPUT_SHIFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_SHIFT, (mask & INPUT_SHIFT) ? 1 : 0,
               t.lo, t.hi);
    }

    s->cave2_injecting = 0;
    s->bb3b10_call_count++;
}

// Helper: capture player coordinates (SEH-protected, NO FLOAT OPS)
// Uses integer-width memcpy to avoid corrupting x87 FPU state.
// Drift computation is deferred to Rust test harness post-playback.
// Snapshot the player object and its physics sub-object, raw, dword by dword
// under SEH. Returns how many dwords of each were readable. Integer copies
// only — no float ops, this runs inside the cave.
static void SnapshotPlayerObjects(TasSharedState* s,
                                  volatile uint32_t* player_out,
                                  volatile uint32_t* physics_out) {
    s->objsnap_player_ok = 0;
    s->objsnap_physics_ok = 0;
    if (!s->player_ptr) return;
    auto player = (const uint32_t*)s->player_ptr;
    uint32_t n = 0;
    __try {
        for (; n < OBJSNAP_PLAYER_DWORDS; ++n) player_out[n] = player[n];
    } __except(EXCEPTION_EXECUTE_HANDLER) {}
    s->objsnap_player_ok = n;
    uint32_t physics = 0;
    __try {
        memcpy(&physics, (const uint8_t*)s->player_ptr + GameAddresses::PLAYER_PHYSICS, 4);
    } __except(EXCEPTION_EXECUTE_HANDLER) { physics = 0; }
    if (!physics) return;
    auto phys = (const uint32_t*)(uintptr_t)physics;
    n = 0;
    __try {
        for (; n < OBJSNAP_PHYSICS_DWORDS; ++n) physics_out[n] = phys[n];
    } __except(EXCEPTION_EXECUTE_HANDLER) {}
    s->objsnap_physics_ok = n;
}

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
        s->capture_ok = 0;
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

        // Stamp the countdown gate: the first frame of this session whose
        // position differs from frame 0. Stamped HERE rather than derived from
        // the coordinate array afterwards because the TICK it happened on is
        // what the prediction model needs, and that is only available now.
        // Integer compare on the raw bits - no float ops inside the hook.
        // capture_ok also means frame 0 of THIS session was really written;
        // without it the comparison below is against the previous session's
        // frame 0 and can stamp a gate that never happened.
        if (s->capture_ok && s->gate_tick == 0 && index > 0) {
            const uint32_t* z = isRec ? (const uint32_t*)&s->rec_coords[0][0]
                                      : (const uint32_t*)&s->play_coords[0][0];
            if (raw[0] != z[0] || raw[1] != z[1] || raw[2] != z[2]) {
                s->gate_tick = s->tick_count;
                s->gate_index = index;
                s->gate_clk = racetimer::ReadClk();
                s->gate_reset_tick = s->reset_tick;
                s->gate_seq = s->frame_count;
                SnapshotPlayerObjects(s, s->objsnap_gate_player, s->objsnap_gate_physics);
                s->gate_qpc_lo = s->clock_delta_lo;
                s->gate_qpc_hi = s->clock_delta_hi;
                s->gate_secs_lo = s->secs_since_reset_lo;
                s->gate_secs_hi = s->secs_since_reset_hi;
            }
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
        // Stamp F5 with the live Kernel::Time too. The restart itself is
        // driven by the DI-buffer write (the observer call is auxiliary), but
        // a wrong stamp here POISONS the observer's event-time window: the
        // old 0x96 constant is hours-of-uptime in the future after a reboot,
        // and steering events stamped with the (smaller) true current time
        // then look out-of-order and get silently dropped — dead steering
        // right after every injected F5 (and the June-9 "environment cliff":
        // a reboot turned the constant into a future stamp).
        KernelTime ft = { 0, GameAddresses::BB3B10_ARG4 };
        GetKernelTimeNow(addr, &ft);
        ft.lo = 0;  // floored like the steering stamp (see CallBB3B10OnTransitions)
        bb3b10(thisPtr, GameAddresses::BB3B10_F5, pressed ? 1 : 0,
               ft.lo, ft.hi);
        s->cave2_injecting = 0;
    }
}

// Release every input the TAS session injected: zero the DI buffer + action
// states and notify the observer that held keys went UP (live Time stamp).
// MUST run on every TAS→OFF transition — without it a key held by the replay
// at STOP stays "down" in the game's input state and the boarder keeps
// steering until the user taps the key (the stuck-input class). Resolves the
// kbobj fresh, so after an auto-stop (level swapped) it clears the NEW
// level's state — release events for keys the new observer never saw pressed
// are no-ops, same as a real keyUp without a down.
static void ReleaseTasInput(TasSharedState* s, GameAddresses* addr) {
    uint8_t held = (uint8_t)s->prev_mask;
    s->prev_mask = 0;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;
    WriteDIBuffer(GetDIBuffer(kbobj), 0);
    WriteActionState(kbobj, 0);
    if (held) {
        CallBB3B10OnTransitions(s, addr, kbobj, 0, held);
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

// A judged PLAY armed this attempt's speed handover. Mirrors g_cave2_contArmed:
// the marker in shared memory says WHERE to hand over, this says the current
// attempt is the one that asked for it. Without it a marker left behind by a
// killed writer, or inherited across an arm, could apply a stale PLAY resume
// speed and clock reset partway through somebody else's replay.
static volatile uint32_t g_cave2_handoffArmed = 0;

// Drop any staged PLAY speed handover. Called from every path that ends or
// invalidates a replay - stop, refusal, restart, playback completion, the
// level-swap auto-stop. A marker that outlives its replay would fire against
// whatever runs next, at a position that means nothing there.
// Drop any gate-relative input alignment. Called from every path that is not
// an aligned PLAY: the field is persistent shared memory, and a value left
// behind by an earlier replay would silently re-index a later one. CONT is the
// dangerous case — it runs the same PLAY handler for its prefix but splices at
// an ARM-relative continue_from_frame, so an inherited alignment would shift
// the input stream while leaving the splice point where it was.
static inline void ClearGateAlign(TasSharedState* s) {
    s->gate_align_rec = 0;
}

static inline void ClearSpeedHandoff(TasSharedState* s) {
    s->speed_handoff_pos = 0;
    g_cave2_handoffArmed = 0;
}
static volatile uint32_t g_cave2_pendingLog = 0;  // 0=none, 1=REC, 2=PLAY, 3=STOP, 4=playback_done
static volatile uint32_t g_cave2_logParam = 0;

// Root player pointer ([SG+1D5450]) captured at ARM time. The root object is
// STABLE across in-process F5 restarts (RDIAG-proven: same pointer through
// restart-cmd/f5-released) but is reallocated when the level itself is torn
// down — quitting to the menu, the menu's attract demo loading a level, or
// switching tracks. A TAS mode left armed across that boundary then drives
// the WRONG context: REC records the menu demo, the scaled playback_speed
// fast-forwards the menu video, and the cave1c gate eats all native keys.
// Cave2 auto-stops the session when the live root no longer matches.
inline volatile uint32_t g_armedRoot = 0;

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

    // Deterministic arm scheduling. If the caller asked for a specific tick,
    // leave the command PENDING until the counter reaches it — do not consume,
    // do not clear. The command slot is a single u32 and nothing else writes it
    // while an arm is outstanding, so holding it is safe.
    //
    // This exists because the arm consumption tick was never controlled: an
    // external send is a memory store, and consumption happens on whichever
    // Supreme::Cycle comes next. first_moving counts from consumption.
    if (s->arm_at_tick != 0 &&
        (cmd == CMD_ARM_REC || cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE)) {
        // Unsigned wrap-safe "have we reached it yet".
        if ((s->tick_count - s->arm_at_tick) >= 0x80000000u) {
            return;
        }
        s->arm_consumed_tick = s->tick_count;
        s->arm_at_tick = 0;
    }
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
            s->mode = MODE_REC;
            g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
            g_cave2_contArmed = 0;
            // REC never hands over a speed, and never replays; make sure it
            // cannot inherit either.
            ClearSpeedHandoff(s);
            ClearGateAlign(s);
            g_diagInjectLogged = 0;
            g_cave2_pendingLog = 1;
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
            // Claim this attempt's speed handover, for the same reason the
            // splice marker above is gated: the marker alone says only WHERE to
            // hand over, not that the replay about to start is the one that
            // asked. Every armer stages it immediately before this command, so
            // a marker present now belongs to this attempt; one that arrives by
            // any other route never opens the gate.
            g_cave2_handoffArmed = (s->speed_handoff_pos != 0) ? 1 : 0;
            LogRootDiag(s, "arm-play");

            // No position forcing — F5 matching must happen naturally.
            // Position forcing (even velocity-preserving) creates physics state
            // inconsistency: position says rc0 but terrain/rotation/angular state
            // is from wherever F5 actually spawned. This causes drift with steering.

            s->mode = MODE_PLAY;
            g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
            g_diagInjectLogged = 0;
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
                ClearSpeedHandoff(s);
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
                ClearSpeedHandoff(s);
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
            g_armedRoot = SafeReadPtr((uint32_t)g_cave2Addr->player_base);
            g_cave2_contArmed = 1;  // the ONLY place the splice gate opens
            // CONT hands over at its splice (cont_resume_speed), never
            // mid-replay. Refuse any speed-handover marker it might have
            // inherited. Gate alignment, however, is now SUPPORTED for CONT:
            // the splice fires at the aligned index while the recording stays
            // in rec-index space (see the splice block). The controller stages
            // gate_align_rec immediately before this arm, so keep it — only a
            // value from any OTHER route is stale, and STOP/RESTART/ARM_REC
            // clear those.
            ClearSpeedHandoff(s);
            g_cave2_pendingLog = 5;
            break;

        case CMD_STOP:
            s->mode = MODE_OFF;
            s->cave2_injecting = 0;
            ReleaseTasInput(s, g_cave2Addr);  // un-stick keys held by the session
            // Defense-in-depth: a CONT that was stopped before its splice fired
            // must not leave a live splice marker behind. Every armer re-writes
            // the marker immediately before CMD_ARM_CONTINUE, so clearing here
            // can't break a legitimate cycle.
            s->continue_from_frame = 0;
            g_cave2_contArmed = 0;
            // Same for a PLAY speed handover. Without this, a judged PLAY that
            // is stopped before it reaches first_moving leaves the request armed
            // - and the NEXT replay, which may be a plain PLAY the user set to
            // 4x on purpose, gets silently dropped to the stale resume speed
            // when its position happens to pass that number. Every armer stages
            // its own handover immediately before the arm command, so clearing
            // here cannot break a legitimate cycle.
            ClearSpeedHandoff(s);
            ClearGateAlign(s);
            memcpy((void*)&s->speed_after_handoff, &ZERO_BITS, 4);
            g_cave2_pendingLog = 3;
            break;

        case CMD_RESTART:
            // Begin in-process F5 restart sequence
            s->trace_count = 0;   // trace from HERE, so the reset is inside it
            s->restart_state = 1;
            s->restart_frames_held = 0;
            // The replay a handover was staged for is about to stop existing.
            // Every armer stages its own AFTER its restart, so clearing here
            // cannot break a controller cycle - it only stops a bare F5 from
            // leaving one armed for whatever replays next. Same for gate
            // alignment: CONT keeps only the value the controller stages AFTER
            // this restart, so a value left by a prior aligned PLAY that ended
            // in OFF cannot leak into a legacy ARM_CONTINUE.
            ClearSpeedHandoff(s);
            ClearGateAlign(s);
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

    // Durable "the arm landed" signal for the judge, published AFTER the switch.
    //
    // Two reasons it lives here and not at the top of each arm case. It must be
    // the LAST store of the arm, so an observer that sees the counter move is
    // guaranteed to see the mode and position the arm wrote (x86 keeps store
    // order, so the reverse - bumping first - leaves a window reading
    // "armed, but mode still OFF and position still 0", which is
    // indistinguishable from a refused arm). And it must count REFUSED arms
    // too, which take an early `break`: a refusal leaves the mode OFF forever,
    // and that is exactly the state the judge would otherwise spin on.
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE || cmd == CMD_ARM_REC) {
        // Stamp the arm tick for EVERY arm, not just a deferred one. The
        // deferral path above returns early while it is still waiting, so
        // reaching here always means the command was really consumed, now.
        s->arm_consumed_tick = s->tick_count;
        // Latch the restart this arm belongs to, so a later CMD_RESTART from
        // anywhere cannot repair the pair into a different attempt's.
        s->arm_restart_tick = s->restart_done_tick;
        s->arm_clk = racetimer::ReadClk();
        s->arm_reset_tick = s->reset_tick;
        s->arm_seq = s->frame_count;
        SnapshotPlayerObjects(s, s->objsnap_arm_player, s->objsnap_arm_physics);
        s->arm_qpc_lo = s->clock_delta_lo;
        s->arm_qpc_hi = s->clock_delta_hi;
        s->arm_secs_lo = s->secs_since_reset_lo;
        s->arm_secs_hi = s->secs_since_reset_hi;
        // Integer copies of the live position — no float ops, and the raw bits
        // are what a settle-frame comparison wants anyway.
        memcpy((void*)&s->arm_pos_x, (const void*)&s->player_x, 4);
        memcpy((void*)&s->arm_pos_y, (const void*)&s->player_y, 4);
        memcpy((void*)&s->arm_pos_z, (const void*)&s->player_z, 4);
        // Fresh session: the gate has not fired yet, and no capture has failed.
        s->gate_tick = 0;
        s->gate_index = 0;
        s->capture_ok = 1;
    }
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE) {
        s->arm_generation++;
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
        case 11: Log(std::format("Cave 2: PLAY speed handover at frame {}", param)); break;
    }
}

// Cave 2 callback logic — called with FPU state saved/restored.
// Separated from the FSAVE wrapper because MSVC forbids __asm in functions with SEH.
static void __declspec(noinline) Cave2_Logic() {
    auto* s = g_cave2State;
    auto* addr = g_cave2Addr;
    if (!s || !addr) return;

    s->frame_count++;
    g_lastCycleMs = GetTickCount();  // pause detector heartbeat (see cave1c)

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

            // Level-reset detector: the most recent tick on which the position
            // changed AT ALL.
            //
            // No distance threshold, because none is needed and the obvious one
            // is wrong. A first attempt looked for a big teleport back to spawn,
            // which never fired in a countdown-only cycle — the boarder had never
            // left the spawn area, so the "teleport" was a fraction of a unit.
            //
            // The real signal is much simpler: during the countdown the boarder
            // is BIT-IDENTICALLY still (measured: the position captured at the
            // arm is the same 36/36 across every arm offset). So the last frame
            // the position moved, read any time during the countdown, IS the
            // frame the level reset. Raw-bit compare, no float ops.
            // ...and only while OFF. The gate is a position change too, so a
            // detector that watches every mode re-stamps itself at the very
            // moment it is meant to be measuring the distance to — which is
            // exactly what made every QPC reading come out as zero. The reset
            // happens in OFF; the gate happens in REC/PLAY.
            if (s->mode == MODE_OFF) {
                uint32_t nb[3], ob[3];
                memcpy(&nb[0], &new_x, 4);
                memcpy(&nb[1], &new_y, 4);
                memcpy(&nb[2], &new_z, 4);
                memcpy(&ob[0], (const void*)&s->player_x, 4);
                memcpy(&ob[1], (const void*)&s->player_y, 4);
                memcpy(&ob[2], (const void*)&s->player_z, 4);
                if (nb[0] != ob[0] || nb[1] != ob[1] || nb[2] != ob[2]) {
                    s->reset_tick = s->tick_count;
                    // cave5 published this frame's engine clock already.
                    s->reset_qpc_lo = s->clock_delta_lo;
                    s->reset_qpc_hi = s->clock_delta_hi;
                    // Restart the seconds accumulator from the reset frame.
                    s->secs_since_reset_lo = 0;
                    s->secs_since_reset_hi = 0;
                }
            }

            // Record the frame. Runs from the F5 press until the buffer fills,
            // which covers the whole countdown — the reset, the settle and the
            // gate all land inside it.
            {
                uint32_t n = s->trace_count;
                if (n < TRACE_FRAMES) {
                    s->trace[n][0] = s->tick_count;
                    memcpy((void*)&s->trace[n][1], &new_x, 4);
                    memcpy((void*)&s->trace[n][2], &new_y, 4);
                    memcpy((void*)&s->trace[n][3], &new_z, 4);
                    s->trace[n][4] = s->clock_delta_lo;
                    s->trace[n][5] = s->clock_delta_hi;
                    {
                        uint32_t ph = 0;
                        memcpy(&ph, (uint8_t*)s->player_ptr + GameAddresses::PLAYER_PHYSICS, 4);
                        s->trace[n][6] = ph;
                    }
                    s->trace_count = n + 1;
                }
            }

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
                // First frame: press F5. THIS is when the level resets and the
                // countdown starts — the release is ten frames of our own
                // making later, and measuring from it put that hold length
                // straight into the prediction error.
                s->f5_press_tick = s->tick_count;
                s->press_seq = s->frame_count;
                s->f5_press_qpc_lo = s->clock_delta_lo;
                s->f5_press_qpc_hi = s->clock_delta_hi;
                InjectF5(s, addr, kbobj, true);
            }
            s->restart_frames_held++;
            if (s->restart_frames_held >= RESTART_F5_HOLD_FRAMES) {
                // Release F5 after holding long enough
                InjectF5(s, addr, kbobj, false);
                s->restart_done_tick = s->tick_count;
                s->restart_clk = racetimer::ReadClk();
                s->restart_state = 2;  // Done
                LogRootDiag(s, "f5-released");
                g_cave2_pendingLog = 8;
            }
        }
    }

    // NOTE: the level-context epoch is NOT bumped here. Supreme::Cycle freezes
    // at static menus, dialogs and level LOADS (frame_limit.hpp:43-47) — exactly
    // the transitions it must detect — so a cycle-driven bump would not land
    // until the new level's first tick, leaving the old track asserted for the
    // whole menu + load. levelscan's background thread polls the same root every
    // 100 ms instead (level_scan.hpp).

    if (s->mode == MODE_OFF) return;

    // Auto-stop when the LEVEL is swapped out under an armed TAS mode. The
    // root object survives F5 restarts (same pointer — RDIAG-proven) but is
    // reallocated on quit-to-menu / the menu demo loading / track switches.
    // Without this, a session left armed across that boundary records the
    // menu demo, fast-forwards the menu video at the scaled playback_speed,
    // and eats every native key via the cave1c gate. root==0 (mid-teardown)
    // is NOT a trigger — restarts pass through that transiently.
    {
        uint32_t curRoot = SafeReadPtr((uint32_t)addr->player_base);
        if (curRoot && g_armedRoot && curRoot != g_armedRoot) {
            s->mode = MODE_OFF;
            s->cave2_injecting = 0;
            ReleaseTasInput(s, addr);  // clears the NEW level's input state
            s->continue_from_frame = 0;
            g_cave2_contArmed = 0;
            ClearSpeedHandoff(s);
            ClearGateAlign(s);
            // The level was swapped out, so any judged cycle is definitionally
            // over — release the live-input block. Left set, it permanently
            // disables the menu present cap (frame_limit gates on it) and the
            // menu video runs at the wrong speed.
            s->cont_suppress_input = 0;
            g_armedRoot = 0;
            LogRing(s, LOG_WARN,
                "TAS auto-stopped: level context changed (left the race / menu demo loaded)");
            g_cave2_pendingLog = 3;  // "stopped"
            return;
        }
    }

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

        // The endpoint has to move with the input.
        //
        // Alignment made the SOURCE index gate-relative but left this test
        // arm-relative, so a replay whose gate landed 4 ticks early stopped 4
        // gate-relative ticks short: its last four source indices ran past
        // recorded_count and injected nothing, and the run ended before the
        // recording did. Invisible to a fixed-window comparison, and exactly
        // the sort of thing that changes a finish.
        uint32_t play_end = s->recorded_count;
        if (s->gate_align_rec > 0 && s->gate_index > 0
                && s->recorded_count > s->gate_align_rec) {
            play_end = s->gate_index + (s->recorded_count - s->gate_align_rec);
        }
        if (pos >= play_end) {
            s->mode = MODE_OFF;
            ClearGateAlign(s);  // aligned PLAY finished — don't leave it armed
            ReleaseTasInput(s, addr);  // replay done — un-stick its held keys
            g_cave2_contArmed = 0;  // hygiene — an armed CONT always splices before here
            ClearSpeedHandoff(s);   // ...and a handover always fires before here
            g_cave2_logParam = pos;
            g_cave2_pendingLog = 4;
            return;
        }

        // Gate-relative input alignment.
        //
        // Normally the replay applies input_log[pos] — indexed from the ARM. If
        // this replay's countdown ends on a different index than the
        // recording's did, every input then lands at the wrong offset relative
        // to the race start. This fixes that indexing failure; the transport's
        // trajectory watcher separately rejects a differing hidden spawn state.
        //
        // With gate_align_rec set, indexing is relative to the gate instead, so
        // a countdown one tick longer or shorter simply shifts where the
        // recording is read from without shifting the run's input timing.
        //
        // Before the gate, replay the recording's real input history until the
        // final safety window, then HOLD the input it had at its own gate.
        //
        // Injecting nothing there instead costs one cycle of input and shows up
        // as a constant 0.389 drift: gate_index is stamped at the END of the
        // cycle whose position first differs, so on that cycle the mask is
        // still chosen without it, and input_log[rec_gate] is never applied at
        // all. Holding it near the end fixes that exactly — the boarder cannot
        // move before the gate, so the value is inert until the moment it
        // becomes the correct one. Preserving earlier transitions still
        // matters to the input observer: moving a long-held key-down all the
        // way back to the arm produced intermittent late drift in the Pico
        // acceptance test.
        uint32_t src = pos;
        if (s->gate_align_rec > 0) {
            src = GateAlignedInputSource(pos, s->gate_index, s->gate_align_rec,
                                         s->recorded_count);
        }
        uint8_t mask = (src == GATE_ALIGN_INVALID_SOURCE) ? (uint8_t)0 : s->input_log[src];

        uint32_t buffer = GetDIBuffer(kbobj);
        WriteDIBuffer(buffer, mask);

        WriteActionState(kbobj, mask);

        uint8_t transitions = mask ^ (uint8_t)s->prev_mask;
        if (s->force_direct == 2 && transitions) {
            CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
        }

        s->prev_mask = mask;

        CapturePlayerCoords(s, pos, false);

        s->playback_pos = pos + 1;

        // Judged-PLAY speed handover, the same trick as the CONT splice below.
        //
        // A bucket-matched PLAY replays the countdown purely so the judge can see
        // where the boarder leaves the spawn; nothing before that is worth
        // watching, and at 1x it costs ~3s on the accepted run AND on every
        // reroll. So the controller replays it at catch-up speed and asks for
        // the drop back here, at the tick it names. Doing it from a polling
        // thread instead would be unbounded: cave5 can already have issued up to
        // CAVE5_PER_FRAME_TICK_CAP ticks before the poll even runs, so the run
        // would start fast-forwarded by a variable amount — precisely the
        // "Problem B" overshoot the CONT splice was changed to avoid.
        //
        // cave5 caps the batch to land on this position, so the handover is
        // exact and not up to a batch late.
        if (g_cave2_handoffArmed && s->speed_handoff_pos > 0
                && s->playback_pos >= s->speed_handoff_pos) {
            float resume = s->speed_after_handoff;
            // Release the claim BEFORE installing the speed, not after.
            //
            // The controller re-asserts a speed every step, gated on this marker.
            // Clearing last leaves a window where it reads "still handing over",
            // writes the catch-up speed, and lands AFTER the speed was installed
            // - with the marker then cleared, so nothing ever corrects it.
            // Clearing first inverts that: a reader either still sees the marker
            // (and this store lands last, winning) or sees it gone (and asserts
            // the resume speed itself).
            ClearSpeedHandoff(s);
            // playback_speed is a PLAIN float, so the volatile clears above do
            // not by themselves stop the compiler hoisting this store past
            // them - MSVC's volatile writes constrain what precedes them, not
            // what follows. The barrier is what makes "release the claim,
            // THEN install" true in the emitted code as well as the source.
            std::atomic_signal_fence(std::memory_order_seq_cst);
            if (resume > 0.0f) {
                s->playback_speed = resume;
            }
            // Clear the catch-up clock backlog on cave5's next tick, exactly as
            // the splice does, so the replay resumes frame-exact at the new speed
            // instead of burning down the accumulated fast-forward debt.
            s->cont_reset_pending = 1;
            g_cave2_logParam = s->playback_pos;
            g_cave2_pendingLog = 11;
        }

        // Continue Record: auto-switch to REC AFTER processing the splice frame.
        // This ensures the splice frame gets normal PLAY processing (input injection
        // + coordinate capture), maintaining symmetry with the final PLAY phase.
        // Gated on g_cave2_contArmed: only a PLAY entered via CMD_ARM_CONTINUE may
        // splice. A marker that lands in shared memory by any other route (stray
        // writer mid-replay, stale value, UI setting continue_from during PLAY)
        // must never hijack a plain replay into REC.
        // Gate-aligned CONT: the splice fires at the aligned PLAY index, but
        // the recording stays in rec-index space — recorded_count and the
        // segment boundary are the ORIGINAL continue_from_frame, so the saved
        // recording is byte-consistent with the one that was loaded and needs
        // no input_log rewrite. Unaligned CONT: aligned==continue_from_frame
        // and rec_splice==splice_pos, so this is byte-identical to before.
        uint32_t aligned_splice = GateAlignedSplicePos(
            s->continue_from_frame, s->gate_index, s->gate_align_rec);
        if (g_cave2_contArmed && s->continue_from_frame > 0
                && s->playback_pos >= aligned_splice) {
            uint32_t splice_pos = s->playback_pos;
            uint32_t rec_splice = s->continue_from_frame;
            s->recorded_count = rec_splice;

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
                s->segment_boundaries[segIdx].frame = rec_splice;
                s->segment_boundaries[segIdx].input_log_offset = rec_splice;
                s->segment_count = segIdx + 1;
                s->segment_index = segIdx;
            }
            s->segment_start_frame = rec_splice;

            s->mode = MODE_REC;
            s->continue_from_frame = 0;  // Clear splice marker
            g_cave2_contArmed = 0;       // splice consumed — close the gate
            g_cave2_logParam = rec_splice;
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
