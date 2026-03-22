//! Three-phase acceptance test — ported from _full_drift_test.lua.
//!
//! Phase 1 (BASELINE): REC with NO input — straight run, captures reference path
//! Phase 2 (RECORD):   REC with Pico steering — alternating L/R via HID
//! Phase 3 (PLAYBACK): PLAY from recorded input — Pico CLOSED before playback
//!
//! Checks:
//!   BASELINE vs RECORD X coords must DIFFER (steering changed path)
//!   RECORD vs PLAYBACK must match (zero drift)

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns;

/// Acceptance test verdicts (matches the acceptance_criteria.md contract).
#[derive(Debug)]
pub struct AcceptanceResult {
    pub steering: Verdict,
    pub replay_steered: Verdict,
    pub zero_drift: Verdict,
    pub baseline_count: u32,
    pub rec_count: u32,
    pub max_drift_x: f64,
    pub max_drift_z: f64,
    pub max_base_vs_rec_x: f64,
    pub max_play_vs_base_x: f64,
}

#[derive(Debug, PartialEq)]
pub enum Verdict {
    Pass,
    Fail,
    Skip,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "PASS"),
            Verdict::Fail => write!(f, "FAIL"),
            Verdict::Skip => write!(f, "SKIP"),
        }
    }
}

impl AcceptanceResult {
    pub fn all_pass(&self) -> bool {
        self.steering == Verdict::Pass
            && self.replay_steered == Verdict::Pass
            && self.zero_drift == Verdict::Pass
    }
}

/// Duration for each test phase (seconds of recording).
const REC_DURATION_SECS: u64 = 5;

/// Steering pattern for Phase 2 — alternating L/R for variety.
const STEER_PATTERN: &str = "LRLRL";
const STEER_HOLD_TICKS: u32 = 56;

