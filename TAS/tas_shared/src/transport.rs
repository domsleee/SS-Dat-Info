//! Shared restart/arm/reroll state machine driven by BOTH tas_ui (once per egui
//! frame) and the tas_test harness (in a poll loop), so the test exercises the
//! exact transport sequence the app runs — one source of truth, no drift.
//!
//! The machine is a non-blocking **stepper**: `step()` performs at most one
//! transition and never sleeps or blocks, so the egui thread can call it per
//! frame and the harness can call it in a `while !terminal { sleep; step }`
//! loop. All side effects go through the [`TransportPort`] trait, which is
//! implemented for the real shared-memory client and for a `FakePort` in tests
//! (so the serialization invariants are checked without a running game).

use crate::align::{check_aligned_trajectory, AlignVerdict};
use crate::owner::OwnerAnswer;
use crate::{TasCommand, TasMode, TAS_OWNER_ACQUIRE, TAS_OWNER_RELEASE};

/// Which session to arm after the restart completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm {
    Rec,
    Play,
    Continue,
}

impl Arm {
    fn command(self) -> TasCommand {
        match self {
            Arm::Rec => TasCommand::ArmRec,
            Arm::Play => TasCommand::ArmPlay,
            Arm::Continue => TasCommand::ArmContinue,
        }
    }
}

/// Everything the controller needs to drive one arm cycle to completion.
#[derive(Debug, Clone, Copy)]
pub struct ArmConfig {
    pub arm: Arm,
    /// Replay/catch-up speed to assert before arming.
    pub speed: f32,
    /// Splice frame for CONT (0 for REC/PLAY).
    pub continue_from_frame: u32,
    /// The recording's first-moving frame (its gate). PLAY and CONT input is
    /// indexed relative to the live gate, so the countdown length does not
    /// matter. Zero means nothing to align or judge: REC, or a recording
    /// that never moves.
    pub gate_align_rec: u32,
    /// Max restart retries for an aligned-trajectory mismatch.
    pub max_retries: u32,
    /// TAS_INPUT_MODEL_* of the loaded take (ignored by REC).
    pub input_model: u32,
    /// Leading recording ticks whose trajectory the take's current input
    /// produced; u32::MAX = all. An input edit cuts it at the first changed
    /// tick: past it the recorded trajectory is stale, so the watcher stops.
    pub trajectory_ticks: u32,
}

/// The few shared-memory operations the transport machine performs. Returns
/// raw `u32` for mode/restart_state to match the live client's accessors.
pub trait TransportPort {
    fn send_command(&mut self, cmd: TasCommand);
    /// True only after the cycle cave consumed and cleared the single command slot.
    fn command_idle(&self) -> bool;
    fn mode(&self) -> u32;
    fn restart_state(&self) -> u32;
    fn reset_restart_state(&mut self);
    fn playback_pos(&self) -> u32;
    fn play_coords(&self) -> &[[f32; 3]];
    /// The loaded recording's trajectory, which the aligned replay must
    /// reproduce bit for bit from the gate onward.
    fn rec_coords(&self) -> &[[f32; 3]];
    fn recorded_count(&self) -> u32;
    /// First live replay frame where the boarder left spawn. Zero until the
    /// gate has been observed for the current arm.
    fn gate_index(&self) -> u32;
    fn set_continue_from_frame(&mut self, frame: u32);
    fn set_gate_align_rec(&mut self, frame: u32);
    fn set_input_model(&mut self, model: u32);
    fn set_playback_speed(&mut self, speed: f32);
    /// Monotonic counter the DLL bumps once per processed arm. Used to tell
    /// this attempt's replay state from the previous one's.
    fn arm_generation(&self) -> u32;
    /// False if any coordinate capture in this session failed, which would
    /// leave a stale hole in the trajectory the watcher compares.
    fn capture_ok(&self) -> bool;
    /// Approve an aligned CONT's splice once the watcher has validated the
    /// whole prefix it will see. The DLL parks an aligned CONT at the splice
    /// until this is written, so a starved or dead controller can never let an
    /// unchecked prefix be spliced.
    fn approve_cont_splice(&mut self);
    /// Submit an ownership request (TAS_OWNER_ACQUIRE / RELEASE) for this
    /// process; returns the sequence to wait on.
    fn request_ownership(&mut self, kind: u32) -> u32;
    /// The DLL's answer to request `seq`, once it has one.
    fn ownership_answer(&self, seq: u32) -> Option<OwnerAnswer>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// Register this process as the owner before touching anything.
    Start,
    /// Waiting for the DLL to answer the ownership request.
    OwnWaitAck,
    /// Owned: send the first stop.
    Stop,
    /// Stop sent; wait until the cycle cave has consumed it and published MODE_OFF.
    StopWaitAck,
    /// Restart sent; waiting for restart_state == 2.
    RestartWaitDone,
    /// Unjudged arm sent; waiting for the DLL to bump arm_generation.
    ArmWaitAck,
    /// Replaying: watch the gate-aligned trajectory.
    Watch,
    Done,
    Aborted,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepOutcome {
    /// Mid-cycle; call `step()` again (after a short poll delay).
    InProgress,
    /// The aligned trajectory did not match, so the cycle restarts.
    /// `observed` is the first gate-relative mismatch, if there was one.
    Reroll { attempt: u32, observed: Option<u32> },
    /// Terminal success: armed (and, for PLAY/CONT, the trajectory matched).
    Done {
        retries_used: u32,
        completed_via: CompletedVia,
    },
    /// Terminal failure: gave up after exhausting retries.
    Aborted { reason: String },
}

/// How a CONT/PLAY cycle reached `Done`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletedVia {
    /// The aligned trajectory positively matched.
    Matched,
    /// Nothing to judge: REC, a recording that never moves, or a CONT
    /// spliced before the boarder starts moving.
    Unjudged,
}

