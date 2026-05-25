use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;

use crate::recording::{
    format_recording_duration, HistoryEntry, HistoryEntryKind, RecordingHistory,
};

pub enum HistoryAction {
    Restore(usize),
}

pub fn show(ui: &mut egui::Ui, history: &RecordingHistory) -> Vec<HistoryAction> {
    let mut actions = Vec::new();

    if history.is_empty() {
        ui.colored_label(
            egui::Color32::from_rgb(140, 140, 140),
            "No history yet. Record, continue, save, or load to create entries.",
        );
        return actions;
    }

    let current = history.current_index();
    let today = Local::now().date_naive();
    let yesterday = today.pred_opt();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Render order: sort by `created_at` descending with vec index
            // as a tie-breaker, so the displayed list is true reverse-
            // chronological even when entries were healed (e.g. mis-
            // migrated future timestamps walked back a day) or pushed in
            // non-chronological order (e.g. recovered from a prior
            // session). The model itself stays in push/action order —
            // we only sort the view, so `current_index` semantics and
            // undo/redo remain unchanged. The vec index travels with
            // each row so Restore(idx) still targets the right entry.
            let entries = history.entries();
            let mut visible: Vec<(usize, &HistoryEntry)> =
                entries.iter().enumerate().collect();
            visible.sort_by(|(a_idx, a), (b_idx, b)| {
                // (created_at desc, idx desc) — same-timestamp entries
                // preserve push recency at the top.
                b.created_at
                    .cmp(&a.created_at)
                    .then_with(|| b_idx.cmp(a_idx))
            });

            let mut last_date: Option<NaiveDate> = None;
            for (idx, entry) in visible {
                let entry_date = entry.created_at.date_naive();
                if last_date != Some(entry_date) {
                    render_day_header(ui, entry_date, today, yesterday);
                    last_date = Some(entry_date);
                }
                let is_current = current == Some(idx);
                render_row(ui, entry, idx, is_current, &mut actions);
            }
        });

    actions
}

fn render_day_header(
    ui: &mut egui::Ui,
    date: NaiveDate,
    today: NaiveDate,
    yesterday: Option<NaiveDate>,
) {
    // Avoid chrono's `%-d` (POSIX no-pad day) which is unsupported on
    // Windows' strftime — would render the literal `-d` instead of the
    // day number. Build the day-month string by hand.
    let short = format!("{} {}", date.day(), month_abbr(date.month()));
    let label = if date == today {
        format!("Today · {}", short)
    } else if Some(date) == yesterday {
        format!("Yesterday · {}", short)
    } else {
        format!(
            "{} {} {} {}",
            weekday_abbr(date.weekday().num_days_from_monday()),
            date.day(),
            month_abbr(date.month()),
            date.year()
        )
    };
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(label)
            .size(10.0)
            .color(egui::Color32::from_gray(120)),
    );
    ui.add_space(2.0);
}

fn month_abbr(m: u32) -> &'static str {
    match m {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr", 5 => "May", 6 => "Jun",
        7 => "Jul", 8 => "Aug", 9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    }
}

fn weekday_abbr(d: u32) -> &'static str {
    match d {
        0 => "Mon", 1 => "Tue", 2 => "Wed", 3 => "Thu",
        4 => "Fri", 5 => "Sat", 6 => "Sun",
        _ => "???",
    }
}

