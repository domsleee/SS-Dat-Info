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
/// Replay time at 1x for the watcher's verdict: the ~3 s countdown plus
/// ALIGN_VERIFY_FRAMES at 100 ticks/s, doubled for slow frames.
const WATCH_TIMEOUT_SECS: f64 = 2.0 * (3.0 + tas_shared::align::ALIGN_VERIFY_FRAMES as f64 / 100.0);

/// Bring the game window to the front. Best effort: a missing window is left alone.
pub fn focus_game() {
    if let Some(hwnd) = win32::find_game_window() {
        win32::bring_to_front(hwnd);
    }
    thread::sleep(Duration::from_millis(200));
}

/// Send Escape through the Pico (mask bit 7), never substitute synthetic input
/// for the hardware delivery that pause/dialog tests claim to exercise.
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
        "  ERROR: Pico not available on {} — cannot send physical Escape",
        pico_port()
    );
    false
}

/// Post Enter to the game window to dismiss the post-run "Save attempt" dialog.
///
/// PostMessage needs no focus, so it reaches the modal dialog that blocks focus
/// changes. Without this the next F5 lands on the dialog and is eaten.
/// Harmless when no dialog is up.
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

/// Send a physical F5. A missing or busy Pico is a failure, not synthetic input.
pub fn send_f5_pico() -> bool {
    send_pico_key(0x40, "F5", 100)
}

/// The finish prompt ignores Escape. Select No with RIGHT, then confirm by
/// physical Enter (firmware command 0xFC), releasing both keys explicitly.
pub fn dismiss_finish_prompt() -> bool {
    send_pico_key(0x02, "RIGHT (No)", 80) && send_pico_key(0xFC, "Enter", 80)
}

fn send_pico_key(command: u8, label: &str, hold_ms: u64) -> bool {
    focus_game();
    let mut keys = match PicoKeys::open_checked() {
        Ok(keys) => keys,
        Err(error) => {
            eprintln!("ERROR: {error}");
            return false;
        }
    };
    if !keys.send(command) {
        eprintln!("ERROR: Pico {label} press failed");
        return false;
    }
    thread::sleep(Duration::from_millis(hold_ms));
    if !keys.send(0xFF) {
        eprintln!("ERROR: Pico {label} release failed");
        return false;
    }
    println!("  {label} sent via Pico ({})", keys.port_name());
    true
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
    // A relaunch re-creates the section and restarts the count, so fc2 can be
    // below fc1; that is not proof of life.
    let delta = fc2.saturating_sub(fc1);
    println!("  Liveness: {} frames/500ms", delta);
    delta > 0
}

/// Pico F5 restart + stabilization. Double-F5: the first normalizes game state,
/// the second produces the deterministic restart position.
pub fn restart_and_stabilize(client: &TasSharedMemoryClient) -> bool {
    dismiss_save_dialog();

    println!("  Sending F5 (normalize)...");
    if !send_f5_pico() {
        return false;
    }
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, 100);

    println!("  Sending F5 (restart)...");
    if !send_f5_pico() {
        return false;
    }
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
    // here: tas_ui arms immediately after the restart, and tests should arm
    // at the same point in the spawn countdown.
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

/// Restart and arm an aligned PLAY of the loaded recording WITHOUT the
/// bit-exact watcher, for callers that must stop mid-replay or record a
/// changed trajectory. Returns `(recording gate, live gate)` once the live
/// gate fires.
pub fn restart_play_aligned_unwatched(
    client: &mut TasSharedMemoryClient,
) -> Result<(u32, u32), String> {
    let rec_gate = {
        let s = client.state();
        tas_shared::align::detect_first_moving(&s.rec_coords[..], s.recorded_count)
            .ok_or("the loaded recording never leaves the spawn")?
    };
    if !restart_and_stabilize_inprocess(client) {
        return Err("in-process restart failed".into());
    }
    client.state_mut().gate_align_rec = rec_gate;
    // STOP and RESTART keep the previous session's gate and position; only the
    // arm clears them. Wait for the arm counter to move before reading either.
    let generation = client.state().arm_generation;
    arm_play(client);
    let deadline = Instant::now() + Duration::from_secs(10);
    while client.state().arm_generation == generation {
        if Instant::now() > deadline {
            return Err("the DLL never processed ARM_PLAY".into());
        }
        thread::sleep(Duration::from_millis(5));
    }
    loop {
        let play_gate = client.state().gate_index;
        if play_gate != 0 {
            return Ok((rec_gate, play_gate));
        }
        if Instant::now() > deadline {
            return Err("aligned PLAY never observed a live gate".into());
        }
        thread::sleep(Duration::from_millis(5));
    }
}

