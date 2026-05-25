use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;

use crate::recording::{HistoryEntry, HistoryEntryKind, RecordingHistory};

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
            // Iterate newest-first. "Newest" = highest index in the stored
            // Vec because RecordingHistory appends. We render top-down to
            // mean newest-on-top.
            let entries = history.entries();
            let mut last_date: Option<NaiveDate> = None;
            for idx in (0..entries.len()).rev() {
                let entry = &entries[idx];
                let entry_date = entry.created_at.date_naive();

                // Sticky day header whenever the date changes as we
                // walk newest→oldest. First entry always gets a header.
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
            // Right side first — using right_to_left layout means the FIRST
            // item added gets pushed to the right edge of the row, and the
            // remaining width flows back to the left for the rest of the
            // content. This stops the time from wrapping onto its own line
            // when the panel is narrow.
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
                            // Icon — kept as the kind's accent color so it
                            // still pops even though the row is colored.
                            ui.label(
                                egui::RichText::new(parts.icon)
                                    .color(kind_color(entry.kind))
                                    .size(13.0),
                            );
                            // Total — right-padded with non-breaking space
                            // equivalents so 0:55 and 1:08 line up
                            // vertically across rows.
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
                            // ellipsis rather than wrapping if the panel
                            // is too narrow.
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
    /// Single-glyph kind icon: ●/▶/💾/📂.
    icon: &'static str,
    /// Duration like `1:08.24`. Empty for markers.
    total: String,
    /// Right-justified context: `from 0:52.00 splice`, `Recorded`,
    /// `Saved → file`, `Loaded ← file`.
    context: String,
    is_marker: bool,
}


/// Parse a history entry's label into (icon, total, context). The labels
/// are produced by `RecoverySessionContext::from_ticks` (`Recorded H:MM.ss`,
/// `Continued from H:MM.ss, total H:MM.ss`) and by `push_save_marker` /
/// `push_loaded_snapshot` (`Save: filename`, `Load: filename`).
fn parse_entry(entry: &HistoryEntry) -> Parts {
    match entry.kind {
        HistoryEntryKind::SaveMarker => Parts {
            icon: "💾",
            total: String::new(),
            context: entry
                .label
                .strip_prefix("Save: ")
                .map(|name| format!("Saved → {}", name))
                .unwrap_or_else(|| entry.label.clone()),
            is_marker: true,
        },
        HistoryEntryKind::LoadSnapshot => Parts {
            icon: "📂",
            total: String::new(),
            context: entry
                .label
                .strip_prefix("Load: ")
                .map(|name| format!("Loaded ← {}", name))
                .unwrap_or_else(|| entry.label.clone()),
            is_marker: true,
        },
        HistoryEntryKind::Snapshot => parse_snapshot_label(&entry.label),
    }
}

fn parse_snapshot_label(label: &str) -> Parts {
    // "Continued from 0:42.35, total 0:50.00" → icon ▶, total 0:50.00,
    // context "from 0:42.35"
    if let Some(rest) = label.strip_prefix("Continued from ") {
        if let Some((splice, total_part)) = rest.split_once(", total ") {
            return Parts {
                icon: "▶",
                total: total_part.trim().to_string(),
                context: format!("from {}", splice.trim()),
                is_marker: false,
            };
        }
    }
    // "Recorded 0:20.55" → icon ●, total 0:20.55, no context (the icon
    // already says it's a recording; "Recorded" as context is just noise).
    if let Some(total) = label.strip_prefix("Recorded ") {
        return Parts {
            icon: "●",
            total: total.trim().to_string(),
            context: String::new(),
            is_marker: false,
        };
    }
    // Unrecognised label: dump it whole into context, no total.
    Parts {
        icon: "●",
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
        assert_eq!(p.icon, "▶");
        assert_eq!(p.total, "1:08.24");
        assert_eq!(p.context, "from 0:42.35");
        assert!(!p.is_marker);
    }

    #[test]
    fn parses_recorded_label() {
        let p = parse_snapshot_label("Recorded 0:20.55");
        assert_eq!(p.icon, "●");
        assert_eq!(p.total, "0:20.55");
        // No context for REC — the icon already says "recording".
        assert!(p.context.is_empty());
    }

    #[test]
    fn parses_unrecognised_label_safely() {
        // Catch-all: unrecognised labels still render without panic.
        let p = parse_snapshot_label("weird label that doesn't match");
        assert_eq!(p.icon, "●");
        assert!(p.total.is_empty());
        assert_eq!(p.context, "weird label that doesn't match");
    }
}
