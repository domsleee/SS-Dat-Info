//! Stop+Play flakiness test against FE-tremendous.
//!
//! User reports: loading FE-tremendous.tasrec, hitting PLAY, then STOP at
//! some point mid-playback, then PLAY again — sometimes the second PLAY
//! drifts even though the first one was fine. Goal of this test is to
//! catch that flakiness by exercising the pattern many times with the
//! STOP point spread across a range of mid-playback ticks.
//!
//! Uses tas_ui's actual PLAY-button flow (CMD_RESTART + ARM_PLAY, no
//! start-position retry loop), not the strict 1000-frame trajectory
//! matcher — the matcher refuses every F5 bucket for this recording
//! and would never characterize the flakiness the user actually sees.
//!
//! Sequence per iteration:
//!   1. CMD_RESTART → ARM_PLAY (no position matching)
//!   2. Wait until playback_pos hits the iteration's STOP_AT_FRAMES[i]
//!   3. Send STOP (mode → OFF mid-playback)
//!   4. CMD_RESTART → ARM_PLAY again
//!   5. Wait until playback_pos hits VERIFY_FRAMES (or recording end)
//!   6. Compute drift over the first VERIFY_FRAMES frames of the SECOND
//!      playback against rec_coords
//!
//! Pass criterion: all iterations produce zero drift in the second
//! playback. Reports per-iteration pass/fail so any flakiness is visible
//! in the summary table. The result for an unrepayable recording like
//! FE-tremendous will typically be "many drifted, a few clean" — that
//! ratio IS the flakiness signal.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use crate::drift;
use crate::harness;
use crate::replay;

const ITERATIONS: u32 = 10;
const VERIFY_FRAMES: u32 = 1000;
/// Stop points spread across mid-playback (must be < FE-tremendous's 4696
/// ticks and > VERIFY_FRAMES so the SECOND replay still has a window to
/// verify).
const STOP_AT_FRAMES: [u32; 10] = [1200, 800, 2000, 1500, 2400, 1800, 1100, 2200, 900, 1700];
const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";

/// Mirrors tas_ui's PLAY button flow: in-process restart, then ARM_PLAY.
/// No position-matching retry — whatever F5 bucket the game lands on is
/// what we replay against. Returns true if the restart succeeded.
fn restart_then_play(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    if !harness::restart_and_stabilize_inprocess(client) {
        return false;
    }
    harness::arm_play(client);
    // Match tas_ui's small post-arm delay before checking state.
    thread::sleep(Duration::from_millis(150));
    client.mode_volatile() == tas_shared::TasMode::Play as u32
}

struct CycleResult {
    iteration: u32,
    stop_at: u32,
    first_match_ok: bool,
    second_match_ok: bool,
    drift_x: f64,
    drift_z: f64,
    first_div_frame: Option<usize>,
}

