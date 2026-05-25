mod history_store;
mod panels;
mod pico;
mod recording;
mod settings;

use eframe::egui;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

/// Find Supreme.exe's PID by enumerating processes. Returns None if not
/// running. Used by the global-shortcut poll to gate "F-key fired while
/// game has focus" — we don't want F9 in the user's browser to start a
/// recording. Cached at the call site and invalidated when shared-memory
/// disconnects (game closed/restarted).
#[cfg(windows)]
fn find_supreme_pid() -> Option<u32> {
    use std::ffi::c_void;
    type HANDLE = *mut c_void;
    type DWORD = u32;
    type BOOL = i32;
    type WCHAR = u16;
    const TH32CS_SNAPPROCESS: DWORD = 0x00000002;
    const MAX_PATH: usize = 260;
    const INVALID_HANDLE_VALUE: HANDLE = -1isize as *mut c_void;

    #[repr(C)]
    struct ProcessEntry32W {
        dw_size: DWORD,
        cnt_usage: DWORD,
        th32_process_id: DWORD,
        th32_default_heap_id: usize,
        th32_module_id: DWORD,
        cnt_threads: DWORD,
        th32_parent_process_id: DWORD,
        pc_pri_class_base: i32,
        dw_flags: DWORD,
        sz_exe_file: [WCHAR; MAX_PATH],
    }

    extern "system" {
        fn CreateToolhelp32Snapshot(flags: DWORD, pid: DWORD) -> HANDLE;
        fn Process32FirstW(snap: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
        fn Process32NextW(snap: HANDLE, entry: *mut ProcessEntry32W) -> BOOL;
        fn CloseHandle(h: HANDLE) -> BOOL;
    }

    // Match both the unversioned and versioned Supreme executable names —
    // is_supreme_running() and the injector also accept Supreme_v1.035.exe,
    // so if we only look for the plain name here, F9-F12 global shortcuts
    // silently stop firing when the user runs the versioned build.
    let targets: [Vec<u16>; 2] = [
        "Supreme.exe".encode_utf16().collect(),
        "Supreme_v1.035.exe".encode_utf16().collect(),
    ];
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return None;
        }
        let mut entry: ProcessEntry32W = std::mem::zeroed();
        entry.dw_size = std::mem::size_of::<ProcessEntry32W>() as DWORD;
        let mut ok = Process32FirstW(snap, &mut entry);
        while ok != 0 {
            let len = entry
                .sz_exe_file
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(MAX_PATH);
            let name = &entry.sz_exe_file[..len];
            if targets.iter().any(|t| name == t.as_slice()) {
                CloseHandle(snap);
                return Some(entry.th32_process_id);
            }
            ok = Process32NextW(snap, &mut entry);
        }
        CloseHandle(snap);
    }
    None
}

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

/// Format a recovery checkpoint's `saved_at` field for compact UI
/// display. The persisted value is an RFC 3339 ISO timestamp with
/// nanoseconds and offset (e.g. `2026-05-23T18:00:19.843268500+10:00`);
/// for a user-facing banner we want something humans can read at a
/// glance — relative if recent, absolute otherwise. Falls back to the
/// raw string if the input doesn't parse, so we never lose information.
fn format_recovery_saved_at(iso: &str) -> String {
    let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(iso) else {
        return iso.to_string();
    };
    let saved = parsed.with_timezone(&chrono::Local);
    let now = chrono::Local::now();
    let delta = now.signed_duration_since(saved);
    if delta.num_minutes() < 60 && delta.num_seconds() >= 0 {
        return format!("{}m ago", delta.num_minutes().max(1));
    }
    if saved.date_naive() == now.date_naive() {
        return format!("today {}", saved.format("%H:%M"));
    }
    if Some(saved.date_naive()) == now.date_naive().pred_opt() {
        return format!("yesterday {}", saved.format("%H:%M"));
    }
    // Avoid chrono's `%-d` (POSIX no-pad day) — unsupported on Windows
    // strftime and would render the literal `-d`. Use chrono::Datelike
    // to build the day-month string manually.
    use chrono::Datelike;
    let month = match saved.month() {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr", 5 => "May", 6 => "Jun",
        7 => "Jul", 8 => "Aug", 9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "???",
    };
    format!("{} {} {}", saved.day(), month, saved.format("%H:%M"))
}

/// Encode an `egui::ColorImage` (RGBA premultiplied, top-to-bottom) as
/// a PNG file. Used by the F8 screenshot path so an external caller
/// (an automation script, the AI agent helping debug a UX problem) can
/// read the rendered framebuffer even when the window is occluded.
fn save_color_image_as_png(
    image: &egui::ColorImage,
    path: &std::path::Path,
) -> Result<(), String> {
    let [width, height] = image.size;
    // ColorImage stores premultiplied RGBA in `Color32` (which is
    // [u8; 4]). Flatten into a byte slice for png encoding.
    let mut bytes = Vec::with_capacity(width * height * 4);
    for c in &image.pixels {
        bytes.extend_from_slice(&[c.r(), c.g(), c.b(), c.a()]);
    }
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut encoder =
        png::Encoder::new(std::io::BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer.write_image_data(&bytes).map_err(|e| e.to_string())?;
    Ok(())
}

/// Force dark title bar on Windows 10+ via DwmSetWindowAttribute.
#[cfg(windows)]
fn set_dark_title_bar(title: &str) {
    use std::ffi::c_void;
    type HWND = *mut c_void;
    type BOOL = i32;
    type DWORD = u32;
    const DWMWA_USE_IMMERSIVE_DARK_MODE: DWORD = 20;
    extern "system" {
        fn FindWindowW(class: *const u16, title: *const u16) -> HWND;
        fn DwmSetWindowAttribute(hwnd: HWND, attr: DWORD, value: *const c_void, size: DWORD)
            -> i32;
    }
    unsafe {
        let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        let hwnd = FindWindowW(std::ptr::null(), wide.as_ptr());
        if !hwnd.is_null() {
            let value: BOOL = 1;
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &value as *const BOOL as *const c_void,
                std::mem::size_of::<BOOL>() as DWORD,
            );
        }
    }
}

use panels::{config, drift, history, log_panel, timeline, trajectory, transport};
use pico::PicoState;
use recording::{RecordingHistory, RecordingSessionKind};

const DEFAULT_PLAYBACK_SPEED: f32 = 1.0;
const PLAYBACK_SPEED_PRESETS: [f32; 5] = [0.25, 0.5, 1.0, 2.0, 4.0];
const CONT_START_MATCH_MAX_RETRIES: u32 = 30;
/// Extra frames of headroom past the recording's first-moving frame before
/// we sample the live play_coords to decide if we landed in the right
/// bucket. The bucket signal is a 1-frame phase offset, so 3 frames is
/// plenty of margin to disambiguate without slowing down detection.
const CONT_BUCKET_CHECK_HEADROOM: u32 = 3;

/// Computes a varied wall-clock delay (in ms) to insert before a CONT
/// retry's Stop. The F5-restart bucket the runtime lands in is
/// determined by the wall-clock-modulo-tick-period at restart time;
/// without varying our retry timing, every retry hits the same modulo
/// and lands in the same bucket. We don't need true randomness — just
/// a sequence that cycles through enough phase offsets to cross tick
/// boundaries (~10ms at 1× tick_advance). Step is 7 because gcd(7,17)
/// is 1 so the sequence visits all 17 residues before repeating.
fn cont_retry_jitter_ms(attempt: u32) -> u64 {
    (attempt as u64 * 7 + 3) % 17 + 1
}

/// Open `tas_ui.log` in append mode next to the history JSON for this
/// session. Returns `None` if the file system isn't usable — silent
/// failure mode, since losing the on-disk mirror is preferable to
/// blocking the UI from starting.
fn open_session_log_file(history_path: &std::path::Path) -> Option<std::fs::File> {
    let dir = history_path.parent()?;
    let log_path = dir.join("tas_ui.log");
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .ok()
}

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

#[derive(Clone, Copy)]
struct ActiveRecordingSession {
    kind: RecordingSessionKind,
    start_tick: u32,
    max_recorded_count: u32,
}

#[derive(Clone, Copy)]
struct ContinueStartGuard {
    expected_start_bits: [u32; 3],
    continue_from_frame: u32,
    retries_remaining: u32,
    /// Frame index of the first position in `rec_coords` that differs from
    /// `rec_coords[0]` — i.e. when the recorded player first moved out of
    /// spawn. None if the recording never moves (degenerate / not loaded).
    /// Used as a "bucket fingerprint": the F5-restart accumulator-leftover
    /// shifts when the countdown completes by ±1 frame; comparing observed
    /// first-moving vs expected discriminates buckets long before chaos
    /// has amplified into a visible position miss.
    expected_first_moving: Option<u32>,
}

struct TasApp {
    shared: Option<TasSharedMemoryClient>,
    connect_error: Option<String>,

    // UI state
    show_config: bool,
    show_pico_panel: bool,
    pico: PicoState,
    history: RecordingHistory,
    history_store: Option<history_store::HistoryStore>,
    recovery_store: Option<recording::RecoveryStore>,
    pending_recovery: Option<recording::RecoveryCheckpoint>,
    log_lines: Vec<String>,
    /// Append-only on-disk mirror of `log_lines`. Lives under
    /// `~/.ssb-inspector/{session}/tas_ui.log` (same session dir as the
    /// history store) so AI agents and post-mortem debuggers can read
    /// the full transport-log scrollback after `log_lines` has been
    /// truncated to its 500-entry in-memory cap.
    log_file: Option<std::fs::File>,
    /// Index in `log_lines` up to which we've already flushed to
    /// `log_file`. Bumped each UI frame; resilient to the cap-drain
    /// because the drain happens AFTER we've persisted.
    log_lines_persisted: usize,
    timeline_zoom: f32,
    timeline_scroll: f32,
    continue_from_frame: u32,
    continue_from_text: String,
    playback_speed: f32,
    step_mode: bool,
    show_debug_drift: bool,
    show_history: bool,
    show_log: bool,
    segment_tracker: recording::SegmentTracker,
    active_recording_session: Option<ActiveRecordingSession>,
    pending_session_kind: Option<RecordingSessionKind>,
    pending_continue_start_tick: Option<u32>,
    last_mode: u32,
    cont_catchup_speed: Option<f32>, // saved speed to restore after CONT catch-up
    cont_catchup_multiplier: f32,    // configurable CONT catch-up speed (default 12x)
    log_read_cursor: u32,

