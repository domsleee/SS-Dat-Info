//! Persistent UI settings and the per-install data directory everything else
//! (history, recovery, recordings, the session log) lives under.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    // View panel toggles
    pub show_pico_panel: bool,
    pub show_debug_drift: bool,
    pub show_history: bool,
    pub show_config: bool,
    pub show_log: bool,
    pub show_trajectory: bool,

    // Playback
    pub playback_speed: f32,
    pub cont_catchup_speed: f32,

    /// Max UNPINNED undo-history entries kept (the v2 store soft cap). Pinned
    /// entries and the current entry are always kept.
    pub history_cap: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_pico_panel: false,
            show_debug_drift: false,
            show_history: false,
            show_config: false,
            show_log: false,
            show_trajectory: false,
            playback_speed: 1.0,
            // The catch-up replay is physics-compute-bound at ~80x effective;
            // 256x saturates that ceiling without costing reliability.
            cont_catchup_speed: 256.0,
            history_cap: 500,
        }
    }
}

fn settings_path() -> PathBuf {
    if let Some(root) = std::env::var_os("SSB_INSPECT_DATA_DIR") {
        return PathBuf::from(root).join("ssb_inspect_settings.json");
    }
    // Store next to the executable
    if let Ok(exe) = std::env::current_exe() {
        exe.with_file_name("ssb_inspect_settings.json")
    } else {
        PathBuf::from("ssb_inspect_settings.json")
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();
        match std::fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let path = settings_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, json);
        }
    }
}

/// Root directory for all per-install TAS data. Resolution order:
///   1. `SSB_INSPECT_DATA_DIR` (tests / custom setups).
///   2. `<game>/Display_Config_Resources/TAS/data` when tas_ui runs deployed
///      inside a `Display_Config_Resources/TAS` folder, so each game install
///      keeps its own recordings and history. A `data/` subfolder rather than a
///      `tas` sibling: that would collide with `TAS` on case-insensitive
///      Windows.
///   3. `~/.ssb-inspector` for un-deployed dev builds.
pub fn data_root_dir() -> PathBuf {
    if let Some(over) = std::env::var_os("SSB_INSPECT_DATA_DIR") {
        return PathBuf::from(over);
    }
    if let Some(dir) = game_data_root_from_exe() {
        return dir;
    }
    user_home_dir()
        .map(|home| home.join(".ssb-inspector"))
        .unwrap_or_else(|| PathBuf::from(".ssb-inspector"))
}

/// `<exe dir>/data` when the running exe sits in a `Display_Config_Resources`
/// child folder (the deployment layout); `None` for a dev build under
/// `target/`, so no stray `data` dir appears there.
fn game_data_root_from_exe() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exe_dir = exe.parent()?;
    let parent = exe_dir.parent()?;
    if parent
        .file_name()?
        .to_str()?
        .eq_ignore_ascii_case("Display_Config_Resources")
    {
        Some(exe_dir.join("data"))
    } else {
        None
    }
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .or_else(|| {
            let drive = std::env::var_os("HOMEDRIVE")?;
            let path = std::env::var_os("HOMEPATH")?;
            let mut buf = PathBuf::from(drive);
            buf.push(path);
            Some(buf)
        })
}
