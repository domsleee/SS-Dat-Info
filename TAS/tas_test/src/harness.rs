//! Shared test harness scaffolding — F5 restart, wait helpers, liveness checks.
//!
//! Ported from _full_drift_test.lua and the existing main.rs helpers.

use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

/// How long to wait after F5 for the game to restart loading.
const F5_SETTLE_MS: u64 = 4000;
/// Frames to wait for physics stabilization after restart.
const STABILIZE_FRAMES: u32 = 500;
/// Playback timeout.
const PLAYBACK_TIMEOUT_SECS: u64 = 30;

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

/// Send F5 via Pico HID (COM7) to restart the race.
pub fn send_f5_pico() {
    focus_game();
    match std::fs::OpenOptions::new()
        .write(true)
        .open("\\\\.\\COM7")
    {
        Ok(mut port) => {
            let _ = port.write_all(&[0x40]); // F5 press (bit 6)
            let _ = port.flush();
            thread::sleep(Duration::from_millis(100));
            let _ = port.write_all(&[0xFF]); // release all
            let _ = port.flush();
            println!("  F5 sent via Pico (COM7)");
        }
        Err(e) => {
            eprintln!("  WARNING: Failed to open COM7: {}. Using SendKeys fallback.", e);
            let _ = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    r#"
                    $wshell = New-Object -ComObject wscript.shell
                    $wshell.SendKeys('{F5}')
                "#,
                ])
                .output();
        }
    }
}

/// Wait for N frames to pass (based on Cave 2 frame_count).
pub fn wait_frames(client: &TasSharedMemoryClient, count: u32) {
    let start_fc = client.state().frame_count;
    let timeout = Instant::now();
    while client.state().frame_count < start_fc + count {
        thread::sleep(Duration::from_millis(10));
        if timeout.elapsed() > Duration::from_secs(15) {
            eprintln!("  WARNING: wait_frames timeout ({} frames)", count);
            break;
        }
    }
}

/// Check that Cave 2 is firing (game is alive and hooks are active).
pub fn check_liveness(client: &TasSharedMemoryClient) -> bool {
    let fc1 = client.state().frame_count;
    thread::sleep(Duration::from_millis(500));
    let fc2 = client.state().frame_count;
    let delta = fc2 - fc1;
    println!("  Liveness: {} frames/500ms", delta);
    delta > 0
}

/// F5 restart + wait for stabilization. Returns true if successful.
pub fn restart_and_stabilize(client: &TasSharedMemoryClient) -> bool {
    println!("  Sending F5 to restart...");
    send_f5_pico();
    thread::sleep(Duration::from_millis(F5_SETTLE_MS));
    wait_frames(client, STABILIZE_FRAMES);
    println!(
        "  Stabilized at frame {}",
        client.state().frame_count
    );
    check_liveness(client)
}

/// Arm recording and wait for mode to switch.
pub fn arm_rec(client: &mut TasSharedMemoryClient) {
    client.send_command(TasCommand::ArmRec);
    thread::sleep(Duration::from_millis(50));
    let mode = client.state().mode;
    println!(
        "  ARM_REC -> mode={} (expect 1=REC)",
        mode
    );
}

/// Arm playback and wait for mode to switch.
pub fn arm_play(client: &mut TasSharedMemoryClient) {
    client.send_command(TasCommand::ArmPlay);
    thread::sleep(Duration::from_millis(50));
    let mode = client.state().mode;
    println!(
        "  ARM_PLAY -> mode={} (expect 2=PLAY)",
        mode
    );
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
        let s = client.state();
        if s.mode != TasMode::Play as u32 {
            println!(
                "  Playback complete: {}/{}",
                s.playback_pos, expected
            );
            return true;
        }
        if start.elapsed() > Duration::from_secs(PLAYBACK_TIMEOUT_SECS) {
            eprintln!("  Playback timeout at {}/{}", s.playback_pos, expected);
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

/// F5 restart + stabilize, then start PLAY and check if play_coords[0]
/// matches the target (rec_coords[0]). If not, stop and retry F5.
/// Returns true if matched and playback is running.
pub fn restart_play_and_match(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!("  Retry {}/{}: play_coords[0] mismatch, restarting...", attempt, max_retries);
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
            if attempt > 0 {
                println!("  Position matched on attempt {}", attempt + 1);
            }
            return true;
        }
        let dx = (pc0[0] as f64 - target[0] as f64).abs();
        let dz = (pc0[2] as f64 - target[2] as f64).abs();
        println!(
            "  play_coords[0] offset: dx={:.9} dz={:.9}",
            dx, dz
        );
        stop(client);
    }
    eprintln!("  WARNING: Could not match position after {} retries", max_retries);
    false
}

/// F5 restart + stabilize, then start REC and check if rec_coords[0]
/// matches the target. If not, stop and retry F5.
/// Returns true if matched and recording is running.
pub fn restart_rec_and_match(
    client: &mut TasSharedMemoryClient,
    target: [f32; 3],
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!("  Retry {}/{}: rec_coords[0] mismatch, restarting...", attempt, max_retries);
        }
        if !restart_and_stabilize(client) {
            eprintln!("  ERROR: Game not alive after F5");
            return false;
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
            if attempt > 0 {
                println!("  Position matched on attempt {}", attempt + 1);
            }
            return true; // Recording is running, caller continues
        }
        let dx = (rc0[0] as f64 - target[0] as f64).abs();
        let dz = (rc0[2] as f64 - target[2] as f64).abs();
        println!(
            "  rec_coords[0] offset: dx={:.9} dz={:.9}",
            dx, dz
        );
        stop(client);
    }
    eprintln!("  WARNING: Could not match position after {} retries", max_retries);
    false
}

/// Print drift results (drift computed post-hoc from coordinate arrays).
pub fn print_results(client: &TasSharedMemoryClient) {
    let s = client.state();
    let drift = crate::drift::compute_drift(s, s.recorded_count.min(s.playback_pos));
    println!("=== RESULTS ===");
    println!("Recorded: {} ticks", s.recorded_count);
    println!("Played: {} ticks", s.playback_pos);
    println!("Max drift X: {:.9} (frame {})", drift.max_drift_x, drift.max_drift_frame_x);
    println!("Max drift Z: {:.9} (frame {})", drift.max_drift_z, drift.max_drift_frame_z);
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
