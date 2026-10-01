//! Crash recovery and relaunch: the game process can die or be replaced while
//! the UI keeps its mapping.
//!
//! One relaunch produces two signals in either order — the Supreme.exe PID
//! changing (1 Hz sample) and `frame_count` going backwards (the new DLL's
//! memset). `expect_ring_restart` ties them to one reset. `mapping_is_old`
//! says whether the section still holds the dead DLL's bytes, which are ours
//! to capture and release; once zeroed, nothing in it is.

use tas_shared::TasMode;

use crate::recording;
use crate::win32;
use crate::{ActiveRecordingSession, TasApp};

impl TasApp {
    /// Bring a pending checkpoint back as a pinned history entry, tagged with
    /// the track / rider / physics it was recorded under. The caller clears the
    /// checkpoint only after the history flush is confirmed durable.
    ///
    /// With `owner`, recovers the file only if it belongs to that session
    /// (same kind and start tick, and no longer than the session ever was): a
    /// rejected finalize must not resurrect an older take's checkpoint as a
    /// pinned duplicate of an entry already in history.
    pub(crate) fn recover_pending_checkpoint_matching(
        &mut self,
        owner: Option<ActiveRecordingSession>,
    ) -> bool {
        if self.recovery_store.is_none() {
            return false;
        }
        // Drain queued checkpoint writes FIRST. The newest one may still be in
        // flight; recovering the older file and then clearing "after durable
        // persist" would let that newer write land only to be deleted.
        if let Err(error) = self.recovery_writer.flush() {
            self.push_log(&format!("Recovery flush failed: {error}"));
        }
        let Some(store) = self.recovery_store.as_ref() else {
            return false;
        };
        let cp = match store.load_pending() {
            Ok(Some(cp)) => cp,
            Ok(None) => return false,
            Err(err) => {
                self.push_log(&format!("Crash recovery check failed: {}", err));
                return false;
            }
        };
        if let Some(owner) = owner {
            let mine = cp.session.kind == owner.kind
                && cp.session.start_tick == owner.start_tick
                && cp.session.end_tick <= owner.max_recorded_count;
            if !mine {
                self.push_log(&format!(
                    "Recovery checkpoint ({}) belongs to another session - kept on disk for the next launch",
                    cp.session.label
                ));
                return false;
            }
        }
        let session_label = cp.session.label.clone();
        let (start, end) = (cp.session.start_tick, cp.session.end_tick);
        let cp_level = cp.session.level.clone();
        let cp_rider = cp.session.rider_label();
        let cp_physics = cp.session.physics_label();
        if !self.history.push_snapshot_data_with_session(
            cp.snapshot,
            session_label.clone(),
            start,
            end,
            Some(cp.session.stamps),
        ) {
            return false;
        }
        if let Some(id) = self.history.entries().last().map(|e| e.entry_id) {
            // Restore the track the checkpoint was RECORDED on. push_* stamps
            // from the live level, which may be None here (startup runs before
            // any level sync); an untagged pinned entry would float at the top
            // of every track's history.
            self.history.set_level(id, cp_level);
            self.history.set_rider(id, cp_rider);
            self.history.set_physics(id, cp_physics);
            self.history.set_pinned(id, true);
            // Mark recovery with a compact ⟲ glyph and let the panel render
            // the duration via the normal parsed format.
            self.history.rename(id, "⟲".to_string());
        }
        self.push_log(&format!(
            "Recovered an unsaved recording ({}) → pinned in history",
            session_label
        ));
        true
    }

    /// A fresh `TAS_Helper.dll` reused the section this process still maps, so
    /// `frame_count` restarted from 0. Nothing else notices (the version
    /// matches, the mapping never went away), so treat it as a reconnect: log
    /// cursor, cached game PID, session view and in-flight cycle all belong to
    /// the dead process.
    pub(crate) fn on_dll_reinitialised(&mut self, fc: u32) {
        let current_pid = win32::find_supreme_pid();
        self.on_dll_reinitialised_with(fc, current_pid);
    }

    pub(crate) fn on_dll_reinitialised_with(&mut self, fc: u32, current_pid: Option<u32>) {
        let expected_for = self.expect_ring_restart.take();
        // A snapshot that momentarily finds no process cannot contradict the
        // expectation; only a DIFFERENT process does.
        let same_relaunch =
            expected_for.is_some() && current_pid.is_none_or(|pid| Some(pid) == expected_for);
        if same_relaunch {
            // The PID watcher already reset the session for THIS relaunch;
            // the memset is the second half of the same event. Only the
            // ring and the heartbeat baseline start over — a cycle the user
            // started in between keeps running.
            self.push_log(&format!(
                "TAS_Helper.dll re-initialised its shared memory (frame counter {} → {})",
                self.cycle_fc, fc
            ));
            self.cycle_fc = fc;
            self.last_frame_count = fc;
            self.stale_frame_ticks = 0;
            self.log_read_cursor = 0;
            return;
        }
        // A pending expectation for some OTHER process is stale (that
        // relaunch never showed a regression): this is a new one.
        let why = format!(
            "TAS_Helper.dll re-initialised its shared memory (frame counter {} → {}, \
             pid {:?}, expected {:?}): the game was restarted with tas_ui open — \
             resetting the session view",
            self.cycle_fc, fc, current_pid, expected_for
        );
        // The counter went backwards: the section is the new DLL's already,
        // and its ring starts over.
        self.reset_session_view_for_new_game(&why, fc, false);
        self.log_read_cursor = 0;
        // Bring the PID watcher up to date so it does not fire a second reset
        // a second later, on top of whatever the user started in between.
        self.game_pid_seen = current_pid;
    }

