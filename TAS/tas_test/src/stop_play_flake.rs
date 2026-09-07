//! Stop+Play flakiness test against FE-tremendous: PLAY, STOP mid-playback,
//! PLAY again, and require the second playback to reproduce a reference
//! playback captured at the start of the run.
//!
//! FE-tremendous carries no rotation/velocity state, so an F5 bucket other
//! than the original recorder's diverges from its own rec_coords at frame
//! ~249. That drift is not what this measures: the question is whether STOP
//! adds non-determinism on top of it, so the judge is "second playback ==
//! reference playback", not "playback == recording".
//!
//! The test captures one reference play, then repeatedly stops and restarts at
//! different frames. It rerolls F5 until the reference bucket returns and
//! compares the replay coordinates bit for bit.

use std::thread;
use std::time::{Duration, Instant};

use crate::{drift, harness, replay};

const VERIFY_FRAMES: u32 = 1000;
/// Stop points spread across the recording; the count is the iteration count.
const STOP_AT_FRAMES: [u32; 10] = [1200, 800, 2000, 1500, 2400, 1800, 1100, 2200, 900, 1700];
const RECORDING: &str = "FE-tremendous.tasrec";
/// F5 rerolls allowed for the second PLAY to land the reference bucket.
const REF_MATCH_RETRIES: u32 = 30;
/// Leading frames the retry loop checks against the reference; covers the
/// stationary phase plus enough motion for a rotation mismatch to surface.
const RETRY_VERIFY_FRAMES: u32 = 300;
/// The reference must travel at least this far over the verify window, or
/// the comparison is zero-vs-zero.
const MIN_REFERENCE_TRAVEL: f64 = 0.1;

struct CycleResult {
    iteration: u32,
    stop_at: u32,
    started_ok: bool,
    second_play_ok: bool,
    /// Max absolute coordinate difference between this iteration's second
    /// playback and the reference over the first VERIFY_FRAMES.
    max_dx_vs_ref: f64,
    max_dz_vs_ref: f64,
    /// First frame where the second playback differs from the reference at
    /// the bit level. None = matches all frames.
    first_div_frame: Option<usize>,
}

/// tas_ui's PLAY button flow: in-process restart, then ARM_PLAY, whatever F5
/// bucket the game lands on.
fn restart_then_play(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    if !harness::restart_and_stabilize_inprocess(client) {
        return false;
    }
    harness::arm_play(client);
    thread::sleep(Duration::from_millis(150));
    client.mode_volatile() == tas_shared::TasMode::Play as u32
}

fn first_bit_divergence(play: &[[f32; 3]], reference: &[[f32; 3]]) -> Option<usize> {
    play.iter()
        .zip(reference)
        .position(|(p, r)| p.iter().zip(r).any(|(a, b)| a.to_bits() != b.to_bits()))
}

/// Restart + ARM_PLAY, rerolling F5 until the first RETRY_VERIFY_FRAMES frames
/// match the reference bit-for-bit.
fn restart_play_match_reference(
    client: &mut tas_shared::TasSharedMemoryClient,
    reference: &[[f32; 3]],
    rec: &replay::LoadedRecording,
    max_retries: u32,
) -> bool {
    for attempt in 0..=max_retries {
        replay::write_to_shared(client, rec);

        if !restart_then_play(client) {
            eprintln!("  Restart-then-play failed on attempt {}", attempt + 1);
            continue;
        }

        let _ = wait_for_pos(client, RETRY_VERIFY_FRAMES, Duration::from_secs(20));

        let window = RETRY_VERIFY_FRAMES as usize;
        let state = client.state();
        match first_bit_divergence(&state.play_coords[..window], &reference[..window]) {
            None => {
                println!(
                    "  Reference-trajectory match (attempt {}, {} frames verified)",
                    attempt + 1,
                    RETRY_VERIFY_FRAMES
                );
                return true;
            }
            Some(frame) => println!(
                "  Reference-trajectory mismatch at frame {} (attempt {}/{}) — retrying for matching F5 bucket",
                frame,
                attempt + 1,
                max_retries
            ),
        }
        harness::stop(client);
        thread::sleep(Duration::from_millis(200));
    }
    eprintln!(
        "  Could not match reference trajectory after {} retries",
        max_retries
    );
    false
}

