use eframe::egui;
use tas_shared::{input_bits, TasMode, TasSharedState, TAS_MAX_TICKS};

const ROW_COLORS: &[(u8, egui::Color32)] = &[
    (input_bits::LEFT, egui::Color32::from_rgb(100, 149, 237)), // cornflower blue
    (input_bits::RIGHT, egui::Color32::from_rgb(255, 165, 0)),  // orange
    (input_bits::UP, egui::Color32::from_rgb(50, 205, 50)),     // lime green
    (input_bits::DOWN, egui::Color32::from_rgb(220, 20, 60)),   // crimson
    (input_bits::JUMP, egui::Color32::from_rgb(186, 85, 211)),  // medium orchid
    (input_bits::SHIFT, egui::Color32::from_rgb(255, 215, 0)),  // gold
];

const ROW_LABELS: &[&str] = &["L", "R", "U", "D", "J", "S"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTickMode {
    Rec,
    Play,
}

pub fn show(
    ui: &mut egui::Ui,
    state: &TasSharedState,
    zoom: &mut f32,
    scroll: &mut f32,
    continue_from: &mut u32,
) -> bool {
    let total = state.recorded_count as usize;
    let has_data = total > 0;

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
    let header_height = 14.0;
    let axis_height = 16.0;
    let total_height = header_height + row_height * num_rows as f32 + axis_height + 6.0;
    let left_margin = 8.0;
    let right_padding = 20.0;
    let width = (avail.x.min(800.0) - right_padding - left_margin).max(120.0);

    // Visible tick range
    let ticks_visible = (width / *zoom).max(1.0) as usize;
    let max_scroll = total.saturating_sub(ticks_visible);

    // Auto-scroll: keep playback position visible during REC or PLAY
    let active_tick = active_timeline_tick(state);
    if let Some((pos, _)) = active_tick {
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

    let (response, painter) = ui.allocate_painter(
        egui::vec2(width, total_height),
        egui::Sense::click_and_drag(),
    );
    let rect = response.rect;

    // Background
    painter.rect_filled(rect, 2.0, egui::Color32::from_rgb(30, 30, 40));

    let label_width = 20.0;
    let bar_left = rect.left() + label_width;
    let bar_width = width - label_width - 4.0; // 4px right inset

    let rows_top = rect.top() + header_height;
    let rows_bottom = rows_top + row_height * num_rows as f32;
    let axis_y = rows_bottom + 2.0;

    let marker_status = match active_tick {
        Some((tick, ActiveTickMode::Rec)) => format!("REC @ frame {}", tick),
        Some((tick, ActiveTickMode::Play)) => format!("PLAY @ frame {}", tick),
        None => "OFF".to_string(),
    };
    let marker_status_color = match active_tick.map(|(_, mode)| mode) {
        Some(ActiveTickMode::Rec) => egui::Color32::from_rgb(120, 255, 120),
        Some(ActiveTickMode::Play) => egui::Color32::from_rgb(255, 235, 120),
        None => egui::Color32::from_rgb(150, 150, 150),
    };
    painter.text(
        egui::pos2(bar_left, rect.top() + 1.0),
        egui::Align2::LEFT_TOP,
        marker_status,
        egui::FontId::monospace(9.0),
        marker_status_color,
    );
    painter.text(
        egui::pos2(rect.right() - 4.0, rect.top() + 1.0),
        egui::Align2::RIGHT_TOP,
        format!("{}..{}", scroll_start, scroll_end.saturating_sub(1)),
        egui::FontId::monospace(9.0),
        egui::Color32::from_rgb(150, 150, 150),
    );

    // Active position marker
    if let Some((tick, mode)) = active_tick {
        if tick >= scroll_start && tick < scroll_end {
            let px = bar_left + ((tick - scroll_start) as f32 / ticks_visible as f32) * bar_width;
            let marker_color = match mode {
                ActiveTickMode::Rec => egui::Color32::from_rgb(120, 255, 120),
                ActiveTickMode::Play => egui::Color32::from_rgb(255, 235, 120),
            };

            let highlight_rect = egui::Rect::from_min_max(
                egui::pos2((px - 2.0).max(bar_left), rows_top),
                egui::pos2((px + 2.0).min(bar_left + bar_width), rows_bottom),
            );
            painter.rect_filled(highlight_rect, 1.0, marker_color.gamma_multiply(0.35));
            painter.line_segment(
                [egui::pos2(px, rows_top), egui::pos2(px, rows_bottom)],
                egui::Stroke::new(2.0, marker_color),
            );
        }
    }

    // Draw rows
    let y_start = rows_top + 2.0;
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
            let end_px =
                bar_left + ((end_tick - scroll_start) as f32 / ticks_visible as f32) * bar_width;
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

    // X-axis frame labels (start / mid / end)
    painter.line_segment(
        [
            egui::pos2(bar_left, axis_y),
            egui::pos2(bar_left + bar_width, axis_y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)),
    );

    let end_tick = scroll_end.saturating_sub(1);
    let mid_tick = scroll_start + (end_tick.saturating_sub(scroll_start) / 2);
    let mut axis_ticks = vec![scroll_start, mid_tick, end_tick];
    axis_ticks.dedup();

    for tick in axis_ticks {
        let px = if tick >= scroll_start {
            bar_left + ((tick - scroll_start) as f32 / ticks_visible as f32) * bar_width
        } else {
            bar_left
        };
        painter.line_segment(
            [egui::pos2(px, axis_y), egui::pos2(px, axis_y + 4.0)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 90, 105)),
        );
        painter.text(
            egui::pos2(px, axis_y + 5.0),
            egui::Align2::CENTER_TOP,
            tick.to_string(),
            egui::FontId::monospace(9.0),
            egui::Color32::from_rgb(165, 165, 180),
        );
    }

    // Tick range label
    ui.label(format!(
        "Showing ticks {}-{} of {} ({:.1}x zoom)",
        scroll_start, scroll_end, total, zoom
    ));
    if !has_data {
        ui.colored_label(
            egui::Color32::from_rgb(150, 150, 150),
            "No recording data yet. Press REC to populate timeline rows.",
        );
    }

    let mut continue_marker_changed = false;
    if response.clicked() || response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            if pos.x >= bar_left
                && pos.x <= bar_left + bar_width
                && pos.y >= rows_top
                && pos.y <= rows_bottom
            {
                let rel = ((pos.x - bar_left) / bar_width).clamp(0.0, 1.0);
                let tick = scroll_start + (rel * ticks_visible as f32) as usize;
                let frame = tick.min(total.saturating_sub(1)) as u32;
                if frame != *continue_from {
                    *continue_from = frame;
                    continue_marker_changed = true;
                }
            }
        }
    }

    let continue_tick = (*continue_from as usize).min(total.saturating_sub(1));
    if continue_tick >= scroll_start && continue_tick < scroll_end {
        let px =
            bar_left + ((continue_tick - scroll_start) as f32 / ticks_visible as f32) * bar_width;
        let continue_color = egui::Color32::from_rgb(120, 200, 255);
        painter.line_segment(
            [egui::pos2(px, rows_top), egui::pos2(px, rows_bottom)],
            egui::Stroke::new(1.5, continue_color),
        );
        painter.text(
            egui::pos2(px + 3.0, rows_top + 1.0),
            egui::Align2::LEFT_TOP,
            "CONT",
            egui::FontId::monospace(8.0),
            continue_color,
        );
    }

    continue_marker_changed
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

