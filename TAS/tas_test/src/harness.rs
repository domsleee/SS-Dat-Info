//! Shared test harness scaffolding — F5 restart, wait helpers, liveness checks.
//!
//! Ported from _full_drift_test.lua and the existing main.rs helpers.

use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

/// Pico HID COM port. Override with `TAS_PICO_PORT` env var (default: COM7).
pub fn pico_port() -> String {
    std::env::var("TAS_PICO_PORT").unwrap_or_else(|_| "COM7".into())
}

/// How long to wait after F5 for the game to restart loading.
const F5_SETTLE_MS: u64 = 4000;
/// Frames to wait for physics stabilization after restart.
const STABILIZE_FRAMES: u32 = 500;
/// Timeout waiting for in-process restart state machine to finish.
const RESTART_TIMEOUT_SECS: u64 = 15;
/// Playback timeout (long enough for 65536 frames at ~50fps unfocused).
const PLAYBACK_TIMEOUT_SECS: u64 = 120;
/// Early CONT anchor frame used to reject countdown/start-phase mismatches.
const CONT_ANCHOR_FRAME: u32 = 250;
/// Timeout waiting to reach the early CONT anchor frame.
const CONT_ANCHOR_TIMEOUT_SECS: u64 = 10;
/// Default retry budget for finding a matching F5 starting bucket. Acceptance
/// observed up to 13 retries in 15 trials; 60 covers the observed tail with
/// margin and is the standard regression suite budget.
pub const START_MATCH_RETRIES: u32 = 60;

/// Focus the Supreme Snowboarding window.
pub fn focus_game() {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"
            $wshell = New-Object -ComObject wscript.shell
            $procs = Get-Process Supreme* -ErrorAction SilentlyContinue
            if ($procs) { $wshell.AppActivate($procs[0].Id) | Out-Null }
        "#,
        ])
        .output();
    thread::sleep(Duration::from_millis(200));
}

