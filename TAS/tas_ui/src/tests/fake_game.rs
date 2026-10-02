//! A deterministic stand-in for TAS_Helper.dll and the game, for the headless
//! frame tests. It models the shared-memory protocol the UI observes
//! (DESIGN.md "The transport cycle", "CONT: replay, check, then record",
//! "Controller ownership"; `caves/cycle_cave.hpp`), not the game: a toy rider
//! whose position integrates its held keys, a countdown that fixes the gate,
//! and the DLL's command, arm, splice and stop transitions. HELD takes only;
//! anything it does not model panics.
//!
//! Positions are multiples of 0.25 and exact. A replay derives its positions
//! from the input it plays, never from the recording, and nothing here
//! touches `rec_coords` outside REC, so an edited take's stale trajectory
//! stays for the UI to deal with.

use std::sync::atomic::Ordering;

use tas_shared::input_bits::{DOWN, JUMP, LEFT, RIGHT, UP};
use tas_shared::*;

/// Where every restart puts the rider (classifies as no known track).
pub const SPAWN: [f32; 3] = [100.0, 0.0, 0.0];
/// Physics ticks per second at 1x.
const TICK_HZ: f64 = 100.0;
/// The countdown: physics ticks from a restart to the rider's release.
const RELEASE_TICKS: u32 = 60;
/// Ticks an F5 restart takes.
const RESTART_TICKS: u32 = 5;
pub const FE: u32 = 0;
pub const VE: u32 = 6;

pub type Hook = Box<dyn FnOnce(&mut FakeGame)>;

/// The mapping. `get` borrows this field mutably, so two live borrows of
/// the state cannot overlap while the fake's other fields stay usable. One
/// thread; the UI holds no borrow of it across a frame or a wait.
struct Mapping(*mut TasSharedState);

impl Mapping {
    fn get(&mut self) -> &mut TasSharedState {
        // SAFETY: the mapping outlives the fake (the harness owns both).
        unsafe { &mut *self.0 }
    }
}

pub struct FakeGame {
    map: Mapping,

    // Script.
    /// Ticks a command waits in the slot before the DLL takes it.
    pub delays: Vec<(TasCommand, u64)>,
    /// The physical keys at each REC index.
    pub keyboard: Box<dyn Fn(u32) -> u8>,
    /// The finish line, as a rider X.
    pub finish_x: Option<f32>,
    /// A replay fault: the captured replay X at this gate-relative tick is
    /// off by 0.25.
    pub perturb_at: Option<u32>,
    /// Runs once, as the next STOP is taken.
    pub on_stop: Option<Hook>,

    // What it saw.
    /// Every command it took, in order.
    pub taken: Vec<TasCommand>,
    /// The live gate of each PLAY/CONT arm.
    pub live_gates: Vec<u32>,
    /// `input_log` as it stood when each STOP was taken.
    pub logs_at_stop: Vec<Vec<u8>>,

    // Rider.
    pos: [f32; 3],
    since_reset: u32,
    clock: f32,

    // DLL.
    ticks: u64,
    owed: f64,
    restart_left: u32,
    cont_armed: bool,
    predicted_gate: u32,
    waiting: Option<(u32, u64)>,
    owner_seq_seen: u32,
    /// What `input_log[..recorded_count]` must hold while a mode runs: the
    /// UI may not write the take the DLL is replaying or recording.
    take: Vec<u8>,
}

impl FakeGame {
    /// A fake over `state`, in a ticking FE race at the spawn: Vincent,
    /// regular, OpenGL / 53-bit.
    pub fn new(state: *mut TasSharedState) -> Self {
        let mut game = Self {
            map: Mapping(state),
            delays: Vec::new(),
            keyboard: Box::new(|_| 0),
            finish_x: None,
            perturb_at: None,
            on_stop: None,
            taken: Vec::new(),
            live_gates: Vec::new(),
            logs_at_stop: Vec::new(),
            pos: SPAWN,
            since_reset: 0,
            clock: 0.0,
            ticks: 0,
            owed: 0.0,
            restart_left: 0,
            cont_armed: false,
            predicted_gate: 0,
            waiting: None,
            owner_seq_seen: 0,
            take: Vec::new(),
        };
        let s = game.map.get();
        s.game_in_game = 1;
        s.renderer_id = TAS_RENDERER_OPENGL;
        s.fpu_control_word = 0x027F;
        s.rider_character = TAS_CHARACTER_VINCENT;
        s.rider_stance = TAS_STANCE_REGULAR;
        s.race_clock_bits = u32::MAX;
        game.set_level(FE);
        game
    }

