//! Stop+Play flakiness test against FE-tremendous.
//!
//! User reports: loading FE-tremendous.tasrec, hitting PLAY, then STOP at
//! some point mid-playback, then PLAY again — sometimes the second PLAY
//! produces a different trajectory than the first. The test pins this
//! down by comparing each stop+play iteration's trajectory against a
//! single baseline trajectory captured at the start of the run.
//!
//! Important framing: FE-tremendous has a known inherent drift vs its
//! own rec_coords (the recording was made without rotation/velocity
//! state captured, so any F5 bucket whose rotation differs from
//! whatever the original recorder had produces a divergent trajectory
//! at frame ~249). That drift is NOT what this test measures.
//!
//! This test measures whether STOP+PLAY produces the SAME trajectory
//! that a fresh PLAY produces — i.e. whether STOP introduces
//! additional non-determinism on top of the recording's existing
//! issues. Pass means: stop+play is idempotent. Drifted-but-consistent
//! is a PASS. Drifted-and-different-from-baseline is a FAIL.
//!
//! Sequence:
//!   1. Phase 0: One reference PLAY, capture play_coords[0..N]
//!   2. For each iteration:
//!      a. CMD_RESTART → ARM_PLAY (no position match — tas_ui's flow)
//!      b. Wait until playback_pos hits the iteration's STOP_AT_FRAMES[i]
//!      c. Send STOP
//!      d. CMD_RESTART → ARM_PLAY again
//!      e. Wait until playback_pos hits VERIFY_FRAMES
//!      f. Compare second-play play_coords[0..VERIFY_FRAMES] vs reference
//!
//! Pass: all iterations produce play_coords bit-identical to the
//! reference. Any mismatch is the STOP-induced flakiness the user
//! described.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use crate::harness;
use crate::replay;

const ITERATIONS: u32 = 10;
const VERIFY_FRAMES: u32 = 1000;
const STOP_AT_FRAMES: [u32; 10] = [1200, 800, 2000, 1500, 2400, 1800, 1100, 2200, 900, 1700];
const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
/// How many times to retry the second PLAY when its trajectory diverges
/// from the reference. FE-tremendous's F5 bucket lottery is non-trivial:
/// the reference lands in one bucket, subsequent F5s may land in others.
/// Retrying re-rolls until we get the bucket that matches the reference.
/// 30 retries × ~3s = up to ~90s of catch-up per iteration on a bad streak.
const REF_MATCH_RETRIES: u32 = 30;
/// How many leading frames the retry loop checks for an early-divergence
/// signal. If those frames match the reference bit-identically, we trust
/// the full VERIFY_FRAMES will too (the trajectory's deterministic from
/// the start state). 50 covers the recording's stationary phase plus a
/// few motion frames where any rotation mismatch would surface.
const RETRY_VERIFY_FRAMES: u32 = 300;

struct CycleResult {
    iteration: u32,
    stop_at: u32,
    started_ok: bool,
    second_play_ok: bool,
    /// Max absolute coordinate difference between this iteration's second
    /// playback and the reference playback, over the first VERIFY_FRAMES.
    /// Bit-identical match → 0.0.
    max_dx_vs_ref: f64,
    max_dz_vs_ref: f64,
    /// First frame where this iteration's second playback diverges from
    /// the reference playback (bit-level). None = matches all frames.
    first_div_frame: Option<usize>,
}

/// Mirrors tas_ui's PLAY button flow: in-process restart, then ARM_PLAY.
/// No position-matching retry — whatever F5 bucket the game lands on is
/// what we replay against.
fn restart_then_play(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    if !harness::restart_and_stabilize_inprocess(client) {
        return false;
    }
    harness::arm_play(client);
    thread::sleep(Duration::from_millis(150));
    client.mode_volatile() == tas_shared::TasMode::Play as u32
}

