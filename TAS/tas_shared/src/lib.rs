use std::sync::atomic::{AtomicU32, Ordering};

pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";
use std::sync::atomic::compiler_fence;

pub const TAS_SHARED_VERSION: u32 = 25; // +restart_done_tick, gate_tick, gate_index (is the gate predictable?)
pub const TAS_LEVEL_PATH_MAX: usize = 128;
pub const TAS_MAX_TICKS: usize = 65536;
pub const TAS_MAX_SEGMENTS: usize = 32;
pub const TAS_LOG_RING_SIZE: usize = 64;
pub const TAS_LOG_ENTRY_SIZE: usize = 120;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasCommand {
    Idle = 0,
    ArmRec = 1,
    ArmPlay = 2,
    Stop = 3,
    ArmContinue = 4,
    Restart = 5,
    /// PROTOTYPE: capture a writable-memory snapshot at the current frame.
    Snapshot = 6,
    /// PROTOTYPE: restore the last snapshot (instant CONT rewind).
    Restore = 7,
    /// PROTOTYPE: arm a snapshot at the next PLAY frame-0 (the spawn) — captures
    /// without disturbing the arm timing that selects the bucket.
    SnapshotAtSpawn = 8,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasMode {
    Off = 0,
    Rec = 1,
    Play = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasLogSeverity {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

/// A single log ring entry (matches C++ TasLogEntry)
#[repr(C)]
pub struct TasLogEntry {
    pub sequence: u32,
    pub severity: u32,
    pub text: [u8; TAS_LOG_ENTRY_SIZE],
}

/// Hook performance counters (cycles measured with __rdtsc).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TasHookPerfCounter {
    pub calls: u64,
    pub cycles_total: u64,
    pub cycles_max: u64,
}

impl TasLogEntry {
    pub fn text_str(&self) -> &str {
        let len = self
            .text
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(TAS_LOG_ENTRY_SIZE);
        std::str::from_utf8(&self.text[..len]).unwrap_or("<invalid utf8>")
    }

    pub fn severity_enum(&self) -> TasLogSeverity {
        match self.severity {
            0 => TasLogSeverity::Debug,
            1 => TasLogSeverity::Info,
            2 => TasLogSeverity::Warn,
            3 => TasLogSeverity::Error,
            _ => TasLogSeverity::Info,
        }
    }
}

/// A segment boundary record (matches C++ TasSegmentBoundary, 8 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TasSegmentBoundary {
    pub frame: u32,            // Frame number where this segment starts
    pub input_log_offset: u32, // Offset into input_log for this segment
}

/// Input mask bit definitions (matches C++ TasInputBit)
pub mod input_bits {
    pub const LEFT: u8 = 0x01;
    pub const RIGHT: u8 = 0x02;
    pub const UP: u8 = 0x04;
    pub const DOWN: u8 = 0x08;
    pub const JUMP: u8 = 0x10;
    pub const SHIFT: u8 = 0x20;

    pub const ALL: &[(u8, &str, &str)] = &[
        (LEFT, "L", "Left"),
        (RIGHT, "R", "Right"),
        (UP, "U", "Up"),
        (DOWN, "D", "Down"),
        (JUMP, "J", "Jump"),
        (SHIFT, "S", "Shift"),
    ];
}

/// Must match the C++ TasSharedState layout exactly.
/// All fields are naturally aligned (u32/f32 = 4 bytes), so repr(C) suffices.
#[repr(C)]
pub struct TasSharedState {
    pub version: u32,

    // Command region (UI writes)
    pub command: u32,
    pub inject_mode: u32,
    pub force_fixed_tick: u32,
    pub force_direct: u32,
    pub self_capture: u32,
    pub use_rec_msg_args: u32,
    pub input_source: u32,
    pub continue_from_frame: u32,

    // Status region (DLL writes)
    pub mode: u32,
    pub recorded_count: u32,
    pub playback_pos: u32,
    pub prev_mask: u32,
    pub cave2_injecting: u32,
    pub player_x: f32,
    pub player_y: f32,
    pub player_z: f32,
    pub max_drift_x: f32,
    pub max_drift_z: f32,

    // Diagnostics
    pub bb3b10_call_count: u32,
    pub handler_block_count: u32,
    pub frame_count: u32,
    pub event_count: u32,
    pub bb3b10_block_count: u32,

    // Hook performance counters (DLL writes, UI/tests read)
    pub perf_cave2: TasHookPerfCounter,
    pub perf_cave5: TasHookPerfCounter,
    pub perf_cave1c_down: TasHookPerfCounter,
    pub perf_cave1c_up: TasHookPerfCounter,
    pub perf_cave1d: TasHookPerfCounter,
    pub perf_replay_capture: TasHookPerfCounter,

    // Hook status
    pub cave2_hooked: u32,
    pub cave1c_hooked: u32,
    pub cave1d_hooked: u32,
    pub cave5_hooked: u32,
    pub replay_capture_hooked: u32,

    // Runtime pointers (DLL internal, exposed for diagnostics)
    pub replay_ptr: u32,
    pub player_ptr: u32,

    // Variable speed playback (1.0 = normal, 0.5 = half, 2.0 = double)
    pub playback_speed: f32,

    // In-process restart state machine (DLL internal)
    // 0=idle, 1=F5 pressed (waiting frames), 2=F5 released (done)
    pub restart_state: u32,
    pub restart_frames_held: u32,

    // Telemetry (DLL writes, UI reads)
    pub prev_player_x: f32,
    pub prev_player_y: f32,
    pub prev_player_z: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub speed: f32, // Squared speed (XZ plane) — UI should sqrt for display
    pub tick_count: u32,

    // Segment fields (DLL writes, UI reads)
    pub segment_index: u32,
    pub segment_start_frame: u32,
    pub segment_count: u32,
    pub snapshot_size: u32,
    pub snapshot_flags: u32,
    pub snapshot_buffer_ptr: u32,
    pub snapshot_buffer_capacity: u32,
    pub segment_boundaries: [TasSegmentBoundary; TAS_MAX_SEGMENTS],

    // Rotation telemetry (DLL writes, UI reads)
    // 3x3 row-major rotation matrix from player+0x104..+0x124
    pub rotation_matrix: [f32; 9],

    pub input_log: [u8; TAS_MAX_TICKS],
    pub rec_coords: [[f32; 3]; TAS_MAX_TICKS],
    pub play_coords: [[f32; 3]; TAS_MAX_TICKS],

    // Log ring buffer (DLL writes, UI reads)
    pub log_write_seq: u32,
    pub log_ring: [TasLogEntry; TAS_LOG_RING_SIZE],

    // CONT splice timing (DLL writes, harness/UI reads) — measures the
    // "resume off by a few frames" skew (Problem B). `frame_count` is stamped
    // when the CONT replay first advances and again at the PLAY→REC splice; the
    // DELTA is the number of game-frames the catch-up replay took to consume
    // the prefix. Its run-to-run spread is the yield jitter. (frame_count is
    // free-running — never reset on F5 — so only the delta is meaningful.)
    pub cont_replay_start_fc: u32,
    pub cont_splice_fc: u32,

    // CONT resume speed (UI writes, DLL reads). Staged before a CONT; the DLL
    // applies it to `playback_speed` ATOMICALLY at the splice instant so the
    // resumed recording never fast-forwards at the catch-up rate while the UI's
    // poll-driven speed-restore lags behind (the Problem B fix). 0.0 = unset
    // (DLL leaves the speed as-is — backward compatible).
    pub cont_resume_speed: f32,

    // CONT clock-backlog reset (cave2 sets at the splice, cave5 consumes on the
    // next tick). When set, cave5 advances the game-time accumulator
    // (clockObj->+0x0C, reached via the hook's ebp) to "now" by adding
    // raw_demand * native_tick_advance — clearing the catch-up backlog WITHOUT
    // processing the backlog ticks, so the resume is frame-exact at full speed
    // (no end-of-replay deceleration). 0 = idle.
    pub cont_reset_pending: u32,

    /// Game-state awareness: 0 = main menu, 1 = in-game (race/level). DLL writes
    /// it each frame from `Supreme.exe + 0x8895C` (RE'd).
    pub game_in_game: u32,

    /// Current track, detected by the DLL's in-process heap scan (majority-vote):
    /// `0..8 = area*3 + difficulty` (area 0=Forest,1=Alpine,2=Village; diff
    /// 0=Easy,1=Medium,2=Hard). `u32::MAX` = unknown / menu. See the DLL's
    /// `level_scan.hpp`.
    pub level_id: u32,

    /// On-screen player race time in centiseconds, read by the DLL from the HUD
    /// text line (SR_UIT `Append_Text`). Exact + map-agnostic. `u32::MAX` = not
    /// racing / unknown.
    pub race_time_cs: u32,

    /// The 16-bit game clock value captured at the gate cross (= clock −
    /// race_time); constant during a run; the F5 spawn-lottery metric.
    /// `u32::MAX` = unknown.
    pub race_start_ts: u32,

    /// Clock-phase pin config: 0 = natural wall-clock tick scheduling (the F5
    /// bucket lottery), nonzero = cave5 pins the OFF-mode in-game tick pattern
    /// to a canonical [1,1,0] cycle whose phase resets at each in-process
    /// restart — every restart replays the same spawn-settle schedule, so the
    /// F5 bucket is machine/fps/OS-independent. DLL defaults this ON.
    pub clock_pin_enabled: u32,

    /// Clock-phase pin internal state: current position in the [1,1,0] cycle.
    /// DLL-written; reset by CMD_RESTART.
    pub clock_pin_phase: u32,

    /// Test hook: when nonzero, the DLL suppresses ALL BB3B10 arg4 calibration
    /// and injects this exact value as the event Time.hi (arg4). The
    /// steer-impact regression test sets this to isolate the inject path —
    /// injected steering then lands only if the event Time is correct without
    /// a keypress. 0 = normal (live Kernel::Time::Current stamp).
    pub test_arg4_override: u32,

    /// DLL-written at each injection batch: where the injected event's Time
    /// stamp came from. 0 = no injection yet, 1 = Kernel::Time::Current (the
    /// proper, focus-independent path), 2 = keypress-calibrated fallback,
    /// 3 = test_arg4_override forced. steer-impact asserts 1 in its live
    /// phase so the proper mechanism can't silently regress to the fallback.
    pub arg4_source: u32,

    /// UI-written: 1 while a Continue cycle is in flight (from CONT start,
    /// before the F5 restart, until the bucket aligns). The DLL blocks the
    /// real key handler whenever this is set — covering the OFF-mode spawn
    /// countdown the mode-based block misses, so live input can't perturb the
    /// bucket. Cleared the instant the bucket aligns (catch-up PLAY / resumed
    /// REC are already handler-blocked by mode, and post-splice REC must see
    /// live input). See cave1c.
    pub cont_suppress_input: u32,

    /// DLL-written: monotonic count of gdi32!SwapBuffers presents. Lets a
    /// poller measure the live present rate (menu vs in-game).
    pub present_count: u32,
    /// UI/config: menu present-rate cap. 0 = OFF (count only, no throttle);
    /// N = cap presents to N fps while the engine cycle is frozen (menu/pause).
    /// Default 34. The static main menu animates one frame per present and
    /// sr.dll's limiter is Sleep-based, so the 1 ms system timer the TAS tooling
    /// raises doubles it to ~68 fps (the "2x menu video"); capping to ~34
    /// restores native. Gameplay (timer-independent accumulator) + CONT (gated
    /// by cont_suppress_input) are untouched.
    pub menu_fps_cap: u32,

    /// Bumped by the DLL whenever the engine's root object
    /// (`[SG+0x1D5450]`) changes, and once when it stays NULL for ~0.5s. That
    /// root SURVIVES an F5 restart but is reallocated on quit-to-menu /
    /// menu-demo load / track switch, so a change is the only trustworthy "the
    /// level under you was swapped" event. A restart passes through NULL
    /// transiently, which is why only a SUSTAINED null counts as a teardown.
    pub level_epoch: u32,
    /// The `level_epoch` the scan thread had observed when it last published a
    /// concrete `level_id`.
    ///
    /// `level_id` is trustworthy **iff `level_scan_epoch == level_epoch`**. When
    /// they differ the context changed and the new track has not been identified
    /// yet — callers must treat the level as UNRESOLVED rather than asserting
    /// the previous one. See [`level_is_resolved`].
    pub level_scan_epoch: u32,

    /// Hit count for the winning track in the last completed scan, and for the
    /// runner-up. A loaded level references its resource paths pervasively;
    /// residue from a level already left is sparse — so these separate
    /// "detected" from "guessed from one stale string", a distinction that is
    /// invisible in `level_id` alone.
    pub level_scan_best_hits: u32,
    pub level_scan_second_hits: u32,
    /// The engine loads each track from loose files under
    /// `Data/Levels/<Area>/<Category>/<Difficulty>/...`, so a file open IS the
    /// level-identity event — exact and immediate, and richer than the heap
    /// scan, which only matches `<area>/Tracks/<diff>` and so cannot see
    /// Practice, Special, Halfpipe or Ramp at all.
    pub level_path: [u8; TAS_LEVEL_PATH_MAX],
    /// Bumped AFTER `level_path` is written, so a reader that sees a new
    /// generation can already see the path it refers to.
    pub level_path_gen: u32,
    /// Seqlock over the level-context group (`level_epoch`,
    /// `level_scan_epoch`, `level_id`, `level_path`). ODD = write in progress.
    ///
    /// `AtomicU32` rather than a plain `u32` read volatile. The seqlock's whole
    /// correctness rests on the payload reads staying BETWEEN the two sequence
    /// reads, and `read_volatile` does not give that: it promises only that the
    /// access is not elided or reordered against other volatile accesses, which
    /// is a statement about the compiler, not about the memory model. An
    /// `Acquire` load plus a fence before the recheck is the real thing, and on
    /// x86 it compiles to the same two `mov`s. Layout is identical (4 bytes,
    /// align 4) so the C++ side stays a plain `volatile uint32_t` bumped with
    /// `InterlockedIncrement`.
    pub level_ctx_seq: AtomicU32,

    /// The game`s own 64-bit elapsed-time delta for the current cycle, as cave5
    /// sees it: the {lo,hi} pair the engine computed at EXE+0x25C6E and left at
    /// [esp+0x40] before __ftol truncated it into a tick count.
    ///
    /// WHY: the F5 bucket is decided sub-tick. bucket-predict proved the spawn
    /// state is CONSTANT across buckets and that the arm phase, measured to exact
    /// tick granularity (0-tick measurement window), does not determine the
    /// bucket either. Everything at tick resolution has been ruled out by
    /// measurement. This is the pre-truncation quantity — the fraction __ftol
    /// throws away — and `lo` is the sub-unit component (the BB3B10 arg4 work
    /// found that FLOORING lo to 0 is what made injected stamps bit-exact).
    /// Defer ARM until the physics tick counter reaches this value (0 = consume
    /// immediately, the historical behaviour).
    ///
    /// WHY. Writing CMD_ARM_REC is a shared-memory store from another process;
    /// cave2 CONSUMES it on some later Supreme::Cycle. So "armed at tick +40"
    /// only ever meant "written at +40" — consumption could be +40 or +41, and
    /// first_moving is measured from consumption. With the gate itself landing on
    /// G or G+1, the difference of two +/-1 quantities produces exactly three
    /// adjacent values with the middle one commonest, which is precisely the
    /// observed 258 x2 / 259 x14 / 260 x4. Scheduling the arm INSIDE the game
    /// thread removes one of the two.
    /// Replay position at which the DLL drops playback_speed to
    /// `speed_after_handoff`, atomically, on that exact tick. 0 = no handoff.
    ///
    /// This is CONT's splice mechanism generalised. A bucket-matched PLAY wants
    /// to replay the countdown fast — the judge cannot rule until the replay
    /// passes first_moving, and at 1x that is ~2.6s of a stationary boarder on
    /// every attempt including the failures — and then hand back to normal speed
    /// the instant the run becomes worth watching. Doing that from the UI thread
    /// has no bound: cave5 may already have issued a batch of up to
    /// CAVE5_PER_FRAME_TICK_CAP ticks, and the poll adds scheduler latency on
    /// top. Doing it here is exact.
    pub speed_handoff_pos: u32,
    /// Speed to assert at the handoff (0 = leave the speed alone).
    pub speed_after_handoff: f32,
    /// Bumped by cave2 every time it PROCESSES an arm that starts a replay
    /// (ARM_PLAY / ARM_CONTINUE), including one it refuses.
    ///
    /// The judge needs to know whether the mode and position it is reading
    /// describe this attempt or the previous one, and neither of those fields
    /// can answer it: mode is transient (a short replay at 256x can begin and
    /// end between two polls, leaving every later poll reading OFF), and the
    /// position still holds the previous replay's final value until the arm
    /// resets it. This counter is monotonic and changes exactly once per arm,
    /// so "has it moved since I armed" is a question with a durable answer.
    ///
    /// Counting REFUSED arms too is deliberate: a refusal leaves the mode OFF
    /// forever, which is precisely the state that used to spin.
    pub arm_generation: u32,
    /// tick_count when the in-process F5 restart completed (restart_state -> 2).
    ///
    /// The three fields below exist to answer one question: is `first_moving`
    /// COMPUTABLE at arm time instead of observable only after replaying the
    /// whole countdown? The model says the countdown is a fixed number of ticks
    /// from the restart, and `first_moving` is measured from the ARM - so it
    /// should be `(restart_done_tick + K) - arm_consumed_tick`, with every term
    /// known before a single tick is replayed.
    pub restart_done_tick: u32,
    /// tick_count at the first captured frame whose position differs from the
    /// session's frame 0 — i.e. when the countdown gate actually fired.
    pub gate_tick: u32,
    /// The REC/PLAY index at that same moment. This is `first_moving` stamped
    /// by the DLL, rather than re-derived from coordinates afterwards.
    pub gate_index: u32,
    pub arm_at_tick: u32,
    /// The tick at which ARM was actually consumed (diagnostic).
    pub arm_consumed_tick: u32,
    pub clock_delta_lo: u32,
    pub clock_delta_hi: u32,
}

/// How many times to retry a torn level-context read before giving up.
///
/// The writer's critical section is a few hundred bytes of stores, and it only
/// opens when the level context actually CHANGES — the DLL skips publications
/// that would write identical values. So the window is both short and rare, and
/// a reader that loses 64 races in a row is not racing: the producer is wedged
/// or dead. Giving up returns UNKNOWN, which every caller treats as "match
/// nothing", so exhausting the bound fails closed rather than spinning a UI
/// frame forever.
const LEVEL_CTX_RETRIES: usize = 64;

/// Seqlock acquire over the level-context group (`level_epoch`,
/// `level_scan_epoch`, `level_id`, `level_path`).
///
/// `read` runs on a possibly-torn group — that is expected; the value is only
/// handed back if the sequence was EVEN before it and UNCHANGED after, which is
/// exactly the window in which no write was in flight. So `read` must not act on
/// what it sees, only collect it.
///
/// `None` means no clean window was obtained: the writer is wedged mid-update
/// (sequence stuck odd — e.g. the game crashed between the two increments) or
/// the retry bound ran out. Both are "unknown", never "assume the last value".
///
/// WHAT THIS IS AND IS NOT. The sequence is a real atomic and the ordering
/// around it is real. The PAYLOAD is not: it is read with `read_volatile` from
/// memory another PROCESS writes, which Rust's memory model does not describe at
/// all — formally a data race, and no amount of `Ordering` fixes that, because
/// the writer is outside the model. What makes it work is the target, not the
/// abstract machine: on x86 aligned byte and word accesses are indivisible,
/// loads are not reordered with loads, the mapping is cache-coherent, and the
/// DLL's `InterlockedIncrement` is a full barrier. So this is a seqlock ON X86
/// over a cross-process mapping. Do not read it as a portable one.
///
/// Freshness is also NOT provided. A clean read is coherent at the instant it
/// was taken and can be stale by the time the caller acts on it — which is why
/// the UI re-reads immediately before writing to the live buffer rather than
/// trusting its frame-start snapshot (see `stop_active_session_for_load`).
fn with_level_context<T>(state: &TasSharedState, read: impl Fn() -> T) -> Option<T> {
    for _ in 0..LEVEL_CTX_RETRIES {
        let s1 = state.level_ctx_seq.load(Ordering::Acquire);
        if s1 & 1 != 0 {
            std::hint::spin_loop();
            continue; // writer mid-update
        }
        let value = read();
        // Keep the payload reads above the recheck. Without this they may sink
        // below the second load, and then the comparison proves nothing about
        // what was actually read.
        std::sync::atomic::fence(Ordering::Acquire);
        if state.level_ctx_seq.load(Ordering::Relaxed) == s1 {
            return Some(value);
        }
        std::hint::spin_loop(); // torn: the group changed under us
    }
    None
}

/// Read the group's identity half. Caller must be inside [`with_level_context`].
///
/// `None` = the context changed and the scan has not re-identified the track,
/// so `level_id` still physically holds the PREVIOUS one and must not be used.
/// Checking `level_id != 0xFFFFFFFF` is NOT equivalent: the scan publishes
/// unknown for transient reasons unrelated to a level change, and a stale id is
/// a perfectly concrete number.
fn read_identity(state: &TasSharedState) -> Option<u32> {
    // SAFETY: plain u32s in a shared mapping written by the DLL's threads.
    // Volatile so the compiler cannot cache them across the sequence loads.
    unsafe {
        let epoch = std::ptr::read_volatile(&state.level_epoch);
        let scan_epoch = std::ptr::read_volatile(&state.level_scan_epoch);
        let id = std::ptr::read_volatile(&state.level_id);
        (epoch == scan_epoch).then_some(id)
    }
}

/// A coherent read of the current track: `Some(level_id)` only when that id was
/// identified in the context we are still in, `None` while unresolved.
///
/// Reading `level_scan_epoch == level_epoch` and THEN reading `level_id`
/// separately is not sound — the level can be swapped between the two, giving
/// "resolved" plus the previous track's id. Both now happen inside one seqlock
/// window, so either the whole group is from a single publication or the read is
/// rejected.
pub fn resolved_level_id(state: &TasSharedState) -> Option<u32> {
    with_level_context(state, || read_identity(state)).flatten()
}

/// A coherent snapshot of the level context: `(level_id, level_path)` together.
///
/// The path is the harder half: 128 bytes that another process can be halfway
/// through rewriting, so no amount of care about the counters alone makes it
/// safe. Same window as [`resolved_level_id`], plus the bytes.
///
/// Returns `None` while unresolved OR while the writer is mid-update; a caller
/// that cannot get a clean read must treat the level as unknown, never guess.
pub fn level_context(state: &TasSharedState) -> Option<(u32, String)> {
    // Collect into a plain buffer inside the window; decode (which allocates)
    // outside it, so a retry never pays for a String it is about to discard.
    let snapshot = with_level_context(state, || {
        let id = read_identity(state)?;
        let mut path = [0u8; TAS_LEVEL_PATH_MAX];
        for (i, b) in path.iter_mut().enumerate() {
            // SAFETY: as read_identity — shared mapping, written by the DLL.
            *b = unsafe { std::ptr::read_volatile(&state.level_path[i]) };
        }
        Some((id, path))
    })
    .flatten();

    let (id, path) = snapshot?;
    let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
    Some((id, String::from_utf8_lossy(&path[..end]).into_owned()))
}

/// Whether the current track is known. Prefer [`resolved_level_id`] when you
/// also need the id — this cannot express "resolved, and it is X" atomically.
pub fn level_is_resolved(state: &TasSharedState) -> bool {
    resolved_level_id(state).is_some()
}

pub const ARG4_SOURCE_NONE: u32 = 0;
pub const ARG4_SOURCE_TIME_CURRENT: u32 = 1;
pub const ARG4_SOURCE_CALIBRATED: u32 = 2;
pub const ARG4_SOURCE_OVERRIDE: u32 = 3;

impl TasSharedState {
    pub fn mode_enum(&self) -> TasMode {
        match self.mode {
            1 => TasMode::Rec,
            2 => TasMode::Play,
            _ => TasMode::Off,
        }
    }

    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            1 => "REC",
            2 => "PLAY",
            _ => "OFF",
        }
    }

    /// Read log entries newer than `after_seq`. Returns (entries, new_cursor).
    /// The cursor should start at 0 and be updated with each call.
    pub fn read_log_entries(&self, after_seq: u32) -> (Vec<(u32, TasLogSeverity, String)>, u32) {
        let write_seq = self.log_write_seq;
        if write_seq == 0 || after_seq >= write_seq {
            return (Vec::new(), after_seq);
        }

        // Only look at the last TAS_LOG_RING_SIZE entries
        let start = write_seq.saturating_sub(TAS_LOG_RING_SIZE as u32);
        let effective_start = start.max(after_seq);

        let mut entries = Vec::new();
        for seq in effective_start..write_seq {
            let idx = (seq % TAS_LOG_RING_SIZE as u32) as usize;
            let entry = &self.log_ring[idx];
            // Sequence in entry is seq+1 (0 means unused)
            if entry.sequence == seq + 1 {
                entries.push((
                    entry.sequence,
                    entry.severity_enum(),
                    entry.text_str().to_string(),
                ));
            }
        }
        (entries, write_seq)
    }

    pub fn reset_hook_perf_counters(&mut self) {
        self.perf_cave2 = TasHookPerfCounter::default();
        self.perf_cave5 = TasHookPerfCounter::default();
        self.perf_cave1c_down = TasHookPerfCounter::default();
        self.perf_cave1c_up = TasHookPerfCounter::default();
        self.perf_cave1d = TasHookPerfCounter::default();
        self.perf_replay_capture = TasHookPerfCounter::default();
    }
}