/// Send Escape to the game via the Pico HID (bit 7 in the input-mask
/// protocol — added to firmware alongside the existing LRUD/JUMP/SHIFT/F5
/// bits). The Pico is a real USB HID keyboard so the keystroke updates the
/// OS keyboard state and reaches Supreme's pause handler. The fallback
/// keybd_event path is kept for development environments without the Pico,
/// but synthetic input does not engage the game's pause and the
/// `escape-speedup` test will report "pause didn't engage" in that mode.
pub fn send_escape() -> bool {
    focus_game();
    let port_name = pico_port();
    let com_path = format!("\\\\.\\{}", port_name);
    if let Ok(mut p) = std::fs::OpenOptions::new().write(true).open(&com_path) {
        // Bit 7 = Escape per the updated Pico firmware (BIT_TO_KEY[7] = Keycode.ESCAPE).
        let _ = p.write_all(&[0x80]);
        let _ = p.flush();
        thread::sleep(Duration::from_millis(80));
        let _ = p.write_all(&[0xFF]); // release all
        let _ = p.flush();
        println!("  Escape sent via Pico ({})", port_name);
        return true;
    }

    // Fallback for machines without the Pico: synthetic key event. The game
    // typically ignores this (game polls hardware state via DirectInput),
    // but it's better than nothing.
    eprintln!(
        "  WARNING: Pico not available on {} — falling back to keybd_event (likely won't pause the game)",
        port_name
    );
    let script = r#"
Add-Type @'
using System; using System.Runtime.InteropServices;
public class K { [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra); }
'@
$VK_ESCAPE = 0x1B
$SCAN_ESCAPE = 0x01
$KEYEVENTF_EXTENDEDKEY = 0x0001
$KEYEVENTF_KEYUP = 0x0002
$KEYEVENTF_SCANCODE = 0x0008
[K]::keybd_event($VK_ESCAPE, $SCAN_ESCAPE, $KEYEVENTF_EXTENDEDKEY -bor $KEYEVENTF_SCANCODE, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 80
[K]::keybd_event($VK_ESCAPE, $SCAN_ESCAPE, $KEYEVENTF_EXTENDEDKEY -bor $KEYEVENTF_KEYUP -bor $KEYEVENTF_SCANCODE, [UIntPtr]::Zero)
"#;
    Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
        .is_ok()
}

/// Send Enter to the game to dismiss the post-run "Save attempt" dialog.
///
/// The dialog only appears after some run completions, but Enter is harmless
/// when it isn't up. Without dismissing it, the next F5 lands on the dialog
/// and gets eaten instead of triggering a real restart, which shifts the F5
/// match buckets by one cycle and breaks playback start matching.
///
/// Uses PostMessage to the game's main HWND rather than SendKeys, since
/// SendKeys depends on focus restoration via WScript.Shell.AppActivate which
/// silently fails when the game has a child modal dialog up (the very state
/// we're trying to dismiss). PostMessage delivers the keystroke directly to
/// the target window regardless of focus.
fn dismiss_save_dialog() {
    if let Some(hwnd) = find_supreme_hwnd() {
        #[allow(non_snake_case)]
        let WM_KEYDOWN: u32 = 0x0100;
        #[allow(non_snake_case)]
        let WM_KEYUP: u32 = 0x0101;
        #[allow(non_snake_case)]
        let VK_RETURN: usize = 0x0D;
        let script = format!(
            r#"
Add-Type @'
using System; using System.Runtime.InteropServices;
public class W {{ [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint m, IntPtr w, IntPtr l); }}
'@
[W]::PostMessage([IntPtr]::new({h}), {kd}, [IntPtr]::new({vk}), [IntPtr]::Zero) | Out-Null
Start-Sleep -Milliseconds 30
[W]::PostMessage([IntPtr]::new({h}), {ku}, [IntPtr]::new({vk}), [IntPtr]::Zero) | Out-Null
"#,
            h = hwnd,
            kd = WM_KEYDOWN,
            ku = WM_KEYUP,
            vk = VK_RETURN,
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output();
    }
    thread::sleep(Duration::from_millis(150));
}

/// Ensure there is no competing shared-memory writer (`tas_ui`) while running
/// deterministic `tas_test` runtime flows.
///
/// `tas_ui` and `tas_test` concurrently writing command fields can cause
/// intermittent ARM_CONTINUE mode=0 failures unrelated to core replay logic.
pub fn stop_competing_tas_ui_writer() -> u32 {
    let script = r#"
        $p = Get-Process -Name tas_ui -ErrorAction SilentlyContinue
        if ($p) {
            $count = @($p).Count
            $p | Stop-Process -Force
            Write-Output ("killed:{0}" -f $count)
        } else {
            Write-Output "none"
        }
    "#;
    if let Ok(output) = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
    {
        let out = String::from_utf8_lossy(&output.stdout);
        let msg = out.trim();
        if let Some(count) = msg.strip_prefix("killed:") {
            println!("  Stopped competing tas_ui writer(s): {}", count.trim());
            return count.trim().parse::<u32>().unwrap_or(1);
        }
    }
    0
}

/// Ensure `tas_test` owns the shared-memory command surface before running
/// speed-sensitive runtime checks.
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

/// Find the Supreme window handle via PowerShell.
fn find_supreme_hwnd() -> Option<isize> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"(Get-Process Supreme -EA SilentlyContinue | Where {$_.Name -notmatch 'service'} | Select -First 1).MainWindowHandle"#,
        ])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout);
    s.trim().parse::<isize>().ok().filter(|&h| h != 0)
}