pub fn run() -> bool {
    println!("=== Stop+Play Flakiness Test (FE-tremendous, {}× iterations) ===\n", ITERATIONS);

    // Resolve the recording fixture — same lookup as fe-cont-reliability.
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
            eprintln!("ERROR: Couldn't locate {} (tried {} paths)", RECORDING_REL, candidates.len());
            return false;
        }
    };
    println!("  Recording: {}\n", path.display());

    let rec = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: Failed to load .tasrec: {}", e);
            return false;
        }
    };
    println!("  Loaded {} ticks, target start = ({:.4}, {:.4}, {:.4})",
        rec.count, rec.rec_coords[0][0], rec.rec_coords[0][1], rec.rec_coords[0][2]);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Write recording to shared memory once — STOP doesn't clobber it.
    replay::write_to_shared(&mut client, &rec);
    let target = rec.rec_coords[0];

    let mut results: Vec<CycleResult> = Vec::with_capacity(ITERATIONS as usize);

    for i in 0..ITERATIONS {
        let stop_at = STOP_AT_FRAMES[(i as usize) % STOP_AT_FRAMES.len()];
        println!("\n--- Iteration {}/{} (stop at frame {}) ---", i + 1, ITERATIONS, stop_at);

        // First playback — tas_ui-style: in-process restart + arm_play, no
        // position-match retry. Whatever F5 bucket the game lands in is the
        // bucket we replay against. Drift is the variable we measure.
        let first_match = restart_then_play(&mut client);
        let _ = target; // (Target only used in commented strict-match path)
        if !first_match {
            println!("  First playback: restart failed");
            results.push(CycleResult {
                iteration: i + 1,
                stop_at,
                first_match_ok: false,
                second_match_ok: false,
                drift_x: f64::INFINITY,
                drift_z: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }
        println!("  First playback: started, waiting until frame {}...", stop_at);

        // Wait until playback_pos reaches stop_at
        let wait_start = Instant::now();
        loop {
            let pos = client.playback_pos_volatile();
            if pos >= stop_at {
                break;
            }
            if wait_start.elapsed() > Duration::from_secs(60) {
                eprintln!("  Timed out waiting for playback to reach frame {} (pos={})", stop_at, pos);
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        let actual_stop_pos = client.playback_pos_volatile();
        println!("  STOP at playback_pos={}", actual_stop_pos);
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        // STOP cleared playback_pos but should leave rec_coords intact —
        // rewrite from the loaded file to be sure cave2 has a clean canvas.
        replay::write_to_shared(&mut client, &rec);

        // Second playback — same tas_ui-style no-match flow
        let second_match = restart_then_play(&mut client);
        if !second_match {
            println!("  Second playback: match failed");
            results.push(CycleResult {
                iteration: i + 1,
                stop_at,
                first_match_ok: true,
                second_match_ok: false,
                drift_x: f64::INFINITY,
                drift_z: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }
        println!("  Second playback: started, waiting until frame {}...", VERIFY_FRAMES);

        // Wait until second playback reaches VERIFY_FRAMES
        let wait_start = Instant::now();
        loop {
            let pos = client.playback_pos_volatile();
            if pos >= VERIFY_FRAMES {
                break;
            }
            if wait_start.elapsed() > Duration::from_secs(60) {
                eprintln!("  Timed out waiting for verify (pos={})", pos);
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }

        let state = client.state();
        let check_end = VERIFY_FRAMES.min(state.playback_pos);
        let drift_res = drift::compute_drift(state, check_end);

        // Locate first divergent frame to characterize any drift.
        let mut first_div: Option<usize> = None;
        for j in 0..check_end as usize {
            let p = state.play_coords[j];
            let r = state.rec_coords[j];
            if p[0].to_bits() != r[0].to_bits()
                || p[1].to_bits() != r[1].to_bits()
                || p[2].to_bits() != r[2].to_bits()
            {
                first_div = Some(j);
                break;
            }
        }

        println!(
            "  Second playback: drift X={:.6} (frame {}) Z={:.6} (frame {}) — {}",
            drift_res.max_drift_x, drift_res.max_drift_frame_x,
            drift_res.max_drift_z, drift_res.max_drift_frame_z,
            if drift_res.is_zero() { "ZERO" } else { "DRIFT" }
        );

        // Stop the second playback so the next iteration starts clean.
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        results.push(CycleResult {
            iteration: i + 1,
            stop_at,
            first_match_ok: true,
            second_match_ok: true,
            drift_x: drift_res.max_drift_x,
            drift_z: drift_res.max_drift_z,
            first_div_frame: first_div,
        });
    }

    // Summary
    println!("\n=== STOP+PLAY FLAKE SUMMARY ===");
    println!(
        "{:>4} {:>9} {:>5} {:>5} {:>10} {:>10} {:>10}",
        "#", "stop_at", "m1", "m2", "drift_x", "drift_z", "first_div"
    );
    println!("{}", "-".repeat(70));
    let mut clean = 0u32;
    let mut drifted = 0u32;
    let mut match_failed = 0u32;
    for r in &results {
        let m1 = if r.first_match_ok { "ok" } else { "FAIL" };
        let m2 = if r.second_match_ok { "ok" } else { "FAIL" };
        let div = r
            .first_div_frame
            .map(|f| f.to_string())
            .unwrap_or_else(|| "-".into());
        let zero = r.first_match_ok && r.second_match_ok && r.drift_x == 0.0 && r.drift_z == 0.0;
        if zero {
            clean += 1;
        } else if !r.first_match_ok || !r.second_match_ok {
            match_failed += 1;
        } else {
            drifted += 1;
        }
        println!(
            "{:>4} {:>9} {:>5} {:>5} {:>10.6} {:>10.6} {:>10}",
            r.iteration, r.stop_at, m1, m2, r.drift_x, r.drift_z, div
        );
    }
    println!();
    println!(
        "  Clean (zero drift): {} / {}    Drifted: {}    Match-failed: {}",
        clean, ITERATIONS, drifted, match_failed
    );

    let pass = clean == ITERATIONS;
    if pass {
        println!("\n*** STOP+PLAY FLAKE TEST PASSED: {}/{} clean ***", clean, ITERATIONS);
    } else {
        println!(
            "\n*** STOP+PLAY FLAKE TEST FAILED: only {} of {} clean ({} drifted, {} match-failed) ***",
            clean, ITERATIONS, drifted, match_failed
        );
    }
    pass
}