// --- Platform-specific shared memory client ---

#[cfg(windows)]
mod platform {
    use super::*;
    use std::ffi::CString;

    // Raw Win32 FFI — avoids windows-sys version churn
    type Handle = *mut std::ffi::c_void;
    const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;

    extern "system" {
        fn OpenFileMappingA(desired_access: u32, inherit_handle: i32, name: *const u8) -> Handle;
        fn MapViewOfFile(
            file_mapping: Handle,
            desired_access: u32,
            offset_high: u32,
            offset_low: u32,
            bytes_to_map: usize,
        ) -> *mut std::ffi::c_void;
        fn UnmapViewOfFile(base_address: *const std::ffi::c_void) -> i32;
        fn CloseHandle(handle: Handle) -> i32;
    }

    /// Opens the named shared memory created by TAS_Helper.dll.
    pub struct TasSharedMemoryClient {
        handle: Handle,
        ptr: *mut TasSharedState,
    }

    unsafe impl Send for TasSharedMemoryClient {}
    unsafe impl Sync for TasSharedMemoryClient {}

    impl TasSharedMemoryClient {
        pub fn open() -> Result<Self, String> {
            let name = CString::new(TAS_SHARED_MEMORY_NAME).unwrap();
            unsafe {
                let handle = OpenFileMappingA(FILE_MAP_ALL_ACCESS, 0, name.as_ptr() as *const u8);
                if handle.is_null() {
                    return Err("OpenFileMappingA failed (is TAS_Helper.dll loaded?)".into());
                }

                let view = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 0);
                if view.is_null() {
                    CloseHandle(handle);
                    return Err("MapViewOfFile failed".into());
                }
                let ptr = view as *mut TasSharedState;

                let version = (*ptr).version;
                if version != TAS_SHARED_VERSION {
                    UnmapViewOfFile(ptr as *const _);
                    CloseHandle(handle);
                    return Err(format!(
                        "Version mismatch: expected {}, got {}",
                        TAS_SHARED_VERSION, version
                    ));
                }

                Ok(Self { handle, ptr })
            }
        }

        pub fn state(&self) -> &TasSharedState {
            unsafe { &*self.ptr }
        }

        pub fn state_mut(&mut self) -> &mut TasSharedState {
            unsafe { &mut *self.ptr }
        }

        pub fn send_command(&mut self, cmd: TasCommand) {
            // Enforce proven zero-drift config before arming REC/PLAY
            if matches!(
                cmd,
                TasCommand::ArmRec | TasCommand::ArmPlay | TasCommand::ArmContinue
            ) {
                unsafe {
                    let fft_ptr = std::ptr::addr_of_mut!((*self.ptr).force_fixed_tick);
                    std::ptr::write_volatile(fft_ptr, 0);
                }
            }
            unsafe {
                let cmd_ptr = std::ptr::addr_of_mut!((*self.ptr).command);
                std::ptr::write_volatile(cmd_ptr, cmd as u32);
            }
        }

        /// Volatile read of mode (poll-hot field written by DLL).
        pub fn mode_volatile(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).mode);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Volatile read of frame_count (poll-hot field written by DLL).
        pub fn frame_count_volatile(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).frame_count);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Volatile read of tick_count (poll-hot field written by DLL).
        ///
        /// This is the PHYSICS tick counter (cave5 emits it per frame), not
        /// the render-frame counter. Anything asking "is the game simulating
        /// faster than real time?" has to read this one: fast-forward means
        /// more ticks per frame, and frame_count cannot see that.
        pub fn tick_count_volatile(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).tick_count);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Volatile read of playback_pos (poll-hot field written by DLL).
        pub fn playback_pos_volatile(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).playback_pos);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Volatile read of recorded_count (poll-hot field written by DLL).
        pub fn recorded_count_volatile(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).recorded_count);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Read the restart state machine status (0=idle, 1=in progress, 2=done).
        pub fn restart_state(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).restart_state);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Reset restart state to idle (call after restart completes).
        pub fn reset_restart_state(&mut self) {
            unsafe {
                let ptr = std::ptr::addr_of_mut!((*self.ptr).restart_state);
                std::ptr::write_volatile(ptr, 0);
            }
        }

        pub fn reset_hook_perf_counters(&mut self) {
            self.state_mut().reset_hook_perf_counters();
        }
    }

    impl Drop for TasSharedMemoryClient {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    UnmapViewOfFile(self.ptr as *const _);
                }
                if !self.handle.is_null() {
                    CloseHandle(self.handle);
                }
            }
        }
    }
}

#[cfg(not(windows))]
mod platform {
    use super::*;

    /// Stub shared memory client for non-Windows platforms (build check only).
    pub struct TasSharedMemoryClient {
        state: Box<TasSharedState>,
    }

    unsafe impl Send for TasSharedMemoryClient {}
    unsafe impl Sync for TasSharedMemoryClient {}

    impl TasSharedMemoryClient {
        pub fn open() -> Result<Self, String> {
            Err("Shared memory is only supported on Windows".into())
        }

        /// Create a heap-backed stub client for testing (no shared memory needed).
        pub fn new_stub() -> Self {
            Self {
                state: zeroed_boxed(),
            }
        }

        pub fn state(&self) -> &TasSharedState {
            &self.state
        }

        pub fn state_mut(&mut self) -> &mut TasSharedState {
            &mut self.state
        }

        pub fn send_command(&mut self, cmd: TasCommand) {
            // Mirror fft=0 policy from Windows implementation
            if matches!(
                cmd,
                TasCommand::ArmRec | TasCommand::ArmPlay | TasCommand::ArmContinue
            ) {
                self.state.force_fixed_tick = 0;
            }
            self.state.command = cmd as u32;
        }

        pub fn restart_state(&self) -> u32 {
            0
        }

        pub fn reset_restart_state(&mut self) {}

        pub fn reset_hook_perf_counters(&mut self) {
            self.state.reset_hook_perf_counters();
        }
    }
}

pub use platform::TasSharedMemoryClient;

/// Heap-allocate a zeroed TasSharedState (avoids stack overflow for ~1.6MB struct).
/// Intended for tests across all crates in the workspace.
pub fn zeroed_boxed() -> Box<TasSharedState> {
    unsafe { Box::<TasSharedState>::new_zeroed().assume_init() }
}

/// Track identification shared by tas_ui (the status chip) and tas_test (the
/// pre-flight guard below). Single source of truth for the `level_id` encoding.
pub mod level {
    /// The nine main Time-Attack Tracks, indexed by `level_id` = area*3 +
    /// difficulty (area 0=Forest, 1=Alpine, 2=Village; diff 0=Easy, 1=Medium,
    /// 2=Hard).
    pub const CODES: [&str; 9] = ["FE", "FM", "FH", "AE", "AM", "AH", "VE", "VM", "VH"];

    /// Level code from the DLL's published `level_id`. `None` = unknown: either
    /// the menu, or a mode that is not one of the nine Tracks (Practice,
    /// Halfpipe, ...). The DLL publishes 0xFFFFFFFF for all of those.
    pub fn code_from_id(level_id: u32) -> Option<&'static str> {
        CODES.get(level_id as usize).copied()
    }

    /// The level a `.tasrec` declares via its filename, e.g.
    /// `FE-tremendous.tasrec` -> `FE`. Recordings are named `<CODE>-<name>`, so
    /// the file itself says which track it can be replayed on. `None` when the
    /// name carries no recognised code.
    pub fn code_from_recording_name(path: &str) -> Option<&'static str> {
        let stem = path
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(path);
        let head = stem.split(['-', '_', '.']).next().unwrap_or("");
        CODES.iter().find(|c| c.eq_ignore_ascii_case(head)).copied()
    }

    /// Pre-flight check: can `recording_path` be replayed on the live track?
    ///
    /// Replaying a recording on the wrong track is not a subtle failure — the
    /// spawn is somewhere else entirely, so the harness's start-position matcher
    /// can never hit its target and simply burns its whole retry budget (~22s
    /// per attempt) before giving up. Catching it up front turns a ~10 minute
    /// mystery timeout into an immediate, accurate error.
    ///
    /// Returns `Err(message)` only when the live level is KNOWN and different.
    /// An unknown live level (menu, mid-teardown, scan not yet run) cannot prove
    /// a mismatch, so it is allowed through.
    pub fn check_recording_matches_live(
        recording_path: &str,
        live_level_id: u32,
    ) -> Result<(), String> {
        let (Some(want), Some(live)) = (
            code_from_recording_name(recording_path),
            code_from_id(live_level_id),
        ) else {
            return Ok(());
        };
        if want == live {
            Ok(())
        } else {
            Err(format!(
                "recording is for track {} but the game is on {} (level_id={}). \
                 Replaying it here can never match the recorded spawn — the start \
                 matcher would exhaust its retries and time out. Navigate to {} first.",
                want, live, live_level_id, want
            ))
        }
    }
}

