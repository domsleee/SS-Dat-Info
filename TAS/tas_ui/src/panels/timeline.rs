use crate::panels::input_script::{runs_from_log, InputEvent};
use eframe::egui;
use tas_shared::{input_bits, TasMode, TasSharedState};

const ROW_COLORS: &[(u8, egui::Color32)] = &[
    (input_bits::LEFT, egui::Color32::from_rgb(100, 149, 237)), // cornflower blue
    (input_bits::RIGHT, egui::Color32::from_rgb(255, 165, 0)),  // orange
    (input_bits::UP, egui::Color32::from_rgb(50, 205, 50)),     // lime green
    (input_bits::DOWN, egui::Color32::from_rgb(220, 20, 60)),   // crimson
    (input_bits::JUMP, egui::Color32::from_rgb(186, 85, 211)),  // medium orchid
    (input_bits::SHIFT, egui::Color32::from_rgb(255, 215, 0)),  // gold
];

const ROW_LABELS: &[&str] = &["L", "R", "U", "D", "J", "S"];

/// `press <name>` keyword for each bit (TMInterface-faithful).
const ROW_ACTIONS: &[&str] = &["left", "right", "up", "down", "jump", "shift"];

/// Smallest window the timeline will zoom to — keeps blocks grabbable and
/// stops the brush handle from collapsing to nothing.
const MIN_WINDOW: u32 = 60;

/// Shortest input a drag/box edit will produce (ticks).
const MIN_LEN: u32 = 1;

const ACCENT: egui::Color32 = egui::Color32::from_rgb(120, 180, 240);

fn row_index(bit: u8) -> Option<usize> {
    ROW_COLORS.iter().position(|(b, _)| *b == bit)
}

/// Visible tick window `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimelineView {
    pub start: u32,
    pub end: u32,
}

impl TimelineView {
    pub fn fit(&mut self, total: u32) {
        self.start = 0;
        self.end = total;
    }

