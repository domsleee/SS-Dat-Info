//! The Win32 client for the `Local\SupremeTAS` mapping TAS_Helper.dll creates.

use std::ffi::CString;
use std::sync::atomic::{compiler_fence, AtomicU32, Ordering};

use crate::menu_screen;
use crate::state::{
    rider_pair, TasCommand, TasSharedState, TAS_CMD_CLAIMED_STOP, TAS_SHARED_MEMORY_NAME,
    TAS_SHARED_VERSION,
};
use crate::{physics_mode_label, rider_label, transport};

// Raw Win32 FFI — avoids windows-sys version churn
type Handle = *mut std::ffi::c_void;
const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;

extern "system" {
    #[cfg(any(test, feature = "test-mapping"))]
    fn CreateFileMappingA(
        file: Handle,
        attrs: *const std::ffi::c_void,
        protect: u32,
        size_high: u32,
        size_low: u32,
        name: *const u8,
    ) -> Handle;
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
        if cfg!(test) {
            return Err("Unit tests must not open the live game mapping".into());
        }
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

    #[cfg(any(test, feature = "test-mapping"))]
    pub fn new_test_mapping() -> Self {
        unsafe {
            // Unnamed pagefile-backed mapping: same Windows client code,
            // but no game can see or consume these test commands.
            let handle = CreateFileMappingA(
                (-1isize) as Handle,
                std::ptr::null(),
                0x04,
                0,
                std::mem::size_of::<TasSharedState>() as u32,
                std::ptr::null(),
            );
            assert!(!handle.is_null(), "CreateFileMappingA failed");
            let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 0) as *mut TasSharedState;
            if ptr.is_null() {
                CloseHandle(handle);
                panic!("MapViewOfFile failed");
            }
            (*ptr).version = TAS_SHARED_VERSION;
            Self { handle, ptr }
        }
    }

    pub fn state(&self) -> &TasSharedState {
        unsafe { &*self.ptr }
    }

    /// A `&mut` over the live mapping, which the DLL writes concurrently,
    /// so this is not a sound exclusive borrow: the seqlocked readers and
    /// the volatile stores in the `TransportPort` impl exist because of
    /// that. Use it for fields only this side writes (the command region,
    /// `menu_cmd_*`) and for tests on a private mapping.
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
            (&*(cmd_ptr.cast::<AtomicU32>())).store(cmd as u32, Ordering::Release);
        }
    }

    /// Acquire-read the command publication/acknowledgement word.
    fn command_word(&self) -> u32 {
        unsafe {
            let cmd_ptr = std::ptr::addr_of!((*self.ptr).command);
            (&*(cmd_ptr.cast::<AtomicU32>())).load(Ordering::Acquire)
        }
    }

    pub fn command_idle(&self) -> bool {
        self.command_word() == TasCommand::Idle as u32
    }

    /// True while a STOP is published or being consumed by the DLL.
    pub fn stop_pending(&self) -> bool {
        let cmd = self.command_word();
        cmd == TasCommand::Stop as u32
            || cmd == TasCommand::StopForRestart as u32
            || cmd == TAS_CMD_CLAIMED_STOP
    }

    /// Raw x87 control word the DLL sampled on the game thread (v41).
    pub fn fpu_control_word(&self) -> u32 {
        unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*self.ptr).fpu_control_word)) }
    }

    /// Loaded renderer plugin (`TAS_RENDERER_*`, v41).
    pub fn renderer_id(&self) -> u32 {
        unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*self.ptr).renderer_id)) }
    }

    /// Live physics-mode stamp (see `physics_mode_label`).
    pub fn physics_mode(&self) -> Option<String> {
        physics_mode_label(self.renderer_id(), self.fpu_control_word())
    }

    /// Live rider stamp (character · stance, v42), `None` until the DLL
    /// has resolved the human rider's loadout.
    pub fn rider(&self) -> Option<String> {
        let (character, stance) = rider_pair(self.state());
        rider_label(character, stance)
    }

    /// The current menu screen title (v44), `None` in a level.
    pub fn menu_screen(&self) -> Option<String> {
        menu_screen(self.state())
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

// Wire the live shared-memory client into the shared transport state machine.
// Inherent methods win for `self.method()` call syntax, so the same-named
// trait methods (send_command/restart_state/reset_restart_state) delegate to
// the inherent ones without recursing.
impl transport::TransportPort for TasSharedMemoryClient {
    fn send_command(&mut self, cmd: TasCommand) {
        self.send_command(cmd);
    }
    fn command_idle(&self) -> bool {
        self.command_idle()
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
    fn gate_index(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.state().gate_index as *const u32) }
    }
    fn set_continue_from_frame(&mut self, frame: u32) {
        unsafe {
            std::ptr::write_volatile(&mut self.state_mut().continue_from_frame as *mut u32, frame);
        }
    }
    fn set_gate_align_rec(&mut self, frame: u32) {
        unsafe {
            std::ptr::write_volatile(&mut self.state_mut().gate_align_rec as *mut u32, frame);
        }
    }
    fn set_playback_speed(&mut self, speed: f32) {
        unsafe {
            std::ptr::write_volatile(&mut self.state_mut().playback_speed as *mut f32, speed);
        }
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
    fn arm_restart_tick(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.state().arm_restart_tick as *const u32) }
    }
    fn arm_consumed_tick(&self) -> u32 {
        unsafe { std::ptr::read_volatile(&self.state().arm_consumed_tick as *const u32) }
    }
    fn capture_ok(&self) -> bool {
        unsafe { std::ptr::read_volatile(&self.state().capture_ok as *const u32) != 0 }
    }
    fn approve_cont_splice(&mut self) {
        // Volatile: the reader is cave5 in another process (see
        // set_speed_handoff for why plain stores are not enough).
        let s = self.state_mut();
        unsafe {
            std::ptr::write_volatile(&mut s.cont_splice_approved as *mut u32, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests use private memory, never the live game mapping.

    /// Two private test mappings must not see each other's writes, and
    /// neither may be the game's mapping. (Whether the game's mapping exists
    /// right now is a property of the machine, not of this crate.)
    #[test]
    fn unit_tests_cannot_open_live_game_memory() {
        let mut a = TasSharedMemoryClient::new_test_mapping();
        let b = TasSharedMemoryClient::new_test_mapping();
        a.send_command(TasCommand::ArmRec);
        assert_eq!(b.state().command, TasCommand::Idle as u32);
    }

    /// Arming REC, PLAY or CONT forces `force_fixed_tick` back to 0, the
    /// proven zero-drift configuration.
    #[test]
    fn send_command_resets_fft_on_arm() {
        for cmd in [
            TasCommand::ArmRec,
            TasCommand::ArmPlay,
            TasCommand::ArmContinue,
        ] {
            let mut client = TasSharedMemoryClient::new_test_mapping();
            client.state_mut().force_fixed_tick = 2;
            client.send_command(cmd);
            assert_eq!(
                client.state().force_fixed_tick,
                0,
                "{:?} must reset fft to 0",
                cmd
            );
        }
    }

    #[test]
    fn send_command_preserves_fft_on_stop() {
        let mut client = TasSharedMemoryClient::new_test_mapping();
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
        let mut client = TasSharedMemoryClient::new_test_mapping();
        client.state_mut().restart_state = 2;
        assert_eq!(client.restart_state(), 2);
        client.reset_restart_state();
        assert_eq!(client.restart_state(), 0);
    }
}
