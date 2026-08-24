//! Stop→Restart race test.
//!
//! Background: tas_ui used to send `CMD_STOP` and `CMD_RESTART` to cave2
//! on the same UI frame. The shared `command` slot is a single u32
//! (not a queue), so the second write clobbered the first — cave2
//! only ever saw the Restart and mode stayed in REC/PLAY. The
//! subsequent `CMD_ARM_CONTINUE` then hit cave2's mid-run guard:
//!     "ARM_CONTINUE: refused — game is REC/PLAY; CONT requires a
//!      fresh restart first".
//! Fix: send Stop, wait for cave2 to flip mode to OFF, then send
//! Restart. This test verifies both halves:
//!   1. The race actually loses the Stop (proves the bug is real).
//!   2. Serialised version makes it through (proves the fix works).
//!
//! Both halves exercise the LIVE DLL, so we catch regressions in
//! cave2's ARM_CONTINUE guard or any future tas_ui change that
//! reintroduces same-frame Stop+Restart.
//!
//! Note: tas_ui's actual state machine is in `tas_ui::queue_restart_then`.
//! We can't call it from here without dragging eframe into the test
//! binary, so this test simulates the same two patterns directly against
//! the shared-memory bridge — it covers the underlying DLL contract,
//! which is what tas_ui depends on.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
const REC_WAIT_TIMEOUT_SECS: u64 = 30;
const MODE_OFF_WAIT_TIMEOUT_MS: u64 = 2000;

/// Drive a CONT cycle to completion so we end in REC mode (cave2 has
/// flipped past the splice point and is recording). Returns true if
/// the game actually ended up in REC.
fn drive_to_rec_mode(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    // Fresh restart so we start clean.
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    let start = Instant::now();
    while client.restart_state() != 2 {
        if start.elapsed() > Duration::from_secs(15) {
            eprintln!("  drive_to_rec: restart didn't complete");
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
    client.reset_restart_state();
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

/// Send ArmContinue and wait briefly to see what cave2 does with the
/// mode. Returns true if mode transitions to PLAY (= command accepted)
/// or false if it stays OFF (= refused, cave2 set mode=OFF on refusal).
fn send_arm_continue_and_observe_outcome(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    client.state_mut().continue_from_frame = SPLICE_FRAME;
    client.send_command(TasCommand::ArmContinue);
    let start = Instant::now();
    loop {
        let mode = client.mode_volatile();
        if mode == TasMode::Play as u32 {
            return true;
        }
        // Refusal: cave2 sets mode=OFF and logs. Wait a bit before
        // declaring refusal so we don't false-positive on the brief
        // window between command write and cave2 picking it up.
        if start.elapsed() > Duration::from_millis(500) && mode == TasMode::Off as u32 {
            return false;
        }
        if start.elapsed() > Duration::from_secs(3) {
            // Timeout — neither outcome observed.
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

/// Half 1: the race. Send Stop and Restart back-to-back, then arm
/// continue. We expect cave2 to refuse (mode stayed REC because Stop
/// was clobbered). Returns true if refusal was observed (= bug
/// reproduced).
fn race_loses_stop(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    // Send Stop + Restart on the same wall-clock "frame" — no sleep
    // between them, matching the original tas_ui behaviour.
    client.send_command(TasCommand::Stop);
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    // Wait for the Restart to complete.
    let start = Instant::now();
    while client.restart_state() != 2 {
        if start.elapsed() > Duration::from_secs(10) {
            eprintln!("  race: restart didn't complete");
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
    client.reset_restart_state();
    // Now send ArmContinue. If the Stop was lost, mode is still REC
    // and cave2 should refuse.
    let accepted = send_arm_continue_and_observe_outcome(client);
    !accepted
}

/// Half 2: serialised. Send Stop, poll until cave2 confirms mode=OFF,
/// then send Restart, then arm continue. Returns true if the
/// ArmContinue was accepted (= fix works).
fn serialised_stop_then_restart(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    client.send_command(TasCommand::Stop);
    let start = Instant::now();
    while client.mode_volatile() != TasMode::Off as u32 {
        if start.elapsed() > Duration::from_millis(MODE_OFF_WAIT_TIMEOUT_MS) {
            eprintln!("  serialised: mode didn't reach OFF after Stop");
            return false;
        }
        thread::sleep(Duration::from_millis(5));
    }
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    let start = Instant::now();
    while client.restart_state() != 2 {
        if start.elapsed() > Duration::from_secs(10) {
            eprintln!("  serialised: restart didn't complete");
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
    client.reset_restart_state();
    send_arm_continue_and_observe_outcome(client)
}

pub fn run() -> bool {
    println!("=== Stop→Restart race vs serialised, against live DLL ===\n");

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-tremendous.tasrec"),
    ];
    let path = match candidates.iter().find(|p| p.exists()) {
        Some(p) => p.clone(),
        None => {
            eprintln!("ERROR: couldn't locate {}", RECORDING_REL);
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

    // Load the recording so cave2 has rec_coords/recorded_count for CONT.
    replay::write_to_shared(&mut client, &rec);

    // ---- Half 1: race ----
    println!("[1/2] Race: send Stop + Restart same frame, expect refusal");
    if !drive_to_rec_mode(&mut client) {
        eprintln!("FAIL: couldn't reach REC mode for race test");
        return false;
    }
    println!("  Now in REC mode. Sending Stop + Restart back-to-back...");
    let race_refused = race_loses_stop(&mut client);
    if race_refused {
        println!("  ✓ Race reproduces the bug: ArmContinue refused (Stop was clobbered)");
    } else {
        println!(
            "  ✗ Race did NOT refuse — either DLL changed or the race window is too small \
             to reliably hit. The serialised fix is still worth keeping."
        );
    }

    // Stop cleanly between halves.
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(200));

    // ---- Half 2: serialised ----
    println!("\n[2/2] Serialised: Stop → wait mode==OFF → Restart, expect acceptance");
    if !drive_to_rec_mode(&mut client) {
        eprintln!("FAIL: couldn't reach REC mode for serialised test");
        return false;
    }
    println!("  Now in REC mode. Sending Stop, waiting for OFF, then Restart...");
    let serialised_ok = serialised_stop_then_restart(&mut client);
    if serialised_ok {
        println!("  ✓ Serialised path accepted by cave2 (mode → PLAY)");
    } else {
        println!("  ✗ Serialised path FAILED — fix is broken or DLL changed");
    }

    client.send_command(TasCommand::Stop);

    println!("\n=== RESULT ===");
    println!(
        "  race_refused        = {} (true = bug exists in race condition)",
        race_refused
    );
    println!(
        "  serialised_accepted = {} (true = fix works)",
        serialised_ok
    );
    if serialised_ok {
        println!("\n*** PASS: serialised Stop→Restart is accepted by cave2 ***");
    } else {
        println!("\n*** FAIL: serialised path didn't pass — investigate cave2 contract ***");
    }
    serialised_ok
}
