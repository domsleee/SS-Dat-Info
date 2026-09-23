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

    // History is scoped to the last-known level (sticky across menu visits),
    // since restoring another track's snapshot is never useful. Before any
    // level is seen everything shows. Untagged entries (no level stamp, or an
    // unclassifiable shared spawn) always show so they are never hidden.
    let level_filter = history.live_level().map(str::to_owned);
    let resolving = history.level_is_resolving();
    if history.level_is_resolving() {
        if in_menu {
            // Quit-to-menu unresolves the level, but the scan is suppressed at
            // menus, so nothing will resolve it: say where the game is instead
            // of "resolving…". Same split as the status chip: with the in-game
            // flag set, the pause menu, a post-race dialog and the main menu
            // after a level all look identical (the engine keeps the level
            // resident). Only a fresh boot (flag 0) is provably the menu.
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
            // Between a level change and the scan publishing the new track the
            // level is unknown; say so rather than show the previous track.
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
        // Mention untagged entries only when some exist, so a fully tagged
        // history does not read as an approximate filter.
        let any_untagged = history.entries().iter().any(|e| e.level.is_none());
        // Race tracks read naturally as their codes (FE, VH), but "PE" is
        // cryptic, so name the practice run. Entries and files keep "PE".
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
        .id_salt("history_runs")
        .animated(false)
        .auto_shrink([false, false])
        // Wheel/scrollbar only: egui's drag-to-scroll would fire on a
        // press-and-hold of a row or the pin.
        .drag_to_scroll(false)
        .show(ui, |ui| {
            // Only the view is sorted (by `created_at`, newest first); the
            // model stays in push order so `current_index` and undo/redo are
            // unaffected. Each row carries its vec index so Restore(idx)
            // still targets the right entry.
            let entries = history.entries();
            let mut visible: Vec<(usize, &HistoryEntry)> = entries
                .iter()
                .enumerate()
                .filter(|(_, e)| {
                    // While resolving, the track is unknown, so hide every row:
                    // a visible row is restorable and could overwrite the live
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
            // The cap can keep len() unchanged when a new take replaces an
            // old one. Follow new stable IDs, not the count or selection, so
            // fresh runs are visible without disturbing ordinary browsing.
            let newest = visible.iter().map(|(_, e)| e.entry_id).max();
            let seen_key = ui.id().with("newest_history_entry");
            let previous: Option<u64> = ui.data(|d| d.get_temp(seen_key));
            let reveal = newest.filter(|id| previous.is_some_and(|seen| *id > seen));
            if let Some(id) = newest {
                ui.data_mut(|d| d.insert_temp(seen_key, id));
            }
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
            // The header of the day being rendered. A reveal scrolls to the
            // span from this header down to the new row, so the day label and
            // the pinned rows above the newest take stay in view.
            let mut day_header: Option<egui::Rect> = None;
            for (idx, entry) in visible {
                let entry_date = entry.created_at.date_naive();
                if last_date != Some(entry_date) {
                    day_header = Some(render_day_header(ui, entry_date, today, yesterday));
                    last_date = Some(entry_date);
                }
                let is_current = current == Some(idx);
                let row = render_row(
                    ui,
                    entry,
                    idx,
                    is_current,
                    &mut edit,
                    &mut actions,
                    (history.live_physics(), history.live_rider()),
                );
                if reveal == Some(entry.entry_id) {
                    let span = day_header.map_or(row.rect, |header| header.union(row.rect));
                    // A day with more pinned rows than fit: the new row wins.
                    let target = if span.height() > ui.clip_rect().height() {
                        row.rect
                    } else {
                        span
                    };
                    ui.scroll_to_rect(target, Some(egui::Align::Min));
                    ui.ctx().request_repaint();
                }
            }

            // Clicking the empty space below the last row clears the
            // selection. At least 40 px is reserved so the target exists even
            // when the rows fill the panel.
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

/// Returns the header's full extent (leading space included) so a reveal can
/// scroll it into view together with the first row beneath it.
fn render_day_header(
    ui: &mut egui::Ui,
    date: NaiveDate,
    today: NaiveDate,
    yesterday: Option<NaiveDate>,
) -> egui::Rect {
    let top = ui.cursor().min;
    // Built by hand: chrono's `%-d` (no-pad day) is unsupported on Windows
    // and renders a literal `-d`.
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
    let label = ui.label(
        egui::RichText::new(label)
            .size(10.0)
            .color(egui::Color32::from_gray(120)),
    );
    ui.add_space(2.0);
    egui::Rect::from_min_max(top, label.rect.max)
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
    live_stamps: (Option<&str>, Option<&str>),
) -> egui::Response {
    let (live_physics, live_rider) = live_stamps;
    let parts = parse_entry(entry);
    let restorable = entry.can_restore();
    let time_str = entry.created_at.format("%H:%M").to_string();

    // Markers (save/load) are dimmed or blue and italic; snapshot rows use the
    // default text color so they stand out.
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

    frame
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;

                // Pin star: its own column and hit box, separate from the
                // restore area so it can never trigger a restore. A real Label
                // is the base so the glyph is drawn in every press state.
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
                // Additive hover/press overlay. Uses contains_pointer because
                // egui's hovered() goes false while the button is held.
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

                // Body: time on the right, then ▶ and name/duration, or the
                // rename box while this row is being edited.
                let editing = edit.as_ref().is_some_and(|(id, _)| *id == entry.entry_id);
                let body = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if !editing {
                        ui.label(
                            egui::RichText::new(&time_str)
                                .size(11.0)
                                .color(egui::Color32::from_gray(120))
                                .monospace(),
                        );
                        // Recorded under a different renderer / x87 precision than
                        // the game is running now: restoring it replays different
                        // physics (24-bit DirectX vs 53-bit OpenGL).
                        if let (Some(stamp), Some(live)) = (entry.physics.as_deref(), live_physics)
                        {
                            if stamp != live {
                                ui.label(
                                    egui::RichText::new("\u{26A0}")
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(255, 140, 60)),
                                )
                                .on_hover_text(format!(
                                    "Recorded under {}; the game is running {}. Different x87 \
                                 precision, so this take will not replay bit-exact.",
                                    stamp, live
                                ));
                            }
                        }
                        // Recorded as another character / stance than the one on
                        // the board now: the physics differ, the take will not
                        // line up.
                        if let (Some(stamp), Some(live)) = (entry.rider.as_deref(), live_rider) {
                            if stamp != live {
                                ui.label(
                                    egui::RichText::new("\u{26A0}")
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(255, 140, 60)),
                                )
                                .on_hover_text(format!(
                                    "Recorded as {}; the rider is {}. A different character or \
                                 stance has different physics, so this take will not line up.",
                                    stamp, live
                                ));
                            }
                        }
                        // Untagged entries show on every track, so mark them or
                        // they look like a broken filter. Dim so it does not
                        // compete with the run time. A word, not a symbol:
                        // egui's bundled fonts lack most marker glyphs. The
                        // layout is right-to-left, so this lands left of the
                        // timestamp.
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
                        // A recovered entry (auto-pinned after a crash) renders
                        // like any other, led by ⟲ instead of ▶. It is detected
                        // by the "⟲" name or the "Recovered · …" name that
                        // older histories carry.
                        let recovered = entry
                            .custom_name
                            .as_deref()
                            .is_some_and(|n| n == "⟲" || n.starts_with("Recovered ·"));
                        ui.label(
                            egui::RichText::new(if recovered { "⟲" } else { "▶" })
                                .color(kind_color(entry.kind))
                                .size(13.0),
                        );
                        // Finished run: the flag stays whatever the entry is
                        // renamed to (the status chip shows the same glyph).
                        if let Some(cs) = entry.finish_time_cs {
                            let source = if entry.finish_time_exact {
                                "the HUD timer"
                            } else {
                                "the start-line to finish-line ticks of the recording \
                             (the HUD timer feed was empty; within ~0.05 s)"
                            };
                            ui.label(egui::RichText::new("\u{1F3C1}").size(13.0))
                                .on_hover_text(format!(
                                    "Crossed the finish line — race time {} from {}; \
                                 the recording was auto-stopped there",
                                    crate::recording::format_finish_time(
                                        cs,
                                        entry.finish_time_exact
                                    ),
                                    source
                                ));
                        }
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
                        } else if let Some(name) =
                            entry.custom_name.as_deref().filter(|_| !recovered)
                        {
                            // User-typed name, then the total. Gold is reserved
                            // for pin status.
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
        })
        .response
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
    /// Context text: `from 0:52.00`, `Saved file`, `Loaded file`. Empty for
    /// plain REC entries.
    context: String,
    is_marker: bool,
}

/// Build display parts for a history entry from its structured metadata.
///
/// In-game time is `tick - first_moving` converted to clock format via
/// `format_recording_duration`. When `first_moving` is `None` we fall
/// back to raw `tick / 100`, which is recording-elapsed (not in-game)
/// time — flagged the same way in the panel layout.
fn parse_entry(entry: &HistoryEntry) -> Parts {
    match entry.kind {
        HistoryEntryKind::SaveMarker => Parts {
            total: String::new(),
            // No arrow glyph: `→` renders as a hollow square in egui's
            // default font.
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

fn finished_parts(cs: u32, exact: bool) -> Parts {
    Parts {
        total: format!("Finish {}", crate::recording::format_finish_time(cs, exact)),
        context: String::new(),
        is_marker: false,
    }
}

fn parse_snapshot(entry: &HistoryEntry) -> Parts {
    let total = in_game_duration(entry.end_tick, entry.first_moving);
    // A finished run has one authoritative visible time. Do not also render
    // its recording duration or continuation origin beside it.
    if let Some(cs) = entry.finish_time_cs {
        return finished_parts(cs, entry.finish_time_exact);
    }
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
    Parts {
        total,
        context: String::new(),
        is_marker: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history_frame(ctx: &egui::Context, history: &RecordingHistory, offset: Option<f32>) -> f32 {
        let mut scroll_y = 0.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(340.0, 240.0),
            )),
            focused: false,
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let id = ui.make_persistent_id(egui::Id::new("history_runs"));
                if let Some(y) = offset {
                    let mut state = egui::scroll_area::State::load(ctx, id).unwrap_or_default();
                    state.offset.y = y;
                    state.store(ctx, id);
                }
                assert!(show(ui, history, false, true).is_empty());
                scroll_y = egui::scroll_area::State::load(ctx, id).unwrap().offset.y;
            });
        });
        scroll_y
    }

    fn full_history() -> (RecordingHistory, crate::recording::RecordingSnapshot) {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 10;
        let snapshot = crate::recording::RecordingSnapshot::from_state(&state);
        let mut history = RecordingHistory::new(20);
        for _ in 0..20 {
            history.push_snapshot_data(snapshot.clone(), "Recorded 0:00.10");
        }
        (history, snapshot)
    }

    #[test]
    fn new_run_at_capacity_is_revealed_without_focus_or_clicks() {
        let ctx = egui::Context::default();
        let (mut history, snapshot) = full_history();
        history_frame(&ctx, &history, None);
        assert!(history_frame(&ctx, &history, Some(250.0)) > 200.0);
        let count = history.len();
        history.push_snapshot_data(snapshot, "Recorded 0:00.11");
        assert_eq!(history.len(), count, "eviction keeps the count unchanged");
        history_frame(&ctx, &history, None);
        assert!(
            history_frame(&ctx, &history, None) < 1.0,
            "new run must be visible together with its day header"
        );
    }

    #[test]
    fn reveal_from_the_top_keeps_the_day_header_in_view() {
        // The live sequence: a fresh UI shows the list from the top, a take
        // ends, the new row is revealed. The "Today" header above it must not
        // be scrolled out by the reveal.
        let ctx = egui::Context::default();
        let (mut history, snapshot) = full_history();
        // A pinned take floats above the newest row within its day, so the
        // revealed row is NOT the first under the header.
        let pinned = history.entries()[0].entry_id;
        history.set_pinned(pinned, true);
        for _ in 0..3 {
            assert_eq!(history_frame(&ctx, &history, None), 0.0);
        }
        history.push_snapshot_data(snapshot, "Recorded 0:00.11");
        let offsets: Vec<f32> = (0..4)
            .map(|_| history_frame(&ctx, &history, None))
            .collect();
        assert!(
            offsets.iter().all(|y| *y < 1.0),
            "reveal scrolled the header away: {offsets:?}"
        );
    }

    #[test]
    fn history_refresh_and_rename_preserve_manual_scroll() {
        let ctx = egui::Context::default();
        let (mut history, _) = full_history();
        history_frame(&ctx, &history, None);
        let offset = history_frame(&ctx, &history, Some(250.0));
        let newest = history.entries().last().unwrap().entry_id;
        history.rename(newest, "My run");
        history.clear_selection();
        for _ in 0..3 {
            assert_eq!(history_frame(&ctx, &history, None), offset);
        }
    }

    #[test]
    fn finished_run_has_one_race_time() {
        let exact = finished_parts(4_690, true);
        assert_eq!(exact.total, "Finish 0:46.90");
        assert!(exact.context.is_empty());

        let estimated = finished_parts(4_690, false);
        assert_eq!(estimated.total, "Finish ~0:46.90");
        assert!(estimated.context.is_empty());
    }
}
