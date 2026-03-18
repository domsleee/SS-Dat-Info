//! Regression suite — 15 test cases ported from _drift_regression_suite.lua.
//!
//! Each case: scripted steering pattern -> REC -> PLAY -> drift check -> CSV output.

use std::fs;
use std::io::Write;
use std::path::Path;

use tas_shared::input_bits;

use crate::cache::{self, Recording};
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

/// Result of running one regression case.
#[derive(Debug)]
pub struct CaseResult {
    pub name: String,
    pub pattern: String,
    pub rec_count: u32,
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

    vec![
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
    ]
}

fn case_pattern(name: &str, pattern: &str, hold: u32, gap: u32) -> RegressionCase {
    RegressionCase {
        name: name.to_string(),
        steps: patterns::build_from_pattern(pattern, hold, gap),
        pattern_str: pattern.to_string(),
    }
}

fn case_explicit(name: &str, defs: &[(&str, u8, u32)]) -> RegressionCase {
    RegressionCase {
        name: name.to_string(),
        steps: patterns::build_from_explicit(defs),
        pattern_str: format!("explicit:{}", name),
    }
}

/// Run the full regression suite.
///
/// If `mock` is true, uses mock input mode (writes input directly to shared memory).
/// If `mock` is false, uses Pico HID for real input during REC.
pub fn run(mock: bool, cache_dir: &Path, csv_path: &Path) -> Vec<CaseResult> {
    let cases = build_cases();
    let mut results = Vec::new();

    // CSV header
    write_csv_header(csv_path);

    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Game not alive. Aborting regression suite.");
        std::process::exit(1);
    }

    println!(
        "\n=== Regression Suite: {} cases, mock={} ===\n",
        cases.len(),
        mock
    );

    for (i, case) in cases.iter().enumerate() {
        println!(
            "--- Case {}/{}: {} (pattern: {}) ---",
            i + 1,
            cases.len(),
            case.name,
            case.pattern_str
        );

        let result = run_single_case(&mut client, case, i, mock, cache_dir);
        append_csv(csv_path, &result);

        println!(
            "  Result: drift_x={:.9} drift_z={:.9} zero={} gates={}",
            result.replay_drift_x,
            result.replay_drift_z,
            result.replay_zero,
            if result.all_gates_pass { "PASS" } else { "FAIL" }
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
            "  {} {} — drift({:.9}, {:.9})",
            if r.all_gates_pass { "PASS" } else { "FAIL" },
            r.name,
            r.replay_drift_x,
            r.replay_drift_z,
        );
    }

    results
}

fn run_single_case(
    client: &mut tas_shared::TasSharedMemoryClient,
    case: &RegressionCase,
    index: usize,
    mock: bool,
    cache_dir: &Path,
) -> CaseResult {
    let input_log = patterns::generate_input_log(&case.steps);
    let total_ticks = patterns::total_ticks(&case.steps);
    let cache_path = cache_dir.join(format!("{:02}_{}.tas", index + 1, slug(&case.name)));

    if mock {
        return run_mock_case(client, case, &input_log, total_ticks, &cache_path);
    }

    // --- Live mode: F5 + REC with Pico HID steering ---
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

    // Save recording to cache
    let recording = cache::capture_from_state(client.state(), &case.name);
    if let Err(e) = recording.save(&cache_path) {
        eprintln!("  WARNING: Failed to save cache: {}", e);
    }

    // Compute live drift (REC vs itself is always zero, but we track the recording quality)
    let live_transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);
    println!("  Live transitions: {}", live_transitions);

    // Phase 2: Playback
    if !harness::restart_and_stabilize(client) {
        return error_result(case, "Game not alive after restart (PLAY phase)");
    }

    harness::arm_play(client);
    let play_ok = harness::wait_playback(client, rec_count);

    if !play_ok {
        return error_result(case, "Playback timeout");
    }

    // Assess
    let assessment = gates::run_gates(client.state(), rec_count);
    assessment.print_summary();

    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        rec_count,
        live_drift_x: 0.0, // REC vs itself
        live_drift_z: 0.0,
        live_zero: true,
        replay_drift_x: assessment.drift.max_drift_x,
        replay_drift_z: assessment.drift.max_drift_z,
        replay_zero: assessment.drift.is_zero(),
        all_gates_pass: assessment.all_pass(),
        error: None,
    }
}

