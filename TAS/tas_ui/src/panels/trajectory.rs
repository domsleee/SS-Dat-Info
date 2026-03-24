use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use tas_shared::TasSharedState;

/// Cached trajectory plot data, invalidated when recorded_count or playback_pos changes.
#[derive(Default)]
pub struct TrajectoryCache {
    rec_points: Vec<[f64; 2]>,
    play_points: Vec<[f64; 2]>,
    rec_altitude: Vec<[f64; 2]>,
    play_altitude: Vec<[f64; 2]>,
    last_recorded: u32,
    last_played: u32,
}

impl TrajectoryCache {
    /// Update the cache if the underlying data has changed. Returns true if refreshed.
    pub fn refresh(&mut self, state: &TasSharedState) -> bool {
        let recorded = state.recorded_count;
        let played = state.playback_pos;
        if recorded == self.last_recorded && played == self.last_played {
            return false;
        }
        self.last_recorded = recorded;
        self.last_played = played;

        let rec = recorded as usize;
        let play = played as usize;

        // REC trajectory (X/Z top-down) + altitude (frame vs Y)
        self.rec_points.clear();
        self.rec_altitude.clear();
        if rec > 0 {
            let step = (rec / 2000).max(1);
            for i in (0..rec).step_by(step) {
                self.rec_points
                    .push([state.rec_coords[i][0] as f64, state.rec_coords[i][2] as f64]);
                self.rec_altitude
                    .push([i as f64, state.rec_coords[i][1] as f64]);
            }
        }

        // PLAY trajectory (X/Z top-down) + altitude (frame vs Y)
        self.play_points.clear();
        self.play_altitude.clear();
        if play > 0 {
            let step = (play / 2000).max(1);
            for i in (0..play).step_by(step) {
                self.play_points.push([
                    state.play_coords[i][0] as f64,
                    state.play_coords[i][2] as f64,
                ]);
                self.play_altitude
                    .push([i as f64, state.play_coords[i][1] as f64]);
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

    ui.label(egui::RichText::new("Top-down (X/Z)").small());
    Plot::new("trajectory_xz")
        .height(top_height)
        .data_aspect(1.0)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .x_axis_label("X")
        .y_axis_label("Z")
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            if !cache.rec_points.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.rec_points.clone()))
                        .name("REC path")
                        .color(egui::Color32::from_rgb(100, 149, 237))
                        .width(2.0),
                );
            }
            if !cache.play_points.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.play_points.clone()))
                        .name("PLAY path")
                        .color(egui::Color32::from_rgb(80, 200, 80))
                        .width(2.0),
                );
            }
            plot_ui.points(
                Points::new(vec![current_pos])
                    .name("Current")
                    .color(egui::Color32::from_rgb(255, 255, 100))
                    .radius(5.0),
            );
        });

    ui.add_space(4.0);
    ui.label(egui::RichText::new("Altitude (Y)").small());
    Plot::new("trajectory_altitude")
        .height(bottom_height)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .x_axis_label("Frame")
        .y_axis_label("Y (altitude)")
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            if !cache.rec_altitude.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.rec_altitude.clone()))
                        .name("REC altitude")
                        .color(egui::Color32::from_rgb(100, 149, 237))
                        .width(1.5),
                );
            }
            if !cache.play_altitude.is_empty() {
                plot_ui.line(
                    Line::new(PlotPoints::new(cache.play_altitude.clone()))
                        .name("PLAY altitude")
                        .color(egui::Color32::from_rgb(80, 200, 80))
                        .width(1.5),
                );
            }
            // Current altitude marker
            let current_frame = state.playback_pos.max(state.recorded_count) as f64;
            let current_alt = state.player_y as f64;
            plot_ui.points(
                Points::new(vec![[current_frame, current_alt]])
                    .name("Current alt")
                    .color(egui::Color32::from_rgb(255, 255, 100))
                    .radius(4.0),
            );
        });
}
