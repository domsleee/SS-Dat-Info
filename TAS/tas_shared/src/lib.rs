pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";
pub const TAS_SHARED_VERSION: u32 = 7; // Phase 7: settle trace telemetry
pub const TAS_MAX_TICKS: usize = 65536;
pub const TAS_MAX_SEGMENTS: usize = 32;
pub const TAS_LOG_RING_SIZE: usize = 64;
pub const TAS_LOG_ENTRY_SIZE: usize = 120;
pub const TAS_SETTLE_TRACE_SIZE: usize = 1024;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasCommand {
    Idle = 0,
    ArmRec = 1,
    ArmPlay = 2,
    Stop = 3,
    ArmContinue = 4,
    Restart = 5,
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

/// A settle trace entry: position at one frame during post-restart settle (16 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TasSettleTraceEntry {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub frame: u32,
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

    // Settle trace (DLL writes during MODE_OFF after restart, tests read)
    pub settle_trace_enabled: u32,
    pub settle_trace_count: u32,
    pub settle_trace: [TasSettleTraceEntry; TAS_SETTLE_TRACE_SIZE],
}

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

        /// Enable or disable settle trace capture in the DLL.
        pub fn set_settle_trace_enabled(&mut self, enabled: bool) {
            unsafe {
                let ptr = std::ptr::addr_of_mut!((*self.ptr).settle_trace_enabled);
                std::ptr::write_volatile(ptr, if enabled { 1 } else { 0 });
            }
        }

        /// Read the current settle trace count.
        pub fn settle_trace_count(&self) -> u32 {
            unsafe {
                let ptr = std::ptr::addr_of!((*self.ptr).settle_trace_count);
                std::ptr::read_volatile(ptr)
            }
        }

        /// Read settle trace entries (up to count).
        pub fn read_settle_trace(&self) -> Vec<TasSettleTraceEntry> {
            let count = self.settle_trace_count() as usize;
            let count = count.min(TAS_SETTLE_TRACE_SIZE);
            let s = self.state();
            s.settle_trace[..count].to_vec()
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
        assert_eq!(mem::size_of::<TasSharedState>(), 1_663_624);
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
    fn size_of_tas_settle_trace_entry() {
        assert_eq!(mem::size_of::<TasSettleTraceEntry>(), 16);
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