    // Cached max drift (incremental scan instead of per-frame O(n))
    cached_max_drift_x: f32,
    cached_max_drift_z: f32,
    last_drift_scan_count: usize,
    last_logged_drift_level: u8, // 0=none, 1=any, 2=>=1.0, 3=>=5.0

    // Cached plot data (avoid per-frame Vec allocation)
    drift_cache: drift::DriftCache,
    trajectory_cache: trajectory::TrajectoryCache,

    // In-process restart state: command to send once restart completes
    pending_after_restart: Option<TasCommand>,
    /// Two-step Stop→Restart sequence: when set, we've sent CMD_STOP and
    /// are waiting for cave2 to flip mode to OFF before sending
    /// CMD_RESTART (and queueing the wrapped command for after the
    /// restart). Without this serialisation, sending Stop and Restart
    /// on the same UI frame just overwrites Stop in the shared `command`
    /// slot (single u32, no queue) — cave2 only sees Restart, mode
    /// stays in REC/PLAY, and the subsequent ArmContinue is refused.
    pending_stop_then_restart: Option<TasCommand>,
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
    continue_start_guard: Option<ContinueStartGuard>,
    // Crash recovery
    last_frame_count: u32,
    stale_frame_ticks: u32,
    last_health_check: std::time::Instant,

    // One-shot: force dark title bar on first frame
    #[cfg(windows)]
    dark_title_bar_set: bool,
    /// One-shot guard for the TAS_UI_AUTOSHOT env-triggered startup
    /// screenshot. Set true after the first request goes out.
    auto_screenshot_taken: bool,
    /// Wall-clock launch time, used to schedule the AUTOSHOT screenshot
    /// reliably (egui's `input.time` doesn't advance when nothing
    /// changes — using Instant gives a real elapsed measurement).
    launched_at: std::time::Instant,
}

impl TasApp {
    fn new() -> Self {
        let (shared, connect_error) = match TasSharedMemoryClient::open() {
            Ok(s) => (Some(s), None),
            Err(e) => (None, Some(e)),
        };

        let settings = settings::Settings::load();
        let mut history_load_notice = None;
        let pending_history = match history_store::load_latest_history() {
            Ok(history) => history,
            Err(err) => {
                history_load_notice = Some(format!("History restore disabled: {}", err));
                None
            }
        };
        let history_store = history_store::HistoryStore::new().ok();
        let history_store_notice = history_store
            .as_ref()
            .map(|store| format!("History autosave: {}", store.path().display()));
        let mut recovery_store = recording::RecoveryStore::new().ok();
        let recovery_store_notice = recovery_store
            .as_ref()
            .map(|store| format!("Crash recovery checkpoints: {}", store.root().display()));
        let mut recovery_load_notice = None;
        let pending_recovery = if let Some(store) = recovery_store.as_ref() {
            match store.load_pending() {
                Ok(checkpoint) => checkpoint,
                Err(err) => {
                    recovery_store = None;
                    recovery_load_notice = Some(format!("Crash recovery disabled: {}", err));
                    None
                }
            }
        } else {
            None
        };
        let log_file = history_store
            .as_ref()
            .and_then(|s| open_session_log_file(s.path()));
        let mut app = Self {
            shared,
            connect_error,
            show_config: settings.show_config,
            show_pico_panel: settings.show_pico_panel,
            pico: PicoState::new(),
            history: RecordingHistory::new(64),
            history_store,
            recovery_store,
            pending_recovery,
            log_lines: Vec::new(),
            log_file,
            log_lines_persisted: 0,
            timeline_zoom: 1.0,
            timeline_scroll: 0.0,
            continue_from_frame: 0,
            continue_from_text: "0".to_string(),
            playback_speed: normalize_playback_speed(settings.playback_speed),
            step_mode: false,
            show_debug_drift: settings.show_debug_drift,
            show_history: settings.show_history,
            show_log: settings.show_log,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            cont_catchup_speed: None,
            cont_catchup_multiplier: settings.cont_catchup_speed,
            log_read_cursor: 0,
            cached_max_drift_x: 0.0,
            cached_max_drift_z: 0.0,
            last_drift_scan_count: 0,
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            pending_after_restart: None,
            pending_stop_then_restart: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
            continue_start_guard: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            #[cfg(windows)]
            dark_title_bar_set: false,
            auto_screenshot_taken: false,
            launched_at: std::time::Instant::now(),
        };

        // Auto-detect Pico on startup
        let detect_logs = app.pico.auto_detect();
        for msg in detect_logs {
            let ts = chrono::Local::now().format("%H:%M:%S");
            app.log_lines.push(format!("[{}] {}", ts, msg));
        }
        // Pico auto-detected but panel hidden by default (use View menu to show)
        let _ = app.pico.auto_detected;
        if let Some(loaded) = pending_history {
            let source = loaded.path.display().to_string();
            match app.history.apply_persisted(loaded.history) {
                Ok(()) => app.push_log(&format!("Recovered history from {}", source)),
                Err(err) => app.push_log(&format!("History restore skipped: {}", err)),
            }
        }
        if let Some(msg) = history_store_notice {
            app.push_log(&msg);
        }
        if let Some(msg) = history_load_notice {
            app.push_log(&msg);
        }
        if let Some(msg) = recovery_store_notice {
            app.push_log(&msg);
        }
        if let Some(msg) = recovery_load_notice {
            app.push_log(&msg);
        }
        if let Some(recovery) = app.pending_recovery.as_ref() {
            app.push_log(&format!(
                "Recovery available: {} (saved {})",
                recovery.label(),
                recovery.saved_at
            ));
        }
        app.persist_history_if_needed();

        app
    }

    fn playback_speed_for_settings(&self) -> f32 {
        let base_speed = self.cont_catchup_speed.unwrap_or(self.playback_speed);
        normalize_playback_speed(base_speed)
    }

    fn try_reconnect(&mut self) {
        match TasSharedMemoryClient::open() {
            Ok(s) => {
                self.shared = Some(s);
                self.connect_error = None;
                self.push_log("Connected to TAS_Helper.dll shared memory");
            }
            Err(e) => self.connect_error = Some(e),
        }
    }

    fn push_log(&mut self, msg: &str) {
        let ts = chrono::Local::now().format("%H:%M:%S");
        self.log_lines.push(format!("[{}] {}", ts, msg));
        if self.log_lines.len() > 500 {
            // Drain the oldest 100 entries. The on-disk log already has
            // them (flush_log_lines_to_file is called before any drain
            // could be hit on the same frame), so we only need to keep
            // the persisted-cursor coherent.
            let drained = 100;
            self.log_lines.drain(..drained);
            self.log_lines_persisted = self.log_lines_persisted.saturating_sub(drained);
        }
    }

    /// Append any newly-pushed log lines to the session log file. Called
    /// once per UI frame from `update`. If the file is unavailable
    /// (couldn't open at start) this is a no-op — the in-memory log
    /// remains the only record.
    fn flush_log_lines_to_file(&mut self) {
        use std::io::Write;
        let Some(file) = self.log_file.as_mut() else {
            return;
        };
        if self.log_lines_persisted >= self.log_lines.len() {
            return;
        }
        for line in &self.log_lines[self.log_lines_persisted..] {
            // Best-effort: a single write failure shouldn't crash the
            // UI. The on-disk log may end up missing entries but that's
            // strictly better than panicking.
            let _ = writeln!(file, "{}", line);
        }
        let _ = file.flush();
        self.log_lines_persisted = self.log_lines.len();
    }

    #[cfg(test)]
    fn begin_cont_catchup(&mut self) {
        if self.cont_catchup_speed.is_none() {
            self.cont_catchup_speed = Some(self.playback_speed);
        }
        self.playback_speed = self.cont_catchup_multiplier;
    }

    fn clear_cont_catchup(&mut self) {
        if let Some(saved) = self.cont_catchup_speed.take() {
            self.playback_speed = saved;
        }
    }

    fn reset_continue_runtime_state(&mut self) {
        self.pending_session_kind = None;
        self.pending_continue_start_tick = None;
        self.continue_start_guard = None;
    }

    fn set_continue_start_guard(&mut self) {
        let expected = self.shared.as_ref().and_then(|shared| {
            let state = shared.state();
            if state.recorded_count == 0 {
                return None;
            }
            let rec0 = state.rec_coords[0];
            let start_bits = [rec0[0].to_bits(), rec0[1].to_bits(), rec0[2].to_bits()];
            let first_moving =
                recording::detect_first_moving(&state.rec_coords, state.recorded_count);
            Some((start_bits, first_moving))
        });
        self.continue_start_guard = expected.map(|(expected_start_bits, expected_first_moving)| {
            ContinueStartGuard {
                expected_start_bits,
                continue_from_frame: self.continue_from_frame,
                retries_remaining: CONT_START_MATCH_MAX_RETRIES,
                expected_first_moving,
            }
        });
    }

    fn send_action_command(&mut self, command: TasCommand, ts: &str) {
        if command == TasCommand::Stop {
            self.clear_cont_catchup();
            self.reset_continue_runtime_state();
            // Cancel any queued restart sequence. Without this, pressing
            // STOP after a RestartThen (CONT/PLAY/REC click but before
            // the polling loop fires the wrapped command) would leave
            // pending_after_restart / pending_stop_then_restart set —
            // the next poll then "completes" the restart and silently
            // starts recording/playback. STOP must mean STOP.
            self.pending_after_restart = None;
            self.pending_stop_then_restart = None;
        }
        if let Some(shared) = self.shared.as_mut() {
            shared.send_command(command);
        }
        self.log_lines.push(format!("[{}] Sent: {:?}", ts, command));
    }

