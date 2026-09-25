#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../gate_alignment.hpp"
#include "../input_gate.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../renderer_info.hpp"
#include "../rider_identity.hpp"
#include "f5_restart_cave.hpp"
#include "../fpu_safe_hook.hpp"
#include "tick_cave.hpp"

// The cycle cave: Supreme::Cycle hook (SG+0x13FE40), the REC/PLAY engine.
// Fires once per physics tick. The game does fld/fmul right after the hook
// site, so the body runs inside CreateMidHook's FSAVE/FRSTOR (fpu_safe_hook.hpp).
// REC samples the keyboard and injects it; PLAY injects the logged mask. Both
// write the DI buffer, notify the observer (BB3B10) on transitions and capture
// coordinates. See DESIGN.md "The hooks".

inline TasSharedState* g_cycleCaveState = nullptr;
inline GameAddresses* g_cycleCaveAddr = nullptr;
static SafetyHookMid cycleCaveHook{};

inline void UninstallCycleCave() {
    cycleCaveHook = {};
    if (g_cycleCaveState) g_cycleCaveState->cycle_cave_hooked = 0;
    g_cycleCaveState = nullptr;
    g_cycleCaveAddr = nullptr;
}

typedef void(__thiscall* BB3B10Fn)(void* thisPtr, uint32_t keyIndex, uint32_t pressed,
                                    uint32_t unk, uint32_t arg4);

