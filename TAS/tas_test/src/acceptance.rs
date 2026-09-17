//! Three-phase acceptance test.
//!
//! Phase 1 (BASELINE): REC with NO input — straight run, captures reference path
//! Phase 2 (RECORD):   REC with Pico steering
//! Phase 3 (PLAYBACK): PLAY from recorded input — Pico CLOSED before playback
//!
//! Checks:
//!   BASELINE vs RECORD coords must DIFFER (steering changed path)
//!   RECORD vs PLAYBACK must match (zero drift)

use serde::Serialize;

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns;

/// Acceptance verdicts; also the certificate body.
#[derive(Debug, Serialize)]
pub struct AcceptanceResult {
    pub baseline_neutral: bool,
    pub steering: Verdict,
    pub replay_steered: Verdict,
    pub zero_drift: Verdict,
    pub baseline_count: u32,
    pub rec_count: u32,
    pub max_drift_x: f64,
    pub max_drift_z: f64,
    pub max_base_vs_rec_x: f64,
    pub max_play_vs_base_x: f64,
    pub gates_pass: bool,
    /// Whether PLAY ran through to `rec_count`. A truncated playback makes the
    /// drift and gate numbers meaningless, so it is part of the verdict.
    pub playback_complete: bool,
    pub rec_gate: u32,
    pub play_gate: u32,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail,
}

impl Verdict {
    fn from_bool(pass: bool) -> Verdict {
        if pass {
            Verdict::Pass
        } else {
            Verdict::Fail
        }
    }
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "PASS"),
            Verdict::Fail => write!(f, "FAIL"),
        }
    }
}

impl AcceptanceResult {
    pub fn all_pass(&self) -> bool {
        self.baseline_neutral
            && self.steering == Verdict::Pass
            && self.replay_steered == Verdict::Pass
            && self.gates_pass
            && self.zero_drift == Verdict::Pass
            && self.playback_complete
    }
}

/// Duration for each test phase (seconds of recording).
const REC_DURATION_SECS: u64 = 5;

/// Steering pattern for Phase 2.
///
/// RIGHT is the calibrated high-impact direction on Forest Easy (the dedicated
/// steer-impact gate measures roughly ten lateral units over two moving
/// seconds). The acceptance driver refreshes it before the Pico's 500ms
/// watchdog, then records a neutral tail so the asynchronous HID key-up is
/// captured before REC stops.
const STEER_PATTERN: &str = "R";
const STEER_HOLD_TICKS: u32 = 450;
const STEER_RELEASE_TICKS: u32 = 50;

