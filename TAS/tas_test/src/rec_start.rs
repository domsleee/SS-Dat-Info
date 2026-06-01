//! Recording-start regression test (guards SSB "armed mid-run" bug).
//!
//! Background: the REC button does an in-process F5 restart and only arms
//! recording once the restart state machine reports "done". An uncommitted
//! restart rework added a long `RESTART_STABILIZE_FRAMES` settle, so "done"
//! landed ~6s after the F5 — well into the fall. The result: `rec[0]` is
//! already in motion, and the spawn / countdown / pre-timer inputs are
//! silently dropped from every recording.
//!
//! Why no existing test caught it: the drift and CONT suites assert
//! *reproducibility* (REC vs PLAY drift == 0, CONT anchor match). A recording
//! that starts mid-fall is perfectly reproducible — REC and PLAY both start
//! mid-fall — so those tests stay green. The missing invariant is *start
//! completeness*: a fresh recording must BEGIN at the stationary spawn, with
//! the player's opening motion much slower than its mid-run motion.
//!
//! This test encodes that invariant. It is intentionally relative (start speed
//! vs mid-run speed) so it is independent of level, spawn position, and units.

use crate::{harness, replay};
use std::thread;
use std::time::Duration;
use tas_shared::TAS_MAX_TICKS;

/// The opening of a recording must be at most this fraction of the mid-run
/// speed to count as "started at the spawn". A spawn start is ~stationary
/// (ratio ~0 during the countdown); a mid-fall start is already at speed
/// (ratio ~1).
const MAX_START_RATIO: f64 = 0.30;
/// Frames averaged at the head of the recording for the opening speed.
const HEAD_FRAMES: usize = 10;
/// A valid run must record at least this many frames and travel at least this
/// far, else the ratio is meaningless. MIN_FRAMES is above the known-good
/// ~183-frame countdown prefix so there is real motion to measure (codex: a
/// 120 floor let the mid-run reference sit inside the prefix).
const MIN_FRAMES: usize = 300;
const MIN_TRAVEL: f64 = 1.0;
/// The stationary countdown prefix that proves the spawn was captured. Floor is
/// well under the known-good prefix (~183) but far above a mid-fall arm (~0).
const MIN_PREFIX: usize = 60;
/// Real motion required after the countdown, so a late-countdown arm (a few
/// stationary frames then mid-fall) can't pass on the ratio alone.
const MIN_MOVING_FRAMES: usize = 120;

#[derive(Debug, Clone, Copy)]
pub struct StartAnalysis {
    pub count: usize,
    pub start_speed: f64,
    pub mid_speed: f64,
    pub ratio: f64,
    pub stationary_prefix: usize,
    pub first_moving: usize,
    pub travel: f64,
}

