#pragma once
#include "stdafx.h"

// Shared memory layout between TAS_Helper.dll and the egui UI process.
// Lock-free single-writer: UI writes command region, DLL writes status region.
// Both use atomic uint32_t for command/mode fields.

constexpr const char* TAS_SHARED_MEMORY_NAME = "Local\\SupremeTAS";
constexpr size_t TRACE_FRAMES = 384;

constexpr size_t OBJSNAP_PLAYER_DWORDS = 128;
constexpr size_t OBJSNAP_PHYSICS_DWORDS = 512;

constexpr uint32_t TAS_SHARED_VERSION = 49; // protected internal STOP command; layout unchanged
constexpr uint32_t TAS_MENU_DOC_MAX = 4096;  // v46 menu document buffer (JSON, NUL-terminated)
constexpr uint32_t TAS_MENU_CMD_TARGET_MAX = 64;  // v47 menu command target (id or label, NUL-terminated)
// v47 menu_cmd_kind
constexpr uint32_t TAS_MENU_CMD_NONE = 0;
constexpr uint32_t TAS_MENU_CMD_ACTIVATE = 1;   // focus the target, then Enter on it
constexpr uint32_t TAS_MENU_CMD_FOCUS = 2;      // just move the cursor to the target
constexpr uint32_t TAS_MENU_CMD_UP = 3;
constexpr uint32_t TAS_MENU_CMD_DOWN = 4;
constexpr uint32_t TAS_MENU_CMD_LEFT = 5;
constexpr uint32_t TAS_MENU_CMD_RIGHT = 6;
constexpr uint32_t TAS_MENU_CMD_TRIGGER = 7;    // Enter on whatever is focused
// v47 menu_cmd_result
constexpr uint32_t TAS_MENU_RESULT_OK = 0;
constexpr uint32_t TAS_MENU_RESULT_NO_MENU = 1;
constexpr uint32_t TAS_MENU_RESULT_NOT_FOUND = 2;
constexpr uint32_t TAS_MENU_RESULT_DISABLED = 3;
constexpr uint32_t TAS_MENU_RESULT_BAD_KIND = 4;
constexpr uint32_t TAS_MENU_RESULT_FAULT = 5;
constexpr uint32_t TAS_MENU_RESULT_NOT_FOCUSABLE = 6;   // the focus did not land on the target; nothing triggered
constexpr uint32_t TAS_MENU_RESULT_STALE_PAGE = 7;      // v48: the page moved on since the agent read it; nothing done
constexpr uint32_t TAS_MENU_RESULT_EXPIRED = 8;         // v48: no menu was executing for 3 s; nothing done
constexpr uint32_t TAS_LEVEL_PATH_MAX = 128;
constexpr uint32_t TAS_MENU_SCREEN_MAX = 32;   // v44: menu-screen title buffer
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
    CMD_STOP_FOR_RESTART = 9, // internal stop: protect live input across restart
    CMD_ARM_CONTINUE = 4,  // PLAY 0..continue_from_frame, then auto-switch to REC
    CMD_RESTART      = 5,  // In-process F5 restart (no Pico/focus needed)
    CMD_SNAPSHOT     = 6,  // PROTOTYPE: capture writable memory snapshot at this frame
    CMD_RESTORE      = 7,  // PROTOTYPE: restore the last snapshot (instant rewind)
    CMD_SNAPSHOT_AT_SPAWN = 8, // PROTOTYPE: arm a snapshot at the next PLAY frame-0 (spawn)
};

// Renderer plugin loaded by sr.dll (shared-state renderer_id).
enum TasRendererId : uint32_t {
    TAS_RENDERER_UNKNOWN   = 0,
    TAS_RENDERER_DIRECTX6  = 1,
    TAS_RENDERER_DIRECTX7  = 2,
    TAS_RENDERER_OPENGL    = 3,
    TAS_RENDERER_GLIDE3X   = 4,
    TAS_RENDERER_SOFTWARE2 = 5,
};

