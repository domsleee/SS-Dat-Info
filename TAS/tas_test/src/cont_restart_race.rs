//! Stop→Restart race against the live DLL.
//!
//! The shared `command` slot is a single u32, not a queue, so `CMD_STOP` and
//! `CMD_RESTART` written on the same UI frame lose the Stop: cave2 only sees
//! the Restart, mode stays REC/PLAY, and the following `CMD_ARM_CONTINUE`
//! hits cave2's mid-run guard ("ARM_CONTINUE: refused — game is REC/PLAY").
//! tas_ui therefore sends Stop, waits for mode OFF, then sends Restart. Both
//! halves of that contract are asserted here:
//!   1. back-to-back Stop+Restart loses the Stop, so ArmContinue is refused;
//!   2. the serialised sequence gets ArmContinue accepted.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::harness;
use crate::replay;

const RECORDING: &str = "FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
const REC_WAIT_TIMEOUT_SECS: u64 = 30;
const MODE_OFF_WAIT_TIMEOUT_MS: u64 = 2000;

/// Drive a CONT cycle past the splice so the game ends up in REC mode.
fn drive_to_rec_mode(client: &mut TasSharedMemoryClient) -> bool {
    if !harness::restart_inprocess(client) {
        eprintln!("  drive_to_rec: restart didn't complete");
        return false;
    }
    client.state_mut().continue_from_frame = SPLICE_FRAME;
    client.state_mut().playback_speed = 64.0;
    client.send_command(TasCommand::ArmContinue);
    let start = Instant::now();
    loop {
        if client.mode_volatile() == TasMode::Rec as u32 {
            return true;
        }
        if start.elapsed() > Duration::from_secs(REC_WAIT_TIMEOUT_SECS) {
            eprintln!(
                "  drive_to_rec: mode never reached REC (current={})",
                client.mode_volatile()
            );
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// Send ArmContinue and report whether cave2 accepted it (mode goes to PLAY)
/// or refused (mode stays OFF; cave2 sets OFF on refusal).
fn send_arm_continue_and_observe_outcome(client: &mut TasSharedMemoryClient) -> bool {
    client.state_mut().continue_from_frame = SPLICE_FRAME;
    client.send_command(TasCommand::ArmContinue);
    let start = Instant::now();
    loop {
        let mode = client.mode_volatile();
        if mode == TasMode::Play as u32 {
            return true;
        }
        // Wait past the window between the command write and cave2 picking
        // it up before calling OFF a refusal.
        if start.elapsed() > Duration::from_millis(500) && mode == TasMode::Off as u32 {
            return false;
        }
        if start.elapsed() > Duration::from_secs(3) {
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

/// Half 1: Stop and Restart back-to-back, then ArmContinue. Returns true when
/// cave2 refused, i.e. the Stop was lost.
fn race_loses_stop(client: &mut TasSharedMemoryClient) -> bool {
    client.send_command(TasCommand::Stop);
    if !harness::restart_inprocess(client) {
        eprintln!("  race: restart didn't complete");
        return false;
    }
    !send_arm_continue_and_observe_outcome(client)
}

/// Half 2: Stop, wait for mode OFF, Restart, then ArmContinue. Returns true
/// when cave2 accepted.
fn serialised_stop_then_restart(client: &mut TasSharedMemoryClient) -> bool {
    client.send_command(TasCommand::Stop);
    let start = Instant::now();
    while client.mode_volatile() != TasMode::Off as u32 {
        if start.elapsed() > Duration::from_millis(MODE_OFF_WAIT_TIMEOUT_MS) {
            eprintln!("  serialised: mode didn't reach OFF after Stop");
            return false;
        }
        thread::sleep(Duration::from_millis(5));
    }
    if !harness::restart_inprocess(client) {
        eprintln!("  serialised: restart didn't complete");
        return false;
    }
    send_arm_continue_and_observe_outcome(client)
}

/// Exercise the real controller and injected DLL up to the first restart
/// boundary: live input stays blocked through the CONT's internal STOP, and an
/// ordinary STOP releases it.
pub fn run_input_protection() -> bool {
    use tas_shared::transport::{Arm, ArmConfig, StepOutcome, TransportController};

    println!("CONT input protection: real controller and live DLL");
    let mut client = harness::connect();
    harness::stop_competing_tas_ui_writer();
    harness::print_status(&client);
    let original_speed = client.state().playback_speed;
    let config = ArmConfig {
        arm: Arm::Continue,
        catchup_speed: original_speed,
        continue_from_frame: 4705,
        gate_align_rec: 299,
        target: None,
        max_retries: 30,
        resume_speed: original_speed,
        predict_bucket: false,
    };
    let mut controller = TransportController::new(config);
    // Same setup as tas_ui's restart queue, immediately before stepping.
    client.state_mut().cont_suppress_input = 1;
    let before = unsafe { std::ptr::read_volatile(&client.state().cont_suppress_input) };
    println!("Before CONT internal STOP: cont_suppress_input={before}");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut passed = false;
    loop {
        match controller.step(&mut client) {
            StepOutcome::Wait { .. } => {
                let state = client.state();
                let mode = unsafe { std::ptr::read_volatile(&state.mode) };
                let command = unsafe { std::ptr::read_volatile(&state.command) };
                let protected = unsafe { std::ptr::read_volatile(&state.cont_suppress_input) };
                println!("After DLL acknowledged STOP: mode={mode} command={command} cont_suppress_input={protected}");
                passed = mode == TasMode::Off as u32 && command == 0 && protected == 1;
                println!(
                    "{}: live input must remain blocked before the restart",
                    if passed { "PASS" } else { "FAIL" }
                );
                break;
            }
            StepOutcome::InProgress if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(1));
            }
            outcome => {
                eprintln!("FAIL: STOP acknowledgement unavailable: {outcome:?}");
                break;
            }
        }
    }
    // Cancelling must release protection, including while already OFF.
    client.send_command(TasCommand::Stop);
    let deadline = Instant::now() + Duration::from_secs(5);
    while !client.command_idle() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    let protected = unsafe { std::ptr::read_volatile(&client.state().cont_suppress_input) };
    let cancelled = client.command_idle() && protected == 0;
    println!(
        "{}: ordinary STOP releases input protection (cont_suppress_input={protected})",
        if cancelled { "PASS" } else { "FAIL" }
    );
    passed &= cancelled;
    client.state_mut().playback_speed = original_speed;
    passed
}

pub fn run() -> bool {
    println!("=== Stop→Restart race vs serialised, against live DLL ===\n");

    let path = match harness::fixture_path(RECORDING) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    let rec = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: load failed: {}", e);
            return false;
        }
    };
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "cont-restart-race");

    replay::write_to_shared(&mut client, &rec);

    println!("[1/2] Race: send Stop + Restart same frame, expect refusal");
    if !drive_to_rec_mode(&mut client) {
        eprintln!("FAIL: couldn't reach REC mode for race test");
        return false;
    }
    println!("  Now in REC mode. Sending Stop + Restart back-to-back...");
    let race_refused = race_loses_stop(&mut client);
    if race_refused {
        println!("  Race loses the Stop: ArmContinue refused");
    } else {
        println!("  Race did NOT refuse: the single-slot command contract has changed");
    }

    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(200));

    println!("\n[2/2] Serialised: Stop → wait mode==OFF → Restart, expect acceptance");
    if !drive_to_rec_mode(&mut client) {
        eprintln!("FAIL: couldn't reach REC mode for serialised test");
        return false;
    }
    println!("  Now in REC mode. Sending Stop, waiting for OFF, then Restart...");
    let serialised_ok = serialised_stop_then_restart(&mut client);
    if serialised_ok {
        println!("  Serialised path accepted by cave2 (mode → PLAY)");
    } else {
        println!("  Serialised path FAILED");
    }

    client.send_command(TasCommand::Stop);

    println!("\n=== RESULT ===");
    println!("  race_refused        = {} (expected true)", race_refused);
    println!("  serialised_accepted = {} (expected true)", serialised_ok);
    let pass = race_refused && serialised_ok;
    if pass {
        println!("\n*** PASS: serialised Stop→Restart is accepted by cave2 ***");
    } else {
        println!("\n*** FAIL: Stop→Restart contract broken — investigate cave2 ***");
    }
    pass
}
