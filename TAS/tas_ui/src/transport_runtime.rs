//! The UI's side of the restart → arm → watch cycle: deciding whether a
//! request may arm, the playback and CONT catch-up speeds, and the in-flight
//! `tas_shared::transport::TransportController` with its stall deadline. The
//! cycle's own state machine stays in tas_shared, shared with the tas_test
//! harness.

use std::time::{Duration, Instant};

use tas_shared::transport::{Arm, ArmConfig, CompletedVia, StepOutcome, TransportController};
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::host::Host;
use crate::ui_log::UiLog;

pub(crate) const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
const PLAYBACK_SPEED_PRESETS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];

pub(crate) fn normalize_playback_speed(speed: f32) -> f32 {
    if !speed.is_finite() {
        return DEFAULT_PLAYBACK_SPEED;
    }
    for preset in PLAYBACK_SPEED_PRESETS {
        if (speed - preset).abs() < 0.01 {
            return preset;
        }
    }
    DEFAULT_PLAYBACK_SPEED
}

// Shared with the tas_test harness so both give up after the same attempts.
const ALIGN_MAX_RETRIES: u32 = tas_shared::align::ALIGN_MAX_RETRIES;

/// How long one transport cycle may run before the UI gives up on it.
///
/// Generous on purpose: this is a stall guard, not a performance bound. A
/// deep CONT catch-up, or a PLAY watched for ALIGN_VERIFY_FRAMES past the gate
/// at 1x, can take tens of seconds. It exists so a cycle that will never
/// finish cannot hold the input block forever.
const CYCLE_BUDGET: Duration = Duration::from_secs(180);

/// A REC, PLAY or CONT request with everything deciding whether it may arm.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ArmRequest {
    pub(crate) command: TasCommand,
    /// The menu gate (`GameConnection::arming_allowed`).
    pub(crate) arming_allowed: bool,
    /// Ticks in the buffer.
    pub(crate) recorded: u32,
    /// The staged CONT splice.
    pub(crate) continue_from_frame: u32,
    /// The take's gate (`take_buffer::recording_gate`).
    pub(crate) gate: u32,
    pub(crate) input_model: u32,
    pub(crate) trajectory_ticks: u32,
}

impl ArmRequest {
    pub(crate) fn arm(&self) -> Arm {
        match self.command {
            TasCommand::ArmPlay => Arm::Play,
            TasCommand::ArmContinue => Arm::Continue,
            _ => Arm::Rec,
        }
    }

    /// Display name for log lines.
    fn label(&self) -> &'static str {
        match self.arm() {
            Arm::Continue => "CONT",
            Arm::Play => "PLAY",
            Arm::Rec => "REC",
        }
    }

    /// Settle what this request arms, or refuse it (logged). `mismatch`
    /// says why the take cannot replay in the live game, when known.
    pub(crate) fn resolve(
        mut self,
        mismatch: impl FnOnce() -> Option<String>,
        log: &mut UiLog,
    ) -> Option<Self> {
        let command = self.command;
        // The menu gate at the funnel every button and hotkey passes through,
        // so a caller that forgot it still cannot arm into a stopped engine.
        if !self.arming_allowed {
            log.push(format!(
                "{:?} ignored: the game is in a menu / paused - enter a level first",
                command
            ));
            return None;
        }
        // CONT with nothing to replay (no recording, or From 0) is a fresh
        // REC. Armed as CONT it would never reach the PLAY→REC transition that
        // calls clear_catchup, leaving the app stuck at catch-up speed.
        if command == TasCommand::ArmContinue
            && (self.recorded == 0 || self.continue_from_frame == 0)
        {
            log.push("CONT from frame 0: nothing to replay, recording a fresh take");
            self.command = TasCommand::ArmRec;
        }
        // continue_from_frame == recorded_count is valid (play it all, then
        // REC). It is the normal state after a CONT, since recorded_count caps
        // at the splice frame, so pressing CONT again redoes the same prefix.
        if self.command == TasCommand::ArmContinue && self.continue_from_frame > self.recorded {
            log.push(format!(
                "CONT ignored: continue_from_frame={} > recorded_count={}",
                self.continue_from_frame, self.recorded
            ));
            return None;
        }
        // A take from another track, rider or x87 precision can never match,
        // so refuse it rather than arm and report the divergence as a TAS
        // bug. Unknown on either side cannot prove a mismatch and is allowed.
        // REC keeps whatever the player chose.
        if self.command != TasCommand::ArmRec {
            if let Some(reason) = mismatch() {
                let verb = if self.command == TasCommand::ArmPlay {
                    "PLAY"
                } else {
                    "CONT"
                };
                log.push(format!("{verb} refused: {reason}"));
                return None;
            }
        }
        match self.command {
            // The prefix input is indexed from the observed gate, so the
            // countdown length does not matter. The recording stays in
            // rec-index space at the splice, so the resumed recording is
            // byte-consistent with the loaded one.
            TasCommand::ArmContinue => {}
            // PLAY starts from tick 0 and indexes its input from the observed
            // gate, which can land on any countdown tick. The controller then
            // watches the gate-relative trajectory and restarts if it differs.
            TasCommand::ArmPlay => self.continue_from_frame = 0,
            _ => {
                self.continue_from_frame = 0;
                self.gate = 0;
            }
        }
        Some(self)
    }
}

