//! FE-10065 CONT reliability + resume-timing regression.
//!
//! Pins the user's real case: load `TAS/recordings/FE-10065.tasrec`, CONT-splice
//! at frame 6200 with 64x catch-up, 12 iterations. Passes only if EVERY splice
//! is zero-drift AND the post-splice overshoot stays tiny.
//!
//! The overshoot assertion is the Problem B guard: it caught (and now locks in
//! the fix for) "resume lands a few frames past the splice". Without the fix
//! (atomic resume-speed drop at the splice + near-splice catch-up deceleration)
//! the resumed REC fast-forwards at the catch-up rate for a variable window
//! (rec_cnt was 6209..6322); with it, rec_cnt is 6200..6201.

use std::path::PathBuf;

use crate::cont_reliability;

const SPLICE_FRAME: u32 = 6200;
const ITERATIONS: u32 = 12;
const SPEED: f32 = 64.0;
/// Frame-exact resume: the recording must start at exactly the splice frame.
/// Pre-fix this was tens-to-hundreds; with the fix + a sub-tick splice poll it
/// is 0. Allow 1 only for rare OS-timer jitter on the detection poll.
const MAX_OVERSHOOT: u32 = 1;
/// Perf guard on time-to-resume (restart→splice). Clean 64x catch-up is
/// ~1.3-1.5s; this loose bound (best case over the run) only trips on a real
/// regression (e.g. the catch-up speed scaling breaking back to ~1x).
const MAX_RESUME_MS: f64 = 3000.0;
const RECORDING_REL: &str = "TAS/recordings/FE-10065.tasrec";

pub fn run() -> bool {
    println!(
        "=== FE-10065 CONT reliability + resume timing (splice={} ×{} at {}× catch-up) ===\n",
        SPLICE_FRAME, ITERATIONS, SPEED
    );

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
            eprintln!("ERROR: couldn't locate {} (tried {:?})", RECORDING_REL, candidates);
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

    let clean = report.all_pass();
    let max_overshoot = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.splice_recorded_count.saturating_sub(SPLICE_FRAME))
        .max()
        .unwrap_or(u32::MAX);
    let timing_ok = max_overshoot <= MAX_OVERSHOOT;

    // Best-case (clean, no-reroll) time to resume — the perf guard.
    let min_resume_ms = report
        .results
        .iter()
        .filter(|r| r.spliced)
        .map(|r| r.resume_ms)
        .fold(f64::INFINITY, f64::min);
    let perf_ok = min_resume_ms <= MAX_RESUME_MS;

    println!();
    if clean && timing_ok && perf_ok {
        println!(
            "*** FE-10065 CONT PASSED: {}/{} splices zero-drift, resume within {} frame(s) of splice, \
             best resume {:.0} ms ***",
            ITERATIONS, ITERATIONS, max_overshoot, min_resume_ms
        );
    } else {
        if !perf_ok {
            println!(
                "*** FE-10065 CONT FAILED: best time-to-resume {:.0} ms > {:.0} ms (catch-up perf \
                 regression) ***",
                min_resume_ms, MAX_RESUME_MS
            );
        }
        if !clean {
            println!("*** FE-10065 CONT FAILED: one or more splices drifted / didn't splice ***");
        }
        if !timing_ok {
            println!(
                "*** FE-10065 CONT FAILED: resume overshoot {} > {} frames (Problem B regression — \
                 is the fixed DLL deployed?) ***",
                max_overshoot, MAX_OVERSHOOT
            );
        }
    }
    clean && timing_ok && perf_ok
}
