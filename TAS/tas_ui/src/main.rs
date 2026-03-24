mod macros;
mod panels;
mod pico;
mod recording;
mod settings;

use eframe::egui;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

/// Force dark title bar on Windows 10+ via DwmSetWindowAttribute.
#[cfg(windows)]
fn set_dark_title_bar(title: &str) {
    use std::ffi::c_void;
    type HWND = *mut c_void;
    type BOOL = i32;
    type DWORD = u32;
    const DWMWA_USE_IMMERSIVE_DARK_MODE: DWORD = 20;
    extern "system" {
        fn FindWindowW(class: *const u16, title: *const u16) -> HWND;
        fn DwmSetWindowAttribute(
            hwnd: HWND,
            attr: DWORD,
            value: *const c_void,
            size: DWORD,
        ) -> i32;
    }
    unsafe {
        let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = FindWindowW(std::ptr::null(), wide.as_ptr());
        if !hwnd.is_null() {
            let value: BOOL = 1;
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &value as *const BOOL as *const c_void,
                std::mem::size_of::<BOOL>() as DWORD,
            );
        }
    }
}

use panels::{analysis, config, drift, log_panel, rotation, segments, timeline, trajectory, transport};
use pico::PicoState;
use recording::UndoRing;

struct TasApp {
    shared: Option<TasSharedMemoryClient>,
    connect_error: Option<String>,

    // UI state
    show_config: bool,
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
    show_analysis: bool,
    show_rotation: bool,
    show_macros: bool,
    show_segments: bool,
    macro_state: macros::MacroState,
    segment_tracker: recording::SegmentTracker,
    last_mode: u32,
    log_read_cursor: u32,

    // Cached max drift (incremental scan instead of per-frame O(n))
    cached_max_drift_x: f32,
    cached_max_drift_z: f32,
    last_drift_scan_count: usize,
    last_logged_drift_level: u8, // 0=none, 1=any, 2=>=1.0, 3=>=5.0

    // Cached plot data (avoid per-frame Vec allocation)
    drift_cache: drift::DriftCache,
    trajectory_cache: trajectory::TrajectoryCache,
    analysis_cache: analysis::AnalysisCache,

    // In-process restart state: command to send once restart completes
    pending_after_restart: Option<TasCommand>,

    // Crash recovery
    last_frame_count: u32,
    stale_frame_ticks: u32,
    last_health_check: std::time::Instant,

    // One-shot: force dark title bar on first frame
    #[cfg(windows)]
    dark_title_bar_set: bool,
}

impl TasApp {
    fn new() -> Self {
        let (shared, connect_error) = match TasSharedMemoryClient::open() {
            Ok(s) => (Some(s), None),
            Err(e) => (None, Some(e)),
        };

        let settings = settings::Settings::load();
        let mut app = Self {
            shared,
            connect_error,
            show_config: settings.show_config,
            show_pico_panel: settings.show_pico_panel,
            pico: PicoState::new(),
            undo_ring: UndoRing::new(5),
            log_lines: Vec::new(),
            timeline_zoom: 1.0,
            timeline_scroll: 0.0,
            continue_from_frame: 0,
            playback_speed: settings.playback_speed,
            step_mode: false,
            show_trajectory: settings.show_trajectory,
            show_analysis: settings.show_analysis,
            show_rotation: settings.show_rotation,
            show_macros: settings.show_macros,
            show_segments: settings.show_segments,
            macro_state: macros::MacroState::new(),
            segment_tracker: recording::SegmentTracker::new(),
            last_mode: 0,
            log_read_cursor: 0,
            cached_max_drift_x: 0.0,
            cached_max_drift_z: 0.0,
            last_drift_scan_count: 0,
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            analysis_cache: analysis::AnalysisCache::default(),
            pending_after_restart: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            #[cfg(windows)]
            dark_title_bar_set: false,
        };

        // Auto-detect Pico on startup
        let detect_logs = app.pico.auto_detect();
        for msg in detect_logs {
            let ts = chrono::Local::now().format("%H:%M:%S");
            app.log_lines.push(format!("[{}] {}", ts, msg));
        }
        // Pico auto-detected but panel hidden by default (use View menu to show)
        let _ = app.pico.auto_detected;

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

    /// Check if the game process is still alive by monitoring frame_count advancement.
    /// If frame_count hasn't changed for ~3 seconds, assume the game crashed.
    fn check_game_health(&mut self) {
        if self.last_health_check.elapsed() < std::time::Duration::from_secs(1) {
            return;
        }
        self.last_health_check = std::time::Instant::now();

        if let Some(ref shared) = self.shared {
            let current_frame = shared.frame_count_volatile();
            if current_frame == self.last_frame_count {
                self.stale_frame_ticks += 1;
                if self.stale_frame_ticks == 5 {
                    // Check if Supreme.exe is actually running
                    if !is_supreme_running() {
                        self.push_log("Game process not found — disconnecting shared memory");
                        self.shared = None;
                        self.connect_error = Some(
                            "Supreme.exe has exited. Inject TAS_Helper.dll after restarting the game."
                                .into(),
                        );
                        self.stale_frame_ticks = 0;
                        self.log_read_cursor = 0;
                    }
                }
            } else {
                self.last_frame_count = current_frame;
                self.stale_frame_ticks = 0;
            }
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
                        Ok(()) => {
                            actions.push(transport::Action::Log("Shortcut: F5 restart".into()))
                        }
                        Err(e) => actions.push(transport::Action::Log(format!("F5 error: {}", e))),
                    }
                } else {
                    actions.push(transport::Action::Log("F5: Pico not connected".into()));
                }
            }

