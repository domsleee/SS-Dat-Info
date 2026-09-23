mod drift_scan;
mod history_store;
mod level;
mod panels;
mod pico;
mod recording;
mod relaunch;
mod script_watch;
mod settings;
mod start_line;
mod ui_log;
mod win32;
mod worker;

use eframe::egui;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use relaunch::CheckpointOwner;

/// Identifiers for the four TAS shortcut keys, used both for
/// `poll_global_shortcuts` and for the pure edge-detector unit tests
/// (which can't link Win32). Order matches `GLOBAL_SHORTCUT_KEYS`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GlobalShortcutSlot {
    F9 = 0,
    F10 = 1,
    F11 = 2,
    F12 = 3,
}

/// Compute press-edge transitions for the four shortcut keys. Pure
/// function so we can unit-test the edge logic without faking Win32.
/// Mutates `prev` to current so the caller's state stays in sync.
fn compute_global_key_edges(now: [bool; 4], prev: &mut [bool; 4]) -> [bool; 4] {
    let mut edges = [false; 4];
    for i in 0..4 {
        edges[i] = now[i] && !prev[i];
        prev[i] = now[i];
    }
    edges
}

use panels::{config, history, input_script, log_panel, status, timeline, transport};
use pico::PicoState;
use recording::{RecordingHistory, RecordingSessionKind};

const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
const PLAYBACK_SPEED_PRESETS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];
// Shared with the tas_test harness so both give up after the same attempts.
const ALIGN_MAX_RETRIES: u32 = tas_shared::align::ALIGN_MAX_RETRIES;

/// How long one transport cycle may run before the UI gives up on it.
///
/// Generous on purpose: this is a stall guard, not a performance bound. A
/// deep CONT catch-up, or a PLAY watched for ALIGN_VERIFY_FRAMES past the gate
/// at 1x, can take tens of seconds. It exists so a cycle that will never
/// finish cannot hold the input block forever.
const CYCLE_BUDGET: std::time::Duration = std::time::Duration::from_secs(180);