#[cfg(test)]
mod level_epoch_tests {
    use super::*;

    /// The distinction the epoch exists to make, which `level_id` alone cannot:
    /// an F5 restart keeps the SAME level context, a track switch does not.
    #[test]
    fn resolved_only_when_the_scan_ran_in_the_current_context() {
        let mut s = zeroed_boxed();

        // Fresh init: nothing identified, but nothing swapped either.
        s.level_epoch = 0;
        s.level_scan_epoch = 0;
        assert!(level_is_resolved(&s));

        // Scan identifies FE in this context.
        s.level_id = 0;
        assert!(level_is_resolved(&s));

        // F5 restart: the root SURVIVES, so no epoch bump — the level must stay
        // trusted. This is the case that made player_ptr the wrong signal.
        assert!(level_is_resolved(&s), "an F5 restart must not unresolve");

        // Track switch: the root is reallocated, DLL bumps the epoch. The scan
        // has not caught up, so level_id still says FE — and MUST NOT be trusted.
        s.level_epoch = 1;
        assert!(
            !level_is_resolved(&s),
            "after a context swap the stale level_id must not be trusted, even \
             though it still holds a concrete (previous) track id"
        );

        // Scan identifies the new track in the new context.
        s.level_id = 1; // FM
        s.level_scan_epoch = 1;
        assert!(level_is_resolved(&s));
        assert_eq!(level::code_from_id(s.level_id), Some("FM"));
    }

    /// The snapshot read must hand back the id and its validity TOGETHER.
    /// Callers that ask "resolved?" and then separately read `level_id` can be
    /// handed "yes" plus the previous track's id if a swap lands between the
    /// two reads — that is the hole this API exists to close.
    #[test]
    fn resolved_read_returns_id_and_validity_together() {
        let mut s = zeroed_boxed();
        s.level_epoch = 7;
        s.level_scan_epoch = 7;
        s.level_id = 2; // FH, identified in this context
        assert_eq!(resolved_level_id(&s), Some(2));

        // Swap: the scan has not caught up, so level_id STILL reads FH. The
        // snapshot must refuse it rather than report a confident wrong track.
        s.level_epoch = 8;
        assert_eq!(
            resolved_level_id(&s),
            None,
            "a stale but concrete level_id must not be handed out as resolved"
        );
        assert_eq!(s.level_id, 2, "the stale id is still physically present");
    }

    /// A transient scan miss inside one context must not look like a swap.
    #[test]
    fn unknown_level_id_alone_is_not_a_context_change() {
        let mut s = zeroed_boxed();
        s.level_epoch = 3;
        s.level_scan_epoch = 3;
        s.level_id = u32::MAX; // scan found nothing this cycle

        // Resolved-but-unknown is a real state (the menu). It is NOT "the level
        // changed" — that is exactly the conflation the old code made.
        assert!(level_is_resolved(&s));
        assert_eq!(level::code_from_id(s.level_id), None);
    }
}

#[cfg(test)]
mod level_tests {
    use super::level::*;

    #[test]
    fn id_maps_to_code_and_unknown_is_none() {
        assert_eq!(code_from_id(0), Some("FE"));
        assert_eq!(code_from_id(8), Some("VH"));
        // Practice / Halfpipe / menu all publish 0xFFFFFFFF.
        assert_eq!(code_from_id(u32::MAX), None);
        assert_eq!(code_from_id(9), None);
    }

    #[test]
    fn recording_name_declares_its_track() {
        assert_eq!(code_from_recording_name("FE-tremendous.tasrec"), Some("FE"));
        assert_eq!(code_from_recording_name("TAS/recordings/FE-10065.tasrec"), Some("FE"));
        assert_eq!(code_from_recording_name(r"C:\x\VH_run.tasrec"), Some("VH"));
        assert_eq!(code_from_recording_name("scratch.tasrec"), None);
    }

    #[test]
    fn mismatch_is_rejected_and_unknown_is_allowed() {
        // The real failure this guards: FE recording, game on another track.
        let err = check_recording_matches_live("FE-tremendous.tasrec", 4).unwrap_err();
        assert!(err.contains("FE"), "{}", err);
        assert!(err.contains("AM"), "{}", err);

        // Same track is fine.
        assert!(check_recording_matches_live("FE-tremendous.tasrec", 0).is_ok());
        // Unknown live level cannot PROVE a mismatch — must not hard-fail.
        assert!(check_recording_matches_live("FE-tremendous.tasrec", u32::MAX).is_ok());
        // Unrecognised recording name carries no claim either.
        assert!(check_recording_matches_live("scratch.tasrec", 0).is_ok());
    }
}

/// Shared CONT (continue-record) F5-bucket judgment, used by BOTH tas_ui (the
/// live transport's `poll_continue_start_guard`) and tas_test (the
/// cont-reliability harness). Sharing this is the whole point: the test then
/// accepts/rejects a CONT bucket on EXACTLY the criteria the user experiences,
/// instead of a stricter harness-only rule (bit-exact anchor) that measured a
/// bucket tas_ui never has to land on.
pub mod cont {
    /// Max F5-restart retries to land the CONT replay on the recording's bucket.
    pub const START_MATCH_MAX_RETRIES: u32 = 30;
    /// Frames past the recording's first-moving frame required before the bucket
    /// fingerprint is distinguishable (so we don't judge too early).
    pub const BUCKET_CHECK_HEADROOM: u32 = 3;

    /// First frame index where `rec_coords` first differs (bit-exact) from
    /// `rec_coords[0]` — when the recorded player leaves spawn. `None` if it
    /// never moves within `recorded_count` (degenerate / positions not loaded).
    pub fn detect_first_moving(rec_coords: &[[f32; 3]], recorded_count: u32) -> Option<u32> {
        if recorded_count == 0 || rec_coords.is_empty() {
            return None;
        }
        let n = (recorded_count as usize).min(rec_coords.len());
        if n < 2 {
            return None;
        }
        let start = rec_coords[0];
        for j in 1..n {
            let c = rec_coords[j];
            if c[0].to_bits() != start[0].to_bits()
                || c[1].to_bits() != start[1].to_bits()
                || c[2].to_bits() != start[2].to_bits()
            {
                return Some(j as u32);
            }
        }
        None
    }