    fn span(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    fn clamp(&mut self, total: u32) {
        if total == 0 {
            self.start = 0;
            self.end = 0;
            return;
        }
        if self.end == 0 || self.end > total {
            self.end = total;
        }
        if self.start >= self.end {
            self.start = 0;
        }
        let min_win = MIN_WINDOW.min(total);
        if self.span() < min_win {
            self.end = (self.start + min_win).min(total);
            if self.span() < min_win {
                self.start = self.end.saturating_sub(min_win);
            }
        }
    }

    pub fn zoom_at(&mut self, cursor_tick: f32, factor: f32, total: u32) {
        let span = self.span() as f32;
        if span <= 0.0 {
            return;
        }
        let new_span = (span * factor).clamp(MIN_WINDOW.min(total) as f32, total as f32);
        let frac = ((cursor_tick - self.start as f32) / span).clamp(0.0, 1.0);
        let mut s = cursor_tick - frac * new_span;
        if s < 0.0 {
            s = 0.0;
        }
        if s + new_span > total as f32 {
            s = total as f32 - new_span;
        }
        if s < 0.0 {
            s = 0.0;
        }
        self.start = s.round() as u32;
        self.end = (self.start + new_span.round() as u32).min(total);
    }

    pub fn zoom_center(&mut self, factor: f32) {
        let span = self.span() as f32;
        if span <= 0.0 {
            return;
        }
        let center = self.start as f32 + span / 2.0;
        let new_span = (span * factor).max(MIN_WINDOW as f32);
        let s = (center - new_span / 2.0).max(0.0);
        self.start = s.round() as u32;
        self.end = self.start + new_span.round() as u32;
    }

    fn pan(&mut self, delta: i64, total: u32) {
        let span = self.span() as i64;
        let mut s = self.start as i64 + delta;
        if s + span > total as i64 {
            s = total as i64 - span;
        }
        if s < 0 {
            s = 0;
        }
        self.start = s as u32;
        self.end = (self.start + span as u32).min(total);
    }

    fn follow(&mut self, pos: u32, total: u32) {
        let span = self.span();
        let max_scroll = total.saturating_sub(span);
        let new_start = auto_scroll_position(
            pos as usize,
            self.start as usize,
            span as usize,
            max_scroll as usize,
        ) as u32;
        self.start = new_start;
        self.end = (new_start + span).min(total);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTickMode {
    Rec,
    Play,
}

#[derive(Clone, Copy, PartialEq)]
enum DragMode {
    Start,
    End,
    Move,
}

struct BlockDrag {
    idx: usize,
    mode: DragMode,
}

/// Persistent edit state for the timeline (selection + active block drag +
/// the working event list). Lives in the app; defaults to "nothing selected".
#[derive(Default)]
pub struct TimelineEdit {
    pub selected: Option<InputEvent>,
    work: Vec<InputEvent>,
    drag: Option<BlockDrag>,
}

/// What `show` wants the app to do this frame.
#[derive(Default)]
pub struct TimelineOutcome {
    pub continue_changed: bool,
    /// New full event list to write into `input_log` (Some when edited).
    pub events: Option<Vec<InputEvent>>,
    /// A gesture finished — push one undo snapshot.
    pub commit_undo: bool,
}

pub fn show(
    ui: &mut egui::Ui,
    state: &TasSharedState,
    view: &mut TimelineView,
    continue_from: &mut u32,
    edit: &mut TimelineEdit,
) -> TimelineOutcome {
    let mut outcome = TimelineOutcome::default();
    let total = state.recorded_count;
    if total == 0 {
        ui.colored_label(
            egui::Color32::from_rgb(150, 150, 150),
            "No recording data yet. Press REC to populate timeline rows.",
        );
        return outcome;
    }
    view.clamp(total);

    let active_tick = active_timeline_tick(state);
    if let Some((pos, _)) = active_tick {
        view.follow(pos as u32, total);
    }
    let editable = state.mode == TasMode::Off as u32;

    // Keep the working event list in sync with the log while not dragging.
    if editable && edit.drag.is_none() {
        edit.work = runs_from_log(&state.input_log, total);
    }

    ui.horizontal(|ui| {
        if ui
            .button("Fit")
            .on_hover_text("Show the whole recording")
            .clicked()
        {
            view.fit(total);
        }
        let zoom = total as f32 / view.span().max(1) as f32;
        ui.label(format!(
            "Showing ticks {}–{} of {}  ({:.2}×)",
            view.start, view.end, total, zoom
        ));
        ui.weak("·  scroll to zoom · drag the bar below to pan");
    });

    let avail = ui.available_size();
    let row_height = 12.0;
    let num_rows = ROW_LABELS.len();
    let header_height = 14.0;
    let axis_height = 16.0;
    let total_height = header_height + row_height * num_rows as f32 + axis_height + 6.0;
    let left_margin = 8.0;
    let right_padding = 20.0;
    let width = (avail.x.min(900.0) - right_padding - left_margin).max(120.0);

    let (response, painter) =
        ui.allocate_painter(egui::vec2(width, total_height), egui::Sense::click_and_drag());
    let rect = response.rect;
    painter.rect_filled(rect, 2.0, egui::Color32::from_rgb(30, 30, 40));

    let label_width = 20.0;
    let bar_left = rect.left() + label_width;
    let bar_width = width - label_width - 4.0;
    let rows_top = rect.top() + header_height;
    let rows_bottom = rows_top + row_height * num_rows as f32;
    let axis_y = rows_bottom + 2.0;

    // Wheel zoom, centred on the cursor (mutates the view before we snapshot).
    // Test the raw pointer position against the timeline rect rather than
    // `response.hovered()`: when editing, per-block interact widgets sit on
    // top of the painter and steal its hover, so hovering a key-down block
    // would otherwise suppress zoom. rect.contains works regardless of which
    // widget is topmost.
    if let Some(p) = ui.input(|i| i.pointer.hover_pos()) {
        if rect.contains(p) {
            let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
            if scroll_y.abs() > 0.0 {
                let frac = ((p.x - bar_left) / bar_width).clamp(0.0, 1.0);
                let cursor_tick = view.start as f32 + frac * view.span().max(1) as f32;
                let factor = if scroll_y > 0.0 { 0.85 } else { 1.18 };
                view.zoom_at(cursor_tick, factor, total);
            }
        }
    }

    // Snapshot the window for this frame's drawing. `to_px` owns copies so
    // later view mutations (brush pan/zoom) don't alias it.
    let vs = view.start;
    let ve = view.end;
    let span = ve.saturating_sub(vs).max(1);
    let span_f = span as f32;
    let to_px = move |t: u32| -> f32 {
        let p = bar_left + t.saturating_sub(vs) as f32 / span_f * bar_width;
        p.clamp(bar_left, bar_left + bar_width)
    };

    // Status text (left) + visible range (right).
    let marker_status = match active_tick {
        Some((tick, ActiveTickMode::Rec)) => format!("REC @ frame {}", tick),
        Some((tick, ActiveTickMode::Play)) => format!("PLAY @ frame {}", tick),
        None if editable => "OFF · editable".to_string(),
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
        format!("{}..{}", view.start, view.end.saturating_sub(1)),
        egui::FontId::monospace(9.0),
        egui::Color32::from_rgb(150, 150, 150),
    );

    // Active position marker.
    if let Some((tick, mode)) = active_tick {
        let tick = tick as u32;
        if tick >= view.start && tick < view.end {
            let px = to_px(tick);
            let color = match mode {
                ActiveTickMode::Rec => egui::Color32::from_rgb(120, 255, 120),
                ActiveTickMode::Play => egui::Color32::from_rgb(255, 235, 120),
            };
            let hi = egui::Rect::from_min_max(
                egui::pos2((px - 2.0).max(bar_left), rows_top),
                egui::pos2((px + 2.0).min(bar_left + bar_width), rows_bottom),
            );
            painter.rect_filled(hi, 1.0, color.gamma_multiply(0.35));
            painter.line_segment(
                [egui::pos2(px, rows_top), egui::pos2(px, rows_bottom)],
                egui::Stroke::new(2.0, color),
            );
        }
    }

    // Row labels.
    let y_start = rows_top + 2.0;
    for (row_idx, label) in ROW_LABELS.iter().enumerate() {
        let y = y_start + row_idx as f32 * row_height;
        painter.text(
            egui::pos2(rect.left() + 4.0, y + row_height * 0.5),
            egui::Align2::LEFT_CENTER,
            *label,
            egui::FontId::monospace(9.0),
            egui::Color32::from_rgb(180, 180, 180),
        );
    }

    if editable {
        // Editable: draw each run as an interactable block.
        let pxpt = bar_width / span_f;
        let work_len = edit.work.len();
        let mut edited = false;
        for i in 0..work_len {
            let ev = edit.work[i];
            let Some(row_idx) = row_index(ev.bit) else {
                continue;
            };
            if ev.end <= view.start || ev.start >= view.end {
                continue;
            }
            let color = ROW_COLORS[row_idx].1;
            let y = y_start + row_idx as f32 * row_height;
            let x0 = to_px(ev.start);
            let x1 = to_px(ev.end).max(x0 + 2.0);
            let block = egui::Rect::from_min_max(
                egui::pos2(x0, y + 1.0),
                egui::pos2(x1, y + row_height - 1.0),
            );
            painter.rect_filled(block, 1.0, color);
            if edit.selected == Some(ev) {
                painter.rect_stroke(block, 1.0, egui::Stroke::new(1.5, egui::Color32::WHITE));
            }

            let id = ui.id().with(("blk", ev.bit, i));
            let hw = 4.0_f32.min(block.width() / 3.0);
            let lh = egui::Rect::from_min_max(block.min, egui::pos2(block.left() + hw, block.bottom()));
            let rh = egui::Rect::from_min_max(egui::pos2(block.right() - hw, block.top()), block.max);
            let body = egui::Rect::from_min_max(
                egui::pos2(block.left() + hw, block.top()),
                egui::pos2(block.right() - hw, block.bottom()),
            );
            let lr = ui
                .interact(lh, id.with("l"), egui::Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::ResizeHorizontal);
            let rr = ui
                .interact(rh, id.with("r"), egui::Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::ResizeHorizontal);
            let br = ui
                .interact(body, id.with("b"), egui::Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::Grab);

            if lr.clicked() || rr.clicked() || br.clicked() {
                edit.selected = Some(ev);
            }
            if lr.drag_started() {
                edit.drag = Some(BlockDrag { idx: i, mode: DragMode::Start });
                edit.selected = Some(ev);
            } else if rr.drag_started() {
                edit.drag = Some(BlockDrag { idx: i, mode: DragMode::End });
                edit.selected = Some(ev);
            } else if br.drag_started() {
                edit.drag = Some(BlockDrag { idx: i, mode: DragMode::Move });
                edit.selected = Some(ev);
            }

            if let Some(drag) = &edit.drag {
                if drag.idx == i {
                    let delta = lr.drag_delta().x + rr.drag_delta().x + br.drag_delta().x;
                    let dt = (delta / pxpt).round() as i64;
                    if dt != 0 {
                        let mut e = edit.work[i];
                        match drag.mode {
                            DragMode::Start => {
                                let ns = (e.start as i64 + dt).clamp(0, e.end as i64 - MIN_LEN as i64);
                                e.start = ns as u32;
                            }
                            DragMode::End => {
                                let ne = (e.end as i64 + dt)
                                    .clamp(e.start as i64 + MIN_LEN as i64, total as i64);
                                e.end = ne as u32;
                            }
                            DragMode::Move => {
                                let len = e.end - e.start;
                                let mut ns = e.start as i64 + dt;
                                if ns < 0 {
                                    ns = 0;
                                }
                                if ns + len as i64 > total as i64 {
                                    ns = total as i64 - len as i64;
                                }
                                e.start = ns as u32;
                                e.end = e.start + len;
                            }
                        }
                        edit.work[i] = e;
                        edit.selected = Some(e);
                        edited = true;
                    }
                    if lr.drag_stopped() || rr.drag_stopped() || br.drag_stopped() {
                        edit.drag = None;
                        outcome.commit_undo = true;
                        edited = true;
                    }
                }
            }
        }
        if edited {
            outcome.events = Some(edit.work.clone());
        }
    } else {
        // Read-only: cheap painted runs.
        for (row_idx, &(bit, color)) in ROW_COLORS.iter().enumerate() {
            let y = y_start + row_idx as f32 * row_height;
            let lane = egui::Rect::from_min_size(
                egui::pos2(bar_left, y + 1.0),
                egui::vec2(bar_width, row_height - 2.0),
            );
            paint_runs(&painter, &state.input_log, bit, color, lane, view.start, view.end);
        }
    }

    // X-axis labels.
    painter.line_segment(
        [
            egui::pos2(bar_left, axis_y),
            egui::pos2(bar_left + bar_width, axis_y),
        ],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)),
    );
    let end_tick = view.end.saturating_sub(1);
    let mid_tick = view.start + (end_tick.saturating_sub(view.start) / 2);
    let mut axis_ticks = vec![view.start, mid_tick, end_tick];
    axis_ticks.dedup();
    for tick in axis_ticks {
        let px = to_px(tick);
        painter.line_segment(
            [egui::pos2(px, axis_y), egui::pos2(px, axis_y + 4.0)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 90, 105)),
        );
        let align = if tick == view.start {
            egui::Align2::LEFT_TOP
        } else if tick == end_tick {
            egui::Align2::RIGHT_TOP
        } else {
            egui::Align2::CENTER_TOP
        };
        painter.text(
            egui::pos2(px, axis_y + 5.0),
            align,
            tick.to_string(),
            egui::FontId::monospace(9.0),
            egui::Color32::from_rgb(165, 165, 180),
        );
    }

