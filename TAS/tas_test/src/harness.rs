//! Shared live-game scaffolding: Pico input, F5 restarts, wait/poll helpers,
//! session liveness, and the locator for the committed recording fixtures.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::win32;

/// Pico HID COM port. Override with `TAS_PICO_PORT` (default COM7).
pub fn pico_port() -> String {
    std::env::var("TAS_PICO_PORT").unwrap_or_else(|_| "COM7".into())
}

/// Release every Pico key (`0xFF` is the firmware's "all up" mask).
///
/// The firmware's own 500 ms watchdog would release them anyway; this makes the
/// starting state explicit and reports whether the device is reachable before a
/// test leans on it. Best effort: if the port will not open there is nothing to
/// release.
pub fn pico_release_all() -> bool {
    let com_path = format!("\\\\.\\{}", pico_port());
    match std::fs::OpenOptions::new().write(true).open(&com_path) {
        Ok(mut p) => p.write_all(&[0xFF]).and_then(|_| p.flush()).is_ok(),
        Err(_) => false,
    }
}

/// Holds the Pico port open for a press/release pair and guarantees the release:
/// `Drop` runs on early return, `?` and panic unwind, so a held key cannot
/// outlive the code that pressed it.
pub struct PicoKeys {
    port: std::fs::File,
    port_name: String,
}

impl PicoKeys {
    pub fn open() -> Option<PicoKeys> {
        Self::open_checked().ok()
    }

    pub fn open_checked() -> Result<PicoKeys, String> {
        let port_name = pico_port();
        let com_path = format!("\\\\.\\{}", port_name);
        std::fs::OpenOptions::new()
            .write(true)
            .open(&com_path)
            .map_err(|error| format!("Cannot open Pico {port_name}: {error}"))
            .map(|port| PicoKeys { port, port_name })
    }

    /// Send a raw mask. Returns false if the write did not reach the device —
    /// callers must not report success on a write they never landed.
    pub fn send(&mut self, mask: u8) -> bool {
        self.port
            .write_all(&[mask])
            .and_then(|_| self.port.flush())
            .is_ok()
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }
}

impl Drop for PicoKeys {
    fn drop(&mut self) {
        let _ = self.port.write_all(&[0xFF]);
        let _ = self.port.flush();
    }
}

/// How long to wait after F5 for the game to restart loading.
const F5_SETTLE_MS: u64 = 4000;
/// Frames to wait for physics stabilization after an F5 restart.
const STABILIZE_FRAMES: u32 = 500;
/// Timeout for the in-process restart state machine.
const RESTART_TIMEOUT_SECS: u64 = 15;
/// Playback timeout (long enough for 65536 frames at ~50fps unfocused).
const PLAYBACK_TIMEOUT_SECS: u64 = 120;
/// Time allowed at 1x for a CONT replay to reach the bucket judge point.
const JUDGE_TIMEOUT_SECS: u64 = 10;
/// Retry budget for landing a matching F5 start bucket. Acceptance has needed
/// up to 13; 60 covers that tail with margin.
pub const START_MATCH_RETRIES: u32 = 60;

/// Bring the game window to the front. Best effort: a missing window is left alone.
pub fn focus_game() {
    if let Some(hwnd) = win32::find_game_window() {
        win32::bring_to_front(hwnd);
    }
    thread::sleep(Duration::from_millis(200));
}

/// Send Escape through the Pico (mask bit 7). The Pico is a real HID keyboard,
/// so the press reaches the game's pause handler; the synthetic fallback for
/// machines without one generally does not, and the caller's verdict must
/// notice (pause-resume checks that the frame counter actually stalled).
pub fn send_escape() -> bool {
    focus_game();
    if let Some(mut keys) = PicoKeys::open() {
        if !keys.send(0x80) {
            eprintln!(
                "  WARNING: Pico on {} opened but the Escape press failed to write",
                keys.port_name()
            );
            return false;
        }
        thread::sleep(Duration::from_millis(80));
        println!("  Escape sent via Pico ({})", keys.port_name());
        return true;
    }
    eprintln!(
        "  WARNING: Pico not available on {} — falling back to keybd_event (likely won't pause the game)",
        pico_port()
    );
    win32::tap_key(win32::VK_ESCAPE, Duration::from_millis(80));
    true
}

/// Post Enter to the game window to dismiss the post-run "Save attempt" dialog.
///
/// PostMessage needs no focus, so it reaches the modal dialog that blocks focus
/// changes. Without this the next F5 lands on the dialog and gets eaten, which
/// shifts the F5 bucket alignment by a cycle. Harmless when no dialog is up.
pub fn dismiss_save_dialog() {
    if let Some(hwnd) = win32::find_game_window() {
        win32::post_key(hwnd, win32::VK_RETURN, Duration::from_millis(30));
    }
    thread::sleep(Duration::from_millis(150));
}

/// Kill any `tas_ui`: two writers on the single-slot command channel cause
/// intermittent ARM_CONTINUE mode=0 failures. Returns how many were stopped.
pub fn stop_competing_tas_ui_writer() -> u32 {
    let stopped = kill_image("tas_ui.exe", None);
    if stopped > 0 {
        println!("  Stopped competing tas_ui writer(s): {stopped}");
    }
    stopped
}

/// Ensure `tas_test` owns the shared-memory command surface before a
/// speed-sensitive mode runs.
pub fn ensure_exclusive_runtime_ownership(client: &mut TasSharedMemoryClient, failure_hint: &str) {
    let stopped_writers = stop_competing_tas_ui_writer();
    stop(client);
    thread::sleep(Duration::from_millis(100));
    if stopped_writers == 0 {
        return;
    }

    println!(
        "  Runtime warmup after tas_ui stop ({} writer(s) killed)",
        stopped_writers
    );
    println!(
        "  NOTE: If {} persists after this, rerun from a clean revive with tas_ui closed.",
        failure_hint
    );
    if !restart_and_stabilize_inprocess(client) {
        eprintln!("ERROR: Game not alive during pre-test warmup restart");
        std::process::exit(1);
    }
    stop(client);
    thread::sleep(Duration::from_millis(100));
}

/// Send F5 via PostMessage (no focus required).
fn send_f5_postmessage() {
    match win32::find_game_window() {
        Some(hwnd) => {
            win32::post_key(hwnd, win32::VK_F5, Duration::from_millis(50));
            println!("  F5 sent via PostMessage (hwnd={:#x})", hwnd);
        }
        None => eprintln!("  ERROR: Cannot find Supreme window for PostMessage F5"),
    }
}

