//! The app: the units it is composed of, and one frame, in order. Each
//! unit owns its state; the frame routes what one observes or emits to the
//! others.

use eframe::egui;
use tas_shared::{TasCommand, TasMode};

use crate::history_runtime::BufferOperation;
use crate::panels::{history, transport};
use crate::pico::PicoState;
use crate::transport_runtime::normalize_playback_speed;
use crate::{
    editor, game_connection, history_runtime, history_store, host, recording, session, settings,
    take_buffer, transport_runtime, ui_log, view,
};

pub(crate) fn stop_is_acknowledged(mode: u32, command_idle: bool) -> bool {
    mode == TasMode::Off as u32 && command_idle
}

pub(crate) struct TasApp {
    /// Clock, waits, keyboard, process and file-dialog access.
    pub(crate) host: Box<dyn host::Host>,
    pub(crate) conn: game_connection::GameConnection,
    pub(crate) history: history_runtime::HistoryRuntime,
    /// The take in the recording buffer: its identity, and the UI's
    /// changes to it.
    pub(crate) take: take_buffer::TakeBuffer,
    pub(crate) editor: editor::Editor,
    pub(crate) transport: transport_runtime::TransportRuntime,
    pub(crate) session: session::Session,
    pub(crate) view: view::View,

    /// Panel toggles, CONT catch-up speed and history cap, edited in place
    /// and saved on exit. Its `playback_speed` is only written at save time;
    /// the live speed is the transport's.
    pub(crate) settings: settings::Settings,
    pub(crate) pico: PicoState,
    pub(crate) log_lines: ui_log::UiLog,
}

impl TasApp {
    /// Every field at its starting value, with no game connection, history
    /// store, recovery store or session log file: `new` attaches those, and
    /// the tests run on it as is.
    pub(crate) fn blank(settings: settings::Settings) -> Self {
        Self::blank_with_host(settings, Box::new(host::RealHost))
    }

    pub(crate) fn blank_with_host(
        mut settings: settings::Settings,
        host: Box<dyn host::Host>,
    ) -> Self {
        settings.history_cap = settings.history_cap.max(1);
        let now = host.now();
        Self {
            conn: game_connection::GameConnection::new(now),
            host,
            pico: PicoState::new(),
            history: history_runtime::HistoryRuntime::new(settings.history_cap),
            take: take_buffer::TakeBuffer::default(),
            log_lines: ui_log::UiLog::default(),
            editor: editor::Editor::default(),
            transport: transport_runtime::TransportRuntime::new(normalize_playback_speed(
                settings.playback_speed,
            )),
            settings,
            session: session::Session::default(),
            view: view::View::default(),
        }
    }

    pub(crate) fn new() -> Self {
        let mut app = Self::blank(settings::Settings::load());
        app.conn.open();

        // File-per-entry history store.
        let history_dir = history_store::default_history_dir();
        let history_notices = app.history.open_stores(&history_dir);
        let recovery_store_notice = app
            .history
            .recovery_store
            .as_ref()
            .map(|store| format!("Crash recovery: {}", store.root().display()));
        app.log_lines = ui_log::UiLog::new(&history_dir);

        // Connect the Pico on startup. The live test owns the physical Pico
        // input, so it is left alone there.
        if std::env::var_os("SSB_INSPECT_E2E_RECORDING").is_none() {
            app.pico.connect();
            let msg = match app.pico.error.as_deref() {
                None => format!("Pico data interface connected on {}", app.pico.port_name),
                Some(error) => format!("Pico auto-detect: {error}"),
            };
            app.log_lines.push(msg);
        }
        for msg in history_notices {
            app.push_log(&msg);
        }
        if let Some(msg) = recovery_store_notice {
            app.push_log(&msg);
        }
        // Recovery-as-history (no banner): an existing checkpoint means an
        // unsaved recording that never reached history (STOP clears it), i.e.
        // the app crashed/closed mid-recording. Bring it back as a PINNED entry.
        let still_recording = app
            .conn
            .shared
            .as_ref()
            .is_some_and(|s| app.history.pending_checkpoint_is_live(s.state()));
        if still_recording {
            app.log_lines.push(
                "The game is still recording the unsaved take; it reaches history when it stops",
            );
        }
        let recovered_checkpoint = !still_recording
            && app
                .history
                .recover_pending_checkpoint_matching(None, &mut app.log_lines);
        app.history
            .persist_if_needed(app.host.now(), &mut app.log_lines);

        // Clear the checkpoint only after the recovered entry is confirmed
        // durable in the history store; on a failed write keep it so the next
        // launch recovers it again.
        if recovered_checkpoint {
            app.history
                .clear_recovery_after_durable_persist(app.host.now(), &mut app.log_lines);
        }

        app.adopt_buffer_identity();
        if let Some(path) = std::env::var_os("SSB_INSPECT_E2E_RECORDING") {
            // main() has already refused to start without SSB_INSPECT_DATA_DIR.
            let setup = (|| -> Result<(), String> {
                let splice = std::env::var("SSB_INSPECT_E2E_SPLICE")
                    .map_err(|e| e.to_string())?
                    .parse::<u32>()
                    .map_err(|e| e.to_string())?;
                app.prepare_e2e_recording(std::path::Path::new(&path), splice)
            })();
            if let Err(error) = setup {
                eprintln!("UI E2E setup failed: {error}");
                std::process::exit(1);
            }
            app.push_log("UI E2E ready");
        }

        app
    }

