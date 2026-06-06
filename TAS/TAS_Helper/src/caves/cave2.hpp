#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../helper.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"

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

// Helper: call BB3B10 for each changed bit (transitions only)
static void CallBB3B10OnTransitions(TasSharedState* s, GameAddresses* addr,
                                     uint32_t kbobj, uint8_t mask, uint8_t transitions) {
    if (!transitions || !kbobj) return;

    auto bb3b10 = (BB3B10Fn)(addr->bb3b10);
    void* thisPtr = (void*)(kbobj + GameAddresses::BB3B10_THIS_OFFSET);

    // Set cave2_injecting so Cave 1C/1D pass through
    s->cave2_injecting = 1;

    if (transitions & INPUT_LEFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_LEFT, (mask & INPUT_LEFT) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
    }
    if (transitions & INPUT_RIGHT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_RIGHT, (mask & INPUT_RIGHT) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
    }
    if (transitions & INPUT_UP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_UP, (mask & INPUT_UP) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
    }
    if (transitions & INPUT_DOWN) {
        bb3b10(thisPtr, GameAddresses::BB3B10_DOWN, (mask & INPUT_DOWN) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
    }
    if (transitions & INPUT_JUMP) {
        bb3b10(thisPtr, GameAddresses::BB3B10_JUMP, (mask & INPUT_JUMP) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
    }
    if (transitions & INPUT_SHIFT) {
        bb3b10(thisPtr, GameAddresses::BB3B10_SHIFT, (mask & INPUT_SHIFT) ? 1 : 0,
               0, GameAddresses::BB3B10_ARG4);
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
               0, GameAddresses::BB3B10_ARG4);
        s->cave2_injecting = 0;
    }
}

// Deferred log messages — set in callback, logged outside callback
static volatile uint32_t g_cave2_pendingLog = 0;  // 0=none, 1=REC, 2=PLAY, 3=STOP, 4=playback_done
static volatile uint32_t g_cave2_logParam = 0;

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
            s->mode = MODE_REC;
            g_cave2_pendingLog = 1;
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
            g_cave2_pendingLog = 5;
            break;

        case CMD_STOP:
            s->mode = MODE_OFF;
            s->cave2_injecting = 0;
            g_cave2_pendingLog = 3;
            break;

        case CMD_RESTART:
            // Begin in-process F5 restart sequence
            s->restart_state = 1;
            s->restart_frames_held = 0;
            g_cave2_pendingLog = 7;  // "restart initiated"
            break;
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
    }
}

// Cave 2 callback logic — called with FPU state saved/restored.
// Separated from the FSAVE wrapper because MSVC forbids __asm in functions with SEH.
static void __declspec(noinline) Cave2_Logic() {
    auto* s = g_cave2State;
    auto* addr = g_cave2Addr;
    if (!s || !addr) return;

    s->frame_count++;

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
                g_cave2_pendingLog = 8;
            }
        }
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

    } else if (s->mode == MODE_PLAY) {
        uint32_t pos = s->playback_pos;

        // CONT replay starting (first replay tick): stamp frame_count so the
        // harness can measure how many game-frames the catch-up replay takes to
        // reach the splice (the "resume off by a few frames" skew).
        if (pos == 0 && s->continue_from_frame > 0) {
            s->cont_replay_start_fc = s->frame_count;
        }

        if (pos >= s->recorded_count) {
            s->mode = MODE_OFF;
            g_cave2_logParam = pos;
            g_cave2_pendingLog = 4;
            return;
        }

        uint8_t mask = s->input_log[pos];

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

        // Continue Record: auto-switch to REC AFTER processing the splice frame.
        // This ensures the splice frame gets normal PLAY processing (input injection
        // + coordinate capture), maintaining symmetry with the final PLAY phase.
        if (s->continue_from_frame > 0 && s->playback_pos >= s->continue_from_frame) {
            uint32_t splice_pos = s->playback_pos;
            s->recorded_count = splice_pos;

            // Stamp the splice instant. (cont_splice_fc - cont_replay_start_fc)
            // is the game-frames the replay took to reach the splice — the
            // diagnostic for "resume yields a few frames early/late".
            s->cont_splice_fc = s->frame_count;

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
