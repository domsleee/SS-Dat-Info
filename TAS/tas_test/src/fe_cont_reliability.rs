//! FE-tremendous CONT-reliability test.
//!
//! Pins down the regression the user hit: loading `FE-tremendous.tasrec`
//! and doing a CONT splice at frame 2200 (resume recording from frame
//! 2200 onward) must produce zero drift over the replayed prefix. Repeats
//! the splice 5 times at the default 12× catch-up speed; passes only if
//! all 5 succeed with zero drift.
//!
//! This is the existing `cont-reliability` test pinned to a specific
//! .tasrec fixture in the repo and a specific splice frame, so the case
//! the user cares about is part of the standard test surface and any
//! future regression on it shows up immediately.

use std::path::PathBuf;

use crate::cont_reliability;

const SPLICE_FRAME: u32 = 2200;
const ITERATIONS: u32 = 5;
const SPEED: f32 = 12.0;
/// Path (relative to repo root) of the recording this test pins.
const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";

pub fn run() -> bool {
    println!(
        "=== FE-tremendous CONT Reliability (splice={} ×{} at {}× catchup) ===\n",
        SPLICE_FRAME, ITERATIONS, SPEED
    );

    // Resolve the recording path relative to the workspace. tas_test runs
    // from `TAS/target/release/`, so step up two levels then drop in.
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidate_paths = [
        // Repo root next to TAS/
        exe_dir.join("../../..").join(RECORDING_REL),
        // CWD next to repo root
        PathBuf::from(RECORDING_REL),
        // CWD inside TAS/
        PathBuf::from("recordings/FE-tremendous.tasrec"),
    ];
    let path = candidate_paths
        .iter()
        .find(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned());
    let path = match path {
        Some(p) => p,
        None => {
            eprintln!(
                "ERROR: Couldn't locate {} (tried {:?})",
                RECORDING_REL,
                candidate_paths
                    .iter()
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
            );
            return false;
        }
    };
    println!("  Using recording: {}", path);

    let report = cont_reliability::run(
        ITERATIONS,
        SPEED,
        SPLICE_FRAME,
        Some(&path),
        cont_reliability::BaselineInputProfile::Taps,
        None,
    );

    let pass = report.all_pass();
    if pass {
        println!(
            "\n*** FE-CONT-RELIABILITY PASSED: {}/{} splices clean against {} ***",
            ITERATIONS, ITERATIONS, RECORDING_REL
        );
    } else {
        let fails = report
            .results
            .iter()
            .filter(|r| {
                !(r.spliced
                    && r.mode_rec_after_splice
                    && r.replay_coverage_ok
                    && r.forward_only_ok
                    && r.max_drift_x == 0.0
                    && r.max_drift_z == 0.0)
            })
            .count();
        println!(
            "\n*** FE-CONT-RELIABILITY FAILED: {} of {} splices had drift or other failure ***",
            fails, ITERATIONS
        );
    }
    pass
}