    pub(crate) fn prepare_e2e_recording(
        &mut self,
        path: &std::path::Path,
        splice: u32,
    ) -> Result<(), String> {
        let shared = self.conn.shared.as_mut().ok_or("No game connection")?;
        if shared.mode_volatile() != TasMode::Off as u32 || !shared.command_idle() {
            return Err("Game must be stopped before UI fixture loading".into());
        }
        if splice > recording::RecordingFile::read_metadata(path)?.recorded_count {
            return Err("Splice outside recording".into());
        }
        tas_shared::level::check_recording_matches_live(
            &path.to_string_lossy(),
            shared.state().level_id,
        )?;
        if !self
            .take
            .load_file(shared.state_mut(), &mut self.log_lines, path)
        {
            return Err("Could not load UI fixture".into());
        }
        self.history
            .list
            .push_loaded_snapshot(shared.state(), path, self.take.stamps().cloned());
        self.apply_transport_action(transport::Action::SetContinueFrame(splice));
        Ok(())
    }

    pub(crate) fn playback_speed_for_settings(&self) -> f32 {
        normalize_playback_speed(self.transport.resume_or_playback_speed())
    }

    pub(crate) fn try_reconnect(&mut self) {
        if self.conn.try_reconnect() {
            self.editor.detach();
            self.push_log("Connected to TAS_Helper.dll shared memory");
            self.adopt_buffer_identity();
        }
    }

    /// A UI opened on a take the DLL kept knows none of its stamps (track,
    /// rider, input model, edit limit). When the take is the current history
    /// entry's, take that entry's.
    pub(crate) fn adopt_buffer_identity(&mut self) {
        if self.take.identity.stamps.is_some() {
            return;
        }
        let Some(shared) = self.conn.shared.as_ref() else {
            return;
        };
        if !self.history.list.current_holds(shared.state()) {
            return;
        }
        self.adopt_selected_entry_stamps();
        self.push_log("The game's take is the selected history entry: using its stamps");
    }

    /// The take in the buffer is the selected history entry's: take its
    /// stamps, and warn if who rode it, or under which physics, differs from
    /// the live game.
    pub(crate) fn adopt_selected_entry_stamps(&mut self) {
        if self.take.adopt_current_entry(&self.history.list) {
            self.take
                .warn_live_mismatch(&mut self.log_lines, &self.history.list);
        }
    }

    pub(crate) fn push_log(&mut self, msg: &str) {
        self.log_lines.push(msg);
    }

    pub(crate) fn send_action_command(&mut self, command: TasCommand) {
        if command == TasCommand::Stop {
            self.transport.clear_catchup();
            // Also cancels any in-flight cycle, so STOP after a RestartThen
            // click can't complete the restart and start REC/PLAY.
            self.reset_continue_runtime_state();
        }
        if let Some(shared) = self.conn.shared.as_mut() {
            shared.send_command(command);
        }
        self.log_lines.push(format!("Sent: {:?}", command));
    }

