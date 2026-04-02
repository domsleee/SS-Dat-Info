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
/// Playback timeout (long enough for 65536 frames at ~50fps unfocused).
const PLAYBACK_TIMEOUT_SECS: u64 = 120;

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

/// Find the Supreme Snowboarding process ID.
fn find_game_pid() -> Option<u32> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            r#"(Get-Process Supreme -EA SilentlyContinue | Where {$_.Name -notmatch 'service'} | Select -First 1).Id"#,
        ])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout);
    s.trim().parse::<u32>().ok().filter(|&id| id != 0)
}

/// Force the player position in game memory via WriteProcessMemory.
/// Writes to player struct offsets +0xF8/FC/100 (pos) and +0x104/108/10C (prev),
/// plus physics sub-object at [player+0x110]+0x3B4..3C8.
fn force_position_in_game(player_ptr: u32, target: [f32; 3]) -> bool {
    let pid = match find_game_pid() {
        Some(p) => p,
        None => {
            eprintln!("  WARNING: Cannot find game PID for position forcing");
            return false;
        }
    };

    // Build a PowerShell script to WriteProcessMemory
    let script = format!(
        r#"
Add-Type @'
using System; using System.Runtime.InteropServices;
public class Mem {{
    [DllImport("kernel32.dll")] public static extern IntPtr OpenProcess(int a, bool b, int pid);
    [DllImport("kernel32.dll")] public static extern bool WriteProcessMemory(IntPtr h, IntPtr addr, byte[] buf, int sz, out int written);
    [DllImport("kernel32.dll")] public static extern bool ReadProcessMemory(IntPtr h, IntPtr addr, byte[] buf, int sz, out int read);
    [DllImport("kernel32.dll")] public static extern bool CloseHandle(IntPtr h);
}}
'@
$h = [Mem]::OpenProcess(0x38, $false, {pid})
if ($h -eq [IntPtr]::Zero) {{ Write-Output "FAIL:OpenProcess"; exit }}
$w = 0
$xb = [BitConverter]::GetBytes([float]{x})
$yb = [BitConverter]::GetBytes([float]{y})
$zb = [BitConverter]::GetBytes([float]{z})
$pp = [IntPtr]::new({pp})
# Primary position
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0xF8), $xb, 4, [ref]$w) | Out-Null
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0xFC), $yb, 4, [ref]$w) | Out-Null
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0x100), $zb, 4, [ref]$w) | Out-Null
# Secondary/prev position
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0x104), $xb, 4, [ref]$w) | Out-Null
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0x108), $yb, 4, [ref]$w) | Out-Null
[Mem]::WriteProcessMemory($h, [IntPtr]::new({pp} + 0x10C), $zb, 4, [ref]$w) | Out-Null
# Read physics sub-object pointer at player+0x110
$buf4 = New-Object byte[] 4
[Mem]::ReadProcessMemory($h, [IntPtr]::new({pp} + 0x110), $buf4, 4, [ref]$w) | Out-Null
$physPtr = [BitConverter]::ToUInt32($buf4, 0)
if ($physPtr -ne 0) {{
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3B4), $xb, 4, [ref]$w) | Out-Null
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3B8), $yb, 4, [ref]$w) | Out-Null
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3BC), $zb, 4, [ref]$w) | Out-Null
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3C0), $xb, 4, [ref]$w) | Out-Null
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3C4), $yb, 4, [ref]$w) | Out-Null
    [Mem]::WriteProcessMemory($h, [IntPtr]::new($physPtr + 0x3C8), $zb, 4, [ref]$w) | Out-Null
}}
[Mem]::CloseHandle($h) | Out-Null
Write-Output "OK"
"#,
        pid = pid,
        pp = player_ptr,
        x = target[0],
        y = target[1],
        z = target[2],
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output();

    match output {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout);
            if out.trim() == "OK" {
                println!(
                    "  Forced start position: ({:.3}, {:.3}, {:.3}) at player_ptr=0x{:X}",
                    target[0], target[1], target[2], player_ptr
                );
                true
            } else {
                eprintln!("  WARNING: Position force failed: {}", out.trim());
                false
            }
        }
        Err(e) => {
            eprintln!("  WARNING: Position force PS error: {}", e);
            false
        }
    }
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
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!("  Retry {}/{}: restarting...", attempt, max_retries);
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after F5");
            return false;
        }

        arm_play(client);
        // Wait for at least 1 frame of playback to capture play_coords[0]
        thread::sleep(Duration::from_millis(100));
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
        if match_x && match_y && match_z {
            println!("  Position matched (attempt {})", attempt + 1);
            return true;
        }
        let dx = (pc0[0] as f64 - target[0] as f64).abs();
        let dz = (pc0[2] as f64 - target[2] as f64).abs();
        println!("  play_coords[0] offset: dx={:.9} dz={:.9}", dx, dz);
        stop(client);
    }
    eprintln!(
        "  WARNING: Could not match position after {} retries",
        max_retries
    );
    false
}

