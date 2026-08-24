use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;

use crate::recording::{
    format_recording_duration, HistoryEntry, HistoryEntryKind, RecordingHistory,
};

pub enum HistoryAction {
    Restore(usize),
    ClearSelection,
    /// Toggle the pin on the entry with this stable id, to `pinned`.
    SetPin(u64, bool),
    /// Set the user-given name of the entry (blank clears it).
    Rename(u64, String),
}

pub fn show(
    ui: &mut egui::Ui,
    history: &RecordingHistory,
    in_menu: bool,
    game_in_game: bool,
) -> Vec<HistoryAction> {
    let mut actions = Vec::new();
    // Rows are clickable, not text — don't show the I-beam / allow text drag.
    ui.style_mut().interaction.selectable_labels = false;

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

    // Per-level view: history is ALWAYS scoped to the current track — you
    // work one level at a time, and restoring another track's snapshot into
    // this session would be wrong anyway. The scope is the last-KNOWN level
    // (sticky across menu visits); before any level is seen, everything
    // shows. Entries with no level tag (legacy, or unclassifiable shared
    // spawns) are always shown — hiding them would "lose" pre-tag history.
    let level_filter = history.live_level().map(str::to_owned);
    let resolving = history.level_is_resolving();
    if history.level_is_resolving() {
        if in_menu {
            // Quit-to-menu also unresolves the level context, but NOTHING is
            // being resolved there — the scan is deliberately suppressed at
            // menus (it would just re-confirm the track you left). A
            // perpetual "resolving…" here read as a stuck spinner; say where
            // the game actually is. Same three-way split as the status chip:
            // the frozen-cycle state with the in-game flag still set could be
            // the pause menu, a post-race dialog (save high time / save
            // replay), OR the main menu after leaving a level — the engine
            // keeps the level resident behind all of them, so they are
            // indistinguishable from outside. Only a fresh boot (flag 0) is
            // provably the plain menu.
            let (label, hover) = if game_in_game {
                (
                    "Level: Menu / Paused",
                    "The engine isn't simulating — the pause menu, a post-race \
                     dialog, or the main menu after leaving a level (they look \
                     identical from outside). Rows are hidden until a level is \
                     running again.",
                )
            } else {
                (
                    "Level: In Menu",
                    "The game is in a menu. Rows are hidden until a level is \
                     entered — restoring a track's snapshot from the menu would \
                     write over the live buffer.",
                )
            };
            ui.label(
                egui::RichText::new(label)
                    .size(10.0)
                    .color(egui::Color32::from_gray(120)),
            )
            .on_hover_text(hover);
        } else {
            // Between a level change and the scan publishing the new track we
            // do NOT know where we are. Say so instead of asserting the old
            // track — silently showing the previous level's entries here is
            // exactly the "loading Forest Medium, seeing Forest Easy's saves"
            // confusion.
            ui.label(
                egui::RichText::new("Level: resolving…")
                    .size(10.0)
                    .color(egui::Color32::from_gray(120)),
            )
            .on_hover_text(
                "The level changed and the track hasn't been identified yet \
                 (usually a fraction of a second — the scan polls fast while \
                 unresolved). Rows are hidden until it is — showing the previous \
                 track's entries here would also let you restore one over the \
                 live buffer.",
            );
        }
    } else if let Some(code) = level_filter.as_deref() {
        // Only CLAIM untagged entries are shown when some actually are. The
        // suffix was unconditional, so it kept advertising a caveat that had
        // stopped applying — and a caveat you can't turn off reads as "this
        // filter is approximate", which is the opposite of the truth once every
        // entry is tagged.
        let any_untagged = history.entries().iter().any(|e| e.level.is_none());
        // The nine race tracks read naturally as their codes (FE/VH — the
        // user's own naming convention), but "PE" is cryptic: the practice
        // run has a name, use it. (Entries/files still use the PE code.)
        let code = if code == "PE" { "Practice" } else { code };
        // The sticky level survives freezes (pause / post-race dialogs / the
        // menu) so the rows stay usable there — but say the engine is not
        // actually running this level right now.
        let pause_suffix = if in_menu { " · paused" } else { "" };
        let (text, hover) = if any_untagged {
            (
                format!("Level: {}{} · untagged shown", code, pause_suffix),
                "History is per-level: entries made on this track, plus ones \
                 marked \"untagged\", which belong to no known track and so \
                 appear everywhere. Switches with the game.",
            )
        } else {
            (
                format!("Level: {}{}", code, pause_suffix),
                "History is per-level: only entries made on this track are listed. \
                 Switches with the game.",
            )
        };
        ui.label(
            egui::RichText::new(text)
                .size(10.0)
                .color(egui::Color32::from_gray(120)),
        )
        .on_hover_text(hover);
    }

    // Inline-rename state (which entry is being edited + its text buffer),
    // persisted in egui memory across frames.
    let rename_key = egui::Id::new("history_rename_state");
    let mut edit: Option<(u64, String)> = ui.data_mut(|d| d.get_temp(rename_key));

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        // Desktop: scroll with the wheel/scrollbar only. Without this, egui's
        // touch-style "drag the content to scroll" fires when you press-and-hold
        // a row (e.g. holding the pin while the screenshot tool grabs a drag).
        .drag_to_scroll(false)
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
            let mut visible: Vec<(usize, &HistoryEntry)> = entries
                .iter()
                .enumerate()
                .filter(|(_, e)| {
                    // RESOLVING: we do not know the track, so nothing qualifies.
                    // Leaving rows visible also leaves them RESTORABLE, which is
                    // how the transition window became able to overwrite the live
                    // buffer with another track's recording.
                    if resolving {
                        return false;
                    }
                    match (&level_filter, &e.level) {
                        (Some(want), Some(have)) => want == have,
                        // No filter active, or an untagged entry: always visible.
                        _ => true,
                    }
                })
                .collect();
            visible.sort_by(|(a_idx, a), (b_idx, b)| {
                // Newest day first; PINNED float to the top within their day;
                // then newest-first, with push recency as the final tiebreak.
                b.created_at
                    .date_naive()
                    .cmp(&a.created_at.date_naive())
                    .then_with(|| b.pinned.cmp(&a.pinned))
                    .then_with(|| b.created_at.cmp(&a.created_at))
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
                render_row(ui, entry, idx, is_current, &mut edit, &mut actions);
            }

            // Empty space below the last row acts as a "deselect" target:
            // a click on the background clears the current-row highlight.
            // We reserve at least 40 px so there's always something to
            // click even when the entries fill the visible area; if the
            // panel has more vertical room, allocate it all so the click
            // target spans the whole gap to the bottom edge.
            let remaining = ui.available_height().max(40.0);
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), remaining),
                egui::Sense::click(),
            );
            let _ = rect;
            if response.clicked() {
                actions.push(HistoryAction::ClearSelection);
            }
        });

    // F2 renames the currently-selected entry (Windows convention).
    if edit.is_none() && ui.input(|i| i.key_pressed(egui::Key::F2)) {
        if let Some(e) = current.and_then(|i| history.entries().get(i)) {
            edit = Some((e.entry_id, e.custom_name.clone().unwrap_or_default()));
            ui.memory_mut(|m| m.request_focus(egui::Id::new(("hist_rename", e.entry_id))));
        }
    }

    // Persist (or clear) the inline-rename state for next frame.
    ui.data_mut(|d| match &edit {
        Some(e) => d.insert_temp(rename_key, e.clone()),
        None => d.remove::<(u64, String)>(rename_key),
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
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

fn weekday_abbr(d: u32) -> &'static str {
    match d {
        0 => "Mon",
        1 => "Tue",
        2 => "Wed",
        3 => "Thu",
        4 => "Fri",
        5 => "Sat",
        6 => "Sun",
        _ => "???",
    }
}

fn render_row(
    ui: &mut egui::Ui,
    entry: &HistoryEntry,
    idx: usize,
    is_current: bool,
    edit: &mut Option<(u64, String)>,
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

    // Current-row background spans the whole row (star + body). Pinned rows get
    // a subtle gold tint so they read as "kept" even when not selected.
    let mut frame = egui::Frame::none().inner_margin(egui::Margin::symmetric(2.0, 1.0));
    if is_current {
        frame = frame.fill(egui::Color32::from_rgba_unmultiplied(192, 132, 252, 38));
    } else if entry.pinned {
        frame = frame.fill(egui::Color32::from_rgba_unmultiplied(232, 184, 75, 12));
    }

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // --- Pin star: its OWN column + generous hit box, fully separate
            //     from the restore area so it can never trigger a restore. ---
            // Real Label as the base so the glyph is ALWAYS drawn (a pure
            // painter star vanished in some pressed/active states).
            let (glyph, base_color) = if entry.pinned {
                ("★", egui::Color32::from_rgb(232, 184, 75))
            } else {
                ("☆", egui::Color32::from_gray(125))
            };
            let star = ui.add_sized(
                egui::vec2(22.0, 20.0),
                egui::Label::new(egui::RichText::new(glyph).size(16.0).color(base_color))
                    .sense(egui::Sense::click()),
            );
            // Hover/press overlay (additive — never hides the base glyph).
            // Use contains_pointer (NOT hovered): egui's hovered() goes false
            // while the button is held down, which made the highlight flicker
            // off mid-press. Show a slightly stronger "pressed" state on hold.
            let over = star.contains_pointer();
            let down = star.is_pointer_button_down_on();
            if over || down {
                ui.painter().rect_filled(
                    star.rect,
                    3.0,
                    egui::Color32::from_rgba_unmultiplied(
                        255,
                        255,
                        255,
                        if down { 40 } else { 22 },
                    ),
                );
                let hc = if entry.pinned {
                    egui::Color32::from_rgb(255, 210, 100)
                } else {
                    egui::Color32::from_gray(215)
                };
                ui.painter().text(
                    star.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    glyph,
                    egui::FontId::proportional(if down { 15.0 } else { 16.0 }),
                    hc,
                );
            }
            let star = star
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(if entry.pinned {
                    "Unpin"
                } else {
                    "Pin (keep forever)"
                });
            if star.clicked() {
                actions.push(HistoryAction::SetPin(entry.entry_id, !entry.pinned));
            }

            // --- Body: time (right), then ▶ + name/duration — or the rename
            //     box if this row is being edited. ---
            let editing = edit.as_ref().is_some_and(|(id, _)| *id == entry.entry_id);
            let body = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !editing {
                    ui.label(
                        egui::RichText::new(&time_str)
                            .size(11.0)
                            .color(egui::Color32::from_gray(120))
                            .monospace(),
                    );
                    // UNTAGGED marker. These belong to no known track, so
                    // they show on EVERY one — which is indistinguishable
                    // from "this row belongs here" unless we say otherwise.
                    // That ambiguity is what made a screenful of Forest Easy
                    // favourites look like a broken filter while on another
                    // track. Dim and glyph-only: it must not compete with the
                    // run time, which is what the eye is actually scanning.
                    // (This layout is right-to-left, so adding it AFTER the
                    // timestamp places it to the LEFT of it.)
                    //
                    // A WORD, not a symbol. The first attempt used "◌"
                    // (U+25CC), which is not in egui's bundled fonts and
                    // rendered as a tofu box — a marker nobody can read is
                    // worse than none, and it took a screenshot to catch,
                    // since the glyph looked fine in the source. Text also
                    // matches the header's wording, so the row and the
                    // summary say the same thing.
                    if entry.level.is_none() {
                        ui.label(
                            egui::RichText::new("untagged")
                                .size(9.0)
                                .color(egui::Color32::from_gray(105)),
                        )
                        .on_hover_text(
                            "No level tag — this recording is not associated \
                                 with a track, so it appears in every track's \
                                 history.",
                        );
                    }
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // A recovered entry is just a normal CONT entry that
                    // was auto-pinned after a crash — it renders through
                    // the SAME path as any entry (total + "from …"); the
                    // only difference is a ⟲ leading glyph instead of ▶.
                    // (Detected from the legacy "Recovered · …" name or the
                    // new "⟲" marker, so old histories normalize too.)
                    let recovered = entry
                        .custom_name
                        .as_deref()
                        .is_some_and(|n| n == "⟲" || n.starts_with("Recovered ·"));
                    ui.label(
                        egui::RichText::new(if recovered { "⟲" } else { "▶" })
                            .color(kind_color(entry.kind))
                            .size(13.0),
                    );
                    if editing {
                        let id = egui::Id::new(("hist_rename", entry.entry_id));
                        let (done, esc, name) = {
                            let buf = &mut edit.as_mut().unwrap().1;
                            let te = ui.add(
                                egui::TextEdit::singleline(buf)
                                    .id(id)
                                    .desired_width(170.0)
                                    .hint_text("name…"),
                            );
                            let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
                            (te.lost_focus(), esc, buf.clone())
                        };
                        // Enter or click-away commits; Esc cancels.
                        if esc {
                            *edit = None;
                        } else if done {
                            actions.push(HistoryAction::Rename(entry.entry_id, name));
                            *edit = None;
                        }
                    } else if let Some(name) = entry.custom_name.as_deref().filter(|_| !recovered) {
                        // Genuine user-typed name. Name leads + the total
                        // (color gold is reserved for pin status).
                        ui.label(egui::RichText::new(name).size(13.0).strong());
                        if !parts.total.is_empty() {
                            ui.label(
                                egui::RichText::new(parts.total.trim())
                                    .monospace()
                                    .size(11.0)
                                    .color(egui::Color32::from_gray(120)),
                            );
                        }
                    } else {
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
                        let mut ctx_rt = egui::RichText::new(&parts.context)
                            .size(12.0)
                            .color(row_color);
                        if parts.is_marker {
                            ctx_rt = ctx_rt.italics();
                        }
                        ui.add(egui::Label::new(ctx_rt).truncate());
                    }
                });
            });

            if !editing {
                let r = body
                    .response
                    .interact(egui::Sense::click())
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text("Click to restore · right-click or F2 to rename");
                let clicked = r.clicked();
                // Right-click context menu — the discoverable rename path
                // (double-click stays "restore/open", per convention).
                r.context_menu(|ui| {
                    if restorable && ui.button("▶  Restore").clicked() {
                        actions.push(HistoryAction::Restore(idx));
                        ui.close_menu();
                    }
                    let pin_label = if entry.pinned {
                        "☆  Unpin"
                    } else {
                        "★  Pin"
                    };
                    if ui.button(pin_label).clicked() {
                        actions.push(HistoryAction::SetPin(entry.entry_id, !entry.pinned));
                        ui.close_menu();
                    }
                    if ui.button("✏  Rename…").clicked() {
                        *edit = Some((
                            entry.entry_id,
                            entry.custom_name.clone().unwrap_or_default(),
                        ));
                        ui.memory_mut(|m| {
                            m.request_focus(egui::Id::new(("hist_rename", entry.entry_id)))
                        });
                        ui.close_menu();
                    }
                });
                if restorable && clicked {
                    actions.push(HistoryAction::Restore(idx));
                }
            }
        });
    });
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