    /// Stop any active REC/PLAY and wait (bounded) for the DLL to reach OFF
    /// before a load or history restore overwrites the recording buffer.
    /// Otherwise the DLL's cycle hook could append to input_log while the UI
    /// rewrites it. Returns `false`, leaving the buffer untouched, if STOP is
    /// not acknowledged in time. The short spin is unnoticeable next to the
    /// file dialog that precedes a load.
    pub(crate) fn stop_active_session_for_load(&mut self) -> bool {
        self.editor.detach();
        let (mode, command_idle) = self
            .conn
            .shared
            .as_ref()
            .map(|s| (s.mode_volatile(), s.command_idle()))
            .unwrap_or((TasMode::Off as u32, true));
        // The DLL can be idle while the UI still has a restart/arm queued.
        // Cancel that controller through STOP before replacing its recording.
        if stop_is_acknowledged(mode, command_idle) && !self.transport.is_running() {
            self.sync_live_level();
            return true;
        }
        let was_rec = mode == TasMode::Rec as u32;
        self.log_lines.push("Stopping active session before load");
        self.send_action_command(TasCommand::Stop);
        // Spin up to ~250ms for the DLL's cycle hook to process CMD_STOP and
        // flip to OFF (typically 1-2 cycles, ~7-14ms).
        let deadline = self.host.now() + std::time::Duration::from_millis(250);
        let stopped = loop {
            let stopped = self
                .conn
                .shared
                .as_ref()
                .map(|s| stop_is_acknowledged(s.mode_volatile(), s.command_idle()))
                .unwrap_or(true);
            if stopped || self.host.now() >= deadline {
                break stopped;
            }
            self.host.wait(std::time::Duration::from_millis(2));
        };
        if !stopped {
            self.log_lines.push(
                "Load/restore refused: Stop was not acknowledged; recording buffer unchanged",
            );
            return false;
        }
        // Finalize the stopped take into history now, before the caller
        // overwrites the buffer; by next frame the mode-transition handler
        // would snapshot the loaded recording instead. last_mode is then
        // pinned to OFF so that handler sees no REC→OFF edge and can't
        // finalize twice.
        if was_rec {
            if let Some((recorded, snap)) = self.conn.shared.as_ref().map(|s| {
                (
                    s.recorded_count_volatile(),
                    recording::RecordingSnapshot::from_state(s.state()),
                )
            }) {
                self.finalize_recording_session(&snap, recorded);
            }
        }
        self.session.last_mode = TasMode::Off as u32;
        // A fresh recording is about to load — clear the finish-flag marker.
        self.session.finish.clear(self.conn.live_finish_seq());
        // Re-read the live level: the callers' level filter was read at frame
        // start, before the wait above, and every caller is about to overwrite
        // the buffer. A track change during the wait must not let another
        // track's recording through.
        self.sync_live_level();
        true
    }

    /// Pull the live identity into history, which stamps it onto every
    /// pushed entry, and whose level filter undo / redo / restore all consult
    /// before overwriting the buffer.
    pub(crate) fn sync_live_level(&mut self) {
        let Some(live) = self.conn.live_identity() else {
            return;
        };
        self.history.list.set_live_physics(live.physics);
        self.history.list.set_live_rider(live.rider);
        self.history.list.set_live_stamps(live.stamps);
        match live.level {
            game_connection::LiveLevel::Resolved(code) => self.history.list.set_live_level(code),
            game_connection::LiveLevel::Changed => self.history.list.enter_resolving(),
            game_connection::LiveLevel::Frozen => {}
        }
    }