    /// Verdict from judging whether a CONT replay landed on the right F5 bucket.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BucketVerdict {
        /// Playback hasn't progressed far enough to judge — keep polling.
        KeepWaiting,
        /// Spawn position doesn't match the recording's start — wrong bucket, reroll.
        WrongStart,
        /// Recording never moves out of spawn → no fingerprint → nothing to match.
        NoSignal,
        /// First-moving frame matched the recording → correct bucket.
        Match,
        /// First-moving frame differs from the recording → wrong bucket, reroll.
        WrongBucket { observed: Option<u32> },
    }

    /// MINIMUM frames past the recording's first-moving frame before the judge
    /// will decide at all — enough settle trajectory to detect the first-moving
    /// frame and seed the blowup guard.
    pub const BUCKET_MATCH_WINDOW: u32 = 64;

    /// MAXIMUM frames past first-moving the judge bothers to validate before
    /// declaring Match (capped so a deep splice still gets a positive Match
    /// well before the splice, instead of running the whole catch-up Unjudged).
    ///
    /// THIS is the fix for the "accepts a bucket that diverges after the
    /// window" bug: the old judge accepted Match at first_moving+64, so a
    /// bucket that tracked through the settle then veered off LATER (live: drift
    /// growing 0.5→2.5→5.6→11 over ticks ~377-1071, ≈ fm+79..fm+773) sailed
    /// through and the resume spliced onto a wrong trajectory. The guard now
    /// runs over the WHOLE replayed prefix up to `min(splice, fm+VALIDATE)` on
    /// every poll, so a late divergence rerolls the instant drift exceeds
    /// epsilon — while a working bucket (which stays ≤~0.42 for the entire run,
    /// per the field's 4500+ logged successes) confirms Match at the cap.
    pub const BUCKET_VALIDATE_WINDOW: u32 = 1024;

    /// Per-axis BLOWUP guard (game units) over the judge window.
    ///
    /// This is deliberately GENEROUS, not a tracking tolerance. The judge's real
    /// bucket criterion is the first-moving FRAME (below) — that is the
    /// fingerprint the proven-in-the-field CONT used when it landed ~80% within
    /// a few rerolls. Measured drift profiles show that even an
    /// accepted-and-working bucket is NOT trajectory-tight in the window: the
    /// replay departs spawn with a sub-tick phase skew, blips ~0.035-0.2 during
    /// the settle, and the user's 4500+ logged successful catch-ups carried that
    /// blip as their whole-run drift ceiling. Earlier attempts to demand
    /// per-frame closeness here (bit-exact, then 0.01/0.02 epsilons) rejected
    /// exactly those working buckets and made CONT "never land".
    ///
    /// So the window check only rejects the catastrophic impostor: a bucket
    /// whose trajectory leaves the recording by more than this within the
    /// window (the logged disasters ran 0.5+, 1.4, 6, even 2265 = off-map).
    /// Working buckets max out around ~0.42 in the window; disasters start
    /// ~0.5+ — the guard sits at the gap's top edge.
    pub const BUCKET_MATCH_EPSILON: f32 = 0.5;

    /// Judge a CONT replay's F5 bucket — the criterion that was reliable in the
    /// field (~80% land rate within a few rerolls):
    ///
    /// 1. Spawn (frame 0) bits must equal the recording's start EXACTLY — the
    ///    pre-movement state is deterministic and is the true bucket identity.
    /// 2. The replay's FIRST-MOVING FRAME must equal the recording's. This is
    ///    the bucket fingerprint; rerolling until it matches is the lottery.
    /// 3. Blowup guard: the trajectory must stay within `BUCKET_MATCH_EPSILON`
    ///    (generous, 0.5) of the recording over the WHOLE replayed prefix seen
    ///    so far, up to `min(match_through, first_moving + VALIDATE_WINDOW)`.
    ///    This does NOT demand tight tracking (a working bucket legitimately
    ///    blips ~0.035-0.4 during the spawn settle, and stays ≤~0.42 for the
    ///    entire run); it rejects the catastrophic impostor that shares the
    ///    first-moving frame yet veers off-trajectory — whether that veer
    ///    starts in the settle OR hundreds of ticks later (the late-divergence
    ///    bug: accepted at fm+64 but drift grew to 11 by fm+773).
    ///
    /// `match_through` is the splice target (continue_from): the bucket must be
    /// validated clean THROUGH there before Match. Capped at fm+VALIDATE_WINDOW
    /// so a deep splice still confirms positively rather than running the whole
    /// catch-up. A bad bucket rerolls as soon as drift exceeds epsilon at ANY
    /// tick up to that cap — long before the splice.
    ///
    /// History: matching the trajectory per-frame TIGHTLY (bit-exact in
    /// 047f99c, then small epsilons) rejected the very buckets that worked —
    /// CONT went from "3-4 retries" to "never lands". Pure function over the
    /// shared-memory snapshot so tas_ui and the harness share one source of
    /// truth. `observed` in WrongBucket is the replay's first-moving frame on a
    /// fingerprint mismatch, or the divergence frame if the blowup guard fired.
    pub fn judge_cont_bucket(
        play_coords: &[[f32; 3]],
        rec_coords: &[[f32; 3]],
        recorded_count: u32,
        playback_pos: u32,
        expected_start_bits: [u32; 3],
        expected_first_moving: Option<u32>,
        match_through: u32,
    ) -> BucketVerdict {
        if playback_pos == 0 || play_coords.is_empty() {
            return BucketVerdict::KeepWaiting;
        }
        let play0 = play_coords[0];
        let play0_bits = [play0[0].to_bits(), play0[1].to_bits(), play0[2].to_bits()];
        if play0_bits != expected_start_bits {
            return BucketVerdict::WrongStart;
        }
        let expected_fm = match expected_first_moving {
            Some(f) => f,
            None => return BucketVerdict::NoSignal,
        };
        // The fingerprint is decidable the moment the replay has passed the
        // recording's first moving frame - 63 ticks earlier than this used to
        // rule, and it is the SAME ruling, not a looser one.
        //
        // `detect_first_moving` returns the FIRST index that differs from
        // frame 0, so widening the scanned prefix can never change an answer it
        // already gave, only supply one where it previously had none. Three
        // cases, and all of them agree with what fm+64 would have said:
        //
        //   found k < expected  -> wrong bucket; scanning further still finds k
        //   found k == expected -> fingerprint matches
        //   found nothing yet   -> the replay left spawn LATER than the
        //                          recording, so any k a longer scan finds is
        //                          > expected and would be rejected anyway
        //
        // Only `observed` in the rejection differs: the third case reports None
        // instead of the exact late frame, which is diagnostic, not a decision.
        //
        // Worth 63 ticks because of where they are spent. A judged PLAY hands
        // back to 1x at first_moving + 1, so a failed attempt used to watch a
        // known-wrong replay in real time for 0.63s before rerolling - most of
        // the cost of the reroll itself.
        if playback_pos < expected_fm + 1 {
            return BucketVerdict::KeepWaiting;
        }
        // The bucket fingerprint: the replay must leave spawn on the SAME frame
        // the recording did.
        let observed_fm = detect_first_moving(play_coords, playback_pos);
        if observed_fm != Some(expected_fm) {
            return BucketVerdict::WrongBucket {
                observed: observed_fm,
            };
        }
        // Accepting, on the other hand, still needs the settle window: the
        // blowup guard below wants trajectory to judge, and BUCKET_MATCH_WINDOW
        // is the minimum that has ever been trusted for it.
        let min_judge = expected_fm + BUCKET_MATCH_WINDOW;
        if playback_pos < min_judge {
            return BucketVerdict::KeepWaiting;
        }
        // Validate clean through the splice target, capped so a deep splice
        // still confirms before running the whole catch-up. Never below the
        // settle window (we already have at least that much).
        let validate_to = match_through
            .min(expected_fm + BUCKET_VALIDATE_WINDOW)
            .max(min_judge);
        // Blowup guard over the WHOLE replayed prefix seen so far (not just the
        // settle window): a late divergence is caught the instant playback
        // reaches it. Working buckets stay ≤~0.42 the entire way — allowed.
        let end = (playback_pos as usize)
            .min(validate_to as usize)
            .min(play_coords.len())
            .min(rec_coords.len())
            .min(recorded_count as usize);
        for k in 0..end {
            let p = play_coords[k];
            let r = rec_coords[k];
            if (p[0] - r[0]).abs() > BUCKET_MATCH_EPSILON
                || (p[1] - r[1]).abs() > BUCKET_MATCH_EPSILON
                || (p[2] - r[2]).abs() > BUCKET_MATCH_EPSILON
            {
                return BucketVerdict::WrongBucket {
                    observed: Some(k as u32),
                };
            }
        }
        // Clean so far — but only declare Match once we've actually validated
        // THROUGH the target depth. Until then keep replaying + re-checking, so
        // a divergence that hasn't happened yet can still reroll.
        if playback_pos < validate_to {
            return BucketVerdict::KeepWaiting;
        }
        BucketVerdict::Match
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn bits(x: f32, y: f32, z: f32) -> [u32; 3] {
            [x.to_bits(), y.to_bits(), z.to_bits()]
        }

        #[test]
        fn detect_first_moving_matches_old_behavior() {
            let mut c = vec![[1.0, 2.0, 3.0]; 10];
            c[5] = [1.0, 2.0, 3.5];
            assert_eq!(detect_first_moving(&c, 10), Some(5));
            assert_eq!(detect_first_moving(&[[1.0, 2.0, 3.0]; 10], 10), None);
            assert_eq!(detect_first_moving(&[], 0), None);
            // respects recorded_count bound
            let mut c2 = vec![[1.0, 2.0, 3.0]; 10];
            c2[5] = [9.0, 9.0, 9.0];
            assert_eq!(detect_first_moving(&c2, 3), None);
            assert_eq!(detect_first_moving(&c2, 6), Some(5));
        }

        #[test]
        fn judge_wrong_start() {
            let play = vec![[9.0, 9.0, 9.0]; 400];
            let rec = vec![[1.0, 2.0, 3.0]; 400];
            assert_eq!(
                judge_cont_bucket(&play, &rec, 400, 320, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::WrongStart
            );
        }

        #[test]
        fn judge_keep_waiting_before_window() {
            let mut play = vec![[1.0, 2.0, 3.0]; 400];
            play[250] = [1.0, 2.0, 3.5];
            let rec = play.clone();
            // pos must be >= first_moving + BUCKET_MATCH_WINDOW (314) to judge.
            assert_eq!(
                judge_cont_bucket(&play, &rec, 400, 100, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::KeepWaiting
            );
            assert_eq!(
                judge_cont_bucket(&play, &rec, 400, 313, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::KeepWaiting
            );
        }

        /// The early fingerprint rejection has to be the SAME ruling as the old
        /// fm+64 one, not a looser one - a judge that rejects buckets the field
        /// proved good is exactly how CONT once went from "3-4 retries" to
        /// "never lands". So: for every way a replay can leave the spawn, the
        /// verdict at first_moving+1 must agree with the verdict at
        /// first_moving+BUCKET_MATCH_WINDOW.
        #[test]
        fn early_rejection_agrees_with_the_old_window() {
            const FM: usize = 250;
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            for (k, item) in rec.iter_mut().enumerate().take(400).skip(FM) {
                item[2] = 3.0 + (k - FM + 1) as f32 * 0.001;
            }
            let start = bits(1.0, 2.0, 3.0);

            // Departure frames either side of the recording's, plus the exact
            // match, plus a replay that never leaves the spawn at all.
            for depart in [Some(FM - 8), Some(FM - 1), Some(FM), Some(FM + 1), Some(FM + 30), None] {
                let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
                if let Some(d) = depart {
                    for (k, item) in play.iter_mut().enumerate().take(400).skip(d) {
                        item[2] = 3.0 + (k - d + 1) as f32 * 0.001;
                    }
                }
                let early = judge_cont_bucket(
                    &play, &rec, 400, (FM + 1) as u32, start, Some(FM as u32), 400,
                );
                let late = judge_cont_bucket(
                    &play,
                    &rec,
                    400,
                    (FM + BUCKET_MATCH_WINDOW as usize) as u32,
                    start,
                    Some(FM as u32),
                    400,
                );
                let agree = match (&early, &late) {
                    // Same rejection. `observed` may differ - a replay that has
                    // not moved yet reports None early and the exact late frame
                    // later - and that field is diagnostic, not a decision.
                    (BucketVerdict::WrongBucket { .. }, BucketVerdict::WrongBucket { .. }) => true,
                    // Matching bucket: rejected by neither. Accepting still waits
                    // for the settle window, so early is KeepWaiting there.
                    (BucketVerdict::KeepWaiting, BucketVerdict::KeepWaiting) => true,
                    _ => false,
                };
                assert!(
                    agree,
                    "depart={:?}: early={:?} but old window said {:?}",
                    depart, early, late
                );
            }
        }

        /// ...and it really is EARLIER: a wrong bucket is rejected at
        /// first_moving+1, where the old judge still answered KeepWaiting.
        #[test]
        fn a_wrong_bucket_is_rejected_at_first_moving_plus_one() {
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
            play[249] = [1.0, 2.0, 3.5]; // left the spawn a frame early
            assert_eq!(
                judge_cont_bucket(&play, &rec, 400, 251, bits(1.0, 2.0, 3.0), Some(250), 400),
                BucketVerdict::WrongBucket {
                    observed: Some(249)
                }
            );
        }
        #[test]
        fn judge_match_vs_wrong_bucket() {
            let mut rec = vec![[1.0, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            // identical trajectory → Match
            assert_eq!(
                judge_cont_bucket(&rec, &rec, 400, 320, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::Match
            );
            // moves at 248 not 250 → diverges at 248
            let mut wrong = vec![[1.0, 2.0, 3.0]; 400];
            wrong[248] = [1.0, 2.0, 3.5];
            assert_eq!(
                judge_cont_bucket(&wrong, &rec, 400, 320, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::WrongBucket { observed: Some(248) }
            );
            // THE KEY CASE: same first-moving frame (250) as rec, but the
            // trajectory diverges later (260). The old frame-only fingerprint
            // accepted this (→ resume a few frames off); now it's rejected.
            let mut near = vec![[1.0, 2.0, 3.0]; 400];
            near[250] = [1.0, 2.0, 3.5];
            near[260] = [9.0, 9.0, 9.0];
            assert_eq!(
                judge_cont_bucket(&near, &rec, 400, 320, bits(1.0, 2.0, 3.0), Some(250), 320),
                BucketVerdict::WrongBucket { observed: Some(260) }
            );
        }

        #[test]
        fn judge_tolerates_right_bucket_float_noise() {
            // THE REGRESSION CASE: a working bucket reproduces the recording's
            // first-moving frame but NOT its trajectory to the bit — irreducible
            // float noise (~6e-5) plus a settle blip up to ~0.4 (sub-tick phase
            // skew; the field-proven CONT accepted these for 4500+ successful
            // catch-ups). Bit-exact/small-epsilon matching rejected them (0%
            // CONT success). Only a catastrophic off-trajectory impostor (0.5+)
            // is rejected.
            let mut rec = vec![[100.0_f32, 200.0, 300.0]; 400];
            for f in 250..400 {
                rec[f] = [100.0 + (f as f32) * 0.5, 200.0, 300.0];
            }
            // Working bucket: same first-moving frame, tiny noise everywhere.
            let mut noisy = rec.clone();
            for f in 250..400 {
                noisy[f][0] += 0.00006;
                noisy[f][2] -= 0.00004;
            }
            assert_eq!(
                judge_cont_bucket(&noisy, &rec, 400, 320, bits(100.0, 200.0, 300.0), Some(250), 320),
                BucketVerdict::Match
            );
            // Working bucket with a settle blip (0.4 transient) — still accepted;
            // this is the bucket that landed 80% of real CONTs.
            let mut blip = rec.clone();
            blip[252][0] += 0.4;
            assert_eq!(
                judge_cont_bucket(&blip, &rec, 400, 320, bits(100.0, 200.0, 300.0), Some(250), 320),
                BucketVerdict::Match
            );
            // Catastrophic impostor: same first-moving frame but veers off the
            // recording past the blowup guard — rejected.
            let mut wrong = rec.clone();
            wrong[252][0] += 0.7;
            assert_eq!(
                judge_cont_bucket(&wrong, &rec, 400, 320, bits(100.0, 200.0, 300.0), Some(250), 320),
                BucketVerdict::WrongBucket { observed: Some(252) }
            );
            // Wrong fingerprint: departs spawn a frame early — rejected with the
            // replay's first-moving frame reported.
            let mut early = rec.clone();
            early[249] = [100.5, 200.0, 300.0];
            assert_eq!(
                judge_cont_bucket(&early, &rec, 400, 320, bits(100.0, 200.0, 300.0), Some(250), 320),
                BucketVerdict::WrongBucket { observed: Some(249) }
            );
        }

        #[test]
        fn judge_rejects_late_divergence_past_settle_window() {
            // THE BUG THIS FIX CLOSES (live: drift grew 0.5→11 over ticks
            // ~377-1071 ≈ fm+79..fm+773, yet "bucket matched" and resumed onto
            // a wrong trajectory). The bucket is bit-clean through the old
            // fm+64 settle window, then veers off WELL PAST it. The judge must
            // reroll, not accept.
            let fm = 250usize;
            let splice = 1200u32; // deep splice, like a real run
            let mut rec = vec![[100.0_f32, 200.0, 300.0]; 1400];
            for f in fm..1400 {
                rec[f] = [100.0, 200.0, 300.0 + (f as f32) * 0.5]; // rides +Z
            }
            // Replay: identical through the settle, then diverges hard at
            // tick 750 (= fm+500, far past the old fm+64=314 window).
            let mut play = rec.clone();
            for f in (fm + 500)..1400 {
                play[f][0] += 8.0; // 8-unit lateral veer — a real wrong bucket
            }

            // Old behaviour accepted Match at fm+64. The fix keeps validating:
            // at pos 320 (just past the settle) it's clean SO FAR but not yet
            // validated to the target → KeepWaiting, NOT a premature Match.
            assert_eq!(
                judge_cont_bucket(&play, &rec, 1400, 320, bits(100.0, 200.0, 300.0), Some(250), splice),
                BucketVerdict::KeepWaiting,
                "must not Match at the old settle window — keep validating"
            );
            // Once playback reaches the divergence (tick 750), reroll.
            assert_eq!(
                judge_cont_bucket(&play, &rec, 1400, 800, bits(100.0, 200.0, 300.0), Some(250), splice),
                BucketVerdict::WrongBucket { observed: Some(750) },
                "late divergence must reroll, not splice onto a wrong trajectory"
            );
            // A clean bucket validated through the splice target → Match.
            assert_eq!(
                judge_cont_bucket(&rec, &rec, 1400, splice, bits(100.0, 200.0, 300.0), Some(250), splice),
                BucketVerdict::Match,
                "a clean bucket confirms Match through the splice target"
            );
        }

        #[test]
        fn judge_no_signal_for_static_recording() {
            let play = vec![[1.0, 2.0, 3.0]; 400];
            let rec = play.clone();
            assert_eq!(
                judge_cont_bucket(&play, &rec, 400, 320, bits(1.0, 2.0, 3.0), None, 320),
                BucketVerdict::NoSignal
            );
        }
    }
}

/// Shared restart/arm/reroll state machine driven by BOTH tas_ui (once per egui
/// frame) and the tas_test harness (in a poll loop), so the test exercises the
/// exact transport sequence the app runs — one source of truth, no drift.
///
/// The machine is a non-blocking **stepper**: `step()` performs at most one
/// transition and never sleeps or blocks, so the egui thread can call it per
/// frame and the harness can call it in a `while !terminal { sleep; step }`
/// loop. All side effects go through the [`TransportPort`] trait, which is
/// implemented for the real shared-memory client and for a `FakePort` in tests
/// (so the serialization invariants are checked without a running game).
pub mod transport {
    use super::cont::{judge_cont_bucket, BucketVerdict};
    use super::{TasCommand, TasMode};

    /// First N rerolls use NO jitter — the plain restart already has natural
    /// wall-clock variance, and for a recording whose restart usually lands the
    /// right bucket (the common case), jittering immediately shoves the phase
    /// AWAY from that natural center and scatters into wrong buckets. Only once
    /// genuinely stuck do we start jittering to escape.
    pub const NATURAL_RESTART_ATTEMPTS: u32 = 8;

    /// Wall-clock jitter (ms) before a reroll's restart, so the injected F5
    /// lands at a different accumulator-modulo-tick phase than the last attempt.
    /// Zero for the first `NATURAL_RESTART_ATTEMPTS` (let natural variance work),
    /// then a phase-cycling delay to escape a stuck bucket. Kept here (not in
    /// tas_ui) so the harness reroll jitters identically.
    pub fn cont_retry_jitter_ms(attempt: u32) -> u64 {
        if attempt <= NATURAL_RESTART_ATTEMPTS {
            return 0;
        }
        (attempt as u64 * 7 + 3) % 17 + 1
    }

    /// Which session to arm after the restart completes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Arm {
        Rec,
        Play,
        Continue,
    }

    impl Arm {
        fn command(self) -> TasCommand {
            match self {
                Arm::Rec => TasCommand::ArmRec,
                Arm::Play => TasCommand::ArmPlay,
                Arm::Continue => TasCommand::ArmContinue,
            }
        }
    }

    /// The recording's F5-bucket fingerprint a CONT replay must match (computed
    /// by the caller from `rec_coords` via [`super::cont::detect_first_moving`]).
    #[derive(Debug, Clone, Copy)]
    pub struct BucketTarget {
        pub expected_start_bits: [u32; 3],
        pub expected_first_moving: Option<u32>,
    }

    /// Everything the controller needs to drive one arm cycle to completion.
    #[derive(Debug, Clone, Copy)]
    pub struct ArmConfig {
        pub arm: Arm,
        /// Replay/catch-up speed to assert before arming.
        pub catchup_speed: f32,
        /// Splice frame for CONT (0 for REC/PLAY).
        pub continue_from_frame: u32,
        /// Bucket fingerprint to match; `Some` only for CONT.
        pub target: Option<BucketTarget>,
        /// Max F5 rerolls to land the recording's bucket (CONT only).
        pub max_retries: u32,
        /// Speed to hand back to once the replay reaches the first moving
        /// frame - 0 disables the handover and the whole cycle stays at
        /// `catchup_speed`.
        ///
        /// WHY THIS EXISTS. A judged cycle replays the spawn countdown purely
        /// so the judge can see which frame the boarder leaves the spawn on.
        /// Nothing before that frame is worth watching - the boarder is
        /// stationary - and for FE-10065 it is ~2.6 seconds, paid on the
        /// accepted attempt AND on every reroll. CONT never had this problem
        /// because it replays the prefix at 256x and splices; a bucket-matched
        /// PLAY has no splice to hand over at, so it needs its own.
        ///
        /// Captured when the cycle is armed, so changing the speed control
        /// during the judge does not move it. The UI restores the speed the user
        /// last chose when the cycle finishes; only the window between the
        /// handover and Done runs at the speed they had when they pressed PLAY.
        ///
        /// The handover is performed by the DLL at the exact tick (cave2, with
        /// cave5 capping the batch to land on it), not by whoever is polling:
        /// a poll-driven drop can be a whole catch-up batch late, which at 64x
        /// means starting the run the user asked to watch already
        /// fast-forwarded. Same reasoning as the CONT splice's
        /// `cont_resume_speed`.
        pub resume_speed: f32,
    }

    /// The few shared-memory operations the transport machine performs. Returns
    /// raw `u32` for mode/restart_state to match the live client's accessors.
    pub trait TransportPort {
        fn send_command(&mut self, cmd: TasCommand);
        fn mode(&self) -> u32;
        fn restart_state(&self) -> u32;
        fn reset_restart_state(&mut self);
        fn playback_pos(&self) -> u32;
        fn play_coords(&self) -> &[[f32; 3]];
        /// The loaded recording's trajectory + length — for the bit-exact bucket
        /// fingerprint (the replay must reproduce this, not just its first-move).
        fn rec_coords(&self) -> &[[f32; 3]];
        fn recorded_count(&self) -> u32;
        fn set_continue_from_frame(&mut self, frame: u32);
        fn set_playback_speed(&mut self, speed: f32);
        /// Ask the DLL to drop the playback speed to `speed` at replay
        /// position `pos`, atomically on that tick. `pos == 0` clears any
        /// pending handover.
        fn set_speed_handoff(&mut self, pos: u32, speed: f32);
        /// True while a handover written by [`Self::set_speed_handoff`] has
        /// not fired yet. The DLL clears it when it fires, which is how the
        /// controller knows to stop re-asserting the catch-up speed.
        fn speed_handoff_pending(&self) -> bool;
        /// Monotonic counter the DLL bumps once per processed arm. Used to tell
        /// this attempt's replay state from the previous one's.
        fn arm_generation(&self) -> u32;
    }

    /// Fixed wall-clock delay (ms) between sending Stop and sending Restart.
    /// This both serialises the single-u32 command slot (cave2 processes Stop →
    /// mode OFF before Restart) AND — critically — fixes the F5 phase: the
    /// post-restart bucket is decided by the wall-clock-modulo-tick at the
    /// Restart, so a CONSISTENT Stop→Restart delay lands a consistent (good)
    /// bucket. The old "poll until mode==OFF" fired Restart at a variable,
    /// step-rate-dependent phase (~10-20ms), scattering hard recordings into
    /// wrong buckets. Matches the legacy restart_and_stabilize_inprocess (50ms),
    /// which lands hard recordings like FE-10065 reliably.
    pub const STOP_SETTLE_MS: u64 = 50;

    /// Delay (ms) between restart_state==2 and sending the Arm command. The arm
    /// point sets where play_coords[0] is captured in the spawn countdown, which
    /// shifts the OBSERVED first-moving frame. The legacy loop arms ~15-20ms
    /// after rs==2 (20ms poll + extra steps); arming immediately (as the bare
    /// controller did) reads first-moving ~2 frames late and never matches
    /// recordings captured with the legacy timing (e.g. FE-10065 wants 298, the
    /// bare controller saw 300). This pins the arm phase to match.
    pub const ARM_SETTLE_MS: u64 = 10;

    /// Observed first-moving frame when the arm fires with ZERO settle, and the
    /// ms of settle that shifts the observed first-moving by one frame (the arm
    /// arms later → shorter remaining countdown → smaller first-moving). Both
    /// empirical (FE-10065: 0ms→300, 20ms→~297.5).
    pub const OBSERVED_AT_ZERO_SETTLE: i64 = 300;
    pub const ARM_SETTLE_MS_PER_FRAME: i64 = 8;

    /// Arm-settle delay (ms) for a given attempt. Rather than blindly sweeping,
    /// COMPUTE the settle that targets the recording's known first-moving
    /// (`observed ≈ OBSERVED_AT_ZERO_SETTLE - settle/MS_PER_FRAME`), then dither
    /// a few frames around it to absorb the ±1-2 frame restart variance. So a
    /// recording whose first-moving is an outlier (FE-goodstart=300) is targeted
    /// on attempt 0 instead of found ~1-in-10. Deterministic calibration applied
    /// to the arm phase.
    /// Attempts that retry the computed base settle UNCHANGED before dithering.
    /// The restart has ±1-2 frame variance, so the correct base lands within a
    /// few plain retries; dithering immediately (as a sweep does) instead wastes
    /// attempts on the wrong target and fattens the tail. Only after this many
    /// base-misses do we assume the base is miscalibrated and start dithering.
    pub const ARM_SETTLE_BASE_TRIES: u32 = 5;

    pub fn arm_settle_ms(attempt: u32, expected_first_moving: Option<u32>) -> u64 {
        let base = match expected_first_moving {
            Some(fm) => {
                ((OBSERVED_AT_ZERO_SETTLE - fm as i64) * ARM_SETTLE_MS_PER_FRAME).clamp(0, 48)
            }
            None => ARM_SETTLE_MS as i64,
        };
        if attempt < ARM_SETTLE_BASE_TRIES {
            // Retry the computed target; the restart variance lands it.
            return base.clamp(0, 64) as u64;
        }
        // Base kept missing → it's probably miscalibrated. Dither ±frames to
        // find the true arm phase.
        const DITHER: [i64; 6] = [8, -8, 16, -16, 24, -24];
        let d = (attempt - ARM_SETTLE_BASE_TRIES) as usize;
        (base + DITHER[d % DITHER.len()]).clamp(0, 64) as u64
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Phase {
        Start,
        /// Stop sent; the caller is honouring a fixed STOP_SETTLE_MS wait (during
        /// which cave2 flips mode to OFF) before we send Restart.
        StopSettle,
        /// Restart sent; waiting for restart_state == 2.
        RestartWaitDone,
        /// restart_state==2 seen; honouring a fixed ARM_SETTLE_MS wait before the
        /// Arm command so the arm phase (→ observed first-moving) is consistent.
        ArmSettle,
        /// CONT only: replaying — judge the F5 bucket each step.
        JudgeBucket,
        Done,
        Aborted,
    }

    /// Result of one `step()`.
    #[derive(Debug, Clone, PartialEq)]
    pub enum StepOutcome {
        /// Mid-cycle; call `step()` again (after a short poll delay).
        InProgress,
        /// Wait exactly `ms` (wall-clock) before the next `step()` — used for the
        /// fixed Stop→Restart settle that pins the F5 phase.
        Wait { ms: u64 },
        /// A reroll was scheduled. Caller should wait `suggested_delay_ms`
        /// (harness: sleep; egui: it already slept) before the next `step()`.
        /// `observed`/`expected` are the rejected bucket's first-moving frame and
        /// the recording's target (diagnostic — shows HOW it mismatched).
        Reroll {
            attempt: u32,
            suggested_delay_ms: u64,
            observed: Option<u32>,
            expected: Option<u32>,
        },
        /// Terminal success: armed (and, for CONT, landed the bucket).
        /// `completed_via` says HOW the bucket was accepted — a `NoSignal` accept
        /// was NOT positively confirmed and may be a near-miss bucket.
        Done {
            retries_used: u32,
            completed_via: CompletedVia,
        },
        /// Terminal failure: gave up after exhausting retries.
        Aborted { reason: String },
    }

    /// How a CONT/PLAY cycle reached `Done` — diagnostic for "did it resume on
    /// the right bucket?".
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CompletedVia {
        /// The bucket fingerprint positively matched the recording. Trustworthy.
        BucketMatched,
        /// Accepted with no movement signal to judge against — the resumed state
        /// could be from a near-miss bucket. Prime suspect for a "wrong" resume.
        NoSignal,
        /// Nothing to judge (plain PLAY/REC with no fingerprint) or the splice
        /// had already fired before judging.
        Unjudged,
    }

    /// Drives one restart→arm(→judge→reroll) cycle to a terminal outcome.
    pub struct TransportController {
        cfg: ArmConfig,
        phase: Phase,
        retries_remaining: u32,
        completed_via: CompletedVia,
        /// A speed handover has been staged with the DLL for this attempt.
        handoff_sent: bool,
        /// The DLL's arm counter as it stood just before this attempt armed.
        /// Once it differs, the mode and position being read describe THIS
        /// attempt; until then they still describe the previous replay.
        ///
        /// ASSUMES ONE WRITER. The counter is global, not per-request, so a
        /// second process arming the same game would move it and this cycle
        /// would read that replay as its own. That is the same assumption the
        /// single-u32 command slot already makes of every caller, and the same
        /// mitigation applies - the harness calls
        /// `stop_competing_tas_ui_writer()` before it drives anything.
        arm_generation_at_arm: u32,
    }

    impl TransportController {
        pub fn new(cfg: ArmConfig) -> Self {
            let retries_remaining = cfg.max_retries;
            Self {
                cfg,
                phase: Phase::Start,
                retries_remaining,
                completed_via: CompletedVia::Unjudged,
                handoff_sent: false,
                arm_generation_at_arm: 0,
            }
        }

        /// True when this cycle owns `playback_speed` outright: it stages a
        /// speed handover, so it asserts the catch-up before that fires and the
        /// resume speed after, on every step.
        ///
        /// Exists so a UI that also syncs the speed can step aside for exactly
        /// the controller's lifetime. Deriving it from the controller means
        /// there is no separate flag to forget to clear on a reroll, an abort or
        /// a STOP - the earlier version of this was a UI latch, and it was
        /// cleared on reroll while the same controller was still running.
        pub fn owns_playback_speed(&self) -> bool {
            self.cfg.resume_speed > 0.0
        }

        /// Which transition the cycle is waiting on, for diagnostics.
        ///
        /// A stalled cycle reports only "no progress within budget", which is
        /// the same message whether the F5 restart never completed, the arm was
        /// never processed, or the judge is waiting on a replay that will not
        /// arrive. Those have completely different causes and the log could not
        /// tell them apart.
        pub fn phase_name(&self) -> &'static str {
            match self.phase {
                Phase::Start => "Start",
                Phase::StopSettle => "StopSettle (waiting out the fixed Stop->Restart delay)",
                Phase::RestartWaitDone => "RestartWaitDone (waiting for the F5 restart)",
                Phase::ArmSettle => "ArmSettle",
                Phase::JudgeBucket => "JudgeBucket (waiting on the replay)",
                Phase::Done => "Done",
                Phase::Aborted => "Aborted",
            }
        }

        pub fn is_terminal(&self) -> bool {
            matches!(self.phase, Phase::Done | Phase::Aborted)
        }

        fn retries_used(&self) -> u32 {
            self.cfg.max_retries - self.retries_remaining
        }

        /// Perform at most one transition. Never blocks/sleeps.
        pub fn step(&mut self, port: &mut impl TransportPort) -> StepOutcome {
            let rec = TasMode::Rec as u32;
            let play = TasMode::Play as u32;
            // Re-assert the catch-up speed on EVERY step, not just at phase
            // boundaries. The in-process F5 restart momentarily resets the game's
            // speed; without continuous re-assertion the post-restart countdown
            // replays at 1x for ~3s until the next phase re-sets it. (tas_ui used
            // to do this via an ungated per-frame sync; the controller owns it
            // now.)
            //
            // EXCEPT once a staged speed handover has fired: from there the
            // speed to hold is the resume speed, and re-asserting the catch-up
            // would undo the handover and leave the user watching the run at
            // 64x. `handoff_sent` keeps the catch-up alive for the phases
            // BEFORE the handover is staged (Start..ArmSettle), where
            // `speed_handoff_pending()` is also false but for the opposite
            // reason.
            //
            // It ASSERTS the resume speed rather than just going quiet, because
            // this read and the write below are not one operation: a step can
            // read "still pending", have cave2 fire underneath it, and then
            // write the catch-up speed over the handover.
            if self.handoff_sent && !port.speed_handoff_pending() {
                port.set_playback_speed(self.cfg.resume_speed);
            } else {
                port.set_playback_speed(self.cfg.catchup_speed);
                // Re-read the marker AFTER that write, and repair it if the
                // handover fired in between. Relying on the next step to repair
                // it is not enough: the window has no timing bound, and cave5
                // programs its next tick batch from playback_speed - so a few
                // ticks of the run could be issued at the catch-up rate,
                // which is exactly the exact-tick guarantee this is all for.
                //
                // This closes it rather than narrowing it. cave2 releases its
                // claim BEFORE installing the speed, so a marker still set at
                // this second read means cave2's own store has not happened yet
                // and will land after ours; a marker cleared by now means the
                // speed is ours to restore.
                if self.handoff_sent && !port.speed_handoff_pending() {
                    port.set_playback_speed(self.cfg.resume_speed);
                }
            }
            match self.phase {
                Phase::Start => {
                    port.set_playback_speed(self.cfg.catchup_speed);
                    port.set_continue_from_frame(self.cfg.continue_from_frame);
                    // Clear any handover left over from a previous cycle before
                    // the replay position resets, so it cannot fire against the
                    // wrong replay.
                    port.set_speed_handoff(0, 0.0);
                    self.handoff_sent = false;
                    // Always Stop then settle a FIXED delay before Restart (like
                    // the legacy loop), so the Restart fires at a consistent
                    // wall-clock phase → consistent (good) F5 bucket.
                    port.send_command(TasCommand::Stop);
                    self.phase = Phase::StopSettle;
                    StepOutcome::Wait {
                        ms: STOP_SETTLE_MS,
                    }
                }
                Phase::StopSettle => {
                    // The settle wait elapsed (caller honoured the Wait), so cave2
                    // has flipped to OFF. Fire the Restart now.
                    port.reset_restart_state();
                    port.send_command(TasCommand::Restart);
                    self.phase = Phase::RestartWaitDone;
                    StepOutcome::InProgress
                }
                Phase::RestartWaitDone => {
                    if port.restart_state() == 2 {
                        port.reset_restart_state();
                        // Don't arm immediately — honour an arm settle so the arm
                        // phase (→ observed first-moving) matches the recording.
                        // Sweep the settle across rerolls so any recording aligns.
                        self.phase = Phase::ArmSettle;
                        let expected_fm =
                            self.cfg.target.and_then(|t| t.expected_first_moving);
                        StepOutcome::Wait {
                            ms: arm_settle_ms(self.retries_used(), expected_fm),
                        }
                    } else {
                        StepOutcome::InProgress
                    }
                }
                Phase::ArmSettle => {
                    // cave2 reads these at ARM time — re-assert post-restart.
                    port.set_continue_from_frame(self.cfg.continue_from_frame);
                    port.set_playback_speed(self.cfg.catchup_speed);
                    // Stage the speed handover BEFORE arming: the arm resets the
                    // replay position to 0, and cave2 only tests the handover
                    // inside the PLAY tick path, so there is no window where a
                    // stale position could trip it.
                    //
                    // Position is first_moving + 1 - the first tick at which the
                    // recording is actually moving. Handing over at the JUDGE
                    // point (fm + BUCKET_MATCH_WINDOW) instead would fast-forward
                    // through the opening 64 ticks of the run, which is the part
                    // the user pressed PLAY to watch.
                    let handoff_at = self
                        .cfg
                        .target
                        .and_then(|t| t.expected_first_moving)
                        .filter(|_| self.cfg.resume_speed > 0.0)
                        .map(|fm| fm.saturating_add(1));
                    match handoff_at {
                        Some(pos) => {
                            port.set_speed_handoff(pos, self.cfg.resume_speed);
                            self.handoff_sent = true;
                        }
                        None => {
                            port.set_speed_handoff(0, 0.0);
                            self.handoff_sent = false;
                        }
                    }
                    // Snapshot the arm counter BEFORE the command goes out, so
                    // the judge can tell this attempt's replay state from the
                    // previous one's no matter how the polling lands.
                    self.arm_generation_at_arm = port.arm_generation();
                    port.send_command(self.cfg.arm.command());
                    // Judge the F5 bucket whenever a fingerprint was given — CONT
                    // always has one; PLAY can too, so a replay rerolls until it
                    // lands the recording's bucket (zero drift), exactly like the
                    // harness's restart_play_and_match. REC has no target (it's a
                    // fresh recording) → done.
                    if self.cfg.target.is_some() {
                        self.phase = Phase::JudgeBucket;
                        StepOutcome::InProgress
                    } else {
                        self.finish(CompletedVia::Unjudged)
                    }
                }
                Phase::JudgeBucket => {
                    // Until the DLL has processed the arm, the mode and position
                    // being read still describe the PREVIOUS replay - so nothing
                    // here can be concluded from them yet. This has to come
                    // before EVERY mode interpretation, the REC one included: a
                    // stale REC from before the arm would otherwise read as
                    // "the splice already fired".
                    if port.arm_generation() == self.arm_generation_at_arm {
                        return StepOutcome::InProgress;
                    }
                    let mode = port.mode();
                    if mode == rec {
                        // Splice already fired (PLAY→REC) — bucket accepted.
                        return self.finish(CompletedVia::Unjudged);
                    }
                    // Past the arm, "not in PLAY" means the replay ENDED (or the
                    // DLL refused the arm outright, which leaves it OFF forever).
                    // Either way nothing more will arrive, so the judge's
                    // KeepWaiting has to become terminal rather than InProgress -
                    // a recording shorter than first_moving + BUCKET_MATCH_WINDOW
                    // can never be ruled on and used to spin the cycle forever.
                    let replay_ended = mode != play;
                    let pos = port.playback_pos();
                    if pos == 0 {
                        if replay_ended {
                            // The arm was processed, the mode is not PLAY, and
                            // not one tick replayed: the DLL REFUSED the arm
                            // (cave2 bounces ARM_CONTINUE from mid-run, for
                            // one). Nothing ran and nothing was judged, so this
                            // is a failure, not a quiet success - reporting it
                            // as Done left a refused CONT's 256x catch-up
                            // asserted with no splice ever coming to undo it.
                            self.phase = Phase::Aborted;
                            return StepOutcome::Aborted {
                                reason: "the DLL refused the arm (nothing replayed)"
                                    .to_string(),
                            };
                        }
                        return StepOutcome::InProgress;
                    }
                    let target = match self.cfg.target {
                        Some(t) => t,
                        // CONT with no fingerprint can't be judged — accept.
                        None => return self.finish(CompletedVia::Unjudged),
                    };
                    let verdict = judge_cont_bucket(
                        port.play_coords(),
                        port.rec_coords(),
                        port.recorded_count(),
                        pos,
                        target.expected_start_bits,
                        target.expected_first_moving,
                        // Validate as deep as CONT does.
                        //
                        // CONT passes its splice frame here. PLAY has no splice
                        // and used to pass continue_from_frame == 0, which the
                        // judge floors at first_moving + BUCKET_MATCH_WINDOW - so
                        // a bucket-matched PLAY stopped checking 64 ticks after
                        // the boarder moved while CONT kept checking for up to
                        // 1024. That is backwards: PLAY is watched end to end, so
                        // a late divergence is MORE visible there, not less.
                        // Passing the recording length makes both arms use the
                        // same rule (the judge still caps at
                        // fm + BUCKET_VALIDATE_WINDOW). Either way the bucket is
                        // validated clean through that depth before Match, so a
                        // late divergence rerolls instead of being accepted.
                        if self.cfg.continue_from_frame > 0 {
                            self.cfg.continue_from_frame
                        } else {
                            port.recorded_count()
                        },
                    );
                    match verdict {
                        BucketVerdict::KeepWaiting => {
                            if replay_ended {
                                // Ran out of replay before the judge could rule.
                                // Nothing more will arrive, so stop rather than
                                // spin.
                                self.finish(CompletedVia::Unjudged)
                            } else {
                                StepOutcome::InProgress
                            }
                        }
                        BucketVerdict::Match => self.finish(CompletedVia::BucketMatched),
                        // Accepted but NOT positively confirmed — flag it so the
                        // UI can warn that the resume may be on a near-miss bucket.
                        BucketVerdict::NoSignal => self.finish(CompletedVia::NoSignal),
                        BucketVerdict::WrongBucket { observed } => self.reroll(
                            port,
                            format!(
                                "bucket mismatch (observed first-moving={:?}, expected={:?})",
                                observed, target.expected_first_moving
                            ),
                            observed,
                        ),
                        BucketVerdict::WrongStart => {
                            self.reroll(port, "start mismatch".to_string(), None)
                        }
                    }
                }
                Phase::Done => StepOutcome::Done {
                    retries_used: self.retries_used(),
                    completed_via: self.completed_via,
                },
                Phase::Aborted => StepOutcome::Aborted {
                    reason: "CONT aborted".to_string(),
                },
            }
        }

        /// Record how the cycle completed and emit the terminal `Done`.
        fn finish(&mut self, via: CompletedVia) -> StepOutcome {
            self.completed_via = via;
            self.phase = Phase::Done;
            StepOutcome::Done {
                retries_used: self.retries_used(),
                completed_via: via,
            }
        }

        fn reroll(
            &mut self,
            port: &mut impl TransportPort,
            detail: String,
            observed: Option<u32>,
        ) -> StepOutcome {
            // Drop the staged handover FIRST, so the terminal abort below leaves
            // nothing armed either. A last-attempt failure returns early, and a
            // marker surviving that return is inherited by whatever arms next -
            // including a CONT, which would then take a PLAY's resume speed and
            // clock reset partway through its catch-up.
            port.set_speed_handoff(0, 0.0);
            self.handoff_sent = false;
            if self.retries_remaining == 0 {
                port.send_command(TasCommand::Stop);
                self.phase = Phase::Aborted;
                return StepOutcome::Aborted {
                    reason: format!("{} after {} retries", detail, self.cfg.max_retries),
                };
            }
            self.retries_remaining -= 1;
            let attempt = self.cfg.max_retries - self.retries_remaining;
            port.set_continue_from_frame(self.cfg.continue_from_frame);
            port.set_playback_speed(self.cfg.catchup_speed);
            port.send_command(TasCommand::Stop);
            self.phase = Phase::StopSettle;
            // Wait the fixed settle plus any escape jitter before the next
            // Restart, so the reroll's Restart phase is settle-pinned too.
            StepOutcome::Reroll {
                attempt,
                suggested_delay_ms: STOP_SETTLE_MS + cont_retry_jitter_ms(attempt),
                observed,
                expected: self.cfg.target.and_then(|t| t.expected_first_moving),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[derive(Default)]
        struct FakePort {
            mode: u32,
            restart_state: u32,
            playback_pos: u32,
            play_coords: Vec<[f32; 3]>,
            rec_coords: Vec<[f32; 3]>,
            recorded_count: u32,
            continue_from_frame: u32,
            playback_speed: f32,
            speed_handoff_pos: u32,
            speed_after_handoff: f32,
            arm_generation: u32,
            commands: Vec<TasCommand>,
            /// Invariant tracker: Restart must NEVER be sent while mode != OFF.
            restart_while_not_off: bool,
        }

        impl TransportPort for FakePort {
            fn send_command(&mut self, cmd: TasCommand) {
                if cmd == TasCommand::Restart && self.mode != TasMode::Off as u32 {
                    self.restart_while_not_off = true;
                }
                // Stand in for cave2 processing the arm.
                if matches!(cmd, TasCommand::ArmPlay | TasCommand::ArmContinue) {
                    self.arm_generation = self.arm_generation.wrapping_add(1);
                }
                self.commands.push(cmd);
            }
            fn mode(&self) -> u32 {
                self.mode
            }
            fn restart_state(&self) -> u32 {
                self.restart_state
            }
            fn reset_restart_state(&mut self) {
                self.restart_state = 0;
            }
            fn playback_pos(&self) -> u32 {
                self.playback_pos
            }
            fn play_coords(&self) -> &[[f32; 3]] {
                &self.play_coords
            }
            fn rec_coords(&self) -> &[[f32; 3]] {
                &self.rec_coords
            }
            fn recorded_count(&self) -> u32 {
                self.recorded_count
            }
            fn set_continue_from_frame(&mut self, frame: u32) {
                self.continue_from_frame = frame;
            }
            fn set_playback_speed(&mut self, speed: f32) {
                self.playback_speed = speed;
            }
            fn set_speed_handoff(&mut self, pos: u32, speed: f32) {
                self.speed_handoff_pos = pos;
                self.speed_after_handoff = speed;
            }
            fn speed_handoff_pending(&self) -> bool {
                self.speed_handoff_pos != 0
            }
            fn arm_generation(&self) -> u32 {
                self.arm_generation
            }
        }

        impl FakePort {
            /// Stand in for cave2: when the replay reaches the staged handover
            /// position, drop the speed on that tick and clear the request.
            fn dll_tick(&mut self) {
                if self.speed_handoff_pos != 0 && self.playback_pos >= self.speed_handoff_pos
                {
                    if self.speed_after_handoff > 0.0 {
                        self.playback_speed = self.speed_after_handoff;
                    }
                    self.speed_handoff_pos = 0;
                }
            }
        }

        fn cfg(arm: Arm, target: Option<BucketTarget>, max_retries: u32) -> ArmConfig {
            ArmConfig {
                arm,
                catchup_speed: 12.0,
                // Splice at 320 so the judge validates through exactly the
                // pos=320 these tests poll at (the controller passes this as
                // judge match_through). The deep validate-through-splice path
                // is covered directly by judge_rejects_late_divergence_*.
                continue_from_frame: if arm == Arm::Continue { 320 } else { 0 },
                target,
                max_retries,
                resume_speed: 0.0,
            }
        }

        fn bits(x: f32, y: f32, z: f32) -> [u32; 3] {
            [x.to_bits(), y.to_bits(), z.to_bits()]
        }

        #[test]
        fn rec_restart_serializes_stop_before_restart() {
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Rec, None, 0));

            // Start: Stop, then a FIXED settle wait before Restart.
            assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: STOP_SETTLE_MS });
            assert_eq!(p.commands, vec![TasCommand::Stop]);
            // During the settle, cave2 processes the Stop → mode flips OFF.
            p.mode = TasMode::Off as u32;
            // StopSettle: Restart fires (mode is OFF, so no clobber).
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);
            assert_eq!(p.commands.last(), Some(&TasCommand::Restart));

            p.restart_state = 2;
            // RestartWaitDone → arm settle wait, then ArmSettle → ArmRec.
            assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: ARM_SETTLE_MS });
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::Unjudged
                }
            );
            assert_eq!(
                p.commands,
                vec![TasCommand::Stop, TasCommand::Restart, TasCommand::ArmRec]
            );
            assert!(!p.restart_while_not_off, "Restart sent while mode != OFF!");
            assert!(c.is_terminal());
        }

        #[test]
        fn play_from_off_still_stops_first() {
            let mut p = FakePort {
                mode: TasMode::Off as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, None, 0));
            // Even from OFF, always Stop + settle (matches the legacy loop, which
            // is what lands hard buckets reliably).
            assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: STOP_SETTLE_MS });
            assert_eq!(p.commands, vec![TasCommand::Stop]);
            assert_eq!(c.step(&mut p), StepOutcome::InProgress); // Restart
            assert_eq!(p.commands, vec![TasCommand::Stop, TasCommand::Restart]);
            p.restart_state = 2;
            assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: ARM_SETTLE_MS });
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::Unjudged
                }
            );
            assert_eq!(
                p.commands,
                vec![TasCommand::Stop, TasCommand::Restart, TasCommand::ArmPlay]
            );
            assert_eq!(p.continue_from_frame, 0);
        }

        /// Drive the controller through restart until it's armed CONT and in the
        /// JudgeBucket phase. Returns once ArmContinue has been sent.
        fn drive_to_judge(c: &mut TransportController, p: &mut FakePort) {
            // Start (Stop, Wait) → settle→OFF → StopSettle (Restart) → rs=2 →
            // RestartWaitDone (arm-settle Wait) → ArmSettle (ArmContinue).
            c.step(p);
            p.mode = TasMode::Off as u32;
            c.step(p);
            p.restart_state = 2;
            c.step(p); // RestartWaitDone -> ArmSettle (Wait)
            c.step(p); // ArmSettle -> arm command, phase -> JudgeBucket
            // Assert against the configured arm, not a hardcoded ArmContinue:
            // PLAY is judged too when it is given a target.
            assert_eq!(p.commands.last(), Some(&c.cfg.arm.command()));
            // game enters PLAY for the replay
            p.mode = TasMode::Play as u32;
        }

        /// Drive the reroll restart (after a Reroll outcome) up to re-arm.
        fn drive_reroll_to_judge(c: &mut TransportController, p: &mut FakePort) {
            p.mode = TasMode::Off as u32;
            c.step(p); // StopSettle -> Restart
            p.restart_state = 2;
            c.step(p); // RestartWaitDone -> ArmSettle (Wait)
            c.step(p); // ArmSettle -> ArmContinue
            p.mode = TasMode::Play as u32;
        }

        /// The whole point of the handover: the countdown replays fast, and the
        /// speed drops at the first moving frame - not at the judge point 64
        /// ticks later, which would fast-forward through the opening of the very
        /// run the user pressed PLAY to watch.
        #[test]
        fn play_hands_speed_back_at_the_first_moving_frame() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut a = cfg(Arm::Play, Some(target), 30);
            a.catchup_speed = 64.0;
            a.resume_speed = 1.0;
            let mut c = TransportController::new(a);
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            assert_eq!(p.speed_handoff_pos, 251, "handover staged at first_moving + 1");
            assert_eq!(p.speed_after_handoff, 1.0);
            assert_eq!(p.playback_speed, 64.0);

            // The countdown replays at the catch-up speed.
            p.play_coords = coords.clone();
            p.playback_pos = 200;
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);
            p.dll_tick();
            assert_eq!(p.playback_speed, 64.0);
            assert_eq!(p.speed_handoff_pos, 251, "not reached yet");

            // The DLL fires the handover on the tick it was asked for.
            p.playback_pos = 251;
            p.dll_tick();
            assert_eq!(p.speed_handoff_pos, 0);
            assert_eq!(p.playback_speed, 1.0);

            // REGRESSION: the controller re-asserts the catch-up speed on every
            // step. Left ungated it would undo the handover on the very next one
            // and the run would be watched at 64x after all.
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);
            assert_eq!(
                p.playback_speed, 1.0,
                "catch-up speed re-asserted after the handover fired"
            );

            p.playback_pos = 400;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::BucketMatched
                }
            );
            assert_eq!(p.playback_speed, 1.0);
        }

        /// CONT hands over at its splice (cont_resume_speed, inside the DLL), so
        /// it must stage nothing here and must keep its catch-up asserted for the
        /// whole replay.
        /// The UI also writes playback_speed every frame, and a judged PLAY's
        /// handover fires mid-replay with no mode change to notice it by. Rather
        /// than race, the UI steps aside for exactly this controller's lifetime -
        /// so "does this cycle own the speed" has to be answerable from the
        /// controller itself, not from a flag the UI keeps in step with it.
        #[test]
        fn only_a_handover_cycle_owns_the_speed() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut a = cfg(Arm::Play, Some(target), 30);
            a.resume_speed = 1.0;
            assert!(TransportController::new(a).owns_playback_speed());

            // CONT hands over at its splice instead, and REC never replays.
            assert!(!TransportController::new(cfg(Arm::Continue, Some(target), 30))
                .owns_playback_speed());
            assert!(!TransportController::new(cfg(Arm::Rec, None, 0)).owns_playback_speed());
        }

        /// The check and the write are two operations, so cave2 can fire between
        /// them and the catch-up store lands last with nothing to correct it.
        /// Waiting for the next step to repair that is not enough: the window has
        /// no timing bound and cave5 programs its next tick batch from
        /// playback_speed, so the run can be issued fast for those ticks.
        #[test]
        fn a_handover_that_fires_mid_step_is_repaired_within_the_same_step() {
            /// Fires the handover on the FIRST speed write of a step - i.e.
            /// exactly inside the window between the controller's check and its
            /// store, which is the interleaving the second read exists for.
            #[derive(Default)]
            struct RacyPort {
                inner: FakePort,
                armed: bool,
            }
            impl TransportPort for RacyPort {
                fn send_command(&mut self, cmd: TasCommand) {
                    self.inner.send_command(cmd)
                }
                fn mode(&self) -> u32 {
                    self.inner.mode()
                }
                fn restart_state(&self) -> u32 {
                    self.inner.restart_state()
                }
                fn reset_restart_state(&mut self) {
                    self.inner.reset_restart_state()
                }
                fn playback_pos(&self) -> u32 {
                    self.inner.playback_pos()
                }
                fn play_coords(&self) -> &[[f32; 3]] {
                    self.inner.play_coords()
                }
                fn rec_coords(&self) -> &[[f32; 3]] {
                    self.inner.rec_coords()
                }
                fn recorded_count(&self) -> u32 {
                    self.inner.recorded_count()
                }
                fn set_continue_from_frame(&mut self, frame: u32) {
                    self.inner.set_continue_from_frame(frame)
                }
                fn set_playback_speed(&mut self, speed: f32) {
                    if self.armed {
                        // cave2 fires BEFORE this store lands - clearing the claim
                        // and installing the resume speed, in that order, as the
                        // DLL does. That is the losing interleaving: the
                        // controller decided on the catch-up while the marker was
                        // still set, and its store now arrives last.
                        self.armed = false;
                        self.inner.playback_pos = self.inner.speed_handoff_pos;
                        self.inner.dll_tick();
                    }
                    self.inner.set_playback_speed(speed);
                }
                fn set_speed_handoff(&mut self, pos: u32, speed: f32) {
                    self.inner.set_speed_handoff(pos, speed)
                }
                fn speed_handoff_pending(&self) -> bool {
                    self.inner.speed_handoff_pending()
                }
                fn arm_generation(&self) -> u32 {
                    self.inner.arm_generation()
                }
            }

            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = RacyPort::default();
            p.inner.mode = TasMode::Rec as u32;
            let mut a = cfg(Arm::Play, Some(target), 30);
            a.catchup_speed = 64.0;
            a.resume_speed = 1.0;
            let mut c = TransportController::new(a);
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.inner.rec_coords = coords.clone();
            p.inner.recorded_count = 400;

            // Drive to JudgeBucket by hand: RacyPort wraps FakePort, and the
            // shared helper takes a FakePort.
            c.step(&mut p);
            p.inner.mode = TasMode::Off as u32;
            c.step(&mut p);
            p.inner.restart_state = 2;
            c.step(&mut p);
            c.step(&mut p); // ArmSettle: stages the handover, sends ArmPlay
            p.inner.mode = TasMode::Play as u32;
            p.inner.play_coords = coords;
            assert_eq!(p.inner.speed_handoff_pos, 251);

            // Next step: the handover fires DURING the controller's own speed
            // write. Without the second read the catch-up store wins and the
            // run keeps fast-forwarding until some later poll notices.
            p.armed = true;
            p.inner.playback_pos = 200;
            c.step(&mut p);
            assert_eq!(
                p.inner.playback_speed, 1.0,
                "a handover that fired mid-step was left overwritten"
            );
        }
        #[test]
        fn cont_stages_no_speed_handover() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            drive_to_judge(&mut c, &mut p);
            assert_eq!(p.speed_handoff_pos, 0);
            assert_eq!(p.speed_after_handoff, 0.0);

            // Something else knocks the speed down mid-replay (the in-process F5
            // restart does exactly this) - the controller must put it back.
            p.playback_speed = 1.0;
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);
            assert_eq!(p.playback_speed, 12.0);
        }

        /// A reroll re-arms from StopSettle, never through Phase::Start, so the
        /// staged handover has to be dropped in the reroll itself - otherwise it
        /// would fire against the next attempt at a position that means nothing.
        #[test]
        fn reroll_drops_the_staged_handover() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut a = cfg(Arm::Play, Some(target), 30);
            a.catchup_speed = 64.0;
            a.resume_speed = 1.0;
            let mut c = TransportController::new(a);
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            p.rec_coords = rec;
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);
            assert_eq!(p.speed_handoff_pos, 251);

            // This replay leaves the spawn one frame late: wrong bucket.
            let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
            play[251] = [1.0, 2.0, 3.5];
            p.play_coords = play;
            p.playback_pos = 320;
            p.dll_tick();
            assert_eq!(p.speed_handoff_pos, 0, "fired during the failed attempt");

            assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
            assert_eq!(p.speed_handoff_pos, 0, "no handover left armed by the reroll");
            assert_eq!(p.speed_after_handoff, 0.0);
            assert_eq!(
                p.playback_speed, 64.0,
                "the next attempt replays its countdown fast again"
            );

            // Re-arming stages a fresh handover for the new attempt.
            drive_reroll_to_judge(&mut c, &mut p);
            assert_eq!(p.speed_handoff_pos, 251);
        }

        /// A recording shorter than first_moving + BUCKET_MATCH_WINDOW can never
        /// be judged. The judge answers KeepWaiting forever, and mapping that
        /// straight to InProgress left the cycle spinning with nothing left to
        /// arrive. Once the replay has ended, KeepWaiting has to be terminal.
        #[test]
        fn judge_stops_when_the_replay_ends_before_it_can_rule() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 260];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 260; // < 250 + BUCKET_MATCH_WINDOW
            drive_to_judge(&mut c, &mut p);

            p.play_coords = coords;
            p.playback_pos = 260;
            // Still replaying: keep waiting, the window might still be reached.
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);

            // Replay over. Nothing more will arrive.
            p.mode = TasMode::Off as u32;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::Unjudged
                }
            );
            assert!(c.is_terminal());
        }

        /// PLAY passes continue_from_frame == 0 to the judge, which floors the
        /// validation depth at first_moving + BUCKET_MATCH_WINDOW - so a PLAY used
        /// to stop checking 64 ticks after the boarder moved while CONT kept
        /// checking for up to 1024. A PLAY is watched end to end, so a late
        /// divergence is more visible there, not less.
        #[test]
        fn play_validates_as_deep_as_cont() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 800];
            for (k, item) in rec.iter_mut().enumerate().take(800).skip(250) {
                *item = [1.0, 2.0, 3.0 + (k - 249) as f32 * 0.01];
            }
            p.rec_coords = rec.clone();
            p.recorded_count = 800;
            drive_to_judge(&mut c, &mut p);

            // Same first-moving frame, same settle - then it veers off long after
            // the old window closed.
            let mut play = rec;
            for item in play.iter_mut().take(800).skip(500) {
                item[2] += 3.0;
            }
            p.play_coords = play;

            p.playback_pos = 320;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::InProgress,
                "accepted at first_moving + 64 instead of validating deeper"
            );

            p.playback_pos = 600;
            assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
        }
        /// The controller and cave2 both write playback_speed, and the read that
        /// decides which speed to write is not part of the write. A step can read
        /// "handover still pending", have cave2 fire underneath it, and land its
        /// catch-up store AFTER the resume speed was installed - with the marker
        /// by then cleared, so nothing would ever look at it again. Going quiet
        /// once the handover fires is therefore not enough; the controller has to
        /// keep asserting the resume speed so a lost race is repaired.
        #[test]
        fn a_lost_speed_race_is_repaired_on_the_next_step() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut a = cfg(Arm::Play, Some(target), 30);
            a.catchup_speed = 64.0;
            a.resume_speed = 1.0;
            let mut c = TransportController::new(a);
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);
            p.play_coords = coords;

            p.playback_pos = 251;
            p.dll_tick();
            assert_eq!(p.playback_speed, 1.0);

            // The lost race: a catch-up store lands after the handover.
            p.playback_speed = 64.0;
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);
            assert_eq!(
                p.playback_speed, 1.0,
                "the controller must put the resume speed back, not just stay quiet"
            );
        }

        /// A final-attempt failure returns from reroll() early. A handover left
        /// staged across that return is inherited by whatever arms next -
        /// including a CONT, which would take a PLAY's resume speed and clock
        /// reset partway through its catch-up.
        #[test]
        fn abort_leaves_no_handover_armed() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut a = cfg(Arm::Play, Some(target), 0); // no retries left
            a.catchup_speed = 64.0;
            a.resume_speed = 1.0;
            let mut c = TransportController::new(a);
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            p.rec_coords = rec;
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);
            assert_eq!(p.speed_handoff_pos, 251);

            // Spawned somewhere else entirely: WrongStart, and no retries left.
            p.play_coords = vec![[9.0f32, 9.0, 9.0]; 400];
            p.playback_pos = 320;
            assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
            assert_eq!(p.speed_handoff_pos, 0, "abort left a handover armed");
            assert_eq!(p.speed_after_handoff, 0.0);
        }

        /// Mode is transient: a short replay at catch-up speed can begin and end
        /// entirely between two polls, so "we saw PLAY mode" is not a sound way to
        /// know the replay ran. Every later poll then reads OFF with no sighting
        /// and the cycle spins forever. The arm also zeroes playback_pos, and
        /// seeing THAT is durable.
        #[test]
        fn judge_finishes_when_play_mode_is_never_observed() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            // The whole replay ran between two polls: PLAY mode is never
            // sampled, and the only evidence it happened is that the arm counter
            // moved and the position is past the end.
            p.mode = TasMode::Off as u32;
            p.play_coords = coords;
            p.playback_pos = 400;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::BucketMatched
                }
            );
        }

        /// A refused arm (cave2 bounces ARM_CONTINUE from mid-run, for one) never
        /// enters PLAY and never replays anything, so the judge waits on a replay
        /// that will not happen. The arm counter counts refusals too, which is
        /// what turns that from a spin into a terminal outcome.
        ///
        /// And the outcome is ABORTED, not a quiet Done: nothing ran and nothing
        /// was judged. Reporting success left a refused CONT's 256x catch-up
        /// asserted with no splice ever coming to restore the speed - the UI only
        /// tears that down on Aborted.
        #[test]
        fn a_refused_arm_aborts_rather_than_reporting_success() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords;
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            // Refused: mode never became PLAY and nothing replayed.
            p.mode = TasMode::Off as u32;
            p.playback_pos = 0;
            assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
            assert!(c.is_terminal());
        }
        #[test]
        fn cont_matches_correct_bucket() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            // recording moves at frame 250
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            // replay reproduces it bit-for-bit → Match (pos past the 314 window)
            p.play_coords = coords;
            p.playback_pos = 320;

            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::BucketMatched
                }
            );
            assert!(c.is_terminal());
        }

        #[test]
        fn play_with_target_judges_and_matches() {
            // PLAY is judged whenever a target is supplied — the controller
            // gates on target.is_some(), not on the arm. This is what lets the
            // UI reroll a PLAY onto the recording's spawn bucket instead of
            // replaying whatever bucket the F5 happened to land.
            //
            // PLAY has no splice to validate through, so the controller passes
            // the recording length as the depth instead - the same rule CONT
            // uses for its splice, and still capped inside the judge at
            // first_moving + BUCKET_VALIDATE_WINDOW. So Match is only declared
            // once the replay has actually been validated that far.
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
            coords[250] = [1.0, 2.0, 3.5];
            p.rec_coords = coords.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            p.play_coords = coords;
            // Clean, but not yet validated to the recording length.
            p.playback_pos = 320;
            assert_eq!(c.step(&mut p), StepOutcome::InProgress);

            p.playback_pos = 400;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::BucketMatched
                }
            );
        }

        #[test]
        fn play_with_target_rerolls_wrong_bucket() {
            // The near-miss the UI toggle exists to prevent: same spawn bits, but
            // the boarder starts moving on a different frame, so the replay
            // diverges from the recording. Must reroll, not accept.
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            p.rec_coords = rec;
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            // identical spawn, movement one frame late
            let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
            play[251] = [1.0, 2.0, 3.5];
            p.play_coords = play;
            p.playback_pos = 320;
            assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
            assert!(!c.is_terminal());
        }

        #[test]
        fn play_without_target_finishes_unjudged() {
            // The toggle OFF path, and the pre-existing behaviour: no target, so
            // the controller arms and is done — no judging, no rerolls, playback
            // starts immediately on whatever bucket the restart landed.
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Play, None, 30));
            c.step(&mut p); // Stop
            p.mode = TasMode::Off as u32;
            c.step(&mut p); // -> Restart
            c.step(&mut p);
            p.restart_state = 2;
            c.step(&mut p); // -> ArmSettle (Wait)
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::Unjudged
                }
            );
        }

        #[test]
        fn cont_no_signal_accept_is_flagged() {
            // Target has no first-moving frame (recording never moved, or it was
            // undetected): the bucket can't be positively judged, so it's
            // accepted as NoSignal — which the UI surfaces as an unconfirmed
            // resume (the likely cause of a "wrong-frame" resume).
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: None,
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            p.rec_coords = vec![[1.0f32, 2.0, 3.0]; 400];
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);
            // start bits match, but there is no movement to discriminate buckets.
            p.play_coords = vec![[1.0f32, 2.0, 3.0]; 400];
            p.playback_pos = 320;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::NoSignal
                }
            );
        }

        #[test]
        fn cont_wrong_bucket_rerolls_then_matches() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            let mut right = vec![[1.0f32, 2.0, 3.0]; 400];
            right[250] = [1.0, 2.0, 3.5];
            p.rec_coords = right.clone();
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);

            // wrong bucket: moves at 248, not 250 → diverges from rec
            let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
            wrong[248] = [1.0, 2.0, 3.5];
            p.play_coords = wrong;
            p.playback_pos = 320;

            match c.step(&mut p) {
                StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
                other => panic!("expected Reroll, got {:?}", other),
            }
            // reroll sent Stop and is waiting for OFF again
            assert_eq!(p.commands.last(), Some(&TasCommand::Stop));

            // complete the reroll restart, this time reproducing the recording
            drive_reroll_to_judge(&mut c, &mut p);
            p.play_coords = right;
            p.playback_pos = 320;

            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 1,
                    completed_via: CompletedVia::BucketMatched
                }
            );
            assert!(!p.restart_while_not_off, "Restart sent while mode != OFF!");
        }

        #[test]
        fn cont_aborts_after_exhausting_retries() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            // only 1 retry allowed
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 1));
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            p.rec_coords = rec;
            p.recorded_count = 400;
            let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
            wrong[248] = [1.0, 2.0, 3.5];

            drive_to_judge(&mut c, &mut p);
            p.play_coords = wrong.clone();
            p.playback_pos = 320;
            // first wrong bucket -> reroll (attempt 1, uses the only retry)
            match c.step(&mut p) {
                StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
                other => panic!("expected Reroll, got {:?}", other),
            }
            drive_reroll_to_judge(&mut c, &mut p);
            p.play_coords = wrong;
            p.playback_pos = 320;
            // second wrong bucket -> no retries left -> abort
            match c.step(&mut p) {
                StepOutcome::Aborted { reason } => assert!(reason.contains("bucket mismatch")),
                other => panic!("expected Aborted, got {:?}", other),
            }
            assert!(c.is_terminal());
        }

        #[test]
        fn wrong_start_rerolls() {
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            drive_to_judge(&mut c, &mut p);
            // spawn position doesn't match the recording's start
            p.play_coords = vec![[9.0f32, 9.0, 9.0]; 300];
            p.playback_pos = 260;
            match c.step(&mut p) {
                StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
                other => panic!("expected Reroll for wrong start, got {:?}", other),
            }
        }

        #[test]
        fn cont_drives_splice_frame_to_the_game() {
            // "Continue must continue on the correct frame": the controller has
            // to push continue_from_frame (the splice frame) into the game both
            // before arming AND re-assert it on every reroll, or cave2 splices
            // at the wrong tick. cfg() sets continue_from_frame = 320 for CONT.
            let target = BucketTarget {
                expected_start_bits: bits(1.0, 2.0, 3.0),
                expected_first_moving: Some(250),
            };
            let mut p = FakePort {
                mode: TasMode::Rec as u32,
                ..Default::default()
            };
            let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
            let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
            rec[250] = [1.0, 2.0, 3.5];
            p.rec_coords = rec;
            p.recorded_count = 400;
            drive_to_judge(&mut c, &mut p);
            assert_eq!(
                p.continue_from_frame, 320,
                "splice frame not handed to the game before arming"
            );

            // Force a wrong bucket so the controller rerolls, after corrupting
            // the frame — the reroll must restore it.
            let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
            wrong[248] = [1.0, 2.0, 3.5];
            p.play_coords = wrong;
            p.playback_pos = 320;
            p.continue_from_frame = 0;
            match c.step(&mut p) {
                StepOutcome::Reroll { .. } => {}
                other => panic!("expected Reroll, got {:?}", other),
            }
            assert_eq!(
                p.continue_from_frame, 320,
                "splice frame not re-asserted on reroll"
            );
        }

        #[test]
        fn jitter_zero_early_then_bounded() {
            // First NATURAL_RESTART_ATTEMPTS use no jitter (natural variance).
            for a in 1..=NATURAL_RESTART_ATTEMPTS {
                assert_eq!(cont_retry_jitter_ms(a), 0, "attempt {} should not jitter", a);
            }
            // After that, a bounded non-zero escape jitter.
            for a in (NATURAL_RESTART_ATTEMPTS + 1)..=40 {
                let j = cont_retry_jitter_ms(a);
                assert!((1..=17).contains(&j), "jitter {} out of range", j);
            }
        }
    }
}