fn normalize_playback_speed(speed: f32) -> f32 {
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

fn stop_is_acknowledged(mode: u32, command_idle: bool) -> bool {
    mode == TasMode::Off as u32 && command_idle
}

/// `stop_pending` = a STOP is already published or being consumed. Any OTHER
/// pending command (e.g. a tick-scheduled arm that never fired because the
/// level was left) must not suppress the auto-stop: STOP overwrites it.
fn should_request_auto_stop(
    racing: bool,
    cycle_frozen: bool,
    debounced: bool,
    stop_pending: bool,
) -> bool {
    racing && cycle_frozen && !debounced && !stop_pending
}

#[derive(Clone, Copy)]
struct ActiveRecordingSession {
    kind: RecordingSessionKind,
    start_tick: u32,
    max_recorded_count: u32,
}

struct TasApp {
    shared: Option<TasSharedMemoryClient>,
    connect_error: Option<String>,

    // UI state
    show_config: bool,
    show_pico_panel: bool,
    pico: PicoState,
    history: RecordingHistory,
    history_writer: Option<history_store::HistoryWriter>,
    /// Highest history revision the worker confirmed durably committed.
    last_persisted_revision: u64,
    /// Highest revision currently queued/in flight. Kept separate from durable
    /// state so a failed background write remains dirty and can be retried.
    last_queued_revision: u64,
    last_failed_revision: u64,
    history_retry_after: Option<std::time::Instant>,
    /// Last automatic `try_reconnect` while disconnected.
    last_reconnect_attempt: std::time::Instant,
    /// Physics-mode stamp (renderer + x87 precision) of the take currently in
    /// the recording buffer: the last loaded file / restored entry, or the live
    /// mode when a REC started. Shown next to the live mode; a mismatch means
    /// the buffer's inputs were recorded under different rounding.
    loaded_physics: Option<String>,
    /// Rider stamp (character · stance) of the take in the recording buffer,
    /// same lifecycle as `loaded_physics`.
    loaded_rider: Option<String>,
    /// Raw identity words of the take in the recording buffer, same lifecycle
    /// as the labels above. Save paths stamp the file with these, never the
    /// live game — `None` halves stay unknown.
    loaded_identity: Option<recording::IdentityStamps>,
    /// Soft cap (max unpinned entries) — from settings.
    history_cap: usize,
    recovery_store: Option<recording::RecoveryStore>,
    /// Serialized off-thread writer for recovery checkpoints. One writer keeps
    /// writes ordered, so a late write can't land after `clear_pending` and
    /// resurrect a stale checkpoint as a duplicate "Recovered" entry.
    recovery_writer: recording::RecoveryWriter,
    log_lines: ui_log::UiLog,
    timeline_view: timeline::TimelineView,
    timeline_edit: timeline::TimelineEdit,
    /// Input edit (new full event list, commit-undo flag) produced by the
    /// timeline this frame, applied to `input_log` at the start of the next.
    pending_input_edit: Option<(Vec<input_script::InputEvent>, bool, String)>,
    /// Latch: a pending edit arrived while a run was active, so we issued one
    /// auto-STOP and are now waiting for mode→Off to apply it. Prevents
    /// re-sending STOP (and re-logging) every frame while the game settles.
    pending_edit_autostop: bool,
    /// Stable-content watcher for this recording's external script.
    script_watch: Option<script_watch::ScriptWatch>,
    continue_from_frame: u32,
    continue_from_text: String,
    playback_speed: f32,
    show_history: bool,
    show_log: bool,
    segment_tracker: recording::SegmentTracker,
    active_recording_session: Option<ActiveRecordingSession>,
    pending_session_kind: Option<RecordingSessionKind>,
    pending_continue_start_tick: Option<u32>,
    last_mode: u32,
    resume_speed: Option<f32>, // speed to return to when a CONT catch-up ends
    cont_catchup_multiplier: f32, // configurable CONT catch-up speed (default 12x)
    /// Which transport (PLAY or CONT) the in-flight cycle is for, so retry and
    /// abort log lines name the right one.
    cycle_arm: tas_shared::transport::Arm,
    log_read_cursor: u32,
    /// Finish-line watcher: scan cursor into rec_coords during REC so
    /// each frame only examines new ticks, and the tick the run crossed the
    /// finish (drives the auto-stop + the 🏁 marker; reset when REC starts).
    finish_scan_cursor: u32,
    finished_at_tick: Option<u32>,
    /// The HUD race time latched when the crossing was detected. Re-reading
    /// the live timer at STOP is unreliable: it may have blanked or moved.
    finished_hud_cs: Option<u32>,

    drift_tracker: drift_scan::DriftTracker,
    last_logged_drift_level: u8, // 0=none, 1=any, 2=>=1.0, 3=>=5.0

    /// In-flight restart → arm → watch cycle, driven by the shared
    /// `tas_shared::transport` controller, the same state machine the tas_test
    /// harness runs. Stepped from each egui update via `step_cycle`, which may
    /// run several transitions per update; `None` when idle. The shared
    /// `command` slot holds one u32, so each command waits for the DLL to
    /// acknowledge the previous one (Stop → wait OFF → Restart).
    cycle: Option<tas_shared::transport::TransportController>,
    /// Wall-clock deadline for the in-flight cycle to make a terminal
    /// transition. `None` when idle.
    ///
    /// The controller has no clock, so a phase that stops progressing (an F5
    /// restart that never completes, an arm the DLL never processes) returns
    /// InProgress forever, with `cont_suppress_input` set and the keyboard
    /// swallowed. This deadline aborts such a cycle.
    cycle_deadline: Option<std::time::Instant>,
    /// The last CONT cycle's result (attempts, how it completed), carried from
    /// controller `Done` to the REC-start transition, where the resume frame is
    /// known, so one summary line can report both.
    cont_last_outcome: Option<(u32, tas_shared::transport::CompletedVia)>,
    /// Previous-frame pressed state for F9, F10, F11, F12, diffed against
    /// GetAsyncKeyState to detect press edges. Updated every frame regardless
    /// of focus so a focus change can't leave a key stuck "pressed".
    prev_global_keys: [bool; 4],
    /// Cached Supreme.exe PID. Resolved on first poll, invalidated when the
    /// shared-memory connection drops (game closed or restarted).
    game_pid_cached: Option<u32>,
    /// Supreme.exe PID seen by the last 1 Hz health check: a relaunch is
    /// noticed by identity even when the frame counter cannot show it (a game
    /// replaced at the main menu goes 0 → 0).
    game_pid_seen: Option<u32>,
    /// The PID watcher reset the session for a relaunch whose memset has not
    /// been seen yet: the coming frame-counter regression only restarts the
    /// ring, it must not reset (and cancel) whatever started in between.
    expect_ring_restart: Option<u32>,
    /// When `cont_suppress_input` was first seen set with the DLL idle and no
    /// cycle of ours in flight, plus the arm generation at that moment. The
    /// flag is released only after it has stayed that way for the grace
    /// period — a harness cycle's restart/settle phase legitimately holds it
    /// in OFF for a couple of seconds, and must not lose it to a UI that
    /// happens to connect then.
    stale_protection_since: Option<(std::time::Instant, u32)>,
    // Crash recovery
    last_frame_count: u32,
    stale_frame_ticks: u32,
    last_health_check: std::time::Instant,
    // Engine-activity tracker. game_in_game (exe+0x8895C) is written by the
    // Supreme::Cycle hook, so when the cycle stops (menu, pause, dialog) it
    // freezes at 1 instead of going to 0. frame_count only advances while the
    // cycle runs, so a recent advance means a level is really ticking.
    // Sampled every frame.
    cycle_fc: u32,
    // True once cycle_fc holds a real baseline sample, so the first sample
    // after launch or reconnect is not mistaken for an advance.
    cycle_fc_seeded: bool,
    cycle_advance_at: std::time::Instant,
    // The menu auto-stop debounce, on its own clock so it never makes the
    // engine look alive to the transport gate.
    auto_stop_debounce: Option<std::time::Instant>,

    // The last track we were confidently on, used only to name a save: the
    // engine's post-run dialog stops the cycle, so the live level reads
    // unknown just as the user clicks Save. Never used to decide what may be
    // restored; that must be live.
    last_resolved_level: Option<String>,
    // The level_epoch we were last resolved in. "Unresolved" has two causes
    // that need opposite handling: a freeze (menu, pause, post-race dialog;
    // the level is still resident and the epoch unchanged) and a real context
    // change (level_scan saw the level path change or disappear and bumped the
    // epoch). Only the second should hide the history panel.
    last_resolved_epoch: Option<u32>,

    // One-shot: force dark title bar on first frame
    #[cfg(windows)]
    dark_title_bar_set: bool,
}

/// Whether the recovery checkpoint may be deleted: only when the recovered
/// take is durably committed to history. A missing history writer is NOT
/// durability — the checkpoint files are then the only copy in existence,
/// and deleting them loses the recovered take permanently.
fn checkpoint_clear_decision(flush: Option<Result<(), String>>) -> (bool, Option<String>) {
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
    fn new() -> Self {
        let (shared, connect_error) = match TasSharedMemoryClient::open() {
            Ok(s) => (Some(s), None),
            Err(e) => (None, Some(e)),
        };

        let settings = settings::Settings::load();
        // File-per-entry history store.
        let history_cap = settings.history_cap.max(1);
        let mut history = RecordingHistory::new(history_cap);
        let history_dir = history_store::default_history_dir();
        let mut history_notices: Vec<String> = Vec::new();
        let history_writer = match history_store::HistoryWriter::open(history_dir.clone()) {
            Ok((writer, load)) => {
                // Entries load lazily: a restore reads its blob from here.
                history.set_blob_dir(history_dir.clone());
                if load.entries.is_empty() {
                    // Preserve the computed id floor even for an empty or
                    // damaged manifest so a new entry cannot reuse a blob id.
                    history.adopt_id_floor(load.next_entry_id);
                } else {
                    history.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
                }
                for warning in load.warnings {
                    history_notices.push(format!("History: {}", warning));
                }
                history_notices.push(format!("History store: {}", history_dir.display()));
                Some(writer)
            }
            Err(e) => {
                history_notices.push(format!("History store disabled: {}", e));
                None
            }
        };
        // One-time level backfill: tag pre-tagging entries by classifying
        // their snapshot's spawn position (only unambiguous spawns — shared
        // Alpine / FM-FH clusters stay untagged and remain visible on every
        // level). Idempotent: already-tagged entries are skipped, so this is
        // a no-op on every launch after the first.
        let backfilled = history.backfill_levels(start_line::level_code_from_spawn);
        if backfilled > 0 {
            history_notices.push(format!(
                "History: backfilled level tags on {} entries (by spawn position)",
                backfilled
            ));
        }
        let recovery_store = recording::RecoveryStore::new().ok();
        let recovery_store_notice = recovery_store
            .as_ref()
            .map(|store| format!("Crash recovery: {}", store.root().display()));
        let mut app = Self {
            shared,
            connect_error,
            show_config: settings.show_config,
            show_pico_panel: settings.show_pico_panel,
            pico: PicoState::new(),
            history,
            history_writer,
            last_persisted_revision: 0,
            last_queued_revision: 0,
            last_failed_revision: 0,
            history_retry_after: None,
            last_reconnect_attempt: std::time::Instant::now(),
            loaded_physics: None,
            loaded_rider: None,
            loaded_identity: None,
            history_cap,
            recovery_store,
            recovery_writer: recording::RecoveryWriter::new(),
            log_lines: ui_log::UiLog::new(&history_dir),
            timeline_view: timeline::TimelineView::default(),
            timeline_edit: timeline::TimelineEdit::default(),
            pending_input_edit: None,
            pending_edit_autostop: false,
            script_watch: None,
            continue_from_frame: 0,
            continue_from_text: "0".to_string(),
            playback_speed: normalize_playback_speed(settings.playback_speed),
            show_history: settings.show_history,
            show_log: settings.show_log,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            resume_speed: None,
            cont_catchup_multiplier: settings.cont_catchup_speed,
            cycle_arm: tas_shared::transport::Arm::Continue,
            log_read_cursor: 0,
            finish_scan_cursor: 0,
            finished_at_tick: None,
            finished_hud_cs: None,
            drift_tracker: drift_scan::DriftTracker::default(),
            last_logged_drift_level: 0,
            cycle: None,
            cycle_deadline: None,
            cont_last_outcome: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
            game_pid_seen: None,
            expect_ring_restart: None,
            stale_protection_since: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            cycle_fc: 0,
            cycle_fc_seeded: false,
            // Ancient, not now(): "ticking" must be FALSE until a real
            // frame_count advance is observed.
            cycle_advance_at: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(600))
                .unwrap_or_else(std::time::Instant::now),
            auto_stop_debounce: None,
            last_resolved_level: None,
            last_resolved_epoch: None,
            #[cfg(windows)]
            dark_title_bar_set: false,
        };

        // Auto-detect Pico on startup
        let detect_logs = if std::env::var_os("SSB_INSPECT_E2E_RECORDING").is_some() {
            Vec::new() // The live test owns the physical Pico input.
        } else {
            app.pico.auto_detect()
        };
        for msg in detect_logs {
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
        let recovered_checkpoint = app.recover_pending_checkpoint();
        app.persist_history_if_needed();

        // Clear the checkpoint only after the recovered entry is confirmed
        // durable in the history store; on a failed write keep it so the next
        // launch recovers it again.
        if recovered_checkpoint {
            app.clear_recovery_after_durable_persist();
        }
        app.poll_stale_input_protection();

        if let Some(path) = std::env::var_os("SSB_INSPECT_E2E_RECORDING") {
            let setup = (|| -> Result<(), String> {
                if std::env::var_os("SSB_INSPECT_DATA_DIR").is_none() {
                    return Err("E2E startup requires an isolated SSB_INSPECT_DATA_DIR".into());
                }
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

    fn prepare_e2e_recording(&mut self, path: &std::path::Path, splice: u32) -> Result<(), String> {
        let shared = self.shared.as_mut().ok_or("No game connection")?;
        if shared.mode_volatile() != TasMode::Off as u32 || !shared.command_idle() {
            return Err("Game must be stopped before UI fixture loading".into());
        }
        let metadata = recording::RecordingFile::read_metadata(path)?;
        if splice == 0 || splice > metadata.recorded_count {
            return Err("Splice outside recording".into());
        }
        tas_shared::level::check_recording_matches_live(
            &path.to_string_lossy(),
            shared.state().level_id,
        )?;
        if !recording::load_recording_path(
            shared.state_mut(),
            &mut self.segment_tracker,
            &mut self.log_lines,
            path,
        ) {
            return Err("Could not load UI fixture".into());
        }
        self.loaded_physics = metadata.physics_label();
        self.loaded_rider = metadata.rider_label();
        self.loaded_identity = Some(recording::IdentityStamps::from_metadata(&metadata));
        self.history.push_loaded_snapshot(
            shared.state(),
            path,
            Some(recording::IdentityStamps::from_metadata(&metadata)),
        );
        self.apply_transport_action(transport::Action::SetContinueFrame(splice));
        Ok(())
    }

    fn playback_speed_for_settings(&self) -> f32 {
        let base_speed = self.resume_speed.unwrap_or(self.playback_speed);
        normalize_playback_speed(base_speed)
    }

    fn try_reconnect(&mut self) {
        match TasSharedMemoryClient::open() {
            Ok(s) => {
                self.detach_input_editor();
                self.shared = Some(s);
                self.connect_error = None;
                // A fresh mapping = a fresh frame_count stream: force the
                // heartbeat to re-baseline instead of treating the first
                // sample as an advance (transport gate false-positive).
                self.cycle_fc_seeded = false;
                self.push_log("Connected to TAS_Helper.dll shared memory");
                self.poll_stale_input_protection();
            }
            Err(e) => self.connect_error = Some(e),
        }
    }

    /// After a history restore: remember the entry's physics-mode stamp for
    /// the status chip and warn if it differs from the live mode (24-bit
    /// DirectX vs 53-bit OpenGL round the sim differently).
    fn note_restored_physics(&mut self) {
        let Some(idx) = self.history.current_index() else {
            return;
        };
        let stamp = self
            .history
            .entries()
            .get(idx)
            .and_then(|e| e.physics.clone());
        if let (Some(stamp), Some(live)) = (stamp.as_deref(), self.history.live_physics()) {
            if stamp != live {
                self.log_lines.push(format!(
                    "WARNING: this take was recorded under {} but the game is running {}: \
                     the physics round differently, a replay will not be bit-exact",
                    stamp, live
                ));
            }
        }
        self.loaded_physics = stamp;
        // Who rode the take vs who is on the board now (character / stance).
        let rider = self
            .history
            .entries()
            .get(idx)
            .and_then(|e| e.rider.clone());
        if let (Some(stamp), Some(live)) = (rider.as_deref(), self.history.live_rider()) {
            if stamp != live {
                self.log_lines.push(format!(
                    "WARNING: this take was recorded as {} but the rider is {}: \
                     a different character or stance has different physics, a replay will not line up",
                    stamp, live
                ));
            }
        }
        self.loaded_rider = rider;
        self.loaded_identity = self.history.entries().get(idx).map(|e| e.stamps.clone());
    }

    fn push_log(&mut self, msg: &str) {
        self.log_lines.push(msg);
    }

    /// Append any newly-pushed log lines to the session log file. Called
    /// once per UI frame from `update`. If the file is unavailable
    /// (couldn't open at start) this is a no-op — the in-memory log
    /// remains the only record.
    fn flush_log_lines_to_file(&mut self) {
        self.log_lines.flush_to_file();
    }

    fn clear_cont_catchup(&mut self) {
        if let Some(saved) = self.resume_speed.take() {
            self.playback_speed = saved;
        }
    }

    fn reset_continue_runtime_state(&mut self) {
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
    fn set_cont_suppress_input(&mut self, on: bool) {
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

    fn send_action_command(&mut self, command: TasCommand) {
        if command == TasCommand::Stop {
            self.clear_cont_catchup();
            // Also cancels any in-flight cycle, so STOP after a RestartThen
            // click can't complete the restart and start REC/PLAY.
            self.reset_continue_runtime_state();
        }
        if let Some(shared) = self.shared.as_mut() {
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
    fn stop_active_session_for_load(&mut self) -> bool {
        self.detach_input_editor();
        let (mode, command_idle) = self
            .shared
            .as_ref()
            .map(|s| (s.mode_volatile(), s.command_idle()))
            .unwrap_or((TasMode::Off as u32, true));
        // The DLL can be idle while the UI still has a restart/arm queued.
        // Cancel that controller through STOP before replacing its recording.
        if stop_is_acknowledged(mode, command_idle) && self.cycle.is_none() {
            self.sync_live_level();
            return true;
        }
        let was_rec = mode == TasMode::Rec as u32;
        self.log_lines.push("Stopping active session before load");
        self.send_action_command(TasCommand::Stop);
        // Spin up to ~250ms for the DLL's cycle hook to process CMD_STOP and
        // flip to OFF (typically 1-2 cycles, ~7-14ms).
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        let mut stopped = false;
        while std::time::Instant::now() < deadline {
            stopped = self
                .shared
                .as_ref()
                .map(|s| stop_is_acknowledged(s.mode_volatile(), s.command_idle()))
                .unwrap_or(true);
            if stopped {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        if !stopped {
            stopped = self
                .shared
                .as_ref()
                .map(|s| stop_is_acknowledged(s.mode_volatile(), s.command_idle()))
                .unwrap_or(true);
        }
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
            if let Some((recorded, snap)) = self.shared.as_ref().map(|s| {
                (
                    s.recorded_count_volatile(),
                    recording::RecordingSnapshot::from_state(s.state()),
                )
            }) {
                self.segment_tracker.on_rec_stop(recorded);
                self.finalize_recording_session(&snap, recorded);
            }
        }
        self.last_mode = TasMode::Off as u32;
        // A fresh recording is about to load — clear the finish-flag marker.
        self.finished_at_tick = None;
        self.finished_hud_cs = None;
        self.finish_scan_cursor = 0;
        // Re-read the live level: the callers' level filter was read at frame
        // start, before the wait above, and every caller is about to overwrite
        // the buffer. A track change during the wait must not let another
        // track's recording through.
        self.sync_live_level();
        true
    }

    /// Pull the live track from shared memory into the history filter, which is
    /// what undo / redo / restore all consult before overwriting the buffer.
    ///
    /// One coherent seqlock read: id and path together, or nothing. Asking
    /// "resolved?" and then reading `level_id` separately could hand back "yes"
    /// plus the previous track. `None` means unknown, and unknown matches
    /// nothing — never "assume we are still where we were".
    fn sync_live_level(&mut self) {
        let Some(shared) = self.shared.as_ref() else {
            return;
        };
        // Renderer + x87 precision and rider (character · stance): stamped onto
        // every pushed history entry and compared on restore/load.
        self.history.set_live_physics(shared.physics_mode());
        self.history.set_live_rider(shared.rider());
        self.history
            .set_live_stamps(recording::IdentityStamps::from_live(shared.state()));
        // id and epoch from one seqlock window: a separate epoch read could
        // pair the old track's id with the new epoch across a switch.
        match tas_shared::resolved_level_id_with_epoch(shared.state()) {
            Some((id, epoch)) => {
                let code = crate::level::level_code_from_id(id);
                if let Some(c) = code {
                    self.last_resolved_level = Some(c.to_string());
                }
                self.last_resolved_epoch = Some(epoch);
                self.history.set_live_level(code);
            }
            None => {
                // A freeze (menu, pause, post-race dialog) also reads as
                // unresolved but leaves level_epoch unchanged, and the history
                // panel must stay. Only a moved epoch hides the rows until the
                // new track is identified. A plain epoch read is fine here:
                // this branch stamps nothing, so a torn read costs one frame.
                let epoch_now = shared.state().level_epoch;
                if self.last_resolved_epoch != Some(epoch_now) {
                    self.history.enter_resolving();
                }
            }
        }
    }

    /// The track a recording being saved right now belongs to: the live level if
    /// we have it, else the last one we were confidently on. See
    /// [`recording::save_dialog_with_segments`] for why this is not read live.
    fn level_for_save(&self) -> Option<&str> {
        crate::level::level_for_save(
            self.shared
                .as_ref()
                .and_then(|s| crate::level::resolved_level_code(s.state())),
            self.last_resolved_level.as_deref(),
        )
    }

    /// Single dispatch for a transport `Action`, shared by the keyboard-shortcut
    /// path and the transport-bar button path so the two can never diverge.
    fn apply_transport_action(&mut self, cmd: transport::Action) {
        match cmd {
            transport::Action::Send(c) => self.send_action_command(c),
            transport::Action::RestartThen(c) => self.queue_restart_then(c),
            // Undo/Redo overwrite the whole shared input/coord buffer like a
            // history restore, so they stop an active REC/PLAY first; the DLL
            // must not be writing input_log during the copy.
            transport::Action::Undo => {
                if self.stop_active_session_for_load() {
                    if let Some(snap) = self.history.undo() {
                        if let Some(shared) = self.shared.as_mut() {
                            snap.restore_to(shared.state_mut());
                        }
                        self.log_lines.push("Undo: restored previous recording");
                        self.note_restored_physics();
                    }
                }
            }
            transport::Action::Redo => {
                if self.stop_active_session_for_load() {
                    if let Some(snap) = self.history.redo() {
                        if let Some(shared) = self.shared.as_mut() {
                            snap.restore_to(shared.state_mut());
                        }
                        self.log_lines.push("Redo: restored next recording");
                        self.note_restored_physics();
                    }
                }
            }
            transport::Action::SetContinueFrame(frame) => {
                // Stage the splice frame UI-side only; the TransportController
                // writes it to shared memory when a cycle starts and on each
                // reroll, and the DLL clears it. Writing it here mid-replay is
                // harmless for plain PLAY (cave2 splices only an armed CONT)
                // but would move a running CONT's splice.
                self.continue_from_frame = frame;
                self.continue_from_text = frame.to_string();
            }
            transport::Action::SetResumeSpeed(spd) => {
                // Catch-up in flight: keep the saved resume speed and the shared
                // cont_resume_speed in sync so the DLL drops to the speed the
                // user just picked at the splice. playback_speed stays the
                // catch-up multiplier until clear_cont_catchup restores it.
                self.resume_speed = Some(spd);
                if let Some(shared) = self.shared.as_mut() {
                    shared.state_mut().cont_resume_speed = spd;
                }
                self.log_lines.push(format!("Resume speed: {}x", spd));
            }
            transport::Action::Log(msg) => {
                self.log_lines.push(msg);
            }
        }
    }

    /// Display name of the in-flight controller cycle for log lines.
    fn cycle_label(&self) -> &'static str {
        match self.cycle_arm {
            tas_shared::transport::Arm::Continue => "CONT",
            tas_shared::transport::Arm::Play => "PLAY",
            tas_shared::transport::Arm::Rec => "REC",
        }
    }

    fn queue_restart_then(&mut self, command: TasCommand) {
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
    fn step_cycle(&mut self, ctx: &egui::Context) {
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
                StepOutcome::Wait { ms } => {
                    // Fixed settle; loop straight on to the next command.
                    std::thread::sleep(std::time::Duration::from_millis(ms));
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

    /// Clear the crash-recovery checkpoint, but only after the history it
    /// represents is durably committed (persist → flush → clear). If the flush
    /// fails the checkpoint is kept, so the next launch recovers it. The
    /// recovery writer is drained first so no in-flight write can recreate the
    /// checkpoint files after they are deleted.
    fn clear_recovery_after_durable_persist(&mut self) {
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

    fn persist_history_if_needed(&mut self) {
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

    /// Discard editors and queued gestures when their recording is replaced.
    fn detach_input_editor(&mut self) {
        self.script_watch = None;
        self.pending_input_edit = None;
        self.pending_edit_autostop = false;
        self.timeline_edit = timeline::TimelineEdit::default();
    }

    /// Apply a pending input edit to the stopped recording before rendering,
    /// pushing one undo snapshot per finished gesture.
    fn apply_pending_input_edit(&mut self) {
        // Peek without consuming — if a run is active we keep the edit queued
        // and apply it once the game stops.
        if self.pending_input_edit.is_none() {
            return;
        }
        let mode = match self.shared.as_ref() {
            Some(shared) => shared.state().mode,
            None => {
                // No shared memory to write into — drop the queued edit.
                self.pending_input_edit = None;
                self.pending_edit_autostop = false;
                return;
            }
        };
        // Never mutate the buffer the game is replaying or recording. Auto-STOP
        // the run once (latched) and keep the edit queued until mode is Off.
        if mode != TasMode::Off as u32 {
            if !self.pending_edit_autostop {
                self.pending_edit_autostop = true;
                self.log_lines.push("[script] stopping run to apply edit…");
                self.send_action_command(TasCommand::Stop);
            }
            return;
        }
        if self
            .shared
            .as_ref()
            .is_some_and(|shared| !shared.command_idle())
        {
            return;
        }
        self.pending_edit_autostop = false;
        let (events, commit, label) = match self.pending_input_edit.take() {
            Some(e) => e,
            None => return,
        };
        let shared = match self.shared.as_mut() {
            Some(shared) => shared,
            None => return,
        };
        let total = shared.state().recorded_count;
        input_script::apply_events_to_log(&mut shared.state_mut().input_log, total, &events);
        if commit {
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            // end_tick = 0 routes through the history panel's label parser so
            // the descriptive action (e.g. "Moved L 324→372t") is what shows.
            let label = if label.is_empty() {
                "Edited inputs".to_string()
            } else {
                label
            };
            self.history.push_snapshot_data_with_session(
                snapshot,
                label,
                0,
                0,
                self.loaded_identity.clone(),
            );
        }
    }

    /// Poll the externally-edited `.tas` file; on save, parse it and queue
    /// the inputs for application (reload-on-save). No-op until a file is
    /// opened via "Open in external editor".
    fn poll_script_file(&mut self) {
        let Some(result) = self.script_watch.as_mut().and_then(|watch| watch.poll()) else {
            return;
        };
        match result {
            Ok(events) => {
                let n = events.len();
                self.pending_input_edit =
                    Some((events, true, "Loaded inputs from script".to_string()));
                self.log_lines
                    .push(format!("[script] reloaded {} inputs", n));
            }
            Err(e) => self.log_lines.push(format!("[script] reload failed: {e}")),
        }
    }

    fn start_recording_session(&mut self, continue_from_frame: u32, recorded_count: u32) {
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

    /// Emit one diagnostic line at the CONT splice: where the recording
    /// actually resumed (the requested splice tick), how many attempts it
    /// took, and whether the prefix was watched.
    fn log_cont_resume_summary(&mut self) {
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

    fn update_recording_recovery_progress(&mut self, snapshot: &recording::RecordingSnapshot) {
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

    fn finalize_recording_session(
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

    /// Check if the game process is still alive by monitoring frame_count advancement.
    /// If frame_count hasn't changed for ~3 seconds, assume the game crashed.
    fn check_game_health(&mut self) {
        // Sample cycle activity every frame (cheap): game_in_game freezes when
        // the Supreme::Cycle hook stops, so a fresh frame_count advance is the
        // real "ticking a level" signal.
        if let Some(fc) = self
            .shared
            .as_ref()
            .map(|shared| shared.frame_count_volatile())
        {
            if !self.cycle_fc_seeded {
                // Baseline only: a frozen menu has a nonzero frame_count too,
                // so the first sample is not evidence of ticking.
                self.cycle_fc_seeded = true;
                self.cycle_fc = fc;
            } else if fc < self.cycle_fc {
                // The counter only increments (once per cycle), so going
                // backwards means a fresh DLL zeroed the section.
                self.on_dll_reinitialised(fc);
            } else if fc != self.cycle_fc {
                self.cycle_fc = fc;
                self.cycle_advance_at = std::time::Instant::now();
            }
        }
        if self.last_health_check.elapsed() < std::time::Duration::from_secs(1) {
            return;
        }
        self.last_health_check = std::time::Instant::now();

        // Game identity: a relaunch that reuses our mapped section is only
        // visible here when the old process never ticked (see
        // on_game_process_changed). Sampled once a second, so a process
        // snapshot is affordable.
        if self.shared.is_some() {
            let pid = win32::find_supreme_pid();
            self.on_game_pid_observed(pid);
        }
        self.poll_stale_input_protection();

        if let Some(ref shared) = self.shared {
            let current_frame = shared.frame_count_volatile();
            if current_frame == self.last_frame_count {
                self.stale_frame_ticks += 1;
                // Re-test liveness every 5 stale seconds, not once: the cycle
                // also freezes at the menu while the game is alive, and the
                // game may exit later while still stale.
                if self.stale_frame_ticks.is_multiple_of(5) && !win32::is_supreme_running() {
                    self.disconnect_from_dead_game();
                }
            } else {
                self.last_frame_count = current_frame;
                self.stale_frame_ticks = 0;
            }
        }
    }

    /// Poll the global keyboard state for F9–F12 and emit the same
    /// transport actions the in-window shortcut handler would, but only
    /// when Supreme.exe is the foreground window. Lets the user trigger
    /// REC/PLAY/STOP/CONT without alt-tabbing to tas_ui. The keys are
    /// observed, not consumed, so the game still receives them.
    ///
    /// Edge state is updated every frame regardless of focus so a key held
    /// across a focus change is not left "pressed".
    fn poll_global_shortcuts(&mut self) -> Vec<transport::Action> {
        let now: [bool; 4] =
            [win32::VK_F9, win32::VK_F10, win32::VK_F11, win32::VK_F12].map(win32::key_is_down);
        let edges = compute_global_key_edges(now, &mut self.prev_global_keys);
        if !edges.iter().any(|&e| e) {
            return Vec::new();
        }

        // Only act while Supreme.exe is the foreground window. A stale cached
        // PID (game restarted without dropping shared memory) just fails to
        // match, which is harmless.
        let game_pid = self.game_pid_cached.or_else(|| {
            let resolved = win32::find_supreme_pid();
            self.game_pid_cached = resolved;
            resolved
        });
        let Some(game_pid) = game_pid else {
            return Vec::new();
        };
        if win32::foreground_window_pid() != Some(game_pid) {
            return Vec::new();
        }

        let mut actions = Vec::new();
        if edges[GlobalShortcutSlot::F9 as usize] {
            actions.extend(self.shortcut_arm("Global F9 (in-game): REC", TasCommand::ArmRec));
        }
        if edges[GlobalShortcutSlot::F10 as usize] {
            actions.extend(self.shortcut_arm("Global F10 (in-game): PLAY", TasCommand::ArmPlay));
        }
        if edges[GlobalShortcutSlot::F11 as usize] {
            actions.push(transport::Action::Send(TasCommand::Stop));
            actions.push(transport::Action::Log("Global F11 (in-game): STOP".into()));
        }
        if edges[GlobalShortcutSlot::F12 as usize] {
            actions
                .extend(self.shortcut_arm("Global F12 (in-game): CONT", TasCommand::ArmContinue));
        }
        actions
    }

    /// The menu gate: connected, in a level, and that level's cycle ticked
    /// within the last 400 ms. Arming drives an F5 restart, and at a menu or
    /// dialog the cycle is frozen (`game_in_game` alone is stale there), so an
    /// arm would fire into a stopped engine and leave a half-armed cycle.
    ///
    /// The button greying, the F9/F10/F12 refusals and `queue_restart_then`
    /// all read it here so they cannot drift apart.
    fn arming_allowed(&self) -> bool {
        self.shared.as_ref().is_some_and(|shared| {
            shared.state().game_in_game != 0
                && self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400)
        })
    }

    fn shortcut_arm(&mut self, source: &str, command: TasCommand) -> Vec<transport::Action> {
        let (mode, recorded) = match self.shared.as_ref() {
            Some(shared) => (shared.state().mode_enum(), shared.recorded_count_volatile()),
            None => {
                return vec![transport::Action::Log(format!(
                    "{source} ignored: not connected to the game"
                ))]
            }
        };
        if let Some(reason) = transport::arm_refusal(command, mode, recorded, self.arming_allowed())
        {
            return vec![transport::Action::Log(format!(
                "{source} ignored: {reason}"
            ))];
        }
        let mut actions = Vec::new();
        if command == TasCommand::ArmContinue {
            match transport::resolve_continue_frame(
                &mut self.continue_from_text,
                &mut self.continue_from_frame,
                recorded,
            ) {
                Ok(frame) => actions.push(transport::Action::SetContinueFrame(frame)),
                Err(reason) => {
                    return vec![transport::Action::Log(format!(
                        "{source} ignored: {reason}"
                    ))]
                }
            }
        }
        actions.push(transport::Action::RestartThen(command));
        actions.push(transport::Action::Log(source.to_string()));
        actions
    }

    /// Process keyboard shortcuts. Returns actions to execute.
    fn handle_shortcuts(&mut self, ctx: &egui::Context) -> Vec<transport::Action> {
        let mut actions = Vec::new();

        // Don't consume shortcuts when a text field has focus
        let any_text_focus = ctx.memory(|m| m.focused().is_some());

        ctx.input(|input| {
            let ctrl = input.modifiers.ctrl || input.modifiers.mac_cmd;

            // F5: Restart game (Pico F5)
            if input.key_pressed(egui::Key::F5) {
                if self.pico.connected {
                    win32::focus_game();
                    match self.pico.send_f5() {
                        Ok(()) => {
                            actions.push(transport::Action::Log("Shortcut: F5 restart".into()))
                        }
                        Err(e) => actions.push(transport::Action::Log(format!("F5 error: {}", e))),
                    }
                } else {
                    actions.push(transport::Action::Log("F5: Pico not connected".into()));
                }
            }

            // F9: Arm REC — same as clicking REC button (restart first)
            if input.key_pressed(egui::Key::F9) {
                actions.extend(self.shortcut_arm("Shortcut: F9 REC", TasCommand::ArmRec));
            }

            // F10: Arm PLAY — same as clicking PLAY button (restart first)
            if input.key_pressed(egui::Key::F10) {
                actions.extend(self.shortcut_arm("Shortcut: F10 PLAY", TasCommand::ArmPlay));
            }

            // F11: STOP
            if input.key_pressed(egui::Key::F11) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: F11 STOP".into()));
            }

            // F12: Continue record — same as clicking CONT button (restart first)
            if input.key_pressed(egui::Key::F12) {
                actions.extend(self.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue));
            }

            // Skip remaining shortcuts if text input has focus
            if any_text_focus {
                return;
            }

            // Space: STOP (toggle off)
            if input.key_pressed(egui::Key::Space) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: Space STOP".into()));
            }

            // Ctrl+Z: Undo (check raw events — egui may consume key_pressed for built-in undo)
            let ctrl_z_raw = input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Z, pressed: true, modifiers, .. }
                    if (modifiers.ctrl || modifiers.mac_cmd) && !modifiers.shift
                )
            });
            if ctrl_z_raw {
                actions.push(transport::Action::Undo);
                actions.push(transport::Action::Log("Shortcut: Ctrl+Z Undo".into()));
            }

            // Panel toggles: Ctrl+H (History), Ctrl+A (Analysis),
            // Ctrl+L (Log). Match on raw Key events for the same reason
            // as Ctrl+Z — egui may consume these as built-in shortcuts.
            for ev in input.events.iter() {
                if let egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } = ev
                {
                    if !(modifiers.ctrl || modifiers.mac_cmd) {
                        continue;
                    }
                    match key {
                        egui::Key::H => self.show_history = !self.show_history,
                        egui::Key::L => self.show_log = !self.show_log,
                        _ => {}
                    }
                }
            }

            // Ctrl+Y or Ctrl+Shift+Z: Redo
            let ctrl_redo_raw = input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Y, pressed: true, modifiers, .. }
                    if modifiers.ctrl || modifiers.mac_cmd
                )
            }) || input.events.iter().any(|e| {
                matches!(e,
                    egui::Event::Key { key: egui::Key::Z, pressed: true, modifiers, .. }
                    if (modifiers.ctrl || modifiers.mac_cmd) && modifiers.shift
                )
            });
            if ctrl_redo_raw {
                actions.push(transport::Action::Redo);
                actions.push(transport::Action::Log("Shortcut: Redo".into()));
            }

            // Ctrl+S: Save recording
            if ctrl && input.key_pressed(egui::Key::S) {
                actions.push(transport::Action::Log("Shortcut: Ctrl+S Save".into()));
            }

            // Ctrl+O: Open recording
            if ctrl && input.key_pressed(egui::Key::O) {
                actions.push(transport::Action::Log("Shortcut: Ctrl+O Open".into()));
            }

            // Plus/Equals: Zoom in timeline
            if input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals) {
                // handled below outside closure
            }

            // Minus: Zoom out timeline
            if input.key_pressed(egui::Key::Minus) {
                // handled below outside closure
            }
        });

        // Handle zoom and file operations outside the input closure to avoid borrow issues
        let (zoom_in, zoom_out, save, open) = ctx.input(|input| {
            let ctrl = input.modifiers.ctrl || input.modifiers.mac_cmd;
            (
                !any_text_focus
                    && (input.key_pressed(egui::Key::Plus) || input.key_pressed(egui::Key::Equals)),
                !any_text_focus && input.key_pressed(egui::Key::Minus),
                ctrl && input.key_pressed(egui::Key::S),
                ctrl && input.key_pressed(egui::Key::O),
            )
        });

        if zoom_in {
            self.timeline_view.zoom_center(0.8);
        }
        if zoom_out {
            self.timeline_view.zoom_center(1.25);
        }
        if save {
            // Resolve the track BEFORE the dialog: it belongs to the recording,
            // and the engine may be frozen in its own post-run dialog by now.
            let level = self.level_for_save().map(str::to_string);
            if let Some(shared) = self.shared.as_ref() {
                if let Some(path) = recording::save_dialog_with_segments(
                    shared.state(),
                    &self.segment_tracker.segments,
                    &mut self.log_lines,
                    level.as_deref(),
                    self.loaded_identity.as_ref(),
                ) {
                    self.history.push_save_marker(
                        shared.state(),
                        &path,
                        self.loaded_identity.clone(),
                    );
                }
            }
        }
        if open {
            self.load_recording_flow();
        }

        actions
    }

    /// Pick a recording, stop any active session, load it, then auto-play.
    /// The stop happens AFTER the (cancellable) file pick so cancelling the
    /// dialog never kills an in-progress recording, and BEFORE the load so the
    /// buffer overwrite doesn't race the DLL's REC cycle hook.
    fn load_recording_flow(&mut self) {
        // Resolved, not raw: the starting folder should be the track we are
        // actually on. Unknown opens the root recordings dir rather than
        // confidently opening the previous track's.
        let level_id = match self.shared.as_ref() {
            Some(s) => tas_shared::resolved_level_id(s.state()).unwrap_or(u32::MAX),
            None => return,
        };
        let Some(path) = recording::pick_recording_path(level_id) else {
            return;
        };
        if !self.stop_active_session_for_load() {
            return;
        }
        // The dialog only starts in the current track's folder; the user can
        // pick any file. Recordings are named `<CODE>-<name>`, so check the
        // file against the level re-read after the stop above. Unknown on
        // either side cannot prove a mismatch and is allowed, as in tas_test.
        let live_id = self
            .shared
            .as_ref()
            .and_then(|s| tas_shared::resolved_level_id(s.state()))
            .unwrap_or(u32::MAX);
        if let Err(msg) =
            tas_shared::level::check_recording_matches_live(&path.to_string_lossy(), live_id)
        {
            self.log_lines.push(format!("Load refused: {}", msg));
            return;
        }
        if let Some(shared) = self.shared.as_mut() {
            let loaded = recording::load_recording_path(
                shared.state_mut(),
                &mut self.segment_tracker,
                &mut self.log_lines,
                &path,
            );
            if loaded {
                // The file's stamp (load_recording_path already logged a
                // mismatch warning); the chip shows it next to the live mode.
                let meta = recording::RecordingFile::read_metadata(&path).ok();
                self.loaded_physics = meta.as_ref().and_then(|m| m.physics_label());
                self.loaded_rider = meta.as_ref().and_then(|m| m.rider_label());
                self.loaded_identity = meta.as_ref().map(recording::IdentityStamps::from_metadata);
                let _ = self.history.push_loaded_snapshot(
                    shared.state(),
                    &path,
                    meta.as_ref().map(recording::IdentityStamps::from_metadata),
                );
                if shared.state().recorded_count > 0 {
                    self.queue_restart_then(TasCommand::ArmPlay);
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
        let s = settings::Settings {
            show_pico_panel: self.show_pico_panel,
            show_history: self.show_history,
            show_config: self.show_config,
            show_log: self.show_log,
            playback_speed: self.playback_speed_for_settings(),
            cont_catchup_speed: self.cont_catchup_multiplier,
            history_cap: self.history_cap,
        };
        s.save();
        // Make sure the latest history is flushed to disk before we exit.
        if let Some(writer) = self.history_writer.as_ref() {
            let _ = writer.flush();
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Force dark theme + title bar on Windows (one-shot, first frame)
        #[cfg(windows)]
        if !self.dark_title_bar_set {
            self.dark_title_bar_set = true;
            ctx.set_theme(egui::Theme::Dark);
            win32::set_dark_title_bar("SSB Inspect");
        }

        // Persist any new log lines added since last frame to the on-disk
        // session log. Done first so a panic later in the frame still
        // captures the events that led up to it.
        self.flush_log_lines_to_file();

        // Synchronise the live level first: the history panel filters and
        // restores against it later this frame, and must not see the previous
        // track's value on the frame after a level change.
        self.sync_live_level();

        // Check game health (crash detection) — also samples cycle activity.
        self.check_game_health();

        // Auto-stop REC/PLAY when the player leaves the level. Quitting to the
        // menu stops Supreme::Cycle, so the DLL's own auto-stop (in the cycle
        // hook) can't fire. The 5 s threshold rides out an F5 reload stall and
        // a brief pause; a longer pause also stops, which is harmless.
        if let Some(ref shared) = self.shared {
            let mode = shared.mode_volatile();
            let racing = mode == TasMode::Rec as u32 || mode == TasMode::Play as u32;
            let frozen = self.cycle_fc != 0
                && self.cycle_advance_at.elapsed() > std::time::Duration::from_secs(5);
            let debounced = self
                .auto_stop_debounce
                .is_some_and(|t| t.elapsed() < std::time::Duration::from_secs(2));
            // Once STOP is pending, do not overwrite/re-log it every two
            // seconds. At a static menu the DLL's out-of-cycle fallback
            // acknowledges it because Supreme::Cycle is not running.
            let stop_pending = shared.stop_pending();
            if should_request_auto_stop(racing, frozen, debounced, stop_pending) {
                self.log_lines.push(format!(
                    "Auto-stopped: left the level (game cycle stopped while {})",
                    if mode == TasMode::Rec as u32 {
                        "recording"
                    } else {
                        "playing"
                    }
                ));
                self.send_action_command(TasCommand::Stop);
                // Debounce on its own timestamp: touching cycle_advance_at
                // would make the transport gate allow arming at a frozen menu.
                self.auto_stop_debounce = Some(std::time::Instant::now());
            }
        }

        // Track mode transitions for segment history + recovery checkpoints.
        let mode_probe = self.shared.as_ref().map(|shared| {
            (
                shared.mode_volatile(),
                shared.recorded_count_volatile(),
                shared.state().continue_from_frame,
            )
        });
        if let Some((current_mode, recorded, continue_from)) = mode_probe {
            // The snapshot copies ~850 KB (full input_log + rec_coords). Only a
            // live REC (recovery checkpoints) or a REC that just ended (history
            // entry) consumes it, so build it only then rather than every frame.
            let need_snapshot = current_mode == 1 || (self.last_mode == 1 && current_mode == 0);
            let state_snapshot = if need_snapshot {
                self.shared
                    .as_ref()
                    .map(|shared| recording::RecordingSnapshot::from_state(shared.state()))
            } else {
                None
            };
            if current_mode != self.last_mode {
                // REC started
                if current_mode == 1 {
                    self.start_recording_session(continue_from, recorded);
                    // A fresh take is by definition in the live physics mode.
                    self.loaded_physics = self.history.live_physics().map(str::to_string);
                    // Same for the rider: the take now in the buffer was recorded as the
                    // live character / stance, so a later PLAY compares against that.
                    self.loaded_rider = self.history.live_rider().map(str::to_string);
                    self.loaded_identity = self
                        .shared
                        .as_ref()
                        .map(|s| recording::IdentityStamps::from_live(s.state()));
                    self.log_cont_resume_summary();
                    self.clear_cont_catchup();
                    // Splice fired (or REC began). This handler runs before
                    // step_cycle, so REC can be seen before the controller
                    // reports Done. Drop it here, and release
                    // cont_suppress_input: the resumed REC records live input.
                    if self.cycle.take().is_some() {
                        self.cycle_deadline = None;
                        self.set_cont_suppress_input(false);
                    }
                    // Fresh finish-line watch. A CONT resumes mid-run, so scan
                    // from the resume tick; the prefix was checked when it was
                    // recorded.
                    self.finish_scan_cursor = continue_from.max(1);
                    self.finished_at_tick = None;
                    self.finished_hud_cs = None;
                }
                // REC stopped (mode went from REC to OFF)
                if self.last_mode == 1 && current_mode == 0 {
                    self.segment_tracker.on_rec_stop(recorded);
                    if let Some(snap) = state_snapshot.as_ref() {
                        self.finalize_recording_session(snap, recorded);
                    }
                }
                self.last_mode = current_mode;
            }
            if current_mode == 1 {
                if let Some(snap) = state_snapshot.as_ref() {
                    self.update_recording_recovery_progress(snap);
                }

                // Finish-line watch: stop REC when the run crosses the finish
                // line, since the run-out is never wanted. Only ticks since
                // the last frame are scanned.
                if self.finished_at_tick.is_none() {
                    let resolved_geometry = self
                        .shared
                        .as_ref()
                        .and_then(|sh| tas_shared::resolved_level_id(sh.state()))
                        .and_then(crate::level::level_code_from_id)
                        .is_some();
                    let cross = self.shared.as_ref().and_then(|shared| {
                        let s = shared.state();
                        // Resolved level only: a stale id would test the run
                        // against another track's finish line. Unresolved
                        // means no crossing is claimed, the safe direction.
                        crate::start_line::finish_cross_tick(
                            &s.rec_coords,
                            recorded,
                            tas_shared::resolved_level_id(s)
                                .and_then(crate::level::level_code_from_id),
                            self.finish_scan_cursor,
                        )
                    });
                    // Advance only past ticks actually scanned; with no
                    // geometry, advancing would skip a crossing permanently.
                    if resolved_geometry {
                        self.finish_scan_cursor = recorded.max(1);
                    }
                    if let Some(tick) = cross {
                        self.finished_at_tick = Some(tick);
                        // Latch the HUD time in the same poll that saw the
                        // crossing, as one coherent pair read.
                        self.finished_hud_cs = self
                            .shared
                            .as_ref()
                            .map(|s| tas_shared::race_pair(s.state()).0)
                            .filter(|&cs| cs != u32::MAX);
                        let hud = self
                            .finished_hud_cs
                            .map(|cs| {
                                format!(" (race time {})", recording::format_recording_duration(cs))
                            })
                            .unwrap_or_default();
                        self.log_lines.push(format!(
                            "\u{1F3C1} Finish line crossed at tick {}{} — recording stopped",
                            tick, hud
                        ));
                        self.apply_transport_action(transport::Action::Send(TasCommand::Stop));
                    }
                }
            }
        }

        // handle_shortcuts takes keys delivered to tas_ui by egui;
        // poll_global_shortcuts takes keys seen while the game has focus.
        // They are gated on the foreground window, so one press fires once.
        let mut shortcut_actions = self.handle_shortcuts(ctx);
        shortcut_actions.extend(self.poll_global_shortcuts());

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        let level = self.level_for_save().map(str::to_string);
                        if let Some(shared) = self.shared.as_ref() {
                            if let Some(path) = recording::save_dialog_with_segments(
                                shared.state(),
                                &self.segment_tracker.segments,
                                &mut self.log_lines,
                                level.as_deref(),
                                self.loaded_identity.as_ref(),
                            ) {
                                self.history.push_save_marker(
                                    shared.state(),
                                    &path,
                                    self.loaded_identity.clone(),
                                );
                            }
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        self.load_recording_flow();
                    }
                    ui.separator();
                    ui.label(
                        egui::RichText::new("Settings")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.horizontal(|ui| {
                        ui.label("CONT catch-up");
                        ui.add(
                            egui::DragValue::new(&mut self.cont_catchup_multiplier)
                                .range(1.0..=384.0)
                                .prefix("\u{00D7}")
                                .speed(1.0),
                        )
                        .on_hover_text(
                            "CONT catch-up replay speed. ~256× ≈ the game's physics \
                             ceiling (~80× effective); the F5 restart is separate and \
                             unaffected.",
                        );
                    });
                });
                ui.menu_button("View", |ui| {
                    // Everyday toggles, grouped under "Panels". The
                    // hotkey labels are advisory only — actual handling
                    // is in `handle_shortcuts`.
                    ui.label(
                        egui::RichText::new("Panels")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.show_history, "History");
                        ui.weak("Ctrl+H");
                    });
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.show_log, "Log");
                        ui.weak("Ctrl+L");
                    });
                    ui.separator();
                    // Debug section — rarely touched diagnostic toggles.
                    ui.label(
                        egui::RichText::new("Debug")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.checkbox(&mut self.show_pico_panel, "Pico HID");
                    ui.checkbox(&mut self.show_config, "Debug Config");
                });
            });
        });

        // Bottom log panel (hidden by default, toggle via View menu).
        // Declared FIRST so it sits at the very bottom of the window;
        // egui stacks subsequent bottom panels above it.
        if self.show_log {
            egui::TopBottomPanel::bottom("log_panel")
                .resizable(true)
                .default_height(100.0)
                .show(ctx, |ui| {
                    log_panel::show(ui, &mut self.log_lines);
                });
        }

        // Connection error state
        if self.connect_error.is_some() {
            // Retry every 2 s (the repaint floor below): the game is often
            // launched after tas_ui, and the mapping only exists once
            // TAS_Helper has initialized.
            if self.last_reconnect_attempt.elapsed() >= std::time::Duration::from_secs(2) {
                self.last_reconnect_attempt = std::time::Instant::now();
                self.try_reconnect();
            }
        }
        if self.connect_error.is_some() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading("SSB Inspect");
                    ui.add_space(20.0);
                    if let Some(ref err) = self.connect_error {
                        ui.colored_label(egui::Color32::from_rgb(255, 100, 100), err);
                    }
                    ui.add_space(10.0);
                    ui.label("Inject TAS_Helper.dll into Supreme.exe first.");
                    ui.add_space(10.0);
                    if ui.button("Retry Connection").clicked() {
                        self.try_reconnect();
                    }
                });
            });
            ctx.request_repaint_after(std::time::Duration::from_secs(2));
            return;
        }

        // Read current state snapshot for UI
        let connected = self.shared.is_some();
        if !connected {
            return;
        }

        // Left side panel: only shown if at least one sub-panel is visible
        let left_panel_visible = self.show_config || self.show_pico_panel;
        if left_panel_visible {
            egui::SidePanel::left("config_panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    if let Some(ref mut shared) = self.shared {
                        if self.show_config {
                            egui::CollapsingHeader::new("Debug Config")
                                .default_open(false)
                                .show(ui, |ui| {
                                    config::show(ui, shared.state());
                                });
                            ui.separator();
                        }
                        if self.show_pico_panel {
                            pico::show_panel(ui, &mut self.pico, &mut self.log_lines);
                        }
                    }
                    // History settings — available even when disconnected.
                    if self.show_config {
                        egui::CollapsingHeader::new("History")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Undo cap:");
                                    let mut cap = self.history_cap as u32;
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut cap)
                                                .range(10..=2000)
                                                .speed(2.0),
                                        )
                                        .on_hover_text(
                                            "Max UNPINNED entries kept. Pinned + current \
                                             entries are always kept, so the total can exceed this.",
                                        )
                                        .changed()
                                    {
                                        self.history_cap = cap.max(1) as usize;
                                        self.history.set_capacity(self.history_cap);
                                    }
                                });
                                ui.label(
                                    egui::RichText::new("pinned + current always kept")
                                        .size(10.0)
                                        .weak(),
                                );
                            });
                    }
                });
        } // left_panel_visible

        // Right-side history panel (optional)
        let mut history_actions = Vec::new();
        let history_dir = self
            .history_writer
            .as_ref()
            .map(|_| history_store::default_history_dir());
        let mut open_history_dir = false;
        if self.show_history {
            egui::SidePanel::right("history_panel")
                .resizable(true)
                .default_width(280.0)
                .show(ctx, |ui| {
                    // Compact header: "History" + count, autosave path in
                    // the tooltip.
                    ui.horizontal(|ui| {
                        let title = ui.label(
                            egui::RichText::new(format!("History · {}", self.history.len()))
                                .strong(),
                        );
                        if let Some(dir) = history_dir.as_ref() {
                            title.on_hover_text(format!("Autosave: {}", dir.display()));
                        }
                        if history_dir.is_some() {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .small_button("Open…")
                                        .on_hover_text("Open the autosave folder in Explorer")
                                        .clicked()
                                    {
                                        open_history_dir = true;
                                    }
                                },
                            );
                        }
                    });

                    // Same menu signal as the status chip (game_in_game is
                    // stale at menus, so also require the cycle ticking). The
                    // panel shows "In Menu" rather than "resolving…", since
                    // nothing is resolved at a menu.
                    let in_menu = !(self
                        .shared
                        .as_ref()
                        .map(|s| s.state().game_in_game != 0)
                        .unwrap_or(false)
                        && self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400));
                    let game_flag = self
                        .shared
                        .as_ref()
                        .map(|s| s.state().game_in_game != 0)
                        .unwrap_or(false);
                    history_actions = history::show(ui, &self.history, in_menu, game_flag);
                });
        }

        if open_history_dir {
            if let Some(dir) = history_dir.as_deref() {
                match open_in_file_browser(dir) {
                    Ok(()) => {
                        self.push_log(&format!("Opened history folder: {}", dir.display()));
                    }
                    Err(err) => {
                        self.push_log(&format!("Open history folder failed: {}", err));
                    }
                }
            }
        }

        // Process history panel restores
        if !history_actions.is_empty() {
            for action in history_actions {
                match action {
                    history::HistoryAction::Restore(idx) => {
                        // Restoring overwrites the live buffer, so stop any
                        // active REC/PLAY first. Resolve the row to its stable
                        // entry_id before stopping: finalizing the stopped take
                        // can evict the oldest entry and shift indices.
                        let target_id = self.history.entries().get(idx).map(|e| e.entry_id);
                        if !self.stop_active_session_for_load() {
                            continue;
                        }
                        let idx = target_id
                            .and_then(|id| {
                                self.history.entries().iter().position(|e| e.entry_id == id)
                            })
                            .unwrap_or(idx);
                        // Checked against the level re-read after the stop.
                        // Log the refusal: a silent no-op reads as a bug.
                        if !self.history.entry_on_current_level(idx) {
                            self.log_lines.push(format!(
                                "History restore refused: that entry is not for the \
                                 current track ({})",
                                self.history.live_level().unwrap_or("resolving")
                            ));
                            continue;
                        }
                        if let Some(shared) = self.shared.as_mut() {
                            let restored = self.history.restore_index(idx).map(|snap| {
                                snap.restore_to(shared.state_mut());
                                snap.recorded_count
                            });
                            if let Some(count) = restored {
                                let label = self
                                    .history
                                    .entries()
                                    .get(idx)
                                    .map(|entry| entry.label.clone())
                                    .unwrap_or_else(|| format!("Entry {}", idx + 1));
                                self.log_lines.push(format!("History restore: {}", label));
                                self.note_restored_physics();
                                // Fit the timeline to the whole loaded recording.
                                self.timeline_view.fit(count);
                            }
                        }
                    }
                    history::HistoryAction::ClearSelection => {
                        self.history.clear_selection();
                    }
                    history::HistoryAction::SetPin(id, pinned) => {
                        self.history.set_pinned(id, pinned);
                    }
                    history::HistoryAction::Rename(id, name) => {
                        self.history.rename(id, name);
                    }
                }
            }
        }

        // Advance the in-flight restart/arm/watch cycle.
        self.step_cycle(ctx);

        // Apply shortcut actions to shared state (same dispatch as the buttons).
        for cmd in shortcut_actions {
            self.apply_transport_action(cmd);
        }

        // Apply any input edit the timeline produced last frame.
        self.poll_script_file();
        self.apply_pending_input_edit();

        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Read before the mutable borrow of `shared` below.
            let arming_allowed = self.arming_allowed();
            // Transport bar at top
            let cmds = if let Some(ref mut shared) = self.shared {
                let mode = shared.state().mode_enum();
                let recorded = shared.state().recorded_count;
                // During catch-up the resume speed lives in resume_speed
                // (playback_speed is the catch-up multiplier); otherwise it's
                // the live play speed. The speed buttons highlight and edit it.
                let resume_speed = self.resume_speed.unwrap_or(self.playback_speed);

                transport::show(
                    ui,
                    transport::TransportProps {
                        mode,
                        recorded,
                        continue_from: &mut self.continue_from_frame,
                        continue_from_text: &mut self.continue_from_text,
                        playback_speed: &mut self.playback_speed,
                        cont_catchup_speed: self.cont_catchup_multiplier,
                        history: &self.history,
                        state: shared.state(),
                        catchup_active: self.resume_speed.is_some(),
                        resume_speed,
                        arming_allowed,
                    },
                )
            } else {
                Vec::new()
            };

            // Same dispatch as keyboard shortcuts, so the two can't diverge.
            for cmd in cmds {
                self.apply_transport_action(cmd);
            }

            if let Some(ref mut shared) = self.shared {
                // Re-assert playback_speed every frame. During a CONT catch-up
                // it is the catch-up multiplier, and continuous re-assertion
                // overrides the brief speed reset an F5 restart causes. Once
                // the splice has fired (mode == REC) the DLL has already
                // dropped to the resume speed, so assert that instead of
                // pushing the multiplier back before the mode handler runs.
                let catchup_active = self.resume_speed.is_some();
                let mode = shared.state().mode;
                let speed_to_assert = if catchup_active && mode == TasMode::Rec as u32 {
                    self.resume_speed.unwrap_or(self.playback_speed)
                } else {
                    self.playback_speed
                };
                shared.state_mut().playback_speed = speed_to_assert;

                ui.separator();

                let state = shared.state();
                let cycle_ticking =
                    self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400);
                status::status_card(
                    ui,
                    state,
                    &status::StatusProps {
                        in_game: state.game_in_game != 0 && cycle_ticking,
                        finished_at_tick: self.finished_at_tick,
                        loaded_physics: self.loaded_physics.as_deref(),
                        loaded_rider: self.loaded_rider.as_deref(),
                    },
                );

                ui.separator();

                status::drift_banner(ui, state, &self.drift_tracker);

                let open_text_script =
                    status::timeline_header(ui, state, self.script_watch.is_some());
                // Keep the game-clock origin after the course unloads. The
                // selected history entry describes the recording itself and
                // is therefore a better fallback than the current menu state.
                let timeline_level = self
                    .history
                    .current_index()
                    .and_then(|i| self.history.entries().get(i))
                    .and_then(|entry| entry.level.as_deref())
                    .or(self.last_resolved_level.as_deref());
                let tl_outcome = timeline::show(
                    ui,
                    state,
                    timeline_level,
                    &mut self.timeline_view,
                    &mut self.continue_from_frame,
                    &mut self.timeline_edit,
                );
                if tl_outcome.continue_changed {
                    self.continue_from_text = self.continue_from_frame.to_string();
                }
                if let Some(events) = tl_outcome.events {
                    self.pending_input_edit = Some((
                        events,
                        tl_outcome.commit_undo,
                        tl_outcome.action_label.unwrap_or_default(),
                    ));
                }

                // Text-script route: poll_script_file reloads it on save.
                if open_text_script {
                    let total = state.recorded_count;
                    let events = input_script::runs_from_log(&state.input_log, total);
                    let timer = tas_shared::align::detect_first_moving(&state.rec_coords, total)
                        .unwrap_or(0);
                    let script = input_script::events_to_script(&events, timer);
                    let path = script_watch::ScriptWatch::fresh_path();
                    match std::fs::write(&path, &script) {
                        Ok(()) => {
                            #[cfg(windows)]
                            {
                                let _ = std::process::Command::new("cmd")
                                    .arg("/C")
                                    .arg("start")
                                    .arg("")
                                    .arg(&path)
                                    .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
                                    .spawn();
                            }
                            self.script_watch =
                                Some(script_watch::ScriptWatch::new(path.clone(), script));
                            self.log_lines.push(format!(
                                "[script] opened {} ({} inputs); edits reload on save",
                                path.display(),
                                events.len()
                            ));
                        }
                        Err(e) => self.log_lines.push(format!("[script] write failed: {e}")),
                    }
                }

                // DLL counters, shown only with Debug Config.
                if self.show_config {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Recorded: {} | Playback: {} | BB3B10: {} | Blocks: {}",
                            state.recorded_count,
                            state.playback_pos,
                            state.bb3b10_call_count,
                            state.handler_block_count
                        ));
                    });
                }

                let previous = (
                    self.drift_tracker.max_dx,
                    self.drift_tracker.max_dz,
                    self.drift_tracker.splice_tick,
                );
                let (count, reset) = self.drift_tracker.scan(state);
                if reset {
                    self.last_logged_drift_level = 0;
                }

                let max_drift = self.drift_tracker.max_drift();
                let new_level = if max_drift >= 5.0 {
                    3
                } else if max_drift >= 1.0 {
                    2
                } else if max_drift > 0.0 {
                    1
                } else {
                    0
                };
                if state.mode == TasMode::Play as u32 && new_level > self.last_logged_drift_level {
                    self.log_lines.push(format!(
                        "{} first at tick {}: max X={:.9} Z={:.9} (was X={:.9} Z={:.9})",
                        if state.continue_from_frame != 0 {
                            "CONT prefix difference"
                        } else {
                            "DRIFT"
                        },
                        self.drift_tracker.first_drift_tick.unwrap_or(count),
                        self.drift_tracker.max_dx,
                        self.drift_tracker.max_dz,
                        if reset { 0.0 } else { previous.0 },
                        if reset { 0.0 } else { previous.1 },
                    ));
                    self.last_logged_drift_level = new_level;
                }
                if (state.mode == TasMode::Play as u32 || state.mode == TasMode::Rec as u32)
                    && (reset || previous.2.is_none())
                {
                    if let Some(tick) = self.drift_tracker.splice_tick {
                        self.log_lines.push(format!(
                            "CONT splice {tick}: X={:.9} Z={:.9}",
                            self.drift_tracker.splice_dx, self.drift_tracker.splice_dz
                        ));
                    }
                }

                // Do not sync continue_from_frame to shared memory here. It
                // stages the next CONT; the controller writes it when a cycle
                // starts and on each reroll, and syncing it here would move a
                // running CONT's splice.
            }
        });

        self.drain_dll_log();

        for w in self.history.take_warnings() {
            self.log_lines.push(format!("History: {}", w));
        }
        self.persist_history_if_needed();

        // Refresh only as fast as there is something to show. tas_ui's own
        // rendering at a flat 30 fps costs the game ~10 ms per menu frame
        // (48 ms vs 58 ms) through GPU contention. Full rate while a TAS mode
        // is armed or the engine is ticking, a lazy rate at a menu. egui still
        // repaints immediately on input; this only sets the idle floor.
        let mode_active = self
            .shared
            .as_ref()
            .map(|s| s.mode_volatile() != TasMode::Off as u32)
            .unwrap_or(false);
        let cycle_ticking = self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400);
        // eframe still runs update and paint while minimized, and each painted
        // frame leaks a little renderer memory on DX12/Vulkan (~15 KB/s at
        // 60 fps), so idle hard when minimized regardless of mode.
        let minimized = ctx.input(|i| i.viewport().minimized.unwrap_or(false));
        let refresh_ms = if minimized {
            500
        } else if mode_active || cycle_ticking {
            33
        } else {
            250
        };
        ctx.request_repaint_after(std::time::Duration::from_millis(refresh_ms));
    }
}