    /// Single dispatch for a transport `Action`, shared by the keyboard-shortcut
    /// path and the transport-bar button path so the two can never diverge.
    pub(crate) fn apply_transport_action(&mut self, cmd: transport::Action) {
        match cmd {
            transport::Action::Send(c) => self.send_action_command(c),
            transport::Action::RestartThen(c) => self.queue_restart_then(c),
            // Undo/Redo overwrite the whole shared input/coord buffer like a
            // history restore, so they stop an active REC/PLAY first; the DLL
            // must not be writing input_log during the copy.
            transport::Action::Undo => self.run_buffer_operation(BufferOperation::Undo),
            transport::Action::Redo => self.run_buffer_operation(BufferOperation::Redo),
            transport::Action::SetContinueFrame(frame) => {
                // Stage the splice frame UI-side only; the TransportController
                // writes it to shared memory when a cycle starts and on each
                // reroll, and the DLL clears it. Writing it here mid-replay is
                // harmless for plain PLAY (the cycle cave splices only an armed CONT)
                // but would move a running CONT's splice.
                self.view.stage_continue_frame(frame);
            }
            transport::Action::SetResumeSpeed(spd) => {
                self.transport
                    .set_resume_speed(spd, self.conn.shared.as_mut());
                self.log_lines.push(format!("Resume speed: {}x", spd));
            }
            transport::Action::Log(msg) => {
                self.log_lines.push(msg);
            }
        }
    }

    /// Apply a pending input edit to the stopped recording before rendering,
    /// pushing one undo snapshot per finished gesture.
    pub(crate) fn apply_pending_input_edit(&mut self) {
        // Peek without consuming — if a run is active we keep the edit queued
        // and apply it once the game stops.
        if !self.editor.has_pending() {
            return;
        }
        let mode = match self.conn.shared.as_ref() {
            Some(shared) => shared.state().mode,
            None => {
                // No shared memory to write into — drop the queued edit.
                self.editor.pending = editor::PendingEdit::None;
                return;
            }
        };
        // Never mutate the buffer the game is replaying or recording. Auto-STOP
        // the run once (latched; it also cancels its cycle) and keep the edit
        // queued until mode is Off.
        if mode != TasMode::Off as u32 {
            if self.editor.wait_for_stop() {
                self.log_lines.push("[script] stopping run to apply edit…");
                self.send_action_command(TasCommand::Stop);
            }
            return;
        }
        if self
            .conn
            .shared
            .as_ref()
            .is_some_and(|shared| !shared.command_idle())
        {
            return;
        }
        // A cycle still restarting would arm the take as it was: cancel it,
        // apply the edit, and arm the same command again with the edited take.
        let rearm = self.transport.running_command();
        if rearm.is_some() {
            self.reset_continue_runtime_state();
        }
        let Some(editor::Edit {
            events,
            commit,
            label,
        }) = self.editor.take_pending()
        else {
            return;
        };
        let shared = match self.conn.shared.as_mut() {
            Some(shared) => shared,
            None => return,
        };
        self.take.apply_edit(shared.state_mut(), &events);
        if commit {
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            // end_tick = 0 routes through the history panel's label parser so
            // the descriptive action (e.g. "Moved L 324→372t") is what shows.
            let label = if label.is_empty() {
                "Edited inputs".to_string()
            } else {
                label
            };
            self.history.list.push_snapshot_data_with_session(
                snapshot,
                label,
                0,
                0,
                self.take.stamps().cloned(),
            );
        }
        if let Some(command) = rearm {
            self.log_lines
                .push(format!("{command:?} re-armed with the edited take"));
            self.queue_restart_then(command);
        }
    }

    /// Check if the game process is still alive by monitoring frame_count advancement.
    /// If frame_count hasn't changed for ~3 seconds, assume the game crashed.
    pub(crate) fn check_game_health(&mut self) {
        if let Some(fc) = self.conn.sample_activity(self.host.now()) {
            self.on_dll_reinitialised(fc);
        }
        if !self.conn.health_check_due(self.host.now()) {
            return;
        }

        // Game identity: a relaunch that reuses our mapped section is only
        // visible here when the old process never ticked (see
        // on_game_process_changed). Sampled once a second, so a process
        // snapshot is affordable.
        if self.conn.shared.is_some() {
            let pid = self.host.game_pid();
            self.on_game_pid_observed(pid);
            if let Some(pid) = pid {
                self.conn.poll_crash_record(pid, &mut self.log_lines);
            }
        }

        if self.conn.stale_check_due() && self.host.game_pid().is_none() {
            self.disconnect_from_dead_game();
        }
    }