// Selectable riders (shared-state rider_character). The physics differ per
// character, so recordings are stamped with this.
enum TasCharacterId : uint32_t {
    TAS_CHARACTER_UNKNOWN = 0,
    TAS_CHARACTER_KEITH   = 1,
    TAS_CHARACTER_VINCENT = 2,
    TAS_CHARACTER_AKIKO   = 3,
    TAS_CHARACTER_KARL    = 4,
    TAS_CHARACTER_MIKE    = 5,
    TAS_CHARACTER_ULRIKA  = 6,
    TAS_CHARACTER_OTHER   = 7,   // a name outside this table (mod / custom rider)
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
    uint32_t cave2_injecting;       // best-effort diagnostic; gates use a thread-local scope
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
    //   level_epoch:      bumped on every root change, AND once when the root
    //                     stays 0 for ~0.5s (a restart passes through 0
    //                     transiently; a teardown/menu leaves it there). Polled
    //                     by levelscan's background thread every 100 ms, NOT
    //                     from the Cycle hook — the cycle freezes at menus and
    //                     level loads, i.e. exactly across the transition.
    //   level_scan_epoch: the epoch the level-scan thread had observed when it
    //                     last published a CONCRETE level_id.
    //
    // level_id is therefore trustworthy iff level_scan_epoch == level_epoch.
    // When they differ, the context changed and the new track has not been
    // identified yet — the UI must say "resolving", not assert the old level.
    uint32_t level_epoch;
    uint32_t level_scan_epoch;

    // -- Level-detection confidence (DLL-written diagnostic) --
    // Hit counts for the winning track and the runner-up in the last completed
    // scan. A genuinely loaded level references its resource paths pervasively;
    // residue from a level already left is sparse. Exposed because the numbers
    // are the difference between "detected FE" and "guessed FE from one stale
    // string", and that distinction is invisible in level_id alone.
    uint32_t level_scan_best_hits;
    uint32_t level_scan_second_hits;

    // -- Root-null gap diagnostic (DLL-written) --

    // -- Level path (DLL-written, event-driven) --
    // The engine loads each track from loose files under
    // Data/Levels/<Area>/<Category>/<Difficulty>/..., so a file open IS the
    // level-identity event: exact, immediate, and richer than the heap scan
    // (which matches "<area>/Tracks/<diff>" - including practice/Tracks/easy,
    // id 9 - but cannot see Special, Halfpipe or Ramp at all).
    // level_path_gen is bumped AFTER the string is written, so a reader that
    // sees a new generation can already see the path it refers to.
    char     level_path[TAS_LEVEL_PATH_MAX];
    uint32_t level_path_gen;
    // SEQLOCK over the WHOLE level-context group: level_epoch,
    // level_scan_epoch, level_id, level_path, level_path_gen.
    // ODD = a write is in progress, EVEN = stable.
    //
    // A plain store plus a barrier is not enough: the path is a 128-byte array,
    // so a reader in the OTHER PROCESS can observe it half-copied while the
    // counters still read old. Readers must take the sequence, read the group,
    // re-take the sequence, and accept only an unchanged EVEN value.
    //
    // The group is exactly the set of fields a decision is made from, and the
    // DLL's level_scan.hpp is its only runtime writer — every store to any of
    // them goes through publishContext(). level_scan_best_hits /
    // level_scan_second_hits are NOT in the group: they are diagnostics nothing
    // branches on. If a field ever joins the decision, it joins the window too.
    volatile uint32_t level_ctx_seq;