// Wire the live shared-memory client into the shared transport state machine.
// Inherent methods win for `self.method()` call syntax, so the same-named
// trait methods (send_command/restart_state/reset_restart_state) delegate to
// the inherent ones without recursing.
#[cfg(windows)]
impl transport::TransportPort for TasSharedMemoryClient {
    fn send_command(&mut self, cmd: TasCommand) {
        self.send_command(cmd);
    }
    fn mode(&self) -> u32 {
        self.mode_volatile()
    }
    fn restart_state(&self) -> u32 {
        self.restart_state()
    }
    fn reset_restart_state(&mut self) {
        self.reset_restart_state();
    }
    fn playback_pos(&self) -> u32 {
        self.playback_pos_volatile()
    }
    fn play_coords(&self) -> &[[f32; 3]] {
        &self.state().play_coords[..]
    }
    fn rec_coords(&self) -> &[[f32; 3]] {
        &self.state().rec_coords[..]
    }
    fn recorded_count(&self) -> u32 {
        self.state().recorded_count
    }
    fn set_continue_from_frame(&mut self, frame: u32) {
        self.state_mut().continue_from_frame = frame;
    }
    fn set_playback_speed(&mut self, speed: f32) {
        self.state_mut().playback_speed = speed;
    }
    fn set_speed_handoff(&mut self, pos: u32, speed: f32) {
        // Payload first, THEN the marker that arms it. cave2 tests the marker
        // and reads the speed only when it is armed, so this order is what
        // stops it acting on an armed handover with a stale speed beside it -
        // and a stale 0.0 there reads as "leave the speed alone", i.e. the run
        // would keep fast-forwarding with the request already consumed.
        //
        // Volatile, not plain, and with a compiler fence between. The other
        // side of these words is a different PROCESS, which Rust's memory model
        // cannot see: plain accesses may be reordered, merged, cached in a
        // register or elided entirely because nothing in this program appears
        // to read them. x86 not reordering stores only matters once the
        // compiler has emitted them in that order.
        let s = self.state_mut();
        unsafe {
            std::ptr::write_volatile(&mut s.speed_after_handoff as *mut f32, speed);
            compiler_fence(std::sync::atomic::Ordering::SeqCst);
            std::ptr::write_volatile(&mut s.speed_handoff_pos as *mut u32, pos);
        }
    }
    fn speed_handoff_pending(&self) -> bool {
        // Volatile for the same reason: cave2 clears this from the game
        // process, and a plain load can be hoisted out of the caller's loop.
        unsafe { std::ptr::read_volatile(&self.state().speed_handoff_pos as *const u32) != 0 }
    }
    fn arm_generation(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.state().arm_generation as *const u32) }
    }
}