    /// The frame counter cannot show a relaunch that happens before the old
    /// process ever ticked (0 → 0 at the main menu), so the health check also
    /// watches Supreme.exe's identity.
    pub(crate) fn on_game_process_changed(&mut self, old_pid: u32, new_pid: u32) {
        let fc = self
            .shared
            .as_ref()
            .map_or(0, |shared| shared.frame_count_volatile());
        // The dead DLL's section is frozen at the counter we last saw; a
        // counter below it means the new DLL has already zeroed the section.
        // A counter that never moved proves nothing either way (the old
        // game sat at a menu): treat the section as unknown, never as ours.
        let old_never_ticked = self.cycle_fc == 0;
        let mapping_is_old = !old_never_ticked && fc >= self.cycle_fc;
        let stale_mode = self
            .shared
            .as_ref()
            .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
        if mapping_is_old {
            // Relaunched between two samples: the dead DLL's record is still here.
            self.report_game_exit(old_pid, false);
        }
        let why = format!(
            "Supreme.exe was replaced (pid {old_pid} → {new_pid}) with tas_ui still mapped: \
             resetting the session view"
        );
        self.reset_session_view_for_new_game(&why, fc, mapping_is_old);
        if mapping_is_old {
            // The dead DLL's mode word stays frozen (REC, say) until the
            // memset. Baseline on it, or the next poll reads it as a fresh
            // "REC started" and opens a phantom session for a take that was
            // just captured — which the memset then "loses" and recovers
            // from the checkpoint a second time.
            self.last_mode = stale_mode;
        }
        if mapping_is_old && !old_never_ticked {
            // Rewinding now would replay lines already shown; the memset that
            // follows restarts the ring, and only that regression, for this
            // process, is the second half of this relaunch.
            self.expect_ring_restart = Some(new_pid);
        } else {
            // The new ring is already in place, or the old game never ticked
            // and its ring held only start-up lines. A 0 → 0 menu relaunch
            // shows no regression, so nothing else would rewind.
            self.expect_ring_restart = None;
            self.log_read_cursor = 0;
        }
    }

    pub(crate) fn drain_dll_log(&mut self) {
        let Some(ref shared) = self.shared else {
            return;
        };
        use tas_shared::TasLogSeverity;
        // The ring's sequence only ever grows — until a fresh DLL zeroes the
        // section. A sequence below the cursor is that restart: rewind, or
        // the new DLL's first lines are skipped until it catches up.
        if shared.state().log_write_seq < self.log_read_cursor {
            self.log_read_cursor = 0;
            self.expect_ring_restart = None;
        }
        let (entries, new_cursor) = shared.state().read_log_entries(self.log_read_cursor);
        self.log_read_cursor = new_cursor;
        for (_, severity, text) in entries {
            let prefix = match severity {
                TasLogSeverity::Debug => "[DLL:DBG]",
                TasLogSeverity::Info => "[DLL]",
                TasLogSeverity::Warn => "[DLL:WARN]",
                TasLogSeverity::Error => "[DLL:ERR]",
            };
            self.log_lines.push(format!("{} {}", prefix, text));
        }
    }

    /// One relaunch, one reset — whichever signal saw it first.
    fn reset_session_view_for_new_game(&mut self, why: &str, fc: u32, mapping_is_old: bool) {
        self.push_log(why);
        if self.active_recording_session.is_some() {
            if mapping_is_old {
                // Capture before resetting anything: the dead DLL's buffer is
                // the complete take, the checkpoint lags by a debounce.
                self.capture_take_from_mapping(
                    "The game process that owned the recording in progress is gone",
                );
            } else {
                // The buffer belongs to the new game now, so the old take's
                // checkpoint is its only copy.
                let owner = self.active_recording_session.take();
                self.push_log(
                    "The recording in progress was lost with the old game process; \
                     recovering its last checkpoint",
                );
                if self.recover_pending_checkpoint_matching(owner) {
                    self.clear_recovery_after_durable_persist();
                }
            }
        }
        self.cycle_fc = fc;
        self.last_frame_count = fc;
        self.stale_frame_ticks = 0;
        self.game_pid_cached = None;
        self.clear_cont_catchup();
        // Only a cycle of ours may release the interlock, and only while the
        // section is still the one it armed: a controller in another process
        // may already own the new game's restart.
        let owned_cycle = self.cycle.is_some();
        self.pending_session_kind = None;
        self.pending_continue_start_tick = None;
        self.cycle_deadline = None;
        self.cycle = None;
        if owned_cycle && mapping_is_old {
            self.set_cont_suppress_input(false);
        }
        self.detach_input_editor();
        self.loaded_physics = None;
        self.loaded_rider = None;
        self.loaded_identity = None;
        self.loaded_level = None;
        self.last_mode = TasMode::Off as u32;
    }

