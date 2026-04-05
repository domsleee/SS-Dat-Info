//! Regression suite — 15 test cases ported from _drift_regression_suite.lua.
//!
//! Each case: scripted steering pattern -> REC -> PLAY -> drift check -> CSV output.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tas_shared::input_bits;
use tas_shared::TAS_MAX_TICKS;

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
    let baseline = match load_mock_baseline(cache_path) {
        Ok(baseline) => baseline,
        Err(err) => return error_result(case, &err),
    };

    let baseline_count = baseline.recording.header.tick_count;
    if baseline.recording.input_log != input_log {
        println!(
            "  Mock baseline uses cached live input ({} ticks) instead of generated pattern ({} ticks)",
            baseline_count,
            input_log.len()
        );
    }
    println!(
        "  Mock baseline: {} ({} ticks)",
        baseline.path.display(),
        baseline_count
    );

    if let Err(err) = baseline.recording.save(cache_path) {
        eprintln!("  WARNING: Failed to write mock cache: {}", err);
    }
    if let Err(err) = write_recording_to_state(client.state_mut(), &baseline.recording) {
        return error_result(case, &format!("Failed to stage mock baseline: {}", err));
    }

    let target = baseline.recording.rec_coords[0];
    if !harness::restart_and_stabilize(client) {
        return error_result(case, "Game not alive after restart (mock PLAY)");
    }
    println!(
        "  Mock start target: ({:.6}, {:.6}, {:.6})",
        target[0], target[1], target[2]
    );
    harness::arm_play(client);

    let play_ok = harness::wait_playback(client, baseline_count);

    if !play_ok {
        return error_result(case, "Mock playback timeout");
    }

    // For mock mode, Gate 1 (REC movement) is skipped since we didn't do a real REC
    let state = client.state();
    let drift_result = drift::compute_drift(state, input_log.len() as u32);

    CaseResult {
        name: case.name.clone(),
        pattern: case.pattern_str.clone(),
        rec_count: baseline_count,
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

/// Drive Pico HID according to the pattern step schedule (delegates to harness).
fn drive_pico_pattern(steps: &[PatternStep]) {
    harness::drive_pico_steps(steps, None);
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

#[derive(Debug)]
struct MockBaseline {
    path: PathBuf,
    recording: Recording,
}

fn load_mock_baseline(cache_path: &Path) -> Result<MockBaseline, String> {
    let mut problems = Vec::new();

    for candidate in mock_baseline_candidates(cache_path) {
        if !candidate.exists() {
            problems.push(format!("{} (missing)", candidate.display()));
            continue;
        }

        match Recording::load(&candidate) {
            Ok(recording) => match validate_mock_baseline(&recording) {
                Ok(()) => {
                    return Ok(MockBaseline {
                        path: candidate,
                        recording,
                    });
                }
                Err(reason) => {
                    problems.push(format!("{} ({})", candidate.display(), reason));
                }
            },
            Err(err) => {
                problems.push(format!("{} (load failed: {})", candidate.display(), err));
            }
        }
    }

    Err(format!(
        "No valid mock baseline found. Checked:\n  - {}",
        problems.join("\n  - ")
    ))
}

fn mock_baseline_candidates(cache_path: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(file_name) = cache_path.file_name() {
        if let Some(mock_dir) = cache_path.parent() {
            if let Some(output_dir) = mock_dir.parent() {
                push_unique_path(
                    &mut candidates,
                    output_dir.join("regression_cache").join(file_name),
                );
            }
        }

        push_unique_path(&mut candidates, mock_fixture_dir().join(file_name));
    }

    push_unique_path(&mut candidates, cache_path.to_path_buf());

    candidates
}

fn push_unique_path(paths: &mut Vec<PathBuf>, candidate: PathBuf) {
    if !paths.iter().any(|existing| existing == &candidate) {
        paths.push(candidate);
    }
}

fn mock_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("regression_cache")
}

fn validate_mock_baseline(recording: &Recording) -> Result<(), String> {
    let count = recording.header.tick_count as usize;
    if count == 0 {
        return Err("recording is empty".into());
    }
    if count > TAS_MAX_TICKS {
        return Err(format!("tick_count {} exceeds TAS_MAX_TICKS", count));
    }
    if recording.input_log.len() < count {
        return Err(format!(
            "input log too short: expected {}, got {}",
            count,
            recording.input_log.len()
        ));
    }
    if recording.rec_coords.len() < count {
        return Err(format!(
            "rec_coords too short: expected {}, got {}",
            count,
            recording.rec_coords.len()
        ));
    }
    if !recording.rec_coords[..count]
        .iter()
        .any(|coord| coord[0] != 0.0 || coord[1] != 0.0 || coord[2] != 0.0)
    {
        return Err("rec_coords are all zero".into());
    }
    Ok(())
}

fn write_recording_to_state(
    state: &mut tas_shared::TasSharedState,
    recording: &Recording,
) -> Result<(), String> {
    let count = recording.header.tick_count as usize;
    validate_mock_baseline(recording)?;

    state.recorded_count = recording.header.tick_count;
    state.inject_mode = recording.header.inject_mode;
    state.force_fixed_tick = recording.header.force_fixed_tick;
    state.force_direct = recording.header.force_direct;
    state.input_log[..count].copy_from_slice(&recording.input_log[..count]);
    state.rec_coords[..count].copy_from_slice(&recording.rec_coords[..count]);
    state.input_log[count..].fill(0);
    state.rec_coords[count..].fill([0.0; 3]);
    state.play_coords[count..].fill([0.0; 3]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), id));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn make_recording(label: &str, count: u32, zero_coords: bool) -> Recording {
        let n = count as usize;
        let input_log = vec![input_bits::LEFT; n];
        let rec_coords = if zero_coords {
            vec![[0.0; 3]; n]
        } else {
            (0..n)
                .map(|i| [100.0 + i as f32, -50.0, 200.0 + i as f32 * 0.5])
                .collect()
        };

        Recording {
            header: cache::RecordingHeader {
                version: 4,
                tick_count: count,
                inject_mode: 6,
                force_fixed_tick: 0,
                force_direct: 2,
                label: label.to_string(),
            },
            input_log,
            rec_coords,
        }
    }

    #[test]
    fn load_mock_baseline_prefers_valid_regression_cache() {
        let root = unique_temp_dir("mock_baseline_prefers_regression");
        let mock_dir = root.join("mock_cache");
        let regression_dir = root.join("regression_cache");
        std::fs::create_dir_all(&mock_dir).expect("create mock dir");
        std::fs::create_dir_all(&regression_dir).expect("create regression dir");

        let mock_path = mock_dir.join("01_case.tas");
        let regression_path = regression_dir.join("01_case.tas");
        make_recording("bad", 10, true)
            .save(&mock_path)
            .expect("save invalid mock baseline");
        make_recording("good", 12, false)
            .save(&regression_path)
            .expect("save valid regression baseline");

        let baseline = load_mock_baseline(&mock_path).expect("load baseline");
        assert_eq!(baseline.path, regression_path);
        assert_eq!(baseline.recording.header.tick_count, 12);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn load_mock_baseline_rejects_zero_coord_recordings() {
        let root = unique_temp_dir("mock_baseline_rejects_zero");
        let mock_dir = root.join("mock_cache");
        std::fs::create_dir_all(&mock_dir).expect("create mock dir");

        let mock_path = mock_dir.join("missing_fixture_case.tas");
        make_recording("bad", 8, true)
            .save(&mock_path)
            .expect("save invalid mock baseline");

        let err = load_mock_baseline(&mock_path).unwrap_err();
        assert!(err.contains("rec_coords are all zero"));

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn write_recording_to_state_copies_and_clears_tail() {
        let recording = make_recording("state", 3, false);
        let mut state = tas_shared::zeroed_boxed();
        state.input_log[5] = 0xFF;
        state.rec_coords[5] = [1.0, 2.0, 3.0];
        state.play_coords[5] = [4.0, 5.0, 6.0];

        write_recording_to_state(&mut state, &recording).expect("write recording");

        assert_eq!(state.recorded_count, 3);
        assert_eq!(state.input_log[0], input_bits::LEFT);
        assert_eq!(state.input_log[2], input_bits::LEFT);
        assert_eq!(state.input_log[3], 0);
        assert_eq!(state.rec_coords[0], [100.0, -50.0, 200.0]);
        assert_eq!(state.rec_coords[2], [102.0, -50.0, 201.0]);
        assert_eq!(state.rec_coords[3], [0.0, 0.0, 0.0]);
        assert_eq!(state.play_coords[3], [0.0, 0.0, 0.0]);
    }
}