    /// Pick a recording, stop any active session, load it, then auto-play.
    /// The stop happens AFTER the (cancellable) file pick so cancelling the
    /// dialog never kills an in-progress recording, and BEFORE the load so the
    /// buffer overwrite doesn't race the DLL's REC cycle hook.
    pub(crate) fn load_recording_flow(&mut self) {
        // Resolved, not raw: the starting folder should be the track we are
        // actually on. Unknown opens the root recordings dir rather than
        // confidently opening the previous track's.
        let level_id = match self.conn.shared.as_ref() {
            Some(s) => tas_shared::resolved_level_id(s.state()).unwrap_or(u32::MAX),
            None => return,
        };
        let Some(path) = self.host.pick_load_path(level_id) else {
            return;
        };
        self.run_buffer_operation(BufferOperation::Load(path));
    }

    /// Replace the recording buffer. Undo/redo, a history restore and a load
    /// all overwrite the whole shared input/coord buffer, so they stop an
    /// active REC/PLAY first; the DLL must not be writing input_log during the
    /// copy.
    pub(crate) fn run_buffer_operation(&mut self, op: BufferOperation) {
        if !self.stop_active_session_for_load() {
            return;
        }
        match op {
            BufferOperation::Undo | BufferOperation::Redo => {
                let undo = matches!(op, BufferOperation::Undo);
                let restored = self.take.restore_from(
                    self.conn.shared.as_mut().map(|s| s.state_mut()),
                    &mut self.history.list,
                    |h| if undo { h.undo() } else { h.redo() },
                );
                if restored.is_some() {
                    self.log_lines.push(if undo {
                        "Undo: restored previous recording"
                    } else {
                        "Redo: restored next recording"
                    });
                    self.take
                        .warn_live_mismatch(&mut self.log_lines, &self.history.list);
                }
            }
            BufferOperation::Restore { entry_id, fallback } => {
                let idx = entry_id
                    .and_then(|id| {
                        self.history
                            .list
                            .entries()
                            .iter()
                            .position(|e| e.entry_id == id)
                    })
                    .unwrap_or(fallback);
                // Checked against the level re-read after the stop.
                // Log the refusal: a silent no-op reads as a bug.
                if !self.history.list.entry_on_current_level(idx) {
                    self.log_lines.push(format!(
                        "History restore refused: that entry is not for the \
                         current track ({})",
                        self.history.list.live_level().unwrap_or("resolving")
                    ));
                    return;
                }
                let Some(shared) = self.conn.shared.as_mut() else {
                    return;
                };
                let restored =
                    self.take
                        .restore_from(Some(shared.state_mut()), &mut self.history.list, |h| {
                            h.restore_index(idx)
                        });
                if let Some(count) = restored {
                    let label = self
                        .history
                        .list
                        .entries()
                        .get(idx)
                        .map(|entry| entry.label.clone())
                        .unwrap_or_else(|| format!("Entry {}", idx + 1));
                    self.log_lines.push(format!("History restore: {}", label));
                    self.take
                        .warn_live_mismatch(&mut self.log_lines, &self.history.list);
                    // Fit the timeline to the whole loaded recording.
                    self.view.timeline.fit(count);
                }
            }
            BufferOperation::Load(path) => {
                // The dialog only starts in the current track's folder; the
                // user can pick any file. Recordings are named
                // `<CODE>-<name>`, so check the file against the level re-read
                // after the stop above. Unknown on either side cannot prove a
                // mismatch and is allowed, as in tas_test.
                let live_id = self
                    .conn
                    .shared
                    .as_ref()
                    .and_then(|s| tas_shared::resolved_level_id(s.state()))
                    .unwrap_or(u32::MAX);
                if let Err(msg) = tas_shared::level::check_recording_matches_live(
                    &path.to_string_lossy(),
                    live_id,
                ) {
                    self.log_lines.push(format!("Load refused: {}", msg));
                    return;
                }
                let Some(shared) = self.conn.shared.as_mut() else {
                    return;
                };
                if self
                    .take
                    .load_file(shared.state_mut(), &mut self.log_lines, &path)
                {
                    let _ = self.history.list.push_loaded_snapshot(
                        shared.state(),
                        &path,
                        self.take.stamps().cloned(),
                    );
                    if shared.state().recorded_count > 0 {
                        self.queue_restart_then(TasCommand::ArmPlay);
                    }
                }
            }
        }
    }
}

