//! Queueing a transport request and handling how its cycle ends: the app
//! side of `transport_runtime`, which touches the session, the editor and
//! the take too.

use eframe::egui;
use tas_shared::TasCommand;

use crate::recording::RecordingSessionKind;
use crate::session::StartContext;
use crate::transport_runtime::{ArmRequest, CycleEvent};
use crate::{take_buffer, TasApp};

impl TasApp {
    pub(crate) fn reset_continue_runtime_state(&mut self) {
        self.session.cancel_pending();
        self.transport.cancel(self.conn.shared.as_mut());
    }

    /// Save both trajectories over the watcher window, gate-aligned, with the
    /// physics stamps, so a divergence can be diagnosed after the fact.
    fn save_divergence_report(&mut self, reason: &str) {
        let rec_gate = self.recording_gate();
        let Some(shared) = self.conn.shared.as_ref() else {
            return;
        };
        let state = shared.state();
        let live_gate = state.gate_index;
        let window = |coords: &[[f32; 3]], from: u32| -> Vec<[f32; 3]> {
            coords
                .iter()
                .skip(from as usize)
                .take(tas_shared::align::ALIGN_VERIFY_FRAMES as usize)
                .copied()
                .collect()
        };
        let report = serde_json::json!({
            "reason": reason,
            "recording_gate": rec_gate,
            "live_gate": live_gate,
            "recorded_count": state.recorded_count,
            "playback_pos": state.playback_pos,
            "physics_mode": shared.physics_mode(),
            "rider": shared.rider(),
            "recorded_from_gate": window(&state.rec_coords[..], rec_gate),
            "replayed_from_gate": window(&state.play_coords[..], live_gate),
        });
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let dir = self.host.diagnostics_dir();
        let path = dir.join(format!("divergence-{stamp}.json"));
        let written = std::fs::create_dir_all(&dir).and_then(|_| {
            std::fs::write(
                &path,
                serde_json::to_vec_pretty(&report).unwrap_or_default(),
            )
        });
        match written {
            Ok(()) => self.push_log(&format!("Divergence report saved to {}", path.display())),
            Err(e) => self.push_log(&format!("Divergence report could not be saved: {e}")),
        }
    }

    /// The take's gate, or 0 with no game connected.
    fn recording_gate(&self) -> u32 {
        self.conn
            .shared
            .as_ref()
            .map_or(0, |shared| take_buffer::recording_gate(shared.state()))
    }

    pub(crate) fn queue_restart_then(&mut self, command: TasCommand) {
        // Ignore a second transport request while a cycle is in flight: it
        // would clobber the single-u32 command slot mid-sequence and could arm
        // the wrong thing.
        if self.transport.is_running() {
            self.log_lines.push(format!(
                "{:?} ignored: a restart/arm cycle is already in progress",
                command
            ));
            return;
        }
        // A native file dialog can block UI updates for seconds while the game
        // keeps running, so refresh the heartbeat before the menu gate.
        self.check_game_health();
        let state = self.conn.shared.as_ref().map(|s| s.state());
        let request = ArmRequest {
            command,
            arming_allowed: self.arming_allowed(),
            recorded: state.map_or(0, |s| s.recorded_count),
            continue_from_frame: self.view.continue_from_frame,
            gate: self.recording_gate(),
            input_model: self.take.input_model(state),
            trajectory_ticks: self.take.trajectory_limit(),
        };
        let mismatch = || {
            state.and_then(|state| {
                self.take
                    .replay_mismatch(state, self.history.list.live_rider())
            })
        };
        let Some(request) = request.resolve(mismatch, &mut self.log_lines) else {
            return;
        };

        let pending = match request.command {
            TasCommand::ArmContinue => Some(StartContext::Continue {
                splice: request.continue_from_frame,
            }),
            TasCommand::ArmPlay => {
                self.view.continue_from_frame = 0;
                self.view.continue_from_text = "0".to_string();
                None
            }
            _ => {
                self.editor.detach();
                Some(StartContext::Rec)
            }
        };
        // Only a finish from here on can stop the take this cycle records.
        self.session.expect(pending, self.conn.live_finish_seq());
        self.transport.start(
            request,
            self.settings.cont_catchup_speed,
            self.conn.shared.as_mut(),
            self.host.now(),
            &mut self.log_lines,
        );
    }

    /// Advance the in-flight transport cycle and handle how it ended. The
    /// transitions live in tas_shared::transport, shared with the tas_test
    /// harness.
    pub(crate) fn step_cycle(&mut self, ctx: &egui::Context) {
        let event = self.transport.step(
            self.conn.shared.as_mut(),
            self.host.as_ref(),
            &mut self.log_lines,
        );
        match event {
            None | Some(CycleEvent::Done) | Some(CycleEvent::Lost) => {}
            Some(CycleEvent::Yielded) => ctx.request_repaint(),
            Some(CycleEvent::Stalled) => {
                if let Some(shared) = self.conn.shared.as_mut() {
                    // Straight to the DLL, as the controller does for its own
                    // abort; abort_cycle covers the rest of
                    // send_action_command's cleanup.
                    shared.send_command(TasCommand::Stop);
                }
                self.abort_cycle();
            }
            Some(CycleEvent::Aborted { reason }) => {
                if reason.contains("diverged") {
                    self.save_divergence_report(&reason);
                }
                self.abort_cycle();
            }
        }
    }

    /// Tear down a cycle that will not complete: restore the speed, drop the
    /// controller and release the input block.
    fn abort_cycle(&mut self) {
        self.transport
            .restore_speed_after_abort(self.conn.shared.as_mut());
        self.reset_continue_runtime_state();
    }

    /// Emit one diagnostic line at the CONT splice: where the recording
    /// actually resumed (the requested splice tick), how many attempts it
    /// took, and whether the prefix was watched.
    pub(crate) fn log_cont_resume_summary(&mut self) {
        use tas_shared::transport::CompletedVia;
        let Some(session) = self.session.active else {
            return;
        };
        if session.kind != RecordingSessionKind::Continue {
            return; // plain REC has no resume to report
        }
        let (attempts, via) = self
            .transport
            .last_outcome
            .take()
            .unwrap_or((1, CompletedVia::Unjudged));
        let verdict = match via {
            CompletedVia::Matched => "trajectory matched",
            CompletedVia::Unjudged => "nothing to watch",
        };
        self.push_log(&format!(
            "CONT resumed at frame {} after {} attempt{} — {}",
            session.start_tick,
            attempts,
            if attempts == 1 { "" } else { "s" },
            verdict,
        ));
    }

    /// The menu gate, on the host clock: see [`GameConnection::arming_allowed`].
    ///
    /// [`GameConnection::arming_allowed`]: crate::game_connection::GameConnection::arming_allowed
    pub(crate) fn arming_allowed(&self) -> bool {
        self.conn.arming_allowed(self.host.now())
    }
}
