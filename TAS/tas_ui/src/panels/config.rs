use eframe::egui;
use tas_shared::TasSharedState;

/// Check if the given state matches the proven zero-drift config.
/// Returns a list of warning strings (empty = config OK).
pub fn validate_zero_drift_config(state: &TasSharedState) -> Vec<String> {
    let mut warnings = Vec::new();
    if state.force_fixed_tick != 0 {
        warnings.push(format!("fft={} (expected 0)", state.force_fixed_tick));
    }
    warnings
}

pub fn show(ui: &mut egui::Ui, state: &mut TasSharedState) {
    ui.heading("Config");

    egui::Grid::new("config_grid")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.label("force_fixed_tick:");
            let mut fft = state.force_fixed_tick as i32;
            if ui
                .add(egui::DragValue::new(&mut fft).range(0..=10))
                .changed()
            {
                state.force_fixed_tick = fft as u32;
            }
            ui.end_row();
        });

    // Proven zero-drift config validation
    let warnings = validate_zero_drift_config(state);

    if warnings.is_empty() {
        ui.colored_label(
            egui::Color32::from_rgb(80, 200, 80),
            "Config OK (zero-drift proven)",
        );
    } else {
        ui.colored_label(
            egui::Color32::from_rgb(255, 200, 60),
            "Config differs from proven zero-drift:",
        );
        for w in &warnings {
            ui.colored_label(egui::Color32::from_rgb(255, 140, 40), format!("  {}", w));
        }
    }

    ui.separator();
    ui.heading("Hooks");
    egui::Grid::new("hooks_grid")
        .num_columns(2)
        .spacing([8.0, 2.0])
        .show(ui, |ui| {
            hook_row(ui, "Cave 2", state.cave2_hooked);
            hook_row(ui, "Cave 1C", state.cave1c_hooked);
            hook_row(ui, "Cave 1D", state.cave1d_hooked);
            hook_row(ui, "Cave 5", state.cave5_hooked);
            hook_row(ui, "Replay Cap", state.replay_capture_hooked);
        });

    ui.separator();
    ui.heading("Pointers");
    ui.label(format!("Replay: 0x{:08X}", state.replay_ptr));
    ui.label(format!("Player: 0x{:08X}", state.player_ptr));
}

fn hook_row(ui: &mut egui::Ui, name: &str, hooked: u32) {
    let (icon, color) = if hooked == 1 {
        ("OK", egui::Color32::from_rgb(80, 200, 80))
    } else {
        ("--", egui::Color32::from_rgb(255, 80, 80))
    };
    ui.colored_label(color, icon);
    ui.label(name);
    ui.end_row();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_warns_only_on_nonzero_fft() {
        let mut state = tas_shared::zeroed_boxed();
        assert!(validate_zero_drift_config(&state).is_empty());
        state.force_fixed_tick = 2;
        let w = validate_zero_drift_config(&state);
        assert_eq!(w.len(), 1);
        assert!(w[0].contains("fft=2"));
    }
}