fn open_in_file_browser(path: &std::path::Path) -> Result<(), String> {
    std::process::Command::new("explorer")
        .arg(path)
        .spawn()
        .map_err(|e| format!("failed to launch explorer: {}", e))?;
    Ok(())
}

fn main() -> eframe::Result {
    // Reject unsafe automation setup before opening any user history/settings.
    if std::env::var_os("SSB_INSPECT_E2E_RECORDING").is_some()
        && std::env::var_os("SSB_INSPECT_DATA_DIR").is_none()
    {
        eprintln!("UI E2E setup requires an isolated SSB_INSPECT_DATA_DIR");
        std::process::exit(1);
    }
    // Single-instance guard via a named Windows mutex.
    let instance = single_instance::SingleInstance::new("SSBInspect").unwrap();
    if !instance.is_single() {
        eprintln!("SSB Inspect is already running.");
        win32::focus_window_titled("SSB Inspect");
        std::process::exit(0);
    }

    // 1100 px wide so the transport row, with `from:` visible beside the
    // 280 px history rail, does not clip Redo or the speed presets.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SSB Inspect",
        options,
        Box::new(|cc| {
            // Force dark theme regardless of OS setting
            cc.egui_ctx.set_theme(egui::Theme::Dark);

            let mut visuals = egui::Visuals::dark();
            let dark_bg = egui::Color32::from_gray(24);
            let widget_bg = egui::Color32::from_gray(35);
            let widget_hover = egui::Color32::from_gray(45);
            let widget_active = egui::Color32::from_gray(40);
            let subtle_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_gray(50));

            // Panel and window backgrounds
            visuals.panel_fill = dark_bg;
            visuals.window_fill = dark_bg;
            visuals.extreme_bg_color = egui::Color32::from_gray(10);
            visuals.faint_bg_color = egui::Color32::from_gray(30);
            visuals.code_bg_color = egui::Color32::from_gray(30);

            // Non-interactive widgets (labels, separators)
            visuals.widgets.noninteractive.bg_fill = dark_bg;
            visuals.widgets.noninteractive.weak_bg_fill = dark_bg;
            visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(40));

            // Inactive widgets (buttons, combo boxes, collapsing headers)
            visuals.widgets.inactive.bg_fill = widget_bg;
            visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.inactive.bg_stroke = subtle_stroke;

            // Hovered widgets
            visuals.widgets.hovered.bg_fill = widget_hover;
            visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(38);
            visuals.widgets.hovered.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(70));

            // Active (pressed) widgets
            visuals.widgets.active.bg_fill = widget_active;
            visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(35);
            visuals.widgets.active.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(80));

            // Open widgets (dropdowns)
            visuals.widgets.open.bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.open.weak_bg_fill = egui::Color32::from_gray(28);

            // Window decoration
            visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_gray(40));

            cc.egui_ctx.set_visuals_of(egui::Theme::Dark, visuals);
            Ok(Box::new(TasApp::new()))
        }),
    )
}