/// Drives one restart→arm(→watch→reroll) cycle to a terminal outcome.
pub struct TransportController {
    cfg: ArmConfig,
    phase: Phase,
    retries_used: u32,
    completed_via: CompletedVia,
    /// The DLL's arm counter as it stood just before this attempt armed.
    /// Once it differs, the mode and position being read describe THIS
    /// attempt; until then they still describe the previous replay.
    ///
    /// ASSUMES ONE WRITER. The counter is global, not per-request, so a
    /// second process arming the same game would move it and this cycle
    /// would read that replay as its own. That is the same assumption the
    /// single-u32 command slot already makes of every caller, and the same
    /// mitigation applies - the harness calls
    /// `stop_competing_tas_ui_writer()` before it drives anything.
    arm_generation_at_arm: u32,
    /// The pending ownership request's sequence.
    owner_seq: u32,
    /// Registered as the owner and not yet released.
    owned: bool,
}

impl TransportController {
    pub fn new(cfg: ArmConfig) -> Self {
        Self {
            cfg,
            phase: Phase::Start,
            retries_used: 0,
            completed_via: CompletedVia::Unjudged,
            arm_generation_at_arm: 0,
            owner_seq: 0,
            owned: false,
        }
    }

    /// Which transition the cycle is waiting on, so a stalled cycle's log
    /// says whether the restart, the arm or the replay never arrived.
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            Phase::Start => "Start",
            Phase::OwnWaitAck => "OwnWaitAck (waiting for the DLL to register this controller)",
            Phase::Stop => "Stop",
            Phase::StopWaitAck => "StopWaitAck (waiting for the cycle cave to consume Stop)",
            Phase::RestartWaitDone => "RestartWaitDone (waiting for the F5 restart)",
            Phase::ArmWaitAck => "ArmWaitAck (waiting for the DLL to take the arm)",
            Phase::Watch => "Watch (waiting on the replay)",
            Phase::Done => "Done",
            Phase::Aborted => "Aborted",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, Phase::Done | Phase::Aborted)
    }

    /// True during the short restart handshake (Stop acknowledgement,
    /// restart-done detection, arm acknowledgement), which a driver should
    /// poll without waiting for vsync. The multi-second `Watch` replay gains
    /// nothing from that, so a UI driver can poll it once per frame.
    pub fn needs_tight_polling(&self) -> bool {
        matches!(
            self.phase,
            Phase::Start
                | Phase::OwnWaitAck
                | Phase::Stop
                | Phase::StopWaitAck
                | Phase::RestartWaitDone
                | Phase::ArmWaitAck
        )
    }

    /// Perform at most one transition. Never blocks/sleeps. A terminal
    /// outcome also releases ownership.
    pub fn step(&mut self, port: &mut impl TransportPort) -> StepOutcome {
        let outcome = self.step_phase(port);
        if self.is_terminal() {
            self.release(port);
        }
        outcome
    }

    /// Give up ownership: a cancelled cycle calls this. The DLL clears the
    /// live-input block with it. Idempotent.
    pub fn release(&mut self, port: &mut impl TransportPort) {
        if self.owned {
            self.owned = false;
            port.request_ownership(TAS_OWNER_RELEASE);
        }
    }

    fn step_phase(&mut self, port: &mut impl TransportPort) -> StepOutcome {
        let rec = TasMode::Rec as u32;
        let play = TasMode::Play as u32;
        // Re-assert the catch-up speed on EVERY step: the in-process F5
        // restart momentarily resets the game's speed, and without this the
        // post-restart countdown replays at 1x until the next phase re-sets it.
        port.set_playback_speed(self.cfg.speed);
        match self.phase {
            Phase::Start => {
                self.owner_seq = port.request_ownership(TAS_OWNER_ACQUIRE);
                self.phase = Phase::OwnWaitAck;
                // A port that answers at once goes straight on.
                self.step_phase(port)
            }
            Phase::OwnWaitAck => match port.ownership_answer(self.owner_seq) {
                None => StepOutcome::InProgress,
                Some(OwnerAnswer::Owned) => {
                    self.owned = true;
                    self.phase = Phase::Stop;
                    self.step_phase(port)
                }
                Some(answer) => {
                    self.phase = Phase::Aborted;
                    StepOutcome::Aborted {
                        reason: format!("cannot drive the TAS: {answer}"),
                    }
                }
            },
            Phase::Stop => {
                port.set_continue_from_frame(self.cfg.continue_from_frame);
                // STOP also clears this in the DLL. Clear it here as part of
                // the controller contract so even a delayed STOP cannot let
                // the previous PLAY's alignment leak into another arm.
                port.set_gate_align_rec(0);
                // Always Stop, even from OFF.
                port.send_command(self.restart_stop_command());
                self.phase = Phase::StopWaitAck;
                StepOutcome::InProgress
            }
            Phase::StopWaitAck => {
                // Restart only once Stop is consumed, or it would overwrite
                // the single command slot.
                if port.command_idle() && port.mode() == TasMode::Off as u32 {
                    port.reset_restart_state();
                    port.send_command(TasCommand::Restart);
                    self.phase = Phase::RestartWaitDone;
                }
                StepOutcome::InProgress
            }
            Phase::RestartWaitDone => {
                if port.restart_state() != 2 {
                    return StepOutcome::InProgress;
                }
                port.reset_restart_state();
                // The cycle cave reads these at ARM time — re-assert post-restart.
                port.set_continue_from_frame(self.cfg.continue_from_frame);
                port.set_gate_align_rec(self.cfg.gate_align_rec);
                port.set_input_model(self.cfg.input_model);
                // Snapshot the arm counter before the command goes out, so
                // this attempt's arm is recognised however the polling lands.
                self.arm_generation_at_arm = port.arm_generation();
                port.send_command(self.cfg.arm.command());
                // The gate fixes input indexing, but it does not fully identify
                // hidden spawn state, so the resulting trajectory is watched.
                self.phase = if self.cfg.gate_align_rec > 0 {
                    Phase::Watch
                } else {
                    Phase::ArmWaitAck
                };
                StepOutcome::InProgress
            }
            Phase::ArmWaitAck => {
                if port.arm_generation() == self.arm_generation_at_arm {
                    StepOutcome::InProgress
                } else {
                    self.finish(CompletedVia::Unjudged)
                }
            }
            Phase::Watch => {
                // Until the DLL has processed the arm, mode and position
                // still describe the previous replay. This must precede
                // every mode check, the REC one included: a stale REC would
                // otherwise read as "the splice already fired".
                if port.arm_generation() == self.arm_generation_at_arm {
                    return StepOutcome::InProgress;
                }
                let mode = port.mode();
                if mode == rec {
                    // Only an approved CONT splice enters REC, and approval
                    // finishes this cycle in the same step, so REC here means
                    // another writer is driving the game.
                    port.send_command(TasCommand::Stop);
                    self.phase = Phase::Aborted;
                    return StepOutcome::Aborted {
                        reason: "REC started before this cycle approved a splice (another writer?)"
                            .to_string(),
                    };
                }
                // Past the arm, "not in PLAY" means the replay ended or the
                // DLL refused the arm. Nothing more will arrive, so a
                // Pending verdict must become terminal rather than spin.
                let replay_ended = mode != play;
                let pos = port.playback_pos();
                if pos == 0 && replay_ended {
                    self.phase = Phase::Aborted;
                    return StepOutcome::Aborted {
                        reason: "the DLL refused the arm (nothing replayed)".to_string(),
                    };
                }
                let is_cont = self.cfg.arm == Arm::Continue;
                // The trajectory can be judged up to the CONT splice (the DLL
                // parks playback there until approved) and up to the first
                // edited tick (the recording is stale past it).
                let judge_end = if is_cont {
                    self.cfg.continue_from_frame.min(self.cfg.trajectory_ticks)
                } else {
                    self.cfg.trajectory_ticks
                };
                if judge_end <= self.cfg.gate_align_rec {
                    // Spliced or edited inside the countdown: the boarder has
                    // not moved by then, so the only thing to check is the
                    // spawn itself.
                    if pos == 0 {
                        return StepOutcome::InProgress;
                    }
                    if !port.capture_ok() {
                        return self.reroll(port, "spawn capture was incomplete".to_string(), None);
                    }
                    let spawn_matches =
                        match (port.play_coords().first(), port.rec_coords().first()) {
                            (Some(p), Some(r)) => p.map(f32::to_bits) == r.map(f32::to_bits),
                            _ => false,
                        };
                    if !spawn_matches {
                        return self.reroll(
                            port,
                            "spawn differs from the recording".to_string(),
                            Some(0),
                        );
                    }
                    if is_cont {
                        port.approve_cont_splice();
                    }
                    return self.finish(CompletedVia::Unjudged);
                }
                // Capped at judge_end: for CONT the verdict must be decidable
                // from the prefix that exists before the splice.
                let max_depth_rel = judge_end - self.cfg.gate_align_rec;
                let verdict = check_aligned_trajectory(
                    port.play_coords(),
                    port.rec_coords(),
                    port.recorded_count(),
                    pos,
                    port.gate_index(),
                    self.cfg.gate_align_rec,
                    port.capture_ok(),
                    max_depth_rel,
                );
                match verdict {
                    AlignVerdict::Pending if replay_ended => self.reroll(
                        port,
                        "aligned replay ended before its watcher completed".to_string(),
                        None,
                    ),
                    AlignVerdict::Pending => StepOutcome::InProgress,
                    AlignVerdict::Matched => {
                        // The approval is the only way a parked CONT splice
                        // fires, so a starved controller delays the splice
                        // but never lets it through unchecked.
                        if is_cont {
                            port.approve_cont_splice();
                        }
                        self.finish(CompletedVia::Matched)
                    }
                    AlignVerdict::Diverged { at } => {
                        let detail = match at {
                            Some(at) => crate::align::describe_divergence(
                                port.play_coords(),
                                port.rec_coords(),
                                port.gate_index(),
                                self.cfg.gate_align_rec,
                                at,
                            ),
                            None => "an unknown frame".to_string(),
                        };
                        self.reroll(port, format!("replay diverged at {detail}"), at)
                    }
                    AlignVerdict::CaptureFailed => self.reroll(
                        port,
                        "aligned trajectory capture was incomplete".to_string(),
                        None,
                    ),
                    AlignVerdict::MissingTrajectory => self.reroll(
                        port,
                        "recording has no gate-relative trajectory to watch".to_string(),
                        None,
                    ),
                }
            }
            Phase::Done => StepOutcome::Done {
                retries_used: self.retries_used,
                completed_via: self.completed_via,
            },
            Phase::Aborted => StepOutcome::Aborted {
                reason: "transport cycle aborted".to_string(),
            },
        }
    }

    /// Record how the cycle completed and emit the terminal `Done`.
    fn finish(&mut self, via: CompletedVia) -> StepOutcome {
        self.completed_via = via;
        self.phase = Phase::Done;
        StepOutcome::Done {
            retries_used: self.retries_used,
            completed_via: via,
        }
    }

    fn restart_stop_command(&self) -> TasCommand {
        if self.cfg.gate_align_rec > 0 {
            TasCommand::StopForRestart
        } else {
            TasCommand::Stop
        }
    }

    fn reroll(
        &mut self,
        port: &mut impl TransportPort,
        detail: String,
        observed: Option<u32>,
    ) -> StepOutcome {
        if self.retries_used == self.cfg.max_retries {
            port.send_command(TasCommand::Stop);
            self.phase = Phase::Aborted;
            let reason = if self.cfg.max_retries == 0 {
                format!("{detail}; a replay must match, so this is a TAS bug (not retried)")
            } else {
                format!("{} after {} retries", detail, self.cfg.max_retries)
            };
            return StepOutcome::Aborted { reason };
        }
        self.retries_used += 1;
        port.set_continue_from_frame(self.cfg.continue_from_frame);
        port.set_gate_align_rec(0);
        port.set_playback_speed(self.cfg.speed);
        port.send_command(self.restart_stop_command());
        self.phase = Phase::StopWaitAck;
        StepOutcome::Reroll {
            attempt: self.retries_used,
            observed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::align::aligned_trajectory;

    #[derive(Default)]
    struct FakePort {
        mode: u32,
        restart_state: u32,
        playback_pos: u32,
        play_coords: Vec<[f32; 3]>,
        rec_coords: Vec<[f32; 3]>,
        recorded_count: u32,
        gate_index: u32,
        continue_from_frame: u32,
        gate_align_rec: u32,
        playback_speed: f32,
        arm_generation: u32,
        capture_ok_flag: bool,
        commands: Vec<TasCommand>,
        /// False by default: existing unit tests model an immediate cycle cave
        /// consumption. Set true to exercise a delayed acknowledgement.
        command_busy: bool,
        /// Invariant tracker: Restart must NEVER be sent while mode != OFF.
        restart_while_not_off: bool,
        splice_approved: bool,
        input_model: u32,
        /// Ownership requests in order (TAS_OWNER_ACQUIRE / RELEASE).
        owner_requests: Vec<u32>,
        /// The DLL's answer to every request; None = answers Owned at once.
        owner_answer: Option<Option<OwnerAnswer>>,
    }

    impl TransportPort for FakePort {
        fn send_command(&mut self, cmd: TasCommand) {
            if cmd == TasCommand::Restart && self.mode != TasMode::Off as u32 {
                self.restart_while_not_off = true;
            }
            // Stand in for the cycle cave processing the arm.
            if matches!(
                cmd,
                TasCommand::ArmRec | TasCommand::ArmPlay | TasCommand::ArmContinue
            ) {
                self.arm_generation = self.arm_generation.wrapping_add(1);
            }
            self.commands.push(cmd);
        }
        fn command_idle(&self) -> bool {
            !self.command_busy
        }
        fn mode(&self) -> u32 {
            self.mode
        }
        fn restart_state(&self) -> u32 {
            self.restart_state
        }
        fn reset_restart_state(&mut self) {
            self.restart_state = 0;
        }
        fn playback_pos(&self) -> u32 {
            self.playback_pos
        }
        fn play_coords(&self) -> &[[f32; 3]] {
            &self.play_coords
        }
        fn rec_coords(&self) -> &[[f32; 3]] {
            &self.rec_coords
        }
        fn recorded_count(&self) -> u32 {
            self.recorded_count
        }
        fn gate_index(&self) -> u32 {
            self.gate_index
        }
        fn set_continue_from_frame(&mut self, frame: u32) {
            self.continue_from_frame = frame;
        }
        fn set_gate_align_rec(&mut self, frame: u32) {
            self.gate_align_rec = frame;
        }
        fn set_input_model(&mut self, model: u32) {
            self.input_model = model;
        }
        fn set_playback_speed(&mut self, speed: f32) {
            self.playback_speed = speed;
        }
        fn arm_generation(&self) -> u32 {
            self.arm_generation
        }
        fn capture_ok(&self) -> bool {
            self.capture_ok_flag
        }
        fn request_ownership(&mut self, kind: u32) -> u32 {
            self.owner_requests.push(kind);
            self.owner_requests.len() as u32
        }
        fn ownership_answer(&self, _seq: u32) -> Option<OwnerAnswer> {
            self.owner_answer.unwrap_or(Some(OwnerAnswer::Owned))
        }
        fn approve_cont_splice(&mut self) {
            self.splice_approved = true;
        }
    }

    fn cfg(arm: Arm, max_retries: u32) -> ArmConfig {
        ArmConfig {
            arm,
            speed: 12.0,
            continue_from_frame: if arm == Arm::Continue { 320 } else { 0 },
            gate_align_rec: 0,
            max_retries,
            input_model: crate::TAS_INPUT_MODEL_INJECTED,
            trajectory_ticks: u32::MAX,
        }
    }

    #[test]
    fn rec_restart_serializes_stop_before_restart() {
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            command_busy: true,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Rec, 0));

        // Start publishes Stop; Restart waits for the cycle cave to ack it.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands, vec![TasCommand::Stop]);
        // Even MODE_OFF is insufficient while the command slot still holds
        // Stop: Restart would overwrite the unacknowledged command.
        p.mode = TasMode::Off as u32;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands, vec![TasCommand::Stop]);
        p.command_busy = false;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands.last(), Some(&TasCommand::Restart));

        // Not done until the DLL has taken the arm.
        p.restart_state = 2;
        let generation = p.arm_generation;
        p.command_busy = true;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands.last(), Some(&TasCommand::ArmRec));
        p.arm_generation = generation;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        p.arm_generation = generation + 1;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
        assert_eq!(
            p.commands,
            vec![TasCommand::Stop, TasCommand::Restart, TasCommand::ArmRec]
        );
        assert!(!p.restart_while_not_off, "Restart sent while mode != OFF!");
        assert!(c.is_terminal());
    }

    #[test]
    fn aligned_play_stages_gate_only_after_restart() {
        // recorded_count 400, rec_gate 299: the watcher validates the
        // full 101-frame gate-relative prefix before declaring Match.
        let (play_coords, rec_coords) = aligned_trajectory(299, 297, 101);
        let mut p = FakePort {
            mode: TasMode::Off as u32,
            gate_align_rec: 777,
            gate_index: 297,
            playback_pos: 398,
            play_coords,
            rec_coords,
            recorded_count: 400,
            capture_ok_flag: true,
            ..Default::default()
        };
        let mut config = cfg(Arm::Play, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        // Even from OFF, always Stop first.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands, vec![TasCommand::StopForRestart]);
        assert_eq!(
            p.gate_align_rec, 0,
            "stale alignment must clear before STOP"
        );
        assert_eq!(c.step(&mut p), StepOutcome::InProgress); // Restart
        assert_eq!(
            p.commands,
            vec![TasCommand::StopForRestart, TasCommand::Restart]
        );
        assert_eq!(
            p.gate_align_rec, 0,
            "restart must happen with alignment clear"
        );
        p.restart_state = 2;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(
            p.commands,
            vec![
                TasCommand::StopForRestart,
                TasCommand::Restart,
                TasCommand::ArmPlay
            ]
        );
        assert_eq!(p.continue_from_frame, 0);
        assert_eq!(p.gate_align_rec, 299, "alignment must be armed with PLAY");
        p.mode = TasMode::Play as u32;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Matched
            }
        );
    }

    #[test]
    fn aligned_play_rerolls_hidden_state_mismatch_then_accepts_exact_retry() {
        let (mut play_coords, rec_coords) = aligned_trajectory(299, 297, 101);
        play_coords[297][1] = f32::from_bits(play_coords[297][1].to_bits() + 1);
        let mut p = FakePort {
            gate_index: 297,
            playback_pos: 298,
            play_coords,
            rec_coords,
            recorded_count: 400,
            capture_ok_flag: true,
            ..Default::default()
        };
        let mut config = cfg(Arm::Play, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);

        assert_eq!(
            c.step(&mut p),
            StepOutcome::Reroll {
                attempt: 1,
                observed: Some(0),
            }
        );
        assert_eq!(p.gate_align_rec, 0, "reroll STOP must clear alignment");

        drive_reroll_to_judge(&mut c, &mut p);
        let (play_coords, rec_coords) = aligned_trajectory(299, 300, 101);
        p.gate_index = 300;
        p.playback_pos = 401;
        p.play_coords = play_coords;
        p.rec_coords = rec_coords;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 1,
                completed_via: CompletedVia::Matched
            }
        );
    }

    #[test]
    fn aligned_play_rerolls_if_replay_ends_before_watcher_completes() {
        let (play_coords, rec_coords) = aligned_trajectory(299, 297, 64);
        let mut p = FakePort {
            gate_index: 297,
            playback_pos: 360,
            play_coords,
            rec_coords,
            recorded_count: 400,
            capture_ok_flag: true,
            ..Default::default()
        };
        let mut config = cfg(Arm::Play, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);
        p.mode = TasMode::Off as u32;

        assert!(matches!(
            c.step(&mut p),
            StepOutcome::Reroll {
                attempt: 1,
                observed: None,
            }
        ));
    }

    /// Drive the controller through the restart until it has sent the arm
    /// command and entered Watch.
    fn drive_to_judge(c: &mut TransportController, p: &mut FakePort) {
        // Start publishes Stop. Tests may begin in REC/PLAY, so model the
        // cycle cave consuming it.
        c.step(p);
        p.mode = TasMode::Off as u32;
        p.command_busy = false;
        c.step(p); // StopWaitAck -> Restart
        p.restart_state = 2;
        c.step(p); // RestartWaitDone -> arm command, phase -> Watch
        assert_eq!(p.commands.last(), Some(&c.cfg.arm.command()));
        // game enters PLAY for the replay
        p.mode = TasMode::Play as u32;
    }

    /// Drive the reroll restart (after a Reroll outcome) up to re-arm.
    fn drive_reroll_to_judge(c: &mut TransportController, p: &mut FakePort) {
        p.mode = TasMode::Off as u32;
        p.command_busy = false;
        c.step(p); // StopWaitAck -> Restart
        p.restart_state = 2;
        c.step(p); // RestartWaitDone -> arm command
        p.mode = TasMode::Play as u32;
    }

    /// A replay that never moves (no gate) arms and is done: nothing to judge.
    #[test]
    fn play_without_a_gate_finishes_unjudged() {
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, 30));
        c.step(&mut p); // Stop
        p.mode = TasMode::Off as u32;
        c.step(&mut p); // -> Restart
        p.restart_state = 2;
        c.step(&mut p); // -> ArmPlay
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
    }

    /// An aligned port: recording gate 299, live gate 297, and `depth`
    /// matching frames past the gate.
    fn aligned_port(depth: usize) -> FakePort {
        let (play_coords, rec_coords) = aligned_trajectory(299, 297, depth);
        FakePort {
            mode: TasMode::Rec as u32,
            gate_index: 297,
            play_coords,
            rec_coords,
            recorded_count: 400,
            capture_ok_flag: true,
            ..Default::default()
        }
    }

    fn aligned_cont(splice: u32, max_retries: u32) -> TransportController {
        let mut config = cfg(Arm::Continue, max_retries);
        config.continue_from_frame = splice;
        config.gate_align_rec = 299;
        TransportController::new(config)
    }

    /// A CONT spliced one frame past the gate is still watched: the depth is
    /// the single frame before the splice, and the approval releases it.
    #[test]
    fn aligned_cont_just_past_the_gate_is_watched_then_approved() {
        let mut p = aligned_port(101);
        let mut c = aligned_cont(300, 1);
        drive_to_judge(&mut c, &mut p);
        assert_eq!(p.continue_from_frame, 300, "splice handed to the game");
        p.playback_pos = 297;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert!(!p.splice_approved);
        p.playback_pos = 298;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Matched
            }
        );
        assert!(p.splice_approved);
    }

    /// Spliced inside the countdown, the boarder has not moved, so the spawn
    /// is all there is to check; the parked splice is approved once it
    /// matches, or it would never fire.
    #[test]
    fn aligned_cont_inside_the_countdown_checks_the_spawn_then_approves() {
        let mut p = aligned_port(101);
        let mut c = aligned_cont(250, 1);
        drive_to_judge(&mut c, &mut p);
        assert_eq!(
            c.step(&mut p),
            StepOutcome::InProgress,
            "nothing replayed yet"
        );
        p.playback_pos = 10;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
        assert!(p.splice_approved);
    }

    /// An edit at tick 320 leaves the recording stale from there: a replay
    /// that departs from it past the edit is the edit's effect, not a
    /// divergence. PLAY and CONT judge only up to the edit.
    #[test]
    fn an_edited_take_is_judged_up_to_the_edit() {
        for arm in [Arm::Play, Arm::Continue] {
            let mut p = aligned_port(101);
            // gate+30 = recording tick 329, after the edit.
            p.play_coords[297 + 30][0] += 1.0;
            let mut config = cfg(arm, 0);
            config.gate_align_rec = 299;
            config.continue_from_frame = if arm == Arm::Continue { 380 } else { 0 };
            config.trajectory_ticks = 320;
            let mut c = TransportController::new(config);
            drive_to_judge(&mut c, &mut p);
            p.playback_pos = 297 + 20;
            assert_eq!(c.step(&mut p), StepOutcome::InProgress, "{arm:?}");
            p.playback_pos = 297 + 21;
            assert_eq!(
                c.step(&mut p),
                StepOutcome::Done {
                    retries_used: 0,
                    completed_via: CompletedVia::Matched
                },
                "{arm:?}"
            );
            assert_eq!(p.splice_approved, arm == Arm::Continue);
        }
    }

    /// An edit inside the countdown leaves only the spawn to check.
    #[test]
    fn a_take_edited_inside_the_countdown_checks_the_spawn() {
        let mut p = aligned_port(101);
        p.play_coords[297 + 5][0] += 1.0;
        let mut config = cfg(Arm::Play, 0);
        config.gate_align_rec = 299;
        config.trajectory_ticks = 250;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);
        p.playback_pos = 10;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
        assert!(!p.splice_approved, "PLAY has no splice to approve");
    }

    /// A different spawn is a different starting state: reroll, never splice.
    #[test]
    fn aligned_cont_inside_the_countdown_rerolls_a_wrong_spawn() {
        let mut p = aligned_port(101);
        p.play_coords[0][0] = f32::from_bits(p.play_coords[0][0].to_bits() + 1);
        let mut c = aligned_cont(250, 1);
        drive_to_judge(&mut c, &mut p);
        p.playback_pos = 10;
        assert!(matches!(
            c.step(&mut p),
            StepOutcome::Reroll {
                attempt: 1,
                observed: Some(0)
            }
        ));
        assert!(!p.splice_approved);
    }

    /// A reroll must re-assert the splice frame, or the cycle cave splices at the
    /// wrong tick; and with no retries left the mismatch aborts.
    #[test]
    fn aligned_cont_reroll_restores_the_splice_then_aborts_when_out_of_retries() {
        let mut p = aligned_port(101);
        p.play_coords[297][0] += 1.0;
        let mut c = aligned_cont(320, 1);
        drive_to_judge(&mut c, &mut p);
        p.playback_pos = 298;
        p.continue_from_frame = 0;
        assert!(matches!(
            c.step(&mut p),
            StepOutcome::Reroll { attempt: 1, .. }
        ));
        assert_eq!(p.continue_from_frame, 320, "splice re-asserted on reroll");
        drive_reroll_to_judge(&mut c, &mut p);
        p.playback_pos = 298;
        match c.step(&mut p) {
            StepOutcome::Aborted { reason } => assert!(reason.contains("diverged"), "{reason}"),
            other => panic!("expected Aborted, got {:?}", other),
        }
        assert!(!p.splice_approved);
        assert!(c.is_terminal());
    }

    /// A short replay at catch-up speed can begin and end between two polls,
    /// so PLAY mode is never sampled. The arm counter moved and the whole
    /// trajectory is there, so it is judged, not spun on.
    #[test]
    fn judge_finishes_when_play_mode_is_never_observed() {
        let mut p = aligned_port(101);
        let mut config = cfg(Arm::Play, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);
        p.mode = TasMode::Off as u32;
        p.playback_pos = 398;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Matched
            }
        );
    }

    /// Only an approved splice enters REC, and approving finishes the cycle, so
    /// REC seen while watching is someone else driving the game: stop and abort,
    /// never report it as a match.
    #[test]
    fn rec_while_watching_aborts_instead_of_matching() {
        let mut p = aligned_port(101);
        let mut c = aligned_cont(320, 30);
        drive_to_judge(&mut c, &mut p);
        p.mode = TasMode::Rec as u32;
        p.playback_pos = 100;
        assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
        assert_eq!(p.commands.last(), Some(&TasCommand::Stop));
        assert!(!p.splice_approved);
    }

    /// A refused arm never enters PLAY and replays nothing. That is a failure,
    /// not a quiet success: the UI only tears down a catch-up on Aborted.
    #[test]
    fn a_refused_arm_aborts_rather_than_reporting_success() {
        let mut p = aligned_port(101);
        let mut c = aligned_cont(320, 30);
        drive_to_judge(&mut c, &mut p);
        p.mode = TasMode::Off as u32;
        p.playback_pos = 0;
        assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
        assert!(c.is_terminal());
    }
    #[test]
    fn a_cycle_registers_before_its_first_command() {
        let mut p = FakePort {
            owner_answer: Some(None),
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, 0));
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.owner_requests, vec![TAS_OWNER_ACQUIRE]);
        assert!(
            p.commands.is_empty(),
            "nothing is sent before the DLL answers"
        );
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert!(p.commands.is_empty());
        p.owner_answer = None;
        c.step(&mut p);
        assert_eq!(p.commands, vec![TasCommand::Stop]);
        assert_eq!(
            p.owner_requests,
            vec![TAS_OWNER_ACQUIRE],
            "one request per cycle"
        );
    }

    #[test]
    fn a_busy_owner_aborts_the_cycle_without_touching_the_game() {
        let mut p = FakePort {
            owner_answer: Some(Some(OwnerAnswer::Busy { owner_pid: 77 })),
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, 0));
        match c.step(&mut p) {
            StepOutcome::Aborted { reason } => assert!(reason.contains("pid 77"), "{reason}"),
            other => panic!("expected Aborted, got {other:?}"),
        }
        assert!(p.commands.is_empty());
        assert_eq!(
            p.owner_requests,
            vec![TAS_OWNER_ACQUIRE],
            "a refused request owns nothing to release"
        );
    }

    #[test]
    fn a_finished_cycle_releases_ownership_once() {
        let mut p = FakePort::default();
        let mut c = TransportController::new(cfg(Arm::Rec, 0));
        for _ in 0..20 {
            if matches!(p.commands.last(), Some(TasCommand::Restart)) {
                p.restart_state = 2;
            }
            if c.is_terminal() {
                break;
            }
            c.step(&mut p);
        }
        assert!(c.is_terminal());
        c.step(&mut p);
        c.release(&mut p);
        assert_eq!(p.owner_requests, vec![TAS_OWNER_ACQUIRE, TAS_OWNER_RELEASE]);
    }

    #[test]
    fn a_cancelled_cycle_releases_ownership() {
        let mut p = FakePort::default();
        let mut c = TransportController::new(cfg(Arm::Play, 0));
        c.step(&mut p);
        assert!(!c.is_terminal());
        c.release(&mut p);
        c.release(&mut p);
        assert_eq!(p.owner_requests, vec![TAS_OWNER_ACQUIRE, TAS_OWNER_RELEASE]);
    }

    #[test]
    fn the_takes_input_model_is_staged_after_the_restart() {
        let mut p = FakePort {
            input_model: crate::TAS_INPUT_MODEL_INJECTED,
            ..Default::default()
        };
        let mut config = cfg(Arm::Play, 0);
        config.input_model = crate::TAS_INPUT_MODEL_HELD;
        let mut c = TransportController::new(config);
        c.step(&mut p); // Stop
        c.step(&mut p); // Restart
        assert_eq!(p.input_model, crate::TAS_INPUT_MODEL_INJECTED);
        p.restart_state = 2;
        c.step(&mut p); // Arm
        assert_eq!(p.commands.last(), Some(&TasCommand::ArmPlay));
        assert_eq!(p.input_model, crate::TAS_INPUT_MODEL_HELD);
    }
}