// Returns 0 on an access violation (memory freed by an F5 restart).
static inline uint32_t SafeReadPtr(uint32_t addr) {
    if (!addr) return 0;
    __try {
        return *(uint32_t*)addr;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// root = [SG+1D5450], kbobj = [root+0x530]
static inline uint32_t GetKeyboardObject(GameAddresses* addr) {
    uint32_t root = SafeReadPtr((uint32_t)addr->player_base);
    if (!root) return 0;
    return SafeReadPtr(root + GameAddresses::KEYBOARD_OBJ_OFFSET);
}

// DI buffer = [kbobj+0x30]
static inline uint32_t GetDIBuffer(uint32_t kbobj) {
    if (!kbobj) return 0;
    return SafeReadPtr(kbobj + GameAddresses::DI_BUFFER_PTR_OFFSET);
}

// REC input. The Pico is a real USB HID device, so GetAsyncKeyState sees it.
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
        // Same values +3940 writes: 0x01 pressed, 0x00 released.
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

// BB3B10's 3rd and 4th arguments are the {lo, hi} dwords of the 64-bit
// Kernel::Time stamped at message-pump dispatch (Win32_Driver::Translate
// passes it through +3940 unchanged). The observer discards events older than
// the current race. Injections use Kernel::Time::Current(); this calibrated
// Time.hi is the fallback if that export fails to resolve.
inline volatile uint32_t g_bb3b10Arg4 = GameAddresses::BB3B10_ARG4;

// GetTickCount() of the last Supreme::Cycle tick. When the cycle has stopped
// (pause menu, dialogs, static menus), the input gate passes every key through
// so the user can operate them while a TAS mode is armed.
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

// The game's Kernel::Time::Current(). False if unresolved or the call faults.
// The callee is x87-balanced.
static bool GetKernelTimeNow(GameAddresses* addr, KernelTime* out) {
    if (!addr->time_current) return false;
    __try {
        addr->time_current(out, nullptr);
        return true;
    } __except(EXCEPTION_EXECUTE_HANDLER) { return false; }
}

// Call BB3B10 for each changed bit.
static void CallBB3B10OnTransitions(TasSharedState* s, GameAddresses* addr,
                                     uint32_t kbobj, uint8_t mask, uint8_t transitions) {
    if (!transitions || !kbobj) return;

    auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
    void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);

    // Stamp priority: test override (steer-impact test) > Time::Current >
    // calibrated fallback.
    KernelTime t = { 0, 0 };
    if (s->test_arg4_override) {
        t.lo = 0;
        t.hi = s->test_arg4_override;
        s->arg4_source = ARG4_SOURCE_OVERRIDE;
    } else if (GetKernelTimeNow(addr, &t)) {
        // The observer only checks hi. Flooring lo makes the stamp identical
        // between a REC and its replay.
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

// Copies coordinates as raw integer bits.
static void CapturePlayerCoords(TasSharedState* s, uint32_t index, bool isRec) {
    // The caller advances the index even when nothing is written, leaving a
    // stale coordinate that could read as movement, so flag the failure.
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

        // Gate = first frame whose position differs from frame 0. capture_ok
        // guarantees frame 0 belongs to this session, not the previous one.
        if (s->capture_ok && s->gate_index == 0 && index > 0) {
            const uint32_t* z = isRec ? (const uint32_t*)&s->rec_coords[0][0]
                                      : (const uint32_t*)&s->play_coords[0][0];
            if (raw[0] != z[0] || raw[1] != z[1] || raw[2] != z[2]) {
                s->gate_index = index;
            }
        }
    }
}


// Clear the raw input state and return the mask that still needs observer UP
// notifications.
static uint8_t ClearTasInputState(GameAddresses* addr) {
    uint8_t held = g_prevMask;
    g_prevMask = 0;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return held;
    WriteDIBuffer(GetDIBuffer(kbobj), 0);
    return held;
}

// Release every injected key: zero the DI buffer and send the observer UP
// events. Must run on every TAS -> OFF transition, or a held key keeps
// steering. After a level swap it clears the new level; UP events for keys
// that level never saw pressed are harmless.
static void ReleaseTasInput(TasSharedState* s, GameAddresses* addr) {
    uint8_t held = ClearTasInputState(addr);
    if (!held) return;
    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;
    CallBB3B10OnTransitions(s, addr, kbobj, 0, held);
}

// Drop gate alignment. The controller stages gate_align_rec right before each
// arm; a leftover value would re-index a later replay and move its splice.
static inline void ClearGateAlign(TasSharedState* s) {
    s->gate_align_rec = 0;
    s->cont_splice_approved = 0;  // no alignment, nothing to approve
}

static volatile uint32_t g_cyclePendingLog = 0;  // 0=none, 1=REC, 2=PLAY, 3=STOP, 4=playback_done
static volatile uint32_t g_cycleLogParam = 0;

// Set by the lifecycle hook when a race launches: refresh the rider and
// renderer stamps over the race's first ticks. Several ticks, because
// player_ptr is adopted after this hook and the first tick can still see the
// previous race's player.
inline volatile LONG g_refreshStamps = 0;
static constexpr LONG REFRESH_STAMP_TICKS = 100;

// Splice gate: 1 only between a successful CMD_ARM_CONTINUE and its splice.
// A stray continue_from_frame can then never turn a plain replay into REC.
// Private, not shared state, so no other process can set it.
static volatile uint32_t g_contArmed = 0;

// The complete TAS -> OFF transition. Game thread only. No logging,
// formatting or float arithmetic: it runs inside hooks.
static void ApplyStopTransition(TasSharedState* s, bool protectRestart) {
    // Set before publishing OFF so real keys cannot enter the restart window.
    s->cont_suppress_input = protectRestart ? 1u : 0u;
    s->mode = MODE_OFF;
    // A controller that stops mid-restart has abandoned it: let go of F5.
    if (s->restart_state == 1) {
        f5restart::Cancel();
        s->restart_state = 0;
    }
    ReleaseTasInput(s, g_cycleCaveAddr);
    s->continue_from_frame = 0;
    g_contArmed = 0;
    ClearGateAlign(s);
    g_cyclePendingLog = 3;
}

// Consume a pending STOP. Resetting to IDLE publishes completion, so it comes
// last, as a compare-exchange so a newer command is not erased.
static bool TryProcessStopCommand(TasSharedState* s) {
    const LONG cmd = InterlockedCompareExchange((volatile LONG*)&s->command, CMD_IDLE, CMD_IDLE);
    if (cmd != CMD_STOP && cmd != CMD_STOP_FOR_RESTART) return false;
    ApplyStopTransition(s, cmd == CMD_STOP_FOR_RESTART);
    InterlockedCompareExchange((volatile LONG*)&s->command, CMD_IDLE, cmd);
    return true;
}

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
    g_contArmed = 0;
    ClearGateAlign(s);
    g_cyclePendingLog = 1;
}

static void ArmPlay(TasSharedState* s) {
    g_cycleLogParam = s->recorded_count;
    ResetArmCounters(s);

    // A plain PLAY never splices.
    s->continue_from_frame = 0;
    g_contArmed = 0;

    // Don't force the spawn position: rotation and terrain contact would
    // still be the real spawn's, and the mismatch drifts.

    s->mode = MODE_PLAY;
    g_cyclePendingLog = 2;
}

static void RefuseArmContinue(TasSharedState* s, const char* reason) {
    LogRing(s, LOG_ERROR, reason);
    s->mode = MODE_OFF;
    s->continue_from_frame = 0;
    g_contArmed = 0;
    ClearGateAlign(s);
    g_cyclePendingLog = 3;  // "stopped"
}

// CONT: PLAY frames 0..continue_from_frame, then switch to REC.
static void ArmContinue(TasSharedState* s) {
    if (s->continue_from_frame == 0 || s->continue_from_frame > s->recorded_count) {
        RefuseArmContinue(s, "ARM_CONTINUE: invalid splice point");
        return;
    }
    // The prefix replay needs the boarder at rec_coords[0]'s state, which
    // holds only right after an F5 restart.
    if (s->mode != MODE_OFF) {
        RefuseArmContinue(s,
            "ARM_CONTINUE: refused — game is REC/PLAY; CONT requires a fresh restart first");
        return;
    }
    g_cycleLogParam = s->continue_from_frame;
    ResetArmCounters(s);
    // segment_count is kept; the splice adds one.
    s->mode = MODE_PLAY;
    g_contArmed = 1;  // the only place the splice gate opens
    s->cont_splice_approved = 0;
    // gate_align_rec is kept: the controller staged it for this arm.
    g_cyclePendingLog = 5;
}

static void BeginRestart(TasSharedState* s) {
    f5restart::Cancel();  // a new request replaces an unfinished one
    f5restart::Request();
    s->restart_state = 1;
    // Alignment is staged after the restart; anything older is stale.
    ClearGateAlign(s);
    g_cyclePendingLog = 7;  // "restart initiated"
}

// No logging or formatting here: runs inside the mid-hook.
static void ProcessCommand(TasSharedState* s) {
    // Acquire-read of command; the Rust writer stores it last, with Release.
    uint32_t cmd = (uint32_t)InterlockedCompareExchange(
        (volatile LONG*)&s->command, CMD_IDLE, CMD_IDLE);
    if (cmd == CMD_IDLE) return;
    if (cmd == CMD_STOP || cmd == CMD_STOP_FOR_RESTART) {
        TryProcessStopCommand(s);
        return;
    }

    switch (cmd) {
        case CMD_ARM_REC:      ArmRec(s);       break;
        case CMD_ARM_PLAY:     ArmPlay(s);      break;
        case CMD_ARM_CONTINUE: ArmContinue(s);  break;
        case CMD_RESTART:      BeginRestart(s); break;
    }

    // arm_generation is the controller's "arm landed" signal, so it is the
    // arm's last store (x86 keeps store order). Refused arms bump it too.
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE || cmd == CMD_ARM_REC) {
        s->gate_index = 0;
        s->capture_ok = 1;
    }
    if (cmd == CMD_ARM_PLAY || cmd == CMD_ARM_CONTINUE) {
        s->arm_generation++;
    }

    InterlockedExchange((volatile LONG*)&s->command, CMD_IDLE);
}

