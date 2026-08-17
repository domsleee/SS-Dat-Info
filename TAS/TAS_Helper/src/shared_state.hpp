#pragma once
#include "stdafx.h"

// Shared memory layout between TAS_Helper.dll and the egui UI process.
// Lock-free single-writer: UI writes command region, DLL writes status region.
// Both use atomic uint32_t for command/mode fields.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";
constexpr uint32_t TAS_SHARED_VERSION = 14; // +level_epoch/level_scan_epoch (root-based level context)
constexpr uint32_t TAS_MAX_TICKS = 65536;
constexpr uint32_t TAS_MAX_SEGMENTS = 32;      // Max segment boundaries
constexpr uint32_t TAS_LOG_RING_SIZE = 64;     // Number of log entries
constexpr uint32_t TAS_LOG_ENTRY_SIZE = 120;   // Max text bytes per entry (incl NUL)

// Commands (UI -> DLL)
enum TasCommand : uint32_t {
    CMD_IDLE         = 0,
    CMD_ARM_REC      = 1,
    CMD_ARM_PLAY     = 2,
    CMD_STOP         = 3,
    CMD_ARM_CONTINUE = 4,  // PLAY 0..continue_from_frame, then auto-switch to REC
    CMD_RESTART      = 5,  // In-process F5 restart (no Pico/focus needed)
    CMD_SNAPSHOT     = 6,  // PROTOTYPE: capture writable memory snapshot at this frame
    CMD_RESTORE      = 7,  // PROTOTYPE: restore the last snapshot (instant rewind)
    CMD_SNAPSHOT_AT_SPAWN = 8, // PROTOTYPE: arm a snapshot at the next PLAY frame-0 (spawn)
};

// Modes (DLL -> UI)
enum TasMode : uint32_t {
    MODE_OFF  = 0,
    MODE_REC  = 1,
    MODE_PLAY = 2,
};

// Input mask bit definitions
enum TasInputBit : uint8_t {
    INPUT_LEFT  = 0x01,  // bit 0
    INPUT_RIGHT = 0x02,  // bit 1
    INPUT_UP    = 0x04,  // bit 2
    INPUT_DOWN  = 0x08,  // bit 3
    INPUT_JUMP  = 0x10,  // bit 4
    INPUT_SHIFT = 0x20,  // bit 5
};

// Where the injected BB3B10 Time stamp came from (TasSharedState::arg4_source)
enum TasArg4Source : uint32_t {
    ARG4_SOURCE_NONE         = 0,  // no injection yet
    ARG4_SOURCE_TIME_CURRENT = 1,  // Kernel::Time::Current() — the proper path
    ARG4_SOURCE_CALIBRATED   = 2,  // fallback: Time.hi observed from real keypresses
    ARG4_SOURCE_OVERRIDE     = 3,  // test_arg4_override forced (steer-impact test)
};

// Log severity levels
enum TasLogSeverity : uint32_t {
    LOG_DEBUG = 0,
    LOG_INFO  = 1,
    LOG_WARN  = 2,
    LOG_ERROR = 3,
};

// A segment boundary record (8 bytes)
struct TasSegmentBoundary {
    uint32_t frame;              // Frame number where this segment starts
    uint32_t input_log_offset;   // Offset into input_log for this segment
};

// A single log ring entry (128 bytes, naturally aligned)
struct TasLogEntry {
    uint32_t  sequence;                      // Monotonic counter (0 = unused)
    uint32_t  severity;                      // TasLogSeverity
    char      text[TAS_LOG_ENTRY_SIZE];      // NUL-terminated message
};

// Hook performance counters (cycles measured with __rdtsc).
struct TasHookPerfCounter {
    uint64_t calls;          // Number of callback invocations
    uint64_t cycles_total;   // Sum of elapsed cycles across all calls
    uint64_t cycles_max;     // Worst single-call elapsed cycles
};

// All fields are naturally aligned (uint32_t/float = 4 bytes).
// No packing needed; must match Rust repr(C) layout.
struct TasSharedState {
    // -- Version (read-only after init) --
    uint32_t version;               // TAS_SHARED_VERSION

