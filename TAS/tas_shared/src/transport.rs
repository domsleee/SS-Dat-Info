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

use crate::cont::{
    detect_first_moving, judge_cont_bucket, BucketVerdict, COUNTDOWN_K_JITTER,
    MAX_BLIND_PREDICTIVE_REJECTS,
};
use crate::{TasCommand, TasMode};

/// First N rerolls use NO jitter — the plain restart already has natural
/// wall-clock variance, and for a recording whose restart usually lands the
/// right bucket (the common case), jittering immediately shoves the phase
/// AWAY from that natural center and scatters into wrong buckets. Only once
/// genuinely stuck do we start jittering to escape.
pub const NATURAL_RESTART_ATTEMPTS: u32 = 8;

/// Wall-clock jitter (ms) before a reroll's restart, so the injected F5
/// lands at a different accumulator-modulo-tick phase than the last attempt.
/// Zero for the first `NATURAL_RESTART_ATTEMPTS` (let natural variance work),
/// then a phase-cycling delay to escape a stuck bucket: 7 and 17 are coprime,
/// so consecutive attempts visit all 17 delays (1..=17 ms) before one
/// repeats. Kept here (not in tas_ui) so the harness reroll jitters
/// identically.
pub fn cont_retry_jitter_ms(attempt: u32) -> u64 {
    if attempt <= NATURAL_RESTART_ATTEMPTS {
        return 0;
    }
    (attempt as u64 * 7 + 3) % 17 + 1
}

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

/// The recording's F5-bucket fingerprint a CONT replay must match (computed
/// by the caller from `rec_coords` via [`crate::cont::detect_first_moving`]).
#[derive(Debug, Clone, Copy)]
pub struct BucketTarget {
    pub expected_start_bits: [u32; 3],
    pub expected_first_moving: Option<u32>,
}

/// Everything the controller needs to drive one arm cycle to completion.
#[derive(Debug, Clone, Copy)]
pub struct ArmConfig {
    pub arm: Arm,
    /// Replay/catch-up speed to assert before arming.
    pub catchup_speed: f32,
    /// Splice frame for CONT (0 for REC/PLAY).
    pub continue_from_frame: u32,
    /// Recording gate used to index PLAY input relative to the live gate.
    /// Zero disables alignment (required for REC and arm-relative CONT).
    pub gate_align_rec: u32,
    /// Arm-relative bucket fingerprint to match; `Some` for CONT and
    /// arm-relative judged PLAY. Gate-aligned PLAY uses `gate_align_rec`
    /// instead.
    pub target: Option<BucketTarget>,
    /// Max restart retries for a bucket or aligned-trajectory mismatch.
    pub max_retries: u32,
    /// Speed to hand back to once the replay reaches the first moving
    /// frame - 0 disables the handover and the whole cycle stays at
    /// `catchup_speed`.
    ///
    /// A judged PLAY replays the stationary spawn countdown only so the
    /// judge can see the departure frame, so it fast-forwards that part and
    /// hands back here, the way CONT hands back at its splice. Captured when
    /// the cycle is armed, so moving the speed control mid-judge does not
    /// move it. The DLL performs the handover on the exact tick (cave2, with
    /// cave5 capping the batch to land on it) because a poll-driven drop can
    /// be a whole catch-up batch late.
    pub resume_speed: f32,
    /// Throw a determined-wrong bucket away at arm time instead of
    /// replaying the countdown to confirm it (see
    /// `TransportController::learned_countdown_k`). The predictive path can
    /// only ever reject, so switching it off costs speed and nothing else.
    pub predict_bucket: bool,
}

/// The few shared-memory operations the transport machine performs. Returns
/// raw `u32` for mode/restart_state to match the live client's accessors.
pub trait TransportPort {
    fn send_command(&mut self, cmd: TasCommand);
    /// True only after Cave2 consumed and cleared the single command slot.
    fn command_idle(&self) -> bool;
    fn mode(&self) -> u32;
    fn restart_state(&self) -> u32;
    fn reset_restart_state(&mut self);
    fn playback_pos(&self) -> u32;
    fn play_coords(&self) -> &[[f32; 3]];
    /// The loaded recording's trajectory + length — for the bit-exact bucket
    /// fingerprint (the replay must reproduce this, not just its first-move).
    fn rec_coords(&self) -> &[[f32; 3]];
    fn recorded_count(&self) -> u32;
    /// First live replay frame where the boarder left spawn. Zero until the
    /// gate has been observed for the current arm.
    fn gate_index(&self) -> u32;
    fn set_continue_from_frame(&mut self, frame: u32);
    fn set_gate_align_rec(&mut self, frame: u32);
    fn set_playback_speed(&mut self, speed: f32);
    /// Ask the DLL to drop the playback speed to `speed` at replay
    /// position `pos`, atomically on that tick. `pos == 0` clears any
    /// pending handover.
    fn set_speed_handoff(&mut self, pos: u32, speed: f32);
    /// True while a handover written by [`Self::set_speed_handoff`] has
    /// not fired yet. The DLL clears it when it fires, which is how the
    /// controller knows to stop re-asserting the catch-up speed.
    fn speed_handoff_pending(&self) -> bool;
    /// Monotonic counter the DLL bumps once per processed arm. Used to tell
    /// this attempt's replay state from the previous one's.
    fn arm_generation(&self) -> u32;
    /// `restart_done_tick` latched when this attempt's arm was consumed.
    fn arm_restart_tick(&self) -> u32;
    /// tick_count when this attempt's arm was consumed.
    fn arm_consumed_tick(&self) -> u32;
    /// False if any coordinate capture in this session failed, which would
    /// leave a stale hole in the prefix a first-moving scan reads.
    fn capture_ok(&self) -> bool;
    /// Approve the aligned CONT splice: the gate-relative watcher has
    /// validated the whole prefix it will ever see. The DLL parks
    /// playback AT the splice until this is written, so a starved or
    /// dead controller can never let an unjudged prefix be spliced.
    /// Default no-op for ports without a DLL behind them.
    fn approve_cont_splice(&mut self) {}
}

/// Fixed wall-clock delay (ms) between an acknowledged Stop and Restart.
/// Command-slot serialization comes from the explicit idle/OFF
/// acknowledgement; this additional delay fixes the F5 phase. The
/// post-restart bucket is decided by the wall-clock-modulo-tick at the
/// Restart, so a CONSISTENT Stop→Restart delay lands a consistent (good)
/// bucket, where firing Restart at a poll-rate-dependent phase scatters hard
/// recordings into wrong buckets. The same 50 ms as
/// `restart_and_stabilize_inprocess` in tas_test, which lands hard
/// recordings like FE-10065 reliably.
pub const STOP_SETTLE_MS: u64 = 50;

/// Delay (ms) between restart_state==2 and sending the Arm command. The arm
/// point sets where play_coords[0] is captured in the spawn countdown, which
/// shifts the OBSERVED first-moving frame: `restart_and_stabilize_inprocess`
/// (tas_test) arms ~15-20 ms after rs==2, and arming immediately reads
/// first-moving ~2 frames late and never matches recordings captured with
/// that timing (FE-10065 wants 298, an immediate arm sees 300). This pins
/// the arm phase to match.
pub const ARM_SETTLE_MS: u64 = 10;

/// Observed first-moving frame when the arm fires with ZERO settle, and the
/// ms of settle that shifts the observed first-moving by one frame (the arm
/// arms later → shorter remaining countdown → smaller first-moving). Both
/// empirical (FE-10065: 0ms→300, 20ms→~297.5).
pub const OBSERVED_AT_ZERO_SETTLE: i64 = 300;
pub const ARM_SETTLE_MS_PER_FRAME: i64 = 8;

/// Attempts that retry the computed base settle UNCHANGED before dithering.
/// The restart has ±1-2 frame variance, so the correct base lands within a
/// few plain retries; dithering immediately (as a sweep does) instead wastes
/// attempts on the wrong target and fattens the tail. Only after this many
/// base-misses do we assume the base is miscalibrated and start dithering.
pub const ARM_SETTLE_BASE_TRIES: u32 = 5;

