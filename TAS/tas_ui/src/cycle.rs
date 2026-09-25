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
        // Cancel any in-flight restart/arm/watch cycle; otherwise the
        // controller would keep stepping and start REC/PLAY after the restart.
        self.cycle = None;
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
        // Refuse CONT requests with nothing to splice (no recording, or frame
        // 0, which is just PLAY). They would never reach the PLAY→REC
        // transition that calls clear_cont_catchup, leaving the app stuck at
        // catch-up speed.
        if command == TasCommand::ArmContinue {
            let recorded = self
                .shared
                .as_ref()
                .map(|s| s.state().recorded_count)
                .unwrap_or(0);
            if recorded == 0 {
                self.log_lines
                    .push("CONT ignored: no recording loaded (recorded_count=0)");
                return;
            }
            if self.continue_from_frame == 0 {
                self.log_lines
                    .push("CONT ignored: continue_from_frame=0 — press PLAY instead");
                return;
            }
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
        // A take recorded as another character or stance cannot replay, and
        // no restart changes that (both are set when the level is entered
        // from the menu), so name the screen that fixes it. The arm still
        // goes ahead. REC keeps whatever the player chose.
        if command != TasCommand::ArmRec {
            if let Some(advice) = tas_shared::rider_mismatch_advice(
                self.loaded_rider.as_deref(),
                self.history.live_rider(),
            ) {
                self.log_lines.push(format!("WARNING: {}", advice));
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
        };
        self.cycle = Some(tas_shared::transport::TransportController::new(cfg));
        self.cycle_deadline = Some(std::time::Instant::now() + CYCLE_BUDGET);
        self.cycle_arm = arm;
        // Block live input for a replay cycle's restarts. Set before the
        // controller's first command so it covers each OFF-mode spawn
        // countdown, which the mode-based handler block misses; a live key
        // there would alter the state being replayed. Cleared at `Done`, after
        // which PLAY is blocked by mode and post-splice REC needs live input.
        if gate_align_rec > 0 {
            if let Some(shared) = self.shared.as_mut() {
                shared.state_mut().cont_suppress_input = 1;
            }
        }
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