    // -- Command region (UI writes, DLL reads) --
    volatile uint32_t command;      // TasCommand
    uint32_t inject_mode;           // 6 = proven mode
    uint32_t force_fixed_tick;      // 0=natural ticks (proven zero-drift), N=force N ticks/frame
    uint32_t force_direct;          // 2=BB3B10 direct calls
    uint32_t self_capture;          // 0=false
    uint32_t use_rec_msg_args;      // 1=use recorded wrapper args
    uint32_t input_source;          // 0=DI buffer sample
    uint32_t continue_from_frame;   // CMD_ARM_CONTINUE: splice point (PLAY 0..N, then REC)

    // -- Status region (DLL writes, UI reads) --
    volatile uint32_t mode;         // TasMode
    uint32_t recorded_count;        // ticks recorded
    uint32_t playback_pos;          // current playback tick
    uint32_t prev_mask;             // last input bitmask
    uint32_t cave2_injecting;       // playback injection active (Cave 1C/1D gate flag)
    float    player_x, player_y, player_z;
    float    max_drift_x, max_drift_z;

    // -- Diagnostics --
    uint32_t bb3b10_call_count;     // BB3B10 observer calls (Cave 2 direct)
    uint32_t handler_block_count;   // Cave 1C: external handler blocks during PLAY
    uint32_t frame_count;           // Cave 2: total frames processed
    uint32_t event_count;           // General event counter
    uint32_t bb3b10_block_count;    // Cave 1D: BB3B10 blocks during REC mode 6

    // -- Hook performance counters (DLL writes, UI/tests read) --
    TasHookPerfCounter perf_cave2;
    TasHookPerfCounter perf_cave5;
    TasHookPerfCounter perf_cave1c_down;
    TasHookPerfCounter perf_cave1c_up;
    TasHookPerfCounter perf_cave1d;
    TasHookPerfCounter perf_replay_capture;

    // -- Hook status (DLL writes, UI reads) --
    uint32_t cave2_hooked;          // 1 if Supreme::Cycle hook installed
    uint32_t cave1c_hooked;         // 1 if handler gate hooks installed
    uint32_t cave1d_hooked;         // 1 if BB3B10 observer hook installed
    uint32_t cave5_hooked;          // 1 if fixed tick hook installed
    uint32_t replay_capture_hooked; // 1 if replay object capture hook installed

    // -- Runtime pointers (DLL internal, exposed for diagnostics) --
    uint32_t replay_ptr;            // Current replay object pointer
    uint32_t player_ptr;            // Current player pointer ([replayObj+0x84])

    // Variable speed playback (1.0 = normal, 0.5 = half, 2.0 = double)
    float    playback_speed;            // UI writes, Cave 5 reads

    // In-process restart state machine (DLL internal)
    // 0=idle, 1=F5 pressed (waiting frames), 2=F5 released (done)
    volatile uint32_t restart_state;
    uint32_t restart_frames_held;       // Frames F5 has been held down

    // -- Telemetry (DLL writes, UI reads) --
    float    prev_player_x, prev_player_y, prev_player_z;  // Previous frame position
    float    velocity_x, velocity_y, velocity_z;            // Per-frame velocity (pos - prev_pos)
    float    speed;                                          // Magnitude of velocity XZ
    uint32_t tick_count;                                    // Current tick count from Cave 5

    // -- Segment fields (DLL writes, UI reads) --
    uint32_t segment_index;          // Current segment number (0-based)
    uint32_t segment_start_frame;    // Frame offset where current segment begins
    uint32_t segment_count;          // Total segments in current run
    uint32_t snapshot_size;          // Bytes in last boundary snapshot
    uint32_t snapshot_flags;         // Bitmask: bit0=player, bit1=physics, bit2=fpu, bit3=rand_seed
    uint32_t snapshot_buffer_ptr;    // External snapshot allocation pointer
    uint32_t snapshot_buffer_capacity; // Total snapshot buffer size
    TasSegmentBoundary segment_boundaries[TAS_MAX_SEGMENTS]; // (frame, offset) pairs

    // -- Rotation telemetry (DLL writes, UI reads) --
    float rotation_matrix[9]; // 3x3 row-major rotation matrix from player+0x104..+0x124

    // -- Input log (DLL reads/writes) --
    uint8_t  input_log[TAS_MAX_TICKS];

    // -- Coordinate logs --
    float    rec_coords[TAS_MAX_TICKS][3];   // X,Y,Z per tick (REC)
    float    play_coords[TAS_MAX_TICKS][3];  // X,Y,Z per tick (PLAY)

