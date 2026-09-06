//! Regression suite — 15 test cases ported from _drift_regression_suite.lua.
//!
//! Each case: scripted steering pattern -> REC -> PLAY -> drift check -> CSV output.

use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use tas_shared::input_bits;

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns::{self, PatternStep};

// Empirically, 20 retries was not enough to reliably hit the replayable F5 bucket
// for short patterns like jump_tap. 60 cleared repeated live stress reruns.
const START_MATCH_RETRIES: u32 = 60;

/// A single regression test case definition.
#[derive(Debug, Clone)]
pub struct RegressionCase {
    pub ordinal: usize,
    pub name: String,
    pub steps: Vec<PatternStep>,
    pub pattern_str: String,
}

#[derive(Debug, Clone, Default)]
pub struct WindowMetrics {
    pub transitions: u32,
    pub first_input_tick: i32,
    pub frame0_dx: f64,
    pub frame0_dz: f64,
    pub full_norm_drift_x: f64,
    pub full_norm_drift_z: f64,
    pub active_start_dx: f64,
    pub active_start_dz: f64,
    pub active_window_ticks: u32,
    pub active_norm_drift_x: f64,
    pub active_norm_drift_z: f64,
}

/// Result of running one regression case.
#[derive(Debug)]
pub struct CaseResult {
    pub name: String,
    pub pattern: String,
    pub start_matched: bool,
    pub rec_count: u32,
    pub transitions: u32,
    pub first_input_tick: i32,
    pub frame0_dx: f64,
    pub frame0_dz: f64,
    pub full_norm_drift_x: f64,
    pub full_norm_drift_z: f64,
    pub active_start_dx: f64,
    pub active_start_dz: f64,
    pub active_window_ticks: u32,
    pub active_norm_drift_x: f64,
    pub active_norm_drift_z: f64,
    pub live_drift_x: f64,
    pub live_drift_z: f64,
    pub live_zero: bool,
    pub replay_drift_x: f64,
    pub replay_drift_z: f64,
    pub replay_zero: bool,
    pub all_gates_pass: bool,
    pub error: Option<String>,
}

