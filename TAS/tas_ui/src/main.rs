mod panels;
mod pico;
mod recording;

use eframe::egui;
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use panels::{config, drift, log_panel, timeline, trajectory, transport};
use pico::PicoState;
use recording::UndoRing;

struct TasApp {
    shared: Option<TasSharedMemoryClient>,
    connect_error: Option<String>,

    // UI state
    show_pico_panel: bool,
    pico: PicoState,
    undo_ring: UndoRing,
    log_lines: Vec<String>,
    timeline_zoom: f32,
    timeline_scroll: f32,
    continue_from_frame: u32,
    playback_speed: f32,
    step_mode: bool,
    show_trajectory: bool,
    log_read_cursor: u32,
}

impl TasApp {
    fn new() -> Self {
        let (shared, connect_error) = match TasSharedMemoryClient::open() {
            Ok(s) => (Some(s), None),
            Err(e) => (None, Some(e)),
        };

        let mut app = Self {
            shared,
            connect_error,
            show_pico_panel: false,
            pico: PicoState::new(),
            undo_ring: UndoRing::new(5),
            log_lines: Vec::new(),
            timeline_zoom: 1.0,
            timeline_scroll: 0.0,
            continue_from_frame: 0,
            playback_speed: 1.0,
            step_mode: false,
            show_trajectory: false,
            log_read_cursor: 0,
        };

        // Auto-detect Pico on startup
        let detect_logs = app.pico.auto_detect();
        for msg in detect_logs {
            let ts = chrono::Local::now().format("%H:%M:%S");
            app.log_lines.push(format!("[{}] {}", ts, msg));
        }
        if app.pico.auto_detected {
            app.show_pico_panel = true;
        }

        app
    }

    fn try_reconnect(&mut self) {
        match TasSharedMemoryClient::open() {
            Ok(s) => {
                self.shared = Some(s);
                self.connect_error = None;
                self.push_log("Connected to TAS_Helper.dll shared memory");
            }
            Err(e) => self.connect_error = Some(e),
        }
    }

    fn push_log(&mut self, msg: &str) {
        let ts = chrono::Local::now().format("%H:%M:%S");
        self.log_lines.push(format!("[{}] {}", ts, msg));
        if self.log_lines.len() > 500 {
            self.log_lines.drain(..100);
        }
    }

    /// Process keyboard shortcuts. Returns actions to execute.
    fn handle_shortcuts(&mut self, ctx: &egui::Context) -> Vec<transport::Action> {
        let mut actions = Vec::new();

        // Don't consume shortcuts when a text field has focus
        let any_text_focus = ctx.memory(|m| m.focused().is_some());

        ctx.input(|input| {
            let ctrl = input.modifiers.ctrl || input.modifiers.mac_cmd;

            // F5: Restart game (Pico F5)
            if input.key_pressed(egui::Key::F5) {
                if self.pico.connected {
                    match self.pico.send_f5() {
                        Ok(()) => actions.push(transport::Action::Log("Shortcut: F5 restart".into())),
                        Err(e) => actions.push(transport::Action::Log(format!("F5 error: {}", e))),
                    }
                } else {
                    actions.push(transport::Action::Log("F5: Pico not connected".into()));
                }
            }

            // F6: Arm REC (only when OFF)
            if input.key_pressed(egui::Key::F6) {
                actions.push(transport::Action::AutoSave);
                actions.push(transport::Action::Send(TasCommand::ArmRec));
                actions.push(transport::Action::Log("Shortcut: F6 REC".into()));
            }

            // F7: Arm PLAY (only when OFF with data)
            if input.key_pressed(egui::Key::F7) {
                actions.push(transport::Action::Send(TasCommand::ArmPlay));
                actions.push(transport::Action::Log("Shortcut: F7 PLAY".into()));
            }

            // F8: STOP
            if input.key_pressed(egui::Key::F8) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: F8 STOP".into()));
            }

            // F9: Continue record
            if input.key_pressed(egui::Key::F9) {
                actions.push(transport::Action::AutoSave);
                actions.push(transport::Action::Send(TasCommand::ArmContinue));
                actions.push(transport::Action::Log("Shortcut: F9 CONT".into()));
            }

            // Skip remaining shortcuts if text input has focus
            if any_text_focus {
                return;
            }

            // Space: STOP (toggle off)
            if input.key_pressed(egui::Key::Space) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: Space STOP".into()));
            }

            // Period: Step forward one frame
            if input.key_pressed(egui::Key::Period) {
                actions.push(transport::Action::StepOne);
            }

            // Ctrl+Z: Undo
            if ctrl && input.key_pressed(egui::Key::Z) {
                actions.push(transport::Action::Undo);
                actions.push(transport::Action::Log("Shortcut: Ctrl+Z Undo".into()));
            }

            // Ctrl+S: Save recording
            if ctrl && input.key_pressed(egui::Key::S) {
                actions.push(transport::Action::Log("Shortcut: Ctrl+S Save".into()));
            }

            // Ctrl+O: Open recording
            if ctrl && input.key_pressed(egui::Key::O) {
                actions.push(transport::Action::Log("Shortcut: Ctrl+O Open".into()));
            }

            // Plus/Equals: Zoom in timeline
            if input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals) {
                // handled below outside closure
            }