/// Run the full 3-phase acceptance test.
pub fn run() -> AcceptanceResult {
    // Fail before launching or changing the game if another controller owns
    // the Pico, or the configured device is missing.
    if let Err(error) = harness::PicoKeys::open_checked() {
        eprintln!("ACCEPTANCE SETUP FAILED: {error}");
        std::process::exit(1);
    }
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::assert_proven_config(&client);

    println!("\n=== Three-Phase Acceptance Test ===\n");

    // ---- Phase 1: BASELINE (no input) ----
    println!("--- Phase 1: BASELINE (no steering) ---");
    if !harness::restart_and_stabilize_inprocess(&mut client) {
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

    // Phase 1 should be input-free — if anything appears in input_log, the Pico
    // is stuck in a non-neutral state and the steering verdict will spuriously
    // pass (both phases see the same input).
    let baseline_neutral = {
        let s = client.state();
        let transitions = crate::drift::count_transitions(&s.input_log, n);
        let first_in = crate::drift::first_input_tick(&s.input_log, n);
        println!(
            "  BASELINE diagnostics: transitions={} firstInput={}",
            transitions, first_in
        );
        transitions == 0 && first_in < 0
    };

    // ---- Phase 2: RECORD (with steering) ----
    println!("\n--- Phase 2: RECORD (with Pico steering) ---");
    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: Game not alive for Phase 2");
        std::process::exit(1);
    }

    harness::arm_rec(&mut client);

    println!("  Steering via Pico HID for {}s...", REC_DURATION_SECS);
    let steps = patterns::build_from_pattern(STEER_PATTERN, STEER_HOLD_TICKS, 0);
    if let Err(error) = drive_pico_acceptance(&steps) {
        harness::stop(&mut client);
        eprintln!("ACCEPTANCE INPUT FAILED: {error}");
        std::process::exit(1);
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

    let rec_start = client.state().rec_coords[0];
    println!(
        "  REC start: ({:.4}, {:.4}, {:.4})",
        rec_start[0], rec_start[1], rec_start[2]
    );

    // ---- Phase 3: PLAYBACK ----
    println!("\n--- Phase 3: PLAYBACK ---");
    let alignment = harness::restart_play_aligned_inprocess(&mut client);
    let (rec_gate, play_gate) = alignment.unwrap_or((0, 0));
    let expected_end = play_gate.saturating_add(rec_count.saturating_sub(rec_gate));
    let play_ok = alignment.is_some() && harness::wait_playback(&client, expected_end);

    if !play_ok {
        eprintln!("WARNING: Playback did not complete normally");
    }

    // ---- Compute verdicts ----
    let state = client.state();
    harness::print_results(&client);

    // Steering verdict: BASELINE vs RECORD must differ by more than 1.0 in 3D
    // distance. At the F5 spawn the track is steeply downhill, so L/R steering
    // shows up in Y/Z before it shows in X.
    let compare_count = baseline_count.min(rec_count) as usize;
    let max_base_vs_rec_x = max_distance(&baseline_coords[..compare_count], &rec_coords);
    let steering = Verdict::from_bool(max_base_vs_rec_x > 1.0);

    // Replay steered verdict: PLAY vs BASELINE must differ the same way.
    let play_compare = baseline_count.min(state.playback_pos) as usize;
    let max_play_vs_base_x = max_distance(&baseline_coords[..play_compare], &state.play_coords);
    let replay_steered = Verdict::from_bool(max_play_vs_base_x > 1.0);

    // Zero drift: equal offsets from the independently observed gates.
    let compared = state
        .playback_pos
        .saturating_sub(play_gate)
        .min(rec_count.saturating_sub(rec_gate));
    let drift_result = drift::compute_gate_relative_drift(state, rec_gate, play_gate, compared);
    let first_bit_mismatch = (0..compared).find(|offset| {
        let rec = state.rec_coords[(rec_gate + offset) as usize];
        let play = state.play_coords[(play_gate + offset) as usize];
        rec.iter()
            .zip(play.iter())
            .any(|(r, p)| r.to_bits() != p.to_bits())
    });
    let zero_drift = Verdict::from_bool(drift_result.is_zero());

    // 4-gate assessment
    let assessment = gates::run_gates_aligned(state, rec_count, rec_gate, play_gate);
    assessment.print_summary();

    let result = AcceptanceResult {
        baseline_neutral,
        steering,
        replay_steered,
        zero_drift,
        baseline_count,
        rec_count,
        max_drift_x: drift_result.max_drift_x,
        max_drift_z: drift_result.max_drift_z,
        max_base_vs_rec_x,
        max_play_vs_base_x,
        gates_pass: assessment.all_pass(),
        playback_complete: play_ok,
        rec_gate,
        play_gate,
    };

    println!("\n=== ACCEPTANCE VERDICT ===");
    println!(
        "VERDICT: baseline_neutral={} steering={} replay_steered={} zero_drift={}",
        result.baseline_neutral, result.steering, result.replay_steered, result.zero_drift
    );
    println!(
        "  base_vs_rec_X={:.4}  play_vs_base_X={:.4}",
        result.max_base_vs_rec_x, result.max_play_vs_base_x
    );
    println!(
        "  max_drift_X={:.9}  max_drift_Z={:.9}",
        result.max_drift_x, result.max_drift_z
    );
    println!(
        "  first gate-relative bit mismatch={:?}",
        first_bit_mismatch
    );

    if result.all_pass() {
        println!("\n*** ACCEPTANCE TEST PASSED ***");
    } else {
        println!("\n*** ACCEPTANCE TEST FAILED ***");

        println!("\n--- Failure control: legacy matched-bucket PLAY ---");
        if harness::restart_play_and_match_inprocess(&mut client, rec_start, 12) {
            let control_complete = harness::wait_playback(&client, rec_count);
            let control = drift::compute_drift(client.state(), rec_count);
            println!(
                "  control complete={} max drift X/Y/Z={:.9}/{:.9}/{:.9}",
                control_complete, control.max_drift_x, control.max_drift_y, control.max_drift_z
            );
        } else {
            println!("  legacy control could not find an acceptable bucket");
        }
    }

    result
}

/// Largest 3D distance between corresponding samples of two traces.
fn max_distance(a: &[[f32; 3]], b: &[[f32; 3]]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(p, q)| {
            (0..3)
                .map(|axis| (p[axis] as f64 - q[axis] as f64).powi(2))
                .sum::<f64>()
                .sqrt()
        })
        .fold(0.0, f64::max)
}

/// Drive Pico HID for acceptance test Phase 2, then wait for the release to
/// be recorded.
fn drive_pico_acceptance(steps: &[patterns::PatternStep]) -> Result<(), String> {
    harness::drive_pico_steps(steps)?;
    std::thread::sleep(std::time::Duration::from_millis(
        STEER_RELEASE_TICKS as u64 * 10,
    ));
    Ok(())
}