    /// Publish track `id` as resolved, as the launch hook does.
    pub fn set_level(&mut self, id: u32) {
        let s = self.map.get();
        s.level_ctx_seq.fetch_add(1, Ordering::AcqRel);
        s.level_epoch += 1;
        s.level_scan_epoch = s.level_epoch;
        s.level_id = id;
        s.level_ctx_seq.fetch_add(1, Ordering::AcqRel);
    }

    /// The track becomes unknown (a context change the scan has not
    /// identified).
    pub fn unresolve_level(&mut self) {
        let s = self.map.get();
        s.level_ctx_seq.fetch_add(1, Ordering::AcqRel);
        s.level_epoch += 1;
        s.level_ctx_seq.fetch_add(1, Ordering::AcqRel);
    }

    pub fn set_rider(&mut self, character: u32, stance: u32) {
        let s = self.map.get();
        s.rider_seq.fetch_add(1, Ordering::AcqRel);
        s.rider_character = character;
        s.rider_stance = stance;
        s.rider_seq.fetch_add(1, Ordering::AcqRel);
    }

    pub fn set_fpu_control_word(&mut self, word: u32) {
        self.map.get().fpu_control_word = word;
    }

    /// Run `d` of game time: one message-pump pass, then the physics ticks
    /// it is owed at the current playback speed.
    pub fn advance(&mut self, d: std::time::Duration) {
        self.pump();
        let speed = self.map.get().playback_speed;
        let speed = if speed.is_finite() && speed > 0.0 {
            speed as f64
        } else {
            1.0
        };
        self.owed += d.as_secs_f64() * TICK_HZ * speed;
        while self.owed >= 1.0 {
            if self.parked() {
                self.owed = 0.0;
                break;
            }
            self.owed -= 1.0;
            self.tick();
        }
    }

    /// Ownership requests and the commands the pump takes while the cycle may
    /// be frozen (STOP; RESTART in a race).
    fn pump(&mut self) {
        let seq = self.map.get().owner_request_seq.load(Ordering::Acquire);
        if seq != self.owner_seq_seen {
            self.owner_seq_seen = seq;
            let s = self.map.get();
            let (kind, pid) = (s.owner_request_kind, s.owner_request_pid);
            s.owner_result = match kind {
                TAS_OWNER_ACQUIRE if s.owner_pid == 0 || s.owner_pid == pid => {
                    s.owner_pid = pid;
                    TAS_OWNER_RESULT_OWNED
                }
                TAS_OWNER_ACQUIRE => TAS_OWNER_RESULT_BUSY,
                TAS_OWNER_RELEASE if s.owner_pid == pid => {
                    s.owner_pid = 0;
                    s.cont_suppress_input = 0;
                    TAS_OWNER_RESULT_RELEASED
                }
                TAS_OWNER_RELEASE => TAS_OWNER_RESULT_NOT_OWNER,
                other => panic!("fake game: unsupported ownership request {other}"),
            };
            s.owner_ack_seq.store(seq, Ordering::Release);
        }
        if let Some(cmd) = self.take_command(|c| {
            c == TasCommand::Stop as u32
                || c == TasCommand::StopForRestart as u32
                || c == TasCommand::Restart as u32
        }) {
            self.execute(cmd);
        }
    }

    /// The command in the slot, once it has waited out its delay and
    /// `wanted` accepts it.
    fn take_command(&mut self, wanted: impl Fn(u32) -> bool) -> Option<u32> {
        let cmd = self.map.get().command;
        if cmd == TasCommand::Idle as u32 {
            self.waiting = None;
            return None;
        }
        if !wanted(cmd) {
            return None;
        }
        let since = match self.waiting {
            Some((c, at)) if c == cmd => at,
            _ => {
                self.waiting = Some((cmd, self.ticks));
                self.ticks
            }
        };
        let delay = self
            .delays
            .iter()
            .find(|(c, _)| *c as u32 == cmd)
            .map_or(0, |(_, d)| *d);
        if self.ticks - since < delay {
            return None;
        }
        self.waiting = None;
        Some(cmd)
    }

