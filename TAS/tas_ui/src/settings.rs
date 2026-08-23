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
            // CONT catch-up speed (tick_advance scaling). The catch-up replay is
            // PHYSICS-COMPUTE-bound, not cap- or render-bound: measured sweep on
            // FE-10065@6200 shows effective rate asymptotes to a hard ~80× ceiling
            // (~8000 ticks/sec, ~125µs/tick) — 96×→66×, 192×→77×, 384×→80×. So the
            // setting saturates; 256× captures ~all of it (~80× effective, ~17%
            // faster replay than 96×) with reliability + zero-drift + frame-exact
            // resume fully preserved (12/12 clean, first-try 9/12 at 256× — no
            // regression vs 96×). Past ~256× is pure diminishing returns. The F5
            // bucket lottery is set at RESTART, not replay speed, so higher catch-up
            // doesn't cost reliability. (Rendering-suppression was tried and proven
            // useless; the cave5 64 tick/frame cap is never even reached — the game
            // runs only ~20 ticks/frame.)
            cont_catchup_speed: 256.0,
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
        // Migrate stale catch-up defaults to the current 256× default. 20× was
        // the pre-cave5 slider cap; 64× and 96× were prior defaults before the
        // physics-ceiling sweep showed 256× is ~all the achievable speed (~80×
        // effective) with no reliability cost. Bump those exact values — anything
        // else (30, 40, 100, 128, etc.) is a deliberate user choice, left alone.
        let s = settings.cont_catchup_speed;
        if (s - 20.0).abs() < f32::EPSILON
            || (s - 64.0).abs() < f32::EPSILON
            || (s - 96.0).abs() < f32::EPSILON
        {
            settings.cont_catchup_speed = 256.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_play_bucket_settings_are_ignored_without_resetting_other_values() {
        let old = r#"{
            "show_pico_panel": true,
            "show_debug_drift": false,
            "show_history": true,
            "show_config": false,
            "show_log": true,
            "show_trajectory": false,
            "playback_speed": 1.0,
            "cont_catchup_speed": 128.0,
            "history_cap": 250,
            "play_bucket_match": false,
            "play_judge_speed": 32.0
        }"#;
        let s: Settings = serde_json::from_str(old).expect("old file must still parse");
        assert!(s.show_pico_panel);
        assert!(s.show_log);
        assert_eq!(s.history_cap, 250);
        assert_eq!(s.cont_catchup_speed, 128.0);
    }
}