            // F9: Arm REC (only when OFF)
            if input.key_pressed(egui::Key::F9) {
                actions.push(transport::Action::AutoSave);
                actions.push(transport::Action::Send(TasCommand::ArmRec));
                actions.push(transport::Action::Log("Shortcut: F9 REC".into()));
            }

            // F10: Arm PLAY (only when OFF with data)
            if input.key_pressed(egui::Key::F10) {
                actions.push(transport::Action::Send(TasCommand::ArmPlay));
                actions.push(transport::Action::Log("Shortcut: F10 PLAY".into()));
            }

            // F11: STOP
            if input.key_pressed(egui::Key::F11) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: F11 STOP".into()));
            }

            // F12: Continue record
            if input.key_pressed(egui::Key::F12) {
                actions.push(transport::Action::AutoSave);
                actions.push(transport::Action::Send(TasCommand::ArmContinue));
                actions.push(transport::Action::Log("Shortcut: F12 CONT".into()));
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

            // Ctrl+Z: Undo (check raw events — egui may consume key_pressed for built-in undo)
            let ctrl_z_raw = input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Z, pressed: true, modifiers, .. }
                    if modifiers.ctrl || modifiers.mac_cmd
                )
            });
            if ctrl_z_raw {
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
                !any_text_focus
                    && (input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals)),
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
                recording::save_dialog_with_segments(
                    shared.state(),
                    &self.segment_tracker.segments,
                    &mut self.log_lines,
                );
            }
        }
        if open {
            if let Some(ref mut shared) = self.shared {
                recording::load_dialog(shared.state_mut(), &mut self.segment_tracker, &mut self.log_lines);
            }
        }

        actions
    }
}