    fn execute(&mut self, cmd: u32) {
        let command = [
            TasCommand::ArmRec,
            TasCommand::ArmPlay,
            TasCommand::Stop,
            TasCommand::ArmContinue,
            TasCommand::Restart,
            TasCommand::StopForRestart,
        ]
        .into_iter()
        .find(|c| *c as u32 == cmd)
        .unwrap_or_else(|| panic!("fake game: unsupported command {cmd}"));
        self.taken.push(command);
        match command {
            TasCommand::Stop | TasCommand::StopForRestart => {
                self.check_take();
                let s = self.map.get();
                let log = s.input_log[..s.recorded_count as usize].to_vec();
                self.logs_at_stop.push(log);
                if let Some(hook) = self.on_stop.take() {
                    hook(self);
                }
                self.stop(command == TasCommand::StopForRestart);
            }
            TasCommand::Restart => {
                if self.map.get().mode != TasMode::Off as u32 {
                    let protect = self.map.get().cont_suppress_input != 0;
                    self.stop(protect);
                }
                self.map.get().restart_state = 1;
                self.restart_left = RESTART_TICKS;
                self.clear_gate_align();
                self.log("Cycle cave: in-process F5 restart initiated");
            }
            TasCommand::ArmRec | TasCommand::ArmPlay | TasCommand::ArmContinue => {
                self.arm(command);
                let s = self.map.get();
                self.predicted_gate = 0;
                s.gate_index = 0;
                s.capture_ok = 1;
                s.arm_generation += 1;
            }
            _ => unreachable!(),
        }
        self.map.get().command = TasCommand::Idle as u32;
    }

    fn arm(&mut self, command: TasCommand) {
        let s = self.map.get();
        s.playback_pos = 0;
        match command {
            TasCommand::ArmRec => {
                s.recorded_count = 0;
                s.input_model = TAS_INPUT_MODEL_HELD;
                s.segment_count = 1;
                s.segment_start_frame = 0;
                s.mode = TasMode::Rec as u32;
                self.take.clear();
                self.cont_armed = false;
                self.clear_gate_align();
                self.log("Cycle cave: entering REC mode");
            }
            TasCommand::ArmPlay => {
                s.continue_from_frame = 0;
                s.mode = TasMode::Play as u32;
                self.take = s.input_log[..s.recorded_count as usize].to_vec();
                let recorded = s.recorded_count;
                self.cont_armed = false;
                self.log(&format!(
                    "Cycle cave: entering PLAY mode ({recorded} ticks recorded)"
                ));
            }
            TasCommand::ArmContinue => {
                let refusal =
                    if s.continue_from_frame == 0 || s.continue_from_frame > s.recorded_count {
                        Some("ARM_CONTINUE: invalid splice point")
                    } else if s.mode != TasMode::Off as u32 {
                        Some("ARM_CONTINUE: refused - game is REC/PLAY")
                    } else {
                        None
                    };
                if let Some(reason) = refusal {
                    s.mode = TasMode::Off as u32;
                    s.continue_from_frame = 0;
                    self.log(reason);
                    self.cont_armed = false;
                    self.clear_gate_align();
                    return;
                }
                s.mode = TasMode::Play as u32;
                s.cont_splice_approved = 0;
                self.take = s.input_log[..s.recorded_count as usize].to_vec();
                let from = s.continue_from_frame;
                self.cont_armed = true;
                self.log(&format!(
                    "Cycle cave: continue record (PLAY until frame {from})"
                ));
            }
            _ => unreachable!(),
        }
    }

    /// ApplyStopTransition.
    fn stop(&mut self, protect_restart: bool) {
        let s = self.map.get();
        s.cont_suppress_input = protect_restart as u32;
        s.mode = TasMode::Off as u32;
        if s.restart_state == 1 {
            s.restart_state = 0;
        }
        s.continue_from_frame = 0;
        self.cont_armed = false;
        self.clear_gate_align();
        self.log("Cycle cave: stopped");
    }

    fn clear_gate_align(&mut self) {
        let s = self.map.get();
        s.gate_align_rec = 0;
        s.cont_splice_approved = 0;
        self.predicted_gate = 0;
    }