    // CONT marker: click empty lane area to set continue_from. (Block
    // interactions above capture clicks on blocks, so this only fires on
    // empty space.)
    if response.clicked() || (response.dragged() && !editable) {
        if let Some(pos) = response.interact_pointer_pos() {
            if pos.x >= bar_left
                && pos.x <= bar_left + bar_width
                && pos.y >= rows_top
                && pos.y <= rows_bottom
            {
                let rel = ((pos.x - bar_left) / bar_width).clamp(0.0, 1.0);
                let tick = view.start + (rel * span_f) as u32;
                let frame = tick.min(total.saturating_sub(1));
                if frame != *continue_from {
                    *continue_from = frame;
                    outcome.continue_changed = true;
                }
            }
        }
    }
    let continue_tick = (*continue_from).min(total.saturating_sub(1));
    if continue_tick >= view.start && continue_tick < view.end {
        let px = to_px(continue_tick);
        let cc = egui::Color32::from_rgb(120, 200, 255);
        painter.line_segment(
            [egui::pos2(px, rows_top), egui::pos2(px, rows_bottom)],
            egui::Stroke::new(1.5, cc),
        );
        painter.text(
            egui::pos2(px + 3.0, rows_top + 1.0),
            egui::Align2::LEFT_TOP,
            "CONT",
            egui::FontId::monospace(8.0),
            cc,
        );
    }