/// Drive one restart -> arm -> watch cycle through the shared transport
/// controller, the state machine tas_ui runs, until it finishes. Each attempt
/// gets a restart budget plus the watcher's replay time at `cfg.speed`,
/// renewed on every reroll. Returns the rerolls used, or None after printing
/// why the cycle failed.
fn drive_cycle(
    client: &mut TasSharedMemoryClient,
    cfg: tas_shared::transport::ArmConfig,
    label: &str,
) -> Option<u32> {
    use tas_shared::transport::{StepOutcome, TransportController};

    let mut controller = TransportController::new(cfg);
    // Keep a competing tas_ui out of the single-slot command channel, and clear
    // any post-run dialog before the first restart (it can eat the in-process F5).
    stop_competing_tas_ui_writer();
    dismiss_save_dialog();

    let replay_secs = WATCH_TIMEOUT_SECS / (cfg.speed as f64).clamp(0.05, 1.0);
    let per_attempt = Duration::from_secs_f64(RESTART_TIMEOUT_SECS as f64 + replay_secs);
    let mut attempt_deadline = Instant::now() + per_attempt;
    loop {
        match controller.step(client) {
            StepOutcome::InProgress => {
                if Instant::now() > attempt_deadline {
                    // Name the phase and the state it is reading: a restart
                    // that never completed, an arm the DLL never processed and
                    // a watcher waiting on a replay are different bugs.
                    let st = client.state();
                    eprintln!(
                        "  ERROR: {} attempt stalled in {} | mode={} restart_state={} playback_pos={} arm_generation={} recorded={}",
                        label,
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
            StepOutcome::Wait { ms } => thread::sleep(Duration::from_millis(ms)),
            StepOutcome::Reroll { attempt, observed } => {
                println!(
                    "  Retry {}/{}: {} watcher reroll (first mismatch at gate+{:?})",
                    attempt, cfg.max_retries, label, observed
                );
                // Clear any save dialog that re-appeared after the Stop.
                dismiss_save_dialog();
                attempt_deadline = Instant::now() + per_attempt;
            }
            StepOutcome::Done { retries_used, .. } => return Some(retries_used),
            StepOutcome::Aborted { reason } => {
                eprintln!("  ERROR: {} aborted: {}", label, reason);
                stop(client);
                return None;
            }
        }
    }
}

/// Restart and arm the product PLAY path with input indexed relative to the
/// recording's gate. Returns `(recorded_gate, live_gate)` once the
/// gate-relative trajectory watcher accepts the attempt.
pub fn restart_play_aligned_inprocess(client: &mut TasSharedMemoryClient) -> Option<(u32, u32)> {
    use tas_shared::transport::{Arm, ArmConfig};

    let rec_gate = {
        let s = client.state();
        tas_shared::align::detect_first_moving(&s.rec_coords[..], s.recorded_count)?
    };
    let cfg = ArmConfig {
        arm: Arm::Play,
        speed: client.state().playback_speed.max(1.0),
        continue_from_frame: 0,
        gate_align_rec: rec_gate,
        max_retries: tas_shared::align::ALIGN_MAX_RETRIES,
    };
    let retries_used = drive_cycle(client, cfg, "PLAY")?;
    // A match requires the live gate, so it is set by now.
    let gate = client.state().gate_index;
    println!(
        "  Aligned PLAY watcher accepted after {} retr{}: recording gate {}, live gate {} (offset {:+})",
        retries_used,
        if retries_used == 1 { "y" } else { "ies" },
        rec_gate,
        gate,
        gate as i64 - rec_gate as i64
    );
    Some((rec_gate, gate))
}

/// An environment variable the live launch path needs; no built-in default.
pub(crate) fn required_env(name: &str) -> Result<String, String> {
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
    let suite_pid = std::env::var("TAS_SUITE_PID")
        .ok()
        .and_then(|s| s.parse().ok());
    command.args(kill_arguments(image, exclude_pid, suite_pid));
    hide_console(&mut command);
    match command.output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|line| line.contains("PID"))
            .count() as u32,
        Err(_) => 0,
    }
}

fn kill_arguments(image: &str, me: Option<u32>, suite: Option<u32>) -> Vec<String> {
    let mut args: Vec<String> = [
        "/F",
        "/IM",
        image,
        "/FI",
        "IMAGENAME ne supreme-service.exe",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    for pid in me.into_iter().chain(suite) {
        args.extend(["/FI".into(), format!("PID ne {pid}")]);
    }
    args
}

/// Kill every game, launcher and TAS process except ourselves and our suite (the same list
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
    // The CONT interlock and gate-alignment fields: what a controller left
    // behind (suppressed input, an unapproved parked splice) is otherwise
    // invisible from outside the UI process.
    let last_input = s
        .recorded_count
        .checked_sub(1)
        .and_then(|i| s.input_log.get(i as usize))
        .copied();
    println!(
        "Cont: from={} suppress_input={} splice_approved={} gate_align_rec={} gate_index={} \
         arm_gen={} restart_state={} speed={} resume_speed={} last_input={} \
         bb3b10_calls={} bb3b10_blocked={} handler_blocked={}",
        s.continue_from_frame,
        s.cont_suppress_input,
        s.cont_splice_approved,
        s.gate_align_rec,
        s.gate_index,
        s.arm_generation,
        s.restart_state,
        s.playback_speed,
        s.cont_resume_speed,
        last_input.map_or("-".to_string(), |b| format!("{b:#04x}")),
        s.bb3b10_call_count,
        s.bb3b10_block_count,
        s.handler_block_count
    );
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

/// In-process CONT splice through the shared transport controller. The
/// controller finishes when the aligned prefix is accepted (before the splice
/// fires); then this waits for the PLAY->REC splice. Returns
/// `Some(reroll_count)` on a clean splice (0 = accepted first try), `None` on
/// failure.
pub fn restart_continue_and_splice_inprocess(
    client: &mut TasSharedMemoryClient,
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    use tas_shared::transport::{Arm, ArmConfig};

    let gate_align_rec = {
        let s = client.state();
        tas_shared::align::detect_first_moving(&s.rec_coords[..], s.recorded_count).unwrap_or(0)
    };
    println!("  CONT aligned on recording gate {}", gate_align_rec);
    let cfg = ArmConfig {
        arm: Arm::Continue,
        speed: client.state().playback_speed,
        continue_from_frame: splice_frame,
        gate_align_rec,
        max_retries,
    };
    let retries_used = drive_cycle(client, cfg, "CONT")?;
    println!(
        "  CONT prefix accepted after {} reroll(s), waiting for splice...",
        retries_used
    );
    wait_continue_splice(client, splice_frame).then_some(retries_used)
}

/// Refresh held keys before the firmware's safety timeout. Timeout behavior is
/// tested separately by the firmware tests, never implicitly inside a pattern.
pub fn drive_pico_steps(steps: &[crate::patterns::PatternStep]) -> Result<(), String> {
    let port_name = pico_port();
    let mut port = PicoKeys::open_checked()?;
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

        let keepalive_due =
            mask != 0 && last_send.is_some_and(|sent| sent.elapsed() >= Duration::from_millis(200));
        if mask != prev_mask || keepalive_due {
            let send_byte = if mask == 0 { 0xFF } else { mask };
            let sent = port.send(send_byte);
            require_pico_write(sent, &port_name)?;
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
    require_pico_write(released, &port_name)?;
    Ok(())
}

fn require_pico_write(sent: bool, port: &str) -> Result<(), String> {
    if sent {
        Ok(())
    } else {
        Err(format!("Pico {port} write failed; cannot verify steering"))
    }
}

/// Require a 1x (or unset) playback speed before a gate runs. Exits 1 with the
/// offending value instead of running a gate whose answer is meaningless.
pub fn assert_proven_config(client: &TasSharedMemoryClient) {
    let s = client.state();
    if s.playback_speed != 1.0 && s.playback_speed != 0.0 {
        eprintln!(
            "ERROR: playback_speed must be 1.0 or 0.0 (got {})",
            s.playback_speed
        );
        std::process::exit(1);
    }
    println!(
        "Config OK: speed={} (Cave5={})",
        s.playback_speed,
        if s.cave5_hooked == 1 {
            "hooked"
        } else {
            "missing"
        }
    );
}

/// Speed modes need Cave 5 (the tick-rate hook). Prints FAIL and returns false
/// instead of panicking.
pub fn require_speed_preconditions(client: &TasSharedMemoryClient) -> bool {
    if client.state().cave5_hooked != 1 {
        eprintln!("FAIL: Cave 5 is not hooked — speed scaling cannot be measured");
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
    fn game_reset_excludes_both_child_and_suite_parent() {
        let args = super::kill_arguments("tas_test.exe", Some(123), Some(456));
        for filter in [
            "PID ne 123",
            "PID ne 456",
            "IMAGENAME ne supreme-service.exe",
        ] {
            assert!(args.windows(2).any(|pair| pair == ["/FI", filter]));
        }
        assert!(!args.iter().any(|a| a == "/T"));
    }
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