#[cfg(not(windows))]
impl transport::TransportPort for TasSharedMemoryClient {
    fn send_command(&mut self, cmd: TasCommand) {
        self.send_command(cmd);
    }
    fn mode(&self) -> u32 {
        self.state().mode
    }
    fn restart_state(&self) -> u32 {
        self.restart_state()
    }
    fn reset_restart_state(&mut self) {
        self.reset_restart_state();
    }
    fn playback_pos(&self) -> u32 {
        self.state().playback_pos
    }
    fn play_coords(&self) -> &[[f32; 3]] {
        &self.state().play_coords[..]
    }
    fn rec_coords(&self) -> &[[f32; 3]] {
        &self.state().rec_coords[..]
    }
    fn recorded_count(&self) -> u32 {
        self.state().recorded_count
    }
    fn set_continue_from_frame(&mut self, frame: u32) {
        self.state_mut().continue_from_frame = frame;
    }
    fn set_playback_speed(&mut self, speed: f32) {
        self.state_mut().playback_speed = speed;
    }
    fn set_speed_handoff(&mut self, pos: u32, speed: f32) {
        // Payload first, THEN the marker that arms it. cave2 tests the marker
        // and reads the speed only when it is armed, so this order is what
        // stops it acting on an armed handover with a stale speed beside it -
        // and a stale 0.0 there reads as "leave the speed alone", i.e. the run
        // would keep fast-forwarding with the request already consumed.
        //
        // Volatile, not plain, and with a compiler fence between. The other
        // side of these words is a different PROCESS, which Rust's memory model
        // cannot see: plain accesses may be reordered, merged, cached in a
        // register or elided entirely because nothing in this program appears
        // to read them. x86 not reordering stores only matters once the
        // compiler has emitted them in that order.
        let s = self.state_mut();
        unsafe {
            std::ptr::write_volatile(&mut s.speed_after_handoff as *mut f32, speed);
            compiler_fence(std::sync::atomic::Ordering::SeqCst);
            std::ptr::write_volatile(&mut s.speed_handoff_pos as *mut u32, pos);
        }
    }
    fn speed_handoff_pending(&self) -> bool {
        // Volatile for the same reason: cave2 clears this from the game
        // process, and a plain load can be hoisted out of the caller's loop.
        unsafe { std::ptr::read_volatile(&self.state().speed_handoff_pos as *const u32) != 0 }
    }
    fn arm_generation(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.state().arm_generation as *const u32) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    fn zeroed_state() -> Box<TasSharedState> {
        zeroed_boxed()
    }

