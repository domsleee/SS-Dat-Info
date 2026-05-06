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
pub fn run() -> AcceptanceResult {
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

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
        println!(
            "Config OK: fft=0, inject_mode=6, force_direct=2, speed={}",
            s.playback_speed
        );
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

    println!("  Steering via Pico HID for {}s...", REC_DURATION_SECS);
    let steps = patterns::build_from_pattern(STEER_PATTERN, STEER_HOLD_TICKS, 0);
    drive_pico_acceptance(&steps);

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
    println!(
        "  REC start: ({:.4}, {:.4}, {:.4})",
        rec_start[0], rec_start[1], rec_start[2]
    );

    // ---- Phase 3: PLAYBACK ----
    println!("\n--- Phase 3: PLAYBACK ---");
    // Match Phase 2 start position via play_coords[0] for zero drift
    if !harness::restart_play_and_match(&mut client, rec_start, 60) {
        eprintln!("ERROR: Could not match REC position for Phase 3 after 60 F5 retries");
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
    for (play, base) in state.play_coords[..play_compare]
        .iter()
        .zip(&baseline_coords[..play_compare])
    {
        let dx = (play[0] as f64 - base[0] as f64).abs();
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

/// Drive Pico HID for acceptance test Phase 2 (delegates to harness).
fn drive_pico_acceptance(steps: &[patterns::PatternStep]) {
    harness::drive_pico_steps(steps, Some(REC_DURATION_SECS * 1000));
}
