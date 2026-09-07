use eframe::egui;
use tas_shared::cont::detect_first_moving;
use tas_shared::{TasCommand, TasMode, TasSharedState};

use crate::recording::{format_recording_duration, RecordingHistory};

pub enum Action {
    Send(TasCommand),
    RestartThen(TasCommand),
    Undo,
    Redo,
    SetContinueFrame(u32),
    /// Set the CONT resume speed *while catch-up is in flight*. During catch-up
    /// `playback_speed` is the catch-up multiplier, so the speed buttons can't
    /// write it directly — they emit this instead, and main.rs restages the
    /// resume speed (and shared `cont_resume_speed`) the splice will drop to.
    SetResumeSpeed(f32),
    Log(String),
}

pub struct TransportProps<'a> {
    pub mode: TasMode,
    pub recorded: u32,
    pub continue_from: &'a mut u32,
    pub continue_from_text: &'a mut String,
    pub playback_speed: &'a mut f32,
    /// CONT catch-up multiplier, shown in the CONT tooltip and caption.
    pub cont_catchup_speed: f32,
    pub history: &'a RecordingHistory,
    pub state: &'a TasSharedState,
    /// A CONT catch-up replay is in flight.
    pub catchup_active: bool,
    /// Speed playback drops to at the splice (the play speed otherwise).
    pub resume_speed: f32,
    /// False while the game sits in a menu / pause / dialog (engine cycle
    /// frozen, or not in a level at all). Arming from there fires an F5 into
    /// a stopped engine, so REC/PLAY/CONT gray out. STOP stays available -
    /// it is the escape hatch and must work everywhere.
    pub arming_allowed: bool,
}

fn can_arm_continue(mode: TasMode, recorded: u32) -> bool {
    if recorded == 0 {
        return false;
    }
    matches!(mode, TasMode::Off | TasMode::Rec | TasMode::Play)
}

