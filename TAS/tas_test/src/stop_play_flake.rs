//! STOP+PLAY flakiness against FE-tremendous: PLAY, STOP mid-playback, PLAY
//! again, and require the second playback to reproduce the recording bit for
//! bit from its gate. The question is whether a mid-run STOP leaves state
//! behind that changes the next replay, so both PLAYs are armed without the
//! watcher: its rerolls would hide exactly that.

use std::thread;
use std::time::{Duration, Instant};

use crate::{drift, harness, replay};

/// Gate-relative frames the second playback must reproduce exactly.
const VERIFY_FRAMES: u32 = 1000;
/// Stop points spread across the recording; the count is the iteration count.
const STOP_AT_FRAMES: [u32; 10] = [1200, 800, 2000, 1500, 2400, 1800, 1100, 2200, 900, 1700];
const RECORDING: &str = "FE-tremendous.tasrec";

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

/// One iteration: PLAY, STOP at `stop_at`, PLAY again. Returns the second
/// playback's gate-relative drift over VERIFY_FRAMES.
fn stop_then_replay(
    client: &mut tas_shared::TasSharedMemoryClient,
    rec: &replay::LoadedRecording,
    stop_at: u32,
) -> Result<drift::DriftResult, String> {
    replay::write_to_shared(client, rec);
    harness::restart_play_aligned_unwatched(client)?;
    let reached = wait_for_pos(client, stop_at, Duration::from_secs(60));
    if reached < stop_at || client.mode_volatile() != tas_shared::TasMode::Play as u32 {
        harness::stop(client);
        return Err(format!("did not reach the mid-PLAY stop point {stop_at}"));
    }
    println!("  STOP at playback_pos={}", reached);
    harness::stop(client);
    thread::sleep(Duration::from_millis(200));

    replay::write_to_shared(client, rec);
    let (rec_gate, play_gate) = harness::restart_play_aligned_unwatched(client)?;
    let window = VERIFY_FRAMES.min(rec.count.saturating_sub(rec_gate));
    let end = play_gate + window;
    if wait_for_pos(client, end, Duration::from_secs(60)) < end {
        harness::stop(client);
        return Err(format!("the second PLAY did not reach gate + {window}"));
    }
    // A failed capture leaves a stale coordinate from an earlier replay in its
    // slot, which could match; without the watcher, check it here.
    if client.state().capture_ok == 0 {
        harness::stop(client);
        return Err("a coordinate capture failed during the second PLAY".into());
    }
    let d = drift::compute_gate_relative_drift(client.state(), rec_gate, play_gate, window);
    harness::stop(client);
    thread::sleep(Duration::from_millis(200));
    Ok(d)
}

pub fn run(iterations: u32) -> bool {
    if iterations == 0 || iterations > STOP_AT_FRAMES.len() as u32 {
        eprintln!("Require 1..={} stop points", STOP_AT_FRAMES.len());
        return false;
    }
    println!(
        "=== Stop+Play Flakiness Test (FE-tremendous, {} iterations) ===\n",
        iterations
    );

    let rec = match harness::fixture_path(RECORDING).and_then(|p| replay::load_tasrec(&p)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    println!("  Loaded {} ticks", rec.count);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut clean = 0u32;
    for (i, &stop_at) in STOP_AT_FRAMES.iter().take(iterations as usize).enumerate() {
        println!(
            "\n--- Iteration {}/{} (stop at frame {}) ---",
            i + 1,
            iterations,
            stop_at
        );
        match stop_then_replay(&mut client, &rec, stop_at) {
            Ok(d) if d.is_zero() => {
                println!("  Second playback: bit-identical to the recording");
                clean += 1;
            }
            Ok(d) => println!(
                "  Second playback DIVERGED: X={:.9} (rec frame {}) Z={:.9} (rec frame {})",
                d.max_drift_x, d.max_drift_frame_x, d.max_drift_z, d.max_drift_frame_z
            ),
            Err(e) => println!("  FAIL: {e}"),
        }
    }

    let pass = clean == iterations;
    println!(
        "\n*** STOP+PLAY FLAKE TEST {}: {}/{} iterations matched the recording ***",
        if pass { "PASSED" } else { "FAILED" },
        clean,
        iterations
    );
    pass
}