/// Send F5 via Pico HID to restart the race.
pub fn send_f5_pico() {
    focus_game();
    let port = pico_port();
    match PicoKeys::open() {
        Some(mut keys) => {
            if !keys.send(0x40) {
                eprintln!("  WARNING: Pico F5 press failed to write; using PostMessage fallback.");
                drop(keys);
                send_f5_postmessage();
                return;
            }
            thread::sleep(Duration::from_millis(100));
            println!("  F5 sent via Pico ({})", port);
        }
        None => {
            eprintln!(
                "  WARNING: Failed to open {}. Using PostMessage fallback.",
                port
            );
            send_f5_postmessage();
        }
    }
}

/// Wait for N frames to pass (based on Cave 2 frame_count).
pub fn wait_frames(client: &TasSharedMemoryClient, count: u32) {
    let start_fc = client.frame_count_volatile();
    let timeout = Instant::now();
    while client.frame_count_volatile() < start_fc + count {
        thread::sleep(Duration::from_millis(10));
        if timeout.elapsed() > Duration::from_secs(15) {
            eprintln!("  WARNING: wait_frames timeout ({} frames)", count);
            break;
        }
    }
}

/// Check that Cave 2 is firing (game is alive and hooks are active).
pub fn check_liveness(client: &TasSharedMemoryClient) -> bool {
    let fc1 = client.frame_count_volatile();
    thread::sleep(Duration::from_millis(500));
    let fc2 = client.frame_count_volatile();
    let delta = fc2 - fc1;
    println!("  Liveness: {} frames/500ms", delta);
    delta > 0
}

/// Pico F5 restart + stabilization. Double-F5: the first normalizes game state,
/// the second produces the deterministic restart position.
pub fn restart_and_stabilize(client: &TasSharedMemoryClient) -> bool {
    dismiss_save_dialog();

    println!("  Sending F5 (normalize)...");
    send_f5_pico();
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, 100);

    println!("  Sending F5 (restart)...");
    send_f5_pico();
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, STABILIZE_FRAMES);
    println!("  Stabilized at frame {}", client.frame_count_volatile());
    check_liveness(client)
}

/// Send CMD_RESTART and wait for cave2's restart machine to report done.
pub fn restart_inprocess(client: &mut TasSharedMemoryClient) -> bool {
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    let start = Instant::now();
    while client.restart_state() != 2 {
        if start.elapsed() > Duration::from_secs(RESTART_TIMEOUT_SECS) {
            eprintln!(
                "  ERROR: In-process restart timeout (restart_state={})",
                client.restart_state()
            );
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
    client.reset_restart_state();
    true
}

/// In-process restart (CMD_RESTART), the path tas_ui's transport uses. The
/// dialog dismissal matters here too: the restart invokes the game's own F5
/// handler, which the dialog intercepts.
pub fn restart_and_stabilize_inprocess(client: &mut TasSharedMemoryClient) -> bool {
    dismiss_save_dialog();
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(50));
    if !restart_inprocess(client) {
        return false;
    }
    println!(
        "  In-process restart stabilized at frame {}",
        client.frame_count_volatile()
    );
    // restart_state==2 is written by cave2 inside the Supreme::Cycle hook, so
    // reaching it already proves the game is alive. Do NOT add a liveness sleep
    // here: every frame spent between restart and arm pushes play_coords[0]
    // later into the spawn countdown, and recordings made by tas_ui (which arms
    // immediately) then never match.
    true
}

/// Arm recording and wait for mode to switch.
pub fn arm_rec(client: &mut TasSharedMemoryClient) {
    client.send_command(TasCommand::ArmRec);
    thread::sleep(Duration::from_millis(50));
    let mode = client.mode_volatile();
    println!("  ARM_REC -> mode={} (expect 1=REC)", mode);
}

/// Arm playback and wait for mode to switch.
pub fn arm_play(client: &mut TasSharedMemoryClient) {
    client.send_command(TasCommand::ArmPlay);
    thread::sleep(Duration::from_millis(50));
    let mode = client.mode_volatile();
    println!("  ARM_PLAY -> mode={} (expect 2=PLAY)", mode);
}

/// Stop recording/playback.
pub fn stop(client: &mut TasSharedMemoryClient) {
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(100));
}

/// Wait for playback to complete (mode exits PLAY or timeout).
pub fn wait_playback(client: &TasSharedMemoryClient, expected: u32) -> bool {
    let start = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(50));
        if client.mode_volatile() != TasMode::Play as u32 {
            let pos = client.playback_pos_volatile();
            if pos >= expected {
                println!("  Playback complete: {}/{}", pos, expected);
                return true;
            }
            eprintln!(
                "  Playback exited PLAY early at {}/{} (treating as failure)",
                pos, expected
            );
            return false;
        }
        if start.elapsed() > Duration::from_secs(PLAYBACK_TIMEOUT_SECS) {
            let pos = client.playback_pos_volatile();
            eprintln!("  Playback timeout at {}/{}", pos, expected);
            return false;
        }
    }
}

