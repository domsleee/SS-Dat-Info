//! Determinism certificate — JSON artifact summarizing a test run.
//!
//! Emitted after regression or acceptance runs for CI traceability.

use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::acceptance::AcceptanceResult;
use crate::regression::CaseResult;

#[derive(Serialize)]
struct RegressionCertificate<'a> {
    r#type: &'static str,
    timestamp: String,
    total_cases: usize,
    passed: usize,
    failed: usize,
    max_raw_drift_x: f64,
    max_raw_drift_z: f64,
    max_full_norm_drift_x: f64,
    max_full_norm_drift_z: f64,
    all_zero_full_norm_drift: bool,
    csv_path: String,
    verdict: &'static str,
    cases: Vec<RegressionCase<'a>>,
}

#[derive(Serialize)]
struct RegressionCase<'a> {
    name: &'a str,
    pattern: &'a str,
    start_matched: bool,
    rec_count: u32,
    transitions: u32,
    first_input_tick: i32,
    frame0_dx: f64,
    frame0_dz: f64,
    full_norm_drift_x: f64,
    full_norm_drift_z: f64,
    active_start_dx: f64,
    active_start_dz: f64,
    active_window_ticks: u32,
    active_norm_drift_x: f64,
    active_norm_drift_z: f64,
    replay_drift_x: f64,
    replay_drift_z: f64,
    replay_zero: bool,
    all_gates_pass: bool,
    error: Option<&'a str>,
}

#[derive(Serialize)]
struct AcceptanceCertificate {
    r#type: &'static str,
    timestamp: String,
    steering: String,
    replay_steered: String,
    zero_drift: String,
    baseline_count: u32,
    rec_count: u32,
    max_drift_x: f64,
    max_drift_z: f64,
    max_base_vs_rec_x: f64,
    max_play_vs_base_x: f64,
    verdict: &'static str,
}

/// Certificate for a regression suite run.
pub fn write_regression_certificate(
    results: &[CaseResult],
    csv_path: &Path,
    cert_path: &Path,
) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.all_gates_pass).count();
    let max_raw_drift_x: f64 = results.iter().map(|r| r.replay_drift_x).fold(0.0, f64::max);
    let max_raw_drift_z: f64 = results.iter().map(|r| r.replay_drift_z).fold(0.0, f64::max);
    let max_full_norm_drift_x: f64 = results
        .iter()
        .map(|r| r.full_norm_drift_x)
        .fold(0.0, f64::max);
    let max_full_norm_drift_z: f64 = results
        .iter()
        .map(|r| r.full_norm_drift_z)
        .fold(0.0, f64::max);

    let cert = RegressionCertificate {
        r#type: "regression",
        timestamp: format_timestamp(),
        total_cases: total,
        passed,
        failed: total - passed,
        max_raw_drift_x,
        max_raw_drift_z,
        max_full_norm_drift_x,
        max_full_norm_drift_z,
        all_zero_full_norm_drift: passed == total
            && max_full_norm_drift_x == 0.0
            && max_full_norm_drift_z == 0.0,
        csv_path: csv_path.display().to_string().replace('\\', "/"),
        verdict: if passed == total { "PASS" } else { "FAIL" },
        cases: results
            .iter()
            .map(|r| RegressionCase {
                name: &r.name,
                pattern: &r.pattern,
                start_matched: r.start_matched,
                rec_count: r.rec_count,
                transitions: r.transitions,
                first_input_tick: r.first_input_tick,
                frame0_dx: r.frame0_dx,
                frame0_dz: r.frame0_dz,
                full_norm_drift_x: r.full_norm_drift_x,
                full_norm_drift_z: r.full_norm_drift_z,
                active_start_dx: r.active_start_dx,
                active_start_dz: r.active_start_dz,
                active_window_ticks: r.active_window_ticks,
                active_norm_drift_x: r.active_norm_drift_x,
                active_norm_drift_z: r.active_norm_drift_z,
                replay_drift_x: r.replay_drift_x,
                replay_drift_z: r.replay_drift_z,
                replay_zero: r.replay_zero,
                all_gates_pass: r.all_gates_pass,
                error: r.error.as_deref(),
            })
            .collect(),
    };

    let json = serde_json::to_string_pretty(&cert).unwrap_or_else(|e| {
        eprintln!("WARNING: Failed to serialize certificate: {}", e);
        String::new()
    });

    write_cert_file(&json, cert_path);
}

