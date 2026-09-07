mod drift_scan;
mod history_store_v2;
mod level;
mod panels;
mod pico;
mod recording;
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

use panels::{
    config, drift, history, input_script, log_panel, status, timeline, trajectory, transport,
};
use pico::PicoState;
use recording::{RecordingHistory, RecordingSessionKind};

const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
const PLAYBACK_SPEED_PRESETS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];
// Shared with the cont-reliability harness via tas_shared::cont — the bucket
// headroom now lives inside judge_cont_bucket (one source of truth).
const CONT_START_MATCH_MAX_RETRIES: u32 = tas_shared::cont::START_MATCH_MAX_RETRIES;

/// How long one judged cycle may run before the UI gives up on it.
///
/// Generous on purpose — this is a stall guard, not a performance bound. A
/// deep CONT catch-up plus its rerolls, or a PLAY judged all the way to
/// first_moving + BUCKET_VALIDATE_WINDOW at 1x, legitimately take tens of
/// seconds. What it must not do is let a cycle that will never finish hold
/// the input block forever.
const CONT_CYCLE_BUDGET: std::time::Duration = std::time::Duration::from_secs(180);

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
    history_writer: Option<history_store_v2::HistoryWriter>,
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
    /// Serialized off-thread writer for recovery checkpoints. Replaces detached
    /// `thread::spawn` per write so a late write can't land after `clear_pending`
    /// and resurrect a stale checkpoint into a duplicate "Recovered" entry.
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
    show_debug_drift: bool,
    show_history: bool,
    show_log: bool,
    show_trajectory: bool,
    segment_tracker: recording::SegmentTracker,
    active_recording_session: Option<ActiveRecordingSession>,
    pending_session_kind: Option<RecordingSessionKind>,
    pending_continue_start_tick: Option<u32>,
    last_mode: u32,
    cont_catchup_speed: Option<f32>, // saved speed to restore after CONT catch-up
    cont_catchup_multiplier: f32,    // configurable CONT catch-up speed (default 12x)
    /// Which transport the in-flight controller cycle is for, for log lines.
    /// The reroll/abort messages used to hardcode "CONT" because only CONT was
    /// ever judged; PLAY can be judged now, and a PLAY that exhausts its
    /// rerolls reporting "CONT aborted" would send someone hunting the wrong bug.
    cont_cycle_arm: tas_shared::transport::Arm,
    log_read_cursor: u32,
    /// Finish-line watcher (1.7): scan cursor into rec_coords during REC so
    /// each frame only examines new ticks, and the tick the run crossed the
    /// finish (drives the auto-stop + the 🏁 marker; reset when REC starts).
    finish_scan_cursor: u32,
    finished_at_tick: Option<u32>,
    /// The HUD race time LATCHED at the moment the crossing was detected (v43
    /// fix): the label used to re-read the live timer at STOP, and a timer
    /// that blanked or moved in between turned an exact time into a wrong or
    /// approximate one.
    finished_hud_cs: Option<u32>,

    drift_tracker: drift_scan::DriftTracker,
    last_logged_drift_level: u8, // 0=none, 1=any, 2=>=1.0, 3=>=5.0

    // Cached plot data (avoid per-frame Vec allocation)
    drift_cache: drift::DriftCache,
    trajectory_cache: trajectory::TrajectoryCache,

    /// In-flight restart→arm(→judge→reroll) cycle, driven by the shared
    /// `tas_shared::transport` controller — the SAME state machine the tas_test
    /// harness runs, so the test is a true oracle. Stepped once per egui frame
    /// via `step_cont_controller`; `None` when idle. Encapsulates the old
    /// two-step Stop→wait-OFF→Restart→wait-rs2 serialisation (the shared
    /// `command` slot is a single u32, so Stop and Restart can't be written on
    /// the same frame) plus the CONT bucket judge/reroll loop.
    cont_controller: Option<tas_shared::transport::TransportController>,
    /// Wall-clock deadline for the in-flight cycle to make a terminal
    /// transition. `None` when idle.
    ///
    /// The controller has no clock and never blocks, so a phase that stops
    /// progressing — an F5 restart that never completes, an arm the DLL never
    /// processes — returns InProgress forever. The tas_test harness has always
    /// had its own budget for this; the UI had none, so the same stall left the
    /// cycle spinning with cont_suppress_input SET and the user's keyboard
    /// swallowed until they found the STOP button.
    cont_cycle_deadline: Option<std::time::Instant>,
    /// Carries the last CONT cycle's result (bucket attempts, how the bucket
    /// was accepted) from controller-`Done` to the REC-start transition, where
    /// the actual resume frame is known — so we can log one "resumed at frame X
    /// after N bucket attempt(s) — <verdict>" summary.
    cont_last_outcome: Option<(u32, tas_shared::transport::CompletedVia)>,
    /// Previous-frame pressed state for the four global-shortcut keys
    /// (F9, F10, F11, F12 in that order). Diffed against the current
    /// GetAsyncKeyState result to detect press edges. Updated every
    /// frame regardless of which window has focus so we never get
    /// stuck on stale "was pressed" state after a focus change.
    prev_global_keys: [bool; 4],
    /// Cached Supreme.exe PID. Resolved on first poll, invalidated when
    /// the shared-memory connection drops (game closed/restarted).
    /// Stored as Option so a re-resolution attempt is just `.take()`
    /// followed by re-call.
    game_pid_cached: Option<u32>,
    // Crash recovery
    last_frame_count: u32,
    stale_frame_ticks: u32,
    last_health_check: std::time::Instant,
    // Cycle-activity tracker for the In-Game chip: game_in_game (exe+0x8895C)
    // is written by the Supreme::Cycle hook, so when the cycle STOPS (quit to
    // menu / pause / dialog) it freezes at its last value (1) instead of going
    // to 0 — the "stale In Game (Forest Easy)" chip. frame_count only advances
    // while the cycle runs, so a fresh advance means the game is actually
    // ticking a level; a stale one means menu/paused. Sampled every frame.
    cycle_fc: u32,
    // True once cycle_fc holds a real baseline sample. Without it the first
    // sample after launch/reconnect counted as an "advance" and opened the
    // transport gate ~400ms against a frozen engine.
    cycle_fc_seeded: bool,
    cycle_advance_at: std::time::Instant,
    // The menu auto-stop debounce, on its OWN clock - it must never make the
    // engine look alive to the transport gate.
    auto_stop_debounce: Option<std::time::Instant>,

    // The last track we were CONFIDENTLY on. Only used to name a save: the
    // engine stops its cycle for its own post-run dialog, so the live level
    // reads unknown at exactly the moment the user clicks Save, and a recording
    // belongs to the track it was recorded on regardless of what is on screen
    // afterwards. Never used to decide what may be restored — that must be live.
    last_resolved_level: Option<String>,
    // The level_epoch we were last RESOLVED in. Unresolved has two causes with
    // opposite correct behaviour: a FREEZE (menu / pause / post-race dialog —
    // the scan refuses to assert anything while the cycle is stopped, but the
    // level is still resident and the epoch unchanged) and a REAL context
    // change (root reallocated → epoch bumped). Hiding the history panel is
    // only right for the second; hiding it on every save-high-time dialog was
    // the "menu on the right disappears" report.
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
        let history_dir = history_store_v2::default_history_dir();
        let mut history_notices: Vec<String> = Vec::new();
        let history_writer = match history_store_v2::HistoryWriter::open(history_dir.clone()) {
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
        // Recovery-as-history (no banner): an existing checkpoint means an
        // unsaved recording that never reached history (STOP clears it), i.e.
        // the app crashed/closed mid-recording. Bring it back as a PINNED entry.
        // We persist + flush the history BEFORE clearing the checkpoint (done
        // after `app` is built) so a crash can never lose it — at worst a
        // duplicate on the next launch.
        let mut recovered_checkpoint = false;
        let mut recovery_notice = None;
        if let Some(store) = recovery_store.as_ref() {
            match store.load_pending() {
                Ok(Some(cp)) => {
                    let session_label = cp.session.label.clone();
                    let (start, end) = (cp.session.start_tick, cp.session.end_tick);
                    let cp_level = cp.session.level.clone();
                    let cp_rider = cp.session.rider_label();
                    let cp_physics = tas_shared::physics_mode_label(
                        cp.session
                            .renderer_id
                            .unwrap_or(tas_shared::TAS_RENDERER_UNKNOWN),
                        cp.session.fpu_control_word.unwrap_or(0),
                    );
                    let cp_stamps = recording::IdentityStamps {
                        renderer_id: cp.session.renderer_id,
                        fpu_control_word: cp.session.fpu_control_word,
                        rider_character: cp.session.rider_character,
                        rider_stance: cp.session.rider_stance,
                    };
                    if history.push_snapshot_data_with_session(
                        cp.snapshot,
                        session_label.clone(),
                        start,
                        end,
                    ) {
                        if let Some(id) = history.entries().last().map(|e| e.entry_id) {
                            // Restore the track the checkpoint was RECORDED on.
                            // push_* stamps from the live level, which is None
                            // here — we are still in the constructor, before any
                            // level sync — so without this every recovered entry
                            // lands untagged AND pinned, i.e. permanently
                            // floating at the top of every track's history. That
                            // is the "my favourited FE runs show on FM" report.
                            history.set_level(id, cp_level);
                            history.set_rider(id, cp_rider);
                            history.set_physics(id, cp_physics);
                            history.set_stamps(id, cp_stamps);
                            history.set_pinned(id, true);
                            // Mark recovery with a compact ⟲ glyph and let the
                            // panel render the duration via the normal parsed
                            // format (total + dimmed "from …"), instead of dumping
                            // the whole verbose "Recovered · Continued from …,
                            // total …" string into the name.
                            history.rename(id, "⟲".to_string());
                        }
                        recovered_checkpoint = true;
                        recovery_notice = Some(format!(
                            "Recovered an unsaved recording ({}) → pinned in history",
                            session_label
                        ));
                    }
                }
                Ok(None) => {}
                Err(err) => recovery_notice = Some(format!("Crash recovery check failed: {}", err)),
            }
        }
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
            show_debug_drift: settings.show_debug_drift,
            show_history: settings.show_history,
            show_log: settings.show_log,
            show_trajectory: settings.show_trajectory,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            cont_catchup_speed: None,
            cont_catchup_multiplier: settings.cont_catchup_speed,
            cont_cycle_arm: tas_shared::transport::Arm::Continue,
            log_read_cursor: 0,
            finish_scan_cursor: 0,
            finished_at_tick: None,
            finished_hud_cs: None,
            drift_tracker: drift_scan::DriftTracker::default(),
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            cont_controller: None,
            cont_cycle_deadline: None,
            cont_last_outcome: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
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
        if let Some(msg) = recovery_notice {
            app.push_log(&msg);
        }
        app.persist_history_if_needed();

        // Recovery-as-history: only AFTER the recovered entry is durably in the
        // v2 store do we clear the checkpoint — so a crash can't lose it. The
        // clear is gated on a CONFIRMED persist: if the flush reports the write
        // failed, keep the checkpoint so the next launch recovers it again.
        if recovered_checkpoint {
            app.clear_recovery_after_durable_persist();
        }

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
        self.history.push_loaded_snapshot(shared.state(), path);
        self.apply_transport_action(transport::Action::SetContinueFrame(splice));
        Ok(())
    }

    fn playback_speed_for_settings(&self) -> f32 {
        let base_speed = self.cont_catchup_speed.unwrap_or(self.playback_speed);
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
        if let Some(saved) = self.cont_catchup_speed.take() {
            self.playback_speed = saved;
        }
    }

    fn reset_continue_runtime_state(&mut self) {
        self.pending_session_kind = None;
        self.pending_continue_start_tick = None;
        self.cont_cycle_deadline = None;
        // Cancel any in-flight restart/arm/reroll cycle. STOP must mean STOP —
        // without this the controller would keep stepping and silently start
        // recording/playback after the restart completes.
        self.cont_controller = None;
        // A cancelled CONT must not leave live input blocked.
        self.set_cont_suppress_input(false);
    }

    /// Write the CONT live-input-suppression flag into shared memory (no-op if
    /// the value is unchanged or the DLL isn't connected). While set, the DLL
    /// blocks the real key handler so live input can't perturb the bucket
    /// during a CONT's OFF-mode spawn countdown. Cleared the moment the bucket
    /// aligns and on every CONT teardown (stop / abort / disconnect).
    fn set_cont_suppress_input(&mut self, on: bool) {
        if let Some(shared) = self.shared.as_mut() {
            let want = on as u32;
            if shared.state().cont_suppress_input != want {
                shared.state_mut().cont_suppress_input = want;
            }
        }
    }

    /// Build the CONT bucket fingerprint (spawn bits + first-moving frame) from
    /// the loaded recording, or `None` if nothing is loaded. Uses the shared
    /// `detect_first_moving` so the app and the harness judge identically.
    fn cont_bucket_target(&self) -> Option<tas_shared::transport::BucketTarget> {
        let shared = self.shared.as_ref()?;
        let state = shared.state();
        if state.recorded_count == 0 {
            return None;
        }
        let rec0 = state.rec_coords[0];
        Some(tas_shared::transport::BucketTarget {
            expected_start_bits: [rec0[0].to_bits(), rec0[1].to_bits(), rec0[2].to_bits()],
            expected_first_moving: tas_shared::cont::detect_first_moving(
                &state.rec_coords,
                state.recorded_count,
            ),
        })
    }

    fn send_action_command(&mut self, command: TasCommand) {
        if command == TasCommand::Stop {
            self.clear_cont_catchup();
            // reset_continue_runtime_state also cancels any in-flight
            // cont_controller cycle, so STOP after a RestartThen click can't
            // silently complete the restart and start recording/playback.
            self.reset_continue_runtime_state();
        }
        if let Some(shared) = self.shared.as_mut() {
            shared.send_command(command);
        }
        self.log_lines.push(format!("Sent: {:?}", command));
    }

    /// Stop any active REC/PLAY and wait (bounded) for the DLL to reach OFF
    /// BEFORE overwriting the recording buffer (load / history restore).
    /// Without this, a load while recording races the DLL's cycle hook — it's
    /// appending to input_log as the UI rewrites the whole buffer, corrupting
    /// state and silently dropping the in-progress recording. Returns once the
    /// DLL is OFF and the STOP command is acknowledged. Returns `false` rather
    /// than overwriting the recording buffer if the bounded wait expires. The
    /// file dialog that precedes a load already blocked far longer, so a
    /// sub-frame spin here is unnoticeable.
    fn stop_active_session_for_load(&mut self) -> bool {
        self.detach_input_editor();
        let (mode, command_idle) = self
            .shared
            .as_ref()
            .map(|s| (s.mode_volatile(), s.command_idle()))
            .unwrap_or((TasMode::Off as u32, true));
        if stop_is_acknowledged(mode, command_idle) {
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
        // Finalize the just-stopped recording into history NOW, before the
        // caller overwrites the buffer — same as a normal STOP, so the
        // in-progress take isn't lost (it becomes an undoable entry). Capturing
        // it here (not next frame, like the mode-transition handler does) is
        // essential: by next frame the buffer holds the LOADED recording, so
        // the transition handler would snapshot the wrong data. We then pin
        // last_mode = OFF so that handler sees no REC→OFF edge and can't
        // double-finalize.
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
        // RE-READ THE LIVE LEVEL. This is the whole reason the guarantee needed
        // more than a coherent read: we just spun up to 250ms waiting for the
        // DLL to stop, and every caller of this function is about to overwrite
        // the live buffer from history. The level the callers were filtering
        // against was read at frame start, BEFORE that wait — so a quit to the
        // menu or a track load during it would have gone unnoticed and the
        // previous track's recording written in anyway. A seqlock makes a read
        // coherent; it cannot make a stale read fresh. Re-reading here, at the
        // last moment before the write, is what closes it.
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
        // Renderer + x87 precision the game thread is running under (v41).
        // Stamped onto every pushed history entry, compared on restore/load.
        self.history.set_live_physics(shared.physics_mode());
        // Who is on the board (character · stance, v42): stamped onto every
        // pushed history entry, compared on restore/load.
        self.history.set_live_rider(shared.rider());
        self.history
            .set_live_stamps(recording::IdentityStamps::from_live(shared.state()));
        // id and epoch from ONE seqlock window: a separate epoch read can pair
        // the old track's id with the new epoch across a switch, and the stamp
        // below then keeps the old track's history through the very change it
        // exists to detect.
        match tas_shared::resolved_level_id_with_epoch(shared.state()) {
            Some((id, epoch)) => {
                let code = crate::level::level_code_from_id(id);
                // Remember the last track we were CONFIDENTLY on. Used only for
                // naming a save: the engine freezes its cycle for its own post-run
                // dialog, so the level reads unknown at exactly the moment the user
                // clicks Save, and a recording must not lose its tag to that.
                if let Some(c) = code {
                    self.last_resolved_level = Some(c.to_string());
                }
                self.last_resolved_epoch = Some(epoch);
                self.history.set_live_level(code);
            }
            None => {
                // Unresolved is AMBIGUOUS. A freeze (menu / pause / the
                // post-race save dialogs) unresolves too — the scan won't
                // assert a track while the cycle is stopped — but the level is
                // still resident and level_epoch unchanged, so the history
                // panel must NOT vanish there (same-track restores stay safe).
                // Only a genuinely new context (epoch moved past the one we
                // last resolved in) hides the rows until the new track is
                // identified.
                // Plain (non-seqlock) epoch read is fine HERE: this branch
                // stamps nothing. A read torn across a switch costs at most
                // one frame of the wrong verdict and self-corrects on the
                // next poll — unlike the Some() branch, whose stamp persists.
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
    /// path and the transport-bar button path so the two can never diverge. (The
    /// button path used to inline its own copy — including a hand-rolled
    /// first-moving scan — which silently bypassed `detect_first_moving`.)
    fn apply_transport_action(&mut self, cmd: transport::Action) {
        match cmd {
            transport::Action::Send(c) => self.send_action_command(c),
            transport::Action::RestartThen(c) => self.queue_restart_then(c),
            // Undo/Redo overwrite the WHOLE shared input/coord buffer, exactly
            // like a history-row restore — so they need the same guard. The
            // panel path stops an active REC/PLAY first (see HistoryAction::
            // Restore) precisely so the DLL is not writing input_log while we
            // bulk-copy over it; Ctrl+Z did not, and its buttons are
            // mode-independent, so undo during REC raced the DLL writer.
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
                // Stage the splice frame UI-side ONLY. Writing it straight into
                // shared memory here armed cave2's per-frame splice check while
                // the game could still be in PLAY (the CONT/From: controls are
                // live during PLAY): if the playhead was at/past the frame, the
                // replay flipped to REC mid-watch and truncated recorded_count.
                // The TransportController is the only shared-memory writer now —
                // it asserts the marker at arm time, and cave2's g_cave2_contArmed
                // gate refuses any splice that wasn't a genuine ARM_CONTINUE.
                self.continue_from_frame = frame;
                self.continue_from_text = frame.to_string();
            }
            transport::Action::SetResumeSpeed(spd) => {
                // Catch-up in flight: keep the saved resume speed and the staged
                // shared cont_resume_speed in sync so the splice (cave2 reads
                // cont_resume_speed) drops to the speed the user just picked.
                // playback_speed stays the catch-up multiplier; clear_cont_catchup
                // restores playback_speed from cont_catchup_speed at the splice.
                self.cont_catchup_speed = Some(spd);
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
    fn cont_cycle_label(&self) -> &'static str {
        match self.cont_cycle_arm {
            tas_shared::transport::Arm::Continue => "CONT",
            tas_shared::transport::Arm::Play => "PLAY",
            tas_shared::transport::Arm::Rec => "REC",
        }
    }

    fn queue_restart_then(&mut self, command: TasCommand) {
        // Debounce overlapping cycles: a transport cycle (REC/PLAY/CONT
        // restart→arm) is already in flight, so a second F9/F10/F12 press
        // would clobber the single-u32 command slot mid-sequence and could
        // arm the wrong thing (observed: a stray ArmRec after a rapid double
        // CONT wiped the recording to a fresh spawn run). Ignore until the
        // current cycle finishes.
        if self.cont_controller.is_some() {
            self.log_lines.push(format!(
                "{:?} ignored: a restart/arm cycle is already in progress",
                command
            ));
            return;
        }
        // THE MENU GATE. Arming drives an in-process F5 restart, which
        // assumes the engine is actually running a level. From a menu, the
        // pause menu or a dialog the cycle is frozen (game_in_game alone is
        // a stale flag there - same reasoning as the status chip), so an arm
        // would fire into a stopped engine and leave a half-armed cycle.
        // This is the single funnel for the transport buttons AND the
        // F9/F10/F12 hotkeys, so gating here covers both.
        let cycle_ticking = self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400);
        let in_level = self
            .shared
            .as_ref()
            .map(|sh| sh.state().game_in_game != 0)
            .unwrap_or(false);
        if !cycle_ticking || !in_level {
            self.log_lines.push(format!(
                "{:?} ignored: the game is in a menu / paused - enter a level first",
                command
            ));
            return;
        }
        // Refuse degenerate CONT requests that would leave the app
        // half-armed: cont_catchup_speed=Some, playback_speed=multiplier,
        // but no actual playback ever starts (cave2 has nothing to splice),
        // so the PLAY→REC transition that calls clear_cont_catchup never
        // fires and the user is stuck at catchup speed in OFF mode with
        // the green "Catching up..." label glued on. Two cases:
        //   - recorded_count == 0: no recording to continue from
        //   - continue_from_frame == 0: that's just PLAY, not CONT
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
            // Cave2 accepts continue_from_frame == recorded_count (= play
            // the whole recording, then enter REC at the end). Only
            // refuse the strictly-past-end case here. The strict-equal
            // case is what the user hits after a successful CONT cycle:
            // recorded_count caps at the splice frame and they want to
            // press CONT again to redo the same prefix.
            if self.continue_from_frame > recorded {
                self.log_lines.push(format!(
                    "CONT ignored: continue_from_frame={} > recorded_count={}",
                    self.continue_from_frame, recorded
                ));
                return;
            }
        }
        // A take recorded as another character or stance cannot replay as
        // the rider is now, and no restart changes that (the game bakes both
        // into the rider when the level is entered from the menu) - say
        // exactly which screen fixes it. The arm still goes ahead. REC keeps
        // whatever the player chose.
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

        let mut target: Option<tas_shared::transport::BucketTarget> = None;
        let mut gate_align_rec = 0;
        let mut continue_from_frame = 0;
        if command == TasCommand::ArmContinue {
            if self.cont_catchup_speed.is_none() {
                self.cont_catchup_speed = Some(self.playback_speed);
            }
            self.playback_speed = self.cont_catchup_multiplier;
            self.pending_session_kind = Some(RecordingSessionKind::Continue);
            self.pending_continue_start_tick = Some(self.continue_from_frame);
            // CONT is gate-aligned too: the prefix input is indexed from the
            // observed gate and the splice fires at the aligned position, so a
            // countdown landing on a different tick no longer forces a reroll
            // (fe10065-cont: 5/8 -> 8/8 first-try). The recording stays in
            // rec-index space at the splice, so the resumed recording is
            // byte-consistent with the loaded one. target stays None so the
            // controller runs the gate-relative watcher, not the old bucket
            // fingerprint.
            // Only align when the splice is comfortably past the gate, so the
            // watcher completes before the destructive splice and near-gate
            // edge cases fall back to the proven bucket match (see harness).
            let rg = self
                .cont_bucket_target()
                .and_then(|t| t.expected_first_moving)
                .unwrap_or(0);
            if rg > 0 && self.continue_from_frame > rg + tas_shared::cont::BUCKET_MATCH_WINDOW {
                gate_align_rec = rg; // aligned → controller runs the watcher
            } else {
                // Near-gate splice: fall back to the proven bucket match.
                target = self.cont_bucket_target();
            }
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
            // PLAY always starts from tick 0 and indexes its input relative to
            // the recording's gate. The live gate can land on any countdown
            // tick; cave2 shifts the source stream and endpoint by the observed
            // offset. The controller then watches the gate-relative trajectory
            // and rerolls if the hidden spawn state still differs.
            if command == TasCommand::ArmPlay {
                self.continue_from_frame = 0;
                self.continue_from_text = "0".to_string();
                gate_align_rec = self
                    .cont_bucket_target()
                    .and_then(|t| t.expected_first_moving)
                    .unwrap_or(0);
            }
        }

        // A recorded input transition inside the pre-gate hold window is
        // replaced by the gate mask during an aligned replay (see
        // gate_alignment.hpp). The shipped recordings hold their mask stable
        // for 78-140 frames so this stays silent for them - but a recording
        // that DOES pulse right before the gate would otherwise reroll or
        // diverge with no visible reason.
        if gate_align_rec > 0 {
            if let Some(shared) = self.shared.as_ref() {
                let n = tas_shared::cont::pre_gate_hold_overwrites(
                    &shared.state().input_log,
                    gate_align_rec,
                );
                if n > 0 {
                    self.log_lines.push(format!(
                        "WARNING: {} recorded input frame(s) in the {} frames before the gate differ from the gate mask; alignment replays them AS the gate mask",
                        n, tas_shared::cont::GATE_ALIGN_PRE_GATE_LEAD
                    ));
                }
            }
        }

        // Reflect the speed the controller will assert into the live state now
        // so the UI updates immediately (the controller re-asserts it too).
        // For CONT, also stage the RESUME speed so the DLL drops to it
        // atomically at the splice — otherwise the resumed recording
        // fast-forwards at the catch-up rate until the UI polls.
        let cont_resume_speed = if command == TasCommand::ArmContinue {
            self.cont_catchup_speed.unwrap_or(DEFAULT_PLAYBACK_SPEED)
        } else {
            0.0 // unset: DLL leaves the speed alone (PLAY/REC don't splice)
        };
        if let Some(shared) = self.shared.as_mut() {
            let s = shared.state_mut();
            s.playback_speed = self.playback_speed;
            s.cont_resume_speed = cont_resume_speed;
        }

        // Hand the whole restart→arm(→judge→reroll) cycle to the shared
        // controller — the SAME state machine the tas_test harness drives. It
        // serialises Stop→wait-OFF→Restart→wait-rs2→Arm (the command slot is a
        // single u32, so Stop+Restart can't share a frame), runs the CONT bucket
        // judge or aligned PLAY watcher, and rerolls failures.
        let cfg = tas_shared::transport::ArmConfig {
            arm,
            catchup_speed: self.playback_speed,
            continue_from_frame,
            gate_align_rec,
            target,
            max_retries: if target.is_some() || gate_align_rec > 0 {
                CONT_START_MATCH_MAX_RETRIES
            } else {
                0
            },
            predict_bucket: true,
            resume_speed: 0.0,
        };
        self.cont_controller = Some(tas_shared::transport::TransportController::new(cfg));
        self.cont_cycle_deadline = Some(std::time::Instant::now() + CONT_CYCLE_BUDGET);
        self.cont_cycle_arm = arm;
        // Block live input for the restart portion of any replay cycle — set BEFORE the
        // controller's first command so it covers every restart's OFF-mode spawn
        // countdown (the window the mode-based handler block misses). Cleared
        // when the bucket aligns (step_cont_controller / Done) — from there the
        // catch-up PLAY and resumed REC are handler-blocked by mode, and
        // post-splice REC must see live input.
        //
        // CONT needs this while its bucket is judged. Aligned PLAY needs it until
        // ARM_PLAY takes over input injection; otherwise a live key during the
        // OFF-mode restart window can alter the state alignment is meant to replay.
        if target.is_some() || gate_align_rec > 0 {
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

    /// Advance the in-flight transport controller one transition per egui frame
    /// and handle its outcome. This replaces the old hand-rolled
    /// poll_pending_stop_then_restart + pending_after_restart poll +
    /// poll_continue_start_guard + schedule_cont_reroll machinery with the
    /// shared state machine the tas_test harness also drives — so the app and
    /// the test can't diverge. Logs/jitter/clear live here (egui side); the
    /// transitions live in tas_shared::transport.
    fn step_cont_controller(&mut self, ctx: &egui::Context) {
        if self.cont_controller.is_none() {
            return;
        }
        use tas_shared::transport::StepOutcome;
        // Drive the controller with harness-grade timing precision.
        //
        // THE BUG this loop fixes: the old code did exactly ONE step() per egui
        // frame and returned. After a Wait{ARM_SETTLE_MS} that means the Arm
        // command only went out on the NEXT repaint — a vsync-limited, load-
        // jittered ~16ms render frame landing BETWEEN the settle and the Arm.
        // That scatters the F5 spawn-clock phase and was measured (cont-
        // reliability, CONT_YIELD_MS mimic) to crater first-try from ~65% to 0%
        // (real-world logs: 19%). The tas_test harness loops immediately after a
        // settle with no render frame in between — which is why it sees 65–93%.
        //
        // So: handle Wait/Reroll inline and `continue` WITHOUT yielding to render
        // (the post-settle command — Restart/Arm — fires immediately, phase
        // intact). Only the InProgress polling phases (restart-done wait, bucket
        // judge) yield to render, and only after a short bounded spin so the
        // restart-done DETECTION isn't quantized to vsync either. The spin is
        // skipped entirely in the judge phase (see below), so the multi-second
        // replay renders at full rate.
        let spin_until = std::time::Instant::now() + std::time::Duration::from_millis(40);
        loop {
            let outcome = match (self.cont_controller.as_mut(), self.shared.as_mut()) {
                (Some(c), Some(p)) => c.step(p),
                _ => {
                    // Lost the shared-memory connection — drop the cycle.
                    self.cont_controller = None;
                    return;
                }
            };
            match outcome {
                StepOutcome::InProgress => {
                    // Only InProgress can stall: every other outcome either
                    // advances a phase or ends the cycle. Name the phase in the
                    // log — an F5 restart that never completed and an arm the
                    // DLL never processed look identical from here otherwise.
                    if self
                        .cont_cycle_deadline
                        .is_some_and(|d| std::time::Instant::now() > d)
                    {
                        let phase = self
                            .cont_controller
                            .as_ref()
                            .map(|c| c.phase_name())
                            .unwrap_or("?");
                        self.push_log(&format!(
                            "{} gave up: stalled in {} for {}s",
                            self.cont_cycle_label(),
                            phase,
                            CONT_CYCLE_BUDGET.as_secs()
                        ));
                        self.clear_cont_catchup();
                        if let Some(shared) = self.shared.as_mut() {
                            // Straight to the DLL, the same route the controller
                            // takes for its own abort - send_action_command is the
                            // button path and wants a timestamp we do not have here.
                            shared.send_command(TasCommand::Stop);
                            let sp = self.playback_speed;
                            let st = shared.state_mut();
                            st.speed_handoff_pos = 0;
                            st.speed_after_handoff = 0.0;
                            st.playback_speed = sp;
                        }
                        self.reset_continue_runtime_state();
                        self.set_cont_suppress_input(false);
                        return;
                    }
                    // Safe to yield here: restart-done polling / bucket judging
                    // don't set the F5 arm phase. Spin tightly for a bounded
                    // budget (so restart-done detection stays ~3ms, not
                    // vsync-quantized) ONLY while the phase feeds the arm
                    // timing. The multi-second JudgeBucket replay gains nothing
                    // from sub-frame latency, and spinning through it held every
                    // PLAY/CONT at ~25 fps (measured 2026-09-02: 38-42 ms per
                    // frame, all in this loop) - there, poll once per frame.
                    let tight = self
                        .cont_controller
                        .as_ref()
                        .is_some_and(|c| c.needs_tight_polling());
                    if !tight || std::time::Instant::now() >= spin_until {
                        ctx.request_repaint();
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(3));
                }
                StepOutcome::Wait { ms } => {
                    // Fixed Stop→Restart / arm settle. Sleep exactly this long so
                    // the next command fires at a consistent F5 phase, then loop
                    // IMMEDIATELY (no render frame between settle and command).
                    std::thread::sleep(std::time::Duration::from_millis(ms));
                }
                StepOutcome::Reroll {
                    attempt,
                    suggested_delay_ms,
                    observed,
                    expected,
                } => {
                    if self.cont_cycle_arm == tas_shared::transport::Arm::Play {
                        let mismatch = observed
                            .map(|frame| frame.to_string())
                            .unwrap_or_else(|| "?".to_string());
                        self.push_log(&format!(
                            "PLAY watcher reroll {}/{} (first mismatch at gate+{})",
                            attempt, CONT_START_MATCH_MAX_RETRIES, mismatch
                        ));
                    } else {
                        self.push_log(&format!(
                            "{} bucket reroll {}/{} (observed first-moving={:?} expected={:?})",
                            self.cont_cycle_label(),
                            attempt,
                            CONT_START_MATCH_MAX_RETRIES,
                            observed,
                            expected
                        ));
                    }
                    // Vary the wall clock so the next F5 lands at a different
                    // accumulator-modulo-tick phase, then loop immediately.
                    std::thread::sleep(std::time::Duration::from_millis(suggested_delay_ms));
                }
                StepOutcome::Done {
                    retries_used,
                    completed_via,
                } => {
                    if retries_used > 0 {
                        if self.cont_cycle_arm == tas_shared::transport::Arm::Play {
                            self.push_log(&format!(
                                "PLAY watcher accepted after {} restart retr{}",
                                retries_used,
                                if retries_used == 1 { "y" } else { "ies" }
                            ));
                        } else {
                            self.push_log(&format!(
                                "{} bucket aligned after {} restart retr{}",
                                self.cont_cycle_label(),
                                retries_used,
                                if retries_used == 1 { "y" } else { "ies" }
                            ));
                        }
                    }
                    // A judged PLAY has no splice to restore its speed at: the
                    // DLL already handed back at the first moving frame, so all
                    // that is left is to stop treating the judge speed as the
                    // user's. CONT must NOT go through here - its catch-up is
                    // still running toward the splice.
                    if self.cont_cycle_arm == tas_shared::transport::Arm::Play {
                        self.clear_cont_catchup();
                    }
                    // Stash for the resume summary emitted at the REC-start splice,
                    // where the actual resume frame is known. attempts = rerolls + 1.
                    self.cont_last_outcome = Some((retries_used + 1, completed_via));
                    self.cont_controller = None;
                    // Bucket aligned — release the live-input block. The
                    // remaining catch-up PLAY → splice → REC are all
                    // handler-blocked by mode, and post-splice REC must record
                    // live input (the resumed recording).
                    self.set_cont_suppress_input(false);
                    return;
                }
                StepOutcome::Aborted { reason } => {
                    self.push_log(&format!("{} aborted: {}", self.cont_cycle_label(), reason));
                    self.clear_cont_catchup();
                    // Drop any handover the aborted attempt had staged, and push
                    // the restored speed through: an aborted PLAY must not leave
                    // the game fast-forwarding at the judge speed.
                    if let Some(shared) = self.shared.as_mut() {
                        let sp = self.playback_speed;
                        let st = shared.state_mut();
                        st.speed_handoff_pos = 0;
                        st.speed_after_handoff = 0.0;
                        st.playback_speed = sp;
                    }
                    // also clears cont_controller
                    self.reset_continue_runtime_state();
                    self.set_cont_suppress_input(false);
                    return;
                }
            }
        }
    }

    /// Clear the crash-recovery checkpoint, but ONLY after the history it
    /// represents is durably committed. Mirrors the startup recovery ordering:
    /// persist → flush (which now reports the real durability result) → clear.
    /// If the flush says the manifest write failed, the checkpoint is KEPT so
    /// the recording is recovered on the next launch instead of lost. The
    /// recovery writer is drained first so no in-flight write can recreate the
    /// checkpoint files after we delete them.
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
        self.recovery_writer.flush();
        if durable {
            if let Some(store) = self.recovery_store.as_mut() {
                let _ = store.clear_pending();
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
        // The re-queue is a new attempt: forget the failure we already reacted
        // to. `persist()` cleared the writer's failed marker, but if the worker
        // fails again before this thread observes that 0, the marker comes
        // back holding the SAME revision. Without this reset that would compare
        // equal to `last_failed_revision`, no retry would be scheduled, and the
        // revision would stay queued-but-never-durable until history changed.
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
        self.drift_cache = drift::DriftCache::default();
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
        // Never mutate the buffer the game is actively replaying/recording.
        // Instead of dropping the edit, auto-STOP the run and keep the edit
        // queued; it applies on the frame mode returns to Off. We issue STOP
        // only once (latched) so we don't spam the command slot while the
        // game settles.
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
            self.history
                .push_snapshot_data_with_session(snapshot, label, 0, 0);
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
    /// actually resumed (the requested splice tick), how many F5 bucket attempts
    /// it took, and HOW the bucket was accepted. A `NoSignal` accept is flagged
    /// loudly because it was not positively confirmed — the resume may be on a
    /// near-miss bucket, which is the likely cause of an "off" resume.
    fn log_cont_resume_summary(&mut self) {
        use tas_shared::transport::CompletedVia;
        let Some(session) = self.active_recording_session else {
            return;
        };
        if session.kind != RecordingSessionKind::Continue {
            return; // plain REC — no bucket/resume story to tell
        }
        let (attempts, via) = self
            .cont_last_outcome
            .take()
            .unwrap_or((1, CompletedVia::Unjudged));
        let verdict = match via {
            CompletedVia::BucketMatched => "bucket matched",
            CompletedVia::NoSignal => {
                "⚠ bucket UNCONFIRMED (no-signal accept — resume may be on a near-miss bucket)"
            }
            CompletedVia::Unjudged => "bucket unjudged",
        };
        self.push_log(&format!(
            "CONT resumed at frame {} after {} bucket attempt{} — {}",
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
        // Decide on the UI thread (throttle to one write per ~1.5s of recording
        // GROWTH — see DEFAULT_RECOVERY_DEBOUNCE_MS), but run the disk write OFF
        // it via the serialized RecoveryWriter so REC never hitches. STOP no
        // longer forces a recovery write: finalize routes the finished recording
        // into durable history instead, then clears the checkpoint. Best-effort:
        // a failed background write just means a slightly staler recovery file.
        let job = match self.recovery_store.as_mut() {
            Some(store) => {
                store.take_write_job(snapshot, &self.segment_tracker.segments, session, force)
            }
            None => None,
        };
        if let Some(job) = job {
            // Submit to the single serialized writer (ordered + flushable),
            // NOT a detached thread — a detached write could complete after a
            // later `clear_pending()` and resurrect the checkpoint.
            self.recovery_writer.submit(job);
        }
    }

    fn update_recording_recovery_progress(&mut self, snapshot: &recording::RecordingSnapshot) {
        // Stamp the track NOW, while we are recording and can still see it. If
        // this checkpoint ever comes back, it comes back during startup — when
        // nothing has read the live level yet — so asking then is too late.
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
        // time ("Finish 0:53.34", flag in the panel), not by its length. The
        // HUD timer froze at the line; when the DLL's race-timer feed is
        // empty, the start-line-to-finish-line tick count from the track
        // geometry is that time (the in-game timer starts at the start
        // trigger, ~5 s below the spawn on Forest Easy - NOT at first
        // movement, which read 3:55.58 for a 3:50.57 run).
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
                    tas_shared::cont::detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
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
        let _ = self.history.push_completed_session(
            snapshot.clone(),
            label,
            session_context.start_tick,
            session_context.end_tick,
            finish,
        );
        // Make the recording DURABLE in history BEFORE clearing the recovery
        // checkpoint — otherwise a crash between "pushed to in-memory history"
        // and "background writer committed" would lose it (checkpoint gone,
        // blob not on disk). The clear is gated on a CONFIRMED persist.
        self.clear_recovery_after_durable_persist();
    }

    /// Check if the game process is still alive by monitoring frame_count advancement.
    /// If frame_count hasn't changed for ~3 seconds, assume the game crashed.
    fn check_game_health(&mut self) {
        // Sample cycle activity every frame (cheap) for the In-Game chip:
        // game_in_game freezes at its last value when the Supreme::Cycle hook
        // stops running (quit to menu / pause / dialog), so a fresh frame_count
        // advance is the real "ticking a level" signal.
        if let Some(ref shared) = self.shared {
            let fc = shared.frame_count_volatile();
            if !self.cycle_fc_seeded {
                // Baseline only. The FIRST sample after launch or reconnect is
                // not an advance — a frozen menu has a nonzero frame_count
                // too, and counting it as one opened a ~400ms window where the
                // transport gate read "ticking" against a stopped engine.
                self.cycle_fc_seeded = true;
                self.cycle_fc = fc;
            } else if fc != self.cycle_fc {
                self.cycle_fc = fc;
                self.cycle_advance_at = std::time::Instant::now();
            }
        }
        if self.last_health_check.elapsed() < std::time::Duration::from_secs(1) {
            return;
        }
        self.last_health_check = std::time::Instant::now();

        if let Some(ref shared) = self.shared {
            let current_frame = shared.frame_count_volatile();
            if current_frame == self.last_frame_count {
                self.stale_frame_ticks += 1;
                // Re-test liveness every 5 stale seconds, not once. The cycle
                // also freezes at the menu while the game is alive, so a
                // one-shot check passes there and then never runs again when
                // the game exits later. That left tas_ui "connected" to a dead
                // mapping for days: repainting at full rate against a stale
                // MODE_PLAY, re-sending STOP every 2 s, and leaking until it
                // burned most of a core (2026-09-02 post-mortem).
                if self.stale_frame_ticks.is_multiple_of(5) {
                    // Check if Supreme.exe is actually running
                    if !win32::is_supreme_running() {
                        self.push_log("Game process not found — disconnecting shared memory");
                        self.shared = None;
                        self.detach_input_editor();
                        self.connect_error = Some(
                            "Supreme.exe has exited. Inject TAS_Helper.dll after restarting the game."
                                .into(),
                        );
                        self.stale_frame_ticks = 0;
                        self.log_read_cursor = 0;
                        // Invalidate cached game PID — a fresh Supreme.exe
                        // launch will get a different PID and our global-
                        // shortcut foreground gate would otherwise stay stale.
                        self.game_pid_cached = None;
                    }
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
    /// REC/PLAY/STOP/CONT without alt-tabbing to tas_ui. Crucially, the
    /// keys are *observed*, not consumed — the game still receives them.
    ///
    /// Edge state is updated every frame regardless of focus so we
    /// never strand on a "was pressed last time we looked" entry after
    /// a focus change while a key was held.
    fn poll_global_shortcuts(&mut self) -> Vec<transport::Action> {
        // Step 1: read current pressed state for all four keys.
        let now: [bool; 4] =
            [win32::VK_F9, win32::VK_F10, win32::VK_F11, win32::VK_F12].map(win32::key_is_down);
        // Step 2: compute edges and update cache (always — see doc comment).
        let edges = compute_global_key_edges(now, &mut self.prev_global_keys);
        // Short-circuit if no key transitioned this frame.
        if !edges.iter().any(|&e| e) {
            return Vec::new();
        }

        // Step 3: gate emission on Supreme.exe being the foreground window.
        // If the cache is stale or unresolved, try to fill it. We invalidate
        // the cache when shared-memory disconnects (game closed), so a stale
        // PID only persists across a same-session game restart that doesn't
        // tear down shared memory — rare and benign (the PID just won't match).
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

        // Step 4: emit actions for each newly-pressed key.
        let mut actions = Vec::new();
        if edges[GlobalShortcutSlot::F9 as usize] {
            actions.push(transport::Action::RestartThen(TasCommand::ArmRec));
            actions.push(transport::Action::Log("Global F9 (in-game): REC".into()));
        }
        if edges[GlobalShortcutSlot::F10 as usize] {
            actions.push(transport::Action::RestartThen(TasCommand::ArmPlay));
            actions.push(transport::Action::Log("Global F10 (in-game): PLAY".into()));
        }
        if edges[GlobalShortcutSlot::F11 as usize] {
            actions.push(transport::Action::Send(TasCommand::Stop));
            actions.push(transport::Action::Log("Global F11 (in-game): STOP".into()));
        }
        if edges[GlobalShortcutSlot::F12 as usize] {
            actions.push(transport::Action::SetContinueFrame(
                self.continue_from_frame,
            ));
            actions.push(transport::Action::RestartThen(TasCommand::ArmContinue));
            actions.push(transport::Action::Log("Global F12 (in-game): CONT".into()));
        }
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
                actions.push(transport::Action::RestartThen(TasCommand::ArmRec));
                actions.push(transport::Action::Log("Shortcut: F9 REC".into()));
            }

            // F10: Arm PLAY — same as clicking PLAY button (restart first)
            if input.key_pressed(egui::Key::F10) {
                actions.push(transport::Action::RestartThen(TasCommand::ArmPlay));
                actions.push(transport::Action::Log("Shortcut: F10 PLAY".into()));
            }

            // F11: STOP
            if input.key_pressed(egui::Key::F11) {
                actions.push(transport::Action::Send(TasCommand::Stop));
                actions.push(transport::Action::Log("Shortcut: F11 STOP".into()));
            }

            // F12: Continue record — same as clicking CONT button (restart first)
            if input.key_pressed(egui::Key::F12) {
                actions.push(transport::Action::SetContinueFrame(
                    self.continue_from_frame,
                ));
                actions.push(transport::Action::RestartThen(TasCommand::ArmContinue));
                actions.push(transport::Action::Log("Shortcut: F12 CONT".into()));
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
                    if modifiers.ctrl || modifiers.mac_cmd
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
            if let Some(ref shared) = self.shared {
                if let Some(path) = recording::save_dialog_with_segments(
                    shared.state(),
                    &self.segment_tracker.segments,
                    &mut self.log_lines,
                    level.as_deref(),
                    self.loaded_identity.as_ref(),
                ) {
                    self.history.push_save_marker(shared.state(), &path);
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
        // Open bypassed the per-level guarantee entirely: the dialog merely
        // STARTED in the current track's folder, then accepted any file the user
        // picked and auto-played it. Recordings are named `<CODE>-<name>`, so the
        // file itself says which track it belongs to — check it, against a level
        // re-read after the stop above. Loading another track's recording is not
        // a subtle failure: the spawn is somewhere else entirely and the run is
        // meaningless. (Unknown on either side cannot prove a mismatch and is
        // allowed through — same rule tas_test's pre-flight uses.)
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
                let _ = self.history.push_loaded_snapshot(shared.state(), &path);
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
            show_debug_drift: self.show_debug_drift,
            show_history: self.show_history,
            show_trajectory: self.show_trajectory,
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

        // Synchronise the live level FIRST, before anything reads or acts on
        // it this frame.
        //
        // This used to run inside the central panel, AFTER the history panel had
        // already rendered its rows and dispatched restores. On the first frame
        // following a level change that panel therefore still listed — and would
        // happily restore — the previous track's entries. Ordering is part of
        // the guarantee: a per-frame fact has to be established before the frame
        // consumes it.
        self.sync_live_level();

        // Check game health (crash detection) — also samples cycle activity.
        self.check_game_health();

        // Auto-stop REC/PLAY when you leave the level. Quitting to the menu
        // tears the level down and STOPS Supreme::Cycle, so the DLL's
        // root-change auto-stop (which runs in the cycle hook) can't fire — the
        // recording would stay armed at the menu. tas_ui sees it: the cycle
        // heartbeat (frame_count) freezes. A generous threshold rides out the
        // CONT F5-reload stall and a brief pause-menu glance, but a sustained
        // freeze means you've left → stop. (Pause >threshold also stops, which
        // is fine — nothing meaningful records while paused.)
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
                // Debounce the re-fire on its OWN timestamp. This used to
                // reset cycle_advance_at, which also told the transport gate
                // "the engine is ticking" for 400ms — at a frozen menu, i.e.
                // exactly when arming must stay refused.
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
            // (Measured 2026-09-02: the per-frame copy was not the source of
            // the slow memory growth, but it is still 850 KB of memcpy a frame
            // that nothing reads.)
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
                    // Splice fired (or REC began).
                    //
                    // The controller has normally cleared itself long before
                    // this: it accepts the bucket at first_moving + 64 while the
                    // splice sits thousands of ticks later. But a splice EARLIER
                    // than the judge window (a redo from a low frame) can reach
                    // REC first, and this handler runs before step_cont_controller
                    // in the same frame. Dropping the controller without its Done
                    // path would strand cont_suppress_input SET - and the whole
                    // point of the resumed REC is to record live input.
                    if self.cont_controller.take().is_some() {
                        self.cont_cycle_deadline = None;
                        self.set_cont_suppress_input(false);
                    }
                    // Fresh finish-line watch for this session. A CONT splice
                    // resumes mid-run, so start the scan at the resume tick —
                    // the prefix was already checked when it was recorded.
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

                // Finish-line watch (1.7): when the recording crosses the
                // track's finish line, stop REC — the race is over, the timer
                // froze at the line; recording the run-out is never wanted.
                // Incremental: only the ticks since the last frame are
                // scanned.
                if self.finished_at_tick.is_none() {
                    let resolved_geometry = self
                        .shared
                        .as_ref()
                        .and_then(|sh| tas_shared::resolved_level_id(sh.state()))
                        .and_then(crate::level::level_code_from_id)
                        .is_some();
                    let cross = self.shared.as_ref().and_then(|shared| {
                        let s = shared.state();
                        // Resolved level only: this picks the FINISH-LINE
                        // GEOMETRY, so a stale id mid-swap would test the run
                        // against the previous track's line and could auto-stop
                        // REC in the wrong place. Unresolved => no geometry =>
                        // no crossing claimed, which is the safe direction.
                        crate::start_line::finish_cross_tick(
                            &s.rec_coords,
                            recorded,
                            tas_shared::resolved_level_id(s)
                                .and_then(crate::level::level_code_from_id),
                            self.finish_scan_cursor,
                        )
                    });
                    // Only advance past ticks we actually SCANNED. While the level
                    // is unresolved there is no geometry, so those ticks were not
                    // examined — advancing would skip a crossing permanently.
                    if resolved_geometry {
                        self.finish_scan_cursor = recorded.max(1);
                    }
                    if let Some(tick) = cross {
                        self.finished_at_tick = Some(tick);
                        // Latch the HUD time NOW, in the same poll that saw the
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

        // Process keyboard shortcuts first. handle_shortcuts handles
        // keys delivered to tas_ui via egui (i.e. when tas_ui has
        // focus); poll_global_shortcuts handles keys observed via
        // GetAsyncKeyState when the game has focus. The two paths are
        // mutually exclusive (gated on which window is foreground), so
        // a single F-key press fires exactly one action regardless of
        // which window the user was in.
        let mut shortcut_actions = self.handle_shortcuts(ctx);
        shortcut_actions.extend(self.poll_global_shortcuts());

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        let level = self.level_for_save().map(str::to_string);
                        if let Some(ref shared) = self.shared {
                            if let Some(path) = recording::save_dialog_with_segments(
                                shared.state(),
                                &self.segment_tracker.segments,
                                &mut self.log_lines,
                                level.as_deref(),
                                self.loaded_identity.as_ref(),
                            ) {
                                self.history.push_save_marker(shared.state(), &path);
                            }
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        self.load_recording_flow();
                    }
                    ui.separator();
                    // Settings — moved out of the transport row to save
                    // horizontal space there.
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
                    ui.checkbox(&mut self.show_trajectory, "Trajectory");
                    ui.separator();
                    // Debug section — rarely touched diagnostic toggles.
                    ui.label(
                        egui::RichText::new("Debug")
                            .small()
                            .color(egui::Color32::from_gray(140)),
                    );
                    ui.checkbox(&mut self.show_debug_drift, "Drift overlay");
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
            // Retry on our own every 2 s (the repaint floor below): the game
            // is often launched, or relaunched, after tas_ui, and the mapping
            // only exists once TAS_Helper has initialized. Until 2026-09-02
            // this needed a click on "Retry Connection".
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
                                    config::show(ui, shared.state_mut());
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
            .map(|_| history_store_v2::default_history_dir());
        let mut open_history_dir = false;
        if self.show_history {
            egui::SidePanel::right("history_panel")
                .resizable(true)
                .default_width(280.0)
                .show(ctx, |ui| {
                    // Compact header: "History" + count, autosave path
                    // moved to a tooltip on hover (was wrapping over two
                    // lines and burning vertical space).
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

                    // The status chip's menu signal, reused: game_in_game is a
                    // stale flag at menus (the Cycle hook stops writing it), so
                    // gate on the cycle actually ticking. The panel uses this to
                    // say "In Menu" instead of a perpetual "resolving…" — at a
                    // menu nothing is being resolved, the scan is deliberately
                    // suppressed there.
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
                        // Restoring writes into the live game buffer — needs a
                        // connection. Pin/rename don't. Stop any active REC/PLAY
                        // first so the DLL isn't writing input_log while we
                        // overwrite the whole buffer (clicking a history entry
                        // mid-record must stop the record, not race it).
                        //
                        // Resolve the clicked row to its STABLE entry_id before
                        // stopping: stop_active_session_for_load finalizes the
                        // interrupted take into history, which can evict the
                        // oldest entry and shift positional indices — so a
                        // post-stop restore_index(idx) could target the wrong row.
                        let target_id = self.history.entries().get(idx).map(|e| e.entry_id);
                        if !self.stop_active_session_for_load() {
                            continue;
                        }
                        let idx = target_id
                            .and_then(|id| {
                                self.history.entries().iter().position(|e| e.entry_id == id)
                            })
                            .unwrap_or(idx);
                        // stop_active_session_for_load re-read the live level
                        // just now, so this decision is made against the track
                        // we are on AFTER the wait, not the one we were on when
                        // the frame began. Say so out loud rather than doing
                        // nothing: a click that silently no-ops reads as a bug.
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

        // Advance the in-flight restart/arm/reroll cycle (shared controller).
        self.step_cont_controller(ctx);

        // Apply shortcut actions to shared state (same dispatch as the buttons).
        for cmd in shortcut_actions {
            self.apply_transport_action(cmd);
        }

        // Apply any input edit the timeline produced last frame.
        self.poll_script_file();
        self.apply_pending_input_edit();

        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Transport bar at top
            let cmds = if let Some(ref mut shared) = self.shared {
                let mode = shared.state().mode_enum();
                let recorded = shared.state().recorded_count;
                // During catch-up the resume speed lives in cont_catchup_speed
                // (playback_speed is the catch-up multiplier); otherwise it's
                // just the live play speed. The buttons highlight/edit this.
                let resume_speed = self.cont_catchup_speed.unwrap_or(self.playback_speed);

                // Stamp new history entries with the level they're made on
                // (per-level history filter).
                //
                // level_id going KNOWN -> UNKNOWN means the level was torn down:
                // its resource path strings left the heap, which is what the
                // DLL's scan reads. Verified live that an F5 restart does NOT
                // clear it (level_id stayed 0x0 across a CMD_RESTART while
                // player_ptr moved 0x0b271a08 -> 0x0d97f340), so this fires on a
                // real level change and not on every reroll — player_ptr would
                // have been a false trigger.
                //
                // Without this the last level stayed asserted through the menu,
                // the load, and the first ~1.5s of the NEW track, so the panel
                // filtered to the old track and mis-stamped anything pushed
                // mid-load. A wrong tag is worse than none: it can't be spotted.

                // Mirror of queue_restart_then's menu gate, for the VISUAL
                // disable: the buttons gray out in a menu instead of
                // accepting a click the funnel would refuse anyway.
                let arming_allowed = shared.state().game_in_game != 0
                    && self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400);
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
                        catchup_active: self.cont_catchup_speed.is_some(),
                        resume_speed,
                        arming_allowed,
                    },
                )
            } else {
                Vec::new()
            };

            // Dispatch button actions through the SAME path as keyboard
            // shortcuts (apply_transport_action) so the two can't diverge. This
            // routes CONT-arming through queue_restart_then →
            // set_continue_start_guard → detect_first_moving (the shared fn),
            // replacing the button path's old hand-rolled first-moving scan, and
            // also fixes the button PLAY path to reset continue_from_frame.
            for cmd in cmds {
                self.apply_transport_action(cmd);
            }

            if let Some(ref mut shared) = self.shared {
                // Sync playback_speed to shared state for Cave 5 every frame.
                // During a CONT catch-up self.playback_speed IS the catch-up
                // multiplier (set in queue_restart_then), so this continuously
                // re-asserts it — overriding the brief speed reset the in-process
                // F5 restart causes. (Gating this on an in-flight controller
                // removed the continuous re-assertion and the catch-up replayed
                // at the play speed instead of the multiplier.)
                //
                // BUT: once the splice has fired (mode == REC) mid-catch-up, the
                // DLL has already dropped playback_speed to the resume speed
                // atomically. Assert THAT resume speed here — not the catch-up
                // multiplier — so we don't stomp it back to e.g. 64x for the
                // frame(s) before our mode-transition handler runs. This closes
                // the post-splice overshoot.
                //
                // A judged PLAY is different again: its handover fires mid-replay
                // with no mode change to notice it by, and it can be staged and
                // consumed between two frames of this closure. So rather than
                // race it, this yields the field entirely - the controller owns
                // playback_speed for that whole cycle, asserting the catch-up
                // before the handover and the resume speed after, and it steps at
                // least as often as this runs. Two writers with different views of
                // the same handover is exactly how the catch-up got reinstated
                // over it.
                let controller_owns_speed = self
                    .cont_controller
                    .as_ref()
                    .is_some_and(|c| c.owns_playback_speed());
                let catchup_active = self.cont_catchup_speed.is_some();
                let mode = shared.state().mode;
                let speed_to_assert = if catchup_active && mode == TasMode::Rec as u32 {
                    self.cont_catchup_speed.unwrap_or(self.playback_speed)
                } else {
                    self.playback_speed
                };
                if !controller_owns_speed {
                    shared.state_mut().playback_speed = speed_to_assert;
                }

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
                    let timer = tas_shared::cont::detect_first_moving(&state.rec_coords, total)
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

                if self.show_debug_drift {
                    ui.separator();
                    ui.label(egui::RichText::new("Debug drift").strong());
                    drift::show(ui, state, &mut self.drift_cache);
                }

                // Trajectory + Rotation — toggleable via the View menu,
                // hidden by default. Fixed 220 px container when shown.
                if self.show_trajectory {
                    ui.add_space(4.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(egui::RichText::new("Trajectory").strong());
                        ui.separator();
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), 220.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                trajectory::show(ui, state, &mut self.trajectory_cache);
                            },
                        );
                    });
                }

                // Diagnostics footer — DLL counters. Only useful when
                // debugging the DLL itself; gated on Debug Config so it's
                // hidden during normal use.
                if self.show_config {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Recorded: {} | Playback: {} | BB3B10: {} | Blocks: {} | Events: {}",
                            state.recorded_count,
                            state.playback_pos,
                            state.bb3b10_call_count,
                            state.handler_block_count,
                            state.event_count
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

                // NOTE: do NOT sync self.continue_from_frame → shared every
                // frame. That value is *staging* for the next CONT, not a
                // live signal: cave2's PLAY handler treats any non-zero
                // continue_from_frame in shared as a splice marker and
                // auto-switches PLAY → REC at that frame. If we synced
                // every frame, then editing the "From:" textbox or dragging
                // the timeline continue marker during PLAY would silently
                // hijack PLAY into a CONT-like splice. Shared is written
                // only when CONT is actually armed (queue_restart_then,
                // SetContinueFrame action, retry paths).
            }
        });

        // Poll DLL log ring buffer
        if let Some(ref shared) = self.shared {
            use tas_shared::TasLogSeverity;
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

        for w in self.history.take_warnings() {
            self.log_lines.push(format!("History: {}", w));
        }
        self.persist_history_if_needed();

        // AUTO-REFRESH, but only as fast as there is something to show.
        //
        // Repainting at a flat 30fps costs the GAME about 10ms per menu frame —
        // measured, and reversible within one session: 48.4ms with tas_ui closed,
        // 58.4ms with it open, 48.4ms again once closed (3/3 each way). MINIMIZING
        // tas_ui also removes it completely, which is what identifies our own
        // RENDERING as the cost rather than the shared-memory polling. Two
        // processes competing for the GPU, and the game loses ~20% of its menu
        // video speed to a window that, while idle, is drawing the same pixels.
        //
        // So: full rate whenever anything is actually moving — a TAS mode is
        // armed, or the engine cycle is ticking (in a level) — and a lazy rate
        // when the game is sitting still at a menu. This does NOT make the UI
        // feel sluggish: egui repaints immediately on input regardless, so
        // `request_repaint_after` only sets the IDLE floor.
        let mode_active = self
            .shared
            .as_ref()
            .map(|s| s.mode_volatile() != TasMode::Off as u32)
            .unwrap_or(false);
        let cycle_ticking = self.cycle_advance_at.elapsed() < std::time::Duration::from_millis(400);
        // eframe 0.29 still runs update()+paint while minimized, and every
        // painted frame costs a little renderer-side memory on the DX12/Vulkan
        // path (measured ~15 KB/s at 60 fps, none on GL). Nobody can see a
        // minimized window, so idle it hard regardless of mode.
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

    // Default window 1100x680. Wider than the previous 960 to give the
    // transport row breathing room when `from:` is visible alongside
    // the 280 px history rail — at 960 the row clipped Redo and ate
    // into the speed presets once a recording loaded.
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
            show_debug_drift: false,
            show_history: false,
            show_log: false,
            show_trajectory: false,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            cont_catchup_speed: None,
            cont_catchup_multiplier: 12.0,
            cont_cycle_arm: tas_shared::transport::Arm::Continue,
            log_read_cursor: 0,
            drift_tracker: drift_scan::DriftTracker::default(),
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            cont_controller: None,
            cont_cycle_deadline: None,
            cont_last_outcome: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
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

    #[test]
    fn shortcut_f9_arms_rec() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F9, Modifiers::NONE);
        assert!(action_has_restart_then(&actions, TasCommand::ArmRec));
        assert!(action_has_log(&actions, "F9"));
    }

    #[test]
    fn shortcut_f10_arms_play() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F10, Modifiers::NONE);
        assert!(action_has_restart_then(&actions, TasCommand::ArmPlay));
        assert!(action_has_log(&actions, "F10"));
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
        let mut app = test_app();
        let actions = press_key(&mut app, Key::F12, Modifiers::NONE);
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

    /// The global-shortcut edge detector must fire exactly once per
    /// false→true transition and update the previous-state cache so
    /// subsequent calls with the same held state do NOT re-fire. If
    /// this breaks, holding F9 in-game would spam REC actions every
    /// frame instead of arming once.
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

    /// Holding a key across multiple frames with intervening focus loss
    /// (simulated here by repeated identical reads) must not produce
    /// repeated edges. This guarantees in-game F9 hold won't spam REC.
    #[test]
    fn global_shortcut_held_does_not_repeat() {
        let mut prev = [false; 4];
        compute_global_key_edges([true, true, true, true], &mut prev);
        for _ in 0..100 {
            let edges = compute_global_key_edges([true, true, true, true], &mut prev);
            assert_eq!(edges, [false, false, false, false]);
        }
    }

    /// The CONT retry-jitter sequence must visit a spread of distinct
    /// values so timing varies enough to cross tick-accumulator
    /// boundaries between retries. If everyone hits the same ms, we'd
    /// re-land in the same F5 bucket every retry.
    #[test]
    fn cont_retry_jitter_visits_distinct_values() {
        use tas_shared::transport::{cont_retry_jitter_ms, NATURAL_RESTART_ATTEMPTS};
        // The escape jitter (after the un-jittered natural-variance attempts)
        // should cycle through distinct phases (gcd(7,17) = 1).
        let lo = NATURAL_RESTART_ATTEMPTS + 1;
        let values: std::collections::HashSet<u64> =
            (lo..lo + 17).map(cont_retry_jitter_ms).collect();
        assert!(
            values.len() >= 10,
            "Jitter sequence too repetitive: {} distinct values",
            values.len()
        );
        // No value should exceed ~20ms — a single retry shouldn't feel
        // like an unresponsive UI freeze.
        let max = (lo..lo + 17).map(cont_retry_jitter_ms).max().unwrap();
        assert!(max <= 20, "Jitter ms upper bound too large: {}", max);
        // And the first attempts must be jitter-free.
        assert_eq!(cont_retry_jitter_ms(1), 0);
    }

    /// Pressing CONT (via F12 or otherwise) with no recording loaded
    /// must NOT leave the app half-armed at catchup speed. Previously
    /// it set cont_catchup_speed=Some(saved) and playback_speed=multiplier,
    /// then sent Restart, but cave2 couldn't start playback (nothing to
    /// continue from). The PLAY→REC transition that normally calls
    /// clear_cont_catchup never fires, leaving the UI stuck at e.g. 32×
    /// in OFF mode with the green "Catching up..." label glued on.
    ///
    /// This test exercises queue_restart_then directly because the test
    /// harness's prepare_restart_action helper bypasses the recorded-count
    /// check. (We can't trigger the real shared-memory path in a unit
    /// test without a live DLL.)
    #[test]
    fn cont_with_no_recording_does_not_arm_catchup() {
        let mut app = test_app();
        app.playback_speed = 1.0;
        app.cont_catchup_multiplier = 32.0;
        // No shared memory set, so recorded_count is implicitly 0 (the
        // helper returns 0 via the unwrap_or fallback). queue_restart_then
        // should bail before touching catchup state.
        app.queue_restart_then(TasCommand::ArmContinue);
        assert!(
            app.cont_catchup_speed.is_none(),
            "CONT without recording must not engage catchup speed"
        );
        assert!(
            (app.playback_speed - 1.0).abs() < 0.001,
            "playback_speed must remain at pre-CONT value, got {}",
            app.playback_speed
        );
        assert!(
            app.cont_controller.is_none(),
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
        assert!(app.cont_controller.is_none());
    }
}
