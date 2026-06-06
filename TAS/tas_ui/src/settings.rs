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
    pub show_trajectory: bool,

    // Playback
    pub playback_speed: f32,
    pub cont_catchup_speed: f32,

    /// Max UNPINNED undo-history entries kept (the v2 store soft cap). Pinned
    /// entries and the current entry are always kept. Default preserves the old
    /// 500-entry depth so existing histories migrate without trimming.
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
            // 96× is the sweet spot: the catch-up saturates the game's 64
            // ticks/frame cap at ~77×+, so 96× is full-speed, and the splice
            // resume is now frame-exact at any speed (cave5 prev_time reset), so
            // there's no precision reason to stay at 64×. 96× over 128× because
            // the F5 bucket lottery rerolls a little more at 128× (each miss is a
            // ~1s restart), and 96× keeps better one-shot reliability for ~the
            // same catch-up time.
            cont_catchup_speed: 96.0,
            history_cap: 500,
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
        // Migrate stale catch-up defaults to the current 96× default. 20× was
        // the pre-cave5 slider cap; 64× was the prior default before the
        // frame-exact resume fix made higher speeds safe. Bump those exact
        // values — anything else (30, 40, 100, 128, etc.) is a deliberate user
        // choice and we leave it alone.
        let s = settings.cont_catchup_speed;
        if (s - 20.0).abs() < f32::EPSILON || (s - 64.0).abs() < f32::EPSILON {
            settings.cont_catchup_speed = 96.0;
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
