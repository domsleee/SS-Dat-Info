//! Drift-at-speed verification test (SSB-186).
//!
//! Validates that speed scaling (Cave 5) does NOT introduce drift:
//!
//!   Case 1: REC at 2x + PLAY at 2x → zero drift (same-speed symmetry)
//!   Case 2: REC at 1x + PLAY at 2x → zero drift (cross-speed is physics-transparent)
//!
//! Why cross-speed is zero-drift:
//!   The time advance constant (EXE+0x6DB08) only controls how many ticks per
//!   second the game processes. Each tick's physics step is identical regardless
//!   of the constant's value. Since the input log is per-tick, replaying the
//!   same tick sequence at any speed produces identical coordinate trajectories.
//!
//! Uses Pico HID for steering (required for zero-drift baseline).
//! Uses F5 position matching for bit-exact starting positions.

use crate::{drift, gates, harness, patterns};
use std::thread;
use std::time::Duration;

/// Steering pattern: L-R-L-R for clear directional transitions.
const PATTERN: &str = "LRLR";
const HOLD_TICKS: u32 = 56;
const GAP_TICKS: u32 = 0;
/// Extra neutral ticks after the steering pattern to let physics settle.
const TAIL_NEUTRAL_TICKS: u32 = 100;

#[derive(Debug)]
pub struct DriftSpeedResult {
    /// Case 1: same-speed 2x REC + 2x PLAY
    pub same_speed_rec_count: u32,
    pub same_speed_drift: drift::DriftResult,
    pub same_speed_pass: bool,

    /// Case 2: cross-speed 1x REC + 2x PLAY
    pub cross_speed_rec_count: u32,
    pub cross_speed_drift: drift::DriftResult,
    pub cross_speed_pass: bool,
}

impl DriftSpeedResult {
    pub fn all_pass(&self) -> bool {
        self.same_speed_pass && self.cross_speed_pass
    }
}

/// Run a REC phase at the given speed with Pico HID steering.
/// Returns (rec_count, rec_coords[0]) or exits on failure.
fn rec_at_speed(client: &mut tas_shared::TasSharedMemoryClient, speed: f32) -> (u32, [f32; 3]) {
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {} for REC", speed);

    if !harness::restart_and_stabilize(client) {
        eprintln!("ERROR: Game not alive for REC at {}x", speed);
        std::process::exit(1);
    }

    harness::arm_rec(client);

    // Build steering pattern + neutral tail
    let mut steps = patterns::build_from_pattern(PATTERN, HOLD_TICKS, GAP_TICKS);
    let last_stop = patterns::total_ticks(&steps);
    steps.push(patterns::PatternStep {
        name: "TAIL".into(),
        mask: 0x00,
        stop_tick: last_stop + TAIL_NEUTRAL_TICKS,
    });

    println!(
        "  Driving Pico HID: pattern={} hold={} gap={} + {}t tail ({} total ticks)",
        PATTERN,
        HOLD_TICKS,
        GAP_TICKS,
        TAIL_NEUTRAL_TICKS,
        patterns::total_ticks(&steps)
    );
    harness::drive_pico_steps(&steps, None);

    // Small settle time for last ticks to register
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
        eprintln!("ERROR: Too few ticks recorded ({}) at {}x", rec_count, speed);
        std::process::exit(1);
    }

    (rec_count, rec_start)
}

/// Run a PLAY phase at the given speed with F5 position matching.
/// Returns true if playback completed.
fn play_at_speed(
    client: &mut tas_shared::TasSharedMemoryClient,
    speed: f32,
    rec_start: [f32; 3],
    rec_count: u32,
) -> bool {
    client.state_mut().playback_speed = speed;
    println!("  Set playback_speed = {} for PLAY", speed);

    if !harness::restart_play_and_match(client, rec_start, 20) {
        eprintln!("WARNING: Could not match position for PLAY at {}x", speed);
        return false;
    }

    harness::wait_playback(client, rec_count)
}