    // Overview brush.
    brush(ui, state, view, total, width, bar_left - rect.left());

    // Edit controls row (Start/End/Delete) for the selected input.
    if editable {
        edit_controls(ui, edit, total, &mut outcome);
    } else {
        ui.weak("Stop playback/record (mode OFF) to edit inputs.");
    }

    outcome
}

/// The selected-input editor: key label, Start/End tick boxes, Delete.
fn edit_controls(
    ui: &mut egui::Ui,
    edit: &mut TimelineEdit,
    total: u32,
    outcome: &mut TimelineOutcome,
) {
    let Some(sel) = edit.selected else {
        ui.weak("Click an input block to edit · drag edges to resize, middle to move.");
        return;
    };
    let Some(idx) = edit.work.iter().position(|e| *e == sel) else {
        edit.selected = None;
        return;
    };
    let mut ev = edit.work[idx];
    let row = row_index(ev.bit).unwrap_or(0);
    let mut changed = false;
    let mut delete = false;
    ui.horizontal(|ui| {
        ui.colored_label(ROW_COLORS[row].1, format!("{} · press {}", ROW_LABELS[row], ROW_ACTIONS[row]));
        ui.label("Start");
        changed |= ui.add(egui::DragValue::new(&mut ev.start).speed(1.0)).changed();
        ui.label("End");
        changed |= ui.add(egui::DragValue::new(&mut ev.end).speed(1.0)).changed();
        ui.weak(format!("({:.2}s – {:.2}s)", ev.start as f32 / 100.0, ev.end as f32 / 100.0));
        if ui.button("Delete").clicked() {
            delete = true;
        }
    });

    if delete {
        edit.work.remove(idx);
        edit.selected = None;
        outcome.events = Some(edit.work.clone());
        outcome.commit_undo = true;
        return;
    }
    if changed {
        // Clamp and keep start < end.
        ev.start = ev.start.min(total.saturating_sub(MIN_LEN));
        ev.end = ev.end.min(total);
        if ev.end <= ev.start {
            ev.end = (ev.start + MIN_LEN).min(total);
        }
        edit.work[idx] = ev;
        edit.selected = Some(ev);
        outcome.events = Some(edit.work.clone());
        outcome.commit_undo = true;
    }
}

