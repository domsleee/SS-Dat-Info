//! Recording-session lifecycle: starting and finalizing a take into history,
//! background history persistence, and the crash-recovery checkpoint.

use crate::recording::{self, RecordingSessionKind};
use crate::relaunch::CheckpointOwner;
use crate::{ActiveRecordingSession, TasApp};

/// Whether the recovery checkpoint may be deleted: only when the recovered
/// take is durably committed to history. A missing history writer is NOT
/// durability — the checkpoint files are then the only copy in existence,
/// and deleting them loses the recovered take permanently.
pub(crate) fn checkpoint_clear_decision(
    flush: Option<Result<(), String>>,
) -> (bool, Option<String>) {
    match flush {
        Some(Ok(())) => (true, None),
        Some(Err(e)) => (
            false,
            Some(format!(
                "[history] persist failed — keeping recovery checkpoint: {}",
                e
            )),
        ),
        None => (
            false,
            Some(
                "[history] store unavailable — keeping recovery checkpoint (no history was written)"
                    .to_string(),
            ),
        ),
    }
}

impl TasApp {
    /// Clear the crash-recovery checkpoint, but only after the history it
    /// represents is durably committed (persist → flush → clear). If the flush
    /// fails the checkpoint is kept, so the next launch recovers it. The
    /// recovery writer is drained first so no in-flight write can recreate the
    /// checkpoint files after they are deleted.
    pub(crate) fn clear_recovery_after_durable_persist(&mut self) {
        self.persist_history_if_needed();
        let flush = self.history_writer.as_ref().map(|writer| {
            let result = writer.flush();
            if result.is_ok() {
                self.last_persisted_revision =
                    self.last_persisted_revision.max(writer.durable_revision());
            }
            result
        });
        let (durable, notice) = checkpoint_clear_decision(flush);
        if let Some(notice) = notice {
            self.log_lines.push(notice);
        }
        // Drain in-flight recovery writes BEFORE clearing the files.
        if let Err(error) = self.recovery_writer.flush() {
            self.log_lines
                .push(format!("Recovery flush failed: {error}"));
        }
        if durable {
            if let Some(store) = self.recovery_store.as_mut() {
                if let Err(error) = store.clear_pending() {
                    self.log_lines.push(format!(
                        "Could not clear durable recovery checkpoint: {error}"
                    ));
                }
            }
        }
    }

    pub(crate) fn persist_history_if_needed(&mut self) {
        let revision = self.history.revision();
        let Some(writer) = self.history_writer.as_ref() else {
            return;
        };

        for error in writer.take_errors() {
            self.log_lines
                .push(format!("History persistence failed: {error}"));
        }

        self.last_persisted_revision = self.last_persisted_revision.max(writer.durable_revision());
        // Takes the writer has committed no longer need a resident copy.
        for (id, blob) in writer.take_durable_blobs() {
            self.history.mark_durable(id, blob);
        }
        let failed = writer.failed_revision();
        if failed == 0 {
            self.last_failed_revision = 0;
        } else if failed != self.last_failed_revision && failed > self.last_persisted_revision {
            self.last_failed_revision = failed;
            self.last_queued_revision = self.last_persisted_revision;
            self.history_retry_after =
                Some(std::time::Instant::now() + std::time::Duration::from_secs(1));
            self.log_lines.push(format!(
                "[history] background persist of revision {} failed; retrying",
                failed
            ));
        }

        if revision == self.last_persisted_revision || revision == self.last_queued_revision {
            return;
        }
        if self
            .history_retry_after
            .is_some_and(|deadline| std::time::Instant::now() < deadline)
        {
            return;
        }
        // UI-thread cost is only the clone; the worker does serialize + disk.
        let entries = self.history.to_stored_entries();
        let current = self.history.current_entry_id();
        let next = self.history.next_entry_id();
        // Only mark the revision persisted if the job actually reached the
        // worker. If the writer thread is gone, leave the revision dirty so a
        // later frame retries instead of silently dropping the change.
        if !writer.persist(entries, current, next, revision) {
            return;
        }
        // The re-queue is a new attempt, so forget the handled failure. If the
        // worker fails again with the same revision before this thread sees
        // the cleared marker, a stale `last_failed_revision` would suppress
        // the retry.
        self.last_failed_revision = 0;
        self.last_queued_revision = revision;
        self.history_retry_after = None;
    }