pub(crate) enum TransportState {
    Idle,
    /// The controller is stepped from each frame and may run several
    /// transitions per frame. The shared `command` slot holds one u32, so
    /// each command waits for the DLL to acknowledge the previous one
    /// (Stop → wait OFF → Restart).
    Running {
        controller: TransportController,
        /// Wall-clock deadline for a terminal transition. The controller has
        /// no clock, so a phase that stops progressing (an F5 restart that
        /// never completes, an arm the DLL never processes) returns
        /// InProgress forever, with `cont_suppress_input` set and the
        /// keyboard swallowed. This deadline aborts such a cycle.
        deadline: Instant,
        request: ArmRequest,
    },
}

/// How a step left the cycle.
pub(crate) enum CycleEvent {
    /// Still running: draw a frame and step again.
    Yielded,
    /// Done; it released ownership and the input block.
    Done,
    /// The watcher (or the DLL) ended it.
    Aborted { reason: String },
    /// No terminal transition before the deadline.
    Stalled,
    /// The mapping went away; the cycle was dropped.
    Lost,
}

pub(crate) struct TransportRuntime {
    pub(crate) state: TransportState,
    /// The live playback speed. During a CONT catch-up it is the catch-up
    /// multiplier and `resume_speed` holds the speed to return to.
    pub(crate) playback_speed: f32,
    pub(crate) resume_speed: Option<f32>,
    /// The last cycle's result (attempts, how it completed), carried from
    /// controller `Done` to the REC-start transition, where the resume frame is
    /// known, so one summary line can report both.
    pub(crate) last_outcome: Option<(u32, CompletedVia)>,
}

impl TransportRuntime {
    pub(crate) fn new(playback_speed: f32) -> Self {
        Self {
            state: TransportState::Idle,
            playback_speed,
            resume_speed: None,
            last_outcome: None,
        }
    }

    pub(crate) fn is_running(&self) -> bool {
        matches!(self.state, TransportState::Running { .. })
    }

    /// The command the cycle in flight arms.
    pub(crate) fn running_command(&self) -> Option<TasCommand> {
        match &self.state {
            TransportState::Running { request, .. } => Some(request.command),
            TransportState::Idle => None,
        }
    }

    /// The speed to return to after a catch-up, else the live one.
    pub(crate) fn resume_or_playback_speed(&self) -> f32 {
        self.resume_speed.unwrap_or(self.playback_speed)
    }

    /// The speed to re-assert every frame. During a CONT catch-up it is the
    /// catch-up multiplier, and continuous re-assertion overrides the brief
    /// speed reset an F5 restart causes. Once the splice has fired (mode ==
    /// REC) the DLL has already dropped to the resume speed, so assert that
    /// instead of pushing the multiplier back before the mode handler runs.
    pub(crate) fn speed_to_assert(&self, mode: u32) -> f32 {
        if self.resume_speed.is_some() && mode == TasMode::Rec as u32 {
            self.resume_or_playback_speed()
        } else {
            self.playback_speed
        }
    }

    /// End a CONT catch-up: back to the speed it saved.
    pub(crate) fn clear_catchup(&mut self) {
        if let Some(saved) = self.resume_speed.take() {
            self.playback_speed = saved;
        }
    }

    /// Catch-up in flight: keep the saved resume speed and the shared
    /// cont_resume_speed in sync so the DLL drops to the speed the user just
    /// picked at the splice. playback_speed stays the catch-up multiplier
    /// until clear_catchup restores it.
    pub(crate) fn set_resume_speed(
        &mut self,
        speed: f32,
        shared: Option<&mut TasSharedMemoryClient>,
    ) {
        self.resume_speed = Some(speed);
        if let Some(shared) = shared {
            shared.state_mut().cont_resume_speed = speed;
        }
    }