/// Wait until playback_pos reaches `target` or the timeout elapses; returns
/// the position seen.
fn wait_for_pos(client: &tas_shared::TasSharedMemoryClient, target: u32, timeout: Duration) -> u32 {
    let start = Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        if pos >= target || start.elapsed() > timeout {
            return pos;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

pub fn run() -> bool {
    let iterations = STOP_AT_FRAMES.len() as u32;
    println!(
        "=== Stop+Play Flakiness Test (FE-tremendous, baseline-comparison, {}× iters) ===\n",
        iterations
    );

    let path = match harness::fixture_path(RECORDING) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
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

    println!("\n--- Phase 0: Reference playback (no stop) ---");
    replay::write_to_shared(&mut client, &rec);

    if !restart_then_play(&mut client) {
        eprintln!("ERROR: Reference playback couldn't start");
        return false;
    }
    let _ = wait_for_pos(&client, VERIFY_FRAMES, Duration::from_secs(60));
    let reference: Vec<[f32; 3]> = client.state().play_coords[..VERIFY_FRAMES as usize].to_vec();
    let player_ptr = client.state().player_ptr;
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
    // A reference that never moved (no player, no coordinate capture) would
    // make every bit-for-bit comparison vacuously clean.
    let (ref_dx, ref_dy, ref_dz) = drift::compute_movement(&reference, reference.len());
    let reference_travel = ref_dx.max(ref_dy).max(ref_dz);
    if player_ptr == 0 {
        eprintln!("FAIL: player_ptr is 0 — the reference playback captured no player");
        return false;
    }
    if reference_travel < MIN_REFERENCE_TRAVEL {
        eprintln!(
            "FAIL: reference playback never moved (max travel {:.4} over {} frames)",
            reference_travel, VERIFY_FRAMES
        );
        return false;
    }
    println!(
        "  Reference travel over the verify window: {:.3}",
        reference_travel
    );

    let mut results: Vec<CycleResult> = Vec::with_capacity(STOP_AT_FRAMES.len());
    for (i, &stop_at) in STOP_AT_FRAMES.iter().enumerate() {
        let iteration = i as u32 + 1;
        println!(
            "\n--- Iteration {}/{} (stop at frame {}) ---",
            iteration, iterations, stop_at
        );

        replay::write_to_shared(&mut client, &rec);

        if !restart_then_play(&mut client) {
            println!("  First playback: restart failed");
            results.push(CycleResult {
                iteration,
                stop_at,
                started_ok: false,
                second_play_ok: false,
                max_dx_vs_ref: f64::INFINITY,
                max_dz_vs_ref: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }

        let actual_stop = wait_for_pos(&client, stop_at, Duration::from_secs(60));
        println!("  STOP at playback_pos={}", actual_stop);
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        let second_ok =
            restart_play_match_reference(&mut client, &reference, &rec, REF_MATCH_RETRIES);
        if !second_ok {
            println!("  Second playback: reference match failed after retries");
            results.push(CycleResult {
                iteration,
                stop_at,
                started_ok: true,
                second_play_ok: false,
                max_dx_vs_ref: f64::INFINITY,
                max_dz_vs_ref: f64::INFINITY,
                first_div_frame: None,
            });
            continue;
        }

        let _ = wait_for_pos(&client, VERIFY_FRAMES, Duration::from_secs(60));

        let state = client.state();
        let play = &state.play_coords[..VERIFY_FRAMES as usize];
        let mut max_dx = 0.0f64;
        let mut max_dz = 0.0f64;
        for (p, r) in play.iter().zip(&reference) {
            max_dx = max_dx.max((p[0] as f64 - r[0] as f64).abs());
            max_dz = max_dz.max((p[2] as f64 - r[2] as f64).abs());
        }
        let first_div = first_bit_divergence(play, &reference);

        println!(
            "  Second playback: vs reference dx_max={:.6} dz_max={:.6} first_div={}",
            max_dx,
            max_dz,
            first_div
                .map(|f| f.to_string())
                .unwrap_or_else(|| "matches all".into())
        );

        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));

        results.push(CycleResult {
            iteration,
            stop_at,
            started_ok: true,
            second_play_ok: true,
            max_dx_vs_ref: max_dx,
            max_dz_vs_ref: max_dz,
            first_div_frame: first_div,
        });
    }

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
        clean, iterations, diverged, start_failed
    );

    let pass = clean == iterations;
    if pass {
        println!(
            "\n*** STOP+PLAY FLAKE TEST PASSED: {}/{} iterations matched the reference playback ***",
            clean, iterations
        );
    } else {
        println!(
            "\n*** STOP+PLAY FLAKE TEST FAILED: {} of {} iterations diverged from the reference ***",
            diverged, iterations
        );
    }
    pass
}