fn active_timeline_tick(state: &TasSharedState) -> Option<(usize, ActiveTickMode)> {
    let total = state.recorded_count as usize;
    if total == 0 {
        return None;
    }

    if state.mode == TasMode::Play as u32 {
        let playback = state.playback_pos as usize;
        return Some((playback.min(total.saturating_sub(1)), ActiveTickMode::Play));
    }

    if state.mode == TasMode::Rec as u32 {
        return Some((total.saturating_sub(1), ActiveTickMode::Rec));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tas_shared::{zeroed_boxed, TasMode};

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

    #[test]
    fn active_tick_rec_uses_last_recorded_frame() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Rec as u32;
        state.recorded_count = 42;
        state.playback_pos = 0;
        assert_eq!(
            active_timeline_tick(&state),
            Some((41, ActiveTickMode::Rec))
        );
    }

    #[test]
    fn active_tick_play_uses_playback_frame() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.recorded_count = 100;
        state.playback_pos = 37;
        assert_eq!(
            active_timeline_tick(&state),
            Some((37, ActiveTickMode::Play))
        );
    }

    #[test]
    fn active_tick_play_clamps_to_last_recorded_frame() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.recorded_count = 10;
        state.playback_pos = 9999;
        assert_eq!(
            active_timeline_tick(&state),
            Some((9, ActiveTickMode::Play))
        );
    }

    #[test]
    fn active_tick_off_has_no_marker() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Off as u32;
        state.recorded_count = 20;
        state.playback_pos = 7;
        assert_eq!(active_timeline_tick(&state), None);
    }

    #[test]
    fn active_tick_none_with_empty_recording() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Rec as u32;
        state.recorded_count = 0;
        state.playback_pos = 0;
        assert_eq!(active_timeline_tick(&state), None);
    }
}
