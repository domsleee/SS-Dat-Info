use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, Points};
use tas_shared::TasSharedState;

/// Cached trajectory plot data, invalidated when recorded_count or playback_pos changes.
#[derive(Default)]
pub struct TrajectoryCache {
    rec_points: Vec<[f64; 2]>,
    play_points: Vec<[f64; 2]>,
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

        // REC trajectory
        self.rec_points.clear();
        if rec > 0 {
            let step = (rec / 2000).max(1);
            for i in (0..rec).step_by(step) {
                self.rec_points
                    .push([state.rec_coords[i][0] as f64, state.rec_coords[i][2] as f64]);
            }
        }

        // PLAY trajectory
        self.play_points.clear();
        if play > 0 {
            let step = (play / 2000).max(1);
            for i in (0..play).step_by(step) {
                self.play_points.push([
                    state.play_coords[i][0] as f64,
                    state.play_coords[i][2] as f64,
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

    // Current position marker
    let current_pos = [state.player_x as f64, state.player_z as f64];

    let avail_height = ui.available_height().max(200.0);

    Plot::new("trajectory_plot")
        .height(avail_height)
        .data_aspect(1.0)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
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
            // Current position dot
            plot_ui.points(
                Points::new(vec![current_pos])
                    .name("Current")
                    .color(egui::Color32::from_rgb(255, 255, 100))
                    .radius(5.0),
            );
        });
}