            // Minus: Zoom out timeline
            if input.key_pressed(egui::Key::Minus) {
                // handled below outside closure
            }
        });

        // Handle zoom and file operations outside the input closure to avoid borrow issues
        let (zoom_in, zoom_out, save, open) = ctx.input(|input| {
            let ctrl = input.modifiers.ctrl || input.modifiers.mac_cmd;
            (
                !any_text_focus && (input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals)),
                !any_text_focus && input.key_pressed(egui::Key::Minus),
                ctrl && input.key_pressed(egui::Key::S),
                ctrl && input.key_pressed(egui::Key::O),
            )
        });

        if zoom_in {
            self.timeline_zoom = (self.timeline_zoom * 1.25).min(10.0);
        }
        if zoom_out {
            self.timeline_zoom = (self.timeline_zoom / 1.25).max(0.1);
        }
        if save {
            if let Some(ref shared) = self.shared {
                recording::save_dialog(shared.state(), &mut self.log_lines);
            }
        }
        if open {
            if let Some(ref mut shared) = self.shared {
                recording::load_dialog(shared.state_mut(), &mut self.log_lines);
            }
        }

        actions
    }
}

impl eframe::App for TasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process keyboard shortcuts first
        let shortcut_actions = self.handle_shortcuts(ctx);

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            recording::save_dialog(shared.state(), &mut self.log_lines);
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        if let Some(ref mut shared) = self.shared {
                            recording::load_dialog(shared.state_mut(), &mut self.log_lines);
                        }
                    }
                    ui.separator();
                    if ui.button("Dump Diagnostics...").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            recording::dump_diagnostics(shared.state(), &mut self.log_lines);
                        }
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_pico_panel, "Pico HID Panel");
                    ui.checkbox(&mut self.show_trajectory, "Trajectory Viewer");
                });
            });
        });

        // Bottom log panel
        egui::TopBottomPanel::bottom("log_panel")
            .resizable(true)
            .default_height(100.0)
            .show(ctx, |ui| {
                log_panel::show(ui, &mut self.log_lines);
            });

        // Connection error state
        if self.connect_error.is_some() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("SSB Inspect");
                    ui.add_space(20.0);
                    if let Some(ref err) = self.connect_error {
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
            return;
        }

        // Read current state snapshot for UI
        let connected = self.shared.is_some();
        if !connected {
            return;
        }

        // Left side panel: config (collapsed by default) + pico
        egui::SidePanel::left("config_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                if let Some(ref mut shared) = self.shared {
                    egui::CollapsingHeader::new("Debug Config")
                        .default_open(false)
                        .show(ui, |ui| {
                            config::show(ui, shared.state_mut());
                        });
                    ui.separator();
                    if self.show_pico_panel {
                        pico::show_panel(ui, &mut self.pico, &mut self.log_lines);
                    }
                }
            });

        // Apply shortcut actions to shared state
        if let Some(ref mut shared) = self.shared {
            for cmd in shortcut_actions {
                let ts = chrono::Local::now().format("%H:%M:%S");
                match cmd {
                    transport::Action::Send(c) => {
                        shared.send_command(c);
                        self.log_lines
                            .push(format!("[{}] Sent: {:?}", ts, c));
                    }
                    transport::Action::AutoSave => {
                        self.undo_ring.push(shared.state());
                    }
                    transport::Action::Undo => {
                        if let Some(snap) = self.undo_ring.pop() {
                            snap.restore_to(shared.state_mut());
                            self.log_lines
                                .push(format!("[{}] Undo: restored previous recording", ts));
                        }
                    }
                    transport::Action::SetContinueFrame(frame) => {
                        shared.state_mut().continue_from_frame = frame;
                    }
                    transport::Action::StepOne => {
                        self.log_lines
                            .push(format!("[{}] Step one frame (requires DLL support)", ts));
                    }
                    transport::Action::Log(msg) => {
                        self.log_lines.push(format!("[{}] {}", ts, msg));
                    }
                }
            }
        }

        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Transport bar at top
            if let Some(ref mut shared) = self.shared {
                let mode = shared.state().mode_enum();
                let recorded = shared.state().recorded_count;

                let cmds = transport::show(
                    ui,
                    mode,
                    recorded,
                    &mut self.continue_from_frame,
                    &mut self.playback_speed,
                    &mut self.step_mode,
                    &self.undo_ring,
                    shared.state(),
                );

                // Process actions - use log_lines directly to avoid borrow conflicts
                for cmd in cmds {
                    let ts = chrono::Local::now().format("%H:%M:%S");
                    match cmd {
                        transport::Action::Send(c) => {
                            shared.send_command(c);
                            self.log_lines.push(format!("[{}] Sent: {:?}", ts, c));
                        }
                        transport::Action::AutoSave => {
                            self.undo_ring.push(shared.state());
                            self.log_lines
                                .push(format!("[{}] Auto-saved to undo ring", ts));
                        }
                        transport::Action::Undo => {
                            if let Some(snap) = self.undo_ring.pop() {
                                snap.restore_to(shared.state_mut());
                                self.log_lines.push(format!(
                                    "[{}] Undo: restored previous recording",
                                    ts
                                ));
                            }
                        }
                        transport::Action::SetContinueFrame(frame) => {
                            shared.state_mut().continue_from_frame = frame;
                        }
                        transport::Action::StepOne => {
                            self.log_lines.push(format!(
                                "[{}] Step one frame (requires DLL support)",
                                ts
                            ));
                        }
                        transport::Action::Log(msg) => {
                            self.log_lines.push(format!("[{}] {}", ts, msg));
                        }
                    }
                }

                // Sync playback_speed to shared state for Cave 5
                shared.state_mut().playback_speed = self.playback_speed;

                ui.separator();

                // Status line
                let state = shared.state();
                ui.horizontal(|ui| {
                    let mode_color = match state.mode_enum() {
                        TasMode::Off => egui::Color32::GRAY,
                        TasMode::Rec => egui::Color32::from_rgb(255, 80, 80),
                        TasMode::Play => egui::Color32::from_rgb(80, 200, 80),
                    };
                    ui.colored_label(
                        mode_color,
                        egui::RichText::new(state.mode_str()).strong(),
                    );
                    ui.separator();
                    ui.label(format!("Frame: {}", state.frame_count));
                    ui.separator();
                    ui.label(format!(
                        "Pos: ({:.1}, {:.1}, {:.1})",
                        state.player_x, state.player_y, state.player_z
                    ));
                    ui.separator();
                    hook_status_dot(ui, "C2", state.cave2_hooked);
                    hook_status_dot(ui, "C1C", state.cave1c_hooked);
                    hook_status_dot(ui, "C1D", state.cave1d_hooked);
                    hook_status_dot(ui, "C5", state.cave5_hooked);
                });

                ui.separator();

                // Two-column layout: timeline left, drift right
                let avail = ui.available_size();
                ui.horizontal(|ui| {
                    // Input timeline (takes ~60% width)
                    let timeline_width = (avail.x * 0.6).max(200.0);
                    ui.vertical(|ui| {
                        ui.set_width(timeline_width);
                        ui.label(egui::RichText::new("Input Timeline").strong());
                        timeline::show(
                            ui,
                            state,
                            &mut self.timeline_zoom,
                            &mut self.timeline_scroll,
                        );
                    });

                    ui.separator();

                    // Right panel: drift monitor or trajectory viewer
                    ui.vertical(|ui| {
                        if self.show_trajectory {
                            ui.label(egui::RichText::new("Trajectory (X-Z)").strong());
                            trajectory::show(ui, state);
                        } else {
                            ui.label(egui::RichText::new("Drift Monitor").strong());
                            drift::show(ui, state);
                        }
                    });
                });

                // Telemetry + Diagnostics footer
                ui.separator();
                ui.horizontal(|ui| {
                    let spd = (state.speed as f64).sqrt();
                    ui.label(format!(
                        "Speed: {:.2} | Vel: ({:.2}, {:.2}, {:.2}) | Tick: {}",
                        spd,
                        state.velocity_x,
                        state.velocity_y,
                        state.velocity_z,
                        state.tick_count,
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Recorded: {} | Playback: {} | BB3B10: {} | Blocks: {} | Events: {}",
                        state.recorded_count,
                        state.playback_pos,
                        state.bb3b10_call_count,
                        state.handler_block_count,
                        state.event_count
                    ));
                });

                ui.horizontal(|ui| {
                    let dx = state.max_drift_x;
                    let dz = state.max_drift_z;
                    let drift_color = if dx == 0.0 && dz == 0.0 {
                        egui::Color32::from_rgb(80, 200, 80)
                    } else if dx.abs() < 1.0 && dz.abs() < 1.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::from_rgb(255, 80, 80)
                    };
                    ui.colored_label(
                        drift_color,
                        format!("Max Drift: X={:.9} Z={:.9}", dx, dz),
                    );
                });
            }
        });

        // Poll DLL log ring buffer
        if let Some(ref shared) = self.shared {
            use tas_shared::TasLogSeverity;
            let (entries, new_cursor) = shared.state().read_log_entries(self.log_read_cursor);
            self.log_read_cursor = new_cursor;
            for (_, severity, text) in entries {
                let prefix = match severity {
                    TasLogSeverity::Debug => "[DLL:DBG]",
                    TasLogSeverity::Info => "[DLL]",
                    TasLogSeverity::Warn => "[DLL:WARN]",
                    TasLogSeverity::Error => "[DLL:ERR]",
                };
                let ts = chrono::Local::now().format("%H:%M:%S");
                self.log_lines
                    .push(format!("[{}] {} {}", ts, prefix, text));
            }
        }

        // Auto-refresh at ~30fps
        ctx.request_repaint_after(std::time::Duration::from_millis(33));
    }
}

fn hook_status_dot(ui: &mut egui::Ui, name: &str, hooked: u32) {
    let (label, color) = if hooked == 1 {
        (format!("[{}]", name), egui::Color32::from_rgb(80, 200, 80))
    } else {
        (format!("({})", name), egui::Color32::from_rgb(255, 80, 80))
    };
    ui.colored_label(color, label);
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SSB Inspect",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(TasApp::new()))
        }),
    )
}