/// Restart and arm the product PLAY path with input indexed relative to the
/// recording's gate. Returns `(recorded_gate, live_gate)` once the live gate has
/// been observed and the gate-relative trajectory watcher accepts the attempt.
pub fn restart_play_aligned_inprocess(client: &mut TasSharedMemoryClient) -> Option<(u32, u32)> {
    use tas_shared::transport::{Arm, ArmConfig, StepOutcome, TransportController};

    let rec_gate = {
        let s = client.state();
        tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count)?
    };
    let catchup_speed = client.state().playback_speed.max(1.0);
    let mut controller = TransportController::new(ArmConfig {
        arm: Arm::Play,
        catchup_speed,
        continue_from_frame: 0,
        gate_align_rec: rec_gate,
        target: None,
        max_retries: tas_shared::cont::START_MATCH_MAX_RETRIES,
        resume_speed: 0.0,
        predict_bucket: false,
    });

    stop_competing_tas_ui_writer();
    dismiss_save_dialog();
    let deadline = Instant::now() + Duration::from_secs(180);
    let retries_used = loop {
        if Instant::now() > deadline {
            eprintln!(
                "  ERROR: aligned PLAY timed out in {}",
                controller.phase_name()
            );
            stop(client);
            return None;
        }
        match controller.step(client) {
            StepOutcome::InProgress => thread::sleep(Duration::from_millis(5)),
            StepOutcome::Wait { ms } => thread::sleep(Duration::from_millis(ms)),
            StepOutcome::Done { retries_used, .. } => {
                break retries_used;
            }
            StepOutcome::Reroll {
                attempt,
                suggested_delay_ms,
                observed,
                ..
            } => {
                let mismatch = observed
                    .map(|frame| frame.to_string())
                    .unwrap_or_else(|| "?".to_string());
                println!(
                    "  Aligned PLAY watcher rejected attempt {}/{} (first mismatch at gate+{})",
                    attempt,
                    tas_shared::cont::START_MATCH_MAX_RETRIES,
                    mismatch
                );
                thread::sleep(Duration::from_millis(suggested_delay_ms));
            }
            StepOutcome::Aborted { reason } => {
                eprintln!("  ERROR: aligned PLAY aborted: {}", reason);
                stop(client);
                return None;
            }
        }
    };

    let gate_deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let gate = client.state().gate_index;
        if gate != 0 {
            println!(
                "  Aligned PLAY watcher accepted after {} retr{}: recording gate {}, live gate {} (offset {:+})",
                retries_used,
                if retries_used == 1 { "y" } else { "ies" },
                rec_gate,
                gate,
                gate as i64 - rec_gate as i64
            );
            return Some((rec_gate, gate));
        }
        if client.mode_volatile() != TasMode::Play as u32 || Instant::now() > gate_deadline {
            eprintln!("  ERROR: aligned PLAY never observed a live gate");
            stop(client);
            return None;
        }
        thread::sleep(Duration::from_millis(5));
    }
}

/// An environment variable the live launch path needs; no built-in default.
fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| format!("{name} is not set (needed to launch the game)"))
}

/// Launch Supreme Snowboarding via the `revive-supreme.nu` script with `NO_CE=1`.
/// Sets `TAS_TEST_PID` so the script can skip killing us.
fn run_revive(script: &str) -> bool {
    println!("Launching game via revive-supreme (NO_CE=1)...");
    println!("  Script: {}", script);
    match Command::new("nu")
        .arg(script)
        .env("NO_CE", "1")
        .env("TAS_TEST_PID", std::process::id().to_string())
        .status()
    {
        Ok(s) if s.success() => {
            println!("  revive-supreme finished successfully");
            true
        }
        Ok(s) => {
            eprintln!("  revive-supreme exited with {}", s);
            false
        }
        Err(e) => {
            eprintln!("  Failed to run revive-supreme: {}", e);
            eprintln!("  Is `nu` on PATH? Script at: {}", script);
            false
        }
    }
}

fn hide_console(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
}

/// `taskkill` one image name; returns how many processes it terminated.
/// Matches exact image names, and never `supreme-service.exe`.
fn kill_image(image: &str, exclude_pid: Option<u32>) -> u32 {
    let mut command = Command::new("taskkill");
    command.args(["/F", "/IM", image, "/FI", "IMAGENAME ne supreme-service.exe"]);
    if let Some(pid) = exclude_pid {
        command.args(["/FI", &format!("PID ne {pid}")]);
    }
    hide_console(&mut command);
    match command.output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|line| line.contains("PID"))
            .count() as u32,
        Err(_) => 0,
    }
}

/// Kill every game, launcher and TAS process except ourselves (the same list
/// and `supreme-service` exclusion as the justfile's `stop_game`).
pub fn kill_game() {
    let me = std::process::id();
    for image in [
        "Supreme.exe",
        "Supreme_v1.035.exe",
        "display-config.exe",
        "Display_Config.exe",
        "tas_ui.exe",
        "tas_test.exe",
    ] {
        kill_image(image, Some(me));
    }
    thread::sleep(Duration::from_millis(500));
}

/// Inject a DLL into the running Supreme.exe process via Injector.exe.
fn inject_dll(label: &str, dll_path: &str, supreme_folder: &str) -> bool {
    let injector = format!(r"{}\Display_Config_Resources\Injector.exe", supreme_folder);

    println!("Injecting {}...", label);
    println!("  DLL: {}", dll_path);

    match Command::new(&injector).arg(dll_path).status() {
        Ok(s) if s.success() => {
            println!("  {} injected", label);
            true
        }
        Ok(s) => {
            eprintln!("  Injector.exe exited with {} for {}", s, label);
            false
        }
        Err(e) => {
            eprintln!("  Failed to run Injector.exe: {}", e);
            false
        }
    }
}