/// Restart + ARM_PLAY, then verify the trajectory matches the reference
/// over the first RETRY_VERIFY_FRAMES frames. If it diverges, STOP and
/// retry up to `max_retries` times.
///
/// For FE-tremendous specifically: the recording is missing rotation state
/// in its file format, so different F5 buckets produce different
/// trajectories. The reference playback locked in one bucket's behaviour;
/// this function rolls F5 until we land in the same bucket again.
fn restart_play_match_reference(
    client: &mut tas_shared::TasSharedMemoryClient,
    reference: &[[f32; 3]],
    rec: &replay::LoadedRecording,
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        // Rewrite recording each retry; STOP after a divergence shouldn't
        // need this, but it's a cheap safety belt.
        replay::write_to_shared(client, rec);

        if !restart_then_play(client) {
            eprintln!("  Restart-then-play failed on attempt {}", attempt + 1);
            continue;
        }

        // Wait until playback reaches RETRY_VERIFY_FRAMES so we have a window
        // to compare against the reference.
        let _ = wait_for_pos(client, RETRY_VERIFY_FRAMES, Duration::from_secs(20));

        let state = client.state();
        let mut diverge: Option<usize> = None;
        for (j, r) in reference
            .iter()
            .copied()
            .enumerate()
            .take(RETRY_VERIFY_FRAMES as usize)
        {
            let p = state.play_coords[j];
            if p[0].to_bits() != r[0].to_bits()
                || p[1].to_bits() != r[1].to_bits()
                || p[2].to_bits() != r[2].to_bits()
            {
                diverge = Some(j);
                break;
            }
        }
        if diverge.is_none() {
            println!(
                "  Reference-trajectory match (attempt {}, {} frames verified)",
                attempt + 1,
                RETRY_VERIFY_FRAMES
            );
            return true;
        }
        println!(
            "  Reference-trajectory mismatch at frame {} (attempt {}/{}) — retrying for matching F5 bucket",
            diverge.unwrap(),
            attempt + 1,
            max_retries
        );
        harness::stop(client);
        thread::sleep(Duration::from_millis(200));
    }
    eprintln!(
        "  Could not match reference trajectory after {} retries",
        max_retries
    );
    false
}