// Called from the message pump, outside Supreme::Cycle.
static void FlushPendingLog() {
    uint32_t log = g_cyclePendingLog;
    if (!log) return;
    g_cyclePendingLog = 0;
    uint32_t param = g_cycleLogParam;
    switch (log) {
        case 1: Log("Cycle cave: entering REC mode"); break;
        case 2: Log(std::format("Cycle cave: entering PLAY mode ({} ticks recorded)", param)); break;
        case 3: Log("Cycle cave: stopped"); break;
        case 4: Log(std::format("Cycle cave: playback complete at frame {}", param)); break;
        case 5: Log(std::format("Cycle cave: continue record (PLAY until frame {})", param)); break;
        case 6: Log(std::format("Cycle cave: spliced to REC at frame {}", param)); break;
        case 7: Log("Cycle cave: in-process F5 restart initiated"); break;
        case 8: Log("Cycle cave: restart complete (the game rebuilt the level)"); break;
    }
}

// Called before each cycle as well as after the last prefix tick: approval
// arrives while the tick cave is parked at the splice, and running another
// PLAY tick on resume would duplicate it. An aligned prefix splices only once
// the watcher approves it.
static void CompleteContinueSplice(TasSharedState* s) {
    uint32_t aligned_splice = GateAlignedSplicePos(
        s->continue_from_frame, s->gate_index, s->gate_align_rec);
    if (g_contArmed && s->continue_from_frame > 0
            && s->playback_pos >= aligned_splice
            && (s->gate_align_rec == 0 || s->cont_splice_approved != 0)) {
        uint32_t rec_splice = s->continue_from_frame;
        s->recorded_count = rec_splice;
        // Switch to the resume speed on the splice tick itself, not when the
        // UI notices, or the take starts at the catch-up rate.
        if (s->cont_resume_speed > 0.0f) {
            s->playback_speed = s->cont_resume_speed;
        }

        uint32_t segIdx = s->segment_count;
        if (segIdx < TAS_MAX_SEGMENTS) {
            s->segment_boundaries[segIdx].frame = rec_splice;
            s->segment_count = segIdx + 1;
        }
        s->segment_start_frame = rec_splice;

        s->mode = MODE_REC;
        s->continue_from_frame = 0;
        g_contArmed = 0;
        g_cycleLogParam = rec_splice;
        g_cyclePendingLog = 6;
    }
}

