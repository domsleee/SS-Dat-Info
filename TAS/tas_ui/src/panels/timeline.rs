use eframe::egui;
use tas_shared::{input_bits, TasSharedState, TAS_MAX_TICKS};

const ROW_COLORS: &[(u8, egui::Color32)] = &[
    (input_bits::LEFT, egui::Color32::from_rgb(100, 149, 237)), // cornflower blue
    (input_bits::RIGHT, egui::Color32::from_rgb(255, 165, 0)),  // orange
    (input_bits::UP, egui::Color32::from_rgb(50, 205, 50)),     // lime green
    (input_bits::DOWN, egui::Color32::from_rgb(220, 20, 60)),   // crimson
    (input_bits::JUMP, egui::Color32::from_rgb(186, 85, 211)),  // medium orchid
    (input_bits::SHIFT, egui::Color32::from_rgb(255, 215, 0)),  // gold
];

const ROW_LABELS: &[&str] = &["L", "R", "U", "D", "J", "S"];

pub fn show(ui: &mut egui::Ui, state: &TasSharedState, zoom: &mut f32, scroll: &mut f32) {
    let total = state.recorded_count as usize;
    if total == 0 {
        ui.label("No recording data");
        return;
    }

    // Zoom slider
    ui.horizontal(|ui| {
        ui.label("Zoom:");
        ui.add(egui::Slider::new(zoom, 0.1..=10.0).logarithmic(true));
        if ui.button("1:1").clicked() {
            *zoom = 1.0;
        }
    });

    let avail = ui.available_size();
    let row_height = 12.0;
    let num_rows = ROW_LABELS.len();
    let total_height = row_height * num_rows as f32 + 20.0; // +20 for header
    let left_margin = 8.0;
    let right_padding = 20.0;
    let width = avail.x.min(800.0) - right_padding - left_margin;

    // Visible tick range
    let ticks_visible = (width / *zoom).max(1.0) as usize;
    let max_scroll = total.saturating_sub(ticks_visible);

    // Auto-scroll: keep playback position visible during REC or PLAY
    let playback = state.playback_pos as usize;
    let rec_count = state.recorded_count as usize;
    let active_pos = if state.mode == tas_shared::TasMode::Play as u32 && playback > 0 {
        Some(playback)
    } else if state.mode == tas_shared::TasMode::Rec as u32 && rec_count > 0 {
        Some(rec_count.saturating_sub(1))
    } else {
        None
    };
    if let Some(pos) = active_pos {
        *scroll = auto_scroll_position(pos, *scroll as usize, ticks_visible, max_scroll) as f32;
    }

    // Scroll bar
    ui.horizontal(|ui| {
        let mut s = *scroll;
        ui.add(egui::Slider::new(&mut s, 0.0..=(max_scroll as f32)).show_value(false));
        *scroll = s;
    });

    let scroll_start = (*scroll as usize).min(max_scroll);
    let scroll_end = (scroll_start + ticks_visible).min(total);

    let (response, painter) =
        ui.allocate_painter(egui::vec2(width, total_height), egui::Sense::hover());
    let rect = response.rect;

    // Background
    painter.rect_filled(rect, 2.0, egui::Color32::from_rgb(30, 30, 40));

    let label_width = 20.0;
    let bar_left = rect.left() + label_width;
    let bar_width = width - label_width - 4.0; // 4px right inset

    // Playback position marker
    let playback = state.playback_pos as usize;
    if playback >= scroll_start && playback < scroll_end {
        let px =
            bar_left + ((playback - scroll_start) as f32 / ticks_visible as f32) * bar_width;
        painter.line_segment(
            [egui::pos2(px, rect.top()), egui::pos2(px, rect.bottom())],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 100)),
        );
    }

    // Draw rows
    let y_start = rect.top() + 2.0;
    for (row_idx, &(bit, color)) in ROW_COLORS.iter().enumerate() {
        let y = y_start + row_idx as f32 * row_height;

        // Label
        painter.text(
            egui::pos2(rect.left() + 4.0, y + row_height * 0.5),
            egui::Align2::LEFT_CENTER,
            ROW_LABELS[row_idx],
            egui::FontId::monospace(9.0),
            egui::Color32::from_rgb(180, 180, 180),
        );

        // Draw active regions as filled rectangles
        let mut in_run = false;
        let mut run_start_px = 0.0f32;

        for tick in scroll_start..scroll_end {
            let mask = if tick < TAS_MAX_TICKS {
                state.input_log[tick]
            } else {
                0
            };
            let active = mask & bit != 0;
            let px = bar_left + ((tick - scroll_start) as f32 / ticks_visible as f32) * bar_width;

            if active && !in_run {
                in_run = true;
                run_start_px = px;
            } else if !active && in_run {
                in_run = false;
                let px_w = (px - run_start_px).max(1.0);
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(run_start_px, y + 1.0),
                        egui::vec2(px_w, row_height - 2.0),
                    ),
                    1.0,
                    color,
                );
            }
        }
        // Close trailing run — end at the last visible tick, not the bar edge,
        // so held keys don't stretch annoyingly to the right during live recording.
        if in_run {
            let end_tick = scroll_end.min(total);
            let end_px = bar_left
                + ((end_tick - scroll_start) as f32 / ticks_visible as f32) * bar_width;
            let px_w = (end_px - run_start_px).max(1.0);
            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(run_start_px, y + 1.0),
                    egui::vec2(px_w, row_height - 2.0),
                ),
                1.0,
                color,
            );
        }
    }

    // Tick range label
    ui.label(format!(
        "Showing ticks {}-{} of {} ({:.1}x zoom)",
        scroll_start, scroll_end, total, zoom
    ));
}

/// Compute new scroll position to keep `pos` visible.
/// Returns the scroll offset in ticks.
fn auto_scroll_position(
    pos: usize,
    current_scroll: usize,
    ticks_visible: usize,
    max_scroll: usize,
) -> usize {
    let scroll_end = current_scroll + ticks_visible;
    let margin = ticks_visible * 4 / 5;
    if pos >= scroll_end || pos < current_scroll || pos > current_scroll + margin {
        pos.saturating_sub(margin).min(max_scroll)
    } else {
        current_scroll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_scroll_stays_when_visible() {
        // Position at tick 50, viewport 0..100, margin at 80 — no scroll needed
        assert_eq!(auto_scroll_position(50, 0, 100, 1000), 0);
    }

    #[test]
    fn auto_scroll_jumps_when_past_end() {
        // Position at tick 150, viewport 0..100 — should scroll forward
        let result = auto_scroll_position(150, 0, 100, 1000);
        assert!(result > 0);
        // Position should now be within the new viewport
        assert!(150 >= result && 150 < result + 100);
    }

    #[test]
    fn auto_scroll_jumps_when_before_start() {
        // Position at tick 10, viewport 200..300 — should scroll back
        let result = auto_scroll_position(10, 200, 100, 1000);
        assert!(result <= 10);
    }

    #[test]
    fn auto_scroll_advances_past_margin() {
        // Position at tick 85, viewport 0..100, margin at 80 — should advance
        let result = auto_scroll_position(85, 0, 100, 1000);
        assert!(result > 0);
    }

    #[test]
    fn auto_scroll_clamps_to_max() {
        // Position near end, max_scroll=50
        let result = auto_scroll_position(900, 0, 100, 50);
        assert_eq!(result, 50);
    }
}