fn brush(
    ui: &mut egui::Ui,
    state: &TasSharedState,
    view: &mut TimelineView,
    total: u32,
    width: f32,
    inset: f32,
) {
    ui.add_space(4.0);
    let (area, _) = ui.allocate_exact_size(egui::vec2(width, 26.0), egui::Sense::hover());
    let brush_rect = egui::Rect::from_min_size(
        egui::pos2(area.left() + inset, area.top()),
        egui::vec2(width - inset - 4.0, 26.0),
    );
    let painter = ui.painter_at(brush_rect);
    painter.rect_filled(brush_rect, 3.0, egui::Color32::from_rgb(18, 18, 26));

    for (i, &(bit, color)) in ROW_COLORS.iter().enumerate() {
        let y = brush_rect.top() + 3.0 + i as f32 * 3.2;
        let lane = egui::Rect::from_min_size(
            egui::pos2(brush_rect.left() + 3.0, y),
            egui::vec2(brush_rect.width() - 6.0, 2.4),
        );
        paint_runs(
            &painter,
            &state.input_log,
            bit,
            color.gamma_multiply(0.85),
            lane,
            0,
            total,
        );
    }

    let inner_left = brush_rect.left() + 3.0;
    let inner_w = brush_rect.width() - 6.0;
    let ppt = inner_w / total as f32;
    let wx0 = inner_left + view.start as f32 * ppt;
    let wx1 = inner_left + view.end as f32 * ppt;
    let top = brush_rect.top() + 1.0;
    let bot = brush_rect.bottom() - 1.0;

    let win_rect = egui::Rect::from_min_max(egui::pos2(wx0, top), egui::pos2(wx1, bot));
    painter.rect(
        win_rect,
        2.0,
        ACCENT.gamma_multiply(0.18),
        egui::Stroke::new(1.0, ACCENT),
    );

    let hw = 8.0;
    let lh = egui::Rect::from_min_max(egui::pos2(wx0 - 1.0, top), egui::pos2(wx0 + hw, bot));
    let rh = egui::Rect::from_min_max(egui::pos2(wx1 - hw, top), egui::pos2(wx1 + 1.0, bot));
    painter.rect_filled(lh, 2.0, ACCENT);
    painter.rect_filled(rh, 2.0, ACCENT);

    let id = ui.id().with("timeline_brush");
    let body = egui::Rect::from_min_max(
        egui::pos2((wx0 + hw).min(wx1), top),
        egui::pos2((wx1 - hw).max(wx0), bot),
    );
    let body_resp = ui
        .interact(body, id.with("body"), egui::Sense::drag())
        .on_hover_cursor(egui::CursorIcon::Grab);
    if body_resp.dragged() {
        let dt = (body_resp.drag_delta().x / ppt).round() as i64;
        if dt != 0 {
            view.pan(dt, total);
        }
    }
    let l_resp = ui
        .interact(lh, id.with("lh"), egui::Sense::drag())
        .on_hover_cursor(egui::CursorIcon::ResizeHorizontal);
    if l_resp.dragged() {
        let dt = (l_resp.drag_delta().x / ppt).round() as i32;
        let ns = (view.start as i32 + dt).clamp(0, view.end as i32 - MIN_WINDOW as i32);
        view.start = ns.max(0) as u32;
    }
    let r_resp = ui
        .interact(rh, id.with("rh"), egui::Sense::drag())
        .on_hover_cursor(egui::CursorIcon::ResizeHorizontal);
    if r_resp.dragged() {
        let dt = (r_resp.drag_delta().x / ppt).round() as i32;
        let ne = (view.end as i32 + dt).clamp(view.start as i32 + MIN_WINDOW as i32, total as i32);
        view.end = ne as u32;
    }
}

