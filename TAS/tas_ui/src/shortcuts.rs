//! Keyboard shortcuts: keys delivered to the tas_ui window by egui, and
//! F9–F12 polled globally while the game has focus.

use eframe::egui;
use tas_shared::TasCommand;

use crate::panels::transport;
use crate::{recording, win32, TasApp};

/// Compute press-edge transitions for the four shortcut keys (F9, F10, F11,
/// F12). Pure function so we can unit-test the edge logic without faking
/// Win32. Mutates `prev` to current so the caller's state stays in sync.
pub(crate) fn compute_global_key_edges(now: [bool; 4], prev: &mut [bool; 4]) -> [bool; 4] {
    let mut edges = [false; 4];
    for i in 0..4 {
        edges[i] = now[i] && !prev[i];
        prev[i] = now[i];
    }
    edges
}

impl TasApp {
    /// Poll the global keyboard state for F9–F12 and emit the same
    /// transport actions the in-window shortcut handler would, but only
    /// when Supreme.exe is the foreground window. Lets the user trigger
    /// REC/PLAY/STOP/CONT without alt-tabbing to tas_ui. The keys are
    /// observed, not consumed, so the game still receives them.
    ///
    /// Edge state is updated every frame regardless of focus so a key held
    /// across a focus change is not left "pressed".
    pub(crate) fn poll_global_shortcuts(&mut self) -> Vec<transport::Action> {
        let now: [bool; 4] =
            [win32::VK_F9, win32::VK_F10, win32::VK_F11, win32::VK_F12].map(win32::key_is_down);
        let [f9, f10, f11, f12] = compute_global_key_edges(now, &mut self.prev_global_keys);
        if !(f9 || f10 || f11 || f12) {
            return Vec::new();
        }

        // Only act while Supreme.exe is the foreground window. A stale cached
        // PID (game restarted without dropping shared memory) just fails to
        // match, which is harmless.
        let game_pid = self.game_pid_cached.or_else(|| {
            let resolved = win32::find_supreme_pid();
            self.game_pid_cached = resolved;
            resolved
        });
        let Some(game_pid) = game_pid else {
            return Vec::new();
        };
        if win32::foreground_window_pid() != Some(game_pid) {
            return Vec::new();
        }

        let mut actions = Vec::new();
        if f9 {
            actions.extend(self.shortcut_arm("Global F9 (in-game): REC", TasCommand::ArmRec));
        }
        if f10 {
            actions.extend(self.shortcut_arm("Global F10 (in-game): PLAY", TasCommand::ArmPlay));
        }
        if f11 {
            actions.push(transport::Action::Send(TasCommand::Stop));
            actions.push(transport::Action::Log("Global F11 (in-game): STOP".into()));
        }
        if f12 {
            actions
                .extend(self.shortcut_arm("Global F12 (in-game): CONT", TasCommand::ArmContinue));
        }
        actions
    }

    pub(crate) fn shortcut_arm(
        &mut self,
        source: &str,
        command: TasCommand,
    ) -> Vec<transport::Action> {
        let (mode, recorded) = match self.shared.as_ref() {
            Some(shared) => (shared.state().mode_enum(), shared.recorded_count_volatile()),
            None => {
                return vec![transport::Action::Log(format!(
                    "{source} ignored: not connected to the game"
                ))]
            }
        };
        if let Some(reason) = transport::arm_refusal(command, mode, recorded, self.arming_allowed())
        {
            return vec![transport::Action::Log(format!(
                "{source} ignored: {reason}"
            ))];
        }
        let mut actions = Vec::new();
        if command == TasCommand::ArmContinue {
            match transport::resolve_continue_frame(
                &mut self.continue_from_text,
                &mut self.continue_from_frame,
                recorded,
            ) {
                Ok(frame) => actions.push(transport::Action::SetContinueFrame(frame)),
                Err(reason) => {
                    return vec![transport::Action::Log(format!(
                        "{source} ignored: {reason}"
                    ))]
                }
            }
        }
        actions.push(transport::Action::RestartThen(command));
        actions.push(transport::Action::Log(source.to_string()));
        actions
    }

