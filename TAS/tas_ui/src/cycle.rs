//! The restart → arm → watch cycle: queueing a transport request, stepping
//! the shared `TransportController`, and the CONT catch-up state around it.

use eframe::egui;
use tas_shared::TasCommand;

use crate::recording::RecordingSessionKind;
use crate::{TasApp, DEFAULT_PLAYBACK_SPEED};

// Shared with the tas_test harness so both give up after the same attempts.
const ALIGN_MAX_RETRIES: u32 = tas_shared::align::ALIGN_MAX_RETRIES;

/// How long one transport cycle may run before the UI gives up on it.
///
/// Generous on purpose: this is a stall guard, not a performance bound. A
/// deep CONT catch-up, or a PLAY watched for ALIGN_VERIFY_FRAMES past the gate
/// at 1x, can take tens of seconds. It exists so a cycle that will never
/// finish cannot hold the input block forever.
const CYCLE_BUDGET: std::time::Duration = std::time::Duration::from_secs(180);

impl TasApp {
    pub(crate) fn clear_cont_catchup(&mut self) {
        if let Some(saved) = self.resume_speed.take() {
            self.playback_speed = saved;
        }
    }

    pub(crate) fn reset_continue_runtime_state(&mut self) {
        self.pending_session_kind = None;
        self.pending_continue_start_tick = None;
        self.cycle_deadline = None;
        self.finish_baseline_armed = false;
        // Cancel any in-flight restart/arm/watch cycle; otherwise the
        // controller would keep stepping and start REC/PLAY after the restart.
        if let Some(mut cycle) = self.cycle.take() {
            if let Some(shared) = self.shared.as_mut() {
                cycle.release(shared);
            }
        }
        // A cancelled CONT must not leave live input blocked.
        self.set_cont_suppress_input(false);
    }

    /// Write the CONT live-input-suppression flag into shared memory (no-op if
    /// the value is unchanged or the DLL isn't connected). While set, the DLL
    /// blocks the real key handler so live input can't disturb the run during
    /// a cycle's OFF-mode spawn countdown. Cleared when the cycle hands over
    /// and on every teardown (stop, abort, disconnect).
    pub(crate) fn set_cont_suppress_input(&mut self, on: bool) {
        if let Some(shared) = self.shared.as_mut() {
            let want = on as u32;
            if shared.state().cont_suppress_input != want {
                shared.state_mut().cont_suppress_input = want;
            }
        }
    }

    /// The model of the take in the buffer: the loaded take's stamp, or,
    /// with nothing loaded in this session (a UI opened on a buffer the DLL
    /// kept), the model the DLL holds for it.
    pub(crate) fn buffer_input_model(&self) -> u32 {
        match self.loaded_identity.as_ref() {
            Some(identity) => identity.input_model_or_injected(),
            None => self
                .shared
                .as_ref()
                .map_or(tas_shared::TAS_INPUT_MODEL_INJECTED, |s| {
                    s.state().input_model
                }),
        }
    }

    /// The track of the take in the buffer: its stamp, else its spawn
    /// position when that is unambiguous. None = unknown.
    pub(crate) fn buffer_level(&self) -> Option<String> {
        self.loaded_level.clone().or_else(|| {
            let state = self.shared.as_ref()?.state();
            (state.recorded_count > 0)
                .then(|| crate::start_line::level_code_from_spawn(&state.rec_coords[0]))
                .flatten()
                .map(Into::into)
        })
    }