/// Arm-settle delay (ms) for a given attempt. Rather than blindly sweeping,
/// COMPUTE the settle that targets the recording's known first-moving
/// (`observed ≈ OBSERVED_AT_ZERO_SETTLE - settle/MS_PER_FRAME`), then dither
/// a few frames around it to absorb the ±1-2 frame restart variance. So a
/// recording whose first-moving is an outlier (FE-goodstart=300) is targeted
/// on attempt 0 instead of found ~1-in-10.
pub fn arm_settle_ms(attempt: u32, expected_first_moving: Option<u32>) -> u64 {
    let base = match expected_first_moving {
        Some(fm) => ((OBSERVED_AT_ZERO_SETTLE - fm as i64) * ARM_SETTLE_MS_PER_FRAME).clamp(0, 48),
        None => ARM_SETTLE_MS as i64,
    };
    if attempt < ARM_SETTLE_BASE_TRIES {
        // Retry the computed target; the restart variance lands it.
        return base.clamp(0, 64) as u64;
    }
    // Base kept missing → it's probably miscalibrated. Dither ±frames to
    // find the true arm phase.
    const DITHER: [i64; 6] = [8, -8, 16, -16, 24, -24];
    let d = (attempt - ARM_SETTLE_BASE_TRIES) as usize;
    (base + DITHER[d % DITHER.len()]).clamp(0, 64) as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Start,
    /// Stop sent; wait until Cave2 has consumed it and published MODE_OFF.
    StopWaitAck,
    /// Stop acknowledged; the caller is honouring the deterministic delay.
    StopSettle,
    /// Restart sent; waiting for restart_state == 2.
    RestartWaitDone,
    /// restart_state==2 seen; honouring a fixed ARM_SETTLE_MS wait before the
    /// Arm command so the arm phase (→ observed first-moving) is consistent.
    ArmSettle,
    /// Replaying: judge either the CONT bucket or aligned PLAY trajectory.
    JudgeBucket,
    Done,
    Aborted,
}

/// Result of one `step()`.
#[derive(Debug, Clone, PartialEq)]
pub enum StepOutcome {
    /// Mid-cycle; call `step()` again (after a short poll delay).
    InProgress,
    /// Wait exactly `ms` (wall-clock) before the next `step()` — used for the
    /// fixed Stop→Restart settle that pins the F5 phase.
    Wait { ms: u64 },
    /// A reroll was scheduled. Caller should wait `suggested_delay_ms`
    /// (harness: sleep; egui: it already slept) before the next `step()`.
    /// For bucket judgment, `observed`/`expected` are first-moving frames.
    /// For aligned PLAY, `observed` is the first gate-relative mismatch and
    /// `expected` is `None`.
    Reroll {
        attempt: u32,
        suggested_delay_ms: u64,
        observed: Option<u32>,
        expected: Option<u32>,
    },
    /// Terminal success: armed (and, for CONT, landed the bucket).
    /// `completed_via` says HOW the bucket was accepted — a `NoSignal` accept
    /// was NOT positively confirmed and may be a near-miss bucket.
    Done {
        retries_used: u32,
        completed_via: CompletedVia,
    },
    /// Terminal failure: gave up after exhausting retries.
    Aborted { reason: String },
}

/// How a CONT/PLAY cycle reached `Done` — diagnostic for "did it resume on
/// the right bucket?".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletedVia {
    /// The bucket fingerprint or aligned trajectory positively matched.
    BucketMatched,
    /// Accepted with no movement signal to judge against — the resumed state
    /// could be from a near-miss bucket. Prime suspect for a "wrong" resume.
    NoSignal,
    /// Nothing to judge (plain PLAY/REC with no fingerprint) or the splice
    /// had already fired before judging.
    Unjudged,
}

/// Ticks between this attempt's restart completing and its arm being
/// consumed — the only thing that moves `first_moving`. `None` when either
/// stamp is missing, which just disables the predictive path.
fn arm_offset(port: &impl TransportPort) -> Option<i64> {
    let r = port.arm_restart_tick();
    let a = port.arm_consumed_tick();
    if r == 0 || a == 0 {
        return None;
    }
    Some(a.wrapping_sub(r) as i64)
}

/// Judge the first gate-relative window of an aligned PLAY.
///
/// CONT deliberately allows sub-tick trajectory skew after matching its
/// bucket because it only needs a safe state to splice from. Aligned PLAY
/// has a stronger contract: cave2 is feeding the same recorded input from
/// the same semantic gate, so the resulting coordinates must be bit-exact.
/// A differing hidden spawn state shows up immediately in this window and
/// must be rerolled rather than allowed to become the watched run.
#[allow(clippy::too_many_arguments)] // a judge reads many independent shared-state fields
pub fn judge_gate_aligned_play(
    play_coords: &[[f32; 3]],
    rec_coords: &[[f32; 3]],
    recorded_count: u32,
    playback_pos: u32,
    live_gate: u32,
    rec_gate: u32,
    capture_ok: bool,
    max_depth_rel: u32,
) -> BucketVerdict {
    if !capture_ok {
        return BucketVerdict::WrongStart;
    }
    if live_gate == 0 || playback_pos <= live_gate {
        return BucketVerdict::KeepWaiting;
    }

    let rec_end = (recorded_count as usize).min(rec_coords.len());
    let rec_gate = rec_gate as usize;
    let live_gate = live_gate as usize;
    if rec_gate >= rec_end || live_gate >= play_coords.len() {
        return BucketVerdict::NoSignal;
    }

    // Validate DEEP, not just through the settle: a replay exact for the
    // first 64 frames can still veer off later, which is why
    // BUCKET_VALIDATE_WINDOW exists. `max_depth_rel` caps the depth at the
    // splice for CONT (0 = uncapped): the verdict must be decidable from
    // the prefix that exists while the DLL parks the splice waiting for
    // approval.
    let depth_cap = if max_depth_rel == 0 {
        usize::MAX
    } else {
        max_depth_rel as usize
    };
    let depth = (rec_end - rec_gate)
        .min(crate::cont::BUCKET_VALIDATE_WINDOW as usize)
        .min(depth_cap);
    if depth == 0 {
        return BucketVerdict::NoSignal;
    }
    let available = (playback_pos as usize)
        .saturating_sub(live_gate)
        .min(play_coords.len().saturating_sub(live_gate))
        .min(depth);

    for k in 0..available {
        let p = play_coords[live_gate + k];
        let r = rec_coords[rec_gate + k];
        if !p.iter().all(|v| v.is_finite())
            || !r.iter().all(|v| v.is_finite())
            || p[0].to_bits() != r[0].to_bits()
            || p[1].to_bits() != r[1].to_bits()
            || p[2].to_bits() != r[2].to_bits()
        {
            return BucketVerdict::WrongBucket {
                observed: Some(k as u32),
            };
        }
    }

    if available == depth {
        BucketVerdict::Match
    } else {
        BucketVerdict::KeepWaiting
    }
}

/// Drives one restart→arm(→judge→reroll) cycle to a terminal outcome.
pub struct TransportController {
    cfg: ArmConfig,
    phase: Phase,
    retries_remaining: u32,
    completed_via: CompletedVia,
    /// A speed handover has been staged with the DLL for this attempt.
    handoff_sent: bool,
    /// The countdown's length in ticks, LEARNED from an attempt that
    /// actually replayed rather than assumed from a constant.
    ///
    /// `first_moving = K - (arm_consumed_tick - restart_done_tick)`, so one
    /// observed first-moving frame plus that attempt's arm offset gives K,
    /// and every LATER attempt's bucket is then computable the moment it
    /// arms — before a single tick is replayed.
    ///
    /// Learned rather than hardcoded on purpose. A constant that is wrong
    /// for some other track or machine would reject every attempt and the
    /// cycle would never land; a value learned from this cycle's own first
    /// attempt cannot be wrong about this cycle. The first attempt pays the
    /// full replay either way — something has to be tried first.
    learned_countdown_k: Option<i64>,
    /// How many rerolls this cycle threw away without replaying anything.
    predictive_rejects: u32,
    /// Consecutive predictive rejects with no replay in between, i.e. with
    /// nothing confirming the K they were based on.
    blind_predictive_rejects: u32,
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
}

impl TransportController {
    pub fn new(cfg: ArmConfig) -> Self {
        let retries_remaining = cfg.max_retries;
        Self {
            cfg,
            phase: Phase::Start,
            retries_remaining,
            completed_via: CompletedVia::Unjudged,
            handoff_sent: false,
            learned_countdown_k: None,
            predictive_rejects: 0,
            blind_predictive_rejects: 0,
            arm_generation_at_arm: 0,
        }
    }

    /// Rerolls this cycle rejected at arm time, with nothing replayed.
    pub fn predictive_rejects(&self) -> u32 {
        self.predictive_rejects
    }

    /// True when this cycle owns `playback_speed` outright: it stages a
    /// speed handover, so it asserts the catch-up before that fires and the
    /// resume speed after, on every step.
    ///
    /// Exists so a UI that also syncs the speed can step aside for exactly
    /// the controller's lifetime. Deriving it from the controller means
    /// there is no separate flag to forget to clear on a reroll, an abort or
    /// a STOP.
    pub fn owns_playback_speed(&self) -> bool {
        self.cfg.resume_speed > 0.0
    }

