//! What the app observes of, and does to, the machine outside its window:
//! the clock and its waits, the global keyboard, the game process, the
//! native window, file pickers and launched files. `RealHost` is the app;
//! the headless frame tests substitute a fake (`tests/e2e`).

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub(crate) trait Host {
    fn now(&self) -> Instant;
    /// One pause of a bounded poll loop (the cycle's tight poll, the STOP
    /// wait before a buffer overwrite).
    fn wait(&self, d: Duration);
    /// Whether the virtual key is held right now, whichever window has focus.
    fn key_is_down(&self, vk: i32) -> bool;
    /// The running Supreme.exe, if any.
    fn game_pid(&self) -> Option<u32>;
    /// The process owning the foreground window.
    fn foreground_pid(&self) -> Option<u32>;
    fn set_dark_title_bar(&self, title: &str);
    /// The Load dialog, starting in `level_id`'s folder. None = cancelled.
    fn pick_load_path(&self, level_id: u32) -> Option<PathBuf>;
    /// The Save dialog for a take on `level`. None = cancelled.
    fn pick_save_path(&self, level: Option<&str>, default_name: &str) -> Option<PathBuf>;
    /// Open a file in its default application (the external script editor).
    fn open_file(&self, path: &Path);
    fn open_folder(&self, path: &Path) -> Result<(), String>;
    /// Where divergence reports go.
    fn diagnostics_dir(&self) -> PathBuf;
}

pub(crate) struct RealHost;

impl Host for RealHost {
    fn now(&self) -> Instant {
        Instant::now()
    }
    fn wait(&self, d: Duration) {
        std::thread::sleep(d);
    }
    fn key_is_down(&self, vk: i32) -> bool {
        crate::win32::key_is_down(vk)
    }
    fn game_pid(&self) -> Option<u32> {
        crate::win32::find_supreme_pid()
    }
    fn foreground_pid(&self) -> Option<u32> {
        crate::win32::foreground_window_pid()
    }
    fn set_dark_title_bar(&self, title: &str) {
        crate::win32::set_dark_title_bar(title);
    }
    fn pick_load_path(&self, level_id: u32) -> Option<PathBuf> {
        crate::recording::pick_recording_path(level_id)
    }
    fn pick_save_path(&self, level: Option<&str>, default_name: &str) -> Option<PathBuf> {
        crate::recording::pick_save_path(level, default_name)
    }
    fn open_file(&self, path: &Path) {
        use std::os::windows::process::CommandExt;
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("")
            .arg(path)
            .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
            .spawn();
    }
    fn open_folder(&self, path: &Path) -> Result<(), String> {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("failed to launch explorer: {}", e))?;
        Ok(())
    }
    fn diagnostics_dir(&self) -> PathBuf {
        crate::settings::data_root_dir().join("diagnostics")
    }
}