#[cfg(test)]
mod tests {
    mod cont_splice;
    mod relaunch;

    #[test]
    fn load_restore_save_and_edit_keep_take_identity() {
        use tas_shared::{TAS_CHARACTER_KEITH, TAS_CHARACTER_VINCENT};
        use tas_shared::{TAS_RENDERER_DIRECTX7, TAS_RENDERER_OPENGL};

        let mut app = test_app();
        // Live game runs Vincent/DirectX7.
        app.history.set_live_stamps(recording::IdentityStamps {
            renderer_id: Some(TAS_RENDERER_DIRECTX7),
            fpu_control_word: Some(0x007F),
            rider_character: Some(TAS_CHARACTER_VINCENT),
            rider_stance: Some(0),
        });
        // A Keith/OpenGL file is loaded: the buffer and its history entry
        // carry the file's stamps, not the live game's.
        let mut file_state = tas_shared::zeroed_boxed();
        file_state.recorded_count = 4;
        let keith = recording::IdentityStamps {
            renderer_id: Some(TAS_RENDERER_OPENGL),
            fpu_control_word: Some(0x027F),
            rider_character: Some(TAS_CHARACTER_KEITH),
            rider_stance: Some(0),
        };
        assert!(app.history.push_loaded_snapshot(
            &file_state,
            std::path::Path::new("keith.tasrec"),
            Some(keith.clone()),
        ));
        assert_eq!(app.history.entries().last().unwrap().stamps, keith);
        // Restore the entry as the UI does, then save a copy: the file must
        // keep saying Keith/OpenGL even though the game runs Vincent/DX7.
        app.note_restored_physics();
        assert_eq!(app.loaded_identity, Some(keith.clone()));
        let path =
            std::env::temp_dir().join(format!("identity_chain_{}.tasrec", std::process::id()));
        recording::RecordingFile::save_with_segments(
            &file_state,
            &path,
            &[],
            app.loaded_identity.as_ref(),
        )
        .unwrap();
        let meta = recording::RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        // An edit derived from the loaded take keeps its identity too.
        assert!(app.history.push_snapshot_data_with_session(
            recording::RecordingSnapshot::from_state(&file_state),
            "Edited inputs",
            0,
            0,
            app.loaded_identity.clone(),
        ));
        assert_eq!(app.history.entries().last().unwrap().stamps, keith);
        let _ = std::fs::remove_file(&path);
    }
    #[test]
    fn checkpoint_survives_missing_or_failed_history() {
        // Durable flush authorizes deletion with no notice.
        assert_eq!(checkpoint_clear_decision(Some(Ok(()))), (true, None));
        // A failed flush keeps the checkpoint and says so.
        let (durable, notice) = checkpoint_clear_decision(Some(Err("disk full".into())));
        assert!(!durable);
        assert!(notice.unwrap().contains("keeping recovery checkpoint"));
        // A missing writer is NOT durability: the checkpoint files are the
        // only copy, so they stay, with an actionable notice.
        let (durable, notice) = checkpoint_clear_decision(None);
        assert!(!durable);
        assert!(notice.unwrap().contains("store unavailable"));
    }

