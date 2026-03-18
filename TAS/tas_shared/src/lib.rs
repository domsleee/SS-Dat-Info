pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";
pub const TAS_SHARED_VERSION: u32 = 4; // Phase 4: segment fields
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

impl TasLogEntry {
    pub fn text_str(&self) -> &str {
        let len = self.text.iter().position(|&b| b == 0).unwrap_or(TAS_LOG_ENTRY_SIZE);
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
    pub frame: u32,              // Frame number where this segment starts
    pub input_log_offset: u32,   // Offset into input_log for this segment
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

    // Telemetry (DLL writes, UI reads)
    pub prev_player_x: f32,
    pub prev_player_y: f32,
    pub prev_player_z: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub speed: f32,         // Squared speed (XZ plane) — UI should sqrt for display
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

    pub input_log: [u8; TAS_MAX_TICKS],
    pub rec_coords: [[f32; 3]; TAS_MAX_TICKS],
    pub play_coords: [[f32; 3]; TAS_MAX_TICKS],

    // Log ring buffer (DLL writes, UI reads)
    pub log_write_seq: u32,
    pub log_ring: [TasLogEntry; TAS_LOG_RING_SIZE],
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
                entries.push((entry.sequence, entry.severity_enum(), entry.text_str().to_string()));
            }
        }
        (entries, write_seq)
    }
}

// --- Platform-specific shared memory client ---

#[cfg(windows)]
mod platform {
    use super::*;
    use std::ffi::CString;

    // Raw Win32 FFI — avoids windows-sys version churn
    type HANDLE = *mut std::ffi::c_void;
    const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;

    extern "system" {
        fn OpenFileMappingA(desired_access: u32, inherit_handle: i32, name: *const u8) -> HANDLE;
        fn MapViewOfFile(
            file_mapping: HANDLE,
            desired_access: u32,
            offset_high: u32,
            offset_low: u32,
            bytes_to_map: usize,
        ) -> *mut std::ffi::c_void;
        fn UnmapViewOfFile(base_address: *const std::ffi::c_void) -> i32;
        fn CloseHandle(handle: HANDLE) -> i32;
    }

    /// Opens the named shared memory created by TAS_Helper.dll.
    pub struct TasSharedMemoryClient {
        handle: HANDLE,
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
                    return Err(
                        "OpenFileMappingA failed (is TAS_Helper.dll loaded?)".into(),
                    );
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
            unsafe {
                let cmd_ptr = std::ptr::addr_of_mut!((*self.ptr).command);
                std::ptr::write_volatile(cmd_ptr, cmd as u32);
            }
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

        pub fn state(&self) -> &TasSharedState {
            &self.state
        }

        pub fn state_mut(&mut self) -> &mut TasSharedState {
            &mut self.state
        }

        pub fn send_command(&mut self, cmd: TasCommand) {
            let _ = cmd;
        }
    }
}

pub use platform::TasSharedMemoryClient;
