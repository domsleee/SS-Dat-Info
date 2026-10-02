//! Recording sessions: the take being recorded (REC or CONT, where it
//! started, how long it got), what the next REC will be once the cycle arms
//! it, and the finish-line watch that stops it. The app starts a session when
//! it sees REC begin, checkpoints it while it records, and finalizes it into
//! history when REC ends.

use tas_shared::{TasMode, TasSharedState};

use crate::history_runtime::CheckpointOwner;
use crate::recording::{self, RecordingSessionKind};
use crate::TasApp;

/// The take being recorded.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ActiveSession {
    pub(crate) kind: RecordingSessionKind,
    pub(crate) start_tick: u32,
    pub(crate) max_recorded_count: u32,
}

impl ActiveSession {
    pub(crate) fn owner(&self) -> CheckpointOwner {
        CheckpointOwner {
            kind: self.kind,
            start_tick: self.start_tick,
            max_recorded_count: self.max_recorded_count,
        }
    }
}

/// What a queued cycle will record once the UI sees REC: the DLL may have
/// cleared its splice by then.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum StartContext {
    Rec,
    Continue { splice: u32 },
}

/// Finish-line watch: the DLL's finish count the take started from, and the
/// tick it finished in (drives the auto-stop + the 🏁 marker).
#[derive(Default)]
pub(crate) struct FinishWatch {
    pub(crate) seen_seq: u32,
    /// A cycle took `seen_seq` when it started; the REC it arms keeps it
    /// (the controller may be gone by the time the UI sees REC).
    pub(crate) baseline_armed: bool,
    pub(crate) at_tick: Option<u32>,
    /// The HUD race time latched when the crossing was detected. Re-reading
    /// the live timer at STOP is unreliable: it may have blanked or moved.
    pub(crate) hud_cs: Option<u32>,
}

impl FinishWatch {
    /// A new take starts: forget the last finish. `seen_seq` is the baseline
    /// unless a cycle already took one.
    pub(crate) fn restart(&mut self, live_seq: u32) {
        if !std::mem::take(&mut self.baseline_armed) {
            self.seen_seq = live_seq;
        }
        self.at_tick = None;
        self.hud_cs = None;
    }

    /// A fresh recording is about to load: clear the finish-flag marker.
    pub(crate) fn clear(&mut self, live_seq: u32) {
        self.at_tick = None;
        self.hud_cs = None;
        self.seen_seq = live_seq;
    }

    /// A REC finish after the baseline, once: the log line to show.
    pub(crate) fn observe(&mut self, state: &TasSharedState) -> Option<String> {
        if self.at_tick.is_some() {
            return None;
        }
        let finish = tas_shared::race_clock::race_finish(state)?;
        if finish.seq == self.seen_seq || finish.mode != TasMode::Rec as u32 {
            return None;
        }
        self.at_tick = Some(finish.tick);
        self.hud_cs = Some(tas_shared::race_clock::hud_cs(
            finish.seconds,
            tas_shared::race_clock::extended_precision(state.fpu_control_word),
        ));
        let time = match self.hud_cs {
            Some(cs) if finish.valid => {
                format!(" (race time {})", recording::format_recording_duration(cs))
            }
            _ => " (a checkpoint was missed: no official time)".to_string(),
        };
        if !finish.valid {
            self.hud_cs = None;
        }
        Some(format!(
            "\u{1F3C1} Finished at tick {}{} — recording stopped",
            finish.tick, time
        ))
    }

    /// The race time of the finished take: the latched HUD time, else ticks
    /// from the start line to the finish line. The in-game timer starts at
    /// the start trigger, not at first movement, which would overstate the
    /// time by several seconds.
    fn stamp(
        &self,
        snapshot: &recording::RecordingSnapshot,
        end_tick: u32,
        level_code: impl FnOnce() -> Option<&'static str>,
    ) -> Option<recording::FinishStamp> {
        let tick = self.at_tick?;
        let hud = self.hud_cs.unwrap_or(u32::MAX);
        Some(if hud != u32::MAX {
            recording::FinishStamp {
                cs: hud,
                exact: true,
            }
        } else {
            let start = crate::start_line::start_cross_tick(
                snapshot.rec_coords.as_ref(),
                end_tick,
                level_code(),
            );
            let first_moving =
                tas_shared::align::detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
            recording::FinishStamp {
                cs: recording::geometry_race_time_cs(tick, start, first_moving),
                exact: false,
            }
        })
    }
}

/// A finished take for history.
pub(crate) struct CompletedTake {
    pub(crate) owner: ActiveSession,
    pub(crate) label: String,
    /// The plain session label, for log lines.
    pub(crate) session_label: String,
    pub(crate) start_tick: u32,
    pub(crate) end_tick: u32,
    pub(crate) finish: Option<recording::FinishStamp>,
}

#[derive(Default)]
pub(crate) struct Session {
    /// The take being recorded. Kept apart from `pending`: a cycle can be
    /// queued in the frame the DLL ended a REC the UI has not finalized yet.
    pub(crate) active: Option<ActiveSession>,
    pub(crate) pending: Option<StartContext>,
    /// The DLL mode last seen, for its edges.
    pub(crate) last_mode: u32,
    pub(crate) finish: FinishWatch,
}

impl Session {
    /// A cycle was queued for `pending` (None = PLAY): take the finish
    /// baseline now, so a finish before the UI sees REC still counts.
    pub(crate) fn expect(&mut self, pending: Option<StartContext>, live_finish_seq: u32) {
        self.pending = pending;
        self.finish.seen_seq = live_finish_seq;
        self.finish.baseline_armed = true;
    }

