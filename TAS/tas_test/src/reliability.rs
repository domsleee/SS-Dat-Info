//! Reliability test: N consecutive steered REC+PLAY cycles at one speed, each
//! with zero drift. REC and PLAY run at the same speed (speed scaling is
//! physics-transparent), so the full catch-up pipeline is exercised without
//! mixed-speed stabilization affecting the position match.

use crate::{drift, gates, harness, patterns};
use std::thread;
use std::time::Duration;

/// Steering pattern for each REC cycle — L-R-L-R for clear directional transitions.
const PATTERN: &str = "LRLR";
const HOLD_TICKS: u32 = 56;
const GAP_TICKS: u32 = 0;
/// Neutral tail after steering to let physics settle.
const TAIL_NEUTRAL_TICKS: u32 = 50;

#[derive(Debug)]
pub struct CycleResult {
    pub iteration: u32,
    pub rec_count: u32,
    pub transitions: u32,
    pub max_drift_x: f64,
    pub max_drift_y: f64,
    pub max_drift_z: f64,
    pub position_matched: bool,
    pub playback_complete: bool,
    pub all_gates_pass: bool,
}

impl CycleResult {
    /// Zero drift on every axis, playback finished, gates green.
    fn is_clean(&self) -> bool {
        self.max_drift_x == 0.0
            && self.max_drift_y == 0.0
            && self.max_drift_z == 0.0
            && self.playback_complete
            && self.all_gates_pass
    }
}

#[derive(Debug)]
pub struct ReliabilityReport {
    pub iterations: u32,
    pub speed: f32,
    pub results: Vec<CycleResult>,
}

impl ReliabilityReport {
    pub fn all_pass(&self) -> bool {
        // `.all()` on an empty set is vacuously true — a run that completed zero
        // cycles would report success. Require at least one measured cycle.
        !self.results.is_empty() && self.results.iter().all(|r| r.is_clean())
    }

    pub fn print_summary(&self) {
        println!("\n=== RELIABILITY TEST SUMMARY ===");
        println!("Speed: {}x (same-speed REC+PLAY)", self.speed);
        println!("Iterations: {}", self.iterations);
        println!();
        println!(
            "{:>3} {:>6} {:>5} {:>12} {:>12} {:>12} {:>6} {:>6} {:>5}",
            "#",
            "ticks",
            "trans",
            "max_drift_x",
            "max_drift_y",
            "max_drift_z",
            "pos",
            "play",
            "gates"
        );
        println!("{}", "-".repeat(86));

        let mut fail_count = 0;
        for r in &self.results {
            if !r.is_clean() {
                fail_count += 1;
            }
            println!(
                "{:>3} {:>6} {:>5} {:>12.9} {:>12.9} {:>12.9} {:>6} {:>6} {:>5}",
                r.iteration,
                r.rec_count,
                r.transitions,
                r.max_drift_x,
                r.max_drift_y,
                r.max_drift_z,
                if r.position_matched { "ok" } else { "MISS" },
                if r.playback_complete { "ok" } else { "FAIL" },
                if r.all_gates_pass { "ok" } else { "FAIL" },
            );
        }

        println!();
        if fail_count == 0 {
            println!(
                "*** RELIABILITY TEST PASSED: {}/{} zero drift at {}x ***",
                self.results.len(),
                self.iterations,
                self.speed
            );
        } else {
            println!(
                "*** RELIABILITY TEST FAILED: {}/{} had issues ***",
                fail_count, self.iterations
            );
        }
    }
}

pub fn run(iterations: u32, speed: f32) -> bool {
    println!(
        "=== Reliability Test: {}x REC+PLAY at {}x speed ===\n",
        iterations, speed
    );

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // tas_ui writes playback_speed = 1.0 every frame from its own state, which
    // would collapse the test back to 1x.
    harness::ensure_exclusive_runtime_ownership(&mut client, "reliability speed scaling");

    harness::assert_proven_config(&client);
    if !harness::require_speed_preconditions(&client) {
        return false;
    }

    let mut results = Vec::new();

    for i in 1..=iterations {
        println!("\n{}", "=".repeat(60));
        println!("  Cycle {}/{}", i, iterations);
        println!("{}", "=".repeat(60));

        client.state_mut().playback_speed = speed;
        println!("\n--- REC at {}x ---", speed);

        if !harness::restart_and_stabilize(&client) {
            eprintln!("ERROR: Game not alive for REC cycle {}", i);
            return false;
        }

        // No focus_game between restart and arm: the extra ~400 ms of slide
        // before rec_coords[0] is captured breaks the PLAY position match.
        harness::arm_rec(&mut client);

        let steps = patterns::with_neutral_tail(
            patterns::build_from_pattern(PATTERN, HOLD_TICKS, GAP_TICKS),
            TAIL_NEUTRAL_TICKS,
        );

        println!(
            "  Driving Pico HID: {} + {}t tail ({} total ticks)",
            PATTERN,
            TAIL_NEUTRAL_TICKS,
            patterns::total_ticks(&steps)
        );
        harness::drive_pico_steps(&steps, None);
        thread::sleep(Duration::from_millis(200));

        let rec_count = client.state().recorded_count;
        let rec_start = client.state().rec_coords[0];
        harness::stop(&mut client);

        let transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);

        println!(
            "  Recorded {} ticks, {} transitions",
            rec_count, transitions
        );
        println!(
            "  REC start: ({:.4}, {:.4}, {:.4})",
            rec_start[0], rec_start[1], rec_start[2]
        );

        if rec_count < 100 {
            eprintln!(
                "ERROR: Too few ticks recorded ({}) in cycle {}",
                rec_count, i
            );
            return false;
        }

        println!("\n--- PLAY at {}x (same-speed) ---", speed);

        let matched =
            harness::restart_play_and_match(&mut client, rec_start, harness::START_MATCH_RETRIES);
        if !matched {
            println!("  WARNING: Position match failed for cycle {}", i);
        }

        let play_ok = harness::wait_playback(&client, rec_count);
        if !play_ok {
            println!("  WARNING: Playback timeout in cycle {}", i);
        }

        // Compute drift
        let state = client.state();
        let d = drift::compute_drift(state, rec_count.min(state.playback_pos));
        let assessment = gates::run_gates(state, rec_count.min(state.playback_pos));

        println!(
            "  Drift: X={:.9} (frame {}) Z={:.9} (frame {})",
            d.max_drift_x, d.max_drift_frame_x, d.max_drift_z, d.max_drift_frame_z
        );
        let gates_passed = assessment.gates.iter().filter(|g| g.passed).count();
        println!(
            "  Gates: {} ({}/4 pass)",
            if assessment.all_pass() {
                "ALL PASS"
            } else {
                "FAIL"
            },
            gates_passed
        );

        results.push(CycleResult {
            iteration: i,
            rec_count,
            transitions,
            max_drift_x: d.max_drift_x,
            max_drift_y: d.max_drift_y,
            max_drift_z: d.max_drift_z,
            position_matched: matched,
            playback_complete: play_ok,
            all_gates_pass: assessment.all_pass(),
        });
    }

    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    let report = ReliabilityReport {
        iterations,
        speed,
        results,
    };

    report.print_summary();
    report.all_pass()
}