fn run_mock_case(
    client: &mut tas_shared::TasSharedMemoryClient,
    case: &RegressionCase,
    input_log: &[u8],
    _total_ticks: u32,
    cache_path: &Path,
) -> CaseResult {
    // Write mock input directly to shared memory
    harness::write_mock_input(client, input_log);

    // Save cache
    let recording = Recording {
        header: cache::RecordingHeader {
            version: client.state().version,
            tick_count: input_log.len() as u32,
            inject_mode: client.state().inject_mode,
            force_fixed_tick: client.state().force_fixed_tick,
            force_direct: client.state().force_direct,
            label: case.name.clone(),
        },
        input_log: input_log.to_vec(),
        rec_coords: vec![[0.0; 3]; input_log.len()],
    };
    let _ = recording.save(cache_path);

    // F5 + PLAY
    if !harness::restart_and_stabilize(client) {
        return error_result(case, "Game not alive after restart (mock PLAY)");
    }

    harness::arm_play(client);
    let play_ok = harness::wait_playback(client, input_log.len() as u32);

    if !play_ok {
        return error_result(case, "Mock playback timeout");
    }

    // For mock mode, Gate 1 (REC movement) is skipped since we didn't do a real REC
    let state = client.state();
    let drift_result = drift::compute_drift(state, input_log.len() as u32);

    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        rec_count: input_log.len() as u32,
        live_drift_x: 0.0,
        live_drift_z: 0.0,
        live_zero: true,
        replay_drift_x: drift_result.max_drift_x,
        replay_drift_z: drift_result.max_drift_z,
        replay_zero: drift_result.is_zero(),
        all_gates_pass: drift_result.is_zero(), // simplified for mock
        error: None,
    }
}

/// Drive Pico HID according to the pattern step schedule.
fn drive_pico_pattern(steps: &[PatternStep]) {
    use std::io::Write;

    let port = match std::fs::OpenOptions::new()
        .write(true)
        .open("\\\\.\\COM7")
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("  ERROR: Cannot open COM7 for Pico HID: {}", e);
            return;
        }
    };

    let mut port = port;
    let mut prev_mask = 0xFFu8; // force initial send
    let start = std::time::Instant::now();

    // Convert steps to a tick-by-tick schedule
    let total = patterns::total_ticks(steps);
    // Approximate: 100 ticks/sec (game runs at ~100fps with fft=0 or 2 ticks/frame at ~50fps)
    // We use 10ms per tick as approximation
    let ms_per_tick = 10u64;

    let mut current_step = 0usize;
    for tick in 0..total {
        // Find which step we're in
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

        // Wait for next tick
        let target = std::time::Duration::from_millis((tick as u64 + 1) * ms_per_tick);
        if let Some(remaining) = target.checked_sub(start.elapsed()) {
            std::thread::sleep(remaining);
        }
    }

    // Release all keys
    let _ = port.write_all(&[0xFF]);
    let _ = port.flush();
}

fn error_result(case: &RegressionCase, msg: &str) -> CaseResult {
    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        rec_count: 0,
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
            "case_name,pattern,rec_count,live_drift_x,live_drift_z,live_zero,replay_drift_x,replay_drift_z,replay_zero,all_gates_pass,error"
        );
    }
}

fn append_csv(path: &Path, r: &CaseResult) {
    if let Ok(mut f) = fs::OpenOptions::new().append(true).open(path) {
        let _ = writeln!(
            f,
            "\"{}\",\"{}\",{},{:.9},{:.9},{},{:.9},{:.9},{},{},\"{}\"",
            r.name,
            r.pattern,
            r.rec_count,
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