    // -- Log ring buffer (DLL writes, UI reads) --
    volatile uint32_t log_write_seq;              // Next sequence number to write (monotonic)
    TasLogEntry       log_ring[TAS_LOG_RING_SIZE]; // Circular buffer of log entries

    // -- CONT splice timing (DLL writes, harness/UI reads) --
    // frame_count stamped at CONT replay start and at the PLAY->REC splice; the
    // delta = game-frames the catch-up replay took (the "resume off by a few
    // frames" measurement). Must stay last to match the Rust struct layout.
    uint32_t cont_replay_start_fc;
    uint32_t cont_splice_fc;

    // -- CONT resume speed (UI writes, DLL reads) --
    // Applied to playback_speed atomically at the splice so the resumed
    // recording doesn't fast-forward at the catch-up rate. 0 = unset.
    float    cont_resume_speed;

    // -- CONT clock-backlog reset (cave2 sets at splice, cave5 consumes) --
    uint32_t cont_reset_pending;

    // -- Game-state awareness (DLL writes each frame from exe+0x8895C) --
    // 0 = main menu, 1 = in-game (in a race/level). Lets the UI know the game
    // state and lets cave5 avoid touching ticks in the menu.
    uint32_t game_in_game;

    // -- Current track (DLL background thread writes; UI reads) --
    // In-process heap scan, majority-voted: 0..8 = area*3 + difficulty
    // (area 0=Forest,1=Alpine,2=Village; diff 0=Easy,1=Medium,2=Hard).
    // 0xFFFFFFFF = unknown / menu. See level_scan.hpp.
    uint32_t level_id;

    // -- Race timer (DLL reads the HUD time line via SR_UIT Append_Text) --
    // race_time_cs  = the on-screen player race time in centiseconds (exact,
    //                 map-agnostic). 0xFFFFFFFF = not racing / unknown.
    // race_start_ts = the 16-bit game clock value captured at the gate cross
    //                 (= clock - race_time); constant during a run; the F5
    //                 spawn-lottery metric. 0xFFFFFFFF = unknown.
    uint32_t race_time_cs;
    uint32_t race_start_ts;

    // -- Clock-phase pin (config: UI/harness; cave2 resets phase; cave5 runs) --
    // The F5 "bucket lottery" is the per-frame tick-count PATTERN during the
    // spawn settle: naturally it depends on the wall-clock phase at restart
    // and the render frame rate, so the spawn microstate at the first-moving
    // frame shifts whenever the machine's timing fingerprint shifts (fps, OS
    // updates, background load) — that is what killed cross-session CONT on
    // 2026-06-09. The pin replaces the wall-clock schedule with a CANONICAL
    // repeating pattern [1,1,0] ticks/frame (~native pacing at ~150 fps)
    // whenever the game is in OFF mode, in-game, at 1x — which is exactly
    // where the settle runs (REC/PLAY arm only after the settle), so live
    // steering and catch-up speed are untouched. The pattern phase is reset
    // by CMD_RESTART, so every in-process restart replays the same schedule
    // -> the same bucket (modulo reload frame-count variance), on any
    // machine/fps/OS state.
    // clock_pin_enabled: config, 0 = pin off (natural lottery), nonzero = on.
    // clock_pin_phase:   status/internal, position in the [1,1,0] cycle.
    uint32_t clock_pin_enabled;
    volatile uint32_t clock_pin_phase;

    // -- Test hook: force the injected BB3B10 arg4 (config) --
    // 0 = normal (inject the live Kernel::Time::Current() stamp). Nonzero =
    // inject this EXACT value as Time.hi (lo=0) and suppress all calibration.
    // The steer-impact regression test drives it in two phases: a
    // deliberately-wrong value (injection must be silently discarded →
    // boarder goes straight) then 0 (live time stamp → boarder turns). That
    // contrast proves the event Time is load-bearing and guards the whole
    // inject path from regressing.
    uint32_t test_arg4_override;

    // -- Injection Time-stamp source (DLL writes at each injection batch) --
    // TasArg4Source: 1 = Kernel::Time::Current (the proper, focus-independent
    // path), 2 = keypress-calibrated fallback, 3 = test override. The
    // steer-impact test asserts 1 so the proper mechanism can't silently
    // regress to the fallback.
    uint32_t arg4_source;

