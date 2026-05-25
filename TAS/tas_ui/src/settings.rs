use serde::{Deserialize, Serialize};

/// Persistent settings saved between sessions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    // View panel toggles
    pub show_pico_panel: bool,
    pub show_debug_drift: bool,
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
            show_debug_drift: false,
            show_history: false,
            show_config: false,
            show_log: false,
            playback_speed: 1.0,
            // Cave5 patches the game's tick clamp from 0x14 (20) to 0x40
            // (64) at install time, lifting effective catch-up from ~12×
            // to ~57× (measured). The catchup-speed setting controls
            // tick_advance scaling — at 64× setting the game's frame
            // loop spends less time per tick, so frame rate (and
            // playback throughput) goes up. 64× one-shot reliability
            // is ~80% on real recordings (vs 65% at 128×), and the
            // auto-reroll handles the misses transparently. Slider
            // allows up to 128× for power users; at 128 we're close
            // to the per-frame overhead ceiling so going higher is
            // diminishing returns.
            cont_catchup_speed: 64.0,
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
        let mut settings: Self = match std::fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        };
        // Migrate the stale 20.0 catch-up default. Pre-cave5 the slider
        // capped at 20×; users who never touched it have that value
        // persisted, but the effective game cap is now 64×. Bump those
        // specific values up — anything else (30, 40, 100, etc.) is a
        // deliberate user choice and we leave it alone.
        if (settings.cont_catchup_speed - 20.0).abs() < f32::EPSILON {
            settings.cont_catchup_speed = 64.0;
        }
        settings
    }

    pub fn save(&self) {
        let path = settings_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, json);
        }
    }
}