    /// Why the take in the buffer cannot replay in the live game, when both
    /// sides are known: another track, rider or x87 precision.
    pub(crate) fn replay_mismatch(&self) -> Option<String> {
        let state = self.shared.as_ref()?.state();
        let live_level =
            tas_shared::resolved_level_id(state).and_then(tas_shared::level::code_from_id);
        if let (Some(take), Some(live)) = (self.buffer_level(), live_level) {
            if take != live {
                return Some(format!(
                    "this take was recorded on {take} but the game is on {live}. Restore or \
                     load a {live} take, or go back to {take}."
                ));
            }
        }
        // Character and stance are set when the level is entered from the
        // menu, so the advice names the screen; a restart does not change them.
        if let Some(advice) = tas_shared::rider_mismatch_advice(
            self.loaded_rider.as_deref(),
            self.history.live_rider(),
        ) {
            return Some(advice);
        }
        let take_bits = self
            .loaded_identity
            .as_ref()
            .and_then(|i| i.fpu_control_word)
            .map(tas_shared::fpu_precision_bits)
            .filter(|&bits| bits != 0);
        let live_bits =
            Some(tas_shared::fpu_precision_bits(state.fpu_control_word)).filter(|&bits| bits != 0);
        if let (Some(take), Some(live)) = (take_bits, live_bits) {
            if take != live {
                return Some(format!(
                    "this take was recorded at {take}-bit x87 precision but the game runs at \
                     {live}-bit: the physics round differently. Pick the take's renderer in \
                     Display_Config and relaunch."
                ));
            }
        }
        None
    }

    /// At a CONT splice, take the replayed prefix as the new take's
    /// trajectory. Past an input edit the recorded one is stale, and the
    /// replay is what the take's input produces; before it they are equal.
    pub(crate) fn adopt_replayed_prefix(&mut self, splice: u32) {
        let rec_gate = self.recording_gate();
        let Some(shared) = self.shared.as_mut() else {
            return;
        };
        let state = shared.state_mut();
        let live_gate = state.gate_index;
        if rec_gate == 0 || live_gate == 0 {
            return;
        }
        let (dst, src) = (rec_gate as usize, live_gate as usize);
        let n = (splice.saturating_sub(rec_gate) as usize)
            .min(state.rec_coords.len().saturating_sub(dst))
            .min(state.play_coords.len().saturating_sub(src));
        state.rec_coords[dst..dst + n].copy_from_slice(&state.play_coords[src..src + n]);
    }

