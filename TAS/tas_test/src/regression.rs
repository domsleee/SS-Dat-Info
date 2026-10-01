//! Regression suite: distinct input contracts, each REC -> PLAY -> drift
//! check -> certificate row.

use serde::Serialize;
use tas_shared::input_bits;

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns::{self, PatternStep};

/// A single regression test case definition.
#[derive(Debug, Clone)]
pub struct RegressionCase {
    pub name: String,
    pub steps: Vec<PatternStep>,
    pub pattern_str: String,
}

/// Result of running one regression case; also the certificate's row.
#[derive(Debug, Serialize)]
pub struct CaseResult {
    pub name: String,
    pub pattern: String,
    pub alignment_accepted: bool,
    pub rec_count: u32,
    pub transitions: u32,
    pub first_input_tick: i32,
    pub replay_drift_x: f64,
    pub replay_drift_z: f64,
    pub replay_zero: bool,
    pub all_gates_pass: bool,
    pub error: Option<String>,
}

/// Cover direction, duration, release/repress, jump and modifier behavior without
/// repeating prefixes of the same alternating sequence.
pub fn build_cases() -> Vec<RegressionCase> {
    vec![
        case_pattern("right_first", "RLR", patterns::DEFAULT_HOLD_TICKS),
        case_explicit(
            "release_repress",
            &[(input_bits::LEFT, 36), (0, 20), (input_bits::LEFT, 36)],
        ),
        case_explicit(
            "L_long_R_short_L",
            &[
                (input_bits::LEFT, 72),
                (input_bits::RIGHT, 36),
                (input_bits::LEFT, 72),
            ],
        ),
        case_pattern("jump_tap", "J", 20),
        case_pattern("jump_hold", "J", 56),
        case_explicit(
            "modifier_edges",
            &[
                (input_bits::LEFT, 36),
                (input_bits::LEFT | input_bits::SHIFT, 36),
                (input_bits::SHIFT, 36),
                (0, 20),
            ],
        ),
        case_explicit(
            "shift_left_right",
            &[
                (input_bits::SHIFT | input_bits::LEFT, 56),
                (0, 20),
                (input_bits::SHIFT | input_bits::RIGHT, 56),
            ],
        ),
    ]
}

fn case_pattern(name: &str, pattern: &str, hold: u32) -> RegressionCase {
    RegressionCase {
        name: name.to_string(),
        steps: patterns::build_from_pattern(pattern, hold),
        pattern_str: pattern.to_string(),
    }
}

fn case_explicit(name: &str, defs: &[(u8, u32)]) -> RegressionCase {
    RegressionCase {
        name: name.to_string(),
        steps: patterns::build_from_explicit(defs),
        pattern_str: format!("explicit:{}", name),
    }
}

/// Run the full regression suite. Drives input via Pico HID for real REC.
pub fn run() -> Vec<CaseResult> {
    let cases = build_cases();
    let mut results = Vec::new();

    let mut client = harness::ensure_game_running();
    harness::ensure_exclusive_runtime_ownership(&mut client, "regression determinism failures");
    harness::print_status(&client);

    harness::assert_proven_config(&client);

    println!("\n=== Regression Suite: {} cases ===\n", cases.len());

    for (i, case) in cases.iter().enumerate() {
        println!(
            "--- Case {}/{}: {} (pattern: {}) ---",
            i + 1,
            cases.len(),
            case.name,
            case.pattern_str
        );

        let result = run_single_case(&mut client, case);
        if let Some(error) = &result.error {
            eprintln!("FAIL {}: {error}", case.name);
        }

        println!(
            "  Result: alignmentAccepted={} gateRelativeDrift=({:.9}, {:.9}) replayZero={} gates={}",
            result.alignment_accepted,
            result.replay_drift_x,
            result.replay_drift_z,
            result.replay_zero,
            if result.all_gates_pass {
                "PASS"
            } else {
                "FAIL"
            }
        );

        results.push(result);
    }

    // Summary
    let passed = results.iter().filter(|r| r.all_gates_pass).count();
    println!(
        "\n=== Regression Summary: {}/{} passed ===",
        passed,
        results.len()
    );
    for r in &results {
        println!(
            "  {} {} — gate-relative({:.9}, {:.9}) alignmentAccepted={}",
            if r.all_gates_pass { "PASS" } else { "FAIL" },
            r.name,
            r.replay_drift_x,
            r.replay_drift_z,
            r.alignment_accepted,
        );
    }

    results
}

fn run_single_case(
    client: &mut tas_shared::TasSharedMemoryClient,
    case: &RegressionCase,
) -> CaseResult {
    // Phase 1: Record
    if !harness::restart_and_stabilize_inprocess(client) {
        return error_result(case, "Game not alive after restart (REC phase)");
    }

    harness::arm_rec(client);
    // Include the stationary countdown so the product's gate-aligned replay
    // can reconstruct the run. Drive short patterns only after physics starts.
    std::thread::sleep(std::time::Duration::from_millis(3500));
    if let Err(error) = harness::drive_pico_steps(&case.steps) {
        harness::stop(client);
        return error_result(case, &error);
    }

    // Capture the final hardware release before stopping the recorder.
    std::thread::sleep(std::time::Duration::from_millis(200));
    let rec_count = client.state().recorded_count;
    harness::stop(client);
    println!("  Recorded {} ticks", rec_count);

    if rec_count == 0 {
        return error_result(case, "No ticks recorded");
    }
    if let Err(error) = patterns::verify_capture(
        &case.steps,
        &client.state().input_log[..rec_count.min(tas_shared::TAS_MAX_TICKS as u32) as usize],
        12,
    ) {
        return error_result(case, &error);
    }

    let live_transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);
    println!("  Live transitions: {}", live_transitions);

    // Phase 2: Playback
    let (rec_gate, play_gate) = match harness::restart_play_aligned_inprocess(client) {
        Some(gates) => gates,
        None => return error_result(case, "Product PLAY alignment failed"),
    };
    let expected_end = play_gate.saturating_add(rec_count.saturating_sub(rec_gate));
    let play_ok = harness::wait_playback(client, expected_end);

    if !play_ok {
        return error_result(case, "Playback timeout");
    }

    // Assess
    let assessment = gates::run_gates(client.state(), rec_count, rec_gate, play_gate);
    assessment.print_summary();
    let n = rec_count as usize;

    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        alignment_accepted: true,
        rec_count,
        transitions: drift::count_transitions(&client.state().input_log, n),
        first_input_tick: drift::first_input_tick(&client.state().input_log, n),
        replay_drift_x: assessment.drift.max_drift_x,
        replay_drift_z: assessment.drift.max_drift_z,
        replay_zero: assessment.drift.is_zero(),
        all_gates_pass: assessment.all_pass(),
        error: None,
    }
}

fn error_result(case: &RegressionCase, msg: &str) -> CaseResult {
    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        alignment_accepted: false,
        rec_count: 0,
        transitions: 0,
        first_input_tick: -1,
        replay_drift_x: 999.0,
        replay_drift_z: 999.0,
        replay_zero: false,
        all_gates_pass: false,
        error: Some(msg.to_string()),
    }
}