    // The engine's 64-bit elapsed-time delta for this cycle, {lo,hi}, read by
    // cave5 from [esp+0x40] before __ftol truncates it to a tick count. This is
    // the sub-tick phase: everything at tick resolution has already been ruled
    // out by measurement as a bucket predictor (see tas_test bucket-predict).
    // Defer ARM until tick_count reaches this (0 = immediate). Writing the
    // command is a store from another process; cave2 consumes it on a later
    // frame, so the consumption tick — which is what first_moving is measured
    // from — was never actually controlled. See the Rust doc for the numbers.
    // Replay position at which cave2 drops playback_speed to speed_after_handoff,
    // on that exact tick, and asks cave5 to clear the catch-up backlog. CONT's
    // splice handover generalised so a judged PLAY can replay the countdown fast
    // and hand back to 1x exactly where the run becomes worth watching.
    volatile uint32_t speed_handoff_pos;
    volatile float speed_after_handoff;
    // Bumped every time cave2 PROCESSES a replay-starting arm (ARM_PLAY /
    // ARM_CONTINUE), refusals included. The judge uses it to tell this
    // attempt's mode/position from the previous replay's - neither of those
    // fields can answer that on its own. See the Rust doc.
    volatile uint32_t arm_generation;
    // Countdown-gate instrumentation. See the Rust doc: these exist to test
    // whether first_moving is COMPUTABLE at arm time rather than only
    // observable after replaying the whole countdown.
    volatile uint32_t restart_done_tick;  // tick_count when restart_state -> 2
    volatile uint32_t gate_tick;          // tick_count when the boarder first moved
    volatile uint32_t gate_index;         // REC/PLAY index at that moment
    // restart_done_tick latched at THIS attempt's arm, published before
    // arm_generation so the pair can never straddle two attempts.
    volatile uint32_t arm_restart_tick;
    // The GAME's own 16-bit centisecond clock (SG+0x1D5334) at the restart, the
    // arm and the gate. tick_count is OUR counter and the countdown is not
    // compared against it; the drift between the two is what limits the
    // predicted first-moving frame to +/-1. See the Rust doc.
    // tick_count of the most recent LEVEL RESET (the player teleporting back
    // to spawn). This is what the countdown actually starts from;
    // restart_done_tick is only when OUR F5 hold finished. See the Rust doc.
    volatile uint32_t reset_tick;
    volatile uint32_t arm_reset_tick;
    volatile uint32_t gate_reset_tick;
    // Raw bits of the player position at the ARM tick. The boarder SETTLES for
    // a few ticks after the level reset before holding still, so this says how
    // far into the settle the arm landed. See the Rust doc.
    // The engine's QPC-domain clock (10MHz) at the reset, arm and gate frames.
    // The only SUB-TICK quantity available: everything else is tick-quantised
    // and identical across cycles that produce different gates. See Rust doc.
    // f64 bits: engine SECONDS accumulated since the level reset, full
    // precision. [esp+0x40] is a double in seconds that feeds fmul x100 then
    // __ftol; that truncation IS the bucket lottery. See the Rust doc.
    // tick_count and 10MHz clock at the frame F5 is PRESSED. The level resets
    // then, not at the release RESTART_F5_HOLD_FRAMES later. See the Rust doc.
    // Frame-by-frame trace from the F5 press: [tick, x, y, z] raw bits. Every
    // "where does the countdown start" detector so far was a guess that left a
    // one or two tick residual; this records what actually happens instead.
    volatile uint32_t trace_count;
    // [tick_count, x, y, z, now_lo, now_hi] — the clock and the emitted ticks
    // are what the game's own tick rule consumes, so the sub-tick residual can
    // be reconstructed from this rather than found in memory. See Rust doc.
    // [tick, x, y, z, now_lo, now_hi, physics_ptr]. The physics pointer is the
    // reset signal that position cannot provide: a boarder already AT the spawn
    // when the level reloads shows no position change at all. See the Rust doc.
    volatile uint32_t trace[TRACE_FRAMES][7];
    // cave2 CYCLE ordinals at press / arm / gate. tick_count is batched by
    // cave5 (esi added before the cycles run), so it cannot tell cycles apart
    // inside a batch — and first_moving counts cycles. See the Rust doc.
    // The RECORDING's first-moving index; non-zero enables gate-relative input
    // alignment for PLAY. Makes the gate index irrelevant rather than predicted.
    // See the Rust doc.
    volatile uint32_t gate_align_rec;
    volatile uint32_t press_seq;
    volatile uint32_t arm_seq;
    volatile uint32_t gate_seq;
    volatile uint32_t f5_press_tick;
    volatile uint32_t f5_press_qpc_lo;
    volatile uint32_t f5_press_qpc_hi;
    volatile uint32_t secs_since_reset_lo;
    volatile uint32_t secs_since_reset_hi;
    volatile uint32_t arm_secs_lo;
    volatile uint32_t arm_secs_hi;
    volatile uint32_t gate_secs_lo;
    volatile uint32_t gate_secs_hi;
    volatile uint32_t reset_qpc_lo;
    volatile uint32_t reset_qpc_hi;
    volatile uint32_t arm_qpc_lo;
    volatile uint32_t arm_qpc_hi;
    volatile uint32_t gate_qpc_lo;
    volatile uint32_t gate_qpc_hi;
    volatile uint32_t arm_pos_x;
    volatile uint32_t arm_pos_y;
    volatile uint32_t arm_pos_z;
    volatile uint32_t restart_clk;
    volatile uint32_t arm_clk;
    volatile uint32_t gate_clk;
    // 1 while every coordinate capture this session has succeeded. A failed
    // capture still advances the index, leaving a stale hole in the prefix.
    volatile uint32_t capture_ok;
    volatile uint32_t arm_at_tick;
    volatile uint32_t arm_consumed_tick;
    volatile uint32_t clock_delta_lo;
    volatile uint32_t clock_delta_hi;
    // Raw dwords of the player object and its physics sub-object at the ARM
    // and at the GATE — the hunt for the hidden spawn state that makes the
    // first moving coordinate differ with the gate index matched. See Rust.
    volatile uint32_t objsnap_arm_player[OBJSNAP_PLAYER_DWORDS];
    volatile uint32_t objsnap_arm_physics[OBJSNAP_PHYSICS_DWORDS];
    volatile uint32_t objsnap_gate_player[OBJSNAP_PLAYER_DWORDS];
    volatile uint32_t objsnap_gate_physics[OBJSNAP_PHYSICS_DWORDS];
    volatile uint32_t objsnap_player_ok;
    volatile uint32_t objsnap_physics_ok;
    // Clock diagnostics from cave5: raw per-frame tick demand (the wall-clock
    // backlog), the private tick-advance the in-game readers see (f32 bits),
    // and how many times the backlog drain has fired. See the Rust doc.
    volatile int32_t  diag_demand;
    volatile uint32_t diag_tick_advance;
    volatile uint32_t diag_drain_count;
    // Aligned-CONT splice interlock. Written 1 by the controller when the
    // gate-relative watcher has validated the prefix (bit-exact up to
    // min(splice, gate+BUCKET_VALIDATE_WINDOW)). Until then cave5 refuses to
    // run a tick past the aligned splice (parks at 0 ticks/frame) and cave2
    // refuses to splice - so an unjudged or starved-controller prefix can
    // never truncate the recording. Cleared by ARM_CONTINUE (each attempt
    // starts unapproved) and by ClearGateAlign (STOP / RESTART / refusals /
    // auto-stop). Unaligned CONT (gate_align_rec == 0) ignores it entirely.
    volatile uint32_t cont_splice_approved;
    volatile uint32_t pad_v40;  // explicit tail pad (struct is align-8) so the size pin stays honest