impl eframe::App for TasApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.094, 0.094, 0.094, 1.0] // gray(24) in 0-1 range
    }

    fn on_exit(&mut self) {
        self.settings.playback_speed = self.playback_speed_for_settings();
        self.settings.save();
        // Make sure the latest history is flushed to disk before we exit.
        self.history.flush();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_frame(ctx);
    }
}

/// The sections of `update`, in the order it runs them.
impl TasApp {
    /// One whole frame. Needs no native window, so the headless frame tests
    /// run exactly this.
    pub(crate) fn update_frame(&mut self, ctx: &egui::Context) {
        self.sync_frame_state();
        self.track_mode_transitions();

        // handle_shortcuts takes keys delivered to tas_ui by egui;
        // poll_global_shortcuts takes keys seen while the game has focus.
        // They are gated on the foreground window, so one press fires once.
        let mut shortcut_actions = self.handle_shortcuts(ctx);
        shortcut_actions.extend(self.poll_global_shortcuts());

        self.show_menu_bar(ctx);
        self.show_game_exit_banner(ctx);
        self.show_log_panel(ctx);
        if self.show_connection_error(ctx) {
            return;
        }

        // Read current state snapshot for UI
        let connected = self.conn.shared.is_some();
        if !connected {
            return;
        }

        self.show_left_panel(ctx);
        let history_actions = self.show_history_panel(ctx);
        self.apply_history_actions(history_actions);

        // Apply any input edit the timeline produced last frame, before this
        // frame's shortcuts can arm a replay of the take it changes.
        self.editor.poll_script(&mut self.log_lines);
        self.apply_pending_input_edit();

        // Advance the in-flight restart/arm/watch cycle.
        self.step_cycle(ctx);

        // Apply shortcut actions to shared state (same dispatch as the buttons).
        for cmd in shortcut_actions {
            self.apply_transport_action(cmd);
        }

        self.show_central_panel(ctx);
        self.assert_playback_speed();
        self.scan_drift();

        self.conn.drain_dll_log(&mut self.log_lines);

        for w in self.history.list.take_warnings() {
            self.log_lines.push(format!("History: {}", w));
        }
        self.history
            .persist_if_needed(self.host.now(), &mut self.log_lines);

        self.schedule_repaint(ctx);
    }

    /// Once-per-frame bookkeeping that must run before anything reads live state.
    pub(crate) fn sync_frame_state(&mut self) {
        // Dark title bar (one-shot, first frame: the window must exist).
        if !self.view.dark_title_bar_set {
            self.view.dark_title_bar_set = true;
            self.host.set_dark_title_bar("SSB Inspect");
        }

        // Persist any new log lines added since last frame to the on-disk
        // session log. Done first so a panic later in the frame still
        // captures the events that led up to it. A no-op when the file could
        // not be opened at start: the in-memory log is then the only record.
        self.log_lines.flush_to_file();

        // Synchronise the live level first: the history panel filters and
        // restores against it later this frame, and must not see the previous
        // track's value on the frame after a level change.
        self.sync_live_level();

        // Check game health (crash detection) — also samples cycle activity.
        self.check_game_health();
    }