    /// Which transition the cycle is waiting on, for diagnostics.
    ///
    /// A stalled cycle reports only "no progress within budget", which is
    /// the same message whether the F5 restart never completed, the arm was
    /// never processed, or the judge is waiting on a replay that will not
    /// arrive. Those have completely different causes and the log could not
    /// tell them apart.
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            Phase::Start => "Start",
            Phase::StopWaitAck => "StopWaitAck (waiting for Cave2 to consume Stop)",
            Phase::StopSettle => "StopSettle (waiting out the fixed Stop->Restart delay)",
            Phase::RestartWaitDone => "RestartWaitDone (waiting for the F5 restart)",
            Phase::ArmSettle => "ArmSettle",
            Phase::JudgeBucket => "JudgeBucket (waiting on the replay)",
            Phase::Done => "Done",
            Phase::Aborted => "Aborted",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, Phase::Done | Phase::Aborted)
    }

    /// True while the NEXT transition's timing feeds the F5 spawn phase
    /// (Stop acknowledgement, restart-done detection, arm settle): a driver
    /// should poll these without vsync quantization. The multi-second
    /// `JudgeBucket` replay gains nothing from sub-frame latency, so a UI
    /// driver can poll it once per frame instead of spinning.
    pub fn needs_tight_polling(&self) -> bool {
        matches!(
            self.phase,
            Phase::Start
                | Phase::StopWaitAck
                | Phase::StopSettle
                | Phase::RestartWaitDone
                | Phase::ArmSettle
        )
    }

    fn retries_used(&self) -> u32 {
        self.cfg.max_retries - self.retries_remaining
    }

    /// Perform at most one transition. Never blocks/sleeps.
    pub fn step(&mut self, port: &mut impl TransportPort) -> StepOutcome {
        let rec = TasMode::Rec as u32;
        let play = TasMode::Play as u32;
        // Re-assert the catch-up speed on EVERY step, not just at phase
        // boundaries. The in-process F5 restart momentarily resets the game's
        // speed; without continuous re-assertion the post-restart countdown
        // replays at 1x for ~3s until the next phase re-sets it. (tas_ui used
        // to do this via an ungated per-frame sync; the controller owns it
        // now.)
        //
        // EXCEPT once a staged speed handover has fired: from there the
        // speed to hold is the resume speed, and re-asserting the catch-up
        // would undo the handover and leave the user watching the run at
        // 64x. `handoff_sent` keeps the catch-up alive for the phases
        // BEFORE the handover is staged (Start..ArmSettle), where
        // `speed_handoff_pending()` is also false but for the opposite
        // reason.
        //
        // It ASSERTS the resume speed rather than just going quiet, because
        // this read and the write below are not one operation: a step can
        // read "still pending", have cave2 fire underneath it, and then
        // write the catch-up speed over the handover.
        if self.handoff_sent && !port.speed_handoff_pending() {
            port.set_playback_speed(self.cfg.resume_speed);
        } else {
            port.set_playback_speed(self.cfg.catchup_speed);
            // Re-read the marker AFTER that write, and repair it if the
            // handover fired in between. Relying on the next step to repair
            // it is not enough: the window has no timing bound, and cave5
            // programs its next tick batch from playback_speed - so a few
            // ticks of the run could be issued at the catch-up rate,
            // which is exactly the exact-tick guarantee this is all for.
            //
            // This closes it rather than narrowing it. cave2 releases its
            // claim BEFORE installing the speed, so a marker still set at
            // this second read means cave2's own store has not happened yet
            // and will land after ours; a marker cleared by now means the
            // speed is ours to restore.
            if self.handoff_sent && !port.speed_handoff_pending() {
                port.set_playback_speed(self.cfg.resume_speed);
            }
        }
        match self.phase {
            Phase::Start => {
                port.set_playback_speed(self.cfg.catchup_speed);
                port.set_continue_from_frame(self.cfg.continue_from_frame);
                // STOP also clears this in the DLL. Clear it here as part of
                // the controller contract so even a delayed STOP cannot let
                // the previous PLAY's alignment leak into another arm.
                port.set_gate_align_rec(0);
                // Clear any handover left over from a previous cycle before
                // the replay position resets, so it cannot fire against the
                // wrong replay.
                port.set_speed_handoff(0, 0.0);
                self.handoff_sent = false;
                // Always Stop then settle a FIXED delay before Restart (as
                // restart_and_stabilize_inprocess does), so the Restart
                // fires at a consistent wall-clock phase → consistent (good)
                // F5 bucket.
                port.send_command(self.restart_stop_command());
                if port.command_idle() && port.mode() == TasMode::Off as u32 {
                    self.phase = Phase::StopSettle;
                    StepOutcome::Wait { ms: STOP_SETTLE_MS }
                } else {
                    self.phase = Phase::StopWaitAck;
                    StepOutcome::InProgress
                }
            }
            Phase::StopWaitAck => {
                // A wall-clock delay cannot serialize a command slot when
                // the game can hitch longer than that delay. Begin the
                // phase-pinning settle only after Stop is acknowledged.
                if port.command_idle() && port.mode() == TasMode::Off as u32 {
                    self.phase = Phase::StopSettle;
                    StepOutcome::Wait { ms: STOP_SETTLE_MS }
                } else {
                    StepOutcome::InProgress
                }
            }
            Phase::StopSettle => {
                // The settle wait elapsed (caller honoured the Wait), so cave2
                // has flipped to OFF. Fire the Restart now.
                port.reset_restart_state();
                port.send_command(TasCommand::Restart);
                self.phase = Phase::RestartWaitDone;
                StepOutcome::InProgress
            }
            Phase::RestartWaitDone => {
                if port.restart_state() == 2 {
                    port.reset_restart_state();
                    // Don't arm immediately — honour an arm settle so the arm
                    // phase (→ observed first-moving) matches the recording.
                    // Sweep the settle across rerolls so any recording aligns.
                    self.phase = Phase::ArmSettle;
                    let expected_fm = self
                        .cfg
                        .target
                        .and_then(|t| t.expected_first_moving)
                        .or_else(|| {
                            (self.cfg.gate_align_rec > 0).then_some(self.cfg.gate_align_rec)
                        });
                    StepOutcome::Wait {
                        ms: arm_settle_ms(self.retries_used(), expected_fm),
                    }
                } else {
                    StepOutcome::InProgress
                }
            }
            Phase::ArmSettle => {
                // cave2 reads these at ARM time — re-assert post-restart.
                port.set_continue_from_frame(self.cfg.continue_from_frame);
                port.set_gate_align_rec(self.cfg.gate_align_rec);
                port.set_playback_speed(self.cfg.catchup_speed);
                // Stage the speed handover BEFORE arming: the arm resets the
                // replay position to 0, and cave2 only tests the handover
                // inside the PLAY tick path, so there is no window where a
                // stale position could trip it.
                //
                // Position is first_moving + 1 - the first tick at which the
                // recording is actually moving. Handing over at the JUDGE
                // point (fm + BUCKET_MATCH_WINDOW) instead would fast-forward
                // through the opening 64 ticks of the run, which is the part
                // the user pressed PLAY to watch.
                let handoff_at = self
                    .cfg
                    .target
                    .and_then(|t| t.expected_first_moving)
                    .filter(|_| self.cfg.resume_speed > 0.0)
                    .map(|fm| fm.saturating_add(1));
                match handoff_at {
                    Some(pos) => {
                        port.set_speed_handoff(pos, self.cfg.resume_speed);
                        self.handoff_sent = true;
                    }
                    None => {
                        port.set_speed_handoff(0, 0.0);
                        self.handoff_sent = false;
                    }
                }
                // Snapshot the arm counter BEFORE the command goes out, so
                // the judge can tell this attempt's replay state from the
                // previous one's no matter how the polling lands.
                self.arm_generation_at_arm = port.arm_generation();
                port.send_command(self.cfg.arm.command());
                // CONT judges its bucket fingerprint. Gate-aligned PLAY still
                // has to watch the resulting trajectory: the gate fixes input
                // indexing, but it does not fully identify hidden spawn state.
                if self.cfg.target.is_some() || self.cfg.gate_align_rec > 0 {
                    self.phase = Phase::JudgeBucket;
                    StepOutcome::InProgress
                } else {
                    self.finish(CompletedVia::Unjudged)
                }
            }
            Phase::JudgeBucket => {
                // Until the DLL has processed the arm, the mode and position
                // being read still describe the PREVIOUS replay - so nothing
                // here can be concluded from them yet. This has to come
                // before EVERY mode interpretation, the REC one included: a
                // stale REC from before the arm would otherwise read as
                // "the splice already fired".
                if port.arm_generation() == self.arm_generation_at_arm {
                    return StepOutcome::InProgress;
                }
                let mode = port.mode();
                // Alignment covers CONT now, not just PLAY. For aligned
                // CONT the gate-relative watcher runs during the prefix
                // replay (mode == PLAY); the splice sits far past the
                // gate + BUCKET_MATCH_WINDOW window, so a hidden-state
                // divergence is rerolled before it can be spliced.
                let aligned = self.cfg.gate_align_rec > 0;
                if mode == rec {
                    // Splice already fired. For unaligned CONT that is the
                    // accept. For aligned CONT it means the prefix replayed
                    // clean past the watcher and reached the splice.
                    return self.finish(if aligned {
                        CompletedVia::BucketMatched
                    } else {
                        CompletedVia::Unjudged
                    });
                }
                // Past the arm, "not in PLAY" means the replay ENDED (or the
                // DLL refused the arm outright, which leaves it OFF forever).
                // Either way nothing more will arrive, so the judge's
                // KeepWaiting has to become terminal rather than InProgress -
                // a recording shorter than first_moving + BUCKET_MATCH_WINDOW
                // can never be ruled on and used to spin the cycle forever.
                let replay_ended = mode != play;
                if aligned {
                    let pos = port.playback_pos();
                    if pos == 0 && replay_ended {
                        self.phase = Phase::Aborted;
                        return StepOutcome::Aborted {
                            reason: "the DLL refused the aligned arm (nothing replayed)"
                                .to_string(),
                        };
                    }
                    // CONT: the watcher can only ever see the prefix up to
                    // the splice (the DLL parks playback there until the
                    // approval below), so cap the required depth at the
                    // splice-relative distance or the verdict could never
                    // complete. PLAY has no splice: full depth.
                    let max_depth_rel = if self.cfg.arm == Arm::Continue
                        && self.cfg.continue_from_frame > self.cfg.gate_align_rec
                    {
                        self.cfg.continue_from_frame - self.cfg.gate_align_rec
                    } else {
                        0
                    };
                    let verdict = judge_gate_aligned_play(
                        port.play_coords(),
                        port.rec_coords(),
                        port.recorded_count(),
                        pos,
                        port.gate_index(),
                        self.cfg.gate_align_rec,
                        port.capture_ok(),
                        max_depth_rel,
                    );
                    return match verdict {
                        BucketVerdict::KeepWaiting if replay_ended => self.reroll(
                            port,
                            "aligned replay ended before its watcher completed".to_string(),
                            None,
                        ),
                        BucketVerdict::KeepWaiting => StepOutcome::InProgress,
                        BucketVerdict::Match => {
                            // Aligned CONT: the DLL parks playback AT the
                            // splice until this approval lands. Writing it
                            // is the ONLY way the splice can fire, so a
                            // starved controller merely delays the splice,
                            // never lets it through unjudged. PLAY has
                            // nothing to approve.
                            if self.cfg.arm == Arm::Continue {
                                port.approve_cont_splice();
                            }
                            self.finish(CompletedVia::BucketMatched)
                        }
                        BucketVerdict::WrongBucket { observed } => self.reroll(
                            port,
                            format!(
                                "aligned trajectory mismatch at gate-relative frame {:?}",
                                observed
                            ),
                            observed,
                        ),
                        BucketVerdict::WrongStart => self.reroll(
                            port,
                            "aligned trajectory capture was incomplete".to_string(),
                            None,
                        ),
                        BucketVerdict::NoSignal => self.reroll(
                            port,
                            "recording has no gate-relative trajectory to watch".to_string(),
                            None,
                        ),
                    };
                }
                // THE PREDICTIVE REJECT. first_moving is decided by the arm
                // offset, not by anything that happens during the replay, so
                // once K is known this attempt's bucket is already determined
                // and a wrong one can be thrown away here — at zero replayed
                // ticks instead of after the whole countdown.
                //
                // It only ever REJECTS. A wrong prediction costs a reroll that
                // might have matched; it can never let a wrong bucket through,
                // because everything that survives still faces the full judge.
                //
                // It sits HERE, below the splice-success check and gated on
                // the replay still being live, rather than at the top of the
                // phase. Above them it could
                // reroll a CONT whose splice had already fired — sending STOP
                // and adding a duplicate segment boundary to a recording that
                // was already correctly spliced. A prediction must never be
                // able to overrule something that already happened.
                //
                // PLAY only, and never on the last retry:
                //
                // * CONT catches up at 256x, where the countdown it would skip
                //   is ~57ms of a ~1800ms reroll — a few percent, against a
                //   proven path that ends in a destructive splice. Not a trade
                //   worth making. The escape hatch is a flag; this is a floor.
                // * Reserving a retry guarantees at least one more attempt can
                //   actually replay and re-learn K. Without it a bad K can burn
                //   the remaining retries on predictions and abort the cycle
                //   having never observed anything to correct itself with.
                if self.cfg.predict_bucket
                    && self.cfg.arm == Arm::Play
                    && !replay_ended
                    && self.retries_remaining > 1
                {
                    if let (Some(k), Some(expected)) = (
                        self.learned_countdown_k,
                        self.cfg.target.and_then(|t| t.expected_first_moving),
                    ) {
                        if let Some(off) = arm_offset(port) {
                            let predicted = k - off;
                            if (predicted - expected as i64).abs() > COUNTDOWN_K_JITTER {
                                if self.blind_predictive_rejects >= MAX_BLIND_PREDICTIVE_REJECTS {
                                    // Nothing has replayed in a while, so
                                    // nothing has confirmed this K. Distrust
                                    // it rather than the buckets: drop it and
                                    // let this attempt replay and re-learn.
                                    self.learned_countdown_k = None;
                                    self.blind_predictive_rejects = 0;
                                } else {
                                    self.blind_predictive_rejects += 1;
                                    self.predictive_rejects += 1;
                                    return self.reroll(
                                            port,
                                            format!(
                                                "arm landed {} tick(s) after the restart, which puts first-moving at {} not {}",
                                                off, predicted, expected
                                            ),
                                            Some(predicted.max(0) as u32),
                                        );
                                }
                            }
                        }
                    }
                }
                let pos = port.playback_pos();
                if pos == 0 {
                    if replay_ended {
                        // The arm was processed, the mode is not PLAY, and
                        // not one tick replayed: the DLL REFUSED the arm
                        // (cave2 bounces ARM_CONTINUE from mid-run, for
                        // one). Nothing ran and nothing was judged, so this
                        // is a failure, not a quiet success - reporting it
                        // as Done left a refused CONT's 256x catch-up
                        // asserted with no splice ever coming to undo it.
                        self.phase = Phase::Aborted;
                        return StepOutcome::Aborted {
                            reason: "the DLL refused the arm (nothing replayed)".to_string(),
                        };
                    }
                    return StepOutcome::InProgress;
                }
                // Learn — or RE-learn — K from any attempt that gets far enough
                // to show its first-moving frame.
                //
                // Most recent observation wins, rather than the first: one odd
                // bucket then cannot poison the rest of the cycle. And ANY
                // observation clears the streak of unconfirmed rejections,
                // including when K was already known — otherwise a replay that
                // happened between rejections left the old streak standing and
                // dropped a K that had just been confirmed.
                //
                // Refuses to learn from a session where a coordinate capture
                // failed: the caller advances the index regardless, so the
                // prefix has a stale hole that reads as early movement.
                if port.capture_ok() {
                    if let (Some(o), Some(off)) = (
                        detect_first_moving(port.play_coords(), pos),
                        arm_offset(port),
                    ) {
                        self.learned_countdown_k = Some(o as i64 + off);
                        self.blind_predictive_rejects = 0;
                    }
                }
                let target = match self.cfg.target {
                    Some(t) => t,
                    // CONT with no fingerprint can't be judged — accept.
                    None => return self.finish(CompletedVia::Unjudged),
                };
                let verdict = judge_cont_bucket(
                    port.play_coords(),
                    port.rec_coords(),
                    port.recorded_count(),
                    pos,
                    target.expected_start_bits,
                    target.expected_first_moving,
                    // Validate as deep as CONT does.
                    //
                    // CONT passes its splice frame here. PLAY has no splice
                    // and used to pass continue_from_frame == 0, which the
                    // judge floors at first_moving + BUCKET_MATCH_WINDOW - so
                    // a bucket-matched PLAY stopped checking 64 ticks after
                    // the boarder moved while CONT kept checking for up to
                    // 1024. That is backwards: PLAY is watched end to end, so
                    // a late divergence is MORE visible there, not less.
                    // Passing the recording length makes both arms use the
                    // same rule (the judge still caps at
                    // fm + BUCKET_VALIDATE_WINDOW). Either way the bucket is
                    // validated clean through that depth before Match, so a
                    // late divergence rerolls instead of being accepted.
                    if self.cfg.continue_from_frame > 0 {
                        self.cfg.continue_from_frame
                    } else {
                        port.recorded_count()
                    },
                );
                match verdict {
                    BucketVerdict::KeepWaiting => {
                        if replay_ended {
                            // Ran out of replay before the judge could rule.
                            // Nothing more will arrive, so stop rather than
                            // spin.
                            self.finish(CompletedVia::Unjudged)
                        } else {
                            StepOutcome::InProgress
                        }
                    }
                    BucketVerdict::Match => self.finish(CompletedVia::BucketMatched),
                    // Accepted but NOT positively confirmed — flag it so the
                    // UI can warn that the resume may be on a near-miss bucket.
                    BucketVerdict::NoSignal => self.finish(CompletedVia::NoSignal),
                    BucketVerdict::WrongBucket { observed } => self.reroll(
                        port,
                        format!(
                            "bucket mismatch (observed first-moving={:?}, expected={:?})",
                            observed, target.expected_first_moving
                        ),
                        observed,
                    ),
                    BucketVerdict::WrongStart => {
                        self.reroll(port, "start mismatch".to_string(), None)
                    }
                }
            }
            Phase::Done => StepOutcome::Done {
                retries_used: self.retries_used(),
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
            retries_used: self.retries_used(),
            completed_via: via,
        }
    }

    fn restart_stop_command(&self) -> TasCommand {
        if self.cfg.gate_align_rec > 0 || self.cfg.target.is_some() {
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
        // Drop the staged handover FIRST, so the terminal abort below leaves
        // nothing armed either. A last-attempt failure returns early, and a
        // marker surviving that return is inherited by whatever arms next -
        // including a CONT, which would then take a PLAY's resume speed and
        // clock reset partway through its catch-up.
        port.set_speed_handoff(0, 0.0);
        self.handoff_sent = false;
        if self.retries_remaining == 0 {
            port.send_command(TasCommand::Stop);
            self.phase = Phase::Aborted;
            return StepOutcome::Aborted {
                reason: format!("{} after {} retries", detail, self.cfg.max_retries),
            };
        }
        self.retries_remaining -= 1;
        let attempt = self.cfg.max_retries - self.retries_remaining;
        port.set_continue_from_frame(self.cfg.continue_from_frame);
        port.set_gate_align_rec(0);
        port.set_playback_speed(self.cfg.catchup_speed);
        port.send_command(self.restart_stop_command());
        self.phase = Phase::StopWaitAck;
        // Jitter can elapse while Stop is in flight, but the fixed settle is
        // applied only after acknowledgement in StopWaitAck.
        StepOutcome::Reroll {
            attempt,
            suggested_delay_ms: cont_retry_jitter_ms(attempt),
            observed,
            expected: self.cfg.target.and_then(|t| t.expected_first_moving),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        speed_handoff_pos: u32,
        speed_after_handoff: f32,
        arm_generation: u32,
        arm_restart_tick: u32,
        arm_consumed_tick: u32,
        capture_ok_flag: bool,
        commands: Vec<TasCommand>,
        /// False by default: existing unit tests model immediate Cave2
        /// consumption. Set true to exercise a delayed acknowledgement.
        command_busy: bool,
        /// Invariant tracker: Restart must NEVER be sent while mode != OFF.
        restart_while_not_off: bool,
        /// Runs once, before the next `set_playback_speed` store lands: a
        /// stand-in for cave2 acting between the controller's read and its
        /// write.
        on_set_speed: Option<fn(&mut FakePort)>,
    }

    impl TransportPort for FakePort {
        fn send_command(&mut self, cmd: TasCommand) {
            if cmd == TasCommand::Restart && self.mode != TasMode::Off as u32 {
                self.restart_while_not_off = true;
            }
            // Stand in for cave2 processing the arm.
            if matches!(cmd, TasCommand::ArmPlay | TasCommand::ArmContinue) {
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
        fn set_playback_speed(&mut self, speed: f32) {
            if let Some(hook) = self.on_set_speed.take() {
                hook(self);
            }
            self.playback_speed = speed;
        }
        fn set_speed_handoff(&mut self, pos: u32, speed: f32) {
            self.speed_handoff_pos = pos;
            self.speed_after_handoff = speed;
        }
        fn speed_handoff_pending(&self) -> bool {
            self.speed_handoff_pos != 0
        }
        fn arm_generation(&self) -> u32 {
            self.arm_generation
        }
        fn arm_restart_tick(&self) -> u32 {
            self.arm_restart_tick
        }
        fn arm_consumed_tick(&self) -> u32 {
            self.arm_consumed_tick
        }
        fn capture_ok(&self) -> bool {
            self.capture_ok_flag
        }
    }

    impl FakePort {
        /// Stand in for cave2: when the replay reaches the staged handover
        /// position, drop the speed on that tick and clear the request.
        fn dll_tick(&mut self) {
            if self.speed_handoff_pos != 0 && self.playback_pos >= self.speed_handoff_pos {
                if self.speed_after_handoff > 0.0 {
                    self.playback_speed = self.speed_after_handoff;
                }
                self.speed_handoff_pos = 0;
            }
        }
    }

    fn cfg(arm: Arm, target: Option<BucketTarget>, max_retries: u32) -> ArmConfig {
        ArmConfig {
            arm,
            catchup_speed: 12.0,
            // Splice at 320 so the judge validates through exactly the
            // pos=320 these tests poll at (the controller passes this as
            // judge match_through). The deep validate-through-splice path
            // is covered by the judge's own tests in cont.rs.
            continue_from_frame: if arm == Arm::Continue { 320 } else { 0 },
            gate_align_rec: 0,
            target,
            max_retries,
            resume_speed: 0.0,
            predict_bucket: true,
        }
    }

    fn bits(x: f32, y: f32, z: f32) -> [u32; 3] {
        [x.to_bits(), y.to_bits(), z.to_bits()]
    }

    fn aligned_trajectory(
        rec_gate: usize,
        live_gate: usize,
        depth: usize,
    ) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
        let len = (rec_gate.max(live_gate) + depth + 16).max(400);
        let mut rec = vec![[0.0; 3]; len];
        let mut play = vec![[0.0; 3]; len];
        for k in 0..depth {
            let point = [k as f32 + 1.25, k as f32 * 0.5 + 2.0, -(k as f32)];
            rec[rec_gate + k] = point;
            play[live_gate + k] = point;
        }
        (play, rec)
    }

    #[test]
    fn aligned_play_judge_matches_a_shifted_gate_relative_trajectory() {
        // recorded_count 400, rec_gate 299 -> full depth is 101; the
        // watcher must see ALL of it before declaring Match.
        let (play, rec) = aligned_trajectory(299, 297, 101);
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 398, 297, 299, true, 0),
            BucketVerdict::Match
        );
    }

    #[test]
    fn aligned_play_judge_validates_past_the_settle_window() {
        // Exact for the first 64 gate-relative frames, divergent at 80:
        // the documented late-divergence bug. A 64-frame window accepted
        // this; the deep watcher must reject it.
        let (mut play, rec) = aligned_trajectory(299, 297, 101);
        play[297 + 80][2] += 0.5;
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 398, 297, 299, true, 0),
            BucketVerdict::WrongBucket { observed: Some(80) }
        );
    }

    #[test]
    fn aligned_cont_judge_depth_is_capped_at_the_splice() {
        // A CONT splicing 70 frames past the gate can only ever show the
        // watcher 70 frames (the DLL parks there) - Match must be
        // decidable from exactly that prefix.
        let (play, rec) = aligned_trajectory(299, 297, 70);
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 367, 297, 299, true, 70),
            BucketVerdict::Match
        );
    }

    #[test]
    fn aligned_play_judge_rejects_a_one_bit_hidden_state_difference() {
        let (mut play, rec) = aligned_trajectory(299, 297, 101);
        play[297][0] = f32::from_bits(play[297][0].to_bits() + 1);
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 298, 297, 299, true, 0),
            BucketVerdict::WrongBucket { observed: Some(0) }
        );
    }

    #[test]
    fn aligned_play_judge_waits_for_the_full_window_and_capture() {
        let (play, rec) = aligned_trajectory(299, 297, 101);
        // One frame short of the full 101-frame depth: keep waiting.
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 397, 297, 299, true, 0),
            BucketVerdict::KeepWaiting
        );
        assert_eq!(
            judge_gate_aligned_play(&play, &rec, 400, 398, 297, 299, false, 0),
            BucketVerdict::WrongStart
        );
    }

    #[test]
    fn rec_restart_serializes_stop_before_restart() {
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            command_busy: true,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Rec, None, 0));

        // Start publishes Stop but cannot start the settle until Cave2 acks.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands, vec![TasCommand::Stop]);
        // Even MODE_OFF is insufficient while the command slot still holds
        // Stop: Restart would overwrite the unacknowledged command.
        p.mode = TasMode::Off as u32;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands, vec![TasCommand::Stop]);
        p.command_busy = false;
        assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: STOP_SETTLE_MS });
        // Only after acknowledgement + the fixed settle does Restart fire.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.commands.last(), Some(&TasCommand::Restart));

        p.restart_state = 2;
        // RestartWaitDone → arm settle wait, then ArmSettle → ArmRec.
        assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: ARM_SETTLE_MS });
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
        let mut config = cfg(Arm::Play, None, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        // Even from OFF, always Stop + settle (the same restart
        // restart_and_stabilize_inprocess does; it lands hard buckets
        // reliably).
        assert_eq!(c.step(&mut p), StepOutcome::Wait { ms: STOP_SETTLE_MS });
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
        p.restart_state = 2;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Wait {
                ms: arm_settle_ms(0, Some(299))
            }
        );
        assert_eq!(
            p.gate_align_rec, 0,
            "restart must happen with alignment clear"
        );
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
                completed_via: CompletedVia::BucketMatched
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
        let mut config = cfg(Arm::Play, None, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);

        assert_eq!(
            c.step(&mut p),
            StepOutcome::Reroll {
                attempt: 1,
                suggested_delay_ms: 0,
                observed: Some(0),
                expected: None,
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
                completed_via: CompletedVia::BucketMatched
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
        let mut config = cfg(Arm::Play, None, 1);
        config.gate_align_rec = 299;
        let mut c = TransportController::new(config);
        drive_to_judge(&mut c, &mut p);
        p.mode = TasMode::Off as u32;

        assert!(matches!(
            c.step(&mut p),
            StepOutcome::Reroll {
                attempt: 1,
                observed: None,
                expected: None,
                ..
            }
        ));
    }

    /// Drive the controller through restart until it's armed CONT and in the
    /// JudgeBucket phase. Returns once ArmContinue has been sent.
    fn drive_to_judge(c: &mut TransportController, p: &mut FakePort) {
        // Start publishes Stop. Tests may begin in REC/PLAY, so model Cave2
        // consuming it before the deterministic settle begins.
        let first = c.step(p);
        p.mode = TasMode::Off as u32;
        p.command_busy = false;
        if first == StepOutcome::InProgress {
            assert_eq!(c.step(p), StepOutcome::Wait { ms: STOP_SETTLE_MS });
        }
        c.step(p);
        p.restart_state = 2;
        c.step(p); // RestartWaitDone -> ArmSettle (Wait)
        c.step(p); // ArmSettle -> arm command, phase -> JudgeBucket
                   // Assert against the configured arm, not a hardcoded ArmContinue:
                   // PLAY is judged too when it is given a target.
        assert_eq!(p.commands.last(), Some(&c.cfg.arm.command()));
        // game enters PLAY for the replay
        p.mode = TasMode::Play as u32;
    }

    /// Drive the reroll restart (after a Reroll outcome) up to re-arm.
    fn drive_reroll_to_judge(c: &mut TransportController, p: &mut FakePort) {
        p.mode = TasMode::Off as u32;
        p.command_busy = false;
        c.step(p); // StopWaitAck -> fixed settle wait
        c.step(p); // StopSettle -> Restart
        p.restart_state = 2;
        c.step(p); // RestartWaitDone -> ArmSettle (Wait)
        c.step(p); // ArmSettle -> ArmContinue
        p.mode = TasMode::Play as u32;
    }

    /// The whole point of the handover: the countdown replays fast, and the
    /// speed drops at the first moving frame - not at the judge point 64
    /// ticks later, which would fast-forward through the opening of the very
    /// run the user pressed PLAY to watch.
    #[test]
    fn play_hands_speed_back_at_the_first_moving_frame() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut a = cfg(Arm::Play, Some(target), 30);
        a.catchup_speed = 64.0;
        a.resume_speed = 1.0;
        let mut c = TransportController::new(a);
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        assert_eq!(
            p.speed_handoff_pos, 251,
            "handover staged at first_moving + 1"
        );
        assert_eq!(p.speed_after_handoff, 1.0);
        assert_eq!(p.playback_speed, 64.0);

        // The countdown replays at the catch-up speed.
        p.play_coords = coords.clone();
        p.playback_pos = 200;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        p.dll_tick();
        assert_eq!(p.playback_speed, 64.0);
        assert_eq!(p.speed_handoff_pos, 251, "not reached yet");

        // The DLL fires the handover on the tick it was asked for.
        p.playback_pos = 251;
        p.dll_tick();
        assert_eq!(p.speed_handoff_pos, 0);
        assert_eq!(p.playback_speed, 1.0);

        // REGRESSION: the controller re-asserts the catch-up speed on every
        // step. Left ungated it would undo the handover on the very next one
        // and the run would be watched at 64x after all.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(
            p.playback_speed, 1.0,
            "catch-up speed re-asserted after the handover fired"
        );

        p.playback_pos = 400;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::BucketMatched
            }
        );
        assert_eq!(p.playback_speed, 1.0);
    }

    /// CONT hands over at its splice (cont_resume_speed, inside the DLL), so
    /// it must stage nothing here and must keep its catch-up asserted for the
    /// whole replay.
    /// The UI also writes playback_speed every frame, and a judged PLAY's
    /// handover fires mid-replay with no mode change to notice it by. Rather
    /// than race, the UI steps aside for exactly this controller's lifetime -
    /// so "does this cycle own the speed" has to be answerable from the
    /// controller itself, not from a flag the UI keeps in step with it.
    #[test]
    fn only_a_handover_cycle_owns_the_speed() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut a = cfg(Arm::Play, Some(target), 30);
        a.resume_speed = 1.0;
        assert!(TransportController::new(a).owns_playback_speed());

        // CONT hands over at its splice instead, and REC never replays.
        assert!(
            !TransportController::new(cfg(Arm::Continue, Some(target), 30)).owns_playback_speed()
        );
        assert!(!TransportController::new(cfg(Arm::Rec, None, 0)).owns_playback_speed());
    }

    /// The check and the write are two operations, so cave2 can fire between
    /// them and the catch-up store lands last with nothing to correct it.
    /// Waiting for the next step to repair that is not enough: the window has
    /// no timing bound and cave5 programs its next tick batch from
    /// playback_speed, so the run can be issued fast for those ticks.
    #[test]
    fn a_handover_that_fires_mid_step_is_repaired_within_the_same_step() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut a = cfg(Arm::Play, Some(target), 30);
        a.catchup_speed = 64.0;
        a.resume_speed = 1.0;
        let mut c = TransportController::new(a);
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        p.play_coords = coords;
        assert_eq!(p.speed_handoff_pos, 251);

        // Next step: the handover fires DURING the controller's own speed
        // write - cave2 clears the claim and installs the resume speed, in
        // that order, BEFORE the controller's store lands. That is the
        // losing interleaving: the controller decided on the catch-up while
        // the marker was still set, and its store arrives last. Without the
        // second read the catch-up store wins and the run keeps
        // fast-forwarding until some later poll notices.
        p.on_set_speed = Some(|p: &mut FakePort| {
            p.playback_pos = p.speed_handoff_pos;
            p.dll_tick();
        });
        p.playback_pos = 200;
        c.step(&mut p);
        assert_eq!(
            p.playback_speed, 1.0,
            "a handover that fired mid-step was left overwritten"
        );
    }

    #[test]
    fn cont_stages_no_speed_handover() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        drive_to_judge(&mut c, &mut p);
        assert_eq!(p.speed_handoff_pos, 0);
        assert_eq!(p.speed_after_handoff, 0.0);

        // Something else knocks the speed down mid-replay (the in-process F5
        // restart does exactly this) - the controller must put it back.
        p.playback_speed = 1.0;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(p.playback_speed, 12.0);
    }

    /// A reroll re-arms from StopSettle, never through Phase::Start, so the
    /// staged handover has to be dropped in the reroll itself - otherwise it
    /// would fire against the next attempt at a position that means nothing.
    #[test]
    fn reroll_drops_the_staged_handover() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut a = cfg(Arm::Play, Some(target), 30);
        a.catchup_speed = 64.0;
        a.resume_speed = 1.0;
        let mut c = TransportController::new(a);
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        assert_eq!(p.speed_handoff_pos, 251);

        // This replay leaves the spawn one frame late: wrong bucket.
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[251] = [1.0, 2.0, 3.5];
        p.play_coords = play;
        p.playback_pos = 320;
        p.dll_tick();
        assert_eq!(p.speed_handoff_pos, 0, "fired during the failed attempt");

        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
        assert_eq!(
            p.speed_handoff_pos, 0,
            "no handover left armed by the reroll"
        );
        assert_eq!(p.speed_after_handoff, 0.0);
        assert_eq!(
            p.playback_speed, 64.0,
            "the next attempt replays its countdown fast again"
        );

        // Re-arming stages a fresh handover for the new attempt.
        drive_reroll_to_judge(&mut c, &mut p);
        assert_eq!(p.speed_handoff_pos, 251);
    }

    /// A recording shorter than first_moving + BUCKET_MATCH_WINDOW can never
    /// be judged. The judge answers KeepWaiting forever, and mapping that
    /// straight to InProgress left the cycle spinning with nothing left to
    /// arrive. Once the replay has ended, KeepWaiting has to be terminal.
    #[test]
    fn judge_stops_when_the_replay_ends_before_it_can_rule() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 260];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 260; // < 250 + BUCKET_MATCH_WINDOW
        drive_to_judge(&mut c, &mut p);

        p.play_coords = coords;
        p.playback_pos = 260;
        // Still replaying: keep waiting, the window might still be reached.
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);

        // Replay over. Nothing more will arrive.
        p.mode = TasMode::Off as u32;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
        assert!(c.is_terminal());
    }

    /// PLAY passes continue_from_frame == 0 to the judge, which floors the
    /// validation depth at first_moving + BUCKET_MATCH_WINDOW - so a PLAY used
    /// to stop checking 64 ticks after the boarder moved while CONT kept
    /// checking for up to 1024. A PLAY is watched end to end, so a late
    /// divergence is more visible there, not less.
    #[test]
    fn play_validates_as_deep_as_cont() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 800];
        for (k, item) in rec.iter_mut().enumerate().take(800).skip(250) {
            *item = [1.0, 2.0, 3.0 + (k - 249) as f32 * 0.01];
        }
        p.rec_coords = rec.clone();
        p.recorded_count = 800;
        drive_to_judge(&mut c, &mut p);

        // Same first-moving frame, same settle - then it veers off long after
        // the old window closed.
        let mut play = rec;
        for item in play.iter_mut().take(800).skip(500) {
            item[2] += 3.0;
        }
        p.play_coords = play;

        p.playback_pos = 320;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::InProgress,
            "accepted at first_moving + 64 instead of validating deeper"
        );

        p.playback_pos = 600;
        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
    }
    /// The controller and cave2 both write playback_speed, and the read that
    /// decides which speed to write is not part of the write. A step can read
    /// "handover still pending", have cave2 fire underneath it, and land its
    /// catch-up store AFTER the resume speed was installed - with the marker
    /// by then cleared, so nothing would ever look at it again. Going quiet
    /// once the handover fires is therefore not enough; the controller has to
    /// keep asserting the resume speed so a lost race is repaired.
    #[test]
    fn a_lost_speed_race_is_repaired_on_the_next_step() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut a = cfg(Arm::Play, Some(target), 30);
        a.catchup_speed = 64.0;
        a.resume_speed = 1.0;
        let mut c = TransportController::new(a);
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        p.play_coords = coords;

        p.playback_pos = 251;
        p.dll_tick();
        assert_eq!(p.playback_speed, 1.0);

        // The lost race: a catch-up store lands after the handover.
        p.playback_speed = 64.0;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
        assert_eq!(
            p.playback_speed, 1.0,
            "the controller must put the resume speed back, not just stay quiet"
        );
    }

    /// A final-attempt failure returns from reroll() early. A handover left
    /// staged across that return is inherited by whatever arms next -
    /// including a CONT, which would take a PLAY's resume speed and clock
    /// reset partway through its catch-up.
    #[test]
    fn abort_leaves_no_handover_armed() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut a = cfg(Arm::Play, Some(target), 0); // no retries left
        a.catchup_speed = 64.0;
        a.resume_speed = 1.0;
        let mut c = TransportController::new(a);
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        assert_eq!(p.speed_handoff_pos, 251);

        // Spawned somewhere else entirely: WrongStart, and no retries left.
        p.play_coords = vec![[9.0f32, 9.0, 9.0]; 400];
        p.playback_pos = 320;
        assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
        assert_eq!(p.speed_handoff_pos, 0, "abort left a handover armed");
        assert_eq!(p.speed_after_handoff, 0.0);
    }

    /// Mode is transient: a short replay at catch-up speed can begin and end
    /// entirely between two polls, so "we saw PLAY mode" is not a sound way to
    /// know the replay ran. Every later poll then reads OFF with no sighting
    /// and the cycle spins forever. The arm also zeroes playback_pos, and
    /// seeing THAT is durable.
    #[test]
    fn judge_finishes_when_play_mode_is_never_observed() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        // The whole replay ran between two polls: PLAY mode is never
        // sampled, and the only evidence it happened is that the arm counter
        // moved and the position is past the end.
        p.mode = TasMode::Off as u32;
        p.play_coords = coords;
        p.playback_pos = 400;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::BucketMatched
            }
        );
    }

    /// A refused arm (cave2 bounces ARM_CONTINUE from mid-run, for one) never
    /// enters PLAY and never replays anything, so the judge waits on a replay
    /// that will not happen. The arm counter counts refusals too, which is
    /// what turns that from a spin into a terminal outcome.
    ///
    /// And the outcome is ABORTED, not a quiet Done: nothing ran and nothing
    /// was judged. Reporting success left a refused CONT's 256x catch-up
    /// asserted with no splice ever coming to restore the speed - the UI only
    /// tears that down on Aborted.
    #[test]
    fn a_refused_arm_aborts_rather_than_reporting_success() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords;
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        // Refused: mode never became PLAY and nothing replayed.
        p.mode = TasMode::Off as u32;
        p.playback_pos = 0;
        assert!(matches!(c.step(&mut p), StepOutcome::Aborted { .. }));
        assert!(c.is_terminal());
    }
    /// Build a port whose restart/arm stamps put the arm `off` ticks after
    /// the restart — the only quantity that moves first_moving.
    fn arm_at_offset(p: &mut FakePort, off: u32) {
        p.arm_restart_tick = 1000;
        p.arm_consumed_tick = 1000 + off;
        p.capture_ok_flag = true;
    }

    /// THE POINT OF ALL THIS. Once the countdown length is known, a wrong
    /// bucket is thrown away at ZERO replayed ticks — no countdown, no
    /// coordinates, nothing. Before this, ruling on the fingerprint meant
    /// replaying ~300 ticks of a stationary boarder first.
    #[test]
    fn a_wrong_bucket_is_rejected_before_replaying_anything() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;

        // Attempt 1 armed 2 ticks after the restart, and replayed a bucket
        // that leaves the spawn at 248. That teaches K = 248 + 2 = 250.
        arm_at_offset(&mut p, 2);
        drive_to_judge(&mut c, &mut p);
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[248] = [1.0, 2.0, 3.5];
        p.play_coords = play;
        p.playback_pos = 320;
        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));

        // Attempt 2 lands its arm 10 ticks late, so first_moving is already
        // determined to be 240 — nowhere near 250. Nothing has been replayed:
        // no coordinates, position still 0.
        arm_at_offset(&mut p, 10);
        drive_reroll_to_judge(&mut c, &mut p);
        p.play_coords = Vec::new();
        p.playback_pos = 0;
        assert!(
            matches!(c.step(&mut p), StepOutcome::Reroll { .. }),
            "a determined-wrong bucket should not need to be replayed"
        );
    }

    /// ...but not before it has any idea what the countdown length is. A
    /// hardcoded K that was wrong for some other track or machine would
    /// reject every attempt and the cycle would never land, so the first
    /// attempt always replays and teaches it.
    #[test]
    fn nothing_is_predicted_until_the_countdown_length_is_known() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        p.recorded_count = 400;
        // An arm offset that WOULD be rejected if K were assumed.
        arm_at_offset(&mut p, 99);
        drive_to_judge(&mut c, &mut p);
        p.playback_pos = 0;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::InProgress,
            "rejected on a guess instead of replaying to learn K"
        );
    }

    /// The prediction is good to +/-1 because the countdown itself rounds by
    /// a tick. A bucket inside that band must survive to the real judge
    /// rather than being rerolled on arithmetic.
    #[test]
    fn a_bucket_within_the_countdown_jitter_still_gets_judged() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;

        arm_at_offset(&mut p, 2);
        drive_to_judge(&mut c, &mut p);
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[248] = [1.0, 2.0, 3.5];
        p.play_coords = play;
        p.playback_pos = 320;
        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. })); // K = 250

        // Offset 1 predicts 249 — one off, inside the jitter. Must replay.
        arm_at_offset(&mut p, 1);
        drive_reroll_to_judge(&mut c, &mut p);
        p.play_coords = Vec::new();
        p.playback_pos = 0;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);
    }
    /// A bucket that lands somewhere else entirely teaches a WRONG countdown
    /// length. A wrong K rejects good buckets at arm time, so they never
    /// replay, so nothing ever observes a first-moving frame to correct it —
    /// the cycle would burn every retry without replaying once. The guard
    /// treats a run of unconfirmed rejections as evidence against K.
    #[test]
    fn a_wrong_countdown_length_cannot_deadlock_the_cycle() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 40));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        arm_at_offset(&mut p, 2);

        // Attempt 1 replays a bucket that left the spawn at 100 — nothing like
        // the recording. K is learned as 100 + 2 = 102, which is wrong, and
        // now predicts ~100 for every arm offset we can reach.
        drive_to_judge(&mut c, &mut p);
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[100] = [1.0, 2.0, 3.5];
        p.play_coords = play;
        p.playback_pos = 320;
        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));

        // The bad K now rejects everything, blind — nothing replays, so
        // nothing can correct it.
        for i in 0..MAX_BLIND_PREDICTIVE_REJECTS {
            drive_reroll_to_judge(&mut c, &mut p);
            p.play_coords = Vec::new();
            p.playback_pos = 0;
            assert!(
                matches!(c.step(&mut p), StepOutcome::Reroll { .. }),
                "blind reject {} should still fire",
                i
            );
        }

        // ...until the streak is long enough to indict K instead. This
        // attempt must be allowed to replay and re-learn.
        drive_reroll_to_judge(&mut c, &mut p);
        p.play_coords = Vec::new();
        p.playback_pos = 0;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::InProgress,
            "a bad countdown length kept rejecting with nothing left to correct it"
        );
    }
    #[test]
    fn cont_matches_correct_bucket() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        // recording moves at frame 250
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        // replay reproduces it bit-for-bit → Match (pos past the 314 window)
        p.play_coords = coords;
        p.playback_pos = 320;

        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::BucketMatched
            }
        );
        assert!(c.is_terminal());
    }

    #[test]
    fn play_with_target_judges_and_matches() {
        // PLAY is judged whenever a target is supplied — the controller
        // gates on target.is_some(), not on the arm. This is what lets the
        // UI reroll a PLAY onto the recording's spawn bucket instead of
        // replaying whatever bucket the F5 happened to land.
        //
        // PLAY has no splice to validate through, so the controller passes
        // the recording length as the depth instead - the same rule CONT
        // uses for its splice, and still capped inside the judge at
        // first_moving + BUCKET_VALIDATE_WINDOW. So Match is only declared
        // once the replay has actually been validated that far.
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 400];
        coords[250] = [1.0, 2.0, 3.5];
        p.rec_coords = coords.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        p.play_coords = coords;
        // Clean, but not yet validated to the recording length.
        p.playback_pos = 320;
        assert_eq!(c.step(&mut p), StepOutcome::InProgress);

        p.playback_pos = 400;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::BucketMatched
            }
        );
    }

    #[test]
    fn play_with_target_rerolls_wrong_bucket() {
        // The near-miss the UI toggle exists to prevent: same spawn bits, but
        // the boarder starts moving on a different frame, so the replay
        // diverges from the recording. Must reroll, not accept.
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, Some(target), 30));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        // identical spawn, movement one frame late
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[251] = [1.0, 2.0, 3.5];
        p.play_coords = play;
        p.playback_pos = 320;
        assert!(matches!(c.step(&mut p), StepOutcome::Reroll { .. }));
        assert!(!c.is_terminal());
    }

    #[test]
    fn play_without_target_finishes_unjudged() {
        // The toggle OFF path, and the pre-existing behaviour: no target, so
        // the controller arms and is done — no judging, no rerolls, playback
        // starts immediately on whatever bucket the restart landed.
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Play, None, 30));
        c.step(&mut p); // Stop
        p.mode = TasMode::Off as u32;
        c.step(&mut p); // -> Restart
        c.step(&mut p);
        p.restart_state = 2;
        c.step(&mut p); // -> ArmSettle (Wait)
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::Unjudged
            }
        );
    }

    #[test]
    fn cont_no_signal_accept_is_flagged() {
        // Target has no first-moving frame (recording never moved, or it was
        // undetected): the bucket can't be positively judged, so it's
        // accepted as NoSignal — which the UI surfaces as an unconfirmed
        // resume (the likely cause of a "wrong-frame" resume).
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: None,
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        p.rec_coords = vec![[1.0f32, 2.0, 3.0]; 400];
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        // start bits match, but there is no movement to discriminate buckets.
        p.play_coords = vec![[1.0f32, 2.0, 3.0]; 400];
        p.playback_pos = 320;
        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 0,
                completed_via: CompletedVia::NoSignal
            }
        );
    }

    #[test]
    fn cont_wrong_bucket_rerolls_then_matches() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        let mut right = vec![[1.0f32, 2.0, 3.0]; 400];
        right[250] = [1.0, 2.0, 3.5];
        p.rec_coords = right.clone();
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);

        // wrong bucket: moves at 248, not 250 → diverges from rec
        let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
        wrong[248] = [1.0, 2.0, 3.5];
        p.play_coords = wrong;
        p.playback_pos = 320;

        match c.step(&mut p) {
            StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
            other => panic!("expected Reroll, got {:?}", other),
        }
        // reroll sent Stop and is waiting for OFF again
        assert_eq!(p.commands.last(), Some(&TasCommand::StopForRestart));

        // complete the reroll restart, this time reproducing the recording
        drive_reroll_to_judge(&mut c, &mut p);
        p.play_coords = right;
        p.playback_pos = 320;

        assert_eq!(
            c.step(&mut p),
            StepOutcome::Done {
                retries_used: 1,
                completed_via: CompletedVia::BucketMatched
            }
        );
        assert!(!p.restart_while_not_off, "Restart sent while mode != OFF!");
    }

    #[test]
    fn cont_aborts_after_exhausting_retries() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        // only 1 retry allowed
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 1));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
        wrong[248] = [1.0, 2.0, 3.5];

        drive_to_judge(&mut c, &mut p);
        p.play_coords = wrong.clone();
        p.playback_pos = 320;
        // first wrong bucket -> reroll (attempt 1, uses the only retry)
        match c.step(&mut p) {
            StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
            other => panic!("expected Reroll, got {:?}", other),
        }
        drive_reroll_to_judge(&mut c, &mut p);
        p.play_coords = wrong;
        p.playback_pos = 320;
        // second wrong bucket -> no retries left -> abort
        match c.step(&mut p) {
            StepOutcome::Aborted { reason } => assert!(reason.contains("bucket mismatch")),
            other => panic!("expected Aborted, got {:?}", other),
        }
        assert!(c.is_terminal());
    }

    #[test]
    fn wrong_start_rerolls() {
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        drive_to_judge(&mut c, &mut p);
        // spawn position doesn't match the recording's start
        p.play_coords = vec![[9.0f32, 9.0, 9.0]; 300];
        p.playback_pos = 260;
        match c.step(&mut p) {
            StepOutcome::Reroll { attempt, .. } => assert_eq!(attempt, 1),
            other => panic!("expected Reroll for wrong start, got {:?}", other),
        }
    }

    #[test]
    fn cont_drives_splice_frame_to_the_game() {
        // "Continue must continue on the correct frame": the controller has
        // to push continue_from_frame (the splice frame) into the game both
        // before arming AND re-assert it on every reroll, or cave2 splices
        // at the wrong tick. cfg() sets continue_from_frame = 320 for CONT.
        let target = BucketTarget {
            expected_start_bits: bits(1.0, 2.0, 3.0),
            expected_first_moving: Some(250),
        };
        let mut p = FakePort {
            mode: TasMode::Rec as u32,
            ..Default::default()
        };
        let mut c = TransportController::new(cfg(Arm::Continue, Some(target), 30));
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        p.rec_coords = rec;
        p.recorded_count = 400;
        drive_to_judge(&mut c, &mut p);
        assert_eq!(
            p.continue_from_frame, 320,
            "splice frame not handed to the game before arming"
        );

        // Force a wrong bucket so the controller rerolls, after corrupting
        // the frame — the reroll must restore it.
        let mut wrong = vec![[1.0f32, 2.0, 3.0]; 400];
        wrong[248] = [1.0, 2.0, 3.5];
        p.play_coords = wrong;
        p.playback_pos = 320;
        p.continue_from_frame = 0;
        match c.step(&mut p) {
            StepOutcome::Reroll { .. } => {}
            other => panic!("expected Reroll, got {:?}", other),
        }
        assert_eq!(
            p.continue_from_frame, 320,
            "splice frame not re-asserted on reroll"
        );
    }

    #[test]
    fn jitter_zero_early_then_bounded() {
        // First NATURAL_RESTART_ATTEMPTS use no jitter (natural variance).
        for a in 1..=NATURAL_RESTART_ATTEMPTS {
            assert_eq!(
                cont_retry_jitter_ms(a),
                0,
                "attempt {} should not jitter",
                a
            );
        }
        // After that, a bounded non-zero escape jitter.
        for a in (NATURAL_RESTART_ATTEMPTS + 1)..=40 {
            let j = cont_retry_jitter_ms(a);
            assert!((1..=17).contains(&j), "jitter {} out of range", j);
        }
    }
}
