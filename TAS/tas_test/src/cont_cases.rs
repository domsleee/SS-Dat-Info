//! Named real-recording cases sharing the CONT reliability runner.
use crate::cont_reliability::{self, ContReliabilityReport};
use std::path::PathBuf;

pub struct Case {
    recording: &'static str,
    splice: u32,
    iterations: u32,
    speeds: &'static [f32],
    timing: Option<(u32, f64)>, // maximum overshoot; maximum best resume time (ms)
    signature: &'static str,
}

pub const FE_TREMENDOUS: Case = Case {
    recording: "FE-tremendous.tasrec",
    splice: 2200,
    iterations: 5,
    speeds: &[12.0],
    timing: None,
    signature: "FE-CONT-RELIABILITY",
};

pub const FE_10065: Case = Case {
    recording: "FE-10065.tasrec",
    splice: 6200,
    iterations: 8,
    speeds: &[64.0, 256.0],
    timing: Some((1, 3000.0)),
    signature: "FE-10065 CONT",
};

fn recording_path(case: &Case) -> Result<PathBuf, String> {
    let relative = PathBuf::from("TAS/recordings").join(case.recording);
    let mut candidates = vec![];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("../../..").join(&relative));
        }
    }
    candidates.extend([relative, PathBuf::from("recordings").join(case.recording)]);
    candidates
        .iter()
        .find(|p| p.is_file())
        .cloned()
        .ok_or_else(|| format!("Couldn't locate {} (tried {candidates:?})", case.recording))
}

fn timing_passes(case: &Case, max_overshoot: u32, best_resume_ms: f64) -> bool {
    case.timing
        .is_none_or(|(frames, ms)| max_overshoot <= frames && best_resume_ms <= ms)
}

fn judge(case: &Case, report: &ContReliabilityReport) -> bool {
    let max_overshoot = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.splice_recorded_count.saturating_sub(case.splice))
        .max()
        .unwrap_or(u32::MAX);
    let best_resume_ms = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.resume_ms)
        .fold(f64::INFINITY, f64::min);
    let clean = report.all_pass() && report.results.len() == case.iterations as usize;
    let timing = timing_passes(case, max_overshoot, best_resume_ms);
    println!("  {}x: clean={clean}, overshoot={max_overshoot}, best_resume_ms={best_resume_ms:.0}, timing_ok={timing}", report.speed);
    clean && timing
}

pub fn run(case: &Case) -> bool {
    let path = match recording_path(case) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("{error}");
            return false;
        }
    };
    println!(
        "=== {}: {} splice={} iterations={} speeds={:?} ===",
        case.signature,
        path.display(),
        case.splice,
        case.iterations,
        case.speeds
    );
    let mut passed = true;
    for &speed in case.speeds {
        let report = cont_reliability::run(
            case.iterations,
            speed,
            case.splice,
            Some(&path.to_string_lossy()),
            cont_reliability::BaselineInputProfile::Taps,
            None,
        );
        passed &= judge(case, &report);
    }
    println!(
        "*** {} {} ***",
        case.signature,
        if passed { "PASSED" } else { "FAILED" }
    );
    passed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_contracts_are_preserved() {
        assert_eq!((FE_TREMENDOUS.splice, FE_TREMENDOUS.iterations), (2200, 5));
        assert_eq!(FE_TREMENDOUS.speeds, &[12.0]);
        assert_eq!((FE_10065.splice, FE_10065.iterations), (6200, 8));
        assert_eq!(FE_10065.speeds, &[64.0, 256.0]);
        assert!(timing_passes(&FE_TREMENDOUS, 100, 9000.0));
        assert!(timing_passes(&FE_10065, 1, 3000.0));
        assert!(!timing_passes(&FE_10065, 2, 3000.0));
        assert!(!timing_passes(&FE_10065, 1, 3000.1));
        assert!(!timing_passes(&FE_10065, 0, f64::NAN));
    }

    #[test]
    fn empty_report_cannot_pass_either_case() {
        for case in [&FE_TREMENDOUS, &FE_10065] {
            let report = ContReliabilityReport {
                iterations: case.iterations,
                speed: case.speeds[0],
                splice_frame: case.splice,
                baseline_ticks: 0,
                baseline_profile: String::new(),
                baseline_transitions: 0,
                results: vec![],
            };
            assert!(!judge(case, &report));
        }
    }

    #[test]
    fn case_runner_keeps_drift_coverage_progress_and_timing_gates() {
        use crate::cont_reliability::ContCycleResult;
        for case in [&FE_TREMENDOUS, &FE_10065] {
            let mut report = ContReliabilityReport {
                iterations: case.iterations,
                speed: case.speeds[0],
                splice_frame: case.splice,
                baseline_ticks: 10000,
                baseline_profile: "taps".into(),
                baseline_transitions: 10,
                results: (0..case.iterations)
                    .map(|iteration| ContCycleResult {
                        iteration,
                        spliced: true,
                        rerolls: 0,
                        resume_ms: 1000.0,
                        mode_rec_after_splice: true,
                        replay_coverage_ok: true,
                        playback_pos_at_splice: case.splice,
                        splice_recorded_count: case.splice,
                        replay_start_fc: 0,
                        splice_fc: 0,
                        max_drift_x: 0.0,
                        max_drift_y: 0.0,
                        max_drift_z: 0.0,
                        max_drift_frame_x: 0,
                        max_drift_frame_z: 0,
                        forward_only_ok: true,
                        prefix_net_z: 1.0,
                        prefix_forward_steps: 1,
                        prefix_backward_steps: 0,
                        prefix_min_x: 0.0,
                        prefix_max_x: 0.0,
                        prefix_min_z: 0.0,
                        prefix_max_z: 1.0,
                    })
                    .collect(),
            };
            assert!(judge(case, &report));
            for axis in 0..3 {
                let r = &mut report.results[0];
                match axis {
                    0 => r.max_drift_x = 0.1,
                    1 => r.max_drift_y = 0.1,
                    _ => r.max_drift_z = f64::NAN,
                }
                assert!(!judge(case, &report));
                let r = &mut report.results[0];
                r.max_drift_x = 0.0;
                r.max_drift_y = 0.0;
                r.max_drift_z = 0.0;
            }
            report.results[0].replay_coverage_ok = false;
            assert!(!judge(case, &report));
            report.results[0].replay_coverage_ok = true;
            report.results[0].forward_only_ok = false;
            assert!(!judge(case, &report));
            report.results[0].forward_only_ok = true;
            report.results[0].splice_recorded_count += 2;
            assert_eq!(judge(case, &report), case.timing.is_none());
            report.results[0].splice_recorded_count = case.splice;
            for r in &mut report.results {
                r.resume_ms = 3001.0;
            }
            assert_eq!(judge(case, &report), case.timing.is_none());
            report.results.pop();
            assert!(!judge(case, &report));
        }
    }
}