    /// The UI must not have rewritten the take a mode is running.
    fn check_take(&mut self) {
        let s = self.map.get();
        if s.mode == TasMode::Off as u32 {
            return;
        }
        let n = s.recorded_count as usize;
        assert!(
            n == self.take.len() && s.input_log[..n] == self.take[..],
            "the UI rewrote the take while {} ran it",
            s.mode_str()
        );
    }

    fn live_gate(predicted: u32, s: &TasSharedState) -> u32 {
        if predicted != 0 {
            predicted
        } else {
            s.gate_index
        }
    }

    /// GateAlignedSplicePos; None while pending.
    fn splice_pos(predicted: u32, s: &TasSharedState) -> Option<u32> {
        let (from, rec_gate) = (s.continue_from_frame, s.gate_align_rec);
        if rec_gate == 0 || from <= rec_gate {
            return Some(from);
        }
        let live = Self::live_gate(predicted, s);
        (live != 0).then(|| live + (from - rec_gate))
    }

    /// A CONT at its splice waiting for approval runs no ticks.
    fn parked(&mut self) -> bool {
        let predicted = self.predicted_gate;
        let s = self.map.get();
        self.cont_armed
            && s.mode == TasMode::Play as u32
            && s.gate_align_rec != 0
            && s.cont_splice_approved == 0
            && Self::splice_pos(predicted, s).is_some_and(|at| s.playback_pos >= at)
    }

    /// One Supreme::Cycle: CycleCave_Logic, then the rider's physics.
    fn tick(&mut self) {
        self.ticks += 1;
        self.map.get().frame_count += 1;
        if let Some(cmd) = self.take_command(|_| true) {
            self.execute(cmd);
        }
        if self.map.get().restart_state == 1 {
            self.restart_left = self.restart_left.saturating_sub(1);
            if self.restart_left == 0 {
                self.map.get().restart_state = 2;
                self.pos = SPAWN;
                self.since_reset = 0;
                self.clock = 0.0;
                self.log("Cycle cave: restart complete (the game rebuilt the level)");
            }
        }
        self.check_take();
        let mode = self.map.get().mode;
        if mode == TasMode::Play as u32 {
            self.complete_splice();
        }
        let (mask, exec) = match self.map.get().mode {
            m if m == TasMode::Rec as u32 => (self.rec_tick(), Some(TasMode::Rec)),
            m if m == TasMode::Play as u32 => match self.play_tick() {
                Some(mask) => (mask, Some(TasMode::Play)),
                None => (0, None),
            },
            _ => (0, None),
        };
        let exec_tick = match exec {
            Some(TasMode::Rec) => self.map.get().recorded_count - 1,
            _ => self.map.get().playback_pos.saturating_sub(1),
        };
        self.physics(mask, exec.map(|m| (m, exec_tick)));
    }

    fn rec_tick(&mut self) -> u8 {
        let s = self.map.get();
        let index = s.recorded_count;
        let mask = (self.keyboard)(index);
        s.input_log[index as usize] = mask;
        self.capture(index, true);
        self.map.get().recorded_count = index + 1;
        self.take.push(mask);
        mask
    }

    /// PlayTick; None when the replay just ended.
    fn play_tick(&mut self) -> Option<u8> {
        let s = self.map.get();
        assert_eq!(
            s.input_model, TAS_INPUT_MODEL_HELD,
            "fake game: only HELD takes replay"
        );
        let pos = s.playback_pos;
        if s.gate_align_rec > 0 && pos == 0 && self.since_reset < RELEASE_TICKS {
            // The countdown fixes the gate: the first capture after release.
            self.predicted_gate = RELEASE_TICKS - self.since_reset + 1;
            self.live_gates.push(self.predicted_gate);
        }
        let predicted = self.predicted_gate;
        let s = self.map.get();
        let live_gate = Self::live_gate(predicted, s);
        let (rec_gate, count) = (s.gate_align_rec, s.recorded_count);
        let play_end = if rec_gate > 0 && live_gate > 0 && count > rec_gate {
            live_gate + (count - rec_gate)
        } else {
            count
        };
        if pos >= play_end {
            s.mode = TasMode::Off as u32;
            self.cont_armed = false;
            self.clear_gate_align();
            self.log(&format!("Cycle cave: playback complete at frame {pos}"));
            return None;
        }
        let src = if rec_gate == 0 || rec_gate >= count {
            Some(pos)
        } else if live_gate == 0 {
            panic!("fake game: a replay armed after the release has no gate prediction")
        } else {
            (pos + rec_gate).checked_sub(live_gate)
        };
        let mask = src
            .filter(|&i| i < count)
            .map_or(0, |i| s.input_log[i as usize]);
        self.capture(pos, false);
        self.map.get().playback_pos = pos + 1;
        self.complete_splice();
        Some(mask)
    }

