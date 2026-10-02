//! What the window shows and the widget state behind it. The panels draw
//! from the units and return actions; the app dispatches them.

use eframe::egui;
use tas_shared::{TasMode, TasSharedState};

use crate::drift_scan::DriftTracker;
use crate::panels::timeline::TimelineView;
use crate::panels::{config, history, input_script, log_panel, status, timeline, transport};
use crate::ui_log::UiLog;
use crate::{editor, history_store, pico, recording, script_watch, TasApp};

pub(crate) struct View {
    pub(crate) timeline: TimelineView,
    /// The CONT splice staged for the next cycle, and the From field's text.
    /// Never synced to shared memory here: the controller writes it when a
    /// cycle starts and on each reroll, and syncing it would move a running
    /// CONT's splice.
    pub(crate) continue_from_frame: u32,
    pub(crate) continue_from_text: String,
    pub(crate) drift: DriftView,
    /// Previous-frame pressed state for F9, F10, F11, F12, diffed against
    /// GetAsyncKeyState to detect press edges. Updated every frame regardless
    /// of focus so a focus change can't leave a key stuck "pressed".
    pub(crate) prev_global_keys: [bool; 4],
    /// One-shot: force dark title bar on first frame.
    pub(crate) dark_title_bar_set: bool,
}

impl Default for View {
    fn default() -> Self {
        Self {
            timeline: TimelineView::default(),
            continue_from_frame: 0,
            continue_from_text: "0".to_string(),
            drift: DriftView::default(),
            prev_global_keys: [false; 4],
            dark_title_bar_set: false,
        }
    }
}

impl View {
    /// Stage the next CONT's splice, UI-side only.
    pub(crate) fn stage_continue_frame(&mut self, frame: u32) {
        self.continue_from_frame = frame;
        self.continue_from_text = frame.to_string();
    }
}

/// The drift banner's scan: replay against recording, X/Z, as positions
/// arrive. A diagnostic, separate from the transport cycle's watcher.
#[derive(Default)]
pub(crate) struct DriftView {
    pub(crate) tracker: DriftTracker,
    last_logged_level: u8, // 0=none, 1=any, 2=>=1.0, 3=>=5.0
}

impl DriftView {
    /// Scan up to the take's edit limit, and log the first drift past each
    /// level and the CONT splice verdict.
    pub(crate) fn scan(&mut self, state: &TasSharedState, trajectory_limit: u32, log: &mut UiLog) {
        let previous = (
            self.tracker.max_dx,
            self.tracker.max_dz,
            self.tracker.splice_tick,
        );
        let (count, reset) = self.tracker.scan_up_to(state, trajectory_limit);
        if reset {
            self.last_logged_level = 0;
        }

        let max_drift = self.tracker.max_drift();
        let new_level = if max_drift >= 5.0 {
            3
        } else if max_drift >= 1.0 {
            2
        } else if max_drift > 0.0 {
            1
        } else {
            0
        };
        if state.mode == TasMode::Play as u32 && new_level > self.last_logged_level {
            log.push(format!(
                "{} first at tick {}: max X={:.9} Z={:.9} (was X={:.9} Z={:.9})",
                if state.continue_from_frame != 0 {
                    "CONT prefix difference"
                } else {
                    "DRIFT"
                },
                self.tracker.first_drift_tick.unwrap_or(count),
                self.tracker.max_dx,
                self.tracker.max_dz,
                if reset { 0.0 } else { previous.0 },
                if reset { 0.0 } else { previous.1 },
            ));
            self.last_logged_level = new_level;
        }
        if (state.mode == TasMode::Play as u32 || state.mode == TasMode::Rec as u32)
            && (reset || previous.2.is_none())
        {
            if let Some(tick) = self.tracker.splice_tick {
                log.push(format!(
                    "CONT splice {tick}: X={:.9} Z={:.9}",
                    self.tracker.splice_dx, self.tracker.splice_dz
                ));
            }
        }
    }
}