    // ========== Layout assertions ==========

    #[test]
    fn size_of_tas_shared_state_pinned() {
        // Pin the total struct size so C++ and Rust sides stay in sync.
        // align-8 struct. Tail: ..., clock_pin_enabled, clock_pin_phase,
        // test_arg4_override (v10, filled the v9 trailing pad), arg4_source
        // (v11, +8 = field + pad), then cont_suppress_input (v12) — fills
        // arg4_source's 4-byte trailing pad, so the total is unchanged at
        // 1_647_280. v13 appends present_count + menu_fps_cap (2x u32 = +8) ->
        // 1_647_288 (still 8-aligned, no extra pad).
        assert_eq!(mem::size_of::<TasSharedState>(), 1_647_480);
    }

    #[test]
    fn offset_of_input_log_pinned() {
        assert_eq!(mem::offset_of!(TasSharedState, input_log), 632);
    }

    #[test]
    fn offset_of_rec_coords() {
        assert_eq!(
            mem::offset_of!(TasSharedState, rec_coords),
            632 + TAS_MAX_TICKS // 66168
        );
    }

    #[test]
    fn offset_of_play_coords() {
        assert_eq!(
            mem::offset_of!(TasSharedState, play_coords),
            632 + TAS_MAX_TICKS + TAS_MAX_TICKS * 12 // 852600
        );
    }

    #[test]
    fn offset_of_log_write_seq() {
        assert_eq!(
            mem::offset_of!(TasSharedState, log_write_seq),
            632 + TAS_MAX_TICKS + TAS_MAX_TICKS * 12 * 2 // 1639032
        );
    }

    #[test]
    fn size_of_tas_log_entry() {
        // 4 (sequence) + 4 (severity) + 120 (text) = 128
        assert_eq!(mem::size_of::<TasLogEntry>(), 128);
    }

    #[test]
    fn size_of_tas_segment_boundary() {
        assert_eq!(mem::size_of::<TasSegmentBoundary>(), 8);
    }

    // ========== TasCommand enum round-trip ==========

