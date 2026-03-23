use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use tas_shared::TasSharedState;

/// Cached drift plot data, invalidated when recorded_count or playback_pos changes.
#[derive(Default)]
pub struct DriftCache {
    drift_x: Vec<[f64; 2]>,
    drift_z: Vec<[f64; 2]>,
    last_recorded: u32,
    last_played: u32,
}

impl DriftCache {
    /// Update the cache if the underlying data has changed. Returns true if refreshed.
    pub fn refresh(&mut self, state: &TasSharedState) -> bool {
        let recorded = state.recorded_count;
        let played = state.playback_pos;
        if recorded == self.last_recorded && played == self.last_played {
            return false;
        }
        self.last_recorded = recorded;
        self.last_played = played;

        let count = (played as usize).min(recorded as usize);
        if count == 0 {
            self.drift_x.clear();
            self.drift_z.clear();
            return true;
        }

        let step = (count / 1000).max(1);
        self.drift_x.clear();
        self.drift_z.clear();
        for i in (0..count).step_by(step) {
            let dx = (state.play_coords[i][0] - state.rec_coords[i][0]) as f64;
            self.drift_x.push([i as f64, dx]);
            let dz = (state.play_coords[i][2] - state.rec_coords[i][2]) as f64;
            self.drift_z.push([i as f64, dz]);
        }
        true
    }
}

pub fn show(ui: &mut egui::Ui, state: &TasSharedState, cache: &mut DriftCache) {
    let recorded = state.recorded_count as usize;
    let played = state.playback_pos as usize;

    if recorded == 0 {
        ui.label("No data");
        return;
    }

    let count = played.min(recorded);
    if count == 0 {
        ui.label("Max drift: X=0.000000000 Z=0.000000000 (no playback data)");
        return;
    }

    cache.refresh(state);

    let avail_height = ui.available_height().clamp(120.0, 300.0);

    Plot::new("drift_plot")
        .height(avail_height)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(PlotPoints::new(cache.drift_x.clone()))
                    .name("Drift X")
                    .color(egui::Color32::from_rgb(100, 149, 237)),
            );
            plot_ui.line(
                Line::new(PlotPoints::new(cache.drift_z.clone()))
                    .name("Drift Z")
                    .color(egui::Color32::from_rgb(255, 165, 0)),
            );
            // Zero reference line
            plot_ui.line(
                Line::new(PlotPoints::new(vec![[0.0, 0.0], [count as f64, 0.0]]))
                    .name("Zero")
                    .color(egui::Color32::from_rgb(255, 60, 60))
                    .style(egui_plot::LineStyle::dashed_dense()),
            );
        });
}