/// F5 restart + stabilize, force start position, then start PLAY and verify
/// play_coords[0] matches the target. Uses position forcing instead of
/// relying on F5 quantized positions (which may never match naturally).
/// Returns true if playback is running with correct start position.
pub fn restart_play_and_force(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!(
                "  Retry {}/{}: play force position...",
                attempt, max_retries
            );
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after F5");
            return false;
        }

        // Force position before arming PLAY
        let player_ptr = client.state().player_ptr;
        if player_ptr != 0 && (target[0] != 0.0 || target[1] != 0.0 || target[2] != 0.0) {
            force_position_in_game(player_ptr, target);
            thread::sleep(Duration::from_millis(50));
        }

        arm_play(client);
        // Wait for at least 1 frame of playback to capture play_coords[0]
        thread::sleep(Duration::from_millis(100));
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
        if match_x && match_y && match_z {
            println!("  Position force-matched (attempt {})", attempt + 1);
            return true;
        }
        let dx = (pc0[0] as f64 - target[0] as f64).abs();
        let dz = (pc0[2] as f64 - target[2] as f64).abs();
        println!(
            "  play_coords[0] offset after force: dx={:.9} dz={:.9}",
            dx, dz
        );
        stop(client);
    }
    eprintln!(
        "  WARNING: Could not force-match position after {} retries",
        max_retries
    );
    false
}

/// F5 restart + stabilize, force start position, then start REC and check if
/// rec_coords[0] matches the target.
/// Returns true if matched and recording is running.
pub fn restart_rec_and_match(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!(
                "  Retry {}/{}: rec_coords[0] mismatch, restarting...",
                attempt, max_retries
            );
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after F5");
            return false;
        }

        // Force the start position before recording
        let player_ptr = client.state().player_ptr;
        if player_ptr != 0 && (target[0] != 0.0 || target[1] != 0.0 || target[2] != 0.0) {
            force_position_in_game(player_ptr, target);
            thread::sleep(Duration::from_millis(50));
        }

        arm_rec(client);
        // Wait for at least 1 frame of recording to capture rec_coords[0]
        thread::sleep(Duration::from_millis(100));
        let s = client.state();
        if s.recorded_count == 0 {
            eprintln!("  WARNING: Recording didn't start");
            stop(client);
            continue;
        }
        let rc0 = s.rec_coords[0];
        let match_x = rc0[0].to_bits() == target[0].to_bits();
        let match_y = rc0[1].to_bits() == target[1].to_bits();
        let match_z = rc0[2].to_bits() == target[2].to_bits();
        if match_x && match_y && match_z {
            println!("  Position matched (attempt {})", attempt + 1);
            return true;
        }
        let dx = (rc0[0] as f64 - target[0] as f64).abs();
        let dz = (rc0[2] as f64 - target[2] as f64).abs();
        println!("  rec_coords[0] offset: dx={:.9} dz={:.9}", dx, dz);
        stop(client);
    }
    eprintln!(
        "  WARNING: Could not match position after {} retries",
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

/// Write input masks into shared memory for mock input mode.
/// This writes directly to the input_log, simulating what the DLL would capture from DI.
pub fn write_mock_input(client: &mut TasSharedMemoryClient, input_log: &[u8]) {
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
        thread::sleep(Duration::from_millis(20));
        let s = client.state();
        if s.mode == TasMode::Rec as u32 {
            println!(
                "  CONT splice complete: mode=REC at pos={}, segment_count={}",
                s.recorded_count, s.segment_count
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

/// F5 restart + ARM_CONTINUE with position matching, then wait for splice.
///
/// Same as restart_play_and_match but uses ARM_CONTINUE instead of ARM_PLAY.
/// On position match, waits for PLAY→REC splice at splice_frame.
///
/// Returns true if CONT successfully spliced (PLAY→REC transition at splice_frame).
pub fn restart_continue_and_splice(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    splice_frame: u32,
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!(
                "  Retry {}/{}: position match for CONT",
                attempt, max_retries
            );
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after F5");
            return false;
        }

        // ARM_CONTINUE: starts as PLAY from tick 0, will auto-switch to REC at splice_frame
        arm_continue(client, splice_frame);
        thread::sleep(Duration::from_millis(100));

        let s = client.state();
        if s.playback_pos == 0 && s.mode == TasMode::Off as u32 {
            eprintln!("  WARNING: CONT didn't start");
            stop(client);
            continue;
        }

        let pc0 = s.play_coords[0];
        let match_x = pc0[0].to_bits() == target[0].to_bits();
        let match_y = pc0[1].to_bits() == target[1].to_bits();
        let match_z = pc0[2].to_bits() == target[2].to_bits();

        if match_x && match_y && match_z {
            if attempt > 0 {
                println!("  Position matched on attempt {}", attempt + 1);
            }
            println!("  CONT position matched, waiting for splice...");
            if wait_continue_splice(client, splice_frame) {
                return true;
            }
            // Splice failed but position matched — retry
            stop(client);
            continue;
        }

        let dx = (pc0[0] as f64 - target[0] as f64).abs();
        let dz = (pc0[2] as f64 - target[2] as f64).abs();
        println!("  play_coords[0] offset: dx={:.9} dz={:.9}", dx, dz);
        stop(client);
    }
    eprintln!(
        "  WARNING: Could not complete CONT splice after {} retries",
        max_retries
    );
    false
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
