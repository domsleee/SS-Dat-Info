//! The status card (transport headline, race clock, run context, live
//! telemetry), the drift banner under it, and the timeline header.

use eframe::egui;
use tas_shared::{TasMode, TasSharedState};

use crate::drift_scan::DriftTracker;
use crate::panels::timeline;

/// Fixed so the card does not resize as the level / rider / clock text
/// changes underneath it.
const STATUS_CARD_WIDTH: f32 = 560.0;

pub struct StatusProps<'a> {
    /// The engine is ticking a level (not a menu, pause or dialog).
    pub in_game: bool,
    /// The last recording ended by crossing the finish line at this tick.
    pub finished_at_tick: Option<u32>,
    /// Physics / rider stamps of the take in the recording buffer.
    pub loaded_physics: Option<&'a str>,
    pub loaded_rider: Option<&'a str>,
}

/// Display name for the DLL's `level_id` (area*3 + difficulty, 9 = Practice).
fn level_name_from_id(id: u32) -> Option<&'static str> {
    const NAMES: [&str; 10] = [
        "Forest Easy",
        "Forest Medium",
        "Forest Hard",
        "Alpine Easy",
        "Alpine Medium",
        "Alpine Hard",
        "Village Easy",
        "Village Medium",
        "Village Hard",
        "Practice",
    ];
    NAMES.get(id as usize).copied()
}

const WARN: egui::Color32 = egui::Color32::from_rgb(255, 140, 60);
const DIM: egui::Color32 = egui::Color32::from_gray(150);

/// Live value plus a warning glyph when the loaded take was stamped
/// differently.
fn stamp_row(ui: &mut egui::Ui, live: &str, loaded: Option<&str>, hover: &str, kind: &str) {
    ui.label(egui::RichText::new(live).color(DIM).size(12.0))
        .on_hover_text(hover);
    if loaded.is_some_and(|stamp| stamp != live) {
        ui.label(egui::RichText::new("\u{26A0}").color(WARN).size(12.0))
            .on_hover_text(format!(
                "{} mismatch\nLive: {}\nTake: {}",
                kind,
                live,
                loaded.unwrap_or("?")
            ));
    }
}

pub fn status_card(ui: &mut egui::Ui, state: &TasSharedState, props: &StatusProps) {
    let mode_color = match state.mode_enum() {
        TasMode::Off => egui::Color32::GRAY,
        TasMode::Rec => egui::Color32::from_rgb(255, 80, 80),
        TasMode::Play => egui::Color32::from_rgb(80, 200, 80),
    };
    let play_pos = state.playback_pos;
    let rec_count = state.recorded_count;
    let headline = match state.mode_enum() {
        TasMode::Play if play_pos > 0 && rec_count > 0 => {
            let pct = (play_pos as f64 / rec_count as f64 * 100.0).min(100.0);
            format!(
                "{} {} / {} ticks ({:.0}%)",
                state.mode_str(),
                play_pos,
                rec_count,
                pct
            )
        }
        TasMode::Rec if rec_count > 0 => format!("{} {} ticks", state.mode_str(), rec_count),
        _ => state.mode_str().to_string(),
    };
    let vx = state.velocity_x as f64;
    let vy = state.velocity_y as f64;
    let vz = state.velocity_z as f64;
    let speed_kmh = (vx * vx + vy * vy + vz * vz).sqrt() * 360.0;

    egui::Frame::group(ui.style()).show(ui, |ui| {
        let card_w = STATUS_CARD_WIDTH.min(ui.available_width());
        ui.set_min_width(card_w);
        ui.set_max_width(card_w);

        let card_level = tas_shared::resolved_level_id(state).and_then(level_name_from_id);
        let live_physics =
            tas_shared::physics_mode_label(state.renderer_id, state.fpu_control_word);
        let live_rider = tas_shared::rider_label(state.rider_character, state.rider_stance);
        let (race_cs, race_start) = tas_shared::race_pair(state);

        // Transport on the left, race clock on the right.
        ui.horizontal(|ui| {
            ui.colored_label(
                mode_color,
                egui::RichText::new(headline).strong().size(16.0),
            );
            if let Some(t) = props.finished_at_tick {
                ui.label(egui::RichText::new("\u{1F3C1}").size(16.0))
                    .on_hover_text(format!(
                        "Recording crossed the finish line at tick {} and was auto-stopped",
                        t
                    ));
            }
            if race_cs != u32::MAX {
                let t = format!(
                    "\u{23F1} {:01}:{:02}.{:02}",
                    race_cs / 6000,
                    (race_cs % 6000) / 100,
                    race_cs % 100
                );
                let clock_color = if props.finished_at_tick.is_some() {
                    egui::Color32::from_rgb(235, 205, 90)
                } else {
                    egui::Color32::from_gray(170)
                };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(t).color(clock_color).size(12.0))
                        .on_hover_text(format!(
                            "Exact race time (from the HUD). start_ts={} — the gate \
                             clock value",
                            race_start
                        ));
                });
            }
        });

        // Stable run context: map, physics mode, rider/stance.
        ui.horizontal_wrapped(|ui| {
            if props.in_game {
                let level = card_level.unwrap_or("Level resolving…");
                ui.label(
                    egui::RichText::new(format!("\u{1F3AE} {}", level))
                        .color(egui::Color32::from_rgb(90, 200, 120))
                        .size(12.0),
                )
                .on_hover_text("Current level");
            }
            if let Some(live) = live_physics.as_deref() {
                stamp_row(
                    ui,
                    live,
                    props.loaded_physics,
                    "Renderer / x87 precision the game thread runs the physics at \
                     (DirectX 6/7 = 24-bit, OpenGL = 53-bit). A take recorded under \
                     the other mode rounds differently and will not replay bit-exact.",
                    "Physics",
                );
            }
            if let Some(live) = live_rider.as_deref() {
                stamp_row(
                    ui,
                    live,
                    props.loaded_rider,
                    "Character and stance the human rider is using. The physics differ \
                     per character and per stance, so a take recorded as another rider \
                     will not line up.",
                    "Rider",
                );
            }
        });

        ui.separator();
        ui.label(format!(
            "Pos: ({:.1}, {:.1}, {:.1})    Speed: {:.1} km/h",
            state.player_x, state.player_y, state.player_z, speed_kmh
        ));
        ui.label(format!(
            "Vel: ({:.2}, {:.2}, {:.2})",
            state.velocity_x, state.velocity_y, state.velocity_z
        ));
    });
}

