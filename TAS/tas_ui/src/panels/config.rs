use eframe::egui;
use tas_shared::TasSharedState;

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    ui.heading("Hooks");
    egui::Grid::new("hooks_grid")
        .num_columns(2)
        .spacing([8.0, 2.0])
        .show(ui, |ui| {
            hook_row(ui, "the cycle cave", state.cycle_cave_hooked);
            hook_row(ui, "the key-handler cave", state.key_handler_cave_hooked);
            hook_row(ui, "the observer cave", state.observer_cave_hooked);
            hook_row(ui, "the tick cave", state.tick_cave_hooked);
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