fn paint_runs(
    painter: &egui::Painter,
    log: &[u8],
    bit: u8,
    color: egui::Color32,
    lane: egui::Rect,
    win_start: u32,
    win_end: u32,
) {
    let span = (win_end.saturating_sub(win_start)).max(1) as f32;
    let w = lane.width();
    let mut run_start_px: Option<f32> = None;
    for t in win_start..win_end {
        let on = log.get(t as usize).map(|m| m & bit != 0).unwrap_or(false);
        let x = lane.left() + (t - win_start) as f32 / span * w;
        if on && run_start_px.is_none() {
            run_start_px = Some(x);
        } else if !on {
            if let Some(sx) = run_start_px.take() {
                let pw = (x - sx).max(1.0);
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(sx, lane.top()),
                        egui::vec2(pw, lane.height()),
                    ),
                    1.0,
                    color,
                );
            }
        }
    }
    if let Some(sx) = run_start_px.take() {
        let endx = lane.left() + (win_end - win_start) as f32 / span * w;
        let pw = (endx - sx).max(1.0);
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(sx, lane.top()), egui::vec2(pw, lane.height())),
            1.0,
            color,
        );
    }
}

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
        assert_eq!(auto_scroll_position(50, 0, 100, 1000), 0);
    }

    #[test]
    fn auto_scroll_jumps_when_past_end() {
        let result = auto_scroll_position(150, 0, 100, 1000);
        assert!(result > 0);
        assert!(150 >= result && 150 < result + 100);
    }

    #[test]
    fn auto_scroll_jumps_when_before_start() {
        let result = auto_scroll_position(10, 200, 100, 1000);
        assert!(result <= 10);
    }

    #[test]
    fn auto_scroll_advances_past_margin() {
        let result = auto_scroll_position(85, 0, 100, 1000);
        assert!(result > 0);
    }

    #[test]
    fn auto_scroll_clamps_to_max() {
        let result = auto_scroll_position(900, 0, 100, 50);
        assert_eq!(result, 50);
    }

    #[test]
    fn view_clamp_snaps_uninit_to_fit() {
        let mut v = TimelineView::default();
        v.clamp(500);
        assert_eq!(v, TimelineView { start: 0, end: 500 });
    }

    #[test]
    fn view_clamp_enforces_min_window() {
        let mut v = TimelineView { start: 100, end: 110 };
        v.clamp(1000);
        assert!(v.span() >= MIN_WINDOW);
    }

    #[test]
    fn view_zoom_at_keeps_cursor_and_clamps() {
        let mut v = TimelineView { start: 0, end: 1000 };
        v.zoom_at(500.0, 0.5, 1000);
        assert!(v.span() < 1000);
        assert!(v.start < 500 && v.end > 500);
        v.zoom_at(500.0, 100.0, 1000);
        assert_eq!(v, TimelineView { start: 0, end: 1000 });
    }

    #[test]
    fn view_zoom_center_shrinks_and_grows() {
        let mut v = TimelineView { start: 100, end: 1100 };
        v.zoom_center(0.8);
        assert!(v.span() < 1000);
        let mut v2 = TimelineView { start: 100, end: 1100 };
        v2.zoom_center(1.25);
        assert!(v2.span() > 1000);
    }

    #[test]
    fn view_pan_preserves_span_and_clamps() {
        let mut v = TimelineView { start: 100, end: 200 };
        v.pan(-1000, 1000);
        assert_eq!(v, TimelineView { start: 0, end: 100 });
        v.pan(100000, 1000);
        assert_eq!(v, TimelineView { start: 900, end: 1000 });
    }

    #[test]
    fn active_tick_rec_uses_last_recorded_frame() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Rec as u32;
        state.recorded_count = 42;
        state.playback_pos = 0;
        assert_eq!(active_timeline_tick(&state), Some((41, ActiveTickMode::Rec)));
    }

    #[test]
    fn active_tick_play_uses_playback_frame() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.recorded_count = 100;
        state.playback_pos = 37;
        assert_eq!(active_timeline_tick(&state), Some((37, ActiveTickMode::Play)));
    }

    #[test]
    fn active_tick_off_has_no_marker() {
        let mut state = zeroed_boxed();
        state.mode = TasMode::Off as u32;
        state.recorded_count = 20;
        state.playback_pos = 7;
        assert_eq!(active_timeline_tick(&state), None);
    }
}