    fn queue_restart_then(&mut self, command: TasCommand, ts: &str) {
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
                self.log_lines.push(format!(
                    "[{}] CONT ignored: no recording loaded (recorded_count=0)",
                    ts
                ));
                return;
            }
            if self.continue_from_frame == 0 {
                self.log_lines.push(format!(
                    "[{}] CONT ignored: continue_from_frame=0 — press PLAY instead",
                    ts
                ));
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
                    "[{}] CONT ignored: continue_from_frame={} > recorded_count={}",
                    ts, self.continue_from_frame, recorded
                ));
                return;
            }
        }
        if command == TasCommand::ArmContinue {
            if self.cont_catchup_speed.is_none() {
                self.cont_catchup_speed = Some(self.playback_speed);
            }
            self.playback_speed = self.cont_catchup_multiplier;
            self.pending_session_kind = Some(RecordingSessionKind::Continue);
            self.pending_continue_start_tick = Some(self.continue_from_frame);
            self.set_continue_start_guard();
        } else {
            self.clear_cont_catchup();
            self.pending_session_kind = if command == TasCommand::ArmRec {
                Some(RecordingSessionKind::Rec)
            } else {
                None
            };
            self.pending_continue_start_tick = None;
            self.continue_start_guard = None;
        }

        if command == TasCommand::ArmRec {
            self.playback_speed = DEFAULT_PLAYBACK_SPEED;
        }

        // PLAY always starts from tick 0 — reset continue_from_frame
        if command == TasCommand::ArmPlay {
            self.continue_from_frame = 0;
            self.continue_from_text = "0".to_string();
        }

        if let Some(shared) = self.shared.as_mut() {
            shared.state_mut().playback_speed = self.playback_speed;
            if command == TasCommand::ArmPlay {
                shared.state_mut().continue_from_frame = 0;
            }
            // Cave2's ARM_CONTINUE handler refuses if the game is currently
            // in REC or PLAY. We can't send Stop + Restart on the same
            // frame: the shared `command` slot is a single u32 — the
            // second write clobbers the first, so cave2 only sees Restart
            // and never the Stop. Instead, send Stop now and stash the
            // wrapped command in pending_stop_then_restart; the per-frame
            // poll fires Restart once cave2 has flipped mode to OFF.
            if shared.mode_volatile() != TasMode::Off as u32 {
                shared.send_command(TasCommand::Stop);
                self.pending_stop_then_restart = Some(command);
                self.log_lines.push(format!(
                    "[{}] In-process Stop → wait for OFF → Restart → {:?}",
                    ts, command
                ));
                return;
            }
            shared.reset_restart_state();
            shared.send_command(TasCommand::Restart);
        }
        self.pending_after_restart = Some(command);
        self.log_lines
            .push(format!("[{}] In-process F5 restart → {:?}", ts, command));
    }

    /// Poll the deferred Stop→Restart sequence. Once cave2 has processed
    /// our earlier CMD_STOP (mode == OFF), send the CMD_RESTART and let
    /// the existing pending_after_restart machinery take over.
    fn poll_pending_stop_then_restart(&mut self, ctx: &egui::Context) {
        let Some(command) = self.pending_stop_then_restart else {
            return;
        };
        let Some(shared) = self.shared.as_mut() else {
            self.pending_stop_then_restart = None;
            return;
        };
        if shared.mode_volatile() != TasMode::Off as u32 {
            ctx.request_repaint();
            return;
        }
        shared.state_mut().playback_speed = self.playback_speed;
        shared.reset_restart_state();
        shared.send_command(TasCommand::Restart);
        self.pending_stop_then_restart = None;
        self.pending_after_restart = Some(command);
        let ts = chrono::Local::now().format("%H:%M:%S");
        self.log_lines.push(format!(
            "[{}] Stop landed (mode=OFF) → Restart → {:?}",
            ts, command
        ));
        ctx.request_repaint();
    }

    fn poll_continue_start_guard(&mut self, ctx: &egui::Context) {
        let Some(mut guard) = self.continue_start_guard else {
            return;
        };
        // If a restart sequence is in flight (Stop sent + waiting for OFF,
        // or Restart sent + ArmContinue pending), skip bucket judgment
        // until the cycle completes — otherwise we'd re-judge stale
        // play_coords from the PREVIOUS run before cave2 has had a chance
        // to actually restart.
        if self.pending_after_restart.is_some() || self.pending_stop_then_restart.is_some() {
            ctx.request_repaint();
            return;
        }
        let Some(shared) = self.shared.as_ref() else {
            return;
        };

        let mode = shared.mode_volatile();
        if mode == TasMode::Rec as u32 {
            self.continue_start_guard = None;
            return;
        }
        if mode != TasMode::Play as u32 {
            return;
        }

        let playback_pos = shared.playback_pos_volatile();
        if playback_pos == 0 {
            ctx.request_repaint();
            return;
        }

        let play0 = shared.state().play_coords[0];
        let play0_bits = [play0[0].to_bits(), play0[1].to_bits(), play0[2].to_bits()];
        let start_matches = play0_bits == guard.expected_start_bits;

        // Phase 2: bucket fingerprint check. Only meaningful once the
        // start position matches (otherwise we'd be running the bucket
        // check on a degenerate run that's about to retry anyway). Needs
        // enough frames played past the recording's expected first-moving
        // frame to distinguish the two buckets.
        if start_matches {
            let expected_first_moving = match guard.expected_first_moving {
                Some(f) => f,
                None => {
                    // Recording never moves out of spawn (or wasn't
                    // loaded with positions). No bucket signal — done.
                    if guard.retries_remaining < CONT_START_MATCH_MAX_RETRIES {
                        let used = CONT_START_MATCH_MAX_RETRIES - guard.retries_remaining;
                        self.push_log(&format!(
                            "CONT start aligned after {} restart retr{}",
                            used,
                            if used == 1 { "y" } else { "ies" }
                        ));
                    }
                    self.continue_start_guard = None;
                    return;
                }
            };
            let needed = expected_first_moving + CONT_BUCKET_CHECK_HEADROOM;
            if playback_pos < needed {
                ctx.request_repaint();
                return;
            }
            // Compute the live first-moving frame from play_coords.
            let state = shared.state();
            let spawn_bits = guard.expected_start_bits;
            let scan_end = (playback_pos as usize).min(state.play_coords.len());
            let mut observed: Option<u32> = None;
            for j in 1..scan_end {
                let c = state.play_coords[j];
                if c[0].to_bits() != spawn_bits[0]
                    || c[1].to_bits() != spawn_bits[1]
                    || c[2].to_bits() != spawn_bits[2]
                {
                    observed = Some(j as u32);
                    break;
                }
            }
            if observed == Some(expected_first_moving) {
                // Bucket match — full success.
                let used = CONT_START_MATCH_MAX_RETRIES - guard.retries_remaining;
                if used > 0 {
                    self.push_log(&format!(
                        "CONT bucket aligned after {} restart retr{}",
                        used,
                        if used == 1 { "y" } else { "ies" }
                    ));
                }
                self.continue_start_guard = None;
                return;
            }
            // Wrong bucket — fall through to the retry path with a
            // bucket-specific log message.
            if guard.retries_remaining == 0 {
                if let Some(shared) = self.shared.as_mut() {
                    shared.send_command(TasCommand::Stop);
                }
                self.clear_cont_catchup();
                self.pending_after_restart = None;
                self.reset_continue_runtime_state();
                self.push_log(&format!(
                    "CONT aborted: bucket mismatch after {} retries (observed first-moving={:?}, expected={})",
                    CONT_START_MATCH_MAX_RETRIES, observed, expected_first_moving
                ));
                return;
            }
            let continue_from_frame = guard.continue_from_frame;
            guard.retries_remaining -= 1;
            let attempt = CONT_START_MATCH_MAX_RETRIES - guard.retries_remaining;
            self.pending_session_kind = Some(RecordingSessionKind::Continue);
            self.pending_continue_start_tick = Some(continue_from_frame);
            self.playback_speed = self.cont_catchup_multiplier;
            if self.cont_catchup_speed.is_none() {
                self.cont_catchup_speed = Some(DEFAULT_PLAYBACK_SPEED);
            }
            // Explicit timing jitter: vary the wall-clock offset before
            // each retry's Stop, so the F5 lands at a different
            // accumulator-modulo-tick-period than the previous attempt.
            // Without this the natural mode-wait jitter is too small/
            // consistent and we just keep hitting the same bucket. See
            // cont_retry_jitter_ms — sequence cycles through 17 distinct
            // offsets, plenty to cross tick boundaries.
            let jitter = cont_retry_jitter_ms(attempt);
            std::thread::sleep(std::time::Duration::from_millis(jitter));
            // Use the two-step Stop→Restart serialisation: send Stop,
            // let poll_pending_stop_then_restart fire Restart once cave2
            // has confirmed mode==OFF. The shared `command` slot is a
            // single u32, so sending Stop+Restart same-frame just loses
            // the Stop.
            if let Some(shared) = self.shared.as_mut() {
                shared.state_mut().continue_from_frame = continue_from_frame;
                shared.state_mut().playback_speed = self.playback_speed;
                shared.send_command(TasCommand::Stop);
            }
            self.pending_stop_then_restart = Some(TasCommand::ArmContinue);
            self.continue_start_guard = Some(guard);
            self.push_log(&format!(
                "CONT bucket mismatch (observed first-moving={:?}, expected={}) -> retry {}/{}",
                observed, expected_first_moving, attempt, CONT_START_MATCH_MAX_RETRIES
            ));
            ctx.request_repaint();
            return;
        }

        // Phase 1: start-position mismatch path (existing behavior).
        let expected_x = f32::from_bits(guard.expected_start_bits[0]);
        let expected_z = f32::from_bits(guard.expected_start_bits[2]);
        let dx = (play0[0] as f64 - expected_x as f64).abs();
        let dz = (play0[2] as f64 - expected_z as f64).abs();

        if guard.retries_remaining == 0 {
            if let Some(shared) = self.shared.as_mut() {
                shared.send_command(TasCommand::Stop);
            }
            self.clear_cont_catchup();
            self.pending_after_restart = None;
            self.reset_continue_runtime_state();
            self.push_log(&format!(
                "CONT aborted: start mismatch after {} retries (dx={:.9}, dz={:.9})",
                CONT_START_MATCH_MAX_RETRIES, dx, dz
            ));
            return;
        }

        let continue_from_frame = guard.continue_from_frame;
        guard.retries_remaining -= 1;
        let attempt = CONT_START_MATCH_MAX_RETRIES - guard.retries_remaining;
        self.pending_session_kind = Some(RecordingSessionKind::Continue);
        self.pending_continue_start_tick = Some(continue_from_frame);
        self.playback_speed = self.cont_catchup_multiplier;
        if self.cont_catchup_speed.is_none() {
            self.cont_catchup_speed = Some(DEFAULT_PLAYBACK_SPEED);
        }
        // Vary wall-clock timing before retry (see bucket-retry path).
        let jitter = cont_retry_jitter_ms(attempt);
        std::thread::sleep(std::time::Duration::from_millis(jitter));
        // Use two-step Stop→Restart so cave2 actually sees the Stop.
        if let Some(shared) = self.shared.as_mut() {
            shared.state_mut().continue_from_frame = continue_from_frame;
            shared.state_mut().playback_speed = self.playback_speed;
            shared.send_command(TasCommand::Stop);
        }
        self.pending_stop_then_restart = Some(TasCommand::ArmContinue);
        self.continue_start_guard = Some(guard);
        self.push_log(&format!(
            "CONT start mismatch (dx={:.9}, dz={:.9}) -> retry {}/{}",
            dx, dz, attempt, CONT_START_MATCH_MAX_RETRIES
        ));
        ctx.request_repaint();
    }

    #[cfg(test)]
    fn prepare_send_action(&mut self, command: TasCommand) {
        if command == TasCommand::Stop {
            self.clear_cont_catchup();
            self.reset_continue_runtime_state();
            self.pending_after_restart = None;
            self.pending_stop_then_restart = None;
        }
    }

    #[cfg(test)]
    fn prepare_restart_action(&mut self, command: TasCommand) {
        if command == TasCommand::ArmContinue {
            self.begin_cont_catchup();
        } else {
            self.clear_cont_catchup();
        }
        if command == TasCommand::ArmRec {
            self.playback_speed = DEFAULT_PLAYBACK_SPEED;
        }
        match command {
            TasCommand::ArmRec => {
                self.pending_session_kind = Some(RecordingSessionKind::Rec);
                self.pending_continue_start_tick = None;
            }
            TasCommand::ArmContinue => {
                self.pending_session_kind = Some(RecordingSessionKind::Continue);
                self.pending_continue_start_tick = Some(self.continue_from_frame);
            }
            _ => {
                self.pending_session_kind = None;
                self.pending_continue_start_tick = None;
            }
        }
        if command != TasCommand::ArmContinue {
            self.continue_start_guard = None;
        }
    }

    fn persist_history_if_needed(&mut self) {
        let persist_result = match self.history_store.as_mut() {
            Some(store) => Some(store.persist_if_changed(&self.history)),
            None => None,
        };

        if let Some(Err(err)) = persist_result {
            self.history_store = None;
            self.push_log(&format!("History autosave disabled: {}", err));
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
        let persist_result = match self.recovery_store.as_mut() {
            Some(store) => Some(store.persist_snapshot_if_needed(
                snapshot,
                &self.segment_tracker.segments,
                session,
                force,
            )),
            None => None,
        };

        if let Some(Err(err)) = persist_result {
            self.recovery_store = None;
            self.push_log(&format!("Crash recovery disabled: {}", err));
        }
    }

    fn update_recording_recovery_progress(&mut self, snapshot: &recording::RecordingSnapshot) {
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

        let _ = self.history.push_snapshot_data_with_session(
            snapshot.clone(),
            session_context.label.clone(),
            session_context.start_tick,
            session_context.end_tick,
        );
        self.persist_recovery_snapshot_if_needed(snapshot, &session_context, true);
    }

    fn restore_pending_recovery(&mut self) {
        let Some(recovery) = self.pending_recovery.take() else {
            return;
        };

        if let Some(ref mut shared) = self.shared {
            let recovery_label = recovery.label().to_string();
            let session_start = recovery.session.start_tick;
            let session_end = recovery.session.end_tick;
            recovery.snapshot.restore_to(shared.state_mut());
            self.segment_tracker.restore_from(recovery.segments);
            self.continue_from_frame = shared.state().recorded_count;
            self.continue_from_text = self.continue_from_frame.to_string();
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            let _ = self.history.push_snapshot_data_with_session(
                snapshot,
                recovery_label.clone(),
                session_start,
                session_end,
            );
            self.push_log(&format!("Restored crash recovery: {}", recovery_label));

            if let Some(store) = self.recovery_store.as_mut() {
                if let Err(err) = store.clear_pending() {
                    self.recovery_store = None;
                    self.push_log(&format!("Crash recovery disabled: {}", err));
                }
            }
        } else {
            self.push_log("Crash recovery restore requires an active game connection.");
            self.pending_recovery = Some(recovery);
        }
    }

    fn discard_pending_recovery(&mut self) {
        self.pending_recovery = None;
        if let Some(store) = self.recovery_store.as_mut() {
            if let Err(err) = store.clear_pending() {
                self.recovery_store = None;
                self.push_log(&format!("Crash recovery disabled: {}", err));
                return;
            }
        }
        self.push_log("Discarded pending crash recovery checkpoint.");
    }

    /// Check if the game process is still alive by monitoring frame_count advancement.
    /// If frame_count hasn't changed for ~3 seconds, assume the game crashed.
    fn check_game_health(&mut self) {
        if self.last_health_check.elapsed() < std::time::Duration::from_secs(1) {
            return;
        }
        self.last_health_check = std::time::Instant::now();

        if let Some(ref shared) = self.shared {
            let current_frame = shared.frame_count_volatile();
            if current_frame == self.last_frame_count {
                self.stale_frame_ticks += 1;
                if self.stale_frame_ticks == 5 {
                    // Check if Supreme.exe is actually running
                    if !is_supreme_running() {
                        self.push_log("Game process not found — disconnecting shared memory");
                        self.shared = None;
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
    #[cfg(windows)]
    fn poll_global_shortcuts(&mut self) -> Vec<transport::Action> {
        use std::ffi::c_void;
        type HWND = *mut c_void;
        type DWORD = u32;
        const VK_F9: i32 = 0x78;
        const VK_F10: i32 = 0x79;
        const VK_F11: i32 = 0x7A;
        const VK_F12: i32 = 0x7B;

        extern "system" {
            fn GetAsyncKeyState(vk: i32) -> i16;
            fn GetForegroundWindow() -> HWND;
            fn GetWindowThreadProcessId(hwnd: HWND, pid: *mut DWORD) -> DWORD;
        }

        // Step 1: read current pressed state for all four keys.
        let now: [bool; 4] = [VK_F9, VK_F10, VK_F11, VK_F12].map(|vk| unsafe {
            (GetAsyncKeyState(vk) as u16 & 0x8000) != 0
        });
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
            let resolved = find_supreme_pid();
            self.game_pid_cached = resolved;
            resolved
        });
        let Some(game_pid) = game_pid else { return Vec::new() };
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_null() {
            return Vec::new();
        }
        let mut fg_pid: DWORD = 0;
        unsafe { GetWindowThreadProcessId(hwnd, &mut fg_pid) };
        if fg_pid != game_pid {
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
            actions.push(transport::Action::SetContinueFrame(self.continue_from_frame));
            actions.push(transport::Action::RestartThen(TasCommand::ArmContinue));
            actions.push(transport::Action::Log("Global F12 (in-game): CONT".into()));
        }
        actions
    }

    /// Non-Windows shim so the call site doesn't need conditional compilation.
    #[cfg(not(windows))]
    fn poll_global_shortcuts(&mut self) -> Vec<transport::Action> {
        Vec::new()
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

            // F8: capture a screenshot of the egui framebuffer to disk.
            // Used for remote debugging — Windows GDI APIs can't read
            // wgpu-rendered windows when they're occluded by another
            // window, but egui's own screenshot mechanism runs through
            // the GPU pipeline and grabs the actual rendered content.
            // The result is written to a fixed path; the caller (e.g.
            // an automation script) reads the file after a frame or two.
            if input.key_pressed(egui::Key::F8) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                actions.push(transport::Action::Log("F8: screenshot requested".into()));
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

            // Period: Step forward one frame
            if input.key_pressed(egui::Key::Period) {
                actions.push(transport::Action::StepOne);
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
            self.timeline_zoom = (self.timeline_zoom * 1.25).min(10.0);
        }
        if zoom_out {
            self.timeline_zoom = (self.timeline_zoom / 1.25).max(0.1);
        }
        if save {
            if let Some(ref shared) = self.shared {
                if let Some(path) = recording::save_dialog_with_segments(
                    shared.state(),
                    &self.segment_tracker.segments,
                    &mut self.log_lines,
                ) {
                    self.history.push_save_marker(shared.state(), &path);
                }
            }
        }
        if open {
            if let Some(ref mut shared) = self.shared {
                if let Some(path) = recording::load_dialog(
                    shared.state_mut(),
                    &mut self.segment_tracker,
                    &mut self.log_lines,
                ) {
                    let _ = self.history.push_loaded_snapshot(shared.state(), &path);
                    // Auto-play after loading a recording
                    if shared.state().recorded_count > 0 {
                        let ts = chrono::Local::now().format("%H:%M:%S").to_string();
                        self.queue_restart_then(TasCommand::ArmPlay, &ts);
                    }
                }
            }
        }

        actions
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
            show_config: self.show_config,
            show_log: self.show_log,
            playback_speed: self.playback_speed_for_settings(),
            cont_catchup_speed: self.cont_catchup_multiplier,
        };
        s.save();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Force dark theme + title bar on Windows (one-shot, first frame)
        #[cfg(windows)]
        if !self.dark_title_bar_set {
            self.dark_title_bar_set = true;
            ctx.set_theme(egui::Theme::Dark);
            set_dark_title_bar("SSB Inspect");
        }

        // Persist any new log lines added since last frame to the on-disk
        // session log. Done first so a panic later in the frame still
        // captures the events that led up to it.
        self.flush_log_lines_to_file();

        // If a screenshot was requested last frame (via F8), the encoded
        // ColorImage arrives in this frame's raw events. Walk them and
        // write any screenshots to disk. egui's screenshot path goes
        // through the GPU pipeline, so it works even when the window
        // is occluded by another window (unlike GDI PrintWindow which
        // returns black for wgpu-rendered windows).
        let screenshots: Vec<std::sync::Arc<egui::ColorImage>> = ctx.input(|i| {
            i.raw
                .events
                .iter()
                .filter_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
                .collect()
        });
        for image in screenshots {
            let path = std::env::temp_dir()
                .join("tas_ui_screenshot.png");
            match save_color_image_as_png(&image, &path) {
                Ok(()) => self.push_log(&format!("Screenshot written to {}", path.display())),
                Err(err) => self.push_log(&format!("Screenshot save failed: {}", err)),
            }
        }

        // Two-step Stop→Restart: fire the deferred Restart once cave2
        // has confirmed mode==OFF.
        self.poll_pending_stop_then_restart(ctx);

        // Check game health (crash detection)
        self.check_game_health();

        // Track mode transitions for segment history + recovery checkpoints.
        let mut mode_snapshot: Option<(u32, u32, u32, recording::RecordingSnapshot)> = None;
        if let Some(ref shared) = self.shared {
            mode_snapshot = Some((
                shared.mode_volatile(),
                shared.recorded_count_volatile(),
                shared.state().continue_from_frame,
                recording::RecordingSnapshot::from_state(shared.state()),
            ));
        }
        if let Some((current_mode, recorded, continue_from, state_snapshot)) = mode_snapshot {
            if current_mode != self.last_mode {
                // REC started
                if current_mode == 1 {
                    self.start_recording_session(continue_from, recorded);
                    self.clear_cont_catchup();
                    self.continue_start_guard = None;
                }
                // REC stopped (mode went from REC to OFF)
                if self.last_mode == 1 && current_mode == 0 {
                    self.segment_tracker.on_rec_stop(recorded);
                    self.finalize_recording_session(&state_snapshot, recorded);
                }
                self.last_mode = current_mode;
            }
            if current_mode == 1 {
                self.update_recording_recovery_progress(&state_snapshot);
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
        let mut restore_pending_recovery = false;
        let mut discard_pending_recovery = false;

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save Recording...  Ctrl+S").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            if let Some(path) = recording::save_dialog_with_segments(
                                shared.state(),
                                &self.segment_tracker.segments,
                                &mut self.log_lines,
                            ) {
                                self.history.push_save_marker(shared.state(), &path);
                            }
                        }
                    }
                    if ui.button("Load Recording...  Ctrl+O").clicked() {
                        ui.close_menu();
                        if let Some(ref mut shared) = self.shared {
                            if let Some(path) = recording::load_dialog(
                                shared.state_mut(),
                                &mut self.segment_tracker,
                                &mut self.log_lines,
                            ) {
                                let _ = self.history.push_loaded_snapshot(shared.state(), &path);
                                // Auto-play after loading a recording
                                if shared.state().recorded_count > 0 {
                                    let ts = chrono::Local::now().format("%H:%M:%S").to_string();
                                    self.queue_restart_then(TasCommand::ArmPlay, &ts);
                                }
                            }
                        }
                    }
                    if let Some(recovery) = self.pending_recovery.as_ref() {
                        ui.separator();
                        ui.label(
                            egui::RichText::new(format!("Crash recovery: {}", recovery.label()))
                                .small(),
                        );
                        if ui.button("Restore Crash Recovery").clicked() {
                            ui.close_menu();
                            restore_pending_recovery = true;
                        }
                        if ui.button("Discard Crash Recovery").clicked() {
                            ui.close_menu();
                            discard_pending_recovery = true;
                        }
                    }
                    ui.separator();
                    if ui.button("Dump Diagnostics...").clicked() {
                        ui.close_menu();
                        if let Some(ref shared) = self.shared {
                            recording::dump_diagnostics(
                                shared.state(),
                                (self.cached_max_drift_x, self.cached_max_drift_z),
                                &mut self.log_lines,
                            );
                        }
                    }
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
                    ui.checkbox(&mut self.show_debug_drift, "Drift overlay");
                    ui.checkbox(&mut self.show_pico_panel, "Pico HID");
                    ui.checkbox(&mut self.show_config, "Debug Config");
                });
            });
        });

        if restore_pending_recovery {
            self.restore_pending_recovery();
            restore_pending_recovery = false;
        }
        if discard_pending_recovery {
            self.discard_pending_recovery();
            discard_pending_recovery = false;
        }

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
                });
        } // left_panel_visible

        // Right-side history panel (optional)
        let mut history_actions = Vec::new();
        let history_dir = self
            .history_store
            .as_ref()
            .and_then(|store| store.path().parent().map(|path| path.to_path_buf()));
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
                        if let Some(store) = self.history_store.as_ref() {
                            title.on_hover_text(format!(
                                "Autosave: {}",
                                store.path().display()
                            ));
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

                    // Compact recovery banner — single-row hint + two
                    // small buttons. Detail goes to a hover-tooltip
                    // instead of taking a whole row.
                    if let Some(recovery) = self.pending_recovery.as_ref() {
                        let saved_label = format_recovery_saved_at(&recovery.saved_at);
                        ui.add_space(4.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_unmultiplied(245, 196, 84, 32))
                            .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                            .rounding(3.0)
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "⚠ Recovery from {}",
                                            saved_label,
                                        ))
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(245, 196, 84)),
                                    )
                                    .on_hover_text(recovery.label().to_string());
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.small_button("Discard").clicked() {
                                                discard_pending_recovery = true;
                                            }
                                            if ui.small_button("Restore").clicked() {
                                                restore_pending_recovery = true;
                                            }
                                        },
                                    );
                                });
                            });
                    }

                    ui.separator();
                    history_actions = history::show(ui, &self.history);
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

        if restore_pending_recovery {
            self.restore_pending_recovery();
        }
        if discard_pending_recovery {
            self.discard_pending_recovery();
        }

        // Process history panel restores
        if !history_actions.is_empty() {
            if let Some(ref mut shared) = self.shared {
                let ts = chrono::Local::now().format("%H:%M:%S");
                for action in history_actions {
                    match action {
                        history::HistoryAction::Restore(idx) => {
                            if let Some(snap) = self.history.restore_index(idx) {
                                snap.restore_to(shared.state_mut());
                                let label = self
                                    .history
                                    .entries()
                                    .get(idx)
                                    .map(|entry| entry.label.clone())
                                    .unwrap_or_else(|| format!("Entry {}", idx + 1));
                                self.log_lines
                                    .push(format!("[{}] History restore: {}", ts, label));
                            }
                        }
                    }
                }
            }
        }

        // Poll in-process restart state machine
        if let Some(pending_cmd) = self.pending_after_restart {
            let mut restart_done = false;
            if let Some(shared) = self.shared.as_mut() {
                let rs = shared.restart_state();
                if rs == 2 {
                    // Restart complete — send the pending command
                    shared.reset_restart_state();
                    shared.send_command(pending_cmd);
                    restart_done = true;
                }
                // Keep polling even with no user input.
                ctx.request_repaint();
            }
            if restart_done {
                let ts = chrono::Local::now().format("%H:%M:%S");
                self.log_lines
                    .push(format!("[{}] Restart done, sent: {:?}", ts, pending_cmd));
                self.pending_after_restart = None;
            }
        }
        self.poll_continue_start_guard(ctx);

        // Apply shortcut actions to shared state
        for cmd in shortcut_actions {
            let ts = chrono::Local::now().format("%H:%M:%S").to_string();
            match cmd {
                transport::Action::Send(c) => self.send_action_command(c, &ts),
                transport::Action::RestartThen(c) => self.queue_restart_then(c, &ts),
                transport::Action::Undo => {
                    if let Some(snap) = self.history.undo() {
                        if let Some(shared) = self.shared.as_mut() {
                            snap.restore_to(shared.state_mut());
                        }
                        self.log_lines
                            .push(format!("[{}] Undo: restored previous recording", ts));
                    }
                }
                transport::Action::Redo => {
                    if let Some(snap) = self.history.redo() {
                        if let Some(shared) = self.shared.as_mut() {
                            snap.restore_to(shared.state_mut());
                        }
                        self.log_lines
                            .push(format!("[{}] Redo: restored next recording", ts));
                    }
                }
                transport::Action::SetContinueFrame(frame) => {
                    self.continue_from_frame = frame;
                    self.continue_from_text = frame.to_string();
                    if let Some(shared) = self.shared.as_mut() {
                        shared.state_mut().continue_from_frame = frame;
                    }
                }
                transport::Action::StepOne => {
                    self.log_lines
                        .push(format!("[{}] Step one frame (requires DLL support)", ts));
                }
                transport::Action::Log(msg) => {
                    self.log_lines.push(format!("[{}] {}", ts, msg));
                }
            }
        }

        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Transport bar at top
            if let Some(ref mut shared) = self.shared {
                let mode = shared.state().mode_enum();
                let recorded = shared.state().recorded_count;

                let cmds = transport::show(
                    ui,
                    mode,
                    recorded,
                    &mut self.continue_from_frame,
                    &mut self.continue_from_text,
                    &mut self.playback_speed,
                    &mut self.cont_catchup_multiplier,
                    &mut self.step_mode,
                    &self.history,
                    shared.state(),
                    self.cont_catchup_speed.is_some(),
                );

                // Process actions - use log_lines directly to avoid borrow conflicts
                for cmd in cmds {
                    let ts = chrono::Local::now().format("%H:%M:%S");
                    match cmd {
                        transport::Action::Send(c) => {
                            if c == TasCommand::Stop {
                                if let Some(saved) = self.cont_catchup_speed.take() {
                                    self.playback_speed = saved;
                                }
                                self.pending_session_kind = None;
                                self.pending_continue_start_tick = None;
                                self.continue_start_guard = None;
                                // Cancel queued restart sequences so STOP
                                // actually stops (see send_action_command
                                // for the full rationale).
                                self.pending_after_restart = None;
                                self.pending_stop_then_restart = None;
                            }
                            shared.send_command(c);
                            self.log_lines.push(format!("[{}] Sent: {:?}", ts, c));
                        }
                        transport::Action::RestartThen(c) => {
                            // Refuse degenerate CONT requests — see
                            // queue_restart_then for the full rationale.
                            if c == TasCommand::ArmContinue {
                                if shared.state().recorded_count == 0 {
                                    self.log_lines.push(format!(
                                        "[{}] CONT ignored: no recording loaded (recorded_count=0)",
                                        ts
                                    ));
                                    continue;
                                }
                                if self.continue_from_frame == 0 {
                                    self.log_lines.push(format!(
                                        "[{}] CONT ignored: continue_from_frame=0 — press PLAY instead",
                                        ts
                                    ));
                                    continue;
                                }
                                let recorded = shared.state().recorded_count;
                                if self.continue_from_frame > recorded {
                                    self.log_lines.push(format!(
                                        "[{}] CONT ignored: continue_from_frame={} > recorded_count={}",
                                        ts, self.continue_from_frame, recorded
                                    ));
                                    continue;
                                }
                            }
                            if c == TasCommand::ArmContinue {
                                if self.cont_catchup_speed.is_none() {
                                    self.cont_catchup_speed = Some(self.playback_speed);
                                }
                                self.playback_speed = self.cont_catchup_multiplier;
                                self.pending_session_kind = Some(RecordingSessionKind::Continue);
                                self.pending_continue_start_tick = Some(self.continue_from_frame);
                                let state = shared.state();
                                if state.recorded_count > 0 {
                                    let rec0 = state.rec_coords[0];
                                    let n = (state.recorded_count as usize).min(state.rec_coords.len());
                                    let mut first_moving: Option<u32> = None;
                                    for j in 1..n {
                                        let c = state.rec_coords[j];
                                        if c[0].to_bits() != rec0[0].to_bits()
                                            || c[1].to_bits() != rec0[1].to_bits()
                                            || c[2].to_bits() != rec0[2].to_bits()
                                        {
                                            first_moving = Some(j as u32);
                                            break;
                                        }
                                    }
                                    self.continue_start_guard = Some(ContinueStartGuard {
                                        expected_start_bits: [
                                            rec0[0].to_bits(),
                                            rec0[1].to_bits(),
                                            rec0[2].to_bits(),
                                        ],
                                        continue_from_frame: self.continue_from_frame,
                                        retries_remaining: CONT_START_MATCH_MAX_RETRIES,
                                        expected_first_moving: first_moving,
                                    });
                                } else {
                                    self.continue_start_guard = None;
                                }
                            } else {
                                if let Some(saved) = self.cont_catchup_speed.take() {
                                    self.playback_speed = saved;
                                }
                                self.pending_session_kind = if c == TasCommand::ArmRec {
                                    Some(RecordingSessionKind::Rec)
                                } else {
                                    None
                                };
                                self.pending_continue_start_tick = None;
                                self.continue_start_guard = None;
                            }
                            if c == TasCommand::ArmRec {
                                self.playback_speed = DEFAULT_PLAYBACK_SPEED;
                            }
                            shared.state_mut().playback_speed = self.playback_speed;
                            // See queue_restart_then for full rationale: the
                            // shared `command` slot is a single u32, so we
                            // can't send Stop and Restart on the same frame
                            // (Restart overwrites Stop). Defer Restart until
                            // cave2 confirms mode==OFF via the per-frame
                            // poll_pending_stop_then_restart.
                            if shared.mode_volatile() != TasMode::Off as u32 {
                                shared.send_command(TasCommand::Stop);
                                self.pending_stop_then_restart = Some(c);
                                self.log_lines.push(format!(
                                    "[{}] In-process Stop → wait for OFF → Restart → {:?}",
                                    ts, c
                                ));
                            } else {
                                shared.reset_restart_state();
                                shared.send_command(TasCommand::Restart);
                                self.pending_after_restart = Some(c);
                                self.log_lines.push(format!(
                                    "[{}] In-process F5 restart → {:?}",
                                    ts, c
                                ));
                            }
                        }
                        transport::Action::Undo => {
                            if let Some(snap) = self.history.undo() {
                                snap.restore_to(shared.state_mut());
                                self.log_lines
                                    .push(format!("[{}] Undo: restored previous recording", ts));
                            }
                        }
                        transport::Action::Redo => {
                            if let Some(snap) = self.history.redo() {
                                snap.restore_to(shared.state_mut());
                                self.log_lines
                                    .push(format!("[{}] Redo: restored next recording", ts));
                            }
                        }
                        transport::Action::SetContinueFrame(frame) => {
                            self.continue_from_frame = frame;
                            self.continue_from_text = frame.to_string();
                            shared.state_mut().continue_from_frame = frame;
                        }
                        transport::Action::StepOne => {
                            self.log_lines.push(format!(
                                "[{}] Step one frame (requires DLL support)",
                                ts
                            ));
                        }
                        transport::Action::Log(msg) => {
                            self.log_lines.push(format!("[{}] {}", ts, msg));
                        }
                    }
                }

                // Sync playback_speed to shared state for Cave 5
                shared.state_mut().playback_speed = self.playback_speed;

                ui.separator();

                // Status block
                let state = shared.state();
                let mode_color = match state.mode_enum() {
                    TasMode::Off => egui::Color32::GRAY,
                    TasMode::Rec => egui::Color32::from_rgb(255, 80, 80),
                    TasMode::Play => egui::Color32::from_rgb(80, 200, 80),
                };
                let play_pos = state.playback_pos;
                let rec_count = state.recorded_count;
                let headline = match state.mode_enum() {
                    TasMode::Play if play_pos > 0 && rec_count > 0 => {
                        let pct = (play_pos as f64 / rec_count as f64 * 100.0).min(100.0);
                        format!(
                            "{} {} / {} ticks ({:.0}%)",
                            state.mode_str(),
                            play_pos,
                            rec_count,
                            pct
                        )
                    }
                    TasMode::Rec if rec_count > 0 => {
                        format!("{} {} ticks", state.mode_str(), rec_count)
                    }
                    _ => state.mode_str().to_string(),
                };
                let vx = state.velocity_x as f64;
                let vy = state.velocity_y as f64;
                let vz = state.velocity_z as f64;
                let speed_kmh = (vx * vx + vy * vy + vz * vz).sqrt() * 360.0;
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.colored_label(
                        mode_color,
                        egui::RichText::new(headline.clone()).strong().size(16.0),
                    );
                    ui.label(format!(
                        "Pos: ({:.1}, {:.1}, {:.1})    Speed: {:.1} km/h",
                        state.player_x, state.player_y, state.player_z, speed_kmh
                    ));
                    // Tick is shown only when meaningful (REC/PLAY); the
                    // headline already says "REC 1234 ticks" so duplicating
                    // it here just adds noise.
                    if matches!(state.mode_enum(), TasMode::Rec | TasMode::Play) {
                        ui.label(format!(
                            "Vel: ({:.2}, {:.2}, {:.2})    Tick: {}",
                            state.velocity_x,
                            state.velocity_y,
                            state.velocity_z,
                            state.tick_count
                        ));
                    } else {
                        ui.label(format!(
                            "Vel: ({:.2}, {:.2}, {:.2})",
                            state.velocity_x, state.velocity_y, state.velocity_z
                        ));
                    }
                });

                ui.separator();

                // DRIFT ALERT BANNER — large, unmissable warning when drift is detected
                let max_drift = self.cached_max_drift_x.max(self.cached_max_drift_z);
                if max_drift > 0.0 && state.mode_enum() == TasMode::Play {
                    let (bg, text, msg) = if max_drift >= 1.0 {
                        (
                            egui::Color32::from_rgb(180, 30, 30),
                            egui::Color32::WHITE,
                            format!("DRIFT DETECTED — TAS INVALID  (X={:.6}  Z={:.6})", self.cached_max_drift_x, self.cached_max_drift_z),
                        )
                    } else {
                        (
                            egui::Color32::from_rgb(180, 140, 20),
                            egui::Color32::BLACK,
                            format!("DRIFT WARNING  (X={:.9}  Z={:.9})", self.cached_max_drift_x, self.cached_max_drift_z),
                        )
                    };
                    egui::Frame::none()
                        .fill(bg)
                        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                        .rounding(4.0)
                        .show(ui, |ui: &mut egui::Ui| {
                            ui.label(egui::RichText::new(msg).color(text).strong().size(16.0));
                        });
                    ui.separator();
                }

                // Input Timeline — full central width. Analysis is now a
                // bottom panel (see below); Debug drift renders inline
                // here when toggled (it's a wide table that reads best
                // next to the timeline it's drifting against).
                ui.label(egui::RichText::new("Input Timeline").strong());
                let continue_changed = timeline::show(
                    ui,
                    state,
                    &mut self.timeline_zoom,
                    &mut self.timeline_scroll,
                    &mut self.continue_from_frame,
                );
                if continue_changed {
                    self.continue_from_text = self.continue_from_frame.to_string();
                }
                if self.show_debug_drift {
                    ui.separator();
                    ui.label(egui::RichText::new("Debug drift").strong());
                    drift::show(ui, state, &mut self.drift_cache);
                }

                // Trajectory + Rotation — always rendered, integrated with
                // the input timeline above. Fixed 220 px container so the
                // section's vertical footprint stays predictable regardless
                // of plot content.
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

                ui.horizontal(|ui| {
                    // Incremental max drift: only scan new coordinates since last frame.
                    let count = (state.playback_pos as usize).min(state.recorded_count as usize);

                    // Reset cache if playback restarted (count decreased)
                    if count < self.last_drift_scan_count {
                        self.cached_max_drift_x = 0.0;
                        self.cached_max_drift_z = 0.0;
                        self.last_drift_scan_count = 0;
                        self.last_logged_drift_level = 0;
                    }

                    // Only scan new coordinates
                    let prev_dx = self.cached_max_drift_x;
                    let prev_dz = self.cached_max_drift_z;
                    for i in self.last_drift_scan_count..count {
                        let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
                        if d > self.cached_max_drift_x { self.cached_max_drift_x = d; }
                        let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
                        if d > self.cached_max_drift_z { self.cached_max_drift_z = d; }
                    }
                    self.last_drift_scan_count = count;

                    // Log drift at key thresholds
                    let max_d = self.cached_max_drift_x.max(self.cached_max_drift_z);
                    let new_level = if max_d >= 5.0 { 3 }
                        else if max_d >= 1.0 { 2 }
                        else if max_d > 0.0 { 1 }
                        else { 0 };
                    if new_level > self.last_logged_drift_level {
                        let ts = chrono::Local::now().format("%H:%M:%S");
                        self.log_lines.push(format!(
                            "[{}] DRIFT at tick {}: X={:.9} Z={:.9} (was X={:.9} Z={:.9})",
                            ts, count, self.cached_max_drift_x, self.cached_max_drift_z,
                            prev_dx, prev_dz
                        ));
                        self.last_logged_drift_level = new_level;
                    }

                    let (dx, dz) = (self.cached_max_drift_x, self.cached_max_drift_z);
                    let drift_color = if dx == 0.0 && dz == 0.0 {
                        egui::Color32::from_rgb(80, 200, 80)
                    } else if dx < 1.0 && dz < 1.0 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::from_rgb(255, 80, 80)
                    };
                    ui.colored_label(
                        drift_color,
                        format!("Max Drift: X={:.9} Z={:.9}", dx, dz),
                    );
                });

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
                let ts = chrono::Local::now().format("%H:%M:%S");
                self.log_lines.push(format!("[{}] {} {}", ts, prefix, text));
            }
        }

        self.persist_history_if_needed();

        // Auto-refresh at ~30fps
        ctx.request_repaint_after(std::time::Duration::from_millis(33));
    }
}