    #[test]
    fn replacing_a_recording_detaches_its_script_and_queued_stop_edit() {
        let mut app = test_app();
        let old_path = crate::script_watch::ScriptWatch::fresh_path();
        std::fs::write(&old_path, "1-5 press left").unwrap();
        app.script_watch = Some(crate::script_watch::ScriptWatch::new(
            old_path.clone(),
            "1-5 press left".into(),
        ));
        app.pending_input_edit = Some((Vec::new(), true, "old script".into()));
        app.pending_edit_autostop = true;
        assert!(app.stop_active_session_for_load());
        std::fs::write(&old_path, "2-9 press shift").unwrap();
        app.poll_script_file();
        assert!(app.script_watch.is_none());
        assert!(app.pending_input_edit.is_none());
        assert!(!app.pending_edit_autostop);
        std::fs::remove_file(old_path).unwrap();
    }

    #[test]
    fn fresh_recording_detaches_editor_but_continue_preserves_it() {
        let mut app = test_app();
        let path = crate::script_watch::ScriptWatch::fresh_path();
        assert_ne!(path, crate::script_watch::ScriptWatch::fresh_path());
        app.script_watch = Some(crate::script_watch::ScriptWatch::new(path, String::new()));
        app.start_recording_session(10, 20);
        assert!(app.script_watch.is_some());
        app.start_recording_session(0, 20);
        assert!(app.script_watch.is_none());
    }

