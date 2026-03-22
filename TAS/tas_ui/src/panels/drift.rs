use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use tas_shared::TasSharedState;

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    let recorded = state.recorded_count as usize;
    let played = state.playback_pos as usize;

    if recorded == 0 {
        ui.label("No data");
        return;
    }

    // Compute per-tick drift (only meaningful if we have both rec and play coords)
    let count = played.min(recorded);
    if count == 0 {
        ui.label("Max drift: X=0.000000000 Z=0.000000000 (no playback data)");
        return;
    }

    // Sample at most 1000 points for performance
    let step = (count / 1000).max(1);

    let drift_x: Vec<[f64; 2]> = (0..count)
        .step_by(step)
        .map(|i| {
            let dx = (state.play_coords[i][0] - state.rec_coords[i][0]) as f64;
            [i as f64, dx]
        })
        .collect();

    let drift_z: Vec<[f64; 2]> = (0..count)
        .step_by(step)
        .map(|i| {
            let dz = (state.play_coords[i][2] - state.rec_coords[i][2]) as f64;
            [i as f64, dz]
        })
        .collect();

    let avail_height = ui.available_height().max(120.0).min(300.0);

    Plot::new("drift_plot")
        .height(avail_height)
        .allow_zoom(true)
        .allow_drag(true)
        .show_axes(true)
        .legend(egui_plot::Legend::default())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(PlotPoints::new(drift_x))
                    .name("Drift X")
                    .color(egui::Color32::from_rgb(100, 149, 237)),
            );
            plot_ui.line(
                Line::new(PlotPoints::new(drift_z))
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
