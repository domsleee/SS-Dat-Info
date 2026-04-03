use serde::{Deserialize, Serialize};

/// Persistent settings saved between sessions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    // View panel toggles
    pub show_pico_panel: bool,
    pub show_segments: bool,
    pub show_trajectory: bool,
    pub show_rotation: bool,
    pub show_analysis: bool,
    pub show_debug_drift: bool,
    pub show_macros: bool,
    pub show_history: bool,
    pub show_config: bool,
    pub show_log: bool,

    // Playback
    pub playback_speed: f32,
    pub cont_catchup_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_pico_panel: false,
            show_segments: false,
            show_trajectory: false,
            show_rotation: false,
            show_analysis: false,
            show_debug_drift: false,
            show_macros: false,
            show_history: false,
            show_config: false,
            show_log: false,
            playback_speed: 1.0,
            cont_catchup_speed: 12.0,
        }
    }
}

fn settings_path() -> std::path::PathBuf {
    // Store next to the executable
    if let Ok(exe) = std::env::current_exe() {
        exe.with_file_name("ssb_inspect_settings.json")
    } else {
        std::path::PathBuf::from("ssb_inspect_settings.json")
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