fn frame_speed(c: &[[f32; 3]], i: usize) -> f64 {
    let a = c[i - 1];
    let b = c[i];
    let dx = (b[0] - a[0]) as f64;
    let dy = (b[1] - a[1]) as f64;
    let dz = (b[2] - a[2]) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Compute opening-vs-run motion stats. `coords` must hold at least `count`
/// frames.
pub fn analyze_start(coords: &[[f32; 3]], count: usize) -> StartAnalysis {
    let n = count.min(coords.len());
    if n < 2 {
        return StartAnalysis {
            count: n,
            start_speed: 0.0,
            mid_speed: 0.0,
            ratio: 1.0,
            stationary_prefix: 0,
            first_moving: n,
            travel: 0.0,
        };
    }

    // Opening speed: mean per-frame motion over the first HEAD_FRAMES.
    let head = HEAD_FRAMES.min(n - 1);
    let mut start_sum = 0.0;
    for i in 1..=head {
        start_sum += frame_speed(coords, i);
    }
    let start_speed = start_sum / head as f64;

    // Mid-run speed: mean per-frame motion over the LAST third. The last third
    // is past any countdown, so it reflects real run speed (codex: the middle
    // third can overlap a ~183-frame stationary prefix and dilute the reference).
    let lo = (2 * n / 3).max(1);
    let mut mid_sum = 0.0;
    let mut midc = 0usize;
    for i in lo..n {
        mid_sum += frame_speed(coords, i);
        midc += 1;
    }
    let mid_speed = if midc > 0 { mid_sum / midc as f64 } else { 0.0 };

    let ratio = if mid_speed > 1e-9 {
        start_speed / mid_speed
    } else {
        1.0
    };

    // Stationary prefix: leading frames moving slower than 10% of mid speed.
    let eps = (mid_speed * 0.1).max(1e-3);
    let mut stationary_prefix = 0usize;
    for i in 1..n {
        if frame_speed(coords, i) < eps {
            stationary_prefix += 1;
        } else {
            break;
        }
    }
    // First frame where the player is actually moving (end of the countdown).
    let first_moving = (1..n).find(|&i| frame_speed(coords, i) >= eps).unwrap_or(n);

    // Distance traveled = cumulative XYZ path length. Robust to a run that
    // loops back near its origin (endpoint displacement would look degenerate).
    let mut travel = 0.0;
    for i in 1..n {
        travel += frame_speed(coords, i);
    }

    StartAnalysis {
        count: n,
        start_speed,
        mid_speed,
        ratio,
        stationary_prefix,
        first_moving,
        travel,
    }
}

/// Invariant check: a real run that begins at the stationary spawn. All four
/// gates must hold — the ratio alone is fooled by a late-countdown arm that
/// keeps a few stationary frames, so we also require a real countdown prefix
/// and real motion after it.
pub fn passes(a: &StartAnalysis) -> bool {
    a.count >= MIN_FRAMES
        && a.travel >= MIN_TRAVEL
        && a.ratio < MAX_START_RATIO
        && a.stationary_prefix >= MIN_PREFIX
        && a.count.saturating_sub(a.first_moving) >= MIN_MOVING_FRAMES
}

fn print_analysis(label: &str, a: &StartAnalysis) {
    println!(
        "  {label}: frames={} start_speed={:.5} mid_speed={:.5} ratio={:.3} \
         stationary_prefix={} first_moving={} path_len={:.2}",
        a.count, a.start_speed, a.mid_speed, a.ratio, a.stationary_prefix, a.first_moving, a.travel
    );
}

fn verdict(a: &StartAnalysis) -> bool {
    if a.count < MIN_FRAMES || a.travel < MIN_TRAVEL {
        println!(
            "*** REC-START INCONCLUSIVE: degenerate recording (frames={}, path_len={:.2}) — \
             need a real run to judge the start. ***",
            a.count, a.travel
        );
        return false;
    }
    if passes(a) {
        println!(
            "*** REC-START OK: recording begins at the spawn (opening is {:.1}% of run speed, \
             stationary_prefix={} frames, {} moving frames after the countdown) ***",
            a.ratio * 100.0,
            a.stationary_prefix,
            a.count.saturating_sub(a.first_moving)
        );
        return true;
    }
    // Spell out which gate(s) failed so a regression is diagnosable, not just red.
    let mut reasons = Vec::new();
    if a.ratio >= MAX_START_RATIO {
        reasons.push(format!(
            "opening is {:.1}% of run speed (>= {:.0}% — rec[0] already moving)",
            a.ratio * 100.0,
            MAX_START_RATIO * 100.0
        ));
    }
    if a.stationary_prefix < MIN_PREFIX {
        reasons.push(format!(
            "stationary_prefix={} (< {} — countdown not captured, armed late)",
            a.stationary_prefix, MIN_PREFIX
        ));
    }
    if a.count.saturating_sub(a.first_moving) < MIN_MOVING_FRAMES {
        reasons.push(format!(
            "only {} moving frames after the countdown (< {})",
            a.count.saturating_sub(a.first_moving),
            MIN_MOVING_FRAMES
        ));
    }
    println!(
        "*** REC-START FAILED: spawn / countdown / pre-timer inputs NOT captured — {} ***",
        reasons.join("; ")
    );
    false
}

/// Analyze a saved `.tasrec` (e.g. a known-good recording) without touching
/// the live game. Used to ground the invariant.
pub fn check_file(path: &str) -> bool {
    println!("=== REC-START check (file): {path} ===");
    let loaded = match replay::load_tasrec(std::path::Path::new(path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: failed to load {path}: {e}");
            std::process::exit(2);
        }
    };
    let count = loaded.count as usize;
    let a = analyze_start(&loaded.rec_coords[..count.min(loaded.rec_coords.len())], count);
    print_analysis(path, &a);
    verdict(&a)
}

/// Record a fresh run from a restart and assert it begins at the spawn.
/// Neutral input is fine — gravity provides the run; we only care that the
/// opening is (near-)stationary relative to the fall.
pub fn run() -> bool {
    println!("=== REC-START regression test (live) ===");
    let mut client = harness::ensure_game_running();

    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: game not alive for restart");
        return false;
    }
    harness::focus_game();
    harness::arm_rec(&mut client);
    // 6s, not 4s: the countdown prefix (~183 live frames) is fixed regardless of
    // REC length, so a longer capture just adds moving frames — keeping total
    // frames comfortably over MIN_FRAMES even on a slow env tick (avoids a
    // spurious INCONCLUSIVE). Neutral input is fine; gravity drives the run.
    println!("  Recording ~6s from restart (neutral input; gravity drives the run)...");
    thread::sleep(Duration::from_secs(6));
    let count = client.state().recorded_count as usize;
    harness::stop(&mut client);

    let n = count.min(TAS_MAX_TICKS);
    let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
    let a = analyze_start(&coords, n);
    print_analysis("<live REC>", &a);
    verdict(&a)
}
