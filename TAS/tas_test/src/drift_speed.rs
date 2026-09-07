//! Drift-at-speed verification: speed scaling (Cave 5) does not introduce
//! drift.
//!
//!   Case 1: REC at 2x + PLAY at 2x → zero drift (same-speed symmetry)
//!   Case 2: REC at 1x + PLAY at 2x → zero drift (cross-speed is physics-transparent)
//!
//! The time advance constant only controls how many ticks per second the game
//! processes; each tick's physics step is identical, and the input log is
//! per-tick, so the same tick sequence replays to the same trajectory at any
//! speed. Steering comes from the Pico; starts are F5 position-matched.

use crate::{drift, gates, harness, patterns};
use std::thread;
use std::time::Duration;

/// Steering pattern: L-R-L-R for clear directional transitions.
const PATTERN: &str = "LRLR";
const HOLD_TICKS: u32 = 56;
const GAP_TICKS: u32 = 0;
/// Extra neutral ticks after the steering pattern to let physics settle.
const TAIL_NEUTRAL_TICKS: u32 = 100;

/// REC at `speed` with Pico steering. Returns `(rec_count, rec_coords[0])`.
fn rec_at_speed(
    client: &mut tas_shared::TasSharedMemoryClient,
    speed: f32,
) -> Option<(u32, [f32; 3])> {
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {} for REC", speed);

    if !harness::restart_and_stabilize(client) {
        eprintln!("ERROR: Game not alive for REC at {}x", speed);
        return None;
    }

    harness::arm_rec(client);

    let steps = patterns::with_neutral_tail(
        patterns::build_from_pattern(PATTERN, HOLD_TICKS, GAP_TICKS),
        TAIL_NEUTRAL_TICKS,
    );
    println!(
        "  Driving Pico HID: pattern={} hold={} gap={} + {}t tail ({} total ticks)",
        PATTERN,
        HOLD_TICKS,
        GAP_TICKS,
        TAIL_NEUTRAL_TICKS,
        patterns::total_ticks(&steps)
    );
    harness::drive_pico_steps(&steps, None);
    thread::sleep(Duration::from_millis(200));

    let rec_count = client.state().recorded_count;
    let rec_start = client.state().rec_coords[0];
    harness::stop(client);

    println!("  Recorded {} ticks at {}x", rec_count, speed);
    println!(
        "  REC start: ({:.4}, {:.4}, {:.4})",
        rec_start[0], rec_start[1], rec_start[2]
    );

    if rec_count < 100 {
        eprintln!(
            "ERROR: Too few ticks recorded ({}) at {}x",
            rec_count, speed
        );
        return None;
    }
    Some((rec_count, rec_start))
}

/// PLAY at `speed` with F5 position matching. Returns true if playback completed.
fn play_at_speed(
    client: &mut tas_shared::TasSharedMemoryClient,
    speed: f32,
    rec_start: [f32; 3],
    rec_count: u32,
) -> bool {
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {} for PLAY", speed);

    if !harness::restart_play_and_match(client, rec_start, harness::START_MATCH_RETRIES) {
        eprintln!("WARNING: Could not match position for PLAY at {}x", speed);
        return false;
    }

    harness::wait_playback(client, rec_count)
}

/// One REC/PLAY case. Completion is part of the verdict: `compute_drift` only
/// inspects frames that played, so a stalled replay reads as clean drift over
/// a truncated window. The gate assessment is printed but not folded in: its
/// movement gates are Z-only and unrelated to speed transparency.
fn case(
    client: &mut tas_shared::TasSharedMemoryClient,
    label: &str,
    rec_speed: f32,
    play_speed: f32,
) -> Option<bool> {
    let (rec_count, rec_start) = rec_at_speed(client, rec_speed)?;
    println!("\n  Starting PLAY at {}x...", play_speed);
    let play_ok = play_at_speed(client, play_speed, rec_start, rec_count);
    if !play_ok {
        eprintln!("WARNING: Playback did not complete for {label}");
    }
    let d = drift::compute_drift(client.state(), rec_count);
    gates::run_gates(client.state(), rec_count).print_summary();
    println!(
        "\n  {label} drift: X={:.9} (frame {}) Y={:.9} Z={:.9} (frame {})",
        d.max_drift_x, d.max_drift_frame_x, d.max_drift_y, d.max_drift_z, d.max_drift_frame_z,
    );
    let pass = d.is_zero() && play_ok;
    println!(
        "  {label} verdict: {} (expect zero drift)",
        if pass { "PASS" } else { "FAIL" }
    );
    Some(pass)
}

pub fn run() -> bool {
    println!("=== Drift-at-Speed Verification Test ===\n");

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    if !harness::require_speed_preconditions(&client) {
        return false;
    }

    println!("--- Case 1: REC at 2x + PLAY at 2x (same-speed) ---\n");
    let Some(same_speed_pass) = case(&mut client, "Case 1", 2.0, 2.0) else {
        return false;
    };

    println!("\n--- Case 2: REC at 1x + PLAY at 2x (cross-speed) ---\n");
    let Some(cross_speed_pass) = case(&mut client, "Case 2", 1.0, 2.0) else {
        return false;
    };
    harness::print_results(&client);

    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    println!("\n=== DRIFT-AT-SPEED TEST RESULTS ===");
    println!(
        "Case 1 (2x REC + 2x PLAY): {}",
        if same_speed_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "Case 2 (1x REC + 2x PLAY): {}",
        if cross_speed_pass { "PASS" } else { "FAIL" }
    );

    let pass = same_speed_pass && cross_speed_pass;
    if pass {
        println!("\n*** DRIFT-AT-SPEED TEST PASSED ***");
    } else {
        println!("\n*** DRIFT-AT-SPEED TEST FAILED ***");
    }
    pass
}
