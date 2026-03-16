use std::ffi::CString;
use std::ptr;

pub const TAS_SHARED_MEMORY_NAME: &str = "Local\\SupremeTAS";
pub const TAS_SHARED_VERSION: u32 = 1;
pub const TAS_MAX_TICKS: usize = 65536;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasCommand {
    Idle = 0,
    ArmRec = 1,
    ArmPlay = 2,
    Stop = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TasMode {
    Off = 0,
    Rec = 1,
    Play = 2,
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

    // Hook status
    pub cave2_hooked: u32,
    pub cave1c_hooked: u32,
    pub cave1d_hooked: u32,
    pub cave5_hooked: u32,

    pub _reserved: [u32; 8],

    pub input_log: [u8; TAS_MAX_TICKS],
    pub rec_coords: [[f32; 3]; TAS_MAX_TICKS],
    pub play_coords: [[f32; 3]; TAS_MAX_TICKS],
}

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