pub fn show(ui: &mut egui::Ui, props: TransportProps<'_>) -> Vec<Action> {
    let TransportProps {
        mode,
        recorded,
        continue_from,
        continue_from_text,
        playback_speed,
        cont_catchup_speed,
        history,
        state,
        catchup_active,
        resume_speed,
        arming_allowed,
    } = props;
    let mut actions = Vec::new();

    ui.horizontal(|ui| {
        let is_off = mode == TasMode::Off;
        let is_rec = mode == TasMode::Rec;
        let is_play = mode == TasMode::Play;
        let can_continue = can_arm_continue(mode, recorded);

        // F-key labels live in tooltips so the buttons stay narrow.
        let rec_text = egui::RichText::new("\u{23FA} REC");
        let rec_text = if is_rec {
            rec_text
                .color(egui::Color32::from_rgb(255, 60, 60))
                .strong()
        } else {
            rec_text
        };
        if ui
            .add_enabled(is_off && arming_allowed, egui::Button::new(rec_text))
            .on_disabled_hover_text("Enter a level first - can't record from a menu")
            .on_hover_text("Record (F9)")
            .clicked()
        {
            actions.push(Action::RestartThen(TasCommand::ArmRec));
        }

        // PLAY button (green when playing, but not during CONT catch-up)
        let play_text = egui::RichText::new("\u{25B6} PLAY");
        let play_text = if is_play && !catchup_active {
            play_text
                .color(egui::Color32::from_rgb(60, 200, 60))
                .strong()
        } else {
            play_text
        };
        if ui
            .add_enabled(
                is_off && recorded > 0 && arming_allowed,
                egui::Button::new(play_text),
            )
            .on_disabled_hover_text("Enter a level first - can't replay from a menu")
            .on_hover_text("Play (F10)")
            .clicked()
        {
            actions.push(Action::RestartThen(TasCommand::ArmPlay));
        }

        if ui
            .add_enabled(!is_off, egui::Button::new("\u{23F9} STOP"))
            .on_hover_text("Stop (F11 / Space)")
            .clicked()
        {
            actions.push(Action::Send(TasCommand::Stop));
        }

        ui.separator();

        // Continue Record (green when CONT catch-up is active)
        let cont_text = egui::RichText::new("\u{23ED} CONT");
        let cont_text = if catchup_active {
            cont_text
                .color(egui::Color32::from_rgb(60, 200, 60))
                .strong()
        } else {
            cont_text
        };
        if ui
            .add_enabled(can_continue && arming_allowed, egui::Button::new(cont_text))
            .on_disabled_hover_text("Enter a level first - can't continue from a menu")
            .on_hover_text(format!(
                "Continue from a specific frame · catch-up ×{}  (F12)",
                cont_catchup_speed
            ))
            .clicked()
        {
            normalize_continue_frame_text(continue_from_text, continue_from, recorded);
            actions.push(Action::Log(format!(
                "Continue recording from frame {} ({}x catch-up)",
                *continue_from, cont_catchup_speed
            )));
            actions.push(Action::SetContinueFrame(*continue_from));
            actions.push(Action::RestartThen(TasCommand::ArmContinue));
        }

        if recorded > 0 {
            ui.label("from:");
            // Tick input with an in-game-time sub-label so the typed frame can
            // be checked against the race clock. The race timer starts at the
            // START-LINE cross, not at first motion; anchor on the line when
            // the track is known, else on first_moving.
            let first_moving = detect_first_moving(&state.rec_coords, recorded);
            let timer_anchor = crate::start_line::start_cross_tick(
                &state.rec_coords,
                recorded,
                crate::level::resolved_level_code(state),
            )
            .or(first_moving);
            ui.vertical(|ui| {
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
                let past_end = *continue_from > recorded;
                let sub_text = if past_end {
                    format!("past end · {}", recorded)
                } else {
                    let offset = timer_anchor.unwrap_or(0);
                    format_recording_duration((*continue_from).saturating_sub(offset))
                };
                let color = if past_end {
                    egui::Color32::from_rgb(217, 123, 92)
                } else {
                    egui::Color32::from_gray(140)
                };
                ui.label(
                    egui::RichText::new(sub_text)
                        .size(9.0)
                        .color(color)
                        .monospace(),
                );
            });
        }

        ui.separator();

        // Undo / Redo — icon-only buttons so the row fits beside the history rail.
        let undo_count = history.undo_depth();
        if ui
            .add_enabled(undo_count > 0, egui::Button::new("\u{21A9}"))
            .on_hover_text(format!("Undo · {} earlier state(s)  (Ctrl+Z)", undo_count))
            .clicked()
        {
            actions.push(Action::Undo);
        }
        let redo_count = history.redo_depth();
        if ui
            .add_enabled(redo_count > 0, egui::Button::new("\u{21AA}"))
            .on_hover_text(format!("Redo · {} later state(s)  (Ctrl+Y)", redo_count))
            .clicked()
        {
            actions.push(Action::Redo);
        }

        ui.separator();

        // Playback speed presets. They stay visible during CONT catch-up and
        // then set the RESUME speed (what playback drops to at the splice), so
        // the highlight follows that; otherwise the live play speed.
        let selected = if catchup_active {
            resume_speed
        } else {
            *playback_speed
        };
        for &spd in &[0.25f32, 0.5, 1.0, 2.0] {
            let label = format!("{}x", spd);
            let btn = egui::Button::new(&label);
            let btn = if (selected - spd).abs() < 0.005 {
                btn.fill(egui::Color32::from_rgb(70, 70, 120))
            } else {
                btn
            };
            if ui.add_enabled(!is_off, btn).clicked() {
                if catchup_active {
                    // playback_speed is the catch-up multiplier here; restage
                    // the resume speed via main.rs instead.
                    actions.push(Action::SetResumeSpeed(spd));
                } else {
                    *playback_speed = spd;
                    actions.push(Action::Log(format!("Playback speed: {}x", spd)));
                }
            }
        }
        if catchup_active {
            ui.label(
                egui::RichText::new(format!("catching up {}x…", cont_catchup_speed))
                    .color(egui::Color32::from_rgb(200, 160, 60)),
            );
        }
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

#[cfg(test)]
mod tests {
    use super::can_arm_continue;
    use tas_shared::TasMode;

    #[test]
    fn continue_requires_recorded_ticks() {
        assert!(!can_arm_continue(TasMode::Off, 0));
        assert!(!can_arm_continue(TasMode::Rec, 0));
        assert!(!can_arm_continue(TasMode::Play, 0));
    }

    #[test]
    fn continue_allowed_in_off_rec_and_play_when_recorded() {
        assert!(can_arm_continue(TasMode::Off, 1));
        assert!(can_arm_continue(TasMode::Rec, 1));
        assert!(can_arm_continue(TasMode::Play, 1));
    }
}