// Publish the live position and velocity in every mode. Holds a __try, so no
// C++ objects needing unwinding (C2712).
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

// Runs in every mode; see f5_restart_cave.hpp.
static void StepInProcessRestart(TasSharedState* s) {
    if (s->restart_state == 1 && f5restart::Step()) {
        s->restart_state = 2;
        g_cyclePendingLog = 8;
    }
}

// The race is being left (Supreme::Stop, lifecycle_cave.hpp). Stop any armed
// mode, and abandon a restart still holding F5, or the next race would
// restart itself.
static void StopForLeftRace(TasSharedState* s) {
    if (s->mode != MODE_OFF) {
        ApplyStopTransition(s, false);
        LogRing(s, LOG_WARN, "TAS stopped: the race was left");
    }
    if (s->restart_state == 1) {
        f5restart::Cancel();
        s->restart_state = 0;
    }
    s->cont_suppress_input = 0;
}

static void RecTick(TasSharedState* s, GameAddresses* addr, uint32_t kbobj) {
    uint32_t index = s->recorded_count;
    if (index >= TAS_MAX_TICKS) {
        // 10 min 55 s at 100 ticks/s. Log it so it isn't mistaken for a STOP.
        LogRing(s, LOG_WARN, "REC stopped: recording buffer full (TAS_MAX_TICKS)");
        s->mode = MODE_OFF;
        ReleaseTasInput(s, addr);
        g_cyclePendingLog = 3;
        return;
    }

    // The key-handler cave blocks +3940, so REC injects exactly as PLAY does.
    uint8_t mask = SampleGAKS();
    uint8_t transitions = mask ^ g_prevMask;

    uint32_t buffer = GetDIBuffer(kbobj);
    WriteDIBuffer(buffer, mask);

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

    // With alignment the end is gate-relative too. recorded_count comes from
    // the UI process, so cap it.
    const uint32_t rec_count =
        s->recorded_count < TAS_MAX_TICKS ? s->recorded_count : TAS_MAX_TICKS;
    uint32_t play_end = rec_count;
    if (s->gate_align_rec > 0 && s->gate_index > 0 && rec_count > s->gate_align_rec) {
        play_end = s->gate_index + (rec_count - s->gate_align_rec);
    }
    if (pos >= play_end) {
        s->mode = MODE_OFF;
        ClearGateAlign(s);
        ReleaseTasInput(s, addr);
        g_contArmed = 0;
        g_cycleLogParam = pos;
        g_cyclePendingLog = 4;
        return;
    }

    // Gate-relative input source; see gate_alignment.hpp.
    uint32_t src = pos;
    if (s->gate_align_rec > 0) {
        src = GateAlignedInputSource(pos, s->gate_index, s->gate_align_rec, rec_count);
    }
    uint8_t mask = (src == GATE_ALIGN_INVALID_SOURCE) ? (uint8_t)0 : s->input_log[src];

    uint32_t buffer = GetDIBuffer(kbobj);
    WriteDIBuffer(buffer, mask);

    uint8_t transitions = mask ^ g_prevMask;
    if (transitions) {
        CallBB3B10OnTransitions(s, addr, kbobj, mask, transitions);
    }

    g_prevMask = mask;

    CapturePlayerCoords(s, pos, false);

    s->playback_pos = pos + 1;

    CompleteContinueSplice(s);
}

