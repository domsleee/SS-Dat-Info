use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use tas_shared::TasSharedState;

const FRAMES_PER_SECOND: f64 = 100.0;

/// Cached trajectory plot data. The key is the counts PLUS the playback
/// generation and a content fingerprint: restoring another snapshot with the
/// same length and an unchanged playback position replaces the coordinates
/// without moving either count, and a counts-only key keeps drawing the
/// previous recording. A generation key alone is not enough either —
/// history restores do not arm playback.
#[derive(Default)]
pub struct TrajectoryCache {
    rec_points: Vec<[f64; 2]>,
    play_points: Vec<[f64; 2]>,
    rec_altitude: Vec<[f64; 2]>,
    play_altitude: Vec<[f64; 2]>,
    last_recorded: u32,
    last_played: u32,
    last_generation: u32,
    last_fingerprint: u64,
}

/// Cheap content identity for the cache key (FNV-1a over a sparse sample):
/// every 256th input byte plus both endpoints of each buffer. Full-buffer
/// hashing would cost 786 KB per frame; the sample catches replacement
/// (which rewrites the whole buffer) while count/generation changes are
/// caught by the other key halves.
fn content_fingerprint(state: &TasSharedState) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    let mut mix = |w: u64| {
        h ^= w;
        h = h.wrapping_mul(FNV_PRIME);
    };
    let rec = state.recorded_count as usize;
    let play = state.playback_pos as usize;
    let mut i = 0;
    while i < rec {
        mix(state.input_log[i] as u64);
        i += 256;
    }
    if rec > 0 {
        for v in state.rec_coords[0].map(f32::to_bits) {
            mix(v as u64);
        }
        for v in state.rec_coords[rec - 1].map(f32::to_bits) {
            mix(v as u64);
        }
    }
    if play > 0 {
        let p = play.min(tas_shared::TAS_MAX_TICKS as usize) - 1;
        for v in state.play_coords[p].map(f32::to_bits) {
            mix(v as u64);
        }
    }
    h
}

impl TrajectoryCache {
    /// Update the cache if the underlying data has changed. Returns true if refreshed.
    pub fn refresh(&mut self, state: &TasSharedState) -> bool {
        let recorded = state.recorded_count;
        let played = state.playback_pos;
        let generation = state.arm_generation;
        let fingerprint = content_fingerprint(state);
        if recorded == self.last_recorded
            && played == self.last_played
            && generation == self.last_generation
            && fingerprint == self.last_fingerprint
        {
            return false;
        }
        self.last_recorded = recorded;
        self.last_played = played;
        self.last_generation = generation;
        self.last_fingerprint = fingerprint;

        let rec = recorded as usize;
        let play = played as usize;

        // REC trajectory (X/Z top-down) + altitude (time vs Y)
        self.rec_points.clear();
        self.rec_altitude.clear();
        if rec > 0 {
            let step = (rec / 2000).max(1);
            for i in (0..rec).step_by(step) {
                self.rec_points
                    .push([state.rec_coords[i][0] as f64, state.rec_coords[i][2] as f64]);
                self.rec_altitude.push([
                    i as f64 / FRAMES_PER_SECOND,
                    -(state.rec_coords[i][1] as f64),
                ]);
            }
        }

        // PLAY trajectory (X/Z top-down) + altitude (time vs Y)
        self.play_points.clear();
        self.play_altitude.clear();
        if play > 0 {
            let step = (play / 2000).max(1);
            for i in (0..play).step_by(step) {
                self.play_points.push([
                    state.play_coords[i][0] as f64,
                    state.play_coords[i][2] as f64,
                ]);
                self.play_altitude.push([
                    i as f64 / FRAMES_PER_SECOND,
                    -(state.play_coords[i][1] as f64),
                ]);
            }
        }
        true
    }
}

