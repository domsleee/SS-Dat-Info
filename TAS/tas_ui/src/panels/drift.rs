use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use tas_shared::TasSharedState;

/// Gate-aligned plot samples from the current playback generation.
#[derive(Default)]
pub struct DriftCache {
    drift_x: Vec<[f64; 2]>,
    drift_z: Vec<[f64; 2]>,
    bases: Option<(usize, usize)>,
    generation: u32,
}

impl DriftCache {
    /// Update the cache if the underlying data has changed. Returns true if refreshed.
    pub fn refresh(&mut self, state: &TasSharedState) -> bool {
        let generation_changed = self.generation != state.arm_generation;
        let (count, _) = crate::drift_window(state, &mut self.bases, &mut self.generation);
        if generation_changed {
            self.drift_x.clear();
            self.drift_z.clear();
        }
        if state.mode != tas_shared::TasMode::Play as u32 {
            return generation_changed;
        }
        let (play_base, rec_base) = self.bases.unwrap_or((0, 0));

        let step = (count / 1000).max(1);
        self.drift_x.clear();
        self.drift_z.clear();
        for i in (0..count).step_by(step) {
            let dx =
                (state.play_coords[play_base + i][0] - state.rec_coords[rec_base + i][0]) as f64;
            self.drift_x.push([(rec_base + i) as f64, dx]);
            let dz =
                (state.play_coords[play_base + i][2] - state.rec_coords[rec_base + i][2]) as f64;
            self.drift_z.push([(rec_base + i) as f64, dz]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_uses_gate_relative_pairs_and_refreshes_equal_length_replacements() {
        let mut state = tas_shared::zeroed_boxed();
        state.mode = tas_shared::TasMode::Play as u32;
        state.arm_generation = 1;
        state.recorded_count = 12;
        state.playback_pos = 10;
        state.gate_align_rec = 4;
        state.gate_index = 2;
        for i in 0..8 {
            state.rec_coords[4 + i] = [i as f32, 0.0, i as f32];
            state.play_coords[2 + i] = state.rec_coords[4 + i];
        }
        let mut cache = DriftCache::default();
        cache.refresh(&state);
        assert_eq!(cache.drift_x.len(), 8);
        assert!(cache
            .drift_x
            .iter()
            .chain(&cache.drift_z)
            .all(|p| p[1] == 0.0));
        state.rec_coords[4][0] += 1.0;
        cache.refresh(&state);
        assert_eq!(cache.drift_x[0], [4.0, -1.0]);
        state.arm_generation += 1;
        state.gate_index = 0;
        cache.refresh(&state);
        assert!(cache.drift_x.is_empty());
    }

    #[test]
    fn graph_freezes_outside_play_and_drops_previous_generation() {
        let mut state = tas_shared::zeroed_boxed();
        state.mode = tas_shared::TasMode::Play as u32;
        state.recorded_count = 1;
        state.playback_pos = 1;
        state.play_coords[0][0] = 1.0;
        let mut cache = DriftCache::default();
        cache.refresh(&state);
        state.mode = tas_shared::TasMode::Off as u32;
        state.rec_coords[0][0] = 99.0;
        cache.refresh(&state);
        assert_eq!(cache.drift_x[0][1], 1.0);
        state.arm_generation += 1;
        cache.refresh(&state);
        assert!(cache.drift_x.is_empty());
    }
}