/// The window's panels, drawn from the units. Actions they return are
/// dispatched by the app as they come, as the shortcuts' are.
impl TasApp {
    pub(crate) fn show_game_exit_banner(&mut self, ctx: &egui::Context) {
        let Some(banner) = self.conn.game_exit_banner.clone() else {
            return;
        };
        egui::TopBottomPanel::top("game_exit_banner").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), banner);
                if ui.button("Dismiss").clicked() {
                    self.conn.game_exit_banner = None;
                }
            });
        });
    }

    pub(crate) fn show_menu_bar(&mut self, ctx: &egui::Context) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        let level = self.conn.level_for_save().map(str::to_string);
                        if let Some(shared) = self.conn.shared.as_ref() {
                            if let Some(path) = recording::save_dialog(
                                self.host.as_ref(),
                                shared.state(),
                                &mut self.log_lines,
                                level.as_deref(),
                                self.take.stamps(),
                            ) {
                                self.history.list.push_save_marker(
                                    shared.state(),
                                    &path,
                                    self.take.stamps().cloned(),
                                );
                            }
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        self.load_recording_flow();
                    }
                    ui.separator();
                    ui.label(
                        egui::RichText::new("Settings")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.horizontal(|ui| {
                        ui.label("CONT catch-up");
                        ui.add(
                            egui::DragValue::new(&mut self.settings.cont_catchup_speed)
                                .range(1.0..=384.0)
                                .prefix("\u{00D7}")
                                .speed(1.0),
                        )
                        .on_hover_text(
                            "CONT catch-up replay speed. ~256× ≈ the game's physics \
                             ceiling (~80× effective); the F5 restart is separate and \
                             unaffected.",
                        );
                    });
                });
                ui.menu_button("View", |ui| {
                    // Everyday toggles, grouped under "Panels". The
                    // hotkey labels are advisory only — actual handling
                    // is in `handle_shortcuts`.
                    ui.label(
                        egui::RichText::new("Panels")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.settings.show_history, "History");
                        ui.weak("Ctrl+H");
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.settings.show_log, "Log");
                        ui.weak("Ctrl+L");
                    });
                    ui.separator();
                    // Debug section — rarely touched diagnostic toggles.
                    ui.label(
                        egui::RichText::new("Debug")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.checkbox(&mut self.settings.show_pico_panel, "Pico HID");
                    ui.checkbox(&mut self.settings.show_config, "Debug Config");
                });
            });
        });
    }

    pub(crate) fn show_log_panel(&mut self, ctx: &egui::Context) {
        // Bottom log panel (hidden by default, toggle via View menu).
        // Declared FIRST so it sits at the very bottom of the window;
        // egui stacks subsequent bottom panels above it.
        if self.settings.show_log {
            egui::TopBottomPanel::bottom("log_panel")
                .resizable(true)
                .default_height(100.0)
                .show(ctx, |ui| {
                    log_panel::show(ui, &mut self.log_lines);
                });
        } else if let Some(last) = self
            .log_lines
            .lines()
            .iter()
            .rev()
            .find(|line| !line.contains("] [DLL"))
        {
            // Refusals and aborts are only logged: with the log hidden, its
            // latest line still shows here.
            egui::TopBottomPanel::bottom("last_log_line").show(ctx, |ui| {
                ui.add(egui::Label::new(egui::RichText::new(last).monospace().weak()).truncate());
            });
        }
    }

    /// Retry the connection and, while it is down, draw the error screen.
    /// Returns true when the rest of the frame must be skipped.
    pub(crate) fn show_connection_error(&mut self, ctx: &egui::Context) -> bool {
        // Connection error state
        // Retry every 2 s (the repaint floor below): the game is often
        // launched after tas_ui, and the mapping only exists once
        // TAS_Helper has initialized.
        if self.conn.connect_error.is_some() && self.conn.reconnect_due(self.host.now()) {
            self.try_reconnect();
        }
        if self.conn.connect_error.is_some() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("SSB Inspect");
                    ui.add_space(20.0);
                    if let Some(ref err) = self.conn.connect_error {
                        ui.colored_label(egui::Color32::from_rgb(255, 100, 100), err);
                    }
                    ui.add_space(10.0);
                    ui.label("Inject TAS_Helper.dll into Supreme.exe first.");
                    ui.add_space(10.0);
                    if ui.button("Retry Connection").clicked() {
                        self.try_reconnect();
                    }
                });
            });
            ctx.request_repaint_after(std::time::Duration::from_secs(2));
            return true;
        }
        false
    }

    pub(crate) fn show_left_panel(&mut self, ctx: &egui::Context) {
        // Left side panel: only shown if at least one sub-panel is visible
        let left_panel_visible = self.settings.show_config || self.settings.show_pico_panel;
        if left_panel_visible {
            egui::SidePanel::left("config_panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    if let Some(ref mut shared) = self.conn.shared {
                        if self.settings.show_config {
                            egui::CollapsingHeader::new("Debug Config")
                                .default_open(false)
                                .show(ui, |ui| {
                                    config::show(ui, shared.state());
                                });
                            ui.separator();
                        }
                        if self.settings.show_pico_panel {
                            pico::show_panel(ui, &mut self.pico, &mut self.log_lines);
                        }
                    }
                    // History settings — available even when disconnected.
                    if self.settings.show_config {
                        egui::CollapsingHeader::new("History")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Undo cap:");
                                    let mut cap = self.settings.history_cap as u32;
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut cap)
                                                .range(10..=2000)
                                                .speed(2.0),
                                        )
                                        .on_hover_text(
                                            "Max UNPINNED entries kept. Pinned + current \
                                             entries are always kept, so the total can exceed this.",
                                        )
                                        .changed()
                                    {
                                        self.settings.history_cap = cap.max(1) as usize;
                                        self.history.list.set_capacity(self.settings.history_cap);
                                    }
                                });
                                ui.label(
                                    egui::RichText::new("pinned + current always kept")
                                        .size(10.0)
                                        .weak(),
                                );
                            });
                    }
                });
        } // left_panel_visible
    }

    pub(crate) fn show_history_panel(
        &mut self,
        ctx: &egui::Context,
    ) -> Vec<history::HistoryAction> {
        // Right-side history panel (optional)
        let mut history_actions = Vec::new();
        let history_dir = self
            .history
            .writer
            .as_ref()
            .map(|_| history_store::default_history_dir());
        let mut open_history_dir = false;
        if self.settings.show_history {
            egui::SidePanel::right("history_panel")
                .resizable(true)
                .default_width(280.0)
                .show(ctx, |ui| {
                    // Compact header: "History" + count, autosave path in
                    // the tooltip.
                    ui.horizontal(|ui| {
                        let title = ui.label(
                            egui::RichText::new(format!("History · {}", self.history.list.len()))
                                .strong(),
                        );
                        if let Some(dir) = history_dir.as_ref() {
                            title.on_hover_text(format!("Autosave: {}", dir.display()));
                        }
                        if history_dir.is_some() {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .small_button("Open…")
                                        .on_hover_text("Open the autosave folder in Explorer")
                                        .clicked()
                                    {
                                        open_history_dir = true;
                                    }
                                },
                            );
                        }
                    });

                    // Same menu signal as the status chip (game_in_game stays
                    // 1 while paused, so also require the cycle ticking). The
                    // panel shows "In Menu" rather than "resolving…", since
                    // nothing is resolved at a menu.
                    let in_menu = !self.arming_allowed();
                    let game_flag = self
                        .conn
                        .shared
                        .as_ref()
                        .map(|s| s.state().game_in_game != 0)
                        .unwrap_or(false);
                    history_actions = history::show(ui, &self.history.list, in_menu, game_flag);
                });
        }

        if open_history_dir {
            if let Some(dir) = history_dir.as_deref() {
                match self.host.open_folder(dir) {
                    Ok(()) => {
                        self.push_log(&format!("Opened history folder: {}", dir.display()));
                    }
                    Err(err) => {
                        self.push_log(&format!("Open history folder failed: {}", err));
                    }
                }
            }
        }
        history_actions
    }

    pub(crate) fn show_central_panel(&mut self, ctx: &egui::Context) {
        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Read before the mutable borrow of `shared` below.
            let arming_allowed = self.arming_allowed();
            // Transport bar at top
            let cmds = if let Some(ref mut shared) = self.conn.shared {
                let mode = shared.state().mode_enum();
                let recorded = shared.state().recorded_count;
                // During catch-up the resume speed lives in resume_speed
                // (playback_speed is the catch-up multiplier); otherwise it's
                // the live play speed. The speed buttons highlight and edit it.
                let resume_speed = self.transport.resume_or_playback_speed();

                transport::show(
                    ui,
                    transport::TransportProps {
                        mode,
                        recorded,
                        continue_from: &mut self.view.continue_from_frame,
                        continue_from_text: &mut self.view.continue_from_text,
                        playback_speed: &mut self.transport.playback_speed,
                        cont_catchup_speed: self.settings.cont_catchup_speed,
                        history: &self.history.list,
                        state: shared.state(),
                        catchup_active: self.transport.resume_speed.is_some(),
                        resume_speed,
                        arming_allowed,
                    },
                )
            } else {
                Vec::new()
            };

            // Same dispatch as keyboard shortcuts, so the two can't diverge.
            for cmd in cmds {
                self.apply_transport_action(cmd);
            }

            // The take's track, for the timeline's game-clock origin below.
            let take_level = self
                .take
                .level(self.conn.shared.as_ref().map(|s| s.state()));
            if let Some(ref shared) = self.conn.shared {
                ui.separator();

                let state = shared.state();
                status::status_card(
                    ui,
                    state,
                    &status::StatusProps {
                        in_game: arming_allowed,
                        finished_at_tick: self.session.finish.at_tick,
                        loaded_physics: self.take.identity.physics.as_deref(),
                        loaded_rider: self.take.identity.rider.as_deref(),
                    },
                );

                ui.separator();

                status::drift_banner(ui, state, &self.view.drift.tracker);

                // The game-clock origin is the take's track, which can differ
                // from the live one. The selected history entry and the last
                // known track are fallbacks once the course unloads.
                let timeline_level = take_level
                    .as_deref()
                    .or(self
                        .history
                        .list
                        .current_index()
                        .and_then(|i| self.history.list.entries().get(i))
                        .and_then(|entry| entry.level.as_deref()))
                    .or(self.conn.last_resolved_level.as_deref());
                let open_text_script = status::timeline_header(
                    ui,
                    state,
                    timeline::game_timer_anchor(state, timeline_level),
                    self.editor.script_watch.is_some(),
                );
                let tl_outcome = timeline::show(
                    ui,
                    state,
                    timeline_level,
                    &mut self.view.timeline,
                    &mut self.view.continue_from_frame,
                    &mut self.editor.timeline,
                );
                if tl_outcome.continue_changed {
                    self.view.continue_from_text = self.view.continue_from_frame.to_string();
                }
                if let Some(events) = tl_outcome.events {
                    self.editor.queue(editor::Edit {
                        events,
                        commit: tl_outcome.commit_undo,
                        label: tl_outcome.action_label.unwrap_or_default(),
                    });
                }

                // Text-script route: poll_script_file reloads it on save.
                if open_text_script {
                    let total = state.recorded_count;
                    let events = input_script::runs_from_log(&state.input_log, total);
                    // Where the game's race clock reads zero, as on the timeline.
                    let timer = timeline::game_timer_anchor(state, timeline_level);
                    let script = input_script::events_to_script(&events, timer);
                    let path = script_watch::ScriptWatch::fresh_path();
                    match std::fs::write(&path, &script) {
                        Ok(()) => {
                            self.host.open_file(&path);
                            self.editor.watch_script(path.clone(), script);
                            self.log_lines.push(format!(
                                "[script] opened {} ({} inputs); edits reload on save",
                                path.display(),
                                events.len()
                            ));
                        }
                        Err(e) => self.log_lines.push(format!("[script] write failed: {e}")),
                    }
                }

                // DLL counters, shown only with Debug Config.
                if self.settings.show_config {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Recorded: {} | Playback: {} | BB3B10: {} | Blocks: {}",
                            state.recorded_count,
                            state.playback_pos,
                            state.bb3b10_call_count,
                            state.handler_block_count
                        ));
                    });
                }
            }
        });
    }

    pub(crate) fn schedule_repaint(&self, ctx: &egui::Context) {
        // Refresh only as fast as there is something to show. tas_ui's own
        // rendering at a flat 30 fps costs the game ~10 ms per menu frame
        // (48 ms vs 58 ms) through GPU contention. Full rate while a TAS mode
        // is armed or the engine is ticking, a lazy rate at a menu. egui still
        // repaints immediately on input; this only sets the idle floor.
        let mode_active = self
            .conn
            .shared
            .as_ref()
            .map(|s| s.mode_volatile() != TasMode::Off as u32)
            .unwrap_or(false);
        let cycle_ticking = self.conn.ticking_within(self.host.now(), 400);
        // eframe still runs update and paint while minimized, and each painted
        // frame leaks a little renderer memory on DX12/Vulkan (~15 KB/s at
        // 60 fps), so idle hard when minimized regardless of mode.
        let minimized = ctx.input(|i| i.viewport().minimized.unwrap_or(false));
        let refresh_ms = if minimized {
            500
        } else if mode_active || cycle_ticking {
            33
        } else {
            250
        };
        ctx.request_repaint_after(std::time::Duration::from_millis(refresh_ms));
    }
}