/// Run the full 3-phase acceptance test.
pub fn run(mock: bool) -> AcceptanceResult {
    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Game not alive. Cannot run acceptance test.");
        std::process::exit(1);
    }

    // Config preconditions: assert proven zero-drift config before running.
    {
        let s = client.state();
        assert_eq!(s.force_fixed_tick, 0, "fft must be 0 (natural ticks)");
        assert_eq!(s.inject_mode, 6, "inject_mode must be 6");
        assert_eq!(s.force_direct, 2, "force_direct must be 2");
        assert!(
            s.playback_speed == 1.0 || s.playback_speed == 0.0,
            "playback_speed must be 1.0 or 0.0 (got {})",
            s.playback_speed
        );
        println!("Config OK: fft=0, inject_mode=6, force_direct=2, speed={}", s.playback_speed);
    }

    println!("\n=== Three-Phase Acceptance Test ===\n");

    // ---- Phase 1: BASELINE (no input) ----
    println!("--- Phase 1: BASELINE (no steering) ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive for Phase 1");
        std::process::exit(1);
    }

    harness::arm_rec(&mut client);
    println!("  Recording baseline for {}s...", REC_DURATION_SECS);
    std::thread::sleep(std::time::Duration::from_secs(REC_DURATION_SECS));

    let baseline_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Baseline: {} ticks", baseline_count);

    if baseline_count == 0 {
        eprintln!("ERROR: No baseline ticks recorded");
        std::process::exit(1);
    }

    // Capture baseline coords
    let n = baseline_count as usize;
    let baseline_coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();

    // ---- Phase 2: RECORD (with steering) ----
    println!("\n--- Phase 2: RECORD (with Pico steering) ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive for Phase 2");
        std::process::exit(1);
    }

    harness::arm_rec(&mut client);

    if mock {
        // Mock: write input pattern directly
        let steps = patterns::build_from_pattern(STEER_PATTERN, STEER_HOLD_TICKS, 0);
        let input_log = patterns::generate_input_log(&steps);
        harness::write_mock_input(&mut client, &input_log);
        println!("  Mock: wrote {} ticks of steering input", input_log.len());
        // Wait for game to process those ticks
        std::thread::sleep(std::time::Duration::from_secs(REC_DURATION_SECS));
    } else {
        // Real Pico HID steering
        println!("  Steering via Pico HID for {}s...", REC_DURATION_SECS);
        let steps = patterns::build_from_pattern(STEER_PATTERN, STEER_HOLD_TICKS, 0);
        drive_pico_acceptance(&steps);
    }

    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Recorded: {} ticks", rec_count);

    if rec_count == 0 {
        eprintln!("ERROR: No steered ticks recorded");
        std::process::exit(1);
    }

    // Capture steered coords
    let n_rec = rec_count as usize;
    let rec_coords: Vec<[f32; 3]> = client.state().rec_coords[..n_rec].to_vec();

    // Capture REC start position for Phase 3 matching
    let rec_start = client.state().rec_coords[0];
    println!("  REC start: ({:.4}, {:.4}, {:.4})", rec_start[0], rec_start[1], rec_start[2]);

    // ---- Phase 3: PLAYBACK ----
    println!("\n--- Phase 3: PLAYBACK ---");
    // Match Phase 2 start position via play_coords[0] for zero drift
    if !harness::restart_play_and_match(&mut client, rec_start, 20) {
        eprintln!("WARNING: Could not match REC position for Phase 3 (continuing anyway)");
    }
    // Playback is already running from restart_play_and_match
    let play_ok = harness::wait_playback(&client, rec_count);

    if !play_ok {
        eprintln!("WARNING: Playback did not complete normally");
    }

    // ---- Compute verdicts ----
    let state = client.state();
    harness::print_results(&client);

    // Steering verdict: BASELINE X coords vs RECORD X coords must differ
    let compare_count = baseline_count.min(rec_count) as usize;
    let mut max_base_vs_rec_x: f64 = 0.0;
    for i in 0..compare_count {
        let dx = (baseline_coords[i][0] as f64 - rec_coords[i][0] as f64).abs();
        if dx > max_base_vs_rec_x {
            max_base_vs_rec_x = dx;
        }
    }
    let steering = if max_base_vs_rec_x > 1.0 {
        Verdict::Pass
    } else {
        Verdict::Fail
    };

    // Replay steered verdict: PLAY X coords vs BASELINE X coords must differ
    let mut max_play_vs_base_x: f64 = 0.0;
    let play_compare = baseline_count.min(state.playback_pos) as usize;
    for i in 0..play_compare {
        let dx = (state.play_coords[i][0] as f64 - baseline_coords[i][0] as f64).abs();
        if dx > max_play_vs_base_x {
            max_play_vs_base_x = dx;
        }
    }
    let replay_steered = if max_play_vs_base_x > 1.0 {
        Verdict::Pass
    } else {
        Verdict::Fail
    };

    // Zero drift: REC vs PLAY coords
    let drift_result = drift::compute_drift(state, rec_count);
    let zero_drift = if drift_result.is_zero() {
        Verdict::Pass
    } else {
        Verdict::Fail
    };

    // 4-gate assessment
    let assessment = gates::run_gates(state, rec_count);
    assessment.print_summary();

    let result = AcceptanceResult {
        steering,
        replay_steered,
        zero_drift,
        baseline_count,
        rec_count,
        max_drift_x: drift_result.max_drift_x,
        max_drift_z: drift_result.max_drift_z,
        max_base_vs_rec_x,
        max_play_vs_base_x,
    };

    println!("\n=== ACCEPTANCE VERDICT ===");
    println!(
        "VERDICT: steering={} replay_steered={} zero_drift={}",
        result.steering, result.replay_steered, result.zero_drift
    );
    println!(
        "  base_vs_rec_X={:.4}  play_vs_base_X={:.4}",
        result.max_base_vs_rec_x, result.max_play_vs_base_x
    );
    println!(
        "  max_drift_X={:.9}  max_drift_Z={:.9}",
        result.max_drift_x, result.max_drift_z
    );

    if result.all_pass() {
        println!("\n*** ACCEPTANCE TEST PASSED ***");
    } else {
        println!("\n*** ACCEPTANCE TEST FAILED ***");
    }

    result
}

/// Drive Pico HID for acceptance test Phase 2.
fn drive_pico_acceptance(steps: &[patterns::PatternStep]) {
    use std::io::Write;

    let port = match std::fs::OpenOptions::new()
        .write(true)
        .open("\\\\.\\COM7")
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("  ERROR: Cannot open COM7: {}. Steering will be absent.", e);
            std::thread::sleep(std::time::Duration::from_secs(REC_DURATION_SECS));
            return;
        }
    };

    let mut port = port;
    let total = patterns::total_ticks(steps);
    let ms_per_tick = 10u64;
    let start = std::time::Instant::now();
    let mut prev_mask = 0xFFu8;
    let mut current_step = 0usize;

    for tick in 0..total {
        while current_step < steps.len() && tick >= steps[current_step].stop_tick {
            current_step += 1;
        }
        let mask = if current_step < steps.len() {
            steps[current_step].mask
        } else {
            0
        };

        if mask != prev_mask {
            let send_byte = if mask == 0 { 0xFF } else { mask };
            let _ = port.write_all(&[send_byte]);
            let _ = port.flush();
            prev_mask = mask;
        }

        let target = std::time::Duration::from_millis((tick as u64 + 1) * ms_per_tick);
        if let Some(remaining) = target.checked_sub(start.elapsed()) {
            std::thread::sleep(remaining);
        }
    }

    // Release all
    let _ = port.write_all(&[0xFF]);
    let _ = port.flush();
}
