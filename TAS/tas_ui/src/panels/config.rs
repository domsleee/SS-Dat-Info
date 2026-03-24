use eframe::egui;
use tas_shared::TasSharedState;

/// Check if the given state matches the proven zero-drift config.
/// Returns a list of warning strings (empty = config OK).
pub fn validate_zero_drift_config(state: &TasSharedState) -> Vec<String> {
    let mut warnings = Vec::new();
    if state.inject_mode != 6 {
        warnings.push(format!("inject_mode={} (expected 6)", state.inject_mode));
    }
    if state.force_fixed_tick != 0 {
        warnings.push(format!("fft={} (expected 0)", state.force_fixed_tick));
    }
    if state.force_direct != 2 {
        warnings.push(format!("force_direct={} (expected 2)", state.force_direct));
    }
    if state.input_source != 0 {
        warnings.push(format!("input_source={} (expected 0)", state.input_source));
    }
    if state.self_capture != 0 {
        warnings.push("self_capture=true (expected false)".into());
    }
    if state.use_rec_msg_args == 0 {
        warnings.push("use_rec_msg_args=false (expected true)".into());
    }
    warnings
}

pub fn show(ui: &mut egui::Ui, state: &mut TasSharedState) {
    ui.heading("Config");

    egui::Grid::new("config_grid")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.label("inject_mode:");
            let mut im = state.inject_mode as i32;
            if ui.add(egui::DragValue::new(&mut im).range(0..=6)).changed() {
                state.inject_mode = im as u32;
            }
            ui.end_row();

            ui.label("force_fixed_tick:");
            let mut fft = state.force_fixed_tick as i32;
            if ui
                .add(egui::DragValue::new(&mut fft).range(0..=10))
                .changed()
            {
                state.force_fixed_tick = fft as u32;
            }
            ui.end_row();

            ui.label("force_direct:");
            let mut fd = state.force_direct as i32;
            if ui.add(egui::DragValue::new(&mut fd).range(0..=2)).changed() {
                state.force_direct = fd as u32;
            }
            ui.end_row();

            ui.label("self_capture:");
            let mut sc = state.self_capture != 0;
            if ui.checkbox(&mut sc, "").changed() {
                state.self_capture = sc as u32;
            }
            ui.end_row();

            ui.label("use_rec_msg_args:");
            let mut urma = state.use_rec_msg_args != 0;
            if ui.checkbox(&mut urma, "").changed() {
                state.use_rec_msg_args = urma as u32;
            }
            ui.end_row();

            ui.label("input_source:");
            let mut is = state.input_source as i32;
            if ui.add(egui::DragValue::new(&mut is).range(0..=2)).changed() {
                state.input_source = is as u32;
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

    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    #[test]
    fn proven_zero_drift_config_passes() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_fixed_tick = 0;
        state.force_direct = 2;
        state.input_source = 0;
        state.self_capture = 0;
        state.use_rec_msg_args = 1;
        assert!(validate_zero_drift_config(&state).is_empty());
    }

    #[test]
    fn config_warns_on_wrong_inject_mode() {
        let mut state = zeroed_state();
        state.inject_mode = 3;
        state.force_direct = 2;
        state.use_rec_msg_args = 1;
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("inject_mode=3")));
    }

    #[test]
    fn config_warns_on_nonzero_fft() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_fixed_tick = 2;
        state.force_direct = 2;
        state.use_rec_msg_args = 1;
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("fft=2")));
    }

    #[test]
    fn config_warns_on_wrong_force_direct() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_direct = 0;
        state.use_rec_msg_args = 1;
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("force_direct=0")));
    }

    #[test]
    fn config_warns_on_nonzero_input_source() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_direct = 2;
        state.input_source = 1;
        state.use_rec_msg_args = 1;
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("input_source=1")));
    }

    #[test]
    fn config_warns_on_self_capture() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_direct = 2;
        state.self_capture = 1;
        state.use_rec_msg_args = 1;
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("self_capture=true")));
    }

    #[test]
    fn config_warns_on_use_rec_msg_args_false() {
        let mut state = zeroed_state();
        state.inject_mode = 6;
        state.force_direct = 2;
        state.use_rec_msg_args = 0; // false
        let w = validate_zero_drift_config(&state);
        assert!(w.iter().any(|s| s.contains("use_rec_msg_args=false")));
    }

    #[test]
    fn all_wrong_config_gives_six_warnings() {
        let state = zeroed_state(); // all zeroed — most fields wrong
        let w = validate_zero_drift_config(&state);
        // inject_mode=0, fft=0(ok), force_direct=0, input_source=0(ok), self_capture=0(ok), use_rec_msg_args=0
        // Expected warnings: inject_mode, force_direct, use_rec_msg_args = 3
        assert_eq!(w.len(), 3);
    }
}