    /// The queued cycle was cancelled.
    pub(crate) fn cancel_pending(&mut self) {
        self.pending = None;
        self.finish.baseline_armed = false;
    }

    /// The splice a pending CONT will record from.
    pub(crate) fn pending_splice(&self) -> Option<u32> {
        match self.pending {
            Some(StartContext::Continue { splice }) => Some(splice),
            _ => None,
        }
    }

    /// REC began: open a session for what was queued, or, for a REC no
    /// cycle of ours armed, for what the DLL says.
    pub(crate) fn start(&mut self, continue_from_frame: u32, recorded_count: u32) -> ActiveSession {
        let (kind, start_tick) = match self.pending.take() {
            Some(StartContext::Rec) => (RecordingSessionKind::Rec, 0),
            Some(StartContext::Continue { splice }) => (RecordingSessionKind::Continue, splice),
            None if continue_from_frame > 0 => {
                (RecordingSessionKind::Continue, continue_from_frame)
            }
            None => (RecordingSessionKind::Rec, 0),
        };
        let session = ActiveSession {
            kind,
            start_tick,
            max_recorded_count: recorded_count.max(start_tick),
        };
        self.active = Some(session);
        session
    }

    /// The checkpoint context of the take at `recorded` ticks.
    pub(crate) fn checkpoint(
        &mut self,
        recorded: u32,
    ) -> Option<recording::RecoverySessionContext> {
        let session = self.active.as_mut()?;
        session.max_recorded_count = session.max_recorded_count.max(recorded);
        recording::RecoverySessionContext::from_ticks(
            session.kind,
            session.start_tick,
            session.max_recorded_count,
        )
    }

    /// Close the session on `snapshot`: what history gets, if anything.
    pub(crate) fn complete(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
        recorded_count: u32,
        level_code: impl FnOnce() -> Option<&'static str>,
    ) -> Option<CompletedTake> {
        let session = self.active.take()?;
        let end_tick = recorded_count.max(session.max_recorded_count);
        let context = recording::RecoverySessionContext::from_ticks(
            session.kind,
            session.start_tick,
            end_tick,
        )?;
        // A session the finish-line watch stopped is labelled by its race
        // time ("Finish 0:53.34"), not its length.
        let finish = self.finish.stamp(snapshot, end_tick, level_code);
        Some(CompletedTake {
            owner: session,
            label: match finish {
                Some(f) => recording::finished_session_label(f),
                None => context.label.clone(),
            },
            session_label: context.label,
            start_tick: context.start_tick,
            end_tick: context.end_tick,
            finish,
        })
    }
}

impl TasApp {
    /// REC began: open its session.
    pub(crate) fn start_recording_session(
        &mut self,
        continue_from_frame: u32,
        recorded_count: u32,
    ) {
        let session = self.session.start(continue_from_frame, recorded_count);
        if session.kind == RecordingSessionKind::Rec {
            self.editor.detach();
        }
    }

    pub(crate) fn update_recording_recovery_progress(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
    ) {
        // Stamp the track now, while it is visible. A checkpoint is recovered
        // at startup, before anything has read the live level.
        let level = self.conn.level_for_save().map(str::to_string);
        // The rider pair is read coherently, through its seqlock.
        let live_stamps = self
            .conn
            .shared
            .as_ref()
            .map_or_else(Default::default, |s| {
                let (character, stance) = tas_shared::rider_pair(s.state());
                recording::IdentityStamps {
                    rider_character: (character != tas_shared::TAS_CHARACTER_UNKNOWN)
                        .then_some(character),
                    rider_stance: (stance != u32::MAX).then_some(stance),
                    ..recording::IdentityStamps::from_live(s.state())
                }
            });
        if let Some(session_context) = self.session.checkpoint(snapshot.recorded_count) {
            let mut session_context = session_context.with_level(level.as_deref());
            session_context.stamps = live_stamps;
            self.history.persist_recovery_snapshot_if_needed(
                snapshot,
                &session_context,
                false,
                &mut self.log_lines,
            );
        }
    }

    pub(crate) fn finalize_recording_session(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
        recorded_count: u32,
    ) {
        let Some(take) = self
            .session
            .complete(snapshot, recorded_count, || self.conn.live_level_code())
        else {
            return;
        };
        let pushed = self.history.list.push_completed_session(
            snapshot.clone(),
            take.label,
            take.start_tick,
            take.end_tick,
            take.finish,
        );
        if !pushed {
            // An empty snapshot (e.g. a fresh mapping after the game
            // restarted) leaves the checkpoint on disk as the only copy, and
            // the next take's checkpoint would replace it. Recover it into
            // history now, but only this session's: an older take whose clear
            // was refused must not come back as a pinned duplicate.
            self.push_log(&format!(
                "Recording session ({}) was not added to history: the buffer is empty — \
                 recovering its checkpoint instead",
                take.session_label
            ));
            if self
                .history
                .recover_pending_checkpoint_matching(Some(take.owner.owner()), &mut self.log_lines)
            {
                self.history
                    .clear_recovery_after_durable_persist(self.host.now(), &mut self.log_lines);
            }
            return;
        }
        // The checkpoint is cleared only once the take is durable in history,
        // so a crash before the background write commits cannot lose it.
        self.history
            .clear_recovery_after_durable_persist(self.host.now(), &mut self.log_lines);
    }
}