fn render_row(
    ui: &mut egui::Ui,
    entry: &HistoryEntry,
    idx: usize,
    is_current: bool,
    actions: &mut Vec<HistoryAction>,
) {
    let parts = parse_entry(entry);
    let restorable = entry.can_restore();
    let time_str = entry.created_at.format("%H:%M").to_string();

    // Color for markers (save/load) — italic + dimmed/blue. Snapshot rows
    // get default text color so they're scannable. The whole row becomes
    // a single click target.
    let row_color = if parts.is_marker {
        match entry.kind {
            HistoryEntryKind::SaveMarker => egui::Color32::from_rgb(120, 170, 220),
            _ => egui::Color32::from_gray(150),
        }
    } else {
        ui.visuals().text_color()
    };

    // Push the current-row background ourselves via a Frame, so the row
    // contents below can use horizontal layout without losing the
    // selection visual that selectable_label would give us.
    let mut frame = egui::Frame::none().inner_margin(egui::Margin::symmetric(4.0, 1.0));
    if is_current {
        frame = frame.fill(egui::Color32::from_rgba_unmultiplied(192, 132, 252, 38));
    }

    let outer = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Time goes to the right edge via right_to_left layout, then
            // the rest flows left-to-right back from there.
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.label(
                        egui::RichText::new(&time_str)
                            .size(11.0)
                            .color(egui::Color32::from_gray(140))
                            .monospace(),
                    );
                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // Single icon (▶) for every row, colored by
                            // kind. Using one Unicode glyph for everything
                            // sidesteps egui's per-codepoint font fallback
                            // (which made ● / 💾 / 📂 render at wildly
                            // inconsistent sizes next to ▶). Color does
                            // the discrimination: orange = REC/CONT, blue
                            // = save, amber = load.
                            ui.label(
                                egui::RichText::new("▶")
                                    .color(kind_color(entry.kind))
                                    .size(13.0),
                            );
                            // Total — right-padded so 0:55 and 1:08 line
                            // up vertically across rows.
                            if !parts.total.is_empty() {
                                let mut rt = egui::RichText::new(format!("{:>7}", parts.total))
                                    .monospace()
                                    .size(12.0)
                                    .color(row_color);
                                if parts.is_marker {
                                    rt = rt.italics();
                                }
                                ui.label(rt);
                            }
                            // Context (e.g. "from 0:52.00"). Truncates with
                            // ellipsis if the panel is too narrow.
                            let mut ctx_rt = egui::RichText::new(&parts.context)
                                .size(12.0)
                                .color(row_color);
                            if parts.is_marker {
                                ctx_rt = ctx_rt.italics();
                            }
                            ui.add(egui::Label::new(ctx_rt).truncate());
                        },
                    );
                },
            );
        });
    });

    // Capture row-level click. The Frame's response is what we want; turn
    // it into a click sensor so any pixel of the row works.
    if restorable {
        let interact = outer
            .response
            .interact(egui::Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        if interact.clicked() {
            actions.push(HistoryAction::Restore(idx));
        }
    }
}

fn kind_color(kind: HistoryEntryKind) -> egui::Color32 {
    match kind {
        HistoryEntryKind::Snapshot => egui::Color32::from_rgb(232, 179, 74),
        HistoryEntryKind::SaveMarker => egui::Color32::from_rgb(120, 170, 220),
        HistoryEntryKind::LoadSnapshot => egui::Color32::from_rgb(192, 160, 96),
    }
}

struct Parts {
    /// Duration like `1:08.24`. Empty for markers.
    total: String,
    /// Right-justified context: `from 0:52.00`, `Saved → file`,
    /// `Loaded ← file`. Empty for plain REC entries (the icon-color
    /// + the duration already convey "recording").
    context: String,
    is_marker: bool,
}

/// Build display parts for a history entry. Snapshot rows prefer the
/// structured `start_tick` / `end_tick` / `first_moving` fields when
/// present, so we render `from <tick> · <in-game time>` directly; legacy
/// entries (persisted before those fields existed) fall back to parsing
/// the label string with the original `parse_snapshot_label`.
///
/// In-game time is `tick - first_moving` converted to clock format via
/// `format_recording_duration`. When `first_moving` is `None` we fall
/// back to raw `tick / 100`, which is recording-elapsed (not in-game)
/// time — flagged the same way in the panel layout.
fn parse_entry(entry: &HistoryEntry) -> Parts {
    match entry.kind {
        HistoryEntryKind::SaveMarker => Parts {
            total: String::new(),
            // The previous `Saved → file` form looked nice in monospace
            // mockups but the `→` glyph (U+2192) rendered as a hollow
            // square in egui's default font fallback. ASCII keeps it
            // tight; the verb + icon color already convey "save".
            context: entry
                .label
                .strip_prefix("Save: ")
                .map(|name| format!("Saved {}", name))
                .unwrap_or_else(|| entry.label.clone()),
            is_marker: true,
        },
        HistoryEntryKind::LoadSnapshot => Parts {
            total: String::new(),
            context: entry
                .label
                .strip_prefix("Load: ")
                .map(|name| format!("Loaded {}", name))
                .unwrap_or_else(|| entry.label.clone()),
            is_marker: true,
        },
        HistoryEntryKind::Snapshot => parse_snapshot(entry),
    }
}