/// Run the drift-at-speed verification test.
pub fn run() -> DriftSpeedResult {
    println!("=== Drift-at-Speed Verification Test (SSB-186) ===\n");

    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Cave 2 not firing");
        std::process::exit(1);
    }

    // Verify Cave 5 is hooked and fft=0
    {
        let s = client.state();
        assert_eq!(s.cave5_hooked, 1, "Cave 5 must be hooked for speed test");
        assert_eq!(
            s.force_fixed_tick, 0,
            "force_fixed_tick must be 0 for speed test"
        );
    }

    // ======== Case 1: REC at 2x + PLAY at 2x (expect zero drift) ========
    println!("--- Case 1: REC at 2x + PLAY at 2x (same-speed) ---\n");

    let (rec_count_2x, rec_start_2x) = rec_at_speed(&mut client, 2.0);

    println!("\n  Starting PLAY at 2x...");
    let play_ok_same = play_at_speed(&mut client, 2.0, rec_start_2x, rec_count_2x);
    if !play_ok_same {
        eprintln!("WARNING: Playback did not complete for case 1");
    }

    let same_speed_drift = drift::compute_drift(client.state(), rec_count_2x);
    let same_speed_assessment = gates::run_gates(client.state(), rec_count_2x);
    same_speed_assessment.print_summary();

    println!(
        "\n  Case 1 drift: X={:.9} (frame {}) Z={:.9} (frame {})",
        same_speed_drift.max_drift_x,
        same_speed_drift.max_drift_frame_x,
        same_speed_drift.max_drift_z,
        same_speed_drift.max_drift_frame_z,
    );
    let same_speed_pass = same_speed_drift.is_zero();
    println!(
        "  Case 1 verdict: {} (expect zero drift)",
        if same_speed_pass { "PASS" } else { "FAIL" }
    );

    // ======== Case 2: REC at 1x + PLAY at 2x (expect zero drift — speed is physics-transparent) ========
    println!("\n--- Case 2: REC at 1x + PLAY at 2x (cross-speed) ---\n");

    let (rec_count_1x, rec_start_1x) = rec_at_speed(&mut client, 1.0);

    println!("\n  Starting PLAY at 2x...");
    let play_ok_cross = play_at_speed(&mut client, 2.0, rec_start_1x, rec_count_1x);
    if !play_ok_cross {
        eprintln!("WARNING: Playback did not complete for case 2");
    }

    let cross_speed_drift = drift::compute_drift(client.state(), rec_count_1x);
    harness::print_results(&client);

    println!(
        "\n  Case 2 drift: X={:.9} (frame {}) Z={:.9} (frame {})",
        cross_speed_drift.max_drift_x,
        cross_speed_drift.max_drift_frame_x,
        cross_speed_drift.max_drift_z,
        cross_speed_drift.max_drift_frame_z,
    );
    let cross_speed_pass = cross_speed_drift.is_zero();
    println!(
        "  Case 2 verdict: {} (expect zero drift — speed scaling is physics-transparent)",
        if cross_speed_pass { "PASS" } else { "FAIL" }
    );

    // Reset speed
    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    // ======== Summary ========
    let result = DriftSpeedResult {
        same_speed_rec_count: rec_count_2x,
        same_speed_drift,
        same_speed_pass,
        cross_speed_rec_count: rec_count_1x,
        cross_speed_drift,
        cross_speed_pass,
    };

    println!("\n=== DRIFT-AT-SPEED TEST RESULTS ===");
    println!(
        "Case 1 (2x REC + 2x PLAY): drift X={:.9} Z={:.9} — {}",
        result.same_speed_drift.max_drift_x,
        result.same_speed_drift.max_drift_z,
        if result.same_speed_pass { "PASS" } else { "FAIL" },
    );
    println!(
        "Case 2 (1x REC + 2x PLAY): drift X={:.9} Z={:.9} — {}",
        result.cross_speed_drift.max_drift_x,
        result.cross_speed_drift.max_drift_z,
        if result.cross_speed_pass { "PASS" } else { "FAIL" },
    );

    if result.all_pass() {
        println!("\n*** DRIFT-AT-SPEED TEST PASSED ***");
    } else {
        println!("\n*** DRIFT-AT-SPEED TEST FAILED ***");
    }

    result
}
