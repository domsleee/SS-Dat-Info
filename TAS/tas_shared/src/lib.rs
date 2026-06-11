pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";
pub const TAS_SHARED_VERSION: u32 = 13; // +present_count/menu_fps_cap (menu SwapBuffers throttle)
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
        // Minimum settle trajectory before we judge the fingerprint at all.
        let min_judge = expected_fm + BUCKET_MATCH_WINDOW;
        if playback_pos < min_judge {
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
    }

    impl TransportController {
        pub fn new(cfg: ArmConfig) -> Self {
            let retries_remaining = cfg.max_retries;
            Self {
                cfg,
                phase: Phase::Start,
                retries_remaining,
                completed_via: CompletedVia::Unjudged,
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
            port.set_playback_speed(self.cfg.catchup_speed);
            match self.phase {
                Phase::Start => {
                    port.set_playback_speed(self.cfg.catchup_speed);
                    port.set_continue_from_frame(self.cfg.continue_from_frame);
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
                    let mode = port.mode();
                    if mode == rec {
                        // Splice already fired (PLAY→REC) — bucket accepted.
                        return self.finish(CompletedVia::Unjudged);
                    }
                    if mode != play {
                        return StepOutcome::InProgress;
                    }
                    let pos = port.playback_pos();
                    if pos == 0 {
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
                        // Validate the bucket clean through the splice point —
                        // not just the settle window — so a late divergence
                        // rerolls instead of splicing onto a wrong trajectory.
                        self.cfg.continue_from_frame,
                    );
                    match verdict {
                        BucketVerdict::KeepWaiting => StepOutcome::InProgress,
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
            commands: Vec<TasCommand>,
            /// Invariant tracker: Restart must NEVER be sent while mode != OFF.
            restart_while_not_off: bool,
        }

        impl TransportPort for FakePort {
            fn send_command(&mut self, cmd: TasCommand) {
                if cmd == TasCommand::Restart && self.mode != TasMode::Off as u32 {
                    self.restart_while_not_off = true;
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
            c.step(p); // ArmSettle -> ArmContinue, phase -> JudgeBucket
            assert_eq!(p.commands.last(), Some(&TasCommand::ArmContinue));
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
        assert_eq!(mem::size_of::<TasSharedState>(), 1_647_288);
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