pub fn show(ui: &mut egui::Ui, state: &TasSharedState, cache: &mut TrajectoryCache) {
    if state.recorded_count == 0 {
        ui.label("No trajectory data");
        return;
    }

    cache.refresh(state);

    let current_pos = [state.player_x as f64, state.player_z as f64];
    let avail_height = ui.available_height().max(240.0);
    let top_height = (avail_height * 0.6).max(120.0);
    let bottom_height = (avail_height * 0.4).max(80.0);

    // Sub-labels + per-plot legends removed — the parent frame already
    // says "Trajectory + Rotation", and the path colours are stable
    // across the session so a once-visible legend just adds clutter.
    Plot::new("trajectory_xz")
        .height(top_height)
        .data_aspect(1.0)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .x_axis_label("X")
        .y_axis_label("Z")
        .show(ui, |plot_ui| {
            if !cache.rec_points.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.rec_points.clone()))
                        .name("REC path")
                        .color(egui::Color32::from_rgb(100, 149, 237))
                        .width(2.0_f32),
                );
            }
            if !cache.play_points.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.play_points.clone()))
                        .name("PLAY path")
                        .color(egui::Color32::from_rgb(80, 200, 80))
                        .width(2.0_f32),
                );
            }
            plot_ui.points(
                Points::new(vec![current_pos])
                    .name("Current")
                    .color(egui::Color32::from_rgb(255, 255, 100))
                    .radius(5.0_f32),
            );
        });

    ui.add_space(4.0);
    Plot::new("trajectory_altitude")
        .height(bottom_height)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .x_axis_label("Time (s)")
        .y_axis_label("Y (altitude)")
        .show(ui, |plot_ui| {
            if !cache.rec_altitude.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.rec_altitude.clone()))
                        .name("REC altitude")
                        .color(egui::Color32::from_rgb(100, 149, 237))
                        .width(1.5_f32),
                );
            }
            if !cache.play_altitude.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.play_altitude.clone()))
                        .name("PLAY altitude")
                        .color(egui::Color32::from_rgb(80, 200, 80))
                        .width(1.5_f32),
                );
            }
            // Current altitude marker
            let current_time =
                state.playback_pos.max(state.recorded_count) as f64 / FRAMES_PER_SECOND;
            let current_alt = -(state.player_y as f64);
            plot_ui.points(
                Points::new(vec![[current_time, current_alt]])
                    .name("Current alt")
                    .color(egui::Color32::from_rgb(255, 255, 100))
                    .radius(4.0_f32),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Altitude values are negated for display (game Y-up inverted for downhill view).
    /// Reference: SS-Dat-Info negates Y in transformPosition.
    #[test]
    fn altitude_negates_y_for_rec() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 3;
        state.rec_coords[0] = [0.0, 100.0, 0.0];
        state.rec_coords[1] = [1.0, 50.0, 1.0];
        state.rec_coords[2] = [2.0, -20.0, 2.0];

        let mut cache = TrajectoryCache::default();
        cache.refresh(&state);

        assert_eq!(cache.rec_altitude.len(), 3);
        assert_eq!(cache.rec_altitude[0][0], 0.0);
        assert_eq!(cache.rec_altitude[1][0], 0.01);
        assert_eq!(cache.rec_altitude[2][0], 0.02);
        assert_eq!(cache.rec_altitude[0][1], -100.0);
        assert_eq!(cache.rec_altitude[1][1], -50.0);
        assert_eq!(cache.rec_altitude[2][1], 20.0);
    }

    /// Same negation for PLAY altitude.
    #[test]
    fn altitude_negates_y_for_play() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 1;
        state.rec_coords[0] = [0.0, 0.0, 0.0];
        state.playback_pos = 2;
        state.play_coords[0] = [0.0, 200.0, 0.0];
        state.play_coords[1] = [1.0, -75.0, 1.0];

        let mut cache = TrajectoryCache::default();
        cache.refresh(&state);

        assert_eq!(cache.play_altitude.len(), 2);
        assert_eq!(cache.play_altitude[0][0], 0.0);
        assert_eq!(cache.play_altitude[1][0], 0.01);
        assert_eq!(cache.play_altitude[0][1], -200.0);
        assert_eq!(cache.play_altitude[1][1], 75.0);
    }

    /// Top-down X/Z coordinates are NOT negated.
    #[test]
    fn xz_coordinates_not_negated() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 1;
        state.rec_coords[0] = [10.0, 999.0, 30.0];

        let mut cache = TrajectoryCache::default();
        cache.refresh(&state);

        assert_eq!(cache.rec_points[0][0], 10.0); // X
        assert_eq!(cache.rec_points[0][1], 30.0); // Z
    }

    /// Replacing a recording with another of the same length (counts and
    /// generation unchanged) must still refresh: the old plot is wrong data.
    #[test]
    fn equal_length_replacement_refreshes() {
        let mut cache = TrajectoryCache::default();
        let mut first = tas_shared::zeroed_boxed();
        first.recorded_count = 2;
        first.rec_coords[0] = [0.0, 0.0, 0.0];
        first.rec_coords[1] = [1.0, 1.0, 1.0];
        assert!(cache.refresh(&first));
        assert!(!cache.refresh(&first));
        let mut second = tas_shared::zeroed_boxed();
        second.recorded_count = 2;
        second.rec_coords[0] = [9.0, 9.0, 9.0];
        second.rec_coords[1] = [8.0, 8.0, 8.0];
        assert!(
            cache.refresh(&second),
            "same counts and generation, different bytes: must refresh"
        );
        assert_eq!(cache.rec_points[0], [9.0, 9.0]);
        assert!(!cache.refresh(&second));
    }

    /// Cache dedup: refresh returns false when data hasn't changed.
    #[test]
    fn cache_dedup_skips_unchanged() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 1;
        state.rec_coords[0] = [1.0, 2.0, 3.0];

        let mut cache = TrajectoryCache::default();
        assert!(cache.refresh(&state)); // first call: changed
        assert!(!cache.refresh(&state)); // second call: no change
    }
}