    /// Finalize the take still sitting in the section into history, for when
    /// the process that owned it is gone but nothing has zeroed it yet.
    ///
    /// Waiting for a REC→OFF transition instead would lose it: that transition
    /// can only arrive from a fresh, empty mapping, which pushes an empty
    /// snapshot and then deletes the checkpoint. No-ops without a session, so
    /// callers on the relaunch path need no guard.
    fn capture_take_from_mapping(&mut self, why: &str) -> bool {
        if self.active_recording_session.is_none() {
            return false;
        }
        let capture = self.shared.as_ref().map(|shared| {
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            let recorded = snapshot.recorded_count;
            (snapshot, recorded)
        });
        if let Some((snapshot, recorded)) = capture {
            self.push_log(&format!(
                "{why} — captured its {recorded} ticks from shared memory"
            ));
            self.finalize_recording_session(&snapshot, recorded);
            return true;
        }
        false
    }

    /// The game's own error handler can catch a fault and hold the process
    /// open behind its dialog, so a new crash record is shown while the game
    /// is still alive.
    pub(crate) fn poll_crash_record(&mut self, pid: u32) {
        let Some(shared) = self.shared.as_ref() else {
            return;
        };
        let seq = shared
            .state()
            .crash_seq
            .load(std::sync::atomic::Ordering::Acquire);
        if seq == self.crash_seq_seen {
            return;
        }
        self.crash_seq_seen = seq;
        if let Some(record) = tas_shared::crash::crash_record(shared.state(), pid) {
            let banner =
                format!("The game faulted: {record}. It may be showing its own error dialog.");
            self.push_log(&banner);
            self.game_exit_banner = Some(banner);
        }
    }

    /// Say how game process `pid` ended, from the crash record its DLL left
    /// in the mapping: a crash or an unexplained exit raises the banner.
    pub(crate) fn report_game_exit(&mut self, pid: u32, take_saved: bool) {
        if self.game_exit_reported_pid == Some(pid) {
            return;
        }
        self.game_exit_reported_pid = Some(pid);
        let Some(shared) = self.shared.as_ref() else {
            return;
        };
        let saved = if take_saved {
            " The take in progress was saved to history."
        } else {
            ""
        };
        let banner = match tas_shared::crash::game_exit(shared.state(), pid) {
            tas_shared::crash::GameExit::Clean => {
                self.push_log("The game closed normally");
                return;
            }
            tas_shared::crash::GameExit::Crashed(record) => {
                format!("The game crashed: {record}.{saved}")
            }
            tas_shared::crash::GameExit::Unexplained => format!(
                "The game closed without a crash record: it was killed, or it crashed \
                 where the TAS could not see.{saved}"
            ),
        };
        self.push_log(&banner);
        self.game_exit_banner = Some(banner);
    }

    /// The 1 Hz identity sample. A new PID is a relaunch; no PID during a
    /// recording is the moment to capture it, because nothing can zero the
    /// section until a new game injects and waiting for the stale-frame
    /// disconnect (up to 5 s) would settle for the lagging checkpoint.
    pub(crate) fn on_game_pid_observed(&mut self, pid: Option<u32>) {
        match pid {
            Some(pid) => {
                if let Some(seen) = self.game_pid_seen {
                    if seen != pid {
                        self.on_game_process_changed(seen, pid);
                    }
                }
                self.game_pid_seen = Some(pid);
            }
            None => {
                let Some(dead) = self.game_pid_seen else {
                    return;
                };
                let mut take_saved = false;
                if self.active_recording_session.is_some() {
                    take_saved = self.capture_take_from_mapping("Game exited during a recording");
                    // The frozen section still reads REC; the memset or the
                    // disconnect must not turn that into a phantom session.
                    self.last_mode = self
                        .shared
                        .as_ref()
                        .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
                }
                self.report_game_exit(dead, take_saved);
            }
        }
    }

    pub(crate) fn disconnect_from_dead_game(&mut self) {
        self.push_log("Game process not found — disconnecting shared memory");
        self.capture_take_from_mapping("Game exited during a recording");
        self.shared = None;
        self.detach_input_editor();
        self.connect_error =
            Some("Supreme.exe has exited. Inject TAS_Helper.dll after restarting the game.".into());
        self.stale_frame_ticks = 0;
        self.log_read_cursor = 0;
        // A fresh Supreme.exe gets a different PID, so the global-shortcut
        // foreground gate would otherwise stay stale.
        self.game_pid_cached = None;
        self.game_pid_seen = None;
        self.expect_ring_restart = None;
        self.clear_cont_catchup();
        self.reset_continue_runtime_state();
        self.last_mode = TasMode::Off as u32;
    }
}
