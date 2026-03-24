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
        ui.label("No segments recorded yet.");
        ui.label("Use REC then CONT to build segments.");
        return actions;
    }

    ui.label(format!("{} segment(s)", segments.len()));
    ui.add_space(4.0);

    // Splice all button at the top
    if segments.len() > 1 {
        if ui
            .button("\u{1F517} Splice All")
            .on_hover_text("Merge all segments into one contiguous recording")
            .clicked()
        {
            actions.push(SegmentAction::SpliceAll);
        }
        ui.add_space(4.0);
    }

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
                            .small_button("\u{21BB} Redo")
                            .on_hover_text(format!(
                                "Continue recording from frame {} (replaces this segment and all after)",
                                seg.start_tick
                            ))
                            .clicked()
                        {
                            actions.push(SegmentAction::RedoFrom(seg.start_tick));
                        }

                        // Delete this segment and everything after
                        if i > 0
                            && ui
                                .small_button("\u{2702} Delete")
                                .on_hover_text("Delete this segment and all subsequent ones")
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
fn input_summary(input_log: &[u8], start: u32, end: u32) -> String {
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