/// Check whether revive is suppressed via `NO_REVIVE=1` env var.
fn no_revive() -> bool {
    std::env::var("NO_REVIVE").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Normalise `playback_speed` to 1.0 before a mode starts.
///
/// `playback_speed` lives in shared memory and survives between tas_test runs,
/// and not every mode that sets it restores it. A leaked speed changes the next
/// mode's answer (a 0.25x leftover records a quarter of the frames a 6 s window
/// should hold), so every mode starts from 1.0 and sets its own speed after.
fn normalize_playback_speed(client: &mut TasSharedMemoryClient) {
    let prev = client.state().playback_speed;
    if prev != 1.0 {
        println!(
            "  Normalised leftover playback_speed {}x -> 1.0x (leaked from a previous run)",
            prev
        );
        client.state_mut().playback_speed = 1.0;
    }
}

/// The track every mode assumes: `revive-supreme` navigates to Forest Easy and
/// the committed `.tasrec` baselines are all FE.
const DEFAULT_EXPECTED_LEVEL: &str = "FE";

/// How long to wait for the DLL to publish a track. The scan runs on every load
/// and menu trip, so a freshly revived session reads "unknown" only briefly.
const LEVEL_SCAN_TIMEOUT_SECS: u64 = 12;

/// Assert the game is on the expected track before any mode runs.
///
/// On the wrong track the spawn is on another map: a replay's start matcher can
/// never hit and burns its retry budget, and a fresh REC records meaningless
/// coordinates. Both read as a hang or a mystery drift, so check up front.
///
/// `TAS_TEST_LEVEL` overrides the expected code (e.g. `AM`); `TAS_TEST_LEVEL=any`
/// disables the check for deliberate off-track work.
pub fn verify_expected_level(client: &TasSharedMemoryClient) {
    let expected =
        std::env::var("TAS_TEST_LEVEL").unwrap_or_else(|_| DEFAULT_EXPECTED_LEVEL.to_string());
    if expected.eq_ignore_ascii_case("any") {
        println!("  Track check: SKIPPED (TAS_TEST_LEVEL=any)");
        return;
    }

    let start = Instant::now();
    let mut live = None;
    while start.elapsed() < Duration::from_secs(LEVEL_SCAN_TIMEOUT_SECS) {
        // Use the resolved snapshot, not raw level_id: during a level change
        // level_id still holds the previous track.
        if let Some(code) =
            tas_shared::resolved_level_id(client.state()).and_then(tas_shared::level::code_from_id)
        {
            live = Some(code);
            break;
        }
        thread::sleep(Duration::from_millis(250));
    }

    match live {
        Some(code) if code.eq_ignore_ascii_case(&expected) => {
            println!("  Track check: on {} as expected", code);
        }
        Some(code) => {
            eprintln!(
                "ERROR: wrong track — the game is on {} but this run expects {}.\n  \
                 Recording or replaying here produces meaningless results: the spawn is on a\n  \
                 different map, so a replay's start matcher can never match and a fresh REC\n  \
                 captures another course entirely.\n  \
                 Navigate to {} (or set TAS_TEST_LEVEL={} / TAS_TEST_LEVEL=any).",
                code, expected, expected, code
            );
            std::process::exit(1);
        }
        None => {
            eprintln!(
                "ERROR: could not identify the track after {}s — level_id stayed 0x{:08X}.\n  \
                 The game is at a menu, mid-teardown, or in a mode the scan does not cover.\n  \
                 This run expects {}. Navigate into {} (or set TAS_TEST_LEVEL=any to bypass).",
                LEVEL_SCAN_TIMEOUT_SECS,
                client.state().level_id,
                expected,
                expected
            );
            std::process::exit(1);
        }
    }
}

/// Ensure the game is running with hooks active.
///
/// If the game is already live, reuse it. Otherwise kill stale instances and
/// launch a clean session through `REVIVE_SUPREME_SCRIPT`, then inject
/// `TAS_Helper.dll` from `SUPREME_FOLDER`. `NO_REVIVE=1` refuses to launch and
/// exits instead.
pub fn ensure_game_running() -> TasSharedMemoryClient {
    if pico_release_all() {
        println!("  Pico: released all keys ({})", pico_port());
    }

    if let Ok(c) = TasSharedMemoryClient::open() {
        if check_liveness(&c) {
            let s = c.state();
            println!(
                "Game already live (version {}). Hooks: cave2={} cave1c={} cave1d={} cave5={}",
                s.version, s.cave2_hooked, s.cave1c_hooked, s.cave1d_hooked, s.cave5_hooked
            );
            // A reused session is exactly where the track and playback_speed
            // can have drifted since the last run.
            verify_expected_level(&c);
            let mut c = c;
            normalize_playback_speed(&mut c);
            return c;
        }
    }

    if no_revive() {
        eprintln!("ERROR: Game not running and NO_REVIVE=1 is set");
        std::process::exit(1);
    }

    println!("Game not live — launching fresh via revive-supreme...");
    let (script, supreme_folder) = match (
        required_env("REVIVE_SUPREME_SCRIPT"),
        required_env("SUPREME_FOLDER"),
    ) {
        (Ok(script), Ok(folder)) => (script, folder),
        (Err(error), _) | (_, Err(error)) => {
            eprintln!("ERROR: {error}");
            std::process::exit(1);
        }
    };
    kill_game();

    if !run_revive(&script) {
        eprintln!("ERROR: revive-supreme failed");
        std::process::exit(1);
    }

    // The launcher flow injects Display_Config_Helper.dll itself; TAS_Helper.dll
    // is ours to inject.
    thread::sleep(Duration::from_secs(2));

    let tas_helper = format!(
        r"{}\Display_Config_Resources\TAS\TAS_Helper.dll",
        supreme_folder
    );
    if !inject_dll("TAS_Helper.dll", &tas_helper, &supreme_folder) {
        eprintln!("ERROR: TAS_Helper.dll injection failed");
        std::process::exit(1);
    }

    thread::sleep(Duration::from_secs(3));

    match TasSharedMemoryClient::open() {
        Ok(c) if check_liveness(&c) => {
            let s = c.state();
            println!(
                "Connected after revive (version {}). Hooks: cave2={} cave1c={} cave1d={} cave5={}",
                s.version, s.cave2_hooked, s.cave1c_hooked, s.cave1d_hooked, s.cave5_hooked
            );
            verify_expected_level(&c);
            let mut c = c;
            normalize_playback_speed(&mut c);
            c
        }
        Ok(_) => {
            eprintln!("ERROR: Game not live after revive-supreme");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("ERROR: No shared memory after revive-supreme: {}", e);
            std::process::exit(1);
        }
    }
}

/// Connect to shared memory, exit on failure.
pub fn connect() -> TasSharedMemoryClient {
    match TasSharedMemoryClient::open() {
        Ok(c) => {
            let s = c.state();
            println!(
                "Connected (version {}). Hooks: cave2={} cave1c={} cave1d={} cave5={}",
                s.version, s.cave2_hooked, s.cave1c_hooked, s.cave1d_hooked, s.cave5_hooked
            );
            c
        }
        Err(e) => {
            eprintln!("Failed to connect to shared memory: {}", e);
            std::process::exit(1);
        }
    }
}

/// Print current shared state status.
pub fn print_status(client: &TasSharedMemoryClient) {
    let s = client.state();
    println!("Config: fft={}", s.force_fixed_tick);
    println!(
        "State: mode={} recorded={} playback_pos={} game_in_game={}",
        s.mode_str(),
        s.recorded_count,
        s.playback_pos,
        s.game_in_game
    );
    println!(
        "Level: epoch={} scan_epoch={} resolved={}",
        s.level_epoch,
        s.level_scan_epoch,
        tas_shared::level_is_resolved(s)
    );
    match tas_shared::level_context(s) {
        Some((id, path)) => println!("Path: gen={} id={:#x} {:?}", s.level_path_gen, id, path),
        None => println!("Path: gen={} UNRESOLVED", s.level_path_gen),
    }
    println!(
        "Ptrs: replay={:#010x} player={:#010x} level_id={:#x} race_time_cs={:#x} pos=({:.2},{:.2},{:.2})",
        s.replay_ptr, s.player_ptr, s.level_id, s.race_time_cs,
        s.player_x, s.player_y, s.player_z
    );
}

/// Pico F5 restart, then PLAY, retrying until play_coords[0] and the leading
/// trajectory match the recording. F5 lands on one of a few quantized spawn
/// buckets, so this rerolls until the right one comes up.
pub fn restart_play_and_match(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    restart_play_and_match_with(client, target, max_retries, |c| restart_and_stabilize(c))
}

/// In-process restart variant of PLAY start matching, for file-backed replays
/// whose restart semantics must match the tas_ui transport path.
pub fn restart_play_and_match_inprocess(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    restart_play_and_match_with(client, target, max_retries, |c| {
        restart_and_stabilize_inprocess(c)
    })
}

/// Leading PLAY frames that must track the recording for a start match.
///
/// Must span any stationary phase at the start of a recording (a race-start
/// countdown holds still for 200-300 frames): on a 10-frame check a stationary
/// prefix trivially matches any F5 bucket while the rotation/velocity is wrong,
/// and the divergence only appears once physics activates. 1000 frames covers
/// every realistic countdown plus several seconds of steered gameplay.
const MATCH_VERIFY_FRAMES: u32 = 1000;

fn restart_play_and_match_with<F>(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
    mut restart_fn: F,
) -> bool
where
    F: FnMut(&mut TasSharedMemoryClient) -> bool,
{
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!("  Retry {}/{}: restarting...", attempt, max_retries);
        }
        if !restart_fn(client) {
            eprintln!("  ERROR: Game not alive after restart");
            return false;
        }

        arm_play(client);
        // Poll until playback_pos reaches the verify window (or the end of the
        // recording), capped at 20 s wall so retries stay bounded.
        let wait_until = MATCH_VERIFY_FRAMES.min(client.state().recorded_count);
        let wait_start = Instant::now();
        loop {
            let pos = client.playback_pos_volatile();
            if pos >= wait_until {
                break;
            }
            if wait_start.elapsed() > Duration::from_secs(20) {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
        let s = client.state();
        if s.playback_pos == 0 {
            eprintln!("  WARNING: Playback didn't start");
            stop(client);
            continue;
        }
        let pc0 = s.play_coords[0];
        let match_x = pc0[0].to_bits() == target[0].to_bits();
        let match_y = pc0[1].to_bits() == target[1].to_bits();
        let match_z = pc0[2].to_bits() == target[2].to_bits();
        if !(match_x && match_y && match_z) {
            let dx = (pc0[0] as f64 - target[0] as f64).abs();
            let dz = (pc0[2] as f64 - target[2] as f64).abs();
            println!("  play_coords[0] offset: dx={:.9} dz={:.9}", dx, dz);
            stop(client);
            continue;
        }

        // Frame 0 matched; the following frames must track the recording within
        // BUCKET_MATCH_EPSILON, not bit-for-bit: the game wiggles low bits each
        // tick, so a bit-exact gate rejects the correct bucket over a ~0.003
        // blip, while a wrong-rotation bucket diverges far past epsilon.
        let eps = tas_shared::cont::BUCKET_MATCH_EPSILON;
        let frames_available = s
            .playback_pos
            .min(s.recorded_count)
            .min(MATCH_VERIFY_FRAMES);
        let mut traj_ok = true;
        let mut diverge_frame = 0u32;
        let mut diverge_dx = 0.0f64;
        let mut diverge_dz = 0.0f64;
        for i in 1..frames_available as usize {
            let p = s.play_coords[i];
            let r = s.rec_coords[i];
            if (p[0] - r[0]).abs() > eps || (p[1] - r[1]).abs() > eps || (p[2] - r[2]).abs() > eps {
                traj_ok = false;
                diverge_frame = i as u32;
                diverge_dx = (p[0] as f64 - r[0] as f64).abs();
                diverge_dz = (p[2] as f64 - r[2] as f64).abs();
                break;
            }
        }
        if traj_ok {
            println!(
                "  Position + trajectory matched ({} frames verified, attempt {})",
                frames_available,
                attempt + 1
            );
            return true;
        }
        println!(
            "  Trajectory diverges at frame {}: dx={:.9} dz={:.9} (likely rotation/velocity mismatch — retrying for different F5 bucket)",
            diverge_frame, diverge_dx, diverge_dz
        );
        stop(client);
    }
    eprintln!(
        "  WARNING: Could not match position+trajectory after {} retries",
        max_retries
    );
    false
}

/// Print drift results (drift computed post-hoc from coordinate arrays).
pub fn print_results(client: &TasSharedMemoryClient) {
    let s = client.state();
    let drift = crate::drift::compute_drift(s, s.recorded_count.min(s.playback_pos));
    println!("=== RESULTS ===");
    println!("Recorded: {} ticks", s.recorded_count);
    println!("Played: {} ticks", s.playback_pos);
    println!(
        "Max drift X: {:.9} (frame {})",
        drift.max_drift_x, drift.max_drift_frame_x
    );
    println!(
        "Max drift Y: {:.9} (frame {})",
        drift.max_drift_y, drift.max_drift_frame_y
    );
    println!(
        "Max drift Z: {:.9} (frame {})",
        drift.max_drift_z, drift.max_drift_frame_z
    );
    println!("BB3B10 calls: {}", s.bb3b10_call_count);
    println!("Handler blocks (Cave 1C): {}", s.handler_block_count);
    println!("BB3B10 blocks (Cave 1D): {}", s.bb3b10_block_count);
}

/// Write a synthetic input log directly into shared memory (benchmark mode;
/// perf timing does not need a real-input oracle).
pub fn write_synthetic_input(client: &mut TasSharedMemoryClient, input_log: &[u8]) {
    let state = client.state_mut();
    let len = input_log.len().min(state.input_log.len());
    state.input_log[..len].copy_from_slice(&input_log[..len]);
    state.recorded_count = len as u32;
}

/// Arm continue-from-frame: set the splice point and send ARM_CONTINUE. The DLL
/// plays 0..frame, then switches to REC.
pub fn arm_continue(client: &mut TasSharedMemoryClient, frame: u32) {
    client.state_mut().continue_from_frame = frame;
    client.send_command(TasCommand::ArmContinue);
    thread::sleep(Duration::from_millis(50));
    let mode = client.mode_volatile();
    println!(
        "  ARM_CONTINUE(frame={}) -> mode={} (expect 2=PLAY initially)",
        frame, mode
    );
}

/// Wait for the PLAY->REC transition of an ARM_CONTINUE splice.
pub fn wait_continue_splice(client: &TasSharedMemoryClient, splice_frame: u32) -> bool {
    let start = Instant::now();
    loop {
        // Poll faster than one 1x tick (10 ms) so the splice is seen within a
        // tick; a slower poll reads the resumed REC's first ticks as overshoot.
        thread::sleep(Duration::from_millis(2));
        let s = client.state();
        if s.mode == TasMode::Rec as u32 {
            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            let baseline_ms = splice_frame as f64 * 10.0;
            let effective_speed = if elapsed_ms > 0.0 {
                baseline_ms / elapsed_ms
            } else {
                f64::INFINITY
            };
            println!(
                "  CONT splice complete: mode=REC at pos={}, segment_count={}",
                s.recorded_count, s.segment_count
            );
            println!(
                "  CONT catch-up wall-clock: {:.1} ms (effective {:.2}x vs 1x baseline {:.1} ms)",
                elapsed_ms, effective_speed, baseline_ms
            );
            return true;
        }
        if s.mode == TasMode::Off as u32 {
            eprintln!("  ERROR: Mode went to OFF during CONT replay");
            return false;
        }
        if start.elapsed() > Duration::from_secs(PLAYBACK_TIMEOUT_SECS) {
            eprintln!(
                "  CONT replay timeout: playback_pos={}/{}",
                s.playback_pos, splice_frame
            );
            return false;
        }
    }
}

/// Pico F5 restart + ARM_CONTINUE with bucket matching, then wait for the splice.
/// Returns `Some(reroll_count)` on a clean splice, `None` on failure.
///
/// The bucket is judged with tas_ui's criteria (`judge_cont_bucket`: spawn bits
/// plus the first-moving fingerprint) EARLY, before the splice fires, so a wrong
/// bucket is stopped before it can mutate the recording.
pub fn restart_continue_and_splice(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    use tas_shared::cont::BucketVerdict;

    let expected_start_bits = [
        target[0].to_bits(),
        target[1].to_bits(),
        target[2].to_bits(),
    ];
    let expected_first_moving = {
        let s = client.state();
        tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count)
    };
    match expected_first_moving {
        Some(fm) => println!(
            "  CONT bucket criteria (tas_ui): spawn match + first-moving frame {}",
            fm
        ),
        None => {
            println!("  CONT bucket criteria (tas_ui): recording never moves — spawn match only")
        }
    }

    for attempt in 0..=max_retries {
        stop_competing_tas_ui_writer();
        if attempt > 0 {
            println!("  Retry {}/{}: CONT bucket reroll", attempt, max_retries);
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after restart");
            return None;
        }
        // Arm immediately after the restart (no focus call): every frame spent
        // here shifts play_coords[0] later into the spawn countdown.
        arm_continue(client, splice_frame);
        thread::sleep(Duration::from_millis(100));

        {
            let s = client.state();
            if s.playback_pos == 0 && s.mode == TasMode::Off as u32 {
                eprintln!("  WARNING: CONT didn't start");
                stop(client);
                continue;
            }
        }

        let verdict = poll_cont_verdict(
            client,
            expected_start_bits,
            expected_first_moving,
            splice_frame,
        );
        match verdict {
            BucketVerdict::Match | BucketVerdict::NoSignal => {
                if attempt > 0 {
                    println!(
                        "  CONT bucket accepted (tas_ui criteria) on attempt {}",
                        attempt + 1
                    );
                }
                println!("  CONT bucket accepted, waiting for splice...");
                if wait_continue_splice(client, splice_frame) {
                    return Some(attempt);
                }
                stop(client);
                continue;
            }
            BucketVerdict::WrongBucket { observed } => {
                println!(
                    "  CONT wrong bucket: observed first-moving={:?} expected={:?} (reroll)",
                    observed, expected_first_moving
                );
                stop(client);
                continue;
            }
            BucketVerdict::WrongStart => {
                let pc0 = client.state().play_coords[0];
                let dx = (pc0[0] as f64 - target[0] as f64).abs();
                let dz = (pc0[2] as f64 - target[2] as f64).abs();
                println!("  CONT start mismatch: dx={:.9} dz={:.9} (reroll)", dx, dz);
                stop(client);
                continue;
            }
            BucketVerdict::KeepWaiting => {
                println!("  CONT bucket inconclusive (timeout before enough frames) — reroll");
                stop(client);
                continue;
            }
        }
    }
    eprintln!(
        "  WARNING: Could not land a tas_ui-acceptable CONT bucket after {} retries",
        max_retries
    );
    None
}