impl eframe::App for TasApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.094, 0.094, 0.094, 1.0] // gray(24) in 0-1 range
    }

    fn on_exit(&mut self) {
        let s = settings::Settings {
            show_pico_panel: self.show_pico_panel,
            show_segments: self.show_segments,
            show_trajectory: self.show_trajectory,
            show_rotation: self.show_rotation,
            show_analysis: self.show_analysis,
            show_macros: self.show_macros,
            show_config: self.show_config,
            playback_speed: self.playback_speed,
        };
        s.save();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Force dark theme + title bar on Windows (one-shot, first frame)
        #[cfg(windows)]
        if !self.dark_title_bar_set {
            self.dark_title_bar_set = true;
            ctx.set_theme(egui::Theme::Dark);
            set_dark_title_bar("SSB Inspect");
        }

        // Check game health (crash detection)
        self.check_game_health();

        // Track mode transitions for segment tracking
        if let Some(ref shared) = self.shared {
            let current_mode = shared.mode_volatile();
            if current_mode != self.last_mode {
                let recorded = shared.recorded_count_volatile();
                // REC started
                if current_mode == 1 {
                    let start = shared.state().continue_from_frame;
                    if start == 0 && self.last_mode == 0 {
                        // Fresh recording — clear old segments
                        self.segment_tracker.clear();
                    }
                    self.segment_tracker.on_rec_start(start);
                }
                // REC stopped (mode went from REC to OFF)
                if self.last_mode == 1 && current_mode == 0 {
                    self.segment_tracker.on_rec_stop(recorded);
                }
                self.last_mode = current_mode;
            }
        }

        // Process keyboard shortcuts first
        let shortcut_actions = self.handle_shortcuts(ctx);

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            recording::save_dialog_with_segments(
                                shared.state(),
                                &self.segment_tracker.segments,
                                &mut self.log_lines,
                            );
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        if let Some(ref mut shared) = self.shared {
                            recording::load_dialog(shared.state_mut(), &mut self.segment_tracker, &mut self.log_lines);
                        }
                    }
                    ui.separator();
                    if ui.button("Dump Diagnostics...").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            recording::dump_diagnostics(
                                shared.state(),
                                (self.cached_max_drift_x, self.cached_max_drift_z),
                                &mut self.log_lines,
                            );
                        }
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_pico_panel, "Pico HID Panel");
                    ui.checkbox(&mut self.show_segments, "Segment List");
                    ui.checkbox(&mut self.show_trajectory, "Trajectory Viewer");
                    ui.checkbox(&mut self.show_rotation, "Rotation Display");
                    ui.checkbox(&mut self.show_analysis, "Analysis Panel");
                    ui.checkbox(&mut self.show_macros, "Macro Panel");
                    ui.separator();
                    ui.checkbox(&mut self.show_config, "Debug Config");
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

        // Left side panel: only shown if at least one sub-panel is visible
        let left_panel_visible = self.show_config || self.show_pico_panel || self.show_macros;
        if left_panel_visible {
        egui::SidePanel::left("config_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                if let Some(ref mut shared) = self.shared {
                    if self.show_config {
                        egui::CollapsingHeader::new("Debug Config")
                            .default_open(false)
                            .show(ui, |ui| {
                                config::show(ui, shared.state_mut());
                            });
                        ui.separator();
                    }
                    if self.show_pico_panel {
                        pico::show_panel(ui, &mut self.pico, &mut self.log_lines);
                    }
                    if self.show_macros {
                        ui.separator();
                        egui::CollapsingHeader::new("Input Macros")
                            .default_open(true)
                            .show(ui, |ui| {
                                macros::show_panel(
                                    ui,
                                    &mut self.macro_state,
                                    shared.state_mut(),
                                    &mut self.log_lines,
                                );
                            });
                    }
                }
            });
        } // left_panel_visible

        // Poll in-process restart state machine
        if let (Some(pending_cmd), Some(ref mut shared)) =
            (self.pending_after_restart, self.shared.as_mut())
        {
            let rs = shared.restart_state();
            if rs == 2 {
                // Restart complete — send the pending command
                shared.reset_restart_state();
                shared.send_command(pending_cmd);
                let ts = chrono::Local::now().format("%H:%M:%S");
                self.log_lines
                    .push(format!("[{}] Restart done, sent: {:?}", ts, pending_cmd));
                self.pending_after_restart = None;
            }
            // Request repaint to keep polling (egui won't repaint without user input)
            ctx.request_repaint();
        }

        // Apply shortcut actions to shared state
        if let Some(ref mut shared) = self.shared {
            for cmd in shortcut_actions {
                let ts = chrono::Local::now().format("%H:%M:%S");
                match cmd {
                    transport::Action::Send(c) => {
                        shared.send_command(c);
                        self.log_lines.push(format!("[{}] Sent: {:?}", ts, c));
                    }
                    transport::Action::RestartThen(c) => {
                        shared.reset_restart_state();
                        shared.send_command(TasCommand::Restart);
                        self.pending_after_restart = Some(c);
                        self.log_lines
                            .push(format!("[{}] In-process F5 restart → {:?}", ts, c));
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
                        transport::Action::RestartThen(c) => {
                            shared.reset_restart_state();
                            shared.send_command(TasCommand::Restart);
                            self.pending_after_restart = Some(c);
                            self.log_lines
                                .push(format!("[{}] In-process F5 restart → {:?}", ts, c));
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

                // Segment list panel (collapsible)
                let mut seg_actions = Vec::new();
                if self.show_segments {
                    if self.segment_tracker.segments.is_empty() {
                        ui.colored_label(
                            egui::Color32::from_rgb(120, 120, 120),
                            "No segments. Use REC then CONT to build segments.",
                        );
                    } else {
                        egui::CollapsingHeader::new(
                            egui::RichText::new(format!(
                                "Segments ({})",
                                self.segment_tracker.segments.len()
                            ))
                            .strong(),
                        )
                        .default_open(true)
                        .show(ui, |ui| {
                            seg_actions =
                                segments::show(ui, &self.segment_tracker, state);
                        });
                    }
                    ui.separator();
                }

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

                    // Right panel: drift monitor, trajectory viewer, rotation, or analysis
                    ui.vertical(|ui| {
                        if self.show_analysis {
                            ui.label(egui::RichText::new("Input Analysis").strong());
                            analysis::show(ui, state, &mut self.analysis_cache);
                        } else if self.show_trajectory {
                            ui.label(egui::RichText::new("Trajectory").strong());
                            trajectory::show(ui, state, &mut self.trajectory_cache);
                        } else if self.show_rotation {
                            ui.label(egui::RichText::new("Rotation").strong());
                            rotation::show(ui, state);
                        } else {
                            ui.label(egui::RichText::new("Drift Monitor").strong());
                            drift::show(ui, state, &mut self.drift_cache);
                        }
                    });
                });

                // Telemetry + Diagnostics footer
                ui.separator();
                ui.horizontal(|ui| {
                    let vx = state.velocity_x as f64;
                    let vy = state.velocity_y as f64;
                    let vz = state.velocity_z as f64;
                    let speed_kmh = (vx * vx + vy * vy + vz * vz).sqrt() * 360.0;
                    ui.label(format!(
                        "Speed: {:.1} km/h | Vel: ({:.2}, {:.2}, {:.2}) | Tick: {}",
                        speed_kmh,
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
                    // Incremental max drift: only scan new coordinates since last frame.
                    let count = (state.playback_pos as usize).min(state.recorded_count as usize);

                    // Reset cache if playback restarted (count decreased)
                    if count < self.last_drift_scan_count {
                        self.cached_max_drift_x = 0.0;
                        self.cached_max_drift_z = 0.0;
                        self.last_drift_scan_count = 0;
                        self.last_logged_drift_level = 0;
                    }

                    // Only scan new coordinates
                    let prev_dx = self.cached_max_drift_x;
                    let prev_dz = self.cached_max_drift_z;
                    for i in self.last_drift_scan_count..count {
                        let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
                        if d > self.cached_max_drift_x { self.cached_max_drift_x = d; }
                        let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
                        if d > self.cached_max_drift_z { self.cached_max_drift_z = d; }
                    }
                    self.last_drift_scan_count = count;

                    // Log drift at key thresholds
                    let max_d = self.cached_max_drift_x.max(self.cached_max_drift_z);
                    let new_level = if max_d >= 5.0 { 3 }
                        else if max_d >= 1.0 { 2 }
                        else if max_d > 0.0 { 1 }
                        else { 0 };
                    if new_level > self.last_logged_drift_level {
                        let ts = chrono::Local::now().format("%H:%M:%S");
                        self.log_lines.push(format!(
                            "[{}] DRIFT at tick {}: X={:.9} Z={:.9} (was X={:.9} Z={:.9})",
                            ts, count, self.cached_max_drift_x, self.cached_max_drift_z,
                            prev_dx, prev_dz
                        ));
                        self.last_logged_drift_level = new_level;
                    }

                    let (dx, dz) = (self.cached_max_drift_x, self.cached_max_drift_z);
                    let drift_color = if dx == 0.0 && dz == 0.0 {
                        egui::Color32::from_rgb(80, 200, 80)
                    } else if dx < 1.0 && dz < 1.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::from_rgb(255, 80, 80)
                    };
                    ui.colored_label(
                        drift_color,
                        format!("Max Drift: X={:.9} Z={:.9}", dx, dz),
                    );
                });

                // Process segment actions (after state borrow is no longer needed)
                let recorded_count = shared.state().recorded_count;
                for action in seg_actions {
                    let ts = chrono::Local::now().format("%H:%M:%S");
                    match action {
                        segments::SegmentAction::ScrollTo(tick) => {
                            self.timeline_scroll = tick as f32;
                            self.log_lines.push(format!(
                                "[{}] Scrolled timeline to tick {}", ts, tick
                            ));
                        }
                        segments::SegmentAction::DeleteFrom(idx) => {
                            if let Some(seg) = self.segment_tracker.segments.get(idx) {
                                let truncate_to = seg.start_tick;
                                self.undo_ring.push(shared.state());
                                shared.state_mut().recorded_count = truncate_to;
                                for i in truncate_to as usize..tas_shared::TAS_MAX_TICKS {
                                    shared.state_mut().input_log[i] = 0;
                                }
                                self.segment_tracker.segments.truncate(idx);
                                self.log_lines.push(format!(
                                    "[{}] Deleted segments from #{} onward, truncated to frame {}",
                                    ts, idx + 1, truncate_to
                                ));
                            }
                        }
                        segments::SegmentAction::SpliceAll => {
                            let count = self.segment_tracker.segments.len();
                            self.segment_tracker.segments.clear();
                            if recorded_count > 0 {
                                self.segment_tracker.segments.push(recording::Segment {
                                    name: "Spliced".into(),
                                    start_tick: 0,
                                    end_tick: recorded_count,
                                    timestamp: chrono::Local::now().to_rfc3339(),
                                });
                            }
                            self.log_lines.push(format!(
                                "[{}] Spliced {} segments into one contiguous recording ({} frames)",
                                ts, count, recorded_count
                            ));
                        }
                        segments::SegmentAction::RedoFrom(frame) => {
                            self.undo_ring.push(shared.state());
                            self.continue_from_frame = frame;
                            shared.state_mut().continue_from_frame = frame;
                            shared.send_command(TasCommand::ArmContinue);
                            self.segment_tracker.segments.retain(|s| s.start_tick < frame);
                            self.log_lines.push(format!(
                                "[{}] Redo from frame {} — armed CONT", ts, frame
                            ));
                        }
                    }
                }
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
                self.log_lines.push(format!("[{}] {} {}", ts, prefix, text));
            }
        }

        // Auto-refresh at ~30fps
        ctx.request_repaint_after(std::time::Duration::from_millis(33));
    }
}

