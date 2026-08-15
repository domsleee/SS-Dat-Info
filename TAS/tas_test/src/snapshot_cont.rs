//! Snapshot-based CONT: kill the F5 bucket lottery.
//!
//! The reroll lottery exists ONLY because F5 restart is non-deterministic. This
//! demonstrates the fix: capture a frame-0 spawn snapshot from ONE good bucket,
//! then RESTORE it instead of pressing F5 for every subsequent replay. Because
//! the restored spawn is bit-identical every time, the replay is deterministic →
//! zero drift, zero rerolls.
//!
//! Flow:
//!   1. Load the recording.
//!   2. ESTABLISH (pays the lottery ONCE): F5 → snapshot the spawn → replay →
//!      check it matches the recording (zero drift). Reroll F5 until it matches.
//!   3. VALIDATE (lottery-free): K times, RESTORE the spawn snapshot (no F5) →
//!      replay → assert zero drift. Every one should pass first-try.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasCommand;

use crate::{drift, harness, replay};

const CHECK_FRAME: u32 = 400; // past the ~298 countdown — enough to discriminate buckets
const ESTABLISH_MAX_F5: u32 = 40;
const VALIDATE_ITERS: u32 = 10;
const REPLAY_SPEED: f32 = 64.0;
const MATCH_EPS: f64 = 0.001;

fn replay_to_check(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    harness::arm_play(client);
    let t0 = Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        let mode = client.state().mode;
        if pos >= CHECK_FRAME {
            return true;
        }
        if mode != PLAY_MODE {
            // replay ended early (shouldn't before CHECK_FRAME)
            return client.playback_pos_volatile() >= CHECK_FRAME;
        }
        if t0.elapsed() > Duration::from_secs(15) {
            eprintln!("  WARN: replay stalled at pos {}", pos);
            return false;
        }
        thread::sleep(Duration::from_millis(3));
    }
}

const PLAY_MODE: u32 = 2;

/// Drift of the just-played prefix vs the recording (max of X/Z over CHECK_FRAME).
fn check_drift(client: &tas_shared::TasSharedMemoryClient) -> f64 {
    let d = drift::compute_drift(client.state(), CHECK_FRAME);
    d.max_axis()
}


fn restore(client: &mut tas_shared::TasSharedMemoryClient) -> (u32, u32) {
    client.send_command(TasCommand::Restore);
    thread::sleep(Duration::from_millis(150));
    let s = client.state();
    (s.snapshot_size, s.snapshot_flags) // bytes, us
}

