//! Playback speed verification test (SSB-162).
//!
//! Validates that variable speed playback (Cave 5) actually scales the game
//! tick rate proportionally. Tests 0.25x, 1.0x, and 2.0x speeds by measuring
//! tick count over a fixed wall-clock duration.
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

/// Tolerance bands (ratio of measured ticks to baseline ticks).
/// Wider than theoretical because tick accumulator rounding + frame timing jitter.
const SLOW_MIN: f64 = 0.15;
const SLOW_MAX: f64 = 0.35;
const FAST_MIN: f64 = 1.65;
const FAST_MAX: f64 = 2.35;

#[derive(Debug)]
pub struct SpeedResult {
    pub baseline_ticks: u32,
    pub slow_ticks: u32,
    pub fast_ticks: u32,
    pub slow_ratio: f64,
    pub fast_ratio: f64,
    pub slow_pass: bool,
    pub fast_pass: bool,
}

impl SpeedResult {
    pub fn all_pass(&self) -> bool {
        self.slow_pass && self.fast_pass
    }
}

/// Measure tick count at a given playback_speed over MEASURE_SECS of recording.
fn measure_ticks_at_speed(client: &mut tas_shared::TasSharedMemoryClient, speed: f32) -> u32 {
    // Set playback speed
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {}", speed);

    // F5 restart + stabilize
    if !harness::restart_and_stabilize(client) {
        eprintln!("  ERROR: Game not alive at speed {}", speed);
        return 0;
    }

    // REC for MEASURE_SECS
    harness::arm_rec(client);
    println!("  Recording for {}s at {}x...", MEASURE_SECS, speed);
    thread::sleep(Duration::from_secs(MEASURE_SECS));

    let ticks = client.state().recorded_count;
    harness::stop(client);
    println!("  Ticks at {}x: {}", speed, ticks);
    ticks
}

/// Run the playback speed verification test.
pub fn run() -> SpeedResult {
    println!("=== Playback Speed Verification Test (SSB-162) ===\n");

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::ensure_exclusive_runtime_ownership(&mut client, "REC speed scaling failures");

    // Verify Cave 5 is hooked (required for speed scaling)
    {
        let s = client.state();
        assert_eq!(s.cave5_hooked, 1, "Cave 5 must be hooked for speed test");
        assert_eq!(
            s.force_fixed_tick, 0,
            "force_fixed_tick must be 0 (would override speed)"
        );
    }

    // Phase 1: Baseline at 1.0x
    println!("--- Phase 1: Baseline (1.0x) ---");
    let baseline_ticks = measure_ticks_at_speed(&mut client, 1.0);
    if baseline_ticks == 0 {
        eprintln!("ERROR: No baseline ticks recorded");
        std::process::exit(1);
    }

    // Phase 2: Slow motion at 0.25x
    println!("\n--- Phase 2: Slow motion (0.25x) ---");
    let slow_ticks = measure_ticks_at_speed(&mut client, 0.25);

    // Phase 3: Fast forward at 2.0x
    println!("\n--- Phase 3: Fast forward (2.0x) ---");
    let fast_ticks = measure_ticks_at_speed(&mut client, 2.0);

    // Reset speed to 1.0x
    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    // Compute ratios
    let slow_ratio = slow_ticks as f64 / baseline_ticks as f64;
    let fast_ratio = fast_ticks as f64 / baseline_ticks as f64;

    let slow_pass = (SLOW_MIN..=SLOW_MAX).contains(&slow_ratio);
    let fast_pass = (FAST_MIN..=FAST_MAX).contains(&fast_ratio);

    let result = SpeedResult {
        baseline_ticks,
        slow_ticks,
        fast_ticks,
        slow_ratio,
        fast_ratio,
        slow_pass,
        fast_pass,
    };

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

    if result.all_pass() {
        println!("\n*** SPEED TEST PASSED ***");
    } else {
        println!("\n*** SPEED TEST FAILED ***");
    }

    result
}
