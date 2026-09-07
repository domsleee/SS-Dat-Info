//! Playback speed verification: variable-speed playback (Cave 5) scales the
//! tick rate proportionally, measured as ticks recorded over a fixed wall
//! clock at 0.25x, 1x and 2x.
//!
//! Pass criteria:
//!   - 0.25x tick count is 15-35% of baseline (1x)
//!   - 2.0x tick count is 165-235% of baseline (1x)
//!   - Speed resets to 1.0x after test

use crate::harness;
use std::thread;
use std::time::Duration;

/// Duration for each speed measurement (seconds).
const MEASURE_SECS: u64 = 3;

/// Tolerance bands (ratio of measured ticks to baseline ticks), wider than
/// theoretical because of tick accumulator rounding and frame timing jitter.
const SLOW_MIN: f64 = 0.15;
const SLOW_MAX: f64 = 0.35;
const FAST_MIN: f64 = 1.65;
const FAST_MAX: f64 = 2.35;

/// Measure tick count at a given playback_speed over MEASURE_SECS of recording.
fn measure_ticks_at_speed(client: &mut tas_shared::TasSharedMemoryClient, speed: f32) -> u32 {
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {}", speed);

    if !harness::restart_and_stabilize(client) {
        eprintln!("  ERROR: Game not alive at speed {}", speed);
        return 0;
    }

    harness::arm_rec(client);
    println!("  Recording for {}s at {}x...", MEASURE_SECS, speed);
    thread::sleep(Duration::from_secs(MEASURE_SECS));

    let ticks = client.state().recorded_count;
    harness::stop(client);
    println!("  Ticks at {}x: {}", speed, ticks);
    ticks
}

pub fn run() -> bool {
    println!("=== Playback Speed Verification Test ===\n");

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::ensure_exclusive_runtime_ownership(&mut client, "REC speed scaling failures");
    if !harness::require_speed_preconditions(&client) {
        return false;
    }

    println!("--- Phase 1: Baseline (1.0x) ---");
    let baseline_ticks = measure_ticks_at_speed(&mut client, 1.0);
    if baseline_ticks == 0 {
        eprintln!("ERROR: No baseline ticks recorded");
        return false;
    }

    println!("\n--- Phase 2: Slow motion (0.25x) ---");
    let slow_ticks = measure_ticks_at_speed(&mut client, 0.25);

    println!("\n--- Phase 3: Fast forward (2.0x) ---");
    let fast_ticks = measure_ticks_at_speed(&mut client, 2.0);

    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    let slow_ratio = slow_ticks as f64 / baseline_ticks as f64;
    let fast_ratio = fast_ticks as f64 / baseline_ticks as f64;
    let slow_pass = (SLOW_MIN..=SLOW_MAX).contains(&slow_ratio);
    let fast_pass = (FAST_MIN..=FAST_MAX).contains(&fast_ratio);

    println!("\n=== SPEED TEST RESULTS ===");
    println!("Baseline (1.0x): {} ticks", baseline_ticks);
    println!(
        "Slow (0.25x):    {} ticks (ratio {:.3}, expected {:.2}-{:.2}) — {}",
        slow_ticks,
        slow_ratio,
        SLOW_MIN,
        SLOW_MAX,
        if slow_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "Fast (2.0x):     {} ticks (ratio {:.3}, expected {:.2}-{:.2}) — {}",
        fast_ticks,
        fast_ratio,
        FAST_MIN,
        FAST_MAX,
        if fast_pass { "PASS" } else { "FAIL" }
    );

    let pass = slow_pass && fast_pass;
    if pass {
        println!("\n*** SPEED TEST PASSED ***");
    } else {
        println!("\n*** SPEED TEST FAILED ***");
    }
    pass
}