/// In-process CONT splice through the shared transport controller
/// (`tas_shared::transport`) — the exact restart/arm/reroll state machine tas_ui
/// drives, so this measures what the app does.
///
/// The controller reaches `Done` when the bucket is accepted (before the splice
/// fires); we then wait for the PLAY->REC splice at `splice_frame`. Returns
/// `Some(reroll_count)` on a clean splice (0 = landed first try), `None` on
/// failure.
pub fn restart_continue_and_splice_inprocess(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    use tas_shared::transport::{Arm, ArmConfig, BucketTarget, StepOutcome, TransportController};

    let expected_start_bits = [
        target[0].to_bits(),
        target[1].to_bits(),
        target[2].to_bits(),
    ];
    let expected_first_moving = {
        let s = client.state();
        tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count)
    };
    match expected_first_moving {
        Some(fm) => println!(
            "  CONT bucket criteria (shared controller): spawn match + first-moving frame {}",
            fm
        ),
        None => println!(
            "  CONT bucket criteria (shared controller): recording never moves — spawn match only"
        ),
    }

    let catchup_speed = client.state().playback_speed;
    // Gate-aligned CONT indexes the prefix input from the observed gate and
    // fires the splice at the aligned position, so a countdown landing on a
    // different tick no longer forces a reroll. Only align when the splice is
    // comfortably past the gate: the gate-relative watcher needs
    // BUCKET_MATCH_WINDOW samples BEFORE the destructive splice. Below that,
    // fall back to the bucket match.
    let rg = expected_first_moving.unwrap_or(0);
    let align_ok = rg > 0 && splice_frame > rg + tas_shared::cont::BUCKET_MATCH_WINDOW;
    let gate_align_rec = if align_ok { rg } else { 0 };
    let cfg = ArmConfig {
        arm: Arm::Continue,
        catchup_speed,
        continue_from_frame: splice_frame,
        gate_align_rec,
        target: if gate_align_rec > 0 {
            None
        } else {
            Some(BucketTarget {
                expected_start_bits,
                expected_first_moving,
            })
        },
        max_retries,
        // CONT hands the speed back at the splice (cont_resume_speed), not
        // mid-replay, so it stages no handover here.
        resume_speed: 0.0,
        predict_bucket: gate_align_rec == 0,
    };
    let mut controller = TransportController::new(cfg);

    // Keep a competing tas_ui out of the single-slot command channel, and clear
    // any post-run dialog before the first restart (it can eat the in-process F5).
    stop_competing_tas_ui_writer();
    dismiss_save_dialog();

    let speed = (catchup_speed as f64).max(0.05);
    // Per-attempt budget: restart handshake + replay-to-judge at this speed.
    let per_attempt = Duration::from_secs_f64(
        RESTART_TIMEOUT_SECS as f64 + (JUDGE_TIMEOUT_SECS as f64) * (1.0 / speed).max(1.0),
    );
    let mut attempt_deadline = Instant::now() + per_attempt;

    loop {
        match controller.step(client) {
            StepOutcome::InProgress => {
                if Instant::now() > attempt_deadline {
                    // Name the phase and the state it is reading: a restart
                    // that never completed, an arm the DLL never processed
                    // and a judge waiting on a replay are three different bugs.
                    let st = client.state();
                    eprintln!(
                        "  ERROR: CONT attempt stalled in {} | mode={} restart_state={} playback_pos={} arm_generation={} recorded={}",
                        controller.phase_name(),
                        st.mode,
                        st.restart_state,
                        st.playback_pos,
                        st.arm_generation,
                        st.recorded_count
                    );
                    client.send_command(TasCommand::Stop);
                    return None;
                }
                thread::sleep(Duration::from_millis(5));
            }
            StepOutcome::Wait { ms } => {
                // Fixed Stop->Restart settle so the Restart fires at a
                // consistent F5 phase.
                thread::sleep(Duration::from_millis(ms));
            }
            StepOutcome::Reroll {
                attempt,
                suggested_delay_ms,
                observed,
                expected,
            } => {
                println!(
                    "  Retry {}/{}: CONT bucket reroll  observed first-moving={:?} expected={:?}",
                    attempt, max_retries, observed, expected
                );
                // Clear any save dialog that re-appeared after the Stop, then
                // jitter the wall clock so the next F5 lands at a new phase.
                dismiss_save_dialog();
                thread::sleep(Duration::from_millis(suggested_delay_ms));
                attempt_deadline = Instant::now() + per_attempt;
            }
            StepOutcome::Done { retries_used, .. } => {
                if retries_used > 0 {
                    println!(
                        "  CONT bucket accepted (shared controller) after {} reroll(s)",
                        retries_used
                    );
                }
                println!("  CONT bucket accepted, waiting for splice...");
                return if wait_continue_splice(client, splice_frame) {
                    Some(retries_used)
                } else {
                    None
                };
            }
            StepOutcome::Aborted { reason } => {
                eprintln!("  WARNING: CONT aborted: {}", reason);
                return None;
            }
        }
    }
}

