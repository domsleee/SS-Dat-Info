//! Determinism certificate: a JSON artifact summarizing a regression or
//! acceptance run, for CI traceability.

use std::path::Path;

use serde::Serialize;
use serde_json::json;

use crate::acceptance::AcceptanceResult;
use crate::regression::CaseResult;

pub fn write_regression(results: &[CaseResult], csv_path: &Path, cert_path: &Path) {
    let passed = results.iter().filter(|r| r.all_gates_pass).count();
    let max = |value: fn(&CaseResult) -> f64| results.iter().map(value).fold(0.0, f64::max);
    write(
        cert_path,
        json!({
            "type": "regression",
            "timestamp": unix_secs(),
            "total_cases": results.len(),
            "passed": passed,
            "failed": results.len() - passed,
            "max_raw_drift_x": max(|r| r.replay_drift_x),
            "max_raw_drift_z": max(|r| r.replay_drift_z),
            "max_full_norm_drift_x": max(|r| r.full_norm_drift_x),
            "max_full_norm_drift_z": max(|r| r.full_norm_drift_z),
            "csv_path": csv_path.display().to_string().replace('\\', "/"),
            "verdict": verdict(passed == results.len()),
            "cases": results,
        }),
    );
}

pub fn write_acceptance(result: &AcceptanceResult, cert_path: &Path) {
    write(
        cert_path,
        json!({
            "type": "acceptance",
            "timestamp": unix_secs(),
            "verdict": verdict(result.all_pass()),
            "result": result,
        }),
    );
}

fn verdict(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "FAIL"
    }
}

fn unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn write(cert_path: &Path, body: impl Serialize) {
    if let Some(parent) = cert_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let written = serde_json::to_string_pretty(&body)
        .map_err(|e| e.to_string())
        .and_then(|json| std::fs::write(cert_path, json).map_err(|e| e.to_string()));
    match written {
        Ok(()) => println!("\nDeterminism certificate: {}", cert_path.display()),
        Err(e) => eprintln!("WARNING: Failed to write certificate: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::acceptance::Verdict;

    #[test]
    fn verdicts_serialize_as_their_printed_form() {
        assert_eq!(serde_json::to_string(&Verdict::Pass).unwrap(), "\"PASS\"");
        assert_eq!(serde_json::to_string(&Verdict::Fail).unwrap(), "\"FAIL\"");
    }
}