    /// CompleteContinueSplice: switch an approved CONT to REC at its splice.
    fn complete_splice(&mut self) {
        let predicted = self.predicted_gate;
        let s = self.map.get();
        let ready = self.cont_armed
            && s.continue_from_frame > 0
            && Self::splice_pos(predicted, s).is_some_and(|at| s.playback_pos >= at)
            && (s.gate_align_rec == 0 || s.cont_splice_approved != 0);
        if !ready {
            return;
        }
        let splice = s.continue_from_frame;
        s.recorded_count = splice;
        s.cont_suppress_input = 0;
        if s.cont_resume_speed > 0.0 {
            s.playback_speed = s.cont_resume_speed;
        }
        if (s.segment_count as usize) < TAS_MAX_SEGMENTS {
            s.segment_boundaries[s.segment_count as usize].frame = splice;
            s.segment_count += 1;
        }
        s.segment_start_frame = splice;
        s.mode = TasMode::Rec as u32;
        s.continue_from_frame = 0;
        self.cont_armed = false;
        self.take = s.input_log[..splice as usize].to_vec();
        // The splice drains the catch-up backlog.
        self.owed = 0.0;
        self.log(&format!("Cycle cave: spliced to REC at frame {splice}"));
    }

    fn capture(&mut self, index: u32, rec: bool) {
        let predicted = self.predicted_gate;
        let s = self.map.get();
        let mut at = self.pos;
        let live_gate = Self::live_gate(predicted, s);
        if !rec
            && self
                .perturb_at
                .is_some_and(|k| Some(index) == k.checked_add(live_gate))
        {
            at[0] += 0.25;
        }
        if rec {
            s.rec_coords[index as usize] = at;
        } else {
            s.play_coords[index as usize] = at;
        }
        let first = if rec {
            s.rec_coords[0]
        } else {
            s.play_coords[0]
        };
        if s.gate_index == 0 && index > 0 && at != first {
            s.gate_index = index;
            if self.predicted_gate != 0 && index != self.predicted_gate {
                s.capture_ok = 0;
            }
        }
    }

    /// The toy rider: frozen through the countdown, then 1 forward per tick,
    /// steered by its held keys. Exact in f32.
    fn physics(&mut self, mask: u8, exec: Option<(TasMode, u32)>) {
        let moving = self.since_reset >= RELEASE_TICKS;
        self.since_reset += 1;
        if !moving {
            return;
        }
        let key = |bit: u8| if mask & bit != 0 { 1.0 } else { 0.0 };
        let before = self.pos[0];
        self.pos[0] += 1.0 + 0.25 * (key(UP) - key(DOWN));
        self.pos[1] = key(JUMP);
        self.pos[2] += 0.5 * (key(RIGHT) - key(LEFT));
        self.clock += 0.01;
        let crossed = self
            .finish_x
            .is_some_and(|x| before < x && self.pos[0] >= x);
        if let (true, Some((mode, tick))) = (crossed, exec) {
            let s = self.map.get();
            s.race_finish_seq.fetch_add(1, Ordering::AcqRel);
            s.race_finish_tick = tick;
            s.race_finish_mode = mode as u32;
            s.race_finish_valid = 1;
            s.race_finish_time_bits = self.clock.to_bits();
            s.race_finish_seq.fetch_add(1, Ordering::AcqRel);
        }
    }

    /// A line in the DLL's log ring.
    fn log(&mut self, text: &str) {
        let s = self.map.get();
        let seq = s.log_write_seq;
        let entry = &mut s.log_ring[seq as usize % TAS_LOG_RING_SIZE];
        entry.severity = TasLogSeverity::Info as u32;
        entry.text = [0; TAS_LOG_ENTRY_SIZE];
        let n = text.len().min(TAS_LOG_ENTRY_SIZE - 1);
        entry.text[..n].copy_from_slice(&text.as_bytes()[..n]);
        entry.sequence = seq + 1;
        s.log_write_seq = seq + 1;
    }
}