    use super::*;
    use egui::{Event, Key, Modifiers, RawInput};
    use tas_shared::TasSharedState;

    #[test]
    fn recording_buffer_overwrite_requires_stop_acknowledgement() {
        assert!(!stop_is_acknowledged(TasMode::Play as u32, false));
        assert!(!stop_is_acknowledged(TasMode::Play as u32, true));
        assert!(!stop_is_acknowledged(TasMode::Off as u32, false));
        assert!(stop_is_acknowledged(TasMode::Off as u32, true));
    }

    #[test]
    fn pending_frozen_stop_is_not_reissued() {
        // Idle slot, or a stale non-STOP command: request the stop.
        assert!(should_request_auto_stop(true, true, false, false));
        // STOP already published/claimed: leave it alone.
        assert!(!should_request_auto_stop(true, true, false, true));
        assert!(!should_request_auto_stop(false, true, false, false));
        assert!(!should_request_auto_stop(true, false, false, false));
        assert!(!should_request_auto_stop(true, true, true, false));
    }

    /// Test constructor: creates TasApp without shared memory or Pico.
    fn test_app() -> TasApp {
        TasApp {
            shared: None,
            connect_error: Some("Test mode: no DLL".into()),
            show_config: false,
            show_pico_panel: false,
            pico: PicoState::new(),
            history: RecordingHistory::new(64),
            history_writer: None,
            recovery_writer: recording::RecoveryWriter::new(),
            last_persisted_revision: 0,
            last_queued_revision: 0,
            last_failed_revision: 0,
            history_retry_after: None,
            last_reconnect_attempt: std::time::Instant::now(),
            loaded_physics: None,
            loaded_rider: None,
            loaded_identity: None,
            history_cap: 64,
            recovery_store: None,
            log_lines: ui_log::UiLog::default(),
            finish_scan_cursor: 0,
            finished_at_tick: None,
            finished_hud_cs: None,
            timeline_view: timeline::TimelineView::default(),
            timeline_edit: timeline::TimelineEdit::default(),
            pending_input_edit: None,
            pending_edit_autostop: false,
            script_watch: None,
            continue_from_frame: 0,
            continue_from_text: "0".to_string(),
            playback_speed: 1.0,
            show_history: false,
            show_log: false,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            resume_speed: None,
            cont_catchup_multiplier: 12.0,
            cycle_arm: tas_shared::transport::Arm::Continue,
            log_read_cursor: 0,
            drift_tracker: drift_scan::DriftTracker::default(),
            last_logged_drift_level: 0,
            cycle: None,
            cycle_deadline: None,
            cont_last_outcome: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
            game_pid_seen: None,
            expect_ring_restart: None,
            stale_protection_since: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            cycle_fc: 0,
            cycle_fc_seeded: false,
            // Ancient, not now(): "ticking" must be FALSE until a real
            // frame_count advance is observed.
            cycle_advance_at: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(600))
                .unwrap_or_else(std::time::Instant::now),
            auto_stop_debounce: None,
            last_resolved_level: None,
            last_resolved_epoch: None,
            #[cfg(windows)]
            dark_title_bar_set: false,
        }
    }

    /// Run handle_shortcuts with a simulated key press and return actions.
    fn press_key(app: &mut TasApp, key: Key, modifiers: Modifiers) -> Vec<transport::Action> {
        let ctx = egui::Context::default();
        let input = RawInput {
            events: vec![Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..Default::default()
        };

        let mut actions = Vec::new();
        let _ = ctx.run(input, |_ctx| {
            actions = app.handle_shortcuts(_ctx);
        });
        actions
    }

    fn action_has_command(actions: &[transport::Action], cmd: TasCommand) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::Send(c) if *c == cmd))
    }

    fn action_has_restart_then(actions: &[transport::Action], cmd: TasCommand) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::RestartThen(c) if *c == cmd))
    }

    fn action_has_log(actions: &[transport::Action], needle: &str) -> bool {
        actions
            .iter()
            .any(|a| matches!(a, transport::Action::Log(s) if s.contains(needle)))
    }

    // ===== App startup without DLL =====

    #[test]
    fn app_starts_without_dll() {
        let app = test_app();
        assert!(app.shared.is_none());
        assert!(app.connect_error.is_some());
        assert_eq!(app.playback_speed, 1.0);
        assert_eq!(app.timeline_view, timeline::TimelineView::default());
        assert!(!app.pico.connected);
    }

    // ===== Keyboard shortcuts =====

    /// An app connected to an idle DLL in a ticking level holding `recorded`
    /// ticks: the state in which the REC / PLAY / CONT buttons are enabled.
    fn idle_in_level_app(recorded: u32) -> TasApp {
        let mut app = test_app();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        shared.state_mut().game_in_game = 1;
        shared.state_mut().recorded_count = recorded;
        app.shared = Some(shared);
        app.cycle_advance_at = std::time::Instant::now();
        app
    }

    #[test]
    fn shortcut_f9_arms_rec() {
        let mut app = idle_in_level_app(0);
        let actions = press_key(&mut app, Key::F9, Modifiers::NONE);
        assert!(action_has_restart_then(&actions, TasCommand::ArmRec));
        assert!(action_has_log(&actions, "F9"));
    }

    #[test]
    fn shortcut_f10_arms_play() {
        let mut app = idle_in_level_app(100);
        let actions = press_key(&mut app, Key::F10, Modifiers::NONE);
        assert!(action_has_restart_then(&actions, TasCommand::ArmPlay));
        assert!(action_has_log(&actions, "F10"));
    }

    #[test]
    fn shortcuts_are_inert_without_a_game() {
        let mut app = test_app();
        for key in [Key::F9, Key::F10, Key::F12] {
            let actions = press_key(&mut app, key, Modifiers::NONE);
            assert!(!action_has_restart_then(&actions, TasCommand::ArmRec));
            assert!(!action_has_restart_then(&actions, TasCommand::ArmPlay));
            assert!(!action_has_restart_then(&actions, TasCommand::ArmContinue));
            assert!(action_has_log(&actions, "not connected"));
        }
    }

    #[test]
    fn shortcut_f11_stops() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F11, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::Stop));
        assert!(action_has_log(&actions, "F11"));
    }

    #[test]
    fn shortcut_f12_arms_continue() {
        let mut app = idle_in_level_app(100);
        app.continue_from_text = "50".into();
        let actions = press_key(&mut app, Key::F12, Modifiers::NONE);
        assert!(matches!(
            actions.first(),
            Some(transport::Action::SetContinueFrame(50))
        ));
        assert!(action_has_restart_then(&actions, TasCommand::ArmContinue));
        assert!(action_has_log(&actions, "F12"));
    }

    #[test]
    fn shortcut_space_stops() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Space, Modifiers::NONE);
        assert!(action_has_command(&actions, TasCommand::Stop));
        assert!(action_has_log(&actions, "Space"));
    }

    #[test]
    fn shortcut_f5_without_pico_logs_not_connected() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F5, Modifiers::NONE);
        assert!(action_has_log(&actions, "not connected"));
    }

    #[test]
    fn shortcut_ctrl_z_undoes() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Z, Modifiers::CTRL);
        // Must produce Undo action — not just "no panic"
        assert!(
            actions.iter().any(|a| matches!(a, transport::Action::Undo)),
            "Ctrl+Z must produce Undo action, got: {:?}",
            actions.len()
        );
        assert!(action_has_log(&actions, "Undo"), "Ctrl+Z must log Undo");
    }

    #[test]
    fn shortcut_ctrl_y_redoes() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Y, Modifiers::CTRL);
        assert!(
            actions.iter().any(|a| matches!(a, transport::Action::Redo)),
            "Ctrl+Y must produce Redo action"
        );
        assert!(action_has_log(&actions, "Redo"), "Ctrl+Y must log Redo");
    }

    #[test]
    fn shortcut_shift_z_only_redoes() {
        for modifiers in [Modifiers::CTRL, Modifiers::MAC_CMD] {
            let mut app = test_app();
            let actions = press_key(
                &mut app,
                Key::Z,
                Modifiers {
                    shift: true,
                    ..modifiers
                },
            );
            assert_eq!(
                actions
                    .iter()
                    .filter(|a| matches!(a, transport::Action::Redo))
                    .count(),
                1
            );
            assert!(!actions.iter().any(|a| matches!(a, transport::Action::Undo)));
        }
    }

    #[test]
    fn restoring_while_idle_cancels_pending_arm() {
        use tas_shared::transport::{Arm, ArmConfig, TransportController};
        for arm in [Arm::Rec, Arm::Play, Arm::Continue] {
            let mut app = test_app();
            app.cycle = Some(TransportController::new(ArmConfig {
                arm,
                speed: 256.0,
                continue_from_frame: 400,
                gate_align_rec: 299,
                max_retries: 1,
            }));
            app.resume_speed = Some(1.0);
            app.playback_speed = 256.0;
            app.pending_session_kind = Some(RecordingSessionKind::Continue);
            app.pending_continue_start_tick = Some(400);
            assert!(app.stop_active_session_for_load());
            assert!(app.cycle.is_none());
            assert!(app.pending_session_kind.is_none());
            assert!(app.pending_continue_start_tick.is_none());
            assert!(app.resume_speed.is_none());
            assert_eq!(app.playback_speed, 1.0);
        }
    }

    // ===== Timeline zoom (keyboard +/-) =====

    #[test]
    fn zoom_in_shrinks_window() {
        let mut app = test_app();
        app.timeline_view = timeline::TimelineView {
            start: 100,
            end: 1100,
        };
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_view.end - app.timeline_view.start < 1000);
    }

    #[test]
    fn zoom_out_grows_window() {
        let mut app = test_app();
        app.timeline_view = timeline::TimelineView {
            start: 100,
            end: 1100,
        };
        press_key(&mut app, Key::Minus, Modifiers::NONE);
        assert!(app.timeline_view.end - app.timeline_view.start > 1000);
    }

    #[test]
    fn zoom_in_clamps_to_min_window() {
        let mut app = test_app();
        // 60-tick window is already the minimum; zooming in must not go below.
        app.timeline_view = timeline::TimelineView {
            start: 500,
            end: 560,
        };
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_view.end - app.timeline_view.start >= 60);
    }

    // ===== Speed edge values =====

    #[test]
    fn playback_speed_defaults_to_1x() {
        let app = test_app();
        assert!((app.playback_speed - 1.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_rejects_catchup_values() {
        assert!((normalize_playback_speed(12.0) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(f32::NAN) - 1.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_keeps_valid_presets() {
        assert!((normalize_playback_speed(0.25) - 0.25).abs() < 0.001);
        assert!((normalize_playback_speed(0.5) - 0.5).abs() < 0.001);
        assert!((normalize_playback_speed(2.0) - 2.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_resets_dropped_presets() {
        // 0.01x, 0.05x and 4x are not preset buttons — any persisted setting
        // at those values should fall back to 1x. (0.25x is a valid preset,
        // covered by playback_speed_normalizer_keeps_valid_presets.)
        assert!((normalize_playback_speed(0.01) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(0.05) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(4.0) - 1.0).abs() < 0.001);
    }

    /// The edge detector fires once per false→true transition, so holding
    /// F9 in-game arms REC once rather than every frame.
    #[test]
    fn global_shortcut_edges_fire_only_on_press() {
        let mut prev = [false; 4];
        // First call: F9 pressed → edge for slot 0 only.
        let edges = compute_global_key_edges([true, false, false, false], &mut prev);
        assert_eq!(edges, [true, false, false, false]);
        assert_eq!(prev, [true, false, false, false]);
        // Hold: F9 still pressed → no edge.
        let edges = compute_global_key_edges([true, false, false, false], &mut prev);
        assert_eq!(edges, [false, false, false, false]);
        // Release: F9 released → no edge (we fire on press, not release).
        let edges = compute_global_key_edges([false, false, false, false], &mut prev);
        assert_eq!(edges, [false, false, false, false]);
        assert_eq!(prev, [false, false, false, false]);
        // Re-press: F9 pressed again → edge fires.
        let edges = compute_global_key_edges([true, false, false, false], &mut prev);
        assert_eq!(edges, [true, false, false, false]);
    }

    /// A key held across many frames (repeated identical reads) produces no
    /// further edges.
    #[test]
    fn global_shortcut_held_does_not_repeat() {
        let mut prev = [false; 4];
        compute_global_key_edges([true, true, true, true], &mut prev);
        for _ in 0..100 {
            let edges = compute_global_key_edges([true, true, true, true], &mut prev);
            assert_eq!(edges, [false, false, false, false]);
        }
    }

    /// CONT with no recording loaded must not leave the app half-armed at
    /// catch-up speed: with nothing to splice, the PLAY→REC transition that
    /// calls clear_cont_catchup would never fire.
    #[test]
    fn cont_with_no_recording_does_not_arm_catchup() {
        let mut app = test_app();
        app.playback_speed = 1.0;
        app.cont_catchup_multiplier = 32.0;
        // No shared memory, so recorded_count reads as 0.
        app.queue_restart_then(TasCommand::ArmContinue);
        assert!(
            app.resume_speed.is_none(),
            "CONT without recording must not engage catchup speed"
        );
        assert!(
            (app.playback_speed - 1.0).abs() < 0.001,
            "playback_speed must remain at pre-CONT value, got {}",
            app.playback_speed
        );
        assert!(
            app.cycle.is_none(),
            "CONT without recording must not arm a controller"
        );
    }

    fn state_with_recorded_count(recorded_count: u32) -> Box<TasSharedState> {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = recorded_count;
        for i in 0..(recorded_count as usize).min(tas_shared::TAS_MAX_TICKS) {
            state.input_log[i] = 0x01;
            state.rec_coords[i] = [i as f32, 0.0, i as f32];
        }
        state
    }

    fn only_log(actions: &[transport::Action]) -> String {
        match actions {
            [transport::Action::Log(line)] => line.clone(),
            other => panic!("expected a single log line, got {}", other.len()),
        }
    }

    #[test]
    fn shortcuts_follow_the_button_rule() {
        let mut app = test_app();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        shared.state_mut().game_in_game = 1;
        shared.state_mut().mode = TasMode::Rec as u32;
        shared.state_mut().recorded_count = 100;
        app.shared = Some(shared);
        app.cycle_advance_at = std::time::Instant::now(); // the level is ticking

        // F9 during a REC: the button is grey, so the key refuses too.
        let line = only_log(&app.shortcut_arm("Shortcut: F9 REC", TasCommand::ArmRec));
        assert!(
            line.contains("ignored") && line.contains("STOP first"),
            "{line}"
        );

        app.shared.as_mut().unwrap().state_mut().mode = TasMode::Off as u32;
        app.shared.as_mut().unwrap().state_mut().recorded_count = 0;
        let line = only_log(&app.shortcut_arm("Shortcut: F10 PLAY", TasCommand::ArmPlay));
        assert!(line.contains("nothing is recorded"), "{line}");

        // F12 with a typo in From: refused, the typo stays for the user to see.
        app.shared.as_mut().unwrap().state_mut().recorded_count = 100;
        app.continue_from_text = "abc".into();
        app.continue_from_frame = 40;
        let line = only_log(&app.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue));
        assert!(line.contains("\"abc\""), "{line}");
        assert_eq!(
            (app.continue_from_text.as_str(), app.continue_from_frame),
            ("abc", 40)
        );

        // F12 with a frame past the end: clamped, displayed, and armed.
        app.continue_from_text = "500".into();
        let actions = app.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue);
        assert!(matches!(
            actions.as_slice(),
            [
                transport::Action::SetContinueFrame(100),
                transport::Action::RestartThen(TasCommand::ArmContinue),
                transport::Action::Log(_)
            ]
        ));
        assert_eq!(
            (app.continue_from_text.as_str(), app.continue_from_frame),
            ("100", 100)
        );

        // At a menu nothing arms.
        app.cycle_advance_at = std::time::Instant::now() - std::time::Duration::from_secs(5);
        let line = only_log(&app.shortcut_arm("Global F9 (in-game): REC", TasCommand::ArmRec));
        assert!(line.contains("enter a level first"), "{line}");
    }

    #[test]
    fn completed_rec_session_pushes_history_entry() {
        let mut app = test_app();
        let state = state_with_recorded_count(2303);
        app.active_recording_session = Some(ActiveRecordingSession {
            kind: RecordingSessionKind::Rec,
            start_tick: 0,
            max_recorded_count: 2303,
        });

        let snapshot = recording::RecordingSnapshot::from_state(&state);
        app.finalize_recording_session(&snapshot, 2303);

        assert_eq!(app.history.len(), 1);
        assert_eq!(app.history.entries()[0].label, "Recorded 0:23.03");
    }

    #[test]
    fn completed_cont_session_pushes_history_entry() {
        let mut app = test_app();
        let state = state_with_recorded_count(5303);
        app.active_recording_session = Some(ActiveRecordingSession {
            kind: RecordingSessionKind::Continue,
            start_tick: 3000,
            max_recorded_count: 5303,
        });

        let snapshot = recording::RecordingSnapshot::from_state(&state);
        app.finalize_recording_session(&snapshot, 5303);

        assert_eq!(app.history.len(), 1);
        assert_eq!(
            app.history.entries()[0].label,
            "Continued from 0:30.00, total 0:53.03"
        );
    }

    #[test]
    fn continue_session_uses_pending_continue_start_when_shared_resets_to_zero() {
        let mut app = test_app();
        app.pending_session_kind = Some(RecordingSessionKind::Continue);
        app.pending_continue_start_tick = Some(3415);
        app.start_recording_session(0, 3415);

        let session = app.active_recording_session.expect("session should start");
        assert_eq!(session.kind, RecordingSessionKind::Continue);
        assert_eq!(session.start_tick, 3415);
        assert_eq!(session.max_recorded_count, 3415);
    }

    #[test]
    fn play_stop_noop_does_not_push_history_entry() {
        let mut app = test_app();
        let state = state_with_recorded_count(100);

        let snapshot = recording::RecordingSnapshot::from_state(&state);
        app.finalize_recording_session(&snapshot, 100);
        assert_eq!(app.history.len(), 0);

        app.active_recording_session = Some(ActiveRecordingSession {
            kind: RecordingSessionKind::Rec,
            start_tick: 100,
            max_recorded_count: 100,
        });
        app.finalize_recording_session(&snapshot, 100);
        assert_eq!(app.history.len(), 0);
    }

    // ===== Transport controller state =====

    #[test]
    fn cont_controller_initially_none() {
        let app = test_app();
        assert!(app.cycle.is_none());
    }
}