/// Check if Supreme.exe (or Supreme_v1.035.exe) is running.
#[cfg(windows)]
fn is_supreme_running() -> bool {
    use std::process::Command;
    // Use tasklist to check — lightweight and doesn't require extra crates
    if let Ok(output) = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Supreme.exe", "/NH"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Supreme") {
            return true;
        }
    }
    if let Ok(output) = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq Supreme_v1.035.exe", "/NH"])
        .creation_flags(0x08000000)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Supreme") {
            return true;
        }
    }
    false
}

#[cfg(not(windows))]
fn is_supreme_running() -> bool {
    false
}

fn open_in_file_browser(path: &std::path::Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("failed to launch explorer: {}", e))?;
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("failed to launch open: {}", e))?;
        return Ok(());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| format!("failed to launch xdg-open: {}", e))?;
        return Ok(());
    }

    #[allow(unreachable_code)]
    Err("opening folders is not supported on this platform".to_string())
}

fn main() -> eframe::Result {
    // Single-instance guard via named mutex (cross-platform crate, uses Windows mutex underneath).
    let instance = single_instance::SingleInstance::new("SSBInspect").unwrap();
    if !instance.is_single() {
        eprintln!("SSB Inspect is already running.");
        #[cfg(windows)]
        unsafe {
            use std::ffi::c_void;
            type HWND = *mut c_void;
            extern "system" {
                fn FindWindowW(class: *const u16, title: *const u16) -> HWND;
                fn SetForegroundWindow(hwnd: HWND) -> i32;
            }
            let title: Vec<u16> = "SSB Inspect"
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
            if !hwnd.is_null() {
                SetForegroundWindow(hwnd);
            }
        }
        std::process::exit(0);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
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
            let subtle_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(50));

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
                egui::Stroke::new(1.0, egui::Color32::from_gray(40));

            // Inactive widgets (buttons, combo boxes, collapsing headers)
            visuals.widgets.inactive.bg_fill = widget_bg;
            visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.inactive.bg_stroke = subtle_stroke;

            // Hovered widgets
            visuals.widgets.hovered.bg_fill = widget_hover;
            visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(38);
            visuals.widgets.hovered.bg_stroke =
                egui::Stroke::new(1.0, egui::Color32::from_gray(70));

            // Active (pressed) widgets
            visuals.widgets.active.bg_fill = widget_active;
            visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(35);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(80));

            // Open widgets (dropdowns)
            visuals.widgets.open.bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.open.weak_bg_fill = egui::Color32::from_gray(28);

            // Window decoration
            visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_gray(40));

            cc.egui_ctx.set_visuals_of(egui::Theme::Dark, visuals);
            Ok(Box::new(TasApp::new()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Event, Key, Modifiers, RawInput};
    use tas_shared::TasSharedState;

    /// Test constructor: creates TasApp without shared memory or Pico.
    fn test_app() -> TasApp {
        TasApp {
            shared: None,
            connect_error: Some("Test mode: no DLL".into()),
            show_config: false,
            show_pico_panel: false,
            pico: PicoState::new(),
            history: RecordingHistory::new(64),
            history_store: None,
            recovery_store: None,
            pending_recovery: None,
            log_lines: Vec::new(),
            log_file: None,
            log_lines_persisted: 0,
            timeline_zoom: 1.0,
            timeline_scroll: 0.0,
            continue_from_frame: 0,
            continue_from_text: "0".to_string(),
            playback_speed: 1.0,
            step_mode: false,
            show_debug_drift: false,
            show_history: false,
            show_log: false,
            segment_tracker: recording::SegmentTracker::new(),
            active_recording_session: None,
            pending_session_kind: None,
            pending_continue_start_tick: None,
            last_mode: 0,
            cont_catchup_speed: None,
            cont_catchup_multiplier: 12.0,
            log_read_cursor: 0,
            cached_max_drift_x: 0.0,
            cached_max_drift_z: 0.0,
            last_drift_scan_count: 0,
            last_logged_drift_level: 0,
            drift_cache: drift::DriftCache::default(),
            trajectory_cache: trajectory::TrajectoryCache::default(),
            pending_after_restart: None,
            pending_stop_then_restart: None,
            prev_global_keys: [false; 4],
            game_pid_cached: None,
            continue_start_guard: None,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: std::time::Instant::now(),
            #[cfg(windows)]
            dark_title_bar_set: false,
            auto_screenshot_taken: false,
            launched_at: std::time::Instant::now(),
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
        assert_eq!(app.timeline_zoom, 1.0);
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

    #[test]
    fn shortcut_period_steps() {
        let mut app = test_app();
        let actions = press_key(&mut app, Key::Period, Modifiers::NONE);
        assert!(actions
            .iter()
            .any(|a| matches!(a, transport::Action::StepOne)));
    }

    // ===== Timeline zoom =====

    #[test]
    fn zoom_in_increases() {
        let mut app = test_app();
        let initial = app.timeline_zoom;
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_zoom > initial);
    }

    #[test]
    fn zoom_out_decreases() {
        let mut app = test_app();
        let initial = app.timeline_zoom;
        press_key(&mut app, Key::Minus, Modifiers::NONE);
        assert!(app.timeline_zoom < initial);
    }

    #[test]
    fn zoom_clamps_max() {
        let mut app = test_app();
        app.timeline_zoom = 10.0;
        press_key(&mut app, Key::Plus, Modifiers::NONE);
        assert!(app.timeline_zoom <= 10.0);
    }

    #[test]
    fn zoom_clamps_min() {
        let mut app = test_app();
        app.timeline_zoom = 0.1;
        press_key(&mut app, Key::Minus, Modifiers::NONE);
        assert!(app.timeline_zoom >= 0.1);
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
        assert!((normalize_playback_speed(2.0) - 2.0).abs() < 0.001);
    }

    #[test]
    fn arm_rec_resets_speed_to_default() {
        let mut app = test_app();
        app.playback_speed = 4.0;
        app.prepare_restart_action(TasCommand::ArmRec);
        assert!((app.playback_speed - 1.0).abs() < 0.001);
    }

    #[test]
    fn settings_speed_uses_pre_catchup_value() {
        let mut app = test_app();
        app.playback_speed = 12.0;
        app.cont_catchup_speed = Some(2.0);
        assert!((app.playback_speed_for_settings() - 2.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_can_be_set_to_extremes() {
        let mut app = test_app();
        app.playback_speed = 0.25;
        assert!((app.playback_speed - 0.25).abs() < 0.001);
        app.playback_speed = 4.0;
        assert!((app.playback_speed - 4.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_syncs_to_shared_state() {
        // Verify that TasSharedState has the playback_speed field
        // and it can accept the expected range of values
        let mut state = tas_shared::zeroed_boxed();
        state.playback_speed = 0.25;
        assert!((state.playback_speed - 0.25).abs() < f32::EPSILON);
        state.playback_speed = 4.0;
        assert!((state.playback_speed - 4.0).abs() < f32::EPSILON);
        // Default (zeroed) speed is 0.0 — Cave 5 interprets 0.0 as 1.0x
        let fresh = tas_shared::zeroed_boxed();
        assert_eq!(fresh.playback_speed, 0.0);
    }

    /// Regression: STOP must cancel any queued restart sequence. Without
    /// this, pressing STOP after a CONT/PLAY click but before the polling
    /// loop fires `pending_after_restart` would leave the queued command
    /// in place — the next poll then "completes" the restart and
    /// silently starts recording/playback. STOP must mean STOP.
    #[test]
    fn stop_cancels_queued_restart() {
        let mut app = test_app();
        // Simulate a CONT being queued: pending_after_restart set by
        // queue_restart_then or the inline action handler.
        app.pending_after_restart = Some(TasCommand::ArmContinue);
        app.pending_stop_then_restart = Some(TasCommand::ArmContinue);
        app.prepare_send_action(TasCommand::Stop);
        assert!(
            app.pending_after_restart.is_none(),
            "STOP must clear pending_after_restart"
        );
        assert!(
            app.pending_stop_then_restart.is_none(),
            "STOP must clear pending_stop_then_restart"
        );
    }

    #[test]
    fn cont_stop_play_clears_catchup_state() {
        let mut app = test_app();
        app.playback_speed = 1.0;
        app.cont_catchup_multiplier = 12.0;

        app.prepare_restart_action(TasCommand::ArmContinue);
        assert_eq!(app.cont_catchup_speed, Some(1.0));
        assert!((app.playback_speed - 12.0).abs() < 0.001);

        app.prepare_send_action(TasCommand::Stop);
        assert!(app.cont_catchup_speed.is_none());
        assert!((app.playback_speed - 1.0).abs() < 0.001);

        app.prepare_restart_action(TasCommand::ArmPlay);
        assert!(app.cont_catchup_speed.is_none());
        assert!((app.playback_speed - 1.0).abs() < 0.001);
        assert!(app.pending_session_kind.is_none());
    }

    /// Regression: after a CONT cycle finishes (the PLAY→REC mode flip
    /// clears the catch-up state and restores playback_speed to its
    /// pre-catchup value), pressing CONT again must re-engage the catch-
    /// up multiplier — the second press shouldn't run at 1×.
    ///
    /// The bug we're fending off: clear_cont_catchup leaves
    /// cont_catchup_speed=None and playback_speed=1.0; if the next
    /// CONT-press path forgets to apply the multiplier, the user sees
    /// "first cont catches up, second cont crawls at native speed".
    #[test]
    fn second_cont_press_reengages_catchup() {
        let mut app = test_app();
        app.playback_speed = 1.0;
        app.cont_catchup_multiplier = 32.0;

        // ---- First CONT press ----
        app.prepare_restart_action(TasCommand::ArmContinue);
        assert_eq!(app.cont_catchup_speed, Some(1.0));
        assert!(
            (app.playback_speed - 32.0).abs() < 0.001,
            "First CONT press should set playback_speed = 32, got {}",
            app.playback_speed
        );

        // Simulate the PLAY→REC mode flip at splice frame: the runtime
        // mode-transition path calls clear_cont_catchup, restoring
        // playback_speed to its saved pre-catchup value.
        app.clear_cont_catchup();
        assert!(app.cont_catchup_speed.is_none());
        assert!(
            (app.playback_speed - 1.0).abs() < 0.001,
            "After REC start, playback_speed should be restored to 1.0, got {}",
            app.playback_speed
        );

        // Simulate user pressing STOP (REC→OFF). State should remain
        // at 1.0 (already restored) with no catchup tracking.
        app.prepare_send_action(TasCommand::Stop);
        assert!(app.cont_catchup_speed.is_none());
        assert!((app.playback_speed - 1.0).abs() < 0.001);

        // ---- Second CONT press ----
        // This is the regression target: must re-engage the multiplier.
        app.prepare_restart_action(TasCommand::ArmContinue);
        assert_eq!(
            app.cont_catchup_speed,
            Some(1.0),
            "Second CONT press should save the current 1.0 speed for later restore"
        );
        assert!(
            (app.playback_speed - 32.0).abs() < 0.001,
            "Second CONT press must re-apply the catchup multiplier (32), got {}",
            app.playback_speed
        );
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
        let values: std::collections::HashSet<u64> =
            (1..=17).map(cont_retry_jitter_ms).collect();
        // 17 retries should hit 17 distinct phases (gcd(7,17) = 1).
        assert!(
            values.len() >= 10,
            "Jitter sequence too repetitive: {} distinct values in first 17 attempts",
            values.len()
        );
        // No value should exceed ~20ms — a single retry shouldn't feel
        // like an unresponsive UI freeze.
        let max = (1..=17).map(cont_retry_jitter_ms).max().unwrap();
        assert!(max <= 20, "Jitter ms upper bound too large: {}", max);
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
        app.queue_restart_then(TasCommand::ArmContinue, "test");
        assert!(
            app.cont_catchup_speed.is_none(),
            "CONT without recording must not engage catchup speed"
        );
        assert!(
            (app.playback_speed - 1.0).abs() < 0.001,
            "playback_speed must remain at pre-CONT value, got {}",
            app.playback_speed
        );
        assert!(app.continue_start_guard.is_none());
        assert!(app.pending_after_restart.is_none());
    }

    /// The CONT catchup slider must allow speeds up to the
    /// per-frame-overhead ceiling. Empirically 128× setting saves
    /// ~200ms over 64× on a 5200-frame splice (one-shot reliability
    /// drops 80%→65% but auto-reroll covers misses). If anyone ever
    /// caps the slider below 128, power users on long recordings
    /// would be silently throttled. Also locks the new default of
    /// 64×, which strikes the reliability/wall-time balance.
    #[test]
    fn cont_catchup_settings_default_and_range() {
        use crate::settings::Settings;
        let s = Settings::default();
        assert!(
            (s.cont_catchup_speed - 64.0).abs() < f32::EPSILON,
            "Default catchup must be 64×, got {}",
            s.cont_catchup_speed
        );
        // Verify a 128× setting round-trips through settings without
        // clamping or rounding.
        let mut s = Settings::default();
        s.cont_catchup_speed = 128.0;
        assert!(
            (s.cont_catchup_speed - 128.0).abs() < f32::EPSILON,
            "Settings must allow 128× catchup for power users"
        );
    }

    // ===== Crash detection =====

    #[test]
    fn crash_detection_no_shared_is_noop() {
        let mut app = test_app();
        // Should not panic when shared is None
        app.check_game_health();
        assert_eq!(app.stale_frame_ticks, 0);
    }

    #[test]
    fn crash_detection_stale_frame_increments_without_shared() {
        // Without shared memory, stale_frame_ticks should never increment
        let mut app = test_app();
        app.last_health_check = std::time::Instant::now() - std::time::Duration::from_secs(2);
        app.check_game_health();
        assert_eq!(
            app.stale_frame_ticks, 0,
            "No shared = no stale tick increment"
        );
    }

    // ===== Panel toggle defaults =====

    #[test]
    fn panel_defaults() {
        let app = test_app();
        assert!(!app.show_debug_drift);
        assert!(!app.show_pico_panel);
        assert!(!app.show_history);
    }

    #[test]
    fn panel_toggles_persist() {
        let mut app = test_app();
        app.show_debug_drift = true;
        app.show_history = true;
        assert!(app.show_debug_drift);
        assert!(app.show_history);
    }

    // ===== Segment tracker mode transitions =====

    #[test]
    fn segment_tracker_integration_rec_stop_cycle() {
        let mut app = test_app();
        // Simulate mode transition OFF -> REC
        app.segment_tracker.on_rec_start(0);
        // Simulate REC -> OFF
        app.segment_tracker.on_rec_stop(100);
        assert_eq!(app.segment_tracker.segments.len(), 1);
        assert_eq!(app.segment_tracker.segments[0].start_tick, 0);
        assert_eq!(app.segment_tracker.segments[0].end_tick, 100);
    }

    #[test]
    fn segment_tracker_multi_segment_rec_cont() {
        let mut app = test_app();
        // First segment
        app.segment_tracker.on_rec_start(0);
        app.segment_tracker.on_rec_stop(200);
        // Continue from 200
        app.segment_tracker.on_rec_start(200);
        app.segment_tracker.on_rec_stop(500);

        assert_eq!(app.segment_tracker.segments.len(), 2);
        assert_eq!(app.segment_tracker.segments[1].start_tick, 200);
        assert_eq!(app.segment_tracker.segments[1].end_tick, 500);
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

    // ===== Pending restart state machine =====

    #[test]
    fn pending_restart_initially_none() {
        let app = test_app();
        assert!(app.pending_after_restart.is_none());
    }

    #[test]
    fn pending_restart_can_be_set() {
        let mut app = test_app();
        app.pending_after_restart = Some(TasCommand::ArmRec);
        assert!(matches!(
            app.pending_after_restart,
            Some(TasCommand::ArmRec)
        ));
    }

    // ===== Incremental drift cache =====

    #[test]
    fn drift_cache_matches_full_scan() {
        let mut app = test_app();

        // Heap-allocate: TasSharedState is ~1.5MB, too large for stack
        let mut state: Box<TasSharedState> = unsafe {
            Box::from_raw(Box::into_raw(
                vec![0u8; std::mem::size_of::<TasSharedState>()].into_boxed_slice(),
            ) as *mut [u8] as *mut TasSharedState)
        };
        state.recorded_count = 100;
        state.playback_pos = 100;

        // Set some coords with known drift
        for i in 0..100usize {
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 2.0];
            state.play_coords[i] = [i as f32 + 0.5, 0.0, i as f32 * 2.0 + 1.0];
        }
        // Spike at index 50
        state.play_coords[50][0] = state.rec_coords[50][0] + 7.5;
        state.play_coords[50][2] = state.rec_coords[50][2] + 3.25;

        // Run incremental scan
        let count = (state.playback_pos as usize).min(state.recorded_count as usize);
        for i in app.last_drift_scan_count..count {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x {
                app.cached_max_drift_x = d;
            }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z {
                app.cached_max_drift_z = d;
            }
        }
        app.last_drift_scan_count = count;

        // Full scan for comparison
        let (mut full_dx, mut full_dz) = (0.0f32, 0.0f32);
        for i in 0..count {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > full_dx {
                full_dx = d;
            }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > full_dz {
                full_dz = d;
            }
        }

        assert_eq!(app.cached_max_drift_x, full_dx);
        assert_eq!(app.cached_max_drift_z, full_dz);
        assert_eq!(app.cached_max_drift_x, 7.5);
        assert_eq!(app.cached_max_drift_z, 3.25);
    }

    #[test]
    fn drift_cache_resets_on_playback_restart() {
        let mut app = test_app();

        // First playback: 50 frames with drift
        app.cached_max_drift_x = 5.0;
        app.cached_max_drift_z = 3.0;
        app.last_drift_scan_count = 50;

        // Playback restarts (count drops to 0)
        let new_count: usize = 0;
        if new_count < app.last_drift_scan_count {
            app.cached_max_drift_x = 0.0;
            app.cached_max_drift_z = 0.0;
            app.last_drift_scan_count = 0;
        }

        assert_eq!(app.cached_max_drift_x, 0.0);
        assert_eq!(app.cached_max_drift_z, 0.0);
        assert_eq!(app.last_drift_scan_count, 0);
    }

    #[test]
    fn drift_cache_incremental_accumulates() {
        let mut app = test_app();

        // First batch: frames 0..10 with small drift
        // Heap-allocate: TasSharedState is ~1.5MB, too large for stack
        let mut state: Box<TasSharedState> = unsafe {
            Box::from_raw(Box::into_raw(
                vec![0u8; std::mem::size_of::<TasSharedState>()].into_boxed_slice(),
            ) as *mut [u8] as *mut TasSharedState)
        };
        state.recorded_count = 20;
        state.playback_pos = 10;
        for i in 0..20usize {
            state.rec_coords[i] = [0.0, 0.0, 0.0];
            state.play_coords[i] = [0.1, 0.0, 0.2];
        }
        // Spike in second batch
        state.play_coords[15] = [9.9, 0.0, 8.8];

        // Scan first batch
        let count1 = 10usize;
        for i in app.last_drift_scan_count..count1 {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x {
                app.cached_max_drift_x = d;
            }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z {
                app.cached_max_drift_z = d;
            }
        }
        app.last_drift_scan_count = count1;

        assert!((app.cached_max_drift_x - 0.1).abs() < 0.001);
        assert!((app.cached_max_drift_z - 0.2).abs() < 0.001);

        // Scan second batch (frames 10..20)
        let count2 = 20usize;
        for i in app.last_drift_scan_count..count2 {
            let d = (state.play_coords[i][0] - state.rec_coords[i][0]).abs();
            if d > app.cached_max_drift_x {
                app.cached_max_drift_x = d;
            }
            let d = (state.play_coords[i][2] - state.rec_coords[i][2]).abs();
            if d > app.cached_max_drift_z {
                app.cached_max_drift_z = d;
            }
        }
        app.last_drift_scan_count = count2;

        assert!((app.cached_max_drift_x - 9.9).abs() < 0.001);
        assert!((app.cached_max_drift_z - 8.8).abs() < 0.001);
    }
}