/// Poll `judge_cont_bucket` until a definitive verdict, the way tas_ui's
/// per-frame guard does. Returns the first non-`KeepWaiting` verdict; on timeout
/// returns `KeepWaiting`. If the splice already fired (mode REC) or playback
/// reached the splice, judges once with the full prefix; if CONT bailed to OFF,
/// reports `WrongStart`.
fn poll_cont_verdict(
    client: &TasSharedMemoryClient,
    expected_start_bits: [u32; 3],
    expected_first_moving: Option<u32>,
    splice_frame: u32,
) -> tas_shared::cont::BucketVerdict {
    use tas_shared::cont::{judge_cont_bucket, BucketVerdict};
    let t0 = Instant::now();
    // The replay advances ~100*speed frames/sec, so a slow speed needs a
    // proportionally longer window to reach the fingerprint.
    let speed = (client.state().playback_speed as f64).max(0.05);
    let timeout = Duration::from_secs_f64(JUDGE_TIMEOUT_SECS as f64 * (1.0 / speed).max(1.0));
    loop {
        let (verdict, mode, pos) = {
            let s = client.state();
            (
                judge_cont_bucket(
                    &s.play_coords[..],
                    &s.rec_coords[..],
                    s.recorded_count,
                    s.playback_pos,
                    expected_start_bits,
                    expected_first_moving,
                    splice_frame,
                ),
                s.mode,
                s.playback_pos,
            )
        };
        if verdict != BucketVerdict::KeepWaiting {
            return verdict;
        }
        if mode == TasMode::Rec as u32 || pos >= splice_frame {
            let s = client.state();
            return judge_cont_bucket(
                &s.play_coords[..],
                &s.rec_coords[..],
                s.recorded_count,
                splice_frame.max(1),
                expected_start_bits,
                expected_first_moving,
                splice_frame,
            );
        }
        if mode == TasMode::Off as u32 {
            return BucketVerdict::WrongStart;
        }
        if t0.elapsed() > timeout {
            return BucketVerdict::KeepWaiting;
        }
        thread::sleep(Duration::from_millis(5));
    }
}

