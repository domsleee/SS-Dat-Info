//! FE-10065 CONT reliability + resume-timing regression.
//!
//! Pins the user's real case: load `TAS/recordings/FE-10065.tasrec` and CONT-
//! splice at frame 6200. Runs at MULTIPLE catch-up speeds (64× and 128×) and
//! passes only if EVERY splice is zero-drift AND the resume is frame-exact
//! (post-splice overshoot ≤ 1) AND the catch-up stays fast.
//!
//! The overshoot assertion is the Problem B guard. The fix (cave5: land the
//! catch-up batch exactly on the splice + reset the game clock accumulator to
//! "now" on the resume frame) is speed-INDEPENDENT, so we assert it at a high
//! speed (128×) too — that's the regression that would catch the resume drifting
//! at speed. Without the fix the resumed REC fast-forwards a variable window
//! (rec_cnt was 6209..6322); with it, rec_cnt == 6200.

use std::path::PathBuf;

use crate::cont_reliability;

const SPLICE_FRAME: u32 = 6200;
const ITERATIONS: u32 = 8;
/// Catch-up speeds to validate. 64× = baseline; 256× is the current default
/// (≈ the game's ~80× physics ceiling) and exercises the speed-independence of
/// the frame-exact resume fix at the top of the usable range.
const SPEEDS: &[f32] = &[64.0, 256.0];
/// Frame-exact resume: the recording must start at the splice frame. With the
/// fix + a sub-tick splice poll this is 0; allow 1 for rare OS-timer jitter.
const MAX_OVERSHOOT: u32 = 1;
/// Perf guard on best-case time-to-resume (restart→splice). Only trips on a real
/// catch-up regression (e.g. speed scaling breaking back toward 1×).
const MAX_RESUME_MS: f64 = 3000.0;
const RECORDING_REL: &str = "TAS/recordings/FE-10065.tasrec";

pub fn run() -> bool {
    // Resolve the recording relative to the workspace (tas_test runs from
    // TAS/target/release/).
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-10065.tasrec"),
    ];
    let path = match candidates.iter().find(|p| p.exists()) {
        Some(p) => p.to_string_lossy().into_owned(),
        None => {
            eprintln!(
                "ERROR: couldn't locate {} (tried {:?})",
                RECORDING_REL, candidates
            );
            return false;
        }
    };
    println!("  Using recording: {}", path);

    let mut all_ok = true;
    for &speed in SPEEDS {
        all_ok &= run_at(&path, speed);
    }
    println!();
    if all_ok {
        println!(
            "*** FE-10065 CONT PASSED at all speeds {:?}: zero-drift + frame-exact resume ***",
            SPEEDS
        );
    } else {
        println!("*** FE-10065 CONT FAILED (see per-speed lines above) ***");
    }
    all_ok
}

/// One catch-up speed: zero-drift + frame-exact (overshoot ≤ MAX_OVERSHOOT) +
/// perf bound. Returns whether all three held.
fn run_at(path: &str, speed: f32) -> bool {
    println!(
        "\n=== FE-10065 CONT (splice={} ×{} at {}× catch-up) ===",
        SPLICE_FRAME, ITERATIONS, speed
    );
    let report = cont_reliability::run(
        ITERATIONS,
        speed,
        SPLICE_FRAME,
        Some(path),
        cont_reliability::BaselineInputProfile::Taps,
        None,
    );

    let clean = report.all_pass();
    let max_overshoot = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.splice_recorded_count.saturating_sub(SPLICE_FRAME))
        .max()
        .unwrap_or(u32::MAX);
    let min_resume_ms = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.resume_ms)
        .fold(f64::INFINITY, f64::min);
    let timing_ok = max_overshoot <= MAX_OVERSHOOT;
    let perf_ok = min_resume_ms <= MAX_RESUME_MS;

    if clean && timing_ok && perf_ok {
        println!(
            "  {}× PASSED: zero-drift, resume within {} frame(s), best resume {:.0} ms",
            speed, max_overshoot, min_resume_ms
        );
    } else {
        if !clean {
            println!("  {}× FAILED: a splice drifted / didn't splice", speed);
        }
        if !timing_ok {
            println!(
                "  {}× FAILED: resume overshoot {} > {} (Problem B regression — fixed DLL deployed?)",
                speed, max_overshoot, MAX_OVERSHOOT
            );
        }
        if !perf_ok {
            println!(
                "  {}× FAILED: best time-to-resume {:.0} ms > {:.0} ms (catch-up perf regression)",
                speed, min_resume_ms, MAX_RESUME_MS
            );
        }
    }
    clean && timing_ok && perf_ok
}