/// Send F5 via PostMessage (no focus required).
fn send_f5_postmessage() {
    if let Some(hwnd) = find_supreme_hwnd() {
        #[allow(non_snake_case)]
        let WM_KEYDOWN: u32 = 0x0100;
        #[allow(non_snake_case)]
        let WM_KEYUP: u32 = 0x0101;
        #[allow(non_snake_case)]
        let VK_F5: usize = 0x74;
        // Use PowerShell to call PostMessage since we can't link user32 from pure Rust easily
        let script = format!(
            r#"
            Add-Type @'
using System; using System.Runtime.InteropServices;
public class W {{ [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint m, IntPtr w, IntPtr l); }}
'@
            [W]::PostMessage([IntPtr]::new({}), {}, [IntPtr]::new({}), [IntPtr]::Zero) | Out-Null
            Start-Sleep -Milliseconds 50
            [W]::PostMessage([IntPtr]::new({}), {}, [IntPtr]::new({}), [IntPtr]::Zero) | Out-Null
            "#,
            hwnd, WM_KEYDOWN, VK_F5, hwnd, WM_KEYUP, VK_F5,
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .output();
        println!("  F5 sent via PostMessage (hwnd={})", hwnd);
    } else {
        eprintln!("  ERROR: Cannot find Supreme window for PostMessage F5");
    }
}

/// Send F5 via Pico HID to restart the race.
pub fn send_f5_pico() {
    focus_game();
    let port = pico_port();
    let com_path = format!("\\\\.\\{}", port);
    match std::fs::OpenOptions::new().write(true).open(&com_path) {
        Ok(mut p) => {
            let _ = p.write_all(&[0x40]); // F5 press (bit 6)
            let _ = p.flush();
            thread::sleep(Duration::from_millis(100));
            let _ = p.write_all(&[0xFF]); // release all
            let _ = p.flush();
            println!("  F5 sent via Pico ({})", port);
        }
        Err(e) => {
            eprintln!(
                "  WARNING: Failed to open {}: {}. Using PostMessage fallback.",
                port, e
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

/// F5 restart + wait for stabilization. Returns true if successful.
/// Uses double-F5: first F5 normalizes game state, second F5 produces
/// the deterministic restart position. This ensures consistent starting
/// positions regardless of what the game was doing before.
pub fn restart_and_stabilize(client: &TasSharedMemoryClient) -> bool {
    // Dismiss any post-run "Save attempt" dialog before F5 — otherwise the
    // first F5 lands on the dialog and gets eaten, shifting bucket alignment.
    dismiss_save_dialog();

    // First F5: normalize game state
    println!("  Sending F5 (normalize)...");
    send_f5_pico();
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, 100); // brief wait for game to restart

    // Second F5: deterministic restart from normalized state
    println!("  Sending F5 (restart)...");
    send_f5_pico();
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, STABILIZE_FRAMES);
    println!("  Stabilized at frame {}", client.frame_count_volatile());
    check_liveness(client)
}

/// In-process restart via CMD_RESTART + restart_state polling.
///
/// This matches the egui transport path (RestartThen) and avoids external
/// focus/Pico timing variance from out-of-process F5 injection.
pub fn restart_and_stabilize_inprocess(client: &mut TasSharedMemoryClient) -> bool {
    // Dismiss any post-run "Save attempt" dialog before requesting restart.
    // The in-process restart eventually invokes the game's F5 handler internally,
    // which can also be intercepted by the dialog if it's up.
    dismiss_save_dialog();

    // Ensure a clean command state before requesting restart.
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(50));

    client.reset_restart_state();
    client.send_command(TasCommand::Restart);

    let start = Instant::now();
    loop {
        let rs = client.restart_state();
        if rs == 2 {
            break;
        }
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
    println!(
        "  In-process restart stabilized at frame {}",
        client.frame_count_volatile()
    );
    // restart_state==2 is set BY cave2's restart machine, which only runs inside
    // the Supreme::Cycle hook — so reaching it already proves the game is alive
    // and cave2 is firing. Do NOT call check_liveness here: its 500ms sleep
    // burned ~50 frames of the 3s spawn countdown before the caller could arm,
    // which is why CONT/REC captured play_coords[0] ~90 frames late (first-moving
    // ~211 instead of ~300) and could never match recordings made by tas_ui
    // (which arms immediately after the restart). Arm ASAP, like tas_ui.
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

/// Path to the `revive-supreme.nu` script used by [`ensure_game_running`].
/// Override with the `REVIVE_SUPREME_SCRIPT` env var.
fn revive_script_path() -> String {
    std::env::var("REVIVE_SUPREME_SCRIPT").unwrap_or_else(|_| {
        r"C:\Users\user\git\cheatengine-mcp-bridge\skills\revive-supreme\scripts\revive-supreme.nu"
            .into()
    })
}

/// Launch Supreme Snowboarding via `revive-supreme.nu` with `NO_CE=1`.
/// Returns true if the script exited successfully.
///
/// Sets `TAS_TEST_PID` so the revive script can skip killing us.
fn run_revive() -> bool {
    let script = revive_script_path();
    println!("Launching game via revive-supreme (NO_CE=1)...");
    println!("  Script: {}", script);
    match Command::new("nu")
        .arg(&script)
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

/// Kill all running instances of Supreme Snowboarding and related processes.
/// Excludes the current process so we don't kill ourselves.
fn kill_game() {
    let my_pid = std::process::id();
    let script = format!(
        r#"
        $names = @('Supreme','Supreme_v1.035','display-config','Display_Config','tas_ui','tas_test')
        $myPid = {}
        foreach ($n in $names) {{
            Get-Process -Name $n -ErrorAction SilentlyContinue |
                Where-Object {{ $_.Id -ne $myPid }} |
                Stop-Process -Force
        }}
        "#,
        my_pid
    );
    let _ = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output();
    thread::sleep(Duration::from_millis(500));
}

/// Default game folder. Override with `SUPREME_FOLDER` env var.
fn supreme_folder() -> String {
    std::env::var("SUPREME_FOLDER").unwrap_or_else(|_| r"T:\Games\SupremeORIG".into())
}

/// Inject a DLL into the running Supreme.exe process via Injector.exe.
fn inject_dll(label: &str, dll_path: &str) -> bool {
    let base = supreme_folder();
    let injector = format!(r"{}\Display_Config_Resources\Injector.exe", base);

    println!("Injecting {}...", label);
    println!("  DLL: {}", dll_path);

    match Command::new(&injector)
        .arg(dll_path)
        .status()
    {
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

/// Inject Display_Config_Helper.dll then TAS_Helper.dll into the running game.
fn inject_all_dlls() -> bool {
    let base = supreme_folder();
    let dc_res = format!(r"{}\Display_Config_Resources", base);
    let dc_helper = format!(r"{}\Display_Config_Helper.dll", dc_res);
    let tas_helper = format!(r"{}\TAS\TAS_Helper.dll", dc_res);

    if !inject_dll("Display_Config_Helper.dll", &dc_helper) {
        return false;
    }
    // Give Display_Config_Helper time to initialize rendering hooks.
    thread::sleep(Duration::from_secs(2));

    inject_dll("TAS_Helper.dll", &tas_helper)
}

/// Check whether revive is suppressed via `NO_REVIVE=1` env var.
fn no_revive() -> bool {
    std::env::var("NO_REVIVE").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Ensure the game is running with hooks active.
///
/// Default behaviour: if the game is already live, reuse it. If not, kill
/// stale instances and run `revive-supreme.nu` to get a clean session.
///
/// Set `NO_REVIVE=1` to skip the automatic launch — `ensure_game_running`
/// will then behave like the old `connect()` and exit if the game isn't up.
pub fn ensure_game_running() -> TasSharedMemoryClient {
    // Fast path: game already up and hooks firing.
    if let Ok(c) = TasSharedMemoryClient::open() {
        if check_liveness(&c) {
            let s = c.state();
            println!(
                "Game already live (version {}). Hooks: cave2={} cave1c={} cave1d={} cave5={}",
                s.version, s.cave2_hooked, s.cave1c_hooked, s.cave1d_hooked, s.cave5_hooked
            );
            return c;
        }
    }

    if no_revive() {
        eprintln!("ERROR: Game not running and NO_REVIVE=1 is set");
        std::process::exit(1);
    }

    println!("Game not live — launching fresh via revive-supreme...");
    kill_game();

    if !run_revive() {
        eprintln!("ERROR: revive-supreme failed");
        std::process::exit(1);
    }

    // revive-supreme launches Supreme.exe via Display_Config flow which injects
    // Display_Config_Helper.dll automatically. We still need to inject TAS_Helper.dll.
    thread::sleep(Duration::from_secs(2));

    let base = supreme_folder();
    let tas_helper = format!(r"{}\Display_Config_Resources\TAS\TAS_Helper.dll", base);
    if !inject_dll("TAS_Helper.dll", &tas_helper) {
        eprintln!("ERROR: TAS_Helper.dll injection failed");
        std::process::exit(1);
    }

    // Wait for TAS hooks to stabilize.
    thread::sleep(Duration::from_secs(3));

    match TasSharedMemoryClient::open() {
        Ok(c) if check_liveness(&c) => {
            let s = c.state();
            println!(
                "Connected after revive (version {}). Hooks: cave2={} cave1c={} cave1d={} cave5={}",
                s.version, s.cave2_hooked, s.cave1c_hooked, s.cave1d_hooked, s.cave5_hooked
            );
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
        "Config: inject_mode={} fft={} force_direct={}",
        s.inject_mode, s.force_fixed_tick, s.force_direct
    );
    println!(
        "State: mode={} recorded={} playback_pos={}",
        s.mode_str(),
        s.recorded_count,
        s.playback_pos
    );
}

/// Read current live player position from shared state.
pub fn get_position(client: &TasSharedMemoryClient) -> [f32; 3] {
    let s = client.state();
    [s.player_x, s.player_y, s.player_z]
}

/// F5 restart + stabilize, then start PLAY and check if play_coords[0] matches target.
/// No position forcing — relies on F5 producing deterministic restart positions.
/// F5 produces ~3 quantized positions; retries until one matches naturally.
/// Returns true if playback is running with correct start position.
pub fn restart_play_and_match(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    restart_play_and_match_with(client, target, max_retries, |c| restart_and_stabilize(c))
}

/// In-process restart variant of PLAY start matching.
///
/// Use this for file-backed replay flows so the restart semantics match the
/// current egui transport path and file-backed CONT validation.
pub fn restart_play_and_match_inprocess(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    restart_play_and_match_with(client, target, max_retries, |c| {
        restart_and_stabilize_inprocess(c)
    })
}

/// Number of leading PLAY frames whose positions must bit-identically match
/// the recording's rec_coords for a start match to be accepted.
///
/// Must be large enough to traverse any stationary phase at the start of a
/// recording (e.g. a race-start countdown that's stationary for 200-300
/// frames before motion begins). Recordings whose first N frames are
/// stationary can trivially "match" any F5 bucket on a 10-frame check while
/// having a totally wrong rotation/velocity — divergence only appears once
/// physics activates and the recorded inputs start steering. 1000 frames
/// covers all realistic countdowns plus several seconds of active gameplay,
/// so any rotation/velocity mismatch surfaces inside the verification
/// window. For F5-spawn recordings (acceptance/regression/reliability) the
/// match passes within a few frames; for stationary-start recordings
/// (load-from-disk in tas_ui) the snowboarder begins moving inside this
/// window and the recorded inputs immediately surface any mismatch.
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
        // Wait until playback_pos reaches MATCH_VERIFY_FRAMES (or end of
        // recording). At 1x speed this is ~10s for 1000 frames; we poll
        // rather than fixed-sleep so faster playback speeds finish sooner.
        // Cap at 20s wall to bound retries.
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

        // Frame-0 position matched. Verify the next few frames also match the
        // recording — same position + same inputs MUST produce same trajectory
        // under deterministic physics. If rotation/velocity at the match
        // moment is wrong, recorded inputs steer the snowboarder onto a
        // divergent trajectory immediately. This catches mid-run recording
        // rotation mismatches without needing a separate rotation_matrix
        // field in the recording metadata.
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
            if p[0].to_bits() != r[0].to_bits()
                || p[1].to_bits() != r[1].to_bits()
                || p[2].to_bits() != r[2].to_bits()
            {
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
        "Max drift Z: {:.9} (frame {})",
        drift.max_drift_z, drift.max_drift_frame_z
    );
    println!("BB3B10 calls: {}", s.bb3b10_call_count);
    println!("Handler blocks (Cave 1C): {}", s.handler_block_count);
    println!("BB3B10 blocks (Cave 1D): {}", s.bb3b10_block_count);
}

/// Write a synthetic input log directly into shared memory.
///
/// Used by the benchmark mode to drive a deterministic input pattern without a Pico,
/// since perf timing doesn't require a real-input oracle.
pub fn write_synthetic_input(client: &mut TasSharedMemoryClient, input_log: &[u8]) {
    let state = client.state_mut();
    let len = input_log.len().min(state.input_log.len());
    state.input_log[..len].copy_from_slice(&input_log[..len]);
    state.recorded_count = len as u32;
}

/// Arm continue-from-frame: set splice point and send ARM_CONTINUE.
/// DLL will PLAY 0..frame, then auto-switch to REC.
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

/// Wait for mode to transition from PLAY to REC (ARM_CONTINUE splice).
/// Returns true if the transition happened within timeout.
pub fn wait_continue_splice(client: &TasSharedMemoryClient, splice_frame: u32) -> bool {
    let start = Instant::now();
    loop {
        // Poll fast (< one 1x tick = 10ms) so the splice is detected within a
        // tick of firing. At 20ms the post-splice "overshoot" read 0-2 purely
        // because the resumed REC advanced 0-2 ticks at the resume speed before
        // the poll caught it — a measurement artifact, not a real overshoot.
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

/// Wait until CONT playback reaches an early anchor frame while still in PLAY mode.
/// Returns false if mode exits to OFF/REC too early or timeout is hit.
fn wait_continue_anchor(client: &TasSharedMemoryClient, anchor_frame: u32) -> bool {
    let start = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(20));
        let s = client.state();
        if s.mode == TasMode::Off as u32 {
            eprintln!(
                "  ERROR: Mode went to OFF before CONT anchor frame {}",
                anchor_frame
            );
            return false;
        }
        if s.mode == TasMode::Rec as u32 {
            // For short splices, mode may flip PLAY->REC before this poll observes
            // playback_pos > anchor. Accept if we already crossed the anchor frame.
            if s.playback_pos >= anchor_frame {
                return true;
            }
            eprintln!(
                "  ERROR: Entered REC too early before CONT anchor frame {}",
                anchor_frame
            );
            return false;
        }
        if s.playback_pos > anchor_frame {
            return true;
        }
        if start.elapsed() > Duration::from_secs(CONT_ANCHOR_TIMEOUT_SECS) {
            eprintln!(
                "  ERROR: CONT anchor timeout at playback_pos={}/{}",
                s.playback_pos, anchor_frame
            );
            return false;
        }
    }
}

/// F5 restart + ARM_CONTINUE with position matching, then wait for splice.
///
/// Same as restart_play_and_match but uses ARM_CONTINUE instead of ARM_PLAY.
/// On position match, waits for PLAY→REC splice at splice_frame.
///
/// Returns `Some(reroll_count)` if CONT spliced (Pico-F5 restart path), else
/// `None`. Used for the cont-stress A/B vs the in-process restart.
pub fn restart_continue_and_splice(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    restart_continue_and_splice_with(client, target, splice_frame, max_retries, |c| {
        restart_and_stabilize(c)
    })
}

/// In-process-restart CONT via the SAME poll loop as the Pico path — used only
/// for the cont-stress A/B so the restart MECHANISM is the only variable (the
/// production path uses `restart_continue_and_splice_inprocess`, the controller).
pub fn restart_continue_and_splice_inprocess_loop(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    restart_continue_and_splice_with(client, target, splice_frame, max_retries, |c| {
        restart_and_stabilize_inprocess(c)
    })
}

/// In-process CONT splice via the SHARED transport controller
/// (`tas_shared::transport`) — the exact restart/arm/reroll state machine tas_ui
/// drives. The harness just steps the controller in a poll loop and applies its
/// suggested reroll delays, so this test reflects precisely what the app does
/// (one source of truth — they can't drift).
///
/// The controller reaches `Done` when the F5 bucket is accepted (at the
/// first-moving fingerprint, BEFORE the splice fires); we then wait for the
/// PLAY→REC splice at `splice_frame`. Returns `Some(reroll_count)` on a clean
/// splice (0 = landed the bucket first try), `None` on failure.
pub fn restart_continue_and_splice_inprocess(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> Option<u32> {
    use tas_shared::transport::{Arm, ArmConfig, BucketTarget, StepOutcome, TransportController};

    let expected_start_bits = [target[0].to_bits(), target[1].to_bits(), target[2].to_bits()];
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
    let cfg = ArmConfig {
        arm: Arm::Continue,
        catchup_speed,
        continue_from_frame: splice_frame,
        target: Some(BucketTarget {
            expected_start_bits,
            expected_first_moving,
        }),
        max_retries,
    };
    let mut controller = TransportController::new(cfg);

    // Keep a competing tas_ui process out of the single-slot command channel,
    // and clear any post-run "Save attempt" dialog before the first restart
    // (it can eat the in-process F5).
    stop_competing_tas_ui_writer();
    dismiss_save_dialog();

    let speed = (catchup_speed as f64).max(0.05);
    // Per-attempt budget: restart handshake + replay-to-judge at this speed.
    let per_attempt = Duration::from_secs_f64(
        RESTART_TIMEOUT_SECS as f64 + (CONT_ANCHOR_TIMEOUT_SECS as f64) * (1.0 / speed).max(1.0),
    );
    let mut attempt_deadline = Instant::now() + per_attempt;

    loop {
        match controller.step(client) {
            StepOutcome::InProgress => {
                if Instant::now() > attempt_deadline {
                    eprintln!("  ERROR: CONT attempt stalled (no progress within budget)");
                    client.send_command(TasCommand::Stop);
                    return None;
                }
                thread::sleep(Duration::from_millis(5));
            }
            StepOutcome::Wait { ms } => {
                // Fixed Stop→Restart settle — sleep exactly this long so the
                // Restart fires at a consistent F5 phase.
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

fn restart_continue_and_splice_with<F>(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
    mut restart_fn: F,
) -> Option<u32>
where
    F: FnMut(&mut TasSharedMemoryClient) -> bool,
{
    // Expected bucket signature from the loaded recording (shared-memory
    // rec_coords), computed once. Same inputs tas_ui's set_continue_start_guard
    // captures: the recording's start bits + first-moving frame.
    let expected_start_bits = [target[0].to_bits(), target[1].to_bits(), target[2].to_bits()];
    let expected_first_moving = {
        let s = client.state();
        tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count)
    };
    match expected_first_moving {
        Some(fm) => println!("  CONT bucket criteria (tas_ui): spawn match + first-moving frame {}", fm),
        None => println!("  CONT bucket criteria (tas_ui): recording never moves — spawn match only"),
    }

    for attempt in 0..=max_retries {
        stop_competing_tas_ui_writer();
        if attempt > 0 {
            println!(
                "  Retry {}/{}: CONT bucket reroll",
                attempt, max_retries
            );
        }
        if !restart_fn(client) {
            eprintln!("  ERROR: Game not alive after restart");
            return None;
        }
        // No focus_game() here: in-process injection writes game memory directly
        // (no window focus needed), and its 200ms sleep was ~20 more frames of
        // the spawn countdown lost before arming. Arm immediately after restart,
        // matching tas_ui's CONT path, so play_coords[0] captures the countdown
        // from its start and first-moving lines up with recordings.

        // ARM_CONTINUE: starts as PLAY from tick 0, will auto-switch to REC at splice_frame
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

        // Judge the CONT bucket with EXACTLY tas_ui's criteria (shared
        // tas_shared::cont::judge_cont_bucket): spawn-bits match + first-moving
        // frame fingerprint — NOT the old harness-only bit-exact anchor@250,
        // which rerolled until a stricter bucket tas_ui never has to land on.
        // We judge EARLY (before the splice fires) so a wrong bucket is STOPped
        // before it can mutate the recording, and the reroll is fast — same as
        // tas_ui's poll_continue_start_guard. Drift over the accepted bucket is
        // checked by the caller, so the test measures the drift the USER sees.
        let verdict = poll_cont_verdict(client, expected_start_bits, expected_first_moving, splice_frame);
        use tas_shared::cont::BucketVerdict;
        match verdict {
            BucketVerdict::Match | BucketVerdict::NoSignal => {
                if attempt > 0 {
                    println!("  CONT bucket accepted (tas_ui criteria) on attempt {}", attempt + 1);
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

/// Poll `tas_shared::cont::judge_cont_bucket` until a definitive verdict,
/// mirroring tas_ui's per-frame `poll_continue_start_guard`. Returns the first
/// non-`KeepWaiting` verdict; on timeout returns `KeepWaiting`. If the splice
/// already fired (mode REC) or playback reached the splice, judges once with the
/// full prefix; if CONT bailed to OFF, reports `WrongStart`.
fn poll_cont_verdict(
    client: &TasSharedMemoryClient,
    expected_start_bits: [u32; 3],
    expected_first_moving: Option<u32>,
    splice_frame: u32,
) -> tas_shared::cont::BucketVerdict {
    use tas_shared::cont::{judge_cont_bucket, BucketVerdict};
    let t0 = Instant::now();
    // The replay advances playback_pos at ~100*speed frames/sec, so a slow speed
    // (e.g. 0.25x) needs a proportionally longer window to reach the bucket
    // fingerprint. Scale the base timeout by 1/speed.
    let speed = (client.state().playback_speed as f64).max(0.05);
    let timeout = Duration::from_secs_f64(CONT_ANCHOR_TIMEOUT_SECS as f64 * (1.0 / speed).max(1.0));
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
/// `fallback_ms` is the sleep duration if the Pico port cannot be opened
/// (allows the test to wait for the equivalent recording duration).
pub fn drive_pico_steps(steps: &[crate::patterns::PatternStep], fallback_ms: Option<u64>) {
    let port_name = pico_port();
    let com_path = format!("\\\\.\\{}", port_name);
    let port = match std::fs::OpenOptions::new().write(true).open(&com_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "  ERROR: Cannot open {}: {}. Steering will be absent.",
                port_name, e
            );
            let ms = fallback_ms.unwrap_or_else(|| crate::patterns::total_ticks(steps) as u64 * 10);
            thread::sleep(Duration::from_millis(ms));
            return;
        }
    };

    let mut port = port;
    let total = crate::patterns::total_ticks(steps);
    let ms_per_tick = 10u64;
    let start = Instant::now();
    let mut prev_mask = 0xFFu8;
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

        if mask != prev_mask {
            let send_byte = if mask == 0 { 0xFF } else { mask };
            let _ = port.write_all(&[send_byte]);
            let _ = port.flush();
            prev_mask = mask;
        }

        let target = Duration::from_millis((tick as u64 + 1) * ms_per_tick);
        if let Some(remaining) = target.checked_sub(start.elapsed()) {
            thread::sleep(remaining);
        }
    }

    // Release all
    let _ = port.write_all(&[0xFF]);
    let _ = port.flush();
}
