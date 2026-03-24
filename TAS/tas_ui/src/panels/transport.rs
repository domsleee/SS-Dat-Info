use eframe::egui;
use tas_shared::{TasCommand, TasMode, TasSharedState};

use crate::recording::UndoRing;

pub enum Action {
    Send(TasCommand),
    RestartThen(TasCommand),
    AutoSave,
    Undo,
    StepOne,
    SetContinueFrame(u32),
    Log(String),
}

#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut egui::Ui,
    mode: TasMode,
    recorded: u32,
    continue_from: &mut u32,
    playback_speed: &mut f32,
    step_mode: &mut bool,
    undo_ring: &UndoRing,
    _state: &TasSharedState,
) -> Vec<Action> {
    let mut actions = Vec::new();

    ui.horizontal(|ui| {
        let is_off = mode == TasMode::Off;
        let is_rec = mode == TasMode::Rec;
        let is_play = mode == TasMode::Play;

        // REC button (red when recording)
        let rec_text = egui::RichText::new("[R] REC [F9]");
        let rec_text = if is_rec {
            rec_text
                .color(egui::Color32::from_rgb(255, 60, 60))
                .strong()
        } else {
            rec_text
        };
        if ui
            .add_enabled(is_off, egui::Button::new(rec_text))
            .clicked()
        {
            if recorded > 0 {
                actions.push(Action::AutoSave);
            }
            actions.push(Action::RestartThen(TasCommand::ArmRec));
        }

        // PLAY button (green when playing)
        let play_text = egui::RichText::new("[>] PLAY [F10]");
        let play_text = if is_play {
            play_text
                .color(egui::Color32::from_rgb(60, 200, 60))
                .strong()
        } else {
            play_text
        };
        if ui
            .add_enabled(is_off && recorded > 0, egui::Button::new(play_text))
            .clicked()
        {
            actions.push(Action::RestartThen(TasCommand::ArmPlay));
        }

        // STOP button
        if ui
            .add_enabled(!is_off, egui::Button::new("[S] STOP [F11]"))
            .clicked()
        {
            actions.push(Action::Send(TasCommand::Stop));
        }

        ui.separator();

        // Continue Record
        if ui
            .add_enabled(is_off && recorded > 0, egui::Button::new("[+] CONT [F12]"))
            .on_hover_text("Continue recording from a specific frame")
            .clicked()
        {
            actions.push(Action::AutoSave);
            actions.push(Action::Log(format!(
                "Continue recording from frame {}",
                *continue_from
            )));
            actions.push(Action::SetContinueFrame(*continue_from));
            actions.push(Action::RestartThen(TasCommand::ArmContinue));
        }

        if recorded > 0 {
            ui.add(
                egui::DragValue::new(continue_from)
                    .range(0..=recorded)
                    .prefix("from: ")
                    .speed(1.0),
            );
        }

        ui.separator();

        // Undo
        let undo_count = undo_ring.len();
        if ui
            .add_enabled(undo_count > 0, egui::Button::new("<< Undo"))
            .on_hover_text(format!("{} saves in ring", undo_count))
            .clicked()
        {
            actions.push(Action::Undo);
        }

        ui.separator();

        // Playback speed (only enabled when actively playing or recording)
        ui.label("Speed:");
        for &spd in &[0.25f32, 0.5, 1.0, 2.0, 4.0] {
            let label = format!("{}x", spd);
            let btn = egui::Button::new(&label);
            let btn = if (*playback_speed - spd).abs() < 0.01 {
                btn.fill(egui::Color32::from_rgb(70, 70, 120))
            } else {
                btn
            };
            if ui.add_enabled(!is_off, btn).clicked() {
                *playback_speed = spd;
                actions.push(Action::Log(format!("Playback speed: {}x", spd)));
            }
        }

        ui.separator();

        // Step mode
        ui.checkbox(step_mode, "Step");
        if *step_mode && ui.button("|>").on_hover_text("Advance one frame").clicked() {
            actions.push(Action::StepOne);
        }
    });

    ui.horizontal(|ui| {
        ui.label(format!("Recorded: {} ticks", recorded));
    });

    actions
}