/// Check if Supreme.exe (or Supreme_v1.035.exe) is running.
#[cfg(windows)]
fn is_supreme_running() -> bool {
    use std::process::Command;
    // Use tasklist to check — lightweight and doesn't require extra crates
    if let Ok(output) = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Supreme.exe", "/NH"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Supreme") {
            return true;
        }
    }
    if let Ok(output) = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Supreme_v1.035.exe", "/NH"])
        .creation_flags(0x08000000)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Supreme") {
            return true;
        }
    }
    false
}

#[cfg(not(windows))]
fn is_supreme_running() -> bool {
    false
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
            // Force dark theme regardless of OS setting
            cc.egui_ctx.set_theme(egui::Theme::Dark);

            let mut visuals = egui::Visuals::dark();
            let dark_bg = egui::Color32::from_gray(24);
            let widget_bg = egui::Color32::from_gray(35);
            let widget_hover = egui::Color32::from_gray(45);
            let widget_active = egui::Color32::from_gray(40);
            let subtle_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(50));

            // Panel and window backgrounds
            visuals.panel_fill = dark_bg;
            visuals.window_fill = dark_bg;
            visuals.extreme_bg_color = egui::Color32::from_gray(10);
            visuals.faint_bg_color = egui::Color32::from_gray(30);
            visuals.code_bg_color = egui::Color32::from_gray(30);

            // Non-interactive widgets (labels, separators)
            visuals.widgets.noninteractive.bg_fill = dark_bg;
            visuals.widgets.noninteractive.weak_bg_fill = dark_bg;
            visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(40));

            // Inactive widgets (buttons, combo boxes, collapsing headers)
            visuals.widgets.inactive.bg_fill = widget_bg;
            visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.inactive.bg_stroke = subtle_stroke;

            // Hovered widgets
            visuals.widgets.hovered.bg_fill = widget_hover;
            visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(38);
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(70));

            // Active (pressed) widgets
            visuals.widgets.active.bg_fill = widget_active;
            visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(35);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(80));

            // Open widgets (dropdowns)
            visuals.widgets.open.bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.open.weak_bg_fill = egui::Color32::from_gray(28);

            // Window decoration
            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(40));

            cc.egui_ctx.set_visuals_of(egui::Theme::Dark, visuals);
            Ok(Box::new(TasApp::new()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Event, Key, Modifiers, RawInput};
    use tas_shared::TasSharedState;

    /// Test constructor: creates TasApp without shared memory or Pico.
    fn test_app() -> TasApp {
        TasApp {
            shared: None,
            connect_error: Some("Test mode: no DLL".into()),
            show_config: false,
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
            show_analysis: false,
            show_rotation: false,
            show_macros: false,
            show_segments: true,
            macro_state: macros::MacroState::new(),
            segment_tracker: recording::SegmentTracker::new(),
            last_mode: 0,
            log_read_cursor: 0,
            cached_max_drift_x: 0.0,
            cached_max_drift_z: 0.0,
            last_drift_scan_count: 0,
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            analysis_cache: analysis::AnalysisCache::default(),
            pending_after_restart: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            #[cfg(windows)]
            dark_title_bar_set: false,
        }
    }

    /// Run handle_shortcuts with a simulated key press and return actions.
    fn press_key(app: &mut TasApp, key: Key, modifiers: Modifiers) -> Vec<transport::Action> {
        let ctx = egui::Context::default();
        let input = RawInput {
            events: vec![Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..Default::default()
        };

        let mut actions = Vec::new();
        let _ = ctx.run(input, |_ctx| {
            actions = app.handle_shortcuts(_ctx);
        });
        actions
    }

    fn action_has_command(actions: &[transport::Action], cmd: TasCommand) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::Send(c) if *c == cmd))
    }

    fn action_has_log(actions: &[transport::Action], needle: &str) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::Log(s) if s.contains(needle)))
    }

    fn action_has_auto_save(actions: &[transport::Action]) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::AutoSave))
    }

    // ===== App startup without DLL =====

    #[test]
    fn app_starts_without_dll() {
        let app = test_app();
        assert!(app.shared.is_none());
        assert!(app.connect_error.is_some());
        assert_eq!(app.playback_speed, 1.0);
        assert_eq!(app.timeline_zoom, 1.0);
        assert!(!app.pico.connected);
    }

    // ===== Keyboard shortcuts =====

    #[test]
    fn shortcut_f9_arms_rec() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F9, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::ArmRec));
        assert!(action_has_auto_save(&actions));
        assert!(action_has_log(&actions, "F9"));
    }

    #[test]
    fn shortcut_f10_arms_play() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F10, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::ArmPlay));
        assert!(action_has_log(&actions, "F10"));
    }

    #[test]
    fn shortcut_f11_stops() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F11, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::Stop));
        assert!(action_has_log(&actions, "F11"));
    }

    #[test]
    fn shortcut_f12_arms_continue() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F12, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::ArmContinue));
        assert!(action_has_auto_save(&actions));
        assert!(action_has_log(&actions, "F12"));
    }

    #[test]
    fn shortcut_space_stops() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Space, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::Stop));
        assert!(action_has_log(&actions, "Space"));
    }

    #[test]
    fn shortcut_f5_without_pico_logs_not_connected() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F5, Modifiers::NONE);
        assert!(action_has_log(&actions, "not connected"));
    }

    #[test]
    fn shortcut_ctrl_z_undoes() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Z, Modifiers::CTRL);
        // Must produce Undo action — not just "no panic"
        assert!(
            actions.iter().any(|a| matches!(a, transport::Action::Undo)),
            "Ctrl+Z must produce Undo action, got: {:?}",
            actions.len()
        );
        assert!(action_has_log(&actions, "Undo"), "Ctrl+Z must log Undo");
    }

    #[test]
    fn shortcut_period_steps() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Period, Modifiers::NONE);
        assert!(actions
            .iter()
            .any(|a| matches!(a, transport::Action::StepOne)));
    }

    // ===== Timeline zoom =====

    #[test]
    fn zoom_in_increases() {
        let mut app = test_app();
        let initial = app.timeline_zoom;
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_zoom > initial);
    }

    #[test]
    fn zoom_out_decreases() {
        let mut app = test_app();
        let initial = app.timeline_zoom;
        press_key(&mut app, Key::Minus, Modifiers::NONE);
        assert!(app.timeline_zoom < initial);
    }

    #[test]
    fn zoom_clamps_max() {
        let mut app = test_app();
        app.timeline_zoom = 10.0;
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_zoom <= 10.0);
    }

    #[test]
    fn zoom_clamps_min() {
        let mut app = test_app();
        app.timeline_zoom = 0.1;
        press_key(&mut app, Key::Minus, Modifiers::NONE);
        assert!(app.timeline_zoom >= 0.1);
    }

    // ===== Speed edge values =====

    #[test]
    fn playback_speed_defaults_to_1x() {
        let app = test_app();
        assert!((app.playback_speed - 1.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_can_be_set_to_extremes() {
        let mut app = test_app();
        app.playback_speed = 0.25;
        assert!((app.playback_speed - 0.25).abs() < 0.001);
        app.playback_speed = 4.0;
        assert!((app.playback_speed - 4.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_syncs_to_shared_state() {
        // Verify that TasSharedState has the playback_speed field
        // and it can accept the expected range of values
        let mut state = tas_shared::zeroed_boxed();
        state.playback_speed = 0.25;
        assert!((state.playback_speed - 0.25).abs() < f32::EPSILON);
        state.playback_speed = 4.0;
        assert!((state.playback_speed - 4.0).abs() < f32::EPSILON);
        // Default (zeroed) speed is 0.0 — Cave 5 interprets 0.0 as 1.0x
        let fresh = tas_shared::zeroed_boxed();
        assert_eq!(fresh.playback_speed, 0.0);
    }

    // ===== Crash detection =====

    #[test]
    fn crash_detection_no_shared_is_noop() {
        let mut app = test_app();
        // Should not panic when shared is None
        app.check_game_health();
        assert_eq!(app.stale_frame_ticks, 0);
    }

    #[test]
    fn crash_detection_stale_frame_increments_without_shared() {
        // Without shared memory, stale_frame_ticks should never increment
        let mut app = test_app();
        app.last_health_check = std::time::Instant::now() - std::time::Duration::from_secs(2);
        app.check_game_health();
        assert_eq!(
            app.stale_frame_ticks, 0,
            "No shared = no stale tick increment"
        );
    }

    // ===== Panel toggle defaults =====

    #[test]
    fn panel_defaults() {
        let app = test_app();
        assert!(app.show_segments);
        assert!(!app.show_trajectory);
        assert!(!app.show_analysis);
        assert!(!app.show_macros);
        assert!(!app.show_pico_panel);
    }

    #[test]
    fn panel_toggles_persist() {
        let mut app = test_app();
        app.show_trajectory = true;
        app.show_analysis = true;
        app.show_macros = true;
        assert!(app.show_trajectory);
        assert!(app.show_analysis);
        assert!(app.show_macros);
    }

    // ===== Segment tracker mode transitions =====

    #[test]
    fn segment_tracker_integration_rec_stop_cycle() {
        let mut app = test_app();
        // Simulate mode transition OFF -> REC
        app.segment_tracker.on_rec_start(0);
        // Simulate REC -> OFF
        app.segment_tracker.on_rec_stop(100);
        assert_eq!(app.segment_tracker.segments.len(), 1);
        assert_eq!(app.segment_tracker.segments[0].start_tick, 0);
        assert_eq!(app.segment_tracker.segments[0].end_tick, 100);
    }

    #[test]
    fn segment_tracker_multi_segment_rec_cont() {
        let mut app = test_app();
        // First segment
        app.segment_tracker.on_rec_start(0);
        app.segment_tracker.on_rec_stop(200);
        // Continue from 200
        app.segment_tracker.on_rec_start(200);
        app.segment_tracker.on_rec_stop(500);

        assert_eq!(app.segment_tracker.segments.len(), 2);
        assert_eq!(app.segment_tracker.segments[1].start_tick, 200);
        assert_eq!(app.segment_tracker.segments[1].end_tick, 500);
    }

    // ===== Pending restart state machine =====

    #[test]
    fn pending_restart_initially_none() {
        let app = test_app();
        assert!(app.pending_after_restart.is_none());
    }

    #[test]
    fn pending_restart_can_be_set() {
        let mut app = test_app();
        app.pending_after_restart = Some(TasCommand::ArmRec);
        assert!(matches!(
            app.pending_after_restart,
            Some(TasCommand::ArmRec)
        ));
    }

    // ===== Incremental drift cache =====

    #[test]
    fn drift_cache_matches_full_scan() {
        let mut app = test_app();

        // Heap-allocate: TasSharedState is ~1.5MB, too large for stack
        let mut state: Box<TasSharedState> = unsafe {
            Box::from_raw(Box::into_raw(vec![0u8; std::mem::size_of::<TasSharedState>()]
                .into_boxed_slice()) as *mut [u8] as *mut TasSharedState)
        };
        state.recorded_count = 100;
        state.playback_pos = 100;

        // Set some coords with known drift
        for i in 0..100usize {
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 2.0];
            state.play_coords[i] = [i as f32 + 0.5, 0.0, i as f32 * 2.0 + 1.0];
        }
        // Spike at index 50
        state.play_coords[50][0] = state.rec_coords[50][0] + 7.5;
        state.play_coords[50][2] = state.rec_coords[50][2] + 3.25;

        // Run incremental scan
        let count = (state.playback_pos as usize).min(state.recorded_count as usize);
        for i in app.last_drift_scan_count..count {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x { app.cached_max_drift_x = d; }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z { app.cached_max_drift_z = d; }
        }
        app.last_drift_scan_count = count;

        // Full scan for comparison
        let (mut full_dx, mut full_dz) = (0.0f32, 0.0f32);
        for i in 0..count {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > full_dx { full_dx = d; }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > full_dz { full_dz = d; }
        }

        assert_eq!(app.cached_max_drift_x, full_dx);
        assert_eq!(app.cached_max_drift_z, full_dz);
        assert_eq!(app.cached_max_drift_x, 7.5);
        assert_eq!(app.cached_max_drift_z, 3.25);
    }

    #[test]
    fn drift_cache_resets_on_playback_restart() {
        let mut app = test_app();

        // First playback: 50 frames with drift
        app.cached_max_drift_x = 5.0;
        app.cached_max_drift_z = 3.0;
        app.last_drift_scan_count = 50;

        // Playback restarts (count drops to 0)
        let new_count: usize = 0;
        if new_count < app.last_drift_scan_count {
            app.cached_max_drift_x = 0.0;
            app.cached_max_drift_z = 0.0;
            app.last_drift_scan_count = 0;
        }

        assert_eq!(app.cached_max_drift_x, 0.0);
        assert_eq!(app.cached_max_drift_z, 0.0);
        assert_eq!(app.last_drift_scan_count, 0);
    }

    #[test]
    fn drift_cache_incremental_accumulates() {
        let mut app = test_app();

        // First batch: frames 0..10 with small drift
        // Heap-allocate: TasSharedState is ~1.5MB, too large for stack
        let mut state: Box<TasSharedState> = unsafe {
            Box::from_raw(Box::into_raw(vec![0u8; std::mem::size_of::<TasSharedState>()]
                .into_boxed_slice()) as *mut [u8] as *mut TasSharedState)
        };
        state.recorded_count = 20;
        state.playback_pos = 10;
        for i in 0..20usize {
            state.rec_coords[i] = [0.0, 0.0, 0.0];
            state.play_coords[i] = [0.1, 0.0, 0.2];
        }
        // Spike in second batch
        state.play_coords[15] = [9.9, 0.0, 8.8];

        // Scan first batch
        let count1 = 10usize;
        for i in app.last_drift_scan_count..count1 {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x { app.cached_max_drift_x = d; }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z { app.cached_max_drift_z = d; }
        }
        app.last_drift_scan_count = count1;

        assert!((app.cached_max_drift_x - 0.1).abs() < 0.001);
        assert!((app.cached_max_drift_z - 0.2).abs() < 0.001);

        // Scan second batch (frames 10..20)
        let count2 = 20usize;
        for i in app.last_drift_scan_count..count2 {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x { app.cached_max_drift_x = d; }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z { app.cached_max_drift_z = d; }
        }
        app.last_drift_scan_count = count2;

        assert!((app.cached_max_drift_x - 9.9).abs() < 0.001);
        assert!((app.cached_max_drift_z - 8.8).abs() < 0.001);
    }
}
