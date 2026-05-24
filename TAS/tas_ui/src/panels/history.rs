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

    // Markers have an empty total column; skip the 7-space pad in that
    // case so the layout reads as `💾  Saved → file  21:17` instead of
    // `💾           Saved → file  21:17`.
    let total_slot = if parts.total.is_empty() {
        String::new()
    } else {
        parts.total_padded()
    };
    let row_text = format!(
        "{}  {}  {}  {}",
        parts.icon,
        total_slot,
        parts.context,
        entry.created_at.format("%H:%M"),
    );

    let mut richtext = egui::RichText::new(&row_text).monospace().size(12.0);
    if parts.is_marker {
        richtext = richtext
            .italics()
            .color(match entry.kind {
                HistoryEntryKind::SaveMarker => egui::Color32::from_rgb(120, 170, 220),
                _ => egui::Color32::from_gray(140),
            });
    }

    let response = if restorable {
        ui.selectable_label(is_current, richtext)
    } else {
        ui.label(richtext)
    };

    if restorable && response.clicked() {
        actions.push(HistoryAction::Restore(idx));
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

impl Parts {
    /// Pad the total to a fixed width so the column visually aligns in a
    /// monospaced row layout, regardless of `0:55.05` vs `1:13.97`.
    fn total_padded(&self) -> String {
        format!("{:>7}", self.total)
    }
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
    // context "from 0:42.35 splice"
    if let Some(rest) = label.strip_prefix("Continued from ") {
        if let Some((splice, total_part)) = rest.split_once(", total ") {
            return Parts {
                icon: "▶",
                total: total_part.trim().to_string(),
                context: format!("from {} splice", splice.trim()),
                is_marker: false,
            };
        }
    }
    // "Recorded 0:20.55" → icon ●, total 0:20.55, context "Recorded"
    if let Some(total) = label.strip_prefix("Recorded ") {
        return Parts {
            icon: "●",
            total: total.trim().to_string(),
            context: "Recorded".to_string(),
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
        assert_eq!(p.context, "from 0:42.35 splice");
        assert!(!p.is_marker);
    }

    #[test]
    fn parses_recorded_label() {
        let p = parse_snapshot_label("Recorded 0:20.55");
        assert_eq!(p.icon, "●");
        assert_eq!(p.total, "0:20.55");
        assert_eq!(p.context, "Recorded");
    }

    #[test]
    fn parses_unrecognised_label_safely() {
        // Catch-all: unrecognised labels still render without panic.
        let p = parse_snapshot_label("weird label that doesn't match");
        assert_eq!(p.icon, "●");
        assert!(p.total.is_empty());
        assert_eq!(p.context, "weird label that doesn't match");
    }

    #[test]
    fn padding_aligns_short_and_long_totals() {
        let short = Parts {
            icon: "▶",
            total: "0:55.05".to_string(),
            context: String::new(),
            is_marker: false,
        };
        let long = Parts {
            icon: "▶",
            total: "1:13.97".to_string(),
            context: String::new(),
            is_marker: false,
        };
        // Both pad to width 7 so the column lines up.
        assert_eq!(short.total_padded().len(), 7);
        assert_eq!(long.total_padded().len(), 7);
    }
}