fn in_game_duration(tick: u32, first_moving: Option<u32>) -> String {
    let offset = first_moving.unwrap_or(0);
    format_recording_duration(tick.saturating_sub(offset))
}

/// Build snapshot Parts. New entries with `start_tick > 0` get the
/// "from <tick> · <in-game time>" form. New REC entries (start_tick = 0
/// with end_tick > 0) get the empty context. Legacy entries — persisted
/// with `start_tick = 0` AND a `Continued from …` label — fall back to
/// the label parser to preserve the original "from H:MM.ss" context
/// rather than collapsing into REC-style display.
fn parse_snapshot(entry: &HistoryEntry) -> Parts {
    // No structured fields at all → pure legacy entry, render via label.
    if entry.end_tick == 0 {
        return parse_snapshot_label(&entry.label);
    }
    let total = in_game_duration(entry.end_tick, entry.first_moving);
    if entry.start_tick > 0 {
        return Parts {
            total,
            context: format!(
                "from {} · {}",
                entry.start_tick,
                in_game_duration(entry.start_tick, entry.first_moving)
            ),
            is_marker: false,
        };
    }
    // start_tick == 0: real REC (label "Recorded …") has no context.
    // A "Continued from …" label here means a legacy entry that lost
    // its start_tick — keep its original context.
    if entry.label.starts_with("Continued from ") {
        let legacy = parse_snapshot_label(&entry.label);
        return Parts {
            total,
            context: legacy.context,
            is_marker: false,
        };
    }
    Parts {
        total,
        context: String::new(),
        is_marker: false,
    }
}

fn parse_snapshot_label(label: &str) -> Parts {
    // "Continued from 0:42.35, total 0:50.00" → total 0:50.00, context "from 0:42.35"
    if let Some(rest) = label.strip_prefix("Continued from ") {
        if let Some((splice, total_part)) = rest.split_once(", total ") {
            return Parts {
                total: total_part.trim().to_string(),
                context: format!("from {}", splice.trim()),
                is_marker: false,
            };
        }
    }
    // "Recorded 0:20.55" → total 0:20.55, no context.
    if let Some(total) = label.strip_prefix("Recorded ") {
        return Parts {
            total: total.trim().to_string(),
            context: String::new(),
            is_marker: false,
        };
    }
    // Unrecognised label: dump it whole into context, no total.
    Parts {
        total: String::new(),
        context: label.to_string(),
        is_marker: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_continued_label() {
        let p = parse_snapshot_label("Continued from 0:42.35, total 1:08.24");
        assert_eq!(p.total, "1:08.24");
        assert_eq!(p.context, "from 0:42.35");
        assert!(!p.is_marker);
    }

    #[test]
    fn parses_recorded_label() {
        let p = parse_snapshot_label("Recorded 0:20.55");
        assert_eq!(p.total, "0:20.55");
        // No context for REC — the icon color already says "recording".
        assert!(p.context.is_empty());
    }

    #[test]
    fn parses_unrecognised_label_safely() {
        // Catch-all: unrecognised labels still render without panic.
        let p = parse_snapshot_label("weird label that doesn't match");
        assert!(p.total.is_empty());
        assert_eq!(p.context, "weird label that doesn't match");
    }
}