/// Certificate for an acceptance test run.
pub fn write_acceptance_certificate(result: &AcceptanceResult, cert_path: &Path) {
    let cert = AcceptanceCertificate {
        r#type: "acceptance",
        timestamp: format_timestamp(),
        steering: result.steering.to_string(),
        replay_steered: result.replay_steered.to_string(),
        zero_drift: result.zero_drift.to_string(),
        baseline_count: result.baseline_count,
        rec_count: result.rec_count,
        max_drift_x: result.max_drift_x,
        max_drift_z: result.max_drift_z,
        max_base_vs_rec_x: result.max_base_vs_rec_x,
        max_play_vs_base_x: result.max_play_vs_base_x,
        verdict: if result.all_pass() { "PASS" } else { "FAIL" },
    };

    let json = serde_json::to_string_pretty(&cert).unwrap_or_else(|e| {
        eprintln!("WARNING: Failed to serialize certificate: {}", e);
        String::new()
    });

    write_cert_file(&json, cert_path);
}

fn write_cert_file(json: &str, cert_path: &Path) {
    if let Some(parent) = cert_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match fs::File::create(cert_path) {
        Ok(mut f) => {
            let _ = f.write_all(json.as_bytes());
            println!("\nDeterminism certificate: {}", cert_path.display());
        }
        Err(e) => eprintln!("WARNING: Failed to write certificate: {}", e),
    }
}