static void __declspec(noinline) CycleCave_Logic() {
    auto* s = g_cycleCaveState;
    auto* addr = g_cycleCaveAddr;
    if (!s || !addr) return;

    s->frame_count++;
    g_lastCycleMs = GetTickCount();

    if (s->replay_ptr) {
        s->player_ptr = SafeReadPtr(s->replay_ptr + GameAddresses::REPLAY_PLAYER_OFFSET);
    }

    if (g_refreshStamps > 0 && s->player_ptr) {
        g_refreshStamps--;
        rider::Refresh(s);
        renderer::Refresh(s);
    }

    PublishLivePosition(s);

    ProcessCommand(s);

    StepInProcessRestart(s);

    if (s->mode == MODE_OFF) return;

    uint32_t kbobj = GetKeyboardObject(addr);
    if (!kbobj) return;

    if (s->mode == MODE_PLAY) CompleteContinueSplice(s);

    if (s->mode == MODE_REC) {
        RecTick(s, addr, kbobj);
    } else if (s->mode == MODE_PLAY) {
        PlayTick(s, addr, kbobj);
    }
}

static void CycleCave_Callback(SafetyHookContext&) {
    if (auto* s = g_cycleCaveState) {
        // The physics precision the renderer set (0x007F DirectX 6/7 = 24-bit,
        // OpenGL = 53/64), from the saved image: the hook re-initialises the FPU.
        s->fpu_control_word = g_hookFpuControlWord;
    }
    CycleCave_Logic();
}

bool InstallCycleCave(GameAddresses& addr, TasSharedState* state) {
    if (!addr.cycle_cave_site) {
        Log("Cycle cave: hook site not resolved");
        return false;
    }

    g_cycleCaveState = state;
    g_cycleCaveAddr = &addr;
    Log(std::format("Cycle cave: hooking Supreme::Cycle at {:p}", (void*)addr.cycle_cave_site));

    cycleCaveHook = CreateMidHook<CycleCave_Callback>(addr.cycle_cave_site);

    if (!cycleCaveHook) {
        Log("Cycle cave: SafetyHook create_mid FAILED");
        g_cycleCaveState = nullptr;
        g_cycleCaveAddr = nullptr;
        return false;
    }

    state->cycle_cave_hooked = 1;
    Log("Cycle cave: hook installed successfully");
    return true;
}