    /// Process keyboard shortcuts. Returns actions to execute.
    pub(crate) fn handle_shortcuts(&mut self, ctx: &egui::Context) -> Vec<transport::Action> {
        let mut actions = Vec::new();

        // Don't consume shortcuts when a text field has focus
        let any_text_focus = ctx.memory(|m| m.focused().is_some());

        // One pass over the input. Zoom and the file dialogs run after it:
        // a modal dialog must not open while the input lock is held.
        let (zoom_in, zoom_out, save, open) = ctx.input(|input| {
            let ctrl = input.modifiers.ctrl || input.modifiers.mac_cmd;
            // Ctrl+S / Ctrl+O act even while a text field has focus.
            let save = ctrl && input.key_pressed(egui::Key::S);
            let open = ctrl && input.key_pressed(egui::Key::O);

            // F5: Restart game (Pico F5)
            if input.key_pressed(egui::Key::F5) {
                if self.pico.connected {
                    win32::focus_game();
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

            // F9: Arm REC — same as clicking REC button (restart first)
            if input.key_pressed(egui::Key::F9) {
                actions.extend(self.shortcut_arm("Shortcut: F9 REC", TasCommand::ArmRec));
            }

            // F10: Arm PLAY — same as clicking PLAY button (restart first)
            if input.key_pressed(egui::Key::F10) {
                actions.extend(self.shortcut_arm("Shortcut: F10 PLAY", TasCommand::ArmPlay));
            }

            // F11: STOP
            if input.key_pressed(egui::Key::F11) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: F11 STOP".into()));
            }

            // F12: Continue record — same as clicking CONT button (restart first)
            if input.key_pressed(egui::Key::F12) {
                actions.extend(self.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue));
            }

            // Skip remaining shortcuts if text input has focus
            if any_text_focus {
                return (false, false, save, open);
            }

            // Space: STOP (toggle off)
            if input.key_pressed(egui::Key::Space) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: Space STOP".into()));
            }

            // Ctrl+Z: Undo (check raw events — egui may consume key_pressed for built-in undo)
            let ctrl_z_raw = input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Z, pressed: true, modifiers, .. }
                    if (modifiers.ctrl || modifiers.mac_cmd) && !modifiers.shift
                )
            });
            if ctrl_z_raw {
                actions.push(transport::Action::Undo);
                actions.push(transport::Action::Log("Shortcut: Ctrl+Z Undo".into()));
            }

            // Panel toggles: Ctrl+H (History), Ctrl+A (Analysis),
            // Ctrl+L (Log). Match on raw Key events for the same reason
            // as Ctrl+Z — egui may consume these as built-in shortcuts.
            for ev in input.events.iter() {
                if let egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } = ev
                {
                    if !(modifiers.ctrl || modifiers.mac_cmd) {
                        continue;
                    }
                    match key {
                        egui::Key::H => self.settings.show_history = !self.settings.show_history,
                        egui::Key::L => self.settings.show_log = !self.settings.show_log,
                        _ => {}
                    }
                }
            }

            // Ctrl+Y or Ctrl+Shift+Z: Redo
            let ctrl_redo_raw = input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Y, pressed: true, modifiers, .. }
                    if modifiers.ctrl || modifiers.mac_cmd
                )
            }) || input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Z, pressed: true, modifiers, .. }
                    if (modifiers.ctrl || modifiers.mac_cmd) && modifiers.shift
                )
            });
            if ctrl_redo_raw {
                actions.push(transport::Action::Redo);
                actions.push(transport::Action::Log("Shortcut: Redo".into()));
            }

            // Ctrl+S: Save recording
            if save {
                actions.push(transport::Action::Log("Shortcut: Ctrl+S Save".into()));
            }

            // Ctrl+O: Open recording
            if open {
                actions.push(transport::Action::Log("Shortcut: Ctrl+O Open".into()));
            }

            // Plus/Equals: zoom in the timeline; Minus: zoom out.
            (
                input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals),
                input.key_pressed(egui::Key::Minus),
                save,
                open,
            )
        });

        if zoom_in {
            self.timeline_view.zoom_center(0.8);
        }
        if zoom_out {
            self.timeline_view.zoom_center(1.25);
        }
        if save {
            // Resolve the track BEFORE the dialog: it belongs to the recording,
            // and the engine may be frozen in its own post-run dialog by now.
            let level = self.level_for_save().map(str::to_string);
            if let Some(shared) = self.shared.as_ref() {
                if let Some(path) = recording::save_dialog(
                    shared.state(),
                    &mut self.log_lines,
                    level.as_deref(),
                    self.loaded_identity.as_ref(),
                ) {
                    self.history.push_save_marker(
                        shared.state(),
                        &path,
                        self.loaded_identity.clone(),
                    );
                }
            }
        }
        if open {
            self.load_recording_flow();
        }

        actions
    }
}