/// Drive Pico HID through a sequence of pattern steps.
///
/// `fallback_ms` is how long to sleep instead if the Pico port cannot be opened,
/// so the recording window still elapses.
pub fn drive_pico_steps(steps: &[crate::patterns::PatternStep], fallback_ms: Option<u64>) {
    let _ = drive_pico_steps_inner(steps, fallback_ms, None, false);
}

/// Acceptance must not substitute an unsteered recording for missing hardware,
/// and refreshes its long hold before the firmware watchdog releases it.
pub fn drive_pico_steps_required(steps: &[crate::patterns::PatternStep]) -> Result<(), String> {
    drive_pico_steps_inner(steps, None, Some(200), true)
}

fn drive_pico_steps_inner(
    steps: &[crate::patterns::PatternStep],
    fallback_ms: Option<u64>,
    keepalive_ms: Option<u64>,
    required: bool,
) -> Result<(), String> {
    let port_name = pico_port();
    // The firmware (TAS/pico/code.py, TIMEOUT_S) releases every key 500 ms
    // after the last byte and this loop writes only on mask changes, so long
    // holds are truncated. The regression gates are baselined with that release
    // inside the recording window: do not add a default keepalive without
    // re-baselining them. Acceptance opts in via `keepalive_ms`.
    let mut port = match PicoKeys::open_checked() {
        Ok(p) => p,
        Err(error) => {
            if required {
                return Err(error);
            }
            eprintln!(
                "  ERROR: Cannot open {}. Steering will be absent.",
                port_name
            );
            let ms = fallback_ms.unwrap_or_else(|| crate::patterns::total_ticks(steps) as u64 * 10);
            thread::sleep(Duration::from_millis(ms));
            return Ok(());
        }
    };
    let total = crate::patterns::total_ticks(steps);
    let ms_per_tick = 10u64;
    let start = Instant::now();
    let mut prev_mask = 0xFFu8;
    let mut last_send: Option<Instant> = None;
    let mut current_step = 0usize;

    for tick in 0..total {
        while current_step < steps.len() && tick >= steps[current_step].stop_tick {
            current_step += 1;
        }
        let mask = if current_step < steps.len() {
            steps[current_step].mask
        } else {
            0
        };

        let keepalive_due = mask != 0
            && keepalive_ms.is_some_and(|ms| {
                last_send.is_some_and(|sent| sent.elapsed() >= Duration::from_millis(ms))
            });
        if mask != prev_mask || keepalive_due {
            let send_byte = if mask == 0 { 0xFF } else { mask };
            let sent = port.send(send_byte);
            if required {
                require_pico_write(sent, &port_name)?;
            }
            prev_mask = mask;
            last_send = Some(Instant::now());
        }

        let target = Duration::from_millis((tick as u64 + 1) * ms_per_tick);
        if let Some(remaining) = target.checked_sub(start.elapsed()) {
            thread::sleep(remaining);
        }
    }

    // Explicit release on the normal path so the keys go up at a known instant
    // rather than whenever `port` happens to drop; Drop then makes it idempotent.
    let released = port.send(0xFF);
    if required {
        require_pico_write(released, &port_name)?;
    }
    Ok(())
}

