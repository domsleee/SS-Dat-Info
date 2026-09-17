//! Win32 bindings shared by every module that touches the game window or the
//! screen. Every `extern` declaration lives here so a signature is declared once.

use std::time::Duration;

pub type Hwnd = isize;
pub type Handle = isize;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_ppm: i32,
    pub y_ppm: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}

#[repr(C)]
pub struct BitmapInfo {
    pub header: BitmapInfoHeader,
    pub colors: [u32; 3],
}

pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const VK_RETURN: u8 = 0x0D;
pub const VK_LEFT: i32 = 0x25;
pub const VK_F11: u8 = 0x7A;
pub const VK_F12: u8 = 0x7B;
pub const SRCCOPY: u32 = 0x00CC_0020;
pub const DIB_RGB_COLORS: u32 = 0;
const SW_RESTORE: i32 = 9;
const KEYEVENTF_KEYUP: u32 = 0x0002;

/// The game's exact window title; the only thing that identifies its window.
pub const GAME_TITLE: &[u8] = b"Supreme Snowboarding Copyright (C) 1999 by Housemarque, Inc.\0";

#[link(name = "user32")]
unsafe extern "system" {
    pub fn FindWindowA(class: *const u8, title: *const u8) -> Hwnd;
    pub fn SetForegroundWindow(hwnd: Hwnd) -> i32;
    pub fn GetForegroundWindow() -> Hwnd;
    pub fn PostMessageW(hwnd: Hwnd, msg: u32, wparam: usize, lparam: isize) -> i32;
    pub fn GetWindowRect(hwnd: Hwnd, rect: *mut Rect) -> i32;
    pub fn GetWindowThreadProcessId(hwnd: Hwnd, pid: *mut u32) -> u32;
    pub fn AttachThreadInput(attach: u32, to: u32, attach_flag: i32) -> i32;
    pub fn ShowWindow(hwnd: Hwnd, cmd: i32) -> i32;
    pub fn BringWindowToTop(hwnd: Hwnd) -> i32;
    pub fn IsIconic(hwnd: Hwnd) -> i32;
    pub fn IsWindow(hwnd: Hwnd) -> i32;
    pub fn GetAsyncKeyState(vk: i32) -> i16;
    pub fn keybd_event(vk: u8, scan: u8, flags: u32, extra: usize);
    pub fn GetDesktopWindow() -> Hwnd;
    pub fn GetDC(hwnd: Hwnd) -> Handle;
    pub fn ReleaseDC(hwnd: Hwnd, hdc: Handle) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetCurrentThreadId() -> u32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    pub fn CreateCompatibleDC(hdc: Handle) -> Handle;
    pub fn DeleteDC(hdc: Handle) -> i32;
    pub fn CreateCompatibleBitmap(hdc: Handle, w: i32, h: i32) -> Handle;
    pub fn DeleteObject(o: Handle) -> i32;
    pub fn SelectObject(hdc: Handle, o: Handle) -> Handle;
    pub fn BitBlt(
        dst: Handle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        src: Handle,
        sx: i32,
        sy: i32,
        rop: u32,
    ) -> i32;
    pub fn GetDIBits(
        hdc: Handle,
        hbm: Handle,
        start: u32,
        lines: u32,
        bits: *mut u8,
        bi: *mut BitmapInfo,
        usage: u32,
    ) -> i32;
}

pub fn find_game_window() -> Option<Hwnd> {
    let hwnd = unsafe { FindWindowA(std::ptr::null(), GAME_TITLE.as_ptr()) };
    (hwnd != 0).then_some(hwnd)
}

/// Make `hwnd` the foreground window and report whether it took.
///
/// `SetForegroundWindow` refuses unless the caller owns the foreground thread's
/// input, so attach to that thread first (what `AppActivate` does internally).
pub fn bring_to_front(hwnd: Hwnd) -> bool {
    unsafe {
        if IsIconic(hwnd) != 0 {
            ShowWindow(hwnd, SW_RESTORE);
        }
        let me = GetCurrentThreadId();
        let foreground = GetForegroundWindow();
        let owner = if foreground != 0 {
            GetWindowThreadProcessId(foreground, std::ptr::null_mut())
        } else {
            0
        };
        let attached = owner != 0 && owner != me && AttachThreadInput(me, owner, 1) != 0;
        BringWindowToTop(hwnd);
        SetForegroundWindow(hwnd);
        if attached {
            AttachThreadInput(me, owner, 0);
        }
        GetForegroundWindow() == hwnd
    }
}

pub fn is_foreground(hwnd: Hwnd) -> bool {
    unsafe { GetForegroundWindow() == hwnd }
}

pub fn is_window(hwnd: Hwnd) -> bool {
    unsafe { IsWindow(hwnd) != 0 }
}

pub fn key_is_down(vk: i32) -> bool {
    unsafe { GetAsyncKeyState(vk) < 0 }
}

pub fn window_rect(hwnd: Hwnd) -> Option<Rect> {
    let mut rect = Rect::default();
    (unsafe { GetWindowRect(hwnd, &mut rect) } != 0).then_some(rect)
}

/// Post a key press straight into `hwnd`'s message queue. Needs no focus, so it
/// reaches a modal dialog that blocks focus changes.
pub fn post_key(hwnd: Hwnd, vk: u8, hold: Duration) {
    unsafe {
        PostMessageW(hwnd, WM_KEYDOWN, vk as usize, 0);
    }
    std::thread::sleep(hold);
    unsafe {
        PostMessageW(hwnd, WM_KEYUP, vk as usize, 0);
    }
}

/// Synthesize a key press on the system input queue (whatever has focus sees it).
pub fn tap_key(vk: u8, hold: Duration) {
    unsafe {
        keybd_event(vk, 0, 0, 0);
    }
    std::thread::sleep(hold);
    unsafe {
        keybd_event(vk, 0, KEYEVENTF_KEYUP, 0);
    }
}