/// Build the 15 regression test cases.
pub fn build_cases() -> Vec<RegressionCase> {
    let hold = patterns::DEFAULT_HOLD_TICKS;
    let gap = patterns::DEFAULT_GAP_TICKS;

    let mut cases = vec![
        // 1-4: Single directions and simple alternations
        case_pattern("L", "L", hold, gap),
        case_pattern("R", "R", hold, gap),
        case_pattern("LR", "LR", hold, gap),
        case_pattern("RL", "RL", hold, gap),
        // 5-8: Longer alternation patterns
        case_pattern("LRL", "LRL", hold, gap),
        case_pattern("RLR", "RLR", hold, gap),
        case_pattern("LRLRL", "LRLRL", hold, gap),
        case_pattern("RLRLR", "RLRLR", hold, gap),
        // 9: Short hold variant
        case_pattern("LRLRL_short", "LRLRL", 36, gap),
        // 10: Asymmetric hold durations
        case_explicit(
            "L_long_R_short_L",
            &[
                ("LEFT1", input_bits::LEFT, 72),
                ("RIGHT2", input_bits::RIGHT, 36),
                ("LEFT3", input_bits::LEFT, 72),
            ],
        ),
        // 11-12: Jump variants
        case_pattern("jump_tap", "J", 20, 0),
        case_pattern("jump_hold", "J", 56, 0),
        // 13-14: Shift variants
        case_pattern("shift_left", "SL", hold, 0),
        case_pattern("shift_right", "SR", hold, 0),
        // 15: Combined shift + left + right
        case_explicit(
            "shift_left_right",
            &[
                ("SHIFT_LEFT", input_bits::SHIFT | input_bits::LEFT, 56),
                ("NEUTRAL", 0, 20),
                ("SHIFT_RIGHT", input_bits::SHIFT | input_bits::RIGHT, 56),
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

fn case_explicit(name: &str, defs: &[(&str, u8, u32)]) -> RegressionCase {
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
pub fn run(cache_dir: &Path, csv_path: &Path) -> Vec<CaseResult> {
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

        let result = run_single_case(&mut client, case, cache_dir);
        append_csv(csv_path, &result);

        println!(
            "  Result: startMatched={} rawDrift=({:.9}, {:.9}) fullNorm=({:.9}, {:.9}) rawZero={} gates={}",
            result.start_matched,
            result.replay_drift_x,
            result.replay_drift_z,
            result.full_norm_drift_x,
            result.full_norm_drift_z,
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
            "  {} {} — raw({:.9}, {:.9}) fullNorm({:.9}, {:.9}) startMatched={}",
            if r.all_gates_pass { "PASS" } else { "FAIL" },
            r.name,
            r.replay_drift_x,
            r.replay_drift_z,
            r.full_norm_drift_x,
            r.full_norm_drift_z,
            r.start_matched,
        );
    }

    results
}

fn run_single_case(
    client: &mut tas_shared::TasSharedMemoryClient,
    case: &RegressionCase,
    cache_dir: &Path,
) -> CaseResult {
    let cache_path = cache_dir.join(format!("{:02}_{}.tas", case.ordinal, slug(&case.name)));

    // Phase 1: Record
    if !harness::restart_and_stabilize(client) {
        return error_result(case, "Game not alive after restart (REC phase)");
    }

    harness::arm_rec(client);

    // Drive the Pico HID according to the pattern schedule
    drive_pico_pattern(&case.steps);

    let rec_count = client.state().recorded_count;
    harness::stop(client);
    println!("  Recorded {} ticks", rec_count);

    if rec_count == 0 {
        return error_result(case, "No ticks recorded");
    }

    // Compute live drift (REC vs itself is always zero, but we track the recording quality)
    let live_transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);
    println!("  Live transitions: {}", live_transitions);

    // Phase 2: Playback
    let rec_start = client.state().rec_coords[0];
    let start_matched = match start_playback_with_fallback(client, rec_start, "playback") {
        Ok(matched) => matched,
        Err(err) => return error_result(case, &err),
    };
    let play_ok = harness::wait_playback(client, rec_count);

    if !play_ok {
        return error_result(case, "Playback timeout");
    }

    // Assess
    let assessment = gates::run_gates(client.state(), rec_count);
    assessment.print_summary();
    let metrics = collect_window_metrics(client.state(), rec_count);
    print_window_metrics(&metrics);
    print_translation_verdict(start_matched, &metrics);
    let start_match_error =
        (!start_matched).then(|| "Could not position-match playback start".to_string());

    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        start_matched,
        rec_count,
        transitions: metrics.transitions,
        first_input_tick: metrics.first_input_tick,
        frame0_dx: metrics.frame0_dx,
        frame0_dz: metrics.frame0_dz,
        full_norm_drift_x: metrics.full_norm_drift_x,
        full_norm_drift_z: metrics.full_norm_drift_z,
        active_start_dx: metrics.active_start_dx,
        active_start_dz: metrics.active_start_dz,
        active_window_ticks: metrics.active_window_ticks,
        active_norm_drift_x: metrics.active_norm_drift_x,
        active_norm_drift_z: metrics.active_norm_drift_z,
        live_drift_x: 0.0, // REC vs itself
        live_drift_z: 0.0,
        live_zero: true,
        replay_drift_x: assessment.drift.max_drift_x,
        replay_drift_z: assessment.drift.max_drift_z,
        replay_zero: assessment.drift.is_zero(),
        all_gates_pass: start_matched && assessment.all_pass(),
        error: start_match_error,
    }
}

fn start_playback_with_fallback(
    client: &mut tas_shared::TasSharedMemoryClient,
    target: [f32; 3],
    label: &str,
) -> Result<bool, String> {
    if harness::restart_play_and_match(client, target, START_MATCH_RETRIES) {
        return Ok(true);
    }

    println!(
        "  WARNING: Could not exact-match {} start after retries; retrying once without exact matching",
        label
    );
    if !harness::restart_and_stabilize(client) {
        return Err(format!(
            "Game not alive after restart ({} fallback phase)",
            label
        ));
    }
    harness::arm_play(client);
    Ok(false)
}

/// Drive Pico HID according to the pattern step schedule (delegates to harness).
fn drive_pico_pattern(steps: &[PatternStep]) {
    harness::drive_pico_steps(steps, None);
}

fn error_result(case: &RegressionCase, msg: &str) -> CaseResult {
    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        start_matched: false,
        rec_count: 0,
        transitions: 0,
        first_input_tick: -1,
        frame0_dx: 999.0,
        frame0_dz: 999.0,
        full_norm_drift_x: 999.0,
        full_norm_drift_z: 999.0,
        active_start_dx: 999.0,
        active_start_dz: 999.0,
        active_window_ticks: 0,
        active_norm_drift_x: 999.0,
        active_norm_drift_z: 999.0,
        live_drift_x: 999.0,
        live_drift_z: 999.0,
        live_zero: false,
        replay_drift_x: 999.0,
        replay_drift_z: 999.0,
        replay_zero: false,
        all_gates_pass: false,
        error: Some(msg.to_string()),
    }
}

fn slug(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn write_csv_header(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(mut f) = fs::File::create(path) {
        let _ = writeln!(
            f,
            "case_name,pattern,start_matched,rec_count,transitions,first_input_tick,frame0_dx,frame0_dz,full_norm_drift_x,full_norm_drift_z,active_start_dx,active_start_dz,active_window_ticks,active_norm_drift_x,active_norm_drift_z,live_drift_x,live_drift_z,live_zero,replay_drift_x,replay_drift_z,replay_zero,all_gates_pass,error"
        );
    }
}

fn append_csv(path: &Path, r: &CaseResult) {
    if let Ok(mut f) = fs::OpenOptions::new().append(true).open(path) {
        let _ = writeln!(
            f,
            "\"{}\",\"{}\",{},{},{},{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{},{:.9},{:.9},{:.9},{:.9},{},{:.9},{:.9},{},{},\"{}\"",
            r.name,
            r.pattern,
            r.start_matched,
            r.rec_count,
            r.transitions,
            r.first_input_tick,
            r.frame0_dx,
            r.frame0_dz,
            r.full_norm_drift_x,
            r.full_norm_drift_z,
            r.active_start_dx,
            r.active_start_dz,
            r.active_window_ticks,
            r.active_norm_drift_x,
            r.active_norm_drift_z,
            r.live_drift_x,
            r.live_drift_z,
            r.live_zero,
            r.replay_drift_x,
            r.replay_drift_z,
            r.replay_zero,
            r.all_gates_pass,
            r.error.as_deref().unwrap_or("")
        );
    }
}

fn collect_window_metrics(state: &tas_shared::TasSharedState, count: u32) -> WindowMetrics {
    let n = count as usize;
    let transitions = drift::count_transitions(&state.input_log, n);
    let first_input_tick = drift::first_input_tick(&state.input_log, n);
    let (frame0_dx, frame0_dz) = coord_offset(state, 0, n > 0);
    let full_norm = drift::compute_normalized_drift_window(state, 0, count);

    let mut metrics = WindowMetrics {
        transitions,
        first_input_tick,
        frame0_dx,
        frame0_dz,
        full_norm_drift_x: full_norm.max_drift_x,
        full_norm_drift_z: full_norm.max_drift_z,
        ..WindowMetrics::default()
    };

    if first_input_tick >= 0 {
        let idx = first_input_tick as usize;
        let (active_start_dx, active_start_dz) = coord_offset(state, idx, idx < n);
        let active_norm = drift::compute_normalized_drift_window(state, idx as u32, count);
        metrics.active_start_dx = active_start_dx;
        metrics.active_start_dz = active_start_dz;
        metrics.active_window_ticks = count.saturating_sub(idx as u32);
        metrics.active_norm_drift_x = active_norm.max_drift_x;
        metrics.active_norm_drift_z = active_norm.max_drift_z;
    }

    metrics
}

fn coord_offset(state: &tas_shared::TasSharedState, idx: usize, present: bool) -> (f64, f64) {
    if !present {
        return (0.0, 0.0);
    }
    let dx = (state.rec_coords[idx][0] as f64 - state.play_coords[idx][0] as f64).abs();
    let dz = (state.rec_coords[idx][2] as f64 - state.play_coords[idx][2] as f64).abs();
    (dx, dz)
}

fn print_window_metrics(metrics: &WindowMetrics) {
    println!(
        "  Window metrics: transitions={} firstInput={} frame0Offset=({:.6}, {:.6}) fullNormDrift=({:.9}, {:.9}) activeStartOffset=({:.6}, {:.6}) activeTicks={} activeNormDrift=({:.9}, {:.9})",
        metrics.transitions,
        metrics.first_input_tick,
        metrics.frame0_dx,
        metrics.frame0_dz,
        metrics.full_norm_drift_x,
        metrics.full_norm_drift_z,
        metrics.active_start_dx,
        metrics.active_start_dz,
        metrics.active_window_ticks,
        metrics.active_norm_drift_x,
        metrics.active_norm_drift_z,
    );
}

fn print_translation_verdict(start_matched: bool, metrics: &WindowMetrics) {
    // NOTE: X/Z only — this is a translation *diagnostic*, and WindowMetrics
    // feeds the CSV artifact schema, so it is deliberately not widened to Y.
    // The authoritative zero-drift verdict (Gate 3 / `replay_zero`) does check
    // all three axes; the name says XZ so this line cannot be misread as one.
    println!(
        "  Translation diagnostic: startMatched={} fullNormZeroXZ={} fullNormXZ=({:.9}, {:.9})",
        start_matched,
        metrics.full_norm_drift_x == 0.0 && metrics.full_norm_drift_z == 0.0,
        metrics.full_norm_drift_x,
        metrics.full_norm_drift_z,
    );
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
        let filters = parse_case_filter(Some("jump_tap,shift_right")).expect("filters");
        let filtered: Vec<_> = build_cases()
            .into_iter()
            .filter(|case| filters.contains(&case.name.to_ascii_lowercase()))
            .collect();

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].name, "jump_tap");
        assert_eq!(filtered[1].name, "shift_right");
    }

    #[test]
    fn build_cases_assigns_stable_ordinals() {
        let cases = build_cases();
        assert_eq!(cases[0].ordinal, 1);
        assert_eq!(cases[13].ordinal, 14);
        assert_eq!(cases[14].ordinal, 15);
        assert_eq!(cases[13].name, "shift_right");
        assert_eq!(cases[14].name, "shift_left_right");
    }
}