    pub(crate) fn start_recording_session(
        &mut self,
        continue_from_frame: u32,
        recorded_count: u32,
    ) {
        let kind = self.pending_session_kind.take().unwrap_or({
            if continue_from_frame > 0 {
                RecordingSessionKind::Continue
            } else {
                RecordingSessionKind::Rec
            }
        });
        let start_tick = match kind {
            RecordingSessionKind::Rec => {
                self.detach_input_editor();
                self.pending_continue_start_tick = None;
                0
            }
            RecordingSessionKind::Continue => {
                self.pending_continue_start_tick
                    .take()
                    .unwrap_or(if continue_from_frame > 0 {
                        continue_from_frame
                    } else {
                        recorded_count
                    })
            }
        };

        if kind == RecordingSessionKind::Rec && self.last_mode == 0 {
            self.segment_tracker.clear();
        }
        self.segment_tracker.on_rec_start(start_tick);
        self.active_recording_session = Some(ActiveRecordingSession {
            kind,
            start_tick,
            max_recorded_count: recorded_count.max(start_tick),
        });
    }

    fn persist_recovery_snapshot_if_needed(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
        session: &recording::RecoverySessionContext,
        force: bool,
    ) {
        for error in self.recovery_writer.take_errors() {
            self.log_lines
                .push(format!("Recovery persistence failed: {error}"));
            if let Some(store) = self.recovery_store.as_mut() {
                store.retry_failed_write();
            }
        }
        // Decide on the UI thread (at most one write per ~1.5s of recording
        // growth, see DEFAULT_RECOVERY_DEBOUNCE_MS) but write off it so REC
        // never hitches. STOP does not write a checkpoint: finalize moves the
        // take into durable history and then clears it. Best-effort: a failed
        // write only leaves a slightly staler recovery file.
        let job = match self.recovery_store.as_mut() {
            Some(store) => {
                store.take_write_job(snapshot, &self.segment_tracker.segments, session, force)
            }
            None => None,
        };
        if let Some(job) = job {
            if !self.recovery_writer.submit(job) {
                self.log_lines
                    .push("Recovery writer unavailable; checkpoint was not queued");
                if let Some(store) = self.recovery_store.as_mut() {
                    store.retry_failed_write();
                }
            }
        }
    }

    pub(crate) fn update_recording_recovery_progress(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
    ) {
        // Stamp the track now, while it is visible. A checkpoint is recovered
        // at startup, before anything has read the live level.
        let level = self.level_for_save().map(str::to_string);
        let live_stamps = self.shared.as_ref().map(|s| {
            (
                s.fpu_control_word(),
                s.renderer_id(),
                tas_shared::rider_pair(s.state()),
            )
        });
        let maybe_session = {
            let Some(session) = self.active_recording_session.as_mut() else {
                return;
            };
            session.max_recorded_count = session.max_recorded_count.max(snapshot.recorded_count);
            recording::RecoverySessionContext::from_ticks(
                session.kind,
                session.start_tick,
                session.max_recorded_count,
            )
        };

        if let Some(session_context) = maybe_session {
            let session_context = session_context
                .with_level(level.as_deref())
                .with_stamps(live_stamps);
            self.persist_recovery_snapshot_if_needed(snapshot, &session_context, false);
        }
    }

    pub(crate) fn finalize_recording_session(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
        recorded_count: u32,
    ) {
        let Some(session) = self.active_recording_session.take() else {
            return;
        };

        let end_tick = recorded_count.max(session.max_recorded_count);
        let Some(session_context) = recording::RecoverySessionContext::from_ticks(
            session.kind,
            session.start_tick,
            end_tick,
        ) else {
            return;
        };

        // A session the finish-line watch stopped is labelled by its race
        // time ("Finish 0:53.34"), not its length. Prefer the latched HUD time;
        // without it, count ticks from the start line to the finish line. The
        // in-game timer starts at the start trigger, not at first movement,
        // which would overstate the time by several seconds.
        let finish = self.finished_at_tick.map(|tick| {
            let hud = self.finished_hud_cs.unwrap_or(u32::MAX);
            if hud != u32::MAX {
                recording::FinishStamp {
                    cs: hud,
                    exact: true,
                }
            } else {
                let level_code = self
                    .shared
                    .as_ref()
                    .and_then(|s| tas_shared::resolved_level_id(s.state()))
                    .and_then(crate::level::level_code_from_id);
                let start = crate::start_line::start_cross_tick(
                    snapshot.rec_coords.as_ref(),
                    end_tick,
                    level_code,
                );
                let first_moving =
                    tas_shared::align::detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
                recording::FinishStamp {
                    cs: recording::geometry_race_time_cs(tick, start, first_moving),
                    exact: false,
                }
            }
        });
        let label = match finish {
            Some(f) => recording::finished_session_label(f),
            None => session_context.label.clone(),
        };
        let pushed = self.history.push_completed_session(
            snapshot.clone(),
            label,
            session_context.start_tick,
            session_context.end_tick,
            finish,
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
                session_context.label
            ));
            if self.recover_pending_checkpoint_matching(Some(CheckpointOwner::from(&session))) {
                self.clear_recovery_after_durable_persist();
            }
            return;
        }
        // The checkpoint is cleared only once the take is durable in history,
        // so a crash before the background write commits cannot lose it.
        self.clear_recovery_after_durable_persist();
    }
}
