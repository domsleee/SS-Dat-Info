use eframe::egui;
use tas_shared::{input_bits, TasSharedState};

use crate::recording::SegmentTracker;

/// Actions emitted by the segment panel.
pub enum SegmentAction {
    /// Scroll the timeline to the given tick offset.
    ScrollTo(u32),
    /// Delete segment at index and truncate recording to its start.
    DeleteFrom(usize),
    /// Splice all segments into a single contiguous recording (clear boundaries).
    SpliceAll,
    /// Set continue_from_frame to the start of the given segment (for redo).
    RedoFrom(u32),
}

/// Render the segment list panel. Returns actions for the caller to process.
pub fn show(
    ui: &mut egui::Ui,
    tracker: &SegmentTracker,
    state: &TasSharedState,
) -> Vec<SegmentAction> {
    let mut actions = Vec::new();
    let segments = &tracker.segments;

    if segments.is_empty() {
        ui.label("No segments yet");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Segments let you build a TAS run in pieces.\n\
                 1. Press REC to record the first segment\n\
                 2. Press CONT to continue from where you left off\n\
                 3. Each CONT creates a new segment\n\
                 4. Redo any segment to try a different approach",
            )
            .small()
            .color(egui::Color32::from_rgb(120, 120, 120)),
        );
        return actions;
    }

    ui.horizontal(|ui| {
        ui.label(format!("{} segment(s)", segments.len()));
        ui.add_space(8.0);
        // Splice all button
        if segments.len() > 1
            && ui
                .small_button("Merge All")
                .on_hover_text("Combine all segments into one contiguous recording (removes boundaries)")
                .clicked()
            {
                actions.push(SegmentAction::SpliceAll);
            }
    });
    ui.add_space(4.0);

    egui::ScrollArea::vertical()
        .max_height(300.0)
        .show(ui, |ui| {
            for (i, seg) in segments.iter().enumerate() {
                let frame_count = seg.end_tick.saturating_sub(seg.start_tick);
                let summary = input_summary(&state.input_log, seg.start_tick, seg.end_tick);

                let header = format!(
                    "#{} \u{2502} frames {}\u{2013}{} ({}) \u{2502} {}",
                    i + 1,
                    seg.start_tick,
                    seg.end_tick,
                    frame_count,
                    summary,
                );

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Clickable label to scroll timeline
                        if ui
                            .link(&header)
                            .on_hover_text("Click to scroll timeline to this segment")
                            .clicked()
                        {
                            actions.push(SegmentAction::ScrollTo(seg.start_tick));
                        }
                    });

                    ui.horizontal(|ui| {
                        // Redo from this segment (continue recording from its start)
                        if ui
                            .small_button("Redo")
                            .on_hover_text(format!(
                                "Re-record from frame {}. Replaces this segment and everything after it.",
                                seg.start_tick
                            ))
                            .clicked()
                        {
                            actions.push(SegmentAction::RedoFrom(seg.start_tick));
                        }

                        // Delete this segment and everything after
                        if i > 0
                            && ui
                                .small_button("Delete")
                                .on_hover_text(format!(
                                    "Remove segment #{} and all after. Recording truncates to frame {}.",
                                    i + 1,
                                    seg.start_tick
                                ))
                                .clicked()
                        {
                            actions.push(SegmentAction::DeleteFrom(i));
                        }
                    });
                });

                ui.add_space(2.0);
            }
        });

    // Show DLL-side segment info if available
    if state.segment_count > 0 {
        ui.add_space(4.0);
        ui.separator();
        ui.label(
            egui::RichText::new(format!("DLL: {} segment(s) tracked", state.segment_count))
                .small()
                .color(egui::Color32::from_rgb(140, 140, 140)),
        );
    }

    actions
}

/// Summarize the input keys used in a tick range as a compact string.
pub fn input_summary(input_log: &[u8], start: u32, end: u32) -> String {
    let start = start as usize;
    let end = (end as usize).min(input_log.len());
    if start >= end {
        return "empty".into();
    }

    let mut seen: u8 = 0;
    for &mask in &input_log[start..end] {
        seen |= mask;
    }

    if seen == 0 {
        return "no input".into();
    }

    let mut parts = Vec::new();
    for &(bit, short, _) in input_bits::ALL {
        if seen & bit != 0 {
            parts.push(short);
        }
    }
    parts.join("+")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tas_shared::input_bits;

    #[test]
    fn input_summary_empty_range() {
        let log = [0u8; 100];
        assert_eq!(input_summary(&log, 5, 5), "empty");
        assert_eq!(input_summary(&log, 10, 5), "empty");
    }

    #[test]
    fn input_summary_no_input() {
        let log = [0u8; 10];
        assert_eq!(input_summary(&log, 0, 5), "no input");
    }

    #[test]
    fn input_summary_single_key() {
        let mut log = [0u8; 10];
        log[2] = input_bits::UP;
        log[3] = input_bits::UP;
        assert_eq!(input_summary(&log, 0, 5), "U");
    }

    #[test]
    fn input_summary_multiple_keys() {
        let mut log = [0u8; 10];
        log[0] = input_bits::LEFT;
        log[1] = input_bits::UP | input_bits::JUMP;
        // Seen: LEFT, UP, JUMP
        assert_eq!(input_summary(&log, 0, 3), "L+U+J");
    }

    #[test]
    fn input_summary_all_keys() {
        let mut log = [0u8; 10];
        log[0] = 0x3F; // all 6 bits
        assert_eq!(input_summary(&log, 0, 1), "L+R+U+D+J+S");
    }

    #[test]
    fn input_summary_clamped_to_log_length() {
        let mut log = [0u8; 5];
        log[3] = input_bits::RIGHT;
        // end=100 but log only has 5 entries — should clamp
        assert_eq!(input_summary(&log, 0, 100), "R");
    }

    #[test]
    fn input_summary_partial_range() {
        let mut log = [0u8; 20];
        log[5] = input_bits::LEFT;
        log[10] = input_bits::RIGHT;
        // Only scan 5..8 — should see LEFT only
        assert_eq!(input_summary(&log, 5, 8), "L");
        // Only scan 8..12 — should see RIGHT only
        assert_eq!(input_summary(&log, 8, 12), "R");
    }
}