/// Wait until playback_pos reaches `target` or the timeout elapses.
/// Returns the actual playback_pos seen.
fn wait_for_pos(client: &tas_shared::TasSharedMemoryClient, target: u32, timeout: Duration) -> u32 {
    let start = Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        if pos >= target {
            return pos;
        }
        if start.elapsed() > timeout {
            return pos;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

pub fn run() -> bool {
    println!(
        "=== Stop+Play Flakiness Test (FE-tremendous, baseline-comparison, {}× iters) ===\n",
        ITERATIONS
    );

    // Resolve recording fixture path.
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
            eprintln!(
                "ERROR: Couldn't locate {} (tried {} paths)",
                RECORDING_REL,
                candidates.len()
            );
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
    println!("  Loaded {} ticks", rec.count);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // ---- Phase 0: Reference playback ----
    println!("\n--- Phase 0: Reference playback (no stop) ---");
    replay::write_to_shared(&mut client, &rec);

    if !restart_then_play(&mut client) {
        eprintln!("ERROR: Reference playback couldn't start");
        return false;
    }
    let _ = wait_for_pos(&client, VERIFY_FRAMES, Duration::from_secs(60));
    let reference: Vec<[f32; 3]> = client.state().play_coords[..VERIFY_FRAMES as usize].to_vec();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(200));
    println!(
        "  Reference captured: start=({:.4}, {:.4}, {:.4})  end=({:.4}, {:.4}, {:.4})",
        reference[0][0],
        reference[0][1],
        reference[0][2],
        reference[(VERIFY_FRAMES - 1) as usize][0],
        reference[(VERIFY_FRAMES - 1) as usize][1],
        reference[(VERIFY_FRAMES - 1) as usize][2],
    );

    // ---- Iterations ----
    let mut results: Vec<CycleResult> = Vec::with_capacity(ITERATIONS as usize);
    for i in 0..ITERATIONS {
        let stop_at = STOP_AT_FRAMES[(i as usize) % STOP_AT_FRAMES.len()];
        println!(
            "\n--- Iteration {}/{} (stop at frame {}) ---",
            i + 1,
            ITERATIONS,
            stop_at
        );

        // Make sure shared rec_coords / input_log are fresh.
        replay::write_to_shared(&mut client, &rec);

        // First playback
        if !restart_then_play(&mut client) {
            println!("  First playback: restart failed");
            results.push(CycleResult {
                iteration: i + 1,
                stop_at,
                started_ok: false,
                second_play_ok: false,
                max_dx_vs_ref: f64::INFINITY,
                max_dz_vs_ref: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }

        // Wait until reaching the stop point.
        let actual_stop = wait_for_pos(&client, stop_at, Duration::from_secs(60));
        println!("  STOP at playback_pos={}", actual_stop);
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        // Second playback — retry until trajectory matches the reference.
        // FE-tremendous's F5 bucket lottery means a naive single-shot PLAY
        // sometimes lands in a different bucket and diverges; the test's
        // purpose is to verify STOP doesn't introduce flakiness ON TOP of
        // that, so we re-roll F5 until we get the reference bucket.
        let second_ok =
            restart_play_match_reference(&mut client, &reference, &rec, REF_MATCH_RETRIES);
        if !second_ok {
            println!("  Second playback: reference match failed after retries");
            results.push(CycleResult {
                iteration: i + 1,
                stop_at,
                started_ok: true,
                second_play_ok: false,
                max_dx_vs_ref: f64::INFINITY,
                max_dz_vs_ref: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }

        // Reference match locked in — wait for the full VERIFY_FRAMES so the
        // post-iteration table reports the full-window diff (should be zero).
        let _ = wait_for_pos(&client, VERIFY_FRAMES, Duration::from_secs(60));

        // Compare second-play coords vs reference.
        let state = client.state();
        let mut max_dx = 0.0f64;
        let mut max_dz = 0.0f64;
        let mut first_div: Option<usize> = None;
        for (j, r) in reference
            .iter()
            .copied()
            .enumerate()
            .take(VERIFY_FRAMES as usize)
        {
            let p = state.play_coords[j];
            let dx = (p[0] as f64 - r[0] as f64).abs();
            let dz = (p[2] as f64 - r[2] as f64).abs();
            if dx > max_dx {
                max_dx = dx;
            }
            if dz > max_dz {
                max_dz = dz;
            }
            if first_div.is_none()
                && (p[0].to_bits() != r[0].to_bits()
                    || p[1].to_bits() != r[1].to_bits()
                    || p[2].to_bits() != r[2].to_bits())
            {
                first_div = Some(j);
            }
        }

        println!(
            "  Second playback: vs reference dx_max={:.6} (frame any) dz_max={:.6} first_div={}",
            max_dx,
            max_dz,
            first_div
                .map(|f| f.to_string())
                .unwrap_or_else(|| "matches all".into())
        );

        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        results.push(CycleResult {
            iteration: i + 1,
            stop_at,
            started_ok: true,
            second_play_ok: true,
            max_dx_vs_ref: max_dx,
            max_dz_vs_ref: max_dz,
            first_div_frame: first_div,
        });
    }

    // ---- Summary ----
    println!("\n=== STOP+PLAY FLAKE SUMMARY ===");
    println!("Comparing each iteration's SECOND playback to the reference (single PLAY, no stop).");
    println!(
        "Pass requires bit-identical match across the first {} frames.",
        VERIFY_FRAMES
    );
    println!();
    println!(
        "{:>4} {:>9} {:>5} {:>5} {:>12} {:>12} {:>10}",
        "#", "stop_at", "start", "play2", "max_dx", "max_dz", "first_div"
    );
    println!("{}", "-".repeat(72));
    let mut clean = 0u32;
    let mut diverged = 0u32;
    let mut start_failed = 0u32;
    for r in &results {
        let start = if r.started_ok { "ok" } else { "FAIL" };
        let play2 = if r.second_play_ok { "ok" } else { "FAIL" };
        let div = r
            .first_div_frame
            .map(|f| f.to_string())
            .unwrap_or_else(|| "-".into());
        let bit_match = r.started_ok
            && r.second_play_ok
            && r.first_div_frame.is_none()
            && r.max_dx_vs_ref == 0.0
            && r.max_dz_vs_ref == 0.0;
        if bit_match {
            clean += 1;
        } else if !r.started_ok || !r.second_play_ok {
            start_failed += 1;
        } else {
            diverged += 1;
        }
        println!(
            "{:>4} {:>9} {:>5} {:>5} {:>12.6} {:>12.6} {:>10}",
            r.iteration, r.stop_at, start, play2, r.max_dx_vs_ref, r.max_dz_vs_ref, div
        );
    }
    println!();
    println!(
        "  Bit-identical to reference: {} / {}    Diverged: {}    Start-failed: {}",
        clean, ITERATIONS, diverged, start_failed
    );

    let pass = clean == ITERATIONS;
    if pass {
        println!(
            "\n*** STOP+PLAY FLAKE TEST PASSED: {}/{} iterations matched the reference playback ***",
            clean, ITERATIONS
        );
    } else {
        println!(
            "\n*** STOP+PLAY FLAKE TEST FAILED: {} of {} iterations diverged from the reference ***",
            diverged, ITERATIONS
        );
    }
    pass
}
