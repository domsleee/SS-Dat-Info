//! Determinism certificate — JSON artifact summarizing a test run.
//!
//! Emitted after regression or acceptance runs for CI traceability.

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::acceptance::AcceptanceResult;
use crate::regression::CaseResult;

/// Certificate for a regression suite run.
pub fn write_regression_certificate(
    results: &[CaseResult],
    mock: bool,
    csv_path: &Path,
    cert_path: &Path,
) {
    let total = results.len();
    let passed = results.iter().filter(|r| r.all_gates_pass).count();
    let max_drift_x: f64 = results
        .iter()
        .map(|r| r.replay_drift_x)
        .fold(0.0, f64::max);
    let max_drift_z: f64 = results
        .iter()
        .map(|r| r.replay_drift_z)
        .fold(0.0, f64::max);

    let cases_json: Vec<String> = results
        .iter()
        .map(|r| {
            format!(
                concat!(
                    "    {{\n",
                    "      \"name\": \"{}\",\n",
                    "      \"pattern\": \"{}\",\n",
                    "      \"rec_count\": {},\n",
                    "      \"replay_drift_x\": {:.9},\n",
                    "      \"replay_drift_z\": {:.9},\n",
                    "      \"replay_zero\": {},\n",
                    "      \"all_gates_pass\": {},\n",
                    "      \"error\": {}\n",
                    "    }}"
                ),
                r.name,
                r.pattern,
                r.rec_count,
                r.replay_drift_x,
                r.replay_drift_z,
                r.replay_zero,
                r.all_gates_pass,
                match &r.error {
                    Some(e) => format!("\"{}\"", e.replace('"', "\\\"")),
                    None => "null".to_string(),
                }
            )
        })
        .collect();

    let now = format_timestamp();
    let json = format!(
        concat!(
            "{{\n",
            "  \"type\": \"regression\",\n",
            "  \"timestamp\": \"{}\",\n",
            "  \"mock\": {},\n",
            "  \"total_cases\": {},\n",
            "  \"passed\": {},\n",
            "  \"failed\": {},\n",
            "  \"max_drift_x\": {:.9},\n",
            "  \"max_drift_z\": {:.9},\n",
            "  \"all_zero_drift\": {},\n",
            "  \"csv_path\": \"{}\",\n",
            "  \"verdict\": \"{}\",\n",
            "  \"cases\": [\n",
            "{}\n",
            "  ]\n",
            "}}\n"
        ),
        now,
        mock,
        total,
        passed,
        total - passed,
        max_drift_x,
        max_drift_z,
        passed == total && max_drift_x == 0.0 && max_drift_z == 0.0,
        csv_path.display().to_string().replace('\\', "/"),
        if passed == total { "PASS" } else { "FAIL" },
        cases_json.join(",\n"),
    );

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

/// Certificate for an acceptance test run.
pub fn write_acceptance_certificate(result: &AcceptanceResult, cert_path: &Path) {
    let now = format_timestamp();
    let json = format!(
        concat!(
            "{{\n",
            "  \"type\": \"acceptance\",\n",
            "  \"timestamp\": \"{}\",\n",
            "  \"steering\": \"{}\",\n",
            "  \"replay_steered\": \"{}\",\n",
            "  \"zero_drift\": \"{}\",\n",
            "  \"baseline_count\": {},\n",
            "  \"rec_count\": {},\n",
            "  \"max_drift_x\": {:.9},\n",
            "  \"max_drift_z\": {:.9},\n",
            "  \"max_base_vs_rec_x\": {:.4},\n",
            "  \"max_play_vs_base_x\": {:.4},\n",
            "  \"verdict\": \"{}\"\n",
            "}}\n"
        ),
        now,
        result.steering,
        result.replay_steered,
        result.zero_drift,
        result.baseline_count,
        result.rec_count,
        result.max_drift_x,
        result.max_drift_z,
        result.max_base_vs_rec_x,
        result.max_play_vs_base_x,
        if result.all_pass() { "PASS" } else { "FAIL" },
    );

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
    // Use std::time for a basic ISO-8601-ish timestamp without chrono dependency
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    // Simple epoch seconds — human-readable timestamps would need chrono
    format!("{}", secs)
}