/// The F12/CONT verdict banner: the splice agreement for a CONT, any measured
/// divergence for a plain PLAY. Nothing is drawn while the tracker has no
/// verdict.
pub fn drift_banner(ui: &mut egui::Ui, state: &TasSharedState, tracker: &DriftTracker) {
    if !tracker.banner_visible(state) {
        return;
    }
    let (dx, dz, splice) = tracker.verdict(state);
    let title = if splice != 0 {
        "CONT SPLICE MISMATCH"
    } else if dx.max(dz) >= 1.0 {
        "DRIFT DETECTED: TAS INVALID"
    } else {
        "DRIFT WARNING"
    };
    let (bg, text, msg) = if dx.max(dz) >= 1.0 {
        (
            egui::Color32::from_rgb(180, 30, 30),
            egui::Color32::WHITE,
            format!("{}  (X={:.6}  Z={:.6})", title, dx, dz),
        )
    } else {
        (
            egui::Color32::from_rgb(180, 140, 20),
            egui::Color32::BLACK,
            format!("{}  (X={:.9}  Z={:.9})", title, dx, dz),
        )
    };
    egui::Frame::none()
        .fill(bg)
        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
        .rounding(4.0)
        .show(ui, |ui: &mut egui::Ui| {
            ui.label(egui::RichText::new(msg).color(text).strong().size(16.0));
        });
    ui.separator();
}

/// "Input Timeline" with the human time at a glance and the text-script route
/// as a primary action. Returns `true` when the Text Script button was clicked.
pub fn timeline_header(ui: &mut egui::Ui, state: &TasSharedState, watching_script: bool) -> bool {
    let total_time = timeline::format_time(state.recorded_count);
    let current_time = match state.mode_enum() {
        TasMode::Play => Some(timeline::format_time(
            state.playback_pos.min(state.recorded_count),
        )),
        TasMode::Rec => Some(total_time.clone()),
        TasMode::Off => None,
    };
    let time_summary = current_time
        .map(|current| format!("{} / {}", current, total_time))
        .unwrap_or_else(|| format!("{} total", total_time));
    let mut open_text_script = false;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Input Timeline").strong());
        ui.label(
            egui::RichText::new(time_summary)
                .monospace()
                .size(12.0)
                .color(egui::Color32::from_gray(165)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let button = egui::Button::new(
                egui::RichText::new("↗ Text Script")
                    .strong()
                    .color(egui::Color32::from_gray(220)),
            )
            .fill(ui.visuals().widgets.inactive.weak_bg_fill)
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgb(62, 86, 110),
            ));
            open_text_script = ui
                .add_enabled(state.recorded_count > 0, button)
                .on_hover_text("Write a .tas file and open it; edits reload on save")
                .clicked();
            if watching_script {
                ui.weak("watching .tas");
            }
        });
    });
    open_text_script
}