    // -- Live-input suppression during a CONT (UI writes, DLL reads) --
    // 1 while a Continue cycle is in flight — from the moment the UI begins
    // the CONT (BEFORE the F5 restart) through to the PLAY→REC splice. cave1c
    // blocks the game's real key handler whenever this is set, regardless of
    // mode. This plugs the hole the mode-based block can't: the post-F5 spawn
    // COUNTDOWN runs in OFF mode, so without this a live keypress (e.g. jump)
    // reaches the spawn and perturbs the physics — the bucket then matches
    // first-moving but diverges right after the judge window (observed: drift
    // growing 0.5→5.6 over ticks 377-533, accepted as "bucket matched").
    // Catch-up PLAY and post-splice REC are already handler-blocked by mode,
    // so the UI clears this the instant the bucket aligns (StepOutcome::Done)
    // — from there on only PLAY/REC run, and post-splice REC must see live
    // input (that's the resumed recording). ESC stays exempt (abort hatch).
    uint32_t cont_suppress_input;

    // -- Menu present-rate throttle (DLL writes present_count; UI writes cap) --
    // The renderer presents via gdi32!SwapBuffers. Windowed on a high-refresh
    // desktop, DWM vsyncs every present to the monitor (e.g. 165 Hz), and the
    // main-menu attract demo advances one video frame per present -> it plays
    // ~2.7x too fast. Gameplay is refresh-independent (cave5 accumulator), so
    // we throttle ONLY while the engine cycle is frozen (menu/pause). See
    // frame_limit.hpp.
    //   present_count: monotonic count of SwapBuffers calls (diagnostic; lets a
    //                  poller measure the live present rate).
    //   menu_fps_cap:  0 = OFF (count only, no throttle); N = cap menu presents
    //                  to N fps. Default 34.
    //
    // ROOT CAUSE of the 2x (RE'd 2026-06-11): the static main menu has no FMV
    // file — Main_Menu.dll animates the background one frame per present, and
    // sr.dll's frame limiter is Sleep-based (it imports kernel32 Sleep +
    // GetTickCount). A Sleep limiter's effective rate depends on the SYSTEM
    // TIMER RESOLUTION: with the TAS tooling running, the resolution is raised
    // to 1 ms (vs the native ~15.6 ms), so Sleep is accurate, the limiter
    // undershoots its delay, and the menu presents ~2x faster (~68 vs ~34 fps)
    // — i.e. "without the dll it is not fast forwarded." NEITHER DLL hooks the
    // present path (proven: caves frozen at the menu + full code search); the
    // doubling is purely the timer-resolution side effect. Capping presents to
    // 34 while the engine cycle is frozen restores the native menu speed and is
    // robust regardless of who raised the timer. Gameplay is refresh- and
    // timer-independent (cave5 accumulator) so it is untouched, and CONT is
    // gated out (cont_suppress_input) so the F5 bucket lottery is unaffected.
    uint32_t present_count;
    uint32_t menu_fps_cap;

    // -- Level context epoch (both DLL-written; UI reads) --
    // The engine's root object ([SG+0x1D5450]) SURVIVES an F5 restart but is
    // reallocated on quit-to-menu / menu-demo load / track switch — the same
    // signal the armed-mode auto-stop uses, RDIAG-proven. That makes a root
    // change the only trustworthy "the level under you was swapped" event;
    // level_id going unknown is merely "the last scan found nothing", which
    // also happens transiently while the level is still loaded.
    //
    //   level_epoch:      bumped on every root change (root==0 is mid-teardown
    //                     and is NOT a change). Tracked every cycle regardless
    //                     of TAS mode, so menu/load transitions are observable.
    //   level_scan_epoch: the epoch the level-scan thread had observed when it
    //                     last published a CONCRETE level_id.
    //
    // level_id is therefore trustworthy iff level_scan_epoch == level_epoch.
    // When they differ, the context changed and the new track has not been
    // identified yet — the UI must say "resolving", not assert the old level.
    uint32_t level_epoch;
    uint32_t level_scan_epoch;
};

