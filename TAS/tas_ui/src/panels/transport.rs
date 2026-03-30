use eframe::egui;
use tas_shared::{TasCommand, TasMode, TasSharedState};

use crate::recording::RecordingHistory;

pub enum Action {
    Send(TasCommand),
    RestartThen(TasCommand),
    Undo,
    Redo,
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
    continue_from_text: &mut String,
    playback_speed: &mut f32,
    cont_catchup_speed: &mut f32,
    _step_mode: &mut bool,
    history: &RecordingHistory,
    _state: &TasSharedState,
    catchup_active: bool,
) -> Vec<Action> {
    let mut actions = Vec::new();

    ui.horizontal(|ui| {
        let is_off = mode == TasMode::Off;
        let is_rec = mode == TasMode::Rec;
        let is_play = mode == TasMode::Play;

        // REC button (red when recording)
        let rec_label = "\u{23FA} REC  F9"; // Unicode record symbol
        let rec_text = egui::RichText::new(rec_label);
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
            actions.push(Action::RestartThen(TasCommand::ArmRec));
        }

        // PLAY button (green when playing)
        let play_label = "\u{25B6} PLAY  F10"; // Unicode play triangle
        let play_text = egui::RichText::new(play_label);
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
        let stop_label = "\u{23F9} STOP  F11"; // Unicode stop symbol
        if ui
            .add_enabled(!is_off, egui::Button::new(stop_label))
            .clicked()
        {
            actions.push(Action::Send(TasCommand::Stop));
        }

        ui.separator();

        // Continue Record
        let cont_label = "\u{23ED} CONT  F12"; // Unicode next track symbol
        if ui
            .add_enabled(is_off && recorded > 0, egui::Button::new(cont_label))
            .on_hover_text(format!(
                "Continue recording from a specific frame (catch-up at {}x)",
                *cont_catchup_speed
            ))
            .clicked()
        {
            normalize_continue_frame_text(continue_from_text, continue_from, recorded);
            actions.push(Action::Log(format!(
                "Continue recording from frame {} ({}x catch-up)",
                *continue_from, *cont_catchup_speed
            )));
            actions.push(Action::SetContinueFrame(*continue_from));
            actions.push(Action::RestartThen(TasCommand::ArmContinue));
        }

        if recorded > 0 {
            ui.label("from:");
            let response = ui.add_sized(
                [72.0, 22.0],
                egui::TextEdit::singleline(continue_from_text).hint_text("frame"),
            );
            if response.changed() {
                if let Some(parsed) = parse_continue_frame(continue_from_text, recorded) {
                    *continue_from = parsed;
                }
            }
            if response.lost_focus() {
                normalize_continue_frame_text(continue_from_text, continue_from, recorded);
            }
        }
        ui.add(
            egui::DragValue::new(cont_catchup_speed)
                .range(1.0..=20.0)
                .prefix("catch-up: ")
                .suffix("x")
                .speed(0.5),
        )
        .on_hover_text("CONT catch-up speed (max effective ~12x at 60 FPS)");

        ui.separator();

        // Undo
        let undo_count = history.undo_depth();
        if ui
            .add_enabled(undo_count > 0, egui::Button::new("\u{21A9} Undo"))
            .on_hover_text(format!("{} earlier restorable state(s)", undo_count))
            .clicked()
        {
            actions.push(Action::Undo);
        }

        // Redo
        let redo_count = history.redo_depth();
        if ui
            .add_enabled(redo_count > 0, egui::Button::new("\u{21AA} Redo"))
            .on_hover_text(format!("{} later restorable state(s)", redo_count))
            .clicked()
        {
            actions.push(Action::Redo);
        }

        ui.separator();

        // Playback speed (disabled during CONT catch-up to avoid state conflicts)
        if catchup_active {
            ui.label(
                egui::RichText::new(format!("Catching up at {}x...", *cont_catchup_speed))
                    .color(egui::Color32::from_rgb(200, 160, 60)),
            );
        } else {
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
        }
    });

    ui.horizontal(|ui| {
        ui.label(format!("Recorded: {} ticks", recorded));
    });

    actions
}

fn parse_continue_frame(text: &str, recorded: u32) -> Option<u32> {
    text.trim()
        .parse::<u32>()
        .ok()
        .map(|v| if recorded > 0 { v.min(recorded) } else { 0 })
}

fn normalize_continue_frame_text(text: &mut String, continue_from: &mut u32, recorded: u32) {
    if let Some(parsed) = parse_continue_frame(text, recorded) {
        *continue_from = parsed;
    }
    if recorded > 0 {
        *continue_from = (*continue_from).min(recorded);
    } else {
        *continue_from = 0;
    }
    *text = continue_from.to_string();
}