    /// Start the cycle for a resolved request.
    pub(crate) fn start(
        &mut self,
        request: ArmRequest,
        catchup_speed: f32,
        shared: Option<&mut TasSharedMemoryClient>,
        now: Instant,
        log: &mut UiLog,
    ) {
        let command = request.command;
        if command == TasCommand::ArmContinue {
            if self.resume_speed.is_none() {
                self.resume_speed = Some(self.playback_speed);
            }
            self.playback_speed = catchup_speed;
        } else {
            self.clear_catchup();
            if command == TasCommand::ArmRec {
                self.playback_speed = DEFAULT_PLAYBACK_SPEED;
            }
        }

        // An aligned replay replaces recorded input in the pre-gate hold
        // window with the gate mask (see gate_alignment.hpp). Warn when that
        // changes anything, or such a recording would diverge with no
        // visible reason.
        if request.gate > 0 {
            if let Some(shared) = shared.as_deref() {
                let n = tas_shared::align::pre_gate_hold_overwrites(
                    &shared.state().input_log,
                    request.gate,
                );
                if n > 0 {
                    log.push(format!(
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
        if let Some(shared) = shared {
            let s = shared.state_mut();
            s.playback_speed = self.playback_speed;
            s.cont_resume_speed = cont_resume_speed;
        }

        // Hand the restart → arm → watch cycle to the shared controller, which
        // restarts and retries when the watcher finds a mismatch. It registers
        // this process as the TAS owner first; an aligned cycle's
        // StopForRestart then blocks live input across each OFF-mode spawn
        // countdown, until `Done` releases the ownership.
        let cfg = ArmConfig {
            arm: request.arm(),
            speed: self.playback_speed,
            continue_from_frame: request.continue_from_frame,
            gate_align_rec: request.gate,
            max_retries: if request.gate > 0 {
                ALIGN_MAX_RETRIES
            } else {
                0
            },
            input_model: request.input_model,
            trajectory_ticks: request.trajectory_ticks,
        };
        // An earlier cycle's outcome must not reach this cycle's CONT summary.
        self.last_outcome = None;
        self.state = TransportState::Running {
            controller: TransportController::new(cfg),
            deadline: now + CYCLE_BUDGET,
            request,
        };
        let resume_at = if command == TasCommand::ArmContinue {
            format!(" @frame {}", request.continue_from_frame)
        } else {
            String::new()
        };
        log.push(format!(
            "In-process restart → {:?}{} (speed {}x)",
            command, resume_at, self.playback_speed
        ));
    }

    /// Advance the in-flight controller. Wait and Reroll are handled inline
    /// without yielding to render, so the command after a settle fires on
    /// time rather than one jittered ~16ms render frame later, matching the
    /// tas_test harness. Only InProgress yields, after a short bounded spin in
    /// the phases that need tight polling. None when idle.
    pub(crate) fn step(
        &mut self,
        mut shared: Option<&mut TasSharedMemoryClient>,
        host: &dyn Host,
        log: &mut UiLog,
    ) -> Option<CycleEvent> {
        let TransportState::Running {
            controller,
            deadline,
            request,
        } = &mut self.state
        else {
            return None;
        };
        let label = request.label();
        let spin_until = host.now() + Duration::from_millis(40);
        loop {
            let Some(port) = shared.as_deref_mut() else {
                // Lost the shared-memory connection — drop the cycle.
                self.state = TransportState::Idle;
                return Some(CycleEvent::Lost);
            };
            match controller.step(port) {
                StepOutcome::InProgress => {
                    // Only InProgress can stall. Name the phase in the log: a
                    // restart that never completed and an arm the DLL never
                    // processed look identical from here otherwise.
                    if host.now() > *deadline {
                        log.push(format!(
                            "{} gave up: stalled in {} for {}s",
                            label,
                            controller.phase_name(),
                            CYCLE_BUDGET.as_secs()
                        ));
                        return Some(CycleEvent::Stalled);
                    }
                    // Spin with ~3ms polls only while the phase feeds arm
                    // timing, so restart-done detection is not quantized to
                    // vsync. The multi-second Watch phase polls once per frame:
                    // spinning through it held the UI at ~25 fps.
                    if !controller.needs_tight_polling() || host.now() >= spin_until {
                        return Some(CycleEvent::Yielded);
                    }
                    host.wait(Duration::from_millis(3));
                }
                StepOutcome::Reroll { attempt, observed } => {
                    let mismatch = observed
                        .map(|frame| frame.to_string())
                        .unwrap_or_else(|| "?".to_string());
                    log.push(format!(
                        "{} watcher reroll {}/{} (first mismatch at gate+{})",
                        label, attempt, ALIGN_MAX_RETRIES, mismatch
                    ));
                }
                StepOutcome::Done {
                    retries_used,
                    completed_via,
                } => {
                    if retries_used > 0 {
                        log.push(format!(
                            "{} watcher accepted after {} restart retr{}",
                            label,
                            retries_used,
                            if retries_used == 1 { "y" } else { "ies" }
                        ));
                    }
                    // Stash for the resume summary emitted at the REC-start
                    // splice, where the actual resume frame is known.
                    // attempts = retries + 1.
                    self.last_outcome = Some((retries_used + 1, completed_via));
                    self.state = TransportState::Idle;
                    // Release the live-input block: the rest of the replay is
                    // blocked by mode, and post-splice REC must record live
                    // input.
                    set_suppress_input(shared, false);
                    return Some(CycleEvent::Done);
                }
                StepOutcome::Aborted { reason } => {
                    log.push(format!("{} aborted: {}", label, reason));
                    return Some(CycleEvent::Aborted { reason });
                }
            }
        }
    }

    /// Cancel the in-flight cycle: release its ownership, and the live-input
    /// block a cancelled CONT must not leave set. Otherwise the controller
    /// would keep stepping and start REC/PLAY after the restart.
    pub(crate) fn cancel(&mut self, mut shared: Option<&mut TasSharedMemoryClient>) {
        if let TransportState::Running { mut controller, .. } =
            std::mem::replace(&mut self.state, TransportState::Idle)
        {
            if let Some(shared) = shared.as_deref_mut() {
                controller.release(shared);
            }
        }
        set_suppress_input(shared, false);
    }

    /// The splice fired (or REC began) before the controller reported Done:
    /// drop it, and release cont_suppress_input, since the resumed REC
    /// records live input.
    pub(crate) fn hand_over_to_rec(&mut self, mut shared: Option<&mut TasSharedMemoryClient>) {
        if let TransportState::Running { mut controller, .. } =
            std::mem::replace(&mut self.state, TransportState::Idle)
        {
            if let Some(shared) = shared.as_deref_mut() {
                controller.release(shared);
            }
            set_suppress_input(shared, false);
        }
    }

    /// A relaunch: the cycle belonged to the dead game. Only a cycle of ours
    /// may release the interlock, and only while the section is still the
    /// one it armed: a controller in another process may already own the new
    /// game's restart.
    pub(crate) fn forget_for_new_game(
        &mut self,
        shared: Option<&mut TasSharedMemoryClient>,
        mapping_is_old: bool,
    ) {
        let owned_cycle = self.is_running();
        self.state = TransportState::Idle;
        if owned_cycle && mapping_is_old {
            set_suppress_input(shared, false);
        }
    }

    /// Tear down a cycle that will not complete, and push the restored speed
    /// through, so an aborted PLAY does not leave the game fast-forwarding at
    /// the catch-up speed.
    pub(crate) fn restore_speed_after_abort(&mut self, shared: Option<&mut TasSharedMemoryClient>) {
        self.clear_catchup();
        if let Some(shared) = shared {
            shared.state_mut().playback_speed = self.playback_speed;
        }
    }
}

/// Write the CONT live-input-suppression flag into shared memory (no-op if
/// the value is unchanged or the DLL isn't connected). While set, the DLL
/// blocks the real key handler so live input can't disturb the run during
/// a cycle's OFF-mode spawn countdown. Cleared when the cycle hands over
/// and on every teardown (stop, abort, disconnect).
pub(crate) fn set_suppress_input(shared: Option<&mut TasSharedMemoryClient>, on: bool) {
    if let Some(shared) = shared {
        let want = on as u32;
        if shared.state().cont_suppress_input != want {
            shared.state_mut().cont_suppress_input = want;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// During a CONT catch-up the frame re-asserts the catch-up speed, until
    /// the DLL shows REC: it has dropped to the resume speed at the splice,
    /// and the UI must not push the multiplier back before it sees REC.
    #[test]
    fn a_catchup_asserts_its_speed_until_rec() {
        let mut transport = TransportRuntime::new(0.5);
        assert_eq!(transport.speed_to_assert(TasMode::Rec as u32), 0.5);
        transport.resume_speed = Some(0.5);
        transport.playback_speed = 64.0;
        assert_eq!(transport.speed_to_assert(TasMode::Play as u32), 64.0);
        assert_eq!(transport.speed_to_assert(TasMode::Off as u32), 64.0);
        assert_eq!(transport.speed_to_assert(TasMode::Rec as u32), 0.5);
        transport.clear_catchup();
        assert_eq!(transport.speed_to_assert(TasMode::Rec as u32), 0.5);
    }
}