    // v41: renderer / x87-precision awareness (wiki: "Why are replays sometimes
    // 0.01s shorter than expected?"). Supreme.exe asks for 24-bit precision
    // (_controlfp(_PC_24, _MCW_PC)); DirectX 6/7 keep it (CW 0x007F) while the
    // OpenGL/Software2 path runs at 53-bit (CW 0x027F), so identical inputs
    // give different physics per renderer. fpu_control_word is the raw x87 CW
    // sampled ON THE GAME THREAD every Supreme::Cycle (per-thread state; the
    // injector thread's word means nothing). renderer_id is the loaded
    // srDD_*.dll (TasRendererId), refreshed by the level-scan worker.
    volatile uint32_t fpu_control_word;
    volatile uint32_t renderer_id;

    // v42: who is riding. The physics depend on the character (a Keith
    // recording does not line up under Vincent) and the stance (the board
    // does not matter), so recordings carry both and a replay warns when the
    // live loadout differs. Read by the level-scan worker from the human
    // Player's Player_Config name / loadout object - see rider_identity.hpp.
    // rider_character: TasCharacterId (0 = not resolved yet);
    // rider_stance: 0 = regular (left-foot icon, the game's default),
    //   1 = goofy (right-foot icon), 0xFFFFFFFF = unknown. Read from the
    //   menu's game-setup object (found by layout, see rider_identity.hpp),
    //   the value the game builds the Player from on every (re)start.
    //   (The stance cannot be switched in-process: writing that dword and
    //   restarting keeps the rider's stance-baked config - the game applies
    //   it only when a level is entered from the menu. Measured 2026-09-02.)
    volatile uint32_t rider_character;
    volatile uint32_t rider_stance;

    // v43: seqlocks for the two published PAIRS (same protocol as
    // level_ctx_seq: odd = writer mid-update, even = stable; the reader
    // re-reads until the sequence is even and unchanged).
    //   rider_seq - (rider_character, rider_stance), written by the level-scan
    //               worker in rider::Refresh.
    //   race_seq  - (race_time_cs, race_start_ts), written on the game thread
    //               in the race timer's Publish.
    // Without them a reader could pair a new character with the previous
    // stance, or a new time with the previous start stamp (codex review
    // 2026-09-03).
    volatile uint32_t rider_seq;
    volatile uint32_t race_seq;

