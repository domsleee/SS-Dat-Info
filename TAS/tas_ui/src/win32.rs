//! The Win32 calls the UI makes directly: the game-process lookup behind the
//! global-shortcut gate and the liveness check, the dark title bar, and
//! focusing an already-running instance.

#![allow(clippy::upper_case_acronyms)] // Win32 type names.

use std::ffi::c_void;

type HANDLE = *mut c_void;
type HWND = *mut c_void;
type DWORD = u32;
type BOOL = i32;

const TH32CS_SNAPPROCESS: DWORD = 0x0000_0002;
const MAX_PATH: usize = 260;
const INVALID_HANDLE_VALUE: HANDLE = -1isize as *mut c_void;
const DWMWA_USE_IMMERSIVE_DARK_MODE: DWORD = 20;

pub const VK_F9: i32 = 0x78;
pub const VK_F10: i32 = 0x79;
pub const VK_F11: i32 = 0x7A;
pub const VK_F12: i32 = 0x7B;

#[repr(C)]
struct ProcessEntry32W {
    dw_size: DWORD,
    cnt_usage: DWORD,
    th32_process_id: DWORD,
    th32_default_heap_id: usize,
    th32_module_id: DWORD,
    cnt_threads: DWORD,
    th32_parent_process_id: DWORD,
    pc_pri_class_base: i32,
    dw_flags: DWORD,
    sz_exe_file: [u16; MAX_PATH],
}

extern "system" {
    fn CreateToolhelp32Snapshot(flags: DWORD, pid: DWORD) -> HANDLE;
    fn Process32FirstW(snap: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
    fn Process32NextW(snap: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
    fn CloseHandle(h: HANDLE) -> BOOL;
    fn FindWindowW(class: *const u16, title: *const u16) -> HWND;
    fn SetForegroundWindow(hwnd: HWND) -> BOOL;
    fn GetForegroundWindow() -> HWND;
    fn GetWindowThreadProcessId(hwnd: HWND, pid: *mut DWORD) -> DWORD;
    fn GetAsyncKeyState(vk: i32) -> i16;
    fn DwmSetWindowAttribute(hwnd: HWND, attr: DWORD, value: *const c_void, size: DWORD) -> i32;
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// PID of the running game (`Supreme.exe` or the versioned `Supreme_v1.035.exe`;
/// the injector accepts both, so the shortcut gate must too).
pub fn find_supreme_pid() -> Option<u32> {
    let targets: [Vec<u16>; 2] = [
        "Supreme.exe".encode_utf16().collect(),
        "Supreme_v1.035.exe".encode_utf16().collect(),
    ];
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return None;
        }
        let mut entry: ProcessEntry32W = std::mem::zeroed();
        entry.dw_size = std::mem::size_of::<ProcessEntry32W>() as DWORD;
        let mut ok = Process32FirstW(snap, &mut entry);
        while ok != 0 {
            let len = entry
                .sz_exe_file
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(MAX_PATH);
            let name = &entry.sz_exe_file[..len];
            if targets.iter().any(|t| name == t.as_slice()) {
                CloseHandle(snap);
                return Some(entry.th32_process_id);
            }
            ok = Process32NextW(snap, &mut entry);
        }
        CloseHandle(snap);
    }
    None
}

pub fn is_supreme_running() -> bool {
    find_supreme_pid().is_some()
}

/// Dark title bar on Windows 10+ for the window with this exact title.
pub fn set_dark_title_bar(title: &str) {
    let title = wide(title);
    unsafe {
        let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
        if !hwnd.is_null() {
            let value: BOOL = 1;
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &value as *const BOOL as *const c_void,
                std::mem::size_of::<BOOL>() as DWORD,
            );
        }
    }
}

/// Bring the window with this exact title to the front (the single-instance
/// guard uses it to surface the instance that is already running).
pub fn focus_window_titled(title: &str) {
    let title = wide(title);
    unsafe {
        let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
        if !hwnd.is_null() {
            SetForegroundWindow(hwnd);
        }
    }
}

/// Whether the virtual key is held right now, whichever window has focus.
pub fn key_is_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}

/// PID owning the foreground window, `None` when there is none.
pub fn foreground_window_pid() -> Option<u32> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_null() {
        return None;
    }
    let mut pid: DWORD = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    Some(pid)
}
