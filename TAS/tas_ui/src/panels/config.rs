use eframe::egui;
use tas_shared::TasSharedState;

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
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