    /// Save both trajectories over the watcher window, gate-aligned, with the
    /// physics stamps, so a divergence can be diagnosed after the fact.
    fn save_divergence_report(&mut self, reason: &str) {
        let rec_gate = self.recording_gate();
        let Some(shared) = self.shared.as_ref() else {
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
        let dir = crate::settings::data_root_dir().join("diagnostics");
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

    /// The loaded recording's gate (its first-moving frame), or 0 if nothing
    /// is loaded or it never moves. Uses the shared `detect_first_moving` so
    /// the app and the harness align identically.
    fn recording_gate(&self) -> u32 {
        self.shared
            .as_ref()
            .and_then(|shared| {
                let state = shared.state();
                tas_shared::align::detect_first_moving(&state.rec_coords, state.recorded_count)
            })
            .unwrap_or(0)
    }

    /// Display name of the in-flight controller cycle for log lines.
    fn cycle_label(&self) -> &'static str {
        match self.cycle_arm {
            tas_shared::transport::Arm::Continue => "CONT",
            tas_shared::transport::Arm::Play => "PLAY",
            tas_shared::transport::Arm::Rec => "REC",
        }
    }

    pub(crate) fn queue_restart_then(&mut self, command: TasCommand) {
        // Ignore a second transport request while a cycle is in flight: it
        // would clobber the single-u32 command slot mid-sequence and could arm
        // the wrong thing.
        if self.cycle.is_some() {
            self.log_lines.push(format!(
                "{:?} ignored: a restart/arm cycle is already in progress",
                command
            ));
            return;
        }
        // The menu gate at the funnel every button and hotkey passes through,
        // so a caller that forgot it still cannot arm into a stopped engine.
        // A native file dialog can block UI updates for seconds while the game
        // keeps running, so refresh the heartbeat before applying it.
        self.check_game_health();
        if !self.arming_allowed() {
            self.log_lines.push(format!(
                "{:?} ignored: the game is in a menu / paused - enter a level first",
                command
            ));
            return;
        }
        // CONT with nothing to replay (no recording, or From 0) is a fresh
        // REC. Armed as CONT it would never reach the PLAY→REC transition that
        // calls clear_cont_catchup, leaving the app stuck at catch-up speed.
        let recorded = self
            .shared
            .as_ref()
            .map(|s| s.state().recorded_count)
            .unwrap_or(0);
        let command = if command == TasCommand::ArmContinue
            && (recorded == 0 || self.continue_from_frame == 0)
        {
            self.log_lines
                .push("CONT from frame 0: nothing to replay, recording a fresh take");
            TasCommand::ArmRec
        } else {
            command
        };
        if command == TasCommand::ArmContinue {
            // continue_from_frame == recorded_count is valid (play it all,
            // then REC). It is the normal state after a CONT, since
            // recorded_count caps at the splice frame, so pressing CONT again
            // redoes the same prefix.
            if self.continue_from_frame > recorded {
                self.log_lines.push(format!(
                    "CONT ignored: continue_from_frame={} > recorded_count={}",
                    self.continue_from_frame, recorded
                ));
                return;
            }
        }
        // A take from another track, rider or x87 precision can never match,
        // so refuse it rather than arm and report the divergence as a TAS
        // bug. Unknown on either side cannot prove a mismatch and is allowed.
        // REC keeps whatever the player chose.
        if command != TasCommand::ArmRec {
            if let Some(reason) = self.replay_mismatch() {
                let verb = if command == TasCommand::ArmPlay {
                    "PLAY"
                } else {
                    "CONT"
                };
                self.log_lines.push(format!("{verb} refused: {reason}"));
                return;
            }
        }
        let arm = match command {
            TasCommand::ArmPlay => tas_shared::transport::Arm::Play,
            TasCommand::ArmContinue => tas_shared::transport::Arm::Continue,
            _ => tas_shared::transport::Arm::Rec,
        };

        let mut gate_align_rec = 0;
        let mut continue_from_frame = 0;
        if command == TasCommand::ArmContinue {
            if self.resume_speed.is_none() {
                self.resume_speed = Some(self.playback_speed);
            }
            self.playback_speed = self.cont_catchup_multiplier;
            self.pending_session_kind = Some(RecordingSessionKind::Continue);
            self.pending_continue_start_tick = Some(self.continue_from_frame);
            // The prefix input is indexed from the observed gate, so the
            // countdown length does not matter. The recording stays in
            // rec-index space at the splice, so the resumed recording is
            // byte-consistent with the loaded one.
            gate_align_rec = self.recording_gate();
            continue_from_frame = self.continue_from_frame;
        } else {
            self.clear_cont_catchup();
            self.pending_session_kind = if command == TasCommand::ArmRec {
                Some(RecordingSessionKind::Rec)
            } else {
                None
            };
            self.pending_continue_start_tick = None;
            if command == TasCommand::ArmRec {
                self.detach_input_editor();
                self.playback_speed = DEFAULT_PLAYBACK_SPEED;
            }
            // PLAY starts from tick 0 and indexes its input from the observed
            // gate, which can land on any countdown tick. The controller then
            // watches the gate-relative trajectory and restarts if it differs.
            if command == TasCommand::ArmPlay {
                self.continue_from_frame = 0;
                self.continue_from_text = "0".to_string();
                gate_align_rec = self.recording_gate();
            }
        }

        // An aligned replay replaces recorded input in the pre-gate hold
        // window with the gate mask (see gate_alignment.hpp). Warn when that
        // changes anything, or such a recording would diverge with no
        // visible reason.
        if gate_align_rec > 0 {
            if let Some(shared) = self.shared.as_ref() {
                let n = tas_shared::align::pre_gate_hold_overwrites(
                    &shared.state().input_log,
                    gate_align_rec,
                );
                if n > 0 {
                    self.log_lines.push(format!(
                        "WARNING: {} recorded input frame(s) in the {} frames before the gate differ from the gate mask; alignment replays them AS the gate mask",
                        n, tas_shared::align::GATE_ALIGN_PRE_GATE_LEAD
                    ));
                }
            }
        }

        // Reflect the speed the controller will assert into the live state now
        // so the UI updates immediately (the controller re-asserts it too).
        // For CONT, also stage the resume speed so the DLL drops to it at the
        // splice itself rather than when the UI next polls.
        let cont_resume_speed = if command == TasCommand::ArmContinue {
            self.resume_speed.unwrap_or(DEFAULT_PLAYBACK_SPEED)
        } else {
            0.0 // unset: DLL leaves the speed alone (PLAY/REC don't splice)
        };
        if let Some(shared) = self.shared.as_mut() {
            let s = shared.state_mut();
            s.playback_speed = self.playback_speed;
            s.cont_resume_speed = cont_resume_speed;
        }

        // Hand the restart → arm → watch cycle to the shared controller, which
        // restarts and retries when the watcher finds a mismatch.
        let cfg = tas_shared::transport::ArmConfig {
            arm,
            speed: self.playback_speed,
            continue_from_frame,
            gate_align_rec,
            max_retries: if gate_align_rec > 0 {
                ALIGN_MAX_RETRIES
            } else {
                0
            },
            input_model: self.buffer_input_model(),
            trajectory_ticks: self
                .loaded_identity
                .as_ref()
                .map_or(u32::MAX, crate::recording::IdentityStamps::trajectory_limit),
        };
        // Only a finish from here on can stop the take this cycle records;
        // taken now, so one that comes before the UI sees REC still counts.
        self.finish_seq_seen = self.live_finish_seq();
        self.finish_baseline_armed = true;
        // The controller registers this process as the TAS owner first; an
        // aligned cycle's StopForRestart then blocks live input across each
        // OFF-mode spawn countdown, until `Done` releases the ownership.
        self.cycle = Some(tas_shared::transport::TransportController::new(cfg));
        self.cycle_deadline = Some(std::time::Instant::now() + CYCLE_BUDGET);
        self.cycle_arm = arm;
        let resume_at = if command == TasCommand::ArmContinue {
            format!(" @frame {}", continue_from_frame)
        } else {
            String::new()
        };
        self.log_lines.push(format!(
            "In-process restart → {:?}{} (speed {}x)",
            command, resume_at, self.playback_speed
        ));
    }

    /// Advance the in-flight transport controller and handle its outcome. The
    /// transitions live in tas_shared::transport, shared with the tas_test
    /// harness; logging and cleanup live here.
    pub(crate) fn step_cycle(&mut self, ctx: &egui::Context) {
        if self.cycle.is_none() {
            return;
        }
        use tas_shared::transport::StepOutcome;
        // Wait and Reroll are handled inline without yielding to render, so
        // the command after a settle fires on time rather than one jittered
        // ~16ms render frame later, matching the tas_test harness. Only
        // InProgress yields, after a short bounded spin in the phases that
        // need tight polling.
        let spin_until = std::time::Instant::now() + std::time::Duration::from_millis(40);
        loop {
            let outcome = match (self.cycle.as_mut(), self.shared.as_mut()) {
                (Some(c), Some(p)) => c.step(p),
                _ => {
                    // Lost the shared-memory connection — drop the cycle.
                    self.cycle = None;
                    return;
                }
            };
            match outcome {
                StepOutcome::InProgress => {
                    // Only InProgress can stall. Name the phase in the log: a
                    // restart that never completed and an arm the DLL never
                    // processed look identical from here otherwise.
                    if self
                        .cycle_deadline
                        .is_some_and(|d| std::time::Instant::now() > d)
                    {
                        let phase = self.cycle.as_ref().map(|c| c.phase_name()).unwrap_or("?");
                        self.push_log(&format!(
                            "{} gave up: stalled in {} for {}s",
                            self.cycle_label(),
                            phase,
                            CYCLE_BUDGET.as_secs()
                        ));
                        self.clear_cont_catchup();
                        if let Some(shared) = self.shared.as_mut() {
                            // Straight to the DLL, as the controller does for
                            // its own abort; reset_continue_runtime_state below
                            // covers the rest of send_action_command's cleanup.
                            shared.send_command(TasCommand::Stop);
                            shared.state_mut().playback_speed = self.playback_speed;
                        }
                        self.reset_continue_runtime_state();
                        self.set_cont_suppress_input(false);
                        return;
                    }
                    // Spin with ~3ms polls only while the phase feeds arm
                    // timing, so restart-done detection is not quantized to
                    // vsync. The multi-second Watch phase polls once per frame:
                    // spinning through it held the UI at ~25 fps.
                    let tight = self.cycle.as_ref().is_some_and(|c| c.needs_tight_polling());
                    if !tight || std::time::Instant::now() >= spin_until {
                        ctx.request_repaint();
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(3));
                }
                StepOutcome::Reroll { attempt, observed } => {
                    let mismatch = observed
                        .map(|frame| frame.to_string())
                        .unwrap_or_else(|| "?".to_string());
                    self.push_log(&format!(
                        "{} watcher reroll {}/{} (first mismatch at gate+{})",
                        self.cycle_label(),
                        attempt,
                        ALIGN_MAX_RETRIES,
                        mismatch
                    ));
                }
                StepOutcome::Done {
                    retries_used,
                    completed_via,
                } => {
                    if retries_used > 0 {
                        self.push_log(&format!(
                            "{} watcher accepted after {} restart retr{}",
                            self.cycle_label(),
                            retries_used,
                            if retries_used == 1 { "y" } else { "ies" }
                        ));
                    }
                    // Stash for the resume summary emitted at the REC-start splice,
                    // where the actual resume frame is known. attempts = retries + 1.
                    self.cont_last_outcome = Some((retries_used + 1, completed_via));
                    self.cycle = None;
                    // Release the live-input block: the rest of the replay is
                    // blocked by mode, and post-splice REC must record live
                    // input.
                    self.set_cont_suppress_input(false);
                    return;
                }
                StepOutcome::Aborted { reason } => {
                    self.push_log(&format!("{} aborted: {}", self.cycle_label(), reason));
                    if reason.contains("diverged") {
                        self.save_divergence_report(&reason);
                    }
                    self.clear_cont_catchup();
                    // Push the restored speed through: an aborted PLAY must not
                    // leave the game fast-forwarding at the catch-up speed.
                    if let Some(shared) = self.shared.as_mut() {
                        shared.state_mut().playback_speed = self.playback_speed;
                    }
                    // also clears cycle
                    self.reset_continue_runtime_state();
                    self.set_cont_suppress_input(false);
                    return;
                }
            }
        }
    }

    /// Emit one diagnostic line at the CONT splice: where the recording
    /// actually resumed (the requested splice tick), how many attempts it
    /// took, and whether the prefix was watched.
    pub(crate) fn log_cont_resume_summary(&mut self) {
        use tas_shared::transport::CompletedVia;
        let Some(session) = self.active_recording_session else {
            return;
        };
        if session.kind != RecordingSessionKind::Continue {
            return; // plain REC has no resume to report
        }
        let (attempts, via) = self
            .cont_last_outcome
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

    /// The menu gate: connected, in a race, and that race's cycle ticked
    /// within the last 400 ms. Arming drives an F5 restart, and in the pause
    /// menu or a dialog the race is still launched (`game_in_game` = 1) but the
    /// cycle is frozen and the game won't take F5, so an arm would leave a
    /// half-armed cycle.
    ///
    /// The button greying, the F9/F10/F12 refusals and `queue_restart_then`
    /// all read it here so they cannot drift apart.
    pub(crate) fn arming_allowed(&self) -> bool {
        self.shared.as_ref().is_some_and(|shared| {
            shared.state().game_in_game != 0
                && self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400)
        })
    }
}