fn require_pico_write(sent: bool, port: &str) -> Result<(), String> {
    if sent {
        Ok(())
    } else {
        Err(format!(
            "Pico {port} write failed; acceptance cannot verify steering"
        ))
    }
}

/// Require the proven zero-drift configuration before a gate runs: natural
/// ticks (`force_fixed_tick == 0`) and a 1x (or unset) playback speed. Exits 1
/// with the offending value instead of running a gate whose answer is
/// meaningless.
pub fn assert_proven_config(client: &TasSharedMemoryClient) {
    let s = client.state();
    if s.force_fixed_tick != 0 {
        eprintln!(
            "ERROR: force_fixed_tick={} — must be 0 (natural ticks)",
            s.force_fixed_tick
        );
        std::process::exit(1);
    }
    if s.playback_speed != 1.0 && s.playback_speed != 0.0 {
        eprintln!(
            "ERROR: playback_speed must be 1.0 or 0.0 (got {})",
            s.playback_speed
        );
        std::process::exit(1);
    }
    println!(
        "Config OK: fft=0, speed={} (Cave5={})",
        s.playback_speed,
        if s.cave5_hooked == 1 { "hooked" } else { "missing" }
    );
}

/// Speed modes need Cave 5 (the tick-rate hook) and natural ticks. Prints FAIL
/// and returns false instead of panicking.
pub fn require_speed_preconditions(client: &TasSharedMemoryClient) -> bool {
    let s = client.state();
    if s.cave5_hooked != 1 {
        eprintln!("FAIL: Cave 5 is not hooked — speed scaling cannot be measured");
        return false;
    }
    if s.force_fixed_tick != 0 {
        eprintln!(
            "FAIL: force_fixed_tick={} — must be 0 (fixed ticks override speed)",
            s.force_fixed_tick
        );
        return false;
    }
    true
}

/// Locate a file by its path relative to the repository root.
///
/// The binary normally runs from `TAS/target/<profile>/`, three levels below
/// the root; the cwd-relative candidate covers `just` recipes run from the root.
pub fn repo_path(relative: impl AsRef<Path>) -> Result<PathBuf, String> {
    let relative = relative.as_ref();
    let mut candidates = Vec::new();
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
    {
        candidates.push(dir.join("../../..").join(relative));
    }
    candidates.push(relative.to_path_buf());
    candidates
        .iter()
        .find(|p| p.is_file())
        .cloned()
        .ok_or_else(|| {
            format!(
                "Couldn't locate {} (tried {candidates:?})",
                relative.display()
            )
        })
}

/// Locate a committed recording under `TAS/recordings/` (also found from `TAS/`).
pub fn fixture_path(name: &str) -> Result<PathBuf, String> {
    repo_path(Path::new("TAS/recordings").join(name)).or_else(|error| {
        let from_tas = Path::new("recordings").join(name);
        from_tas.is_file().then_some(from_tas).ok_or(error)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn failed_steering_or_release_write_is_an_error() {
        assert!(super::require_pico_write(true, "test-port").is_ok());
        assert!(super::require_pico_write(false, "test-port")
            .unwrap_err()
            .contains("write failed"));
    }

    #[test]
    fn required_env_rejects_missing_and_blank() {
        // SAFETY: the test never runs harness threads.
        unsafe { std::env::remove_var("TAS_TEST_REQUIRED_ENV_PROBE") };
        let err = super::required_env("TAS_TEST_REQUIRED_ENV_PROBE").unwrap_err();
        assert!(err.contains("TAS_TEST_REQUIRED_ENV_PROBE"));
        unsafe { std::env::set_var("TAS_TEST_REQUIRED_ENV_PROBE", "  ") };
        assert!(super::required_env("TAS_TEST_REQUIRED_ENV_PROBE").is_err());
        unsafe { std::env::set_var("TAS_TEST_REQUIRED_ENV_PROBE", "x") };
        assert_eq!(
            super::required_env("TAS_TEST_REQUIRED_ENV_PROBE").unwrap(),
            "x"
        );
    }
}