// Write a log entry to the ring buffer. Safe to call from hook callbacks
// (no file I/O, no heap allocation, no printf/format).
// text must be a NUL-terminated C string.
inline void LogRing(TasSharedState* s, TasLogSeverity severity, const char* text) {
    if (!s) return;
    uint32_t seq = s->log_write_seq++;
    uint32_t idx = seq % TAS_LOG_RING_SIZE;
    TasLogEntry* entry = &s->log_ring[idx];
    entry->severity = severity;
    // Safe string copy without x87/float ops
    size_t i = 0;
    while (i < TAS_LOG_ENTRY_SIZE - 1 && text[i] != '\0') {
        entry->text[i] = text[i];
        i++;
    }
    entry->text[i] = '\0';
    // Write sequence last (acts as release fence for reader)
    entry->sequence = seq + 1;  // +1 so 0 means "unused"
}

inline void PerfSample(TasHookPerfCounter& c, uint64_t elapsedCycles) {
    c.calls++;
    c.cycles_total += elapsedCycles;
    if (elapsedCycles > c.cycles_max) {
        c.cycles_max = elapsedCycles;
    }
}

inline void ResetHookPerfCounters(TasSharedState* s) {
    if (!s) return;
    memset(&s->perf_cave2, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave5, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1c_down, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1c_up, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_cave1d, 0, sizeof(TasHookPerfCounter));
    memset(&s->perf_replay_capture, 0, sizeof(TasHookPerfCounter));
}

// Shared memory management (DLL side - creates the mapping)
class TasSharedMemory {
public:
    HANDLE hMapFile = nullptr;
    TasSharedState* state = nullptr;

    bool Create() {
        hMapFile = CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            nullptr,
            PAGE_READWRITE,
            0,
            sizeof(TasSharedState),
            TAS_SHARED_MEMORY_NAME
        );
        if (!hMapFile) return false;

        state = (TasSharedState*)MapViewOfFile(
            hMapFile,
            FILE_MAP_ALL_ACCESS,
            0, 0,
            sizeof(TasSharedState)
        );
        if (!state) {
            CloseHandle(hMapFile);
            hMapFile = nullptr;
            return false;
        }

        // Zero-init and set version
        memset(state, 0, sizeof(TasSharedState));
        state->version = TAS_SHARED_VERSION;
        state->inject_mode = 6;        // proven default
        state->force_fixed_tick = 0;    // natural ticks (proven zero-drift config)
        state->force_direct = 2;        // BB3B10 direct calls
        state->use_rec_msg_args = 1;   // proven zero-drift config
        state->playback_speed = 1.0f;  // normal speed
        state->level_id = 0xFFFFFFFFu;  // unknown until the scan thread runs
        state->race_time_cs = 0xFFFFFFFFu;
        state->race_start_ts = 0xFFFFFFFFu;
        // Clock-phase pin OFF by default. The v1 pin froze the game during
        // level reloads; v2 (passthrough when behind) still coincided with an
        // sr.dll renderer crash at a pinned restart (2026-06-10 14:21,
        // c0000005 @ sr.dll+0x13568) — suspected 0-tick frames at reload
        // boundaries the renderer doesn't tolerate. Opt in via shared memory
        // (write 1) for pin experiments; do not default-enable until the
        // reload-boundary interaction is understood and the pin is
        // re-validated through full REC/PLAY/CONT cycles INCLUDING the
        // save-dialog and menu paths.
        state->clock_pin_enabled = 0;
        state->clock_pin_phase = 0;
        state->test_arg4_override = 0;
        state->arg4_source = ARG4_SOURCE_NONE;
        state->cont_suppress_input = 0;
        state->present_count = 0;
        // Cap the static-menu present rate to its native ~34 fps (the TAS
        // tooling raises the system timer to 1 ms, which doubles sr.dll's
        // Sleep-based menu limiter to ~68 fps = the "2x menu video"). Engaged
        // only while the engine cycle is frozen (menu/pause); gameplay + CONT
        // are untouched. Set 0 via shared memory to disable.
        state->menu_fps_cap = 34;
        // Start EQUAL: at init nothing has been identified yet, and level_id is
        // already 0xFFFFFFFF, so "trustworthy but unknown" is the honest state
        // (we are at the menu / not yet scanned) rather than "resolving".
        state->level_epoch = 0;
        state->level_scan_epoch = 0;
        return true;
    }

    void Destroy() {
        if (state) {
            UnmapViewOfFile(state);
            state = nullptr;
        }
        if (hMapFile) {
            CloseHandle(hMapFile);
            hMapFile = nullptr;
        }
    }

    ~TasSharedMemory() { Destroy(); }
};