    // v44: which menu screen the game is showing, as its on-screen title
    // ("Main Menu", "Select Character", "Select Board", "TOP5 Attack Settings",
    // "Arcade", ...); empty while a level is running. Captured by the race
    // timer's SR_UIT text hook (no memory scan) and cleared by the in-game
    // clock tick (which is frozen at menus, so the last title stays put there).
    // A plain buffer, not seqlocked: a torn read is a one-frame cosmetic blip
    // in a display string, and it self-heals on the next poll.
    char menu_screen[TAS_MENU_SCREEN_MAX];

    // v45: the focused menu item's index within the current page container
    // (0xFFFFFFFF = no menu / unreadable). Read from the menu object by the
    // level-scan worker via UI_Menu::Get_Active_Component; see menu_state.hpp.
    volatile uint32_t menu_selector;

    // v46: the MENU DOCUMENT - the current page's items with their visible
    // labels and stable ids, as compact JSON, e.g.
    //   {"screen":"ID_ARCADE_MENU","sel":0,"items":[
    //     {"label":"Time Attack","id":"ID_ARCADE_TIME_ATTACK_SEQUENCE","en":true,"vis":true}, ...]}
    // "sel" is the index into "items" of the focused one (null = none) and
    // equals menu_selector. Empty (menu_doc[0] == 0) while a level runs. Read
    // by FIELD from the UIT objects (see menu_state.hpp), written by the
    // level-scan worker only when it changes, under menu_seq (odd = mid-write)
    // so a reader never sees a half-written document.
    volatile uint32_t menu_seq;
    char menu_doc[TAS_MENU_DOC_MAX];

    // v47: the MENU COMMAND channel (agent -> DLL). The agent writes kind +
    // target, then bumps menu_cmd_seq. The DLL consumes it on the MENU THREAD
    // (a mid-hook at UI_Menu::Execute, per frame while a menu is shown) and
    // executes it through the game's own entry points (Request_Focus, then
    // UI_Menu::Trigger / Up / Down / ...), writes menu_cmd_result, and sets
    // menu_cmd_ack = menu_cmd_seq (result first, ack last). One agent at a
    // time; the reader polls menu_cmd_ack. See caves/menu_state.hpp.
    volatile uint32_t menu_cmd_seq;
    volatile uint32_t menu_cmd_kind;      // TAS_MENU_CMD_*
    char menu_cmd_target[TAS_MENU_CMD_TARGET_MAX];
    // v48: the page id the agent read the target from ("ID_ARCADE_MENU");
    // the command is refused with STALE_PAGE if the page moved on. Empty =
    // no check. Written before menu_cmd_seq is bumped.
    char menu_cmd_screen[TAS_MENU_SCREEN_MAX];
    volatile uint32_t menu_cmd_ack;
    volatile uint32_t menu_cmd_result;    // TAS_MENU_RESULT_*
};

// The C++ and Rust views of this struct MUST agree byte-for-byte — they map the
// same shared memory from two processes. Rust pins the same number
// (tas_shared/src/lib.rs, `size_of::<TasSharedState>()`), but until now only the
// Rust side would catch a mismatch, and only if someone ran the Rust tests. Pin
// it here too so a layout change fails the DLL build immediately.
// Bump TAS_SHARED_VERSION whenever this number changes.
static_assert(sizeof(TasSharedState) == 1667776,
              "TasSharedState layout changed: bump TAS_SHARED_VERSION and update "
              "the Rust size pin in tas_shared/src/lib.rs");

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
        state->rider_stance = 0xFFFFFFFFu;         // unknown until the setup object is read
        state->menu_selector = 0xFFFFFFFFu;        // no menu selection until the menu publishes one
        state->menu_seq = 0;
        state->menu_doc[0] = 0;                    // no menu document until the menu publishes one
        state->menu_cmd_seq = 0;                   // v47: nothing pending (a command from a previous
        state->menu_cmd_kind = 0;                  //      DLL life is never replayed)
        state->menu_cmd_target[0] = 0;
        state->menu_cmd_screen[0] = 0;
        state->menu_cmd_ack = 0;
        state->menu_cmd_result = 0;
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
        state->menu_fps_cap = 20;
        // Start EQUAL: at init nothing has been identified yet, and level_id is
        // already 0xFFFFFFFF, so "trustworthy but unknown" is the honest state
        // (we are at the menu / not yet scanned) rather than "resolving".
        state->level_epoch = 0;
        state->level_scan_epoch = 0;
        state->level_scan_best_hits = 0;
        state->level_scan_second_hits = 0;
        state->level_path[0] = 0;
        state->level_path_gen = 0;
        state->level_ctx_seq = 0;
        state->rider_seq = 0;
        state->race_seq = 0;
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
