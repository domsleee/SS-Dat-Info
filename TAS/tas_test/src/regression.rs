//! Regression suite: distinct input contracts, each REC -> PLAY -> drift
//! check -> CSV row.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use serde::Serialize;
use tas_shared::input_bits;

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns::{self, PatternStep};

/// A single regression test case definition.
#[derive(Debug, Clone)]
pub struct RegressionCase {
    pub ordinal: usize,
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
    let hold = patterns::DEFAULT_HOLD_TICKS;
    let gap = patterns::DEFAULT_GAP_TICKS;

    let mut cases = vec![
        case_pattern("right_first", "RLR", hold, gap),
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
        case_pattern("jump_tap", "J", 20, 0),
        case_pattern("jump_hold", "J", 56, 0),
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
    ];

    for (ordinal, case) in cases.iter_mut().enumerate() {
        case.ordinal = ordinal + 1;
    }

    cases
}

fn case_pattern(name: &str, pattern: &str, hold: u32, gap: u32) -> RegressionCase {
    RegressionCase {
        ordinal: 0,
        name: name.to_string(),
        steps: patterns::build_from_pattern(pattern, hold, gap),
        pattern_str: pattern.to_string(),
    }
}

fn case_explicit(name: &str, defs: &[(u8, u32)]) -> RegressionCase {
    RegressionCase {
        ordinal: 0,
        name: name.to_string(),
        steps: patterns::build_from_explicit(defs),
        pattern_str: format!("explicit:{}", name),
    }
}

fn parse_case_filter(raw: Option<&str>) -> Option<HashSet<String>> {
    let raw = raw?;
    let filters: HashSet<String> = raw
        .split(',')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| !part.is_empty())
        .collect();
    if filters.is_empty() {
        None
    } else {
        Some(filters)
    }
}

fn filter_cases(cases: Vec<RegressionCase>) -> Vec<RegressionCase> {
    let requested = std::env::var("TAS_TEST_CASE_FILTER").ok();
    let Some(filters) = parse_case_filter(requested.as_deref()) else {
        return cases;
    };

    cases
        .into_iter()
        .filter(|case| filters.contains(&case.name.to_ascii_lowercase()))
        .collect()
}

/// Run the full regression suite. Drives input via Pico HID for real REC.
pub fn run(csv_path: &Path) -> Vec<CaseResult> {
    let cases = filter_cases(build_cases());
    let mut results = Vec::new();

    if cases.is_empty() {
        eprintln!("ERROR: Regression case filter matched no cases.");
        std::process::exit(1);
    }

    // CSV header
    write_csv_header(csv_path);

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
        append_csv(csv_path, &result);

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

fn write_csv_header(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut f) = fs::File::create(path) {
        let _ = writeln!(
            f,
            "case_name,pattern,alignment_accepted,rec_count,transitions,first_input_tick,replay_drift_x,replay_drift_z,replay_zero,all_gates_pass,error"
        );
    }
}

fn append_csv(path: &Path, r: &CaseResult) {
    if let Ok(mut f) = fs::OpenOptions::new().append(true).open(path) {
        let _ = writeln!(
            f,
            "\"{}\",\"{}\",{},{},{},{},{:.9},{:.9},{},{},\"{}\"",
            r.name,
            r.pattern,
            r.alignment_accepted,
            r.rec_count,
            r.transitions,
            r.first_input_tick,
            r.replay_drift_x,
            r.replay_drift_z,
            r.replay_zero,
            r.all_gates_pass,
            r.error.as_deref().unwrap_or("")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_case_filter_handles_empty_and_spacing() {
        assert_eq!(parse_case_filter(None), None);
        assert_eq!(parse_case_filter(Some(" , ")), None);

        let filters = parse_case_filter(Some(" R , shift_left_right ")).expect("filters");
        assert!(filters.contains("r"));
        assert!(filters.contains("shift_left_right"));
        assert_eq!(filters.len(), 2);
    }

    #[test]
    fn filter_cases_keeps_only_requested_names() {
        let filters = parse_case_filter(Some("jump_tap,modifier_edges")).expect("filters");
        let filtered: Vec<_> = build_cases()
            .into_iter()
            .filter(|case| filters.contains(&case.name.to_ascii_lowercase()))
            .collect();

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "jump_tap");
        assert_eq!(filtered[1].name, "modifier_edges");
    }

    #[test]
    fn build_cases_assigns_stable_ordinals() {
        let cases = build_cases();
        assert_eq!(cases[0].ordinal, 1);
        assert_eq!(cases.len(), 7);
        assert_eq!(cases[6].ordinal, 7);
        assert_eq!(cases[6].name, "shift_left_right");
    }
}