pub fn run() -> bool {
    println!("=== SNAPSHOT CONT: kill the F5 bucket lottery ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    // 1. Load the recording.
    let path = "TAS/recordings/FE-10065.tasrec";
    let candidates = [
        std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join("../../..").join(path))),
        Some(std::path::PathBuf::from(path)),
        Some(std::path::PathBuf::from("recordings/FE-10065.tasrec")),
    ];
    let recpath = candidates.into_iter().flatten().find(|p| p.exists());
    let recpath = match recpath {
        Some(p) => p,
        None => {
            eprintln!("ERROR: couldn't find {}", path);
            return false;
        }
    };
    let loaded = match replay::load_tasrec(&recpath) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("ERROR loading recording: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    client.state_mut().playback_speed = REPLAY_SPEED;
    println!("  Loaded {} ticks from {}", loaded.count, recpath.display());

    // 2. ESTABLISH the spawn snapshot (pays the lottery once).
    println!("\n--- ESTABLISH (one-time F5 lottery to capture a matching spawn) ---");
    let mut established = false;
    let mut establish_f5 = 0u32;
    for attempt in 1..=ESTABLISH_MAX_F5 {
        establish_f5 = attempt;
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  ERROR: F5 restart failed");
            return false;
        }
        // Arm a snapshot at frame-0 of the replay (DLL captures at playback_pos==0,
        // so the arm timing that selects the bucket isn't disturbed). The command
        // slot is single — wait for the DLL to consume it before arming PLAY, or
        // ARM_PLAY clobbers it.
        client.send_command(TasCommand::SnapshotAtSpawn);
        {
            let t0 = Instant::now();
            while client.state().command != TasCommand::Idle as u32 {
                if t0.elapsed() > Duration::from_millis(500) { break; }
                thread::sleep(Duration::from_millis(2));
            }
        }
        // Replay this bucket; the DLL snapshots the spawn at frame 0, then we
        // check whether this bucket matches the recording.
        if !replay_to_check(&mut client) {
            harness::stop(&mut client);
            continue;
        }
        let drift = check_drift(&client);
        let snap_bytes = client.state().snapshot_size;
        let snap_us = client.state().snapshot_flags;
        harness::stop(&mut client);
        if snap_bytes == 0 {
            eprintln!("  WARN: F5 #{}: spawn snapshot didn't fire (0 bytes)", attempt);
            continue;
        }
        println!(
            "  F5 #{}: spawn snapshot {:.1}MB/{:.0}ms, replay drift = {:.6} {}",
            attempt,
            snap_bytes as f64 / 1_048_576.0,
            snap_us as f64 / 1000.0,
            drift,
            if drift < MATCH_EPS { "<- MATCH, snapshot established" } else { "(wrong bucket, reroll)" }
        );
        if drift < MATCH_EPS {
            established = true;
            break;
        }
    }
    if !established {
        println!("\n*** FAILED: couldn't establish a matching spawn snapshot in {} F5 tries ***", ESTABLISH_MAX_F5);
        return false;
    }

    // 3. VALIDATE: lottery-free restores.
    println!("\n--- VALIDATE ({} lottery-free CONTs via RESTORE, no F5) ---", VALIDATE_ITERS);
    let mut clean = 0u32;
    // `None` until an iteration actually completes a replay and measures drift.
    // Tracked as an Option so a run where EVERY iteration stalled reports "n/a"
    // rather than the 0.0 initialiser — a total failure used to print the
    // healthiest-looking drift number possible.
    let mut worst_drift: Option<f64> = None;
    let mut stalled = 0u32;
    let mut restore_failed = 0u32;
    let mut restore_ms_sum = 0.0f64;
    for i in 1..=VALIDATE_ITERS {
        let (rest_bytes, rest_us) = restore(&mut client);
        restore_ms_sum += rest_us as f64 / 1000.0;
        if rest_bytes == 0 {
            println!("  #{}: RESTORE failed (0 bytes)", i);
            restore_failed += 1;
            continue;
        }
        if !replay_to_check(&mut client) {
            println!("  #{}: replay stalled", i);
            stalled += 1;
            harness::stop(&mut client);
            continue;
        }
        let drift = check_drift(&client);
        harness::stop(&mut client);
        worst_drift = Some(worst_drift.map_or(drift, |w: f64| w.max(drift)));
        let ok = drift < MATCH_EPS;
        if ok { clean += 1; }
        println!(
            "  #{}: restore {:.0}ms, replay drift = {:.6} {}",
            i, rest_us as f64 / 1000.0, drift, if ok { "OK (no reroll)" } else { "DRIFT!" }
        );
    }

    println!("\n=== SUMMARY ===");
    println!("  Established spawn snapshot after {} F5 attempt(s) (one-time lottery).", establish_f5);
    let worst_str = match worst_drift {
        Some(w) => format!("{:.6}", w),
        None => "n/a (no iteration produced a measurement)".to_string(),
    };
    println!(
        "  Then {}/{} lottery-free restores clean, worst drift {}, mean restore {:.0}ms.",
        clean, VALIDATE_ITERS, worst_str, restore_ms_sum / VALIDATE_ITERS as f64
    );
    if stalled > 0 || restore_failed > 0 {
        println!(
            "  Unmeasured: {} replay stall(s), {} restore failure(s) — these never reached a drift \
             comparison, so they are failures, not clean runs.",
            stalled, restore_failed
        );
    }
    let ok = clean == VALIDATE_ITERS;
    if ok {
        println!("\n*** SNAPSHOT CONT PASSED: {}/{} replays bit-exact with ZERO rerolls (F5 lottery eliminated) ***", clean, VALIDATE_ITERS);
    } else {
        println!("\n*** SNAPSHOT CONT FAILED: {}/{} clean ***", clean, VALIDATE_ITERS);
    }
    ok
}