fn format_timestamp() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    static CERT_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn temp_path(name: &str) -> PathBuf {
        let id = CERT_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("tas_cert_{}_{}_{}", std::process::id(), id, name))
    }

    fn sample_case_results() -> Vec<CaseResult> {
        vec![
            CaseResult {
                name: "L".to_string(),
                pattern: "L".to_string(),
                start_matched: true,
                rec_count: 56,
                transitions: 1,
                first_input_tick: 0,
                frame0_dx: 0.0,
                frame0_dz: 0.0,
                full_norm_drift_x: 0.0,
                full_norm_drift_z: 0.0,
                active_start_dx: 0.0,
                active_start_dz: 0.0,
                active_window_ticks: 56,
                active_norm_drift_x: 0.0,
                active_norm_drift_z: 0.0,
                live_drift_x: 0.0,
                live_drift_z: 0.0,
                live_zero: true,
                replay_drift_x: 0.0,
                replay_drift_z: 0.0,
                replay_zero: true,
                all_gates_pass: true,
                error: None,
            },
            CaseResult {
                name: "LR".to_string(),
                pattern: "LR".to_string(),
                start_matched: false,
                rec_count: 112,
                transitions: 2,
                first_input_tick: 0,
                frame0_dx: 0.25,
                frame0_dz: 0.5,
                full_norm_drift_x: 0.0,
                full_norm_drift_z: 0.0,
                active_start_dx: 0.25,
                active_start_dz: 0.5,
                active_window_ticks: 112,
                active_norm_drift_x: 0.5,
                active_norm_drift_z: 0.0,
                live_drift_x: 0.0,
                live_drift_z: 0.0,
                live_zero: true,
                replay_drift_x: 0.5,
                replay_drift_z: 0.0,
                replay_zero: false,
                all_gates_pass: false,
                error: Some("Could not position-match playback start".to_string()),
            },
        ]
    }

    #[test]
    fn regression_certificate_creates_valid_json() {
        let results = sample_case_results();
        let csv_path = PathBuf::from("test.csv");
        let cert_path = temp_path("regression.json");

        write_regression_certificate(&results, &csv_path, &cert_path);

        let content = std::fs::read_to_string(&cert_path).expect("read cert");
        // Verify it parses as valid JSON
        let val: serde_json::Value = serde_json::from_str(&content).expect("valid JSON");
        assert_eq!(val["type"], "regression");
        assert_eq!(val["total_cases"], 2);
        assert_eq!(val["passed"], 1);
        assert_eq!(val["failed"], 1);
        assert_eq!(val["verdict"], "FAIL");
        assert_eq!(val["max_raw_drift_x"], 0.5);
        assert_eq!(val["max_full_norm_drift_x"], 0.0);
        assert_eq!(val["cases"][0]["name"], "L");
        assert_eq!(val["cases"][1]["name"], "LR");
        assert_eq!(val["cases"][1]["start_matched"], false);
        assert_eq!(val["cases"][1]["full_norm_drift_x"], 0.0);
        assert_eq!(val["cases"][1]["full_norm_drift_z"], 0.0);
        assert!(val["cases"][0]["error"].is_null());
        assert_eq!(
            val["cases"][1]["error"],
            "Could not position-match playback start"
        );

        let _ = std::fs::remove_file(&cert_path);
    }

    #[test]
    fn regression_certificate_all_pass() {
        let results = vec![sample_case_results().remove(0)];
        let cert_path = temp_path("regression_pass.json");

        write_regression_certificate(&results, &PathBuf::from("x.csv"), &cert_path);

        let content = std::fs::read_to_string(&cert_path).expect("read cert");
        let val: serde_json::Value = serde_json::from_str(&content).expect("valid JSON");
        assert_eq!(val["verdict"], "PASS");
        assert_eq!(val["all_zero_full_norm_drift"], true);

        let _ = std::fs::remove_file(&cert_path);
    }

    #[test]
    fn acceptance_certificate_creates_valid_json() {
        let result = AcceptanceResult {
            steering: crate::acceptance::Verdict::Pass,
            replay_steered: crate::acceptance::Verdict::Pass,
            zero_drift: crate::acceptance::Verdict::Pass,
            baseline_count: 500,
            rec_count: 500,
            max_drift_x: 0.0,
            max_drift_z: 0.0,
            max_base_vs_rec_x: 15.5,
            max_play_vs_base_x: 14.2,
            gates_pass: true,
            playback_complete: true,
        };
        let cert_path = temp_path("acceptance.json");

        write_acceptance_certificate(&result, &cert_path);

        let content = std::fs::read_to_string(&cert_path).expect("read cert");
        let val: serde_json::Value = serde_json::from_str(&content).expect("valid JSON");
        assert_eq!(val["type"], "acceptance");
        assert_eq!(val["steering"], "PASS");
        assert_eq!(val["replay_steered"], "PASS");
        assert_eq!(val["zero_drift"], "PASS");
        assert_eq!(val["verdict"], "PASS");
        assert_eq!(val["baseline_count"], 500);

        let _ = std::fs::remove_file(&cert_path);
    }

    #[test]
    fn acceptance_certificate_fail_verdict() {
        let result = AcceptanceResult {
            steering: crate::acceptance::Verdict::Fail,
            replay_steered: crate::acceptance::Verdict::Pass,
            zero_drift: crate::acceptance::Verdict::Pass,
            baseline_count: 300,
            rec_count: 300,
            max_drift_x: 0.0,
            max_drift_z: 0.0,
            max_base_vs_rec_x: 0.1,
            max_play_vs_base_x: 0.1,
            gates_pass: false,
            playback_complete: true,
        };
        let cert_path = temp_path("acceptance_fail.json");

        write_acceptance_certificate(&result, &cert_path);

        let content = std::fs::read_to_string(&cert_path).expect("read cert");
        let val: serde_json::Value = serde_json::from_str(&content).expect("valid JSON");
        assert_eq!(val["verdict"], "FAIL");
        assert_eq!(val["steering"], "FAIL");

        let _ = std::fs::remove_file(&cert_path);
    }
}
