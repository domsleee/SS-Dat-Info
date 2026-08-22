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

    /// Reroll the F5 restart on PLAY until it lands the recording's spawn
    /// bucket, the way CONT already does.
    ///
    /// OFF, a PLAY starts on whatever bucket the restart happened to land, and a
    /// near-miss diverges from the recording as soon as the boarder moves
    /// (~tick 299) — the replay silently stops being the run you recorded.
    ///
    /// ON used to cost real time: the judge cannot rule until the replay has
    /// passed the recording's first moving frame, which at 1x is ~3s of a
    /// stationary boarder on the attempt that succeeds AND on every reroll.
    /// `play_judge_speed` removed most of that by replaying the countdown fast
    /// and handing back the moment the boarder moves - measured on FE-10065,
    /// 3161 ms to first movement at 1x against 331 ms at 64x, with the replay
    /// bit-identical either way. The switch stays because the judge speed can
    /// be set to 1x, and because a user may simply not want rerolls.
    pub play_bucket_match: bool,

    /// Speed to replay the pre-movement countdown at while a bucket-matched
    /// PLAY is being judged. 1.0 disables the catch-up and plays it at normal
    /// speed.
    ///
    /// The judge cannot rule on the bucket until the replay has passed the
    /// recording's first moving frame, and for a real recording that is ~2.6s
    /// of watching a boarder stand still through the countdown - paid on the
    /// attempt that succeeds AND on every reroll. Nothing in that stretch is
    /// worth watching, so it is replayed fast; the DLL hands the speed back at
    /// the exact tick the boarder starts moving.
    #[serde(default = "default_play_judge_speed")]
    pub play_judge_speed: f32,
}

/// Default judge-time catch-up speed for a bucket-matched PLAY.
///
/// 64x, matching the CONT catch-up multiplier's usual setting. Higher is
/// possible - CONT judges at 256x - but the countdown is only ~2.6s, so
/// past ~64x the restart dominates and there is nothing left to win.
fn default_play_judge_speed() -> f32 {
    64.0
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
            // Default ON: a replay that quietly diverges from the recording is
            // worse than a replay that takes longer to start, and until now PLAY
            // had no way to tell you it had landed the wrong bucket.
            play_bucket_match: true,
            play_judge_speed: default_play_judge_speed(),
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
    fn old_settings_file_without_play_bucket_match_defaults_on() {
        // A settings file written before play_bucket_match existed must still
        // load, with the new key defaulting on. Without #[serde(default)] on the
        // struct this deserialize fails, load() falls back to Default, and every
        // other setting the user had chosen is silently reset — which is a much
        // louder bug than the one this field was added for.
        let old = r#"{
            "show_pico_panel": true,
            "show_debug_drift": false,
            "show_history": true,
            "show_config": false,
            "show_log": true,
            "show_trajectory": false,
            "playback_speed": 1.0,
            "cont_catchup_speed": 128.0,
            "history_cap": 250
        }"#;
        let s: Settings = serde_json::from_str(old).expect("old file must still parse");
        assert!(s.play_bucket_match, "new field defaults on");
        // The struct-level #[serde(default)] would give this f32 0.0, which
        // reads as "no catch-up" and silently returns every old settings file
        // to the slow judge. The per-field default is what stops that.
        assert_eq!(
            s.play_judge_speed,
            default_play_judge_speed(),
            "a missing judge speed must default to the catch-up, not 0"
        );
        // and the user's existing choices survive
        assert!(s.show_pico_panel);
        assert!(s.show_log);
        assert_eq!(s.history_cap, 250);
        assert_eq!(s.cont_catchup_speed, 128.0);
    }

    #[test]
    fn play_bucket_match_round_trips() {
        let mut s = Settings::default();
        s.play_bucket_match = false;
        let json = serde_json::to_string(&s).unwrap();
        let back: Settings = serde_json::from_str(&json).unwrap();
        assert!(!back.play_bucket_match);
    }
}