    pub(crate) fn track_mode_transitions(&mut self) {
        // Track mode transitions for history sessions + recovery checkpoints.
        let mode_probe = self.conn.shared.as_ref().map(|shared| {
            (
                shared.mode_volatile(),
                shared.recorded_count_volatile(),
                shared.state().continue_from_frame,
            )
        });
        if let Some((current_mode, recorded, continue_from)) = mode_probe {
            // A CONT just spliced: before anything snapshots the new take.
            if current_mode == 1
                && self.session.last_mode != 1
                && self.session.pending_splice().is_some()
            {
                // The DLL clears continue_from_frame once it splices.
                let splice = self.session.pending_splice().unwrap_or(continue_from);
                if let Some(shared) = self.conn.shared.as_mut() {
                    take_buffer::TakeBuffer::adopt_replayed_prefix(shared.state_mut(), splice);
                }
            }
            // The snapshot copies ~850 KB (full input_log + rec_coords). Only a
            // live REC (recovery checkpoints) or a REC that just ended (history
            // entry) consumes it, so build it only then rather than every frame.
            let need_snapshot =
                current_mode == 1 || (self.session.last_mode == 1 && current_mode == 0);
            let state_snapshot = if need_snapshot {
                self.conn
                    .shared
                    .as_ref()
                    .map(|shared| recording::RecordingSnapshot::from_state(shared.state()))
            } else {
                None
            };
            if current_mode != self.session.last_mode {
                // REC started
                if current_mode == 1 {
                    self.start_recording_session(continue_from, recorded);
                    // A fresh take is by definition the live game's: a later
                    // PLAY compares against that.
                    self.take.recorded_live(
                        &self.history.list,
                        self.conn.shared.as_ref().map(|s| s.state()),
                        self.conn.live_level_code(),
                    );
                    self.log_cont_resume_summary();
                    self.transport.clear_catchup();
                    // Splice fired (or REC began). This handler runs before
                    // step_cycle, so REC can be seen before the controller
                    // reports Done. Drop it here, and release
                    // cont_suppress_input: the resumed REC records live input.
                    self.transport.hand_over_to_rec(self.conn.shared.as_mut());
                    // Fresh finish-line watch. A cycle took its baseline
                    // when it started; a REC with no cycle takes it now.
                    self.session.finish.restart(self.conn.live_finish_seq());
                }
                // REC stopped (mode went from REC to OFF)
                if self.session.last_mode == 1 && current_mode == 0 {
                    if let Some(snap) = state_snapshot.as_ref() {
                        self.finalize_recording_session(snap, recorded);
                    }
                }
                self.session.last_mode = current_mode;
            }
            if current_mode == 1 {
                if let Some(snap) = state_snapshot.as_ref() {
                    self.update_recording_recovery_progress(snap);
                }

                self.watch_finish_line();
            }
        }
    }

    /// Finish-line watch: stop REC when the rider finishes, since the
    /// run-out is never wanted. The finish comes from the game's own
    /// Finish_Point, through the DLL.
    pub(crate) fn watch_finish_line(&mut self) {
        let Some(shared) = self.conn.shared.as_ref() else {
            return;
        };
        if let Some(line) = self.session.finish.observe(shared.state()) {
            self.log_lines.push(line);
            self.apply_transport_action(transport::Action::Send(TasCommand::Stop));
        }
    }

    pub(crate) fn apply_history_actions(&mut self, history_actions: Vec<history::HistoryAction>) {
        // Process history panel restores
        if !history_actions.is_empty() {
            for action in history_actions {
                match action {
                    history::HistoryAction::Restore(idx) => {
                        // Resolve the row to its stable entry_id before
                        // stopping: finalizing the stopped take can evict the
                        // oldest entry and shift indices.
                        let entry_id = self.history.list.entries().get(idx).map(|e| e.entry_id);
                        self.run_buffer_operation(BufferOperation::Restore {
                            entry_id,
                            fallback: idx,
                        });
                    }
                    history::HistoryAction::ClearSelection => {
                        self.history.list.clear_selection();
                    }
                    history::HistoryAction::SetPin(id, pinned) => {
                        self.history.list.set_pinned(id, pinned);
                    }
                    history::HistoryAction::Rename(id, name) => {
                        self.history.list.rename(id, name);
                    }
                }
            }
        }
    }

    /// Re-assert the playback speed every frame (see
    /// `TransportRuntime::speed_to_assert`).
    pub(crate) fn assert_playback_speed(&mut self) {
        if let Some(shared) = self.conn.shared.as_mut() {
            let mode = shared.state().mode;
            shared.state_mut().playback_speed = self.transport.speed_to_assert(mode);
        }
    }

    /// The drift banner's scan. Runs after the mode transitions, so at a
    /// CONT splice it sees the adopted prefix and the new take's limit.
    pub(crate) fn scan_drift(&mut self) {
        if let Some(shared) = self.conn.shared.as_ref() {
            self.view.drift.scan(
                shared.state(),
                self.take.trajectory_limit(),
                &mut self.log_lines,
            );
        }
    }
}