    #[test]
    fn tas_command_round_trip() {
        let variants: &[(TasCommand, u32)] = &[
            (TasCommand::Idle, 0),
            (TasCommand::ArmRec, 1),
            (TasCommand::ArmPlay, 2),
            (TasCommand::Stop, 3),
            (TasCommand::ArmContinue, 4),
            (TasCommand::Restart, 5),
        ];
        for &(cmd, val) in variants {
            assert_eq!(cmd as u32, val, "{:?} should be {}", cmd, val);
        }
    }

    // ========== TasMode enum round-trip ==========

    #[test]
    fn tas_mode_round_trip() {
        assert_eq!(TasMode::Off as u32, 0);
        assert_eq!(TasMode::Rec as u32, 1);
        assert_eq!(TasMode::Play as u32, 2);
    }

    // ========== mode_str / mode_enum edge cases ==========

    #[test]
    fn mode_enum_known_values() {
        let mut state = zeroed_state();
        state.mode = 0;
        assert_eq!(state.mode_enum(), TasMode::Off);
        state.mode = 1;
        assert_eq!(state.mode_enum(), TasMode::Rec);
        state.mode = 2;
        assert_eq!(state.mode_enum(), TasMode::Play);
    }

    #[test]
    fn mode_enum_unknown_defaults_to_off() {
        let mut state = zeroed_state();
        for bogus in [3, 99, u32::MAX] {
            state.mode = bogus;
            assert_eq!(
                state.mode_enum(),
                TasMode::Off,
                "mode {} should map to Off",
                bogus
            );
        }
    }

    #[test]
    fn mode_str_known_values() {
        let mut state = zeroed_state();
        state.mode = 0;
        assert_eq!(state.mode_str(), "OFF");
        state.mode = 1;
        assert_eq!(state.mode_str(), "REC");
        state.mode = 2;
        assert_eq!(state.mode_str(), "PLAY");
    }

    #[test]
    fn mode_str_unknown_defaults_to_off() {
        let mut state = zeroed_state();
        state.mode = 42;
        assert_eq!(state.mode_str(), "OFF");
    }

    // ========== TasLogSeverity ==========

    #[test]
    fn log_severity_round_trip() {
        assert_eq!(TasLogSeverity::Debug as u32, 0);
        assert_eq!(TasLogSeverity::Info as u32, 1);
        assert_eq!(TasLogSeverity::Warn as u32, 2);
        assert_eq!(TasLogSeverity::Error as u32, 3);
    }

    #[test]
    fn log_entry_severity_unknown_defaults_to_info() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        entry.severity = 99;
        assert_eq!(entry.severity_enum(), TasLogSeverity::Info);
    }

    // ========== TasLogEntry::text_str ==========

    #[test]
    fn log_entry_text_str_normal() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        let msg = b"hello world";
        entry.text[..msg.len()].copy_from_slice(msg);
        assert_eq!(entry.text_str(), "hello world");
    }

    #[test]
    fn log_entry_text_str_full_buffer() {
        let mut entry: TasLogEntry = unsafe { mem::zeroed() };
        entry.text.fill(b'A');
        assert_eq!(entry.text_str().len(), TAS_LOG_ENTRY_SIZE);
    }

    #[test]
    fn log_entry_text_str_empty() {
        let entry: TasLogEntry = unsafe { mem::zeroed() };
        assert_eq!(entry.text_str(), "");
    }

    // ========== read_log_entries: cursor semantics ==========

    fn write_log_entry(state: &mut TasSharedState, seq: u32, severity: u32, text: &str) {
        let idx = (seq % TAS_LOG_RING_SIZE as u32) as usize;
        state.log_ring[idx].sequence = seq + 1; // entry stores seq+1
        state.log_ring[idx].severity = severity;
        let bytes = text.as_bytes();
        let len = bytes.len().min(TAS_LOG_ENTRY_SIZE);
        state.log_ring[idx].text[..len].copy_from_slice(&bytes[..len]);
        if len < TAS_LOG_ENTRY_SIZE {
            state.log_ring[idx].text[len] = 0;
        }
    }

    #[test]
    fn read_log_entries_empty() {
        let state = zeroed_state();
        let (entries, cursor) = state.read_log_entries(0);
        assert!(entries.is_empty());
        assert_eq!(cursor, 0);
    }

    #[test]
    fn read_log_entries_single() {
        let mut state = zeroed_state();
        write_log_entry(&mut state, 0, 1, "first");
        state.log_write_seq = 1;

        let (entries, cursor) = state.read_log_entries(0);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, 1); // sequence stored is seq+1
        assert_eq!(entries[0].2, "first");
        assert_eq!(cursor, 1);
    }

    #[test]
    fn read_log_entries_cursor_skips_already_seen() {
        let mut state = zeroed_state();
        for i in 0..5u32 {
            write_log_entry(&mut state, i, 0, &format!("msg{}", i));
        }
        state.log_write_seq = 5;

        // Read from cursor=3 should only get entries 3,4
        let (entries, cursor) = state.read_log_entries(3);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].2, "msg3");
        assert_eq!(entries[1].2, "msg4");
        assert_eq!(cursor, 5);
    }

    #[test]
    fn read_log_entries_cursor_at_write_seq_returns_empty() {
        let mut state = zeroed_state();
        write_log_entry(&mut state, 0, 0, "msg0");
        state.log_write_seq = 1;

        let (entries, cursor) = state.read_log_entries(1);
        assert!(entries.is_empty());
        assert_eq!(cursor, 1);
    }

    #[test]
    fn read_log_entries_wraparound() {
        let mut state = zeroed_state();
        // Write more than ring size entries — only last 64 should be visible
        let total = TAS_LOG_RING_SIZE as u32 + 10;
        for i in 0..total {
            write_log_entry(&mut state, i, 0, &format!("w{}", i));
        }
        state.log_write_seq = total;

        // Reading from 0 should only return last 64 (ring clamps)
        let (entries, cursor) = state.read_log_entries(0);
        assert_eq!(entries.len(), TAS_LOG_RING_SIZE);
        assert_eq!(entries[0].2, "w10"); // first visible after wraparound
        assert_eq!(cursor, total);
    }

    #[test]
    fn read_log_entries_sequence_gap_skips_stale() {
        let mut state = zeroed_state();
        // Write entry at seq=0 but set log_write_seq=2 (gap at seq=1)
        write_log_entry(&mut state, 0, 1, "zero");
        // Don't write seq=1 — its slot has sequence=0 (stale)
        state.log_write_seq = 2;

        let (entries, _) = state.read_log_entries(0);
        // seq=0 matches (entry.sequence == 0+1 == 1), seq=1 doesn't (slot has sequence=0, expects 2)
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].2, "zero");
    }

    // ========== TasSegmentBoundary defaults ==========

    #[test]
    fn segment_boundary_default() {
        let seg = TasSegmentBoundary::default();
        assert_eq!(seg.frame, 0);
        assert_eq!(seg.input_log_offset, 0);
    }

    // ========== input_bits constants match protocol ==========

    #[test]
    fn input_bits_values() {
        assert_eq!(input_bits::LEFT, 0x01);
        assert_eq!(input_bits::RIGHT, 0x02);
        assert_eq!(input_bits::UP, 0x04);
        assert_eq!(input_bits::DOWN, 0x08);
        assert_eq!(input_bits::JUMP, 0x10);
        assert_eq!(input_bits::SHIFT, 0x20);
    }

    #[test]
    fn input_bits_no_overlap() {
        let bits = [
            input_bits::LEFT,
            input_bits::RIGHT,
            input_bits::UP,
            input_bits::DOWN,
            input_bits::JUMP,
            input_bits::SHIFT,
        ];
        for i in 0..bits.len() {
            for j in (i + 1)..bits.len() {
                assert_eq!(bits[i] & bits[j], 0, "bits {} and {} overlap", i, j);
            }
        }
    }

    #[test]
    fn input_bits_all_table_consistent() {
        assert_eq!(input_bits::ALL.len(), 6);
        let expected = [
            (input_bits::LEFT, "L", "Left"),
            (input_bits::RIGHT, "R", "Right"),
            (input_bits::UP, "U", "Up"),
            (input_bits::DOWN, "D", "Down"),
            (input_bits::JUMP, "J", "Jump"),
            (input_bits::SHIFT, "S", "Shift"),
        ];
        for (i, &(bit, short, long)) in input_bits::ALL.iter().enumerate() {
            assert_eq!(bit, expected[i].0);
            assert_eq!(short, expected[i].1);
            assert_eq!(long, expected[i].2);
        }
    }

    #[test]
    fn input_bits_each_is_single_bit() {
        for &(bit, _, _) in input_bits::ALL {
            assert!(bit.is_power_of_two(), "0x{:02X} is not a single bit", bit);
        }
    }

    // ========== send_command policy: fft forced to 0 on arm ==========
    // These tests verify the fft=0 enforcement via the platform-independent
    // stub path (non-Windows) or live shared memory (Windows with DLL).

    fn make_client_or_skip() -> Option<TasSharedMemoryClient> {
        // On non-Windows, open() always fails so we use new_stub().
        // On Windows without game, open() fails too.
        #[cfg(not(windows))]
        {
            Some(TasSharedMemoryClient::new_stub())
        }
        #[cfg(windows)]
        {
            TasSharedMemoryClient::open().ok()
        }
    }

    #[test]
    fn send_command_resets_fft_on_arm_rec() {
        let Some(mut client) = make_client_or_skip() else {
            return;
        };
        client.state_mut().force_fixed_tick = 2;
        client.send_command(TasCommand::ArmRec);
        assert_eq!(
            client.state().force_fixed_tick,
            0,
            "ArmRec must reset fft to 0"
        );
    }

    #[test]
    fn send_command_resets_fft_on_arm_play() {
        let Some(mut client) = make_client_or_skip() else {
            return;
        };
        client.state_mut().force_fixed_tick = 5;
        client.send_command(TasCommand::ArmPlay);
        assert_eq!(
            client.state().force_fixed_tick,
            0,
            "ArmPlay must reset fft to 0"
        );
    }

    #[test]
    fn send_command_resets_fft_on_arm_continue() {
        let Some(mut client) = make_client_or_skip() else {
            return;
        };
        client.state_mut().force_fixed_tick = 3;
        client.send_command(TasCommand::ArmContinue);
        assert_eq!(
            client.state().force_fixed_tick,
            0,
            "ArmContinue must reset fft to 0"
        );
    }

    #[test]
    fn send_command_preserves_fft_on_stop() {
        let Some(mut client) = make_client_or_skip() else {
            return;
        };
        client.state_mut().force_fixed_tick = 2;
        client.send_command(TasCommand::Stop);
        assert_eq!(
            client.state().force_fixed_tick,
            2,
            "Stop must not reset fft"
        );
    }

    #[test]
    fn restart_state_helpers() {
        let Some(mut client) = make_client_or_skip() else {
            return;
        };
        let initial = client.restart_state();
        // Live shared memory may have any valid state (0=idle, 1=in_progress, 2=done)
        assert!(initial <= 2, "restart_state out of range: {}", initial);
        // Reset brings it to idle; on stub this is a no-op but should not panic
        client.reset_restart_state();
        assert_eq!(client.restart_state(), 0);
    }
}

#[cfg(test)]
mod level_seqlock_tests {
    use super::*;

    /// A reader must never observe a HALF-WRITTEN context — and the id it gets
    /// must belong to the path it gets.
    ///
    /// The path is a 128-byte array, so another process can be midway through
    /// rewriting it while the counters still read old; and `level_id` lives
    /// somewhere else entirely, so the two can disagree even when neither is
    /// itself torn. A writer thread publishes BOTH, byte-by-byte, with the
    /// sequence held odd, and the reader asserts the pair it gets is always one
    /// consistent publication.
    ///
    /// ON THE FORMAL DATA RACE: the payload is written with `write_volatile` and
    /// read with `read_volatile` from two threads, which Rust's memory model
    /// calls UB. That is deliberate and unavoidable here — the production reader
    /// has exactly this shape, because the real writer is ANOTHER PROCESS
    /// writing a shared mapping, which Rust's model has no vocabulary for at
    /// all. The target is x86: aligned byte and word accesses are indivisible,
    /// loads are not reordered with loads, and the mapping is cache-coherent, so
    /// the sequence is what actually orders things. This test models that, it
    /// does not launder it. Only `level_ctx_seq` is a real atomic, because that
    /// is the one location the protocol's correctness depends on.
    ///
    /// No `&mut TasSharedState` is ever created while the reader holds `&` —
    /// the writer goes through raw pointers via `addr_of_mut!`.
    #[test]
    fn seqlock_reader_never_sees_a_spliced_context() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        // Two publications with paths of the SAME length differing in every
        // byte, each paired with its own id, so a splice is detectable rather
        // than accidentally valid — and so an id/path mismatch is too.
        const PATH_A: &str = "data/levels/Forest/Tracks/Easy/aaaaa";
        const PATH_B: &str = "data/levels/Alpine/Tracks/Hard/bbbbb";
        const ID_A: u32 = 0; // Forest Easy
        const ID_B: u32 = 5; // Alpine Hard

        let mut boxed = zeroed_boxed();
        boxed.level_epoch = 1;
        boxed.level_scan_epoch = 1;
        let ptr = Box::into_raw(boxed) as usize;
        let stop = Arc::new(AtomicBool::new(false));

        let w_stop = stop.clone();
        let writer = std::thread::spawn(move || {
            let s = ptr as *mut TasSharedState;
            let mut which = false;
            while !w_stop.load(Ordering::Relaxed) {
                let (src, id) = if which {
                    (PATH_A.as_bytes(), ID_A)
                } else {
                    (PATH_B.as_bytes(), ID_B)
                };
                which = !which;
                unsafe {
                    // Mirror the DLL exactly: InterlockedIncrement to odd,
                    // mutate, InterlockedIncrement to even. AcqRel is what the
                    // Interlocked intrinsic gives, so the stand-in writer is not
                    // weaker than the real one.
                    (*s).level_ctx_seq.fetch_add(1, Ordering::AcqRel); // -> odd
                    std::ptr::write_volatile(std::ptr::addr_of_mut!((*s).level_id), id);
                    for (i, &c) in src.iter().enumerate() {
                        std::ptr::write_volatile(std::ptr::addr_of_mut!((*s).level_path[i]), c);
                        if i % 8 == 0 {
                            std::hint::spin_loop(); // widen the tear window
                        }
                    }
                    std::ptr::write_volatile(
                        std::ptr::addr_of_mut!((*s).level_path[src.len()]),
                        0,
                    );
                    (*s).level_ctx_seq.fetch_add(1, Ordering::AcqRel); // -> even
                }
                // Leave a stable window. The real writer publishes on level
                // changes only; a back-to-back loop would hold the sequence odd
                // almost always and starve the reader, which tests nothing.
                std::thread::sleep(std::time::Duration::from_micros(200));
            }
        });

        let s = unsafe { &*(ptr as *const TasSharedState) };
        let mut clean = 0usize;
        let mut saw_a = false;
        let mut saw_b = false;
        for _ in 0..20_000 {
            if let Some((id, path)) = level_context(s) {
                if path.is_empty() {
                    continue; // pre-first-publication zeros
                }
                match (path.as_str(), id) {
                    (PATH_A, ID_A) => saw_a = true,
                    (PATH_B, ID_B) => saw_b = true,
                    other => panic!(
                        "incoherent context: {:?} — expected exactly one publication, \
                         i.e. ({:?}, {}) or ({:?}, {})",
                        other, PATH_A, ID_A, PATH_B, ID_B
                    ),
                }
                clean += 1;
            }
        }

        stop.store(true, Ordering::Relaxed);
        writer.join().unwrap();
        unsafe { drop(Box::from_raw(ptr as *mut TasSharedState)) };

        // Reads must succeed, not merely never splice — a reader that always
        // returned None would pass every assertion above. And BOTH publications
        // must have been seen, which is what proves the reader was actually
        // running concurrently with the writer rather than sampling one quiet
        // value 20,000 times.
        assert!(clean > 0, "no clean read ever completed — reader is starving");
        assert!(
            saw_a && saw_b,
            "only ever saw one publication (A={} B={}) — the reader never \
             overlapped the writer, so nothing was actually tested",
            saw_a,
            saw_b
        );
    }

    /// An odd sequence means a write is in flight; the reader must refuse.
    #[test]
    fn odd_sequence_is_refused() {
        let mut s = zeroed_boxed();
        s.level_epoch = 4;
        s.level_scan_epoch = 4;
        s.level_ctx_seq.store(3, Ordering::Relaxed); // odd => mid-write
        assert_eq!(level_context(&s), None, "must not read while seq is odd");

        s.level_ctx_seq.store(4, Ordering::Relaxed); // even => stable
        assert!(level_context(&s).is_some());
    }

    /// Unresolved must stay unresolved even when the sequence is clean.
    #[test]
    fn clean_sequence_does_not_imply_resolved() {
        let mut s = zeroed_boxed();
        s.level_ctx_seq.store(8, Ordering::Relaxed);
        s.level_epoch = 5;
        s.level_scan_epoch = 4; // scan has not caught up
        s.level_id = 2;
        assert_eq!(level_context(&s), None);
    }
}
