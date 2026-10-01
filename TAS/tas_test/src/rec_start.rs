//! Recording-start check: a recording must BEGIN at the stationary spawn,
//! capturing the countdown and pre-timer inputs. rec-repro applies it to its
//! fresh live captures; `rec-start --file` judges a saved `.tasrec`.
//!
//! The REC button does an in-process F5 restart and arms once the restart
//! state machine reports done; a settle that is too long arms mid-fall. The
//! drift and CONT suites cannot see that: a recording that starts mid-fall
//! is perfectly reproducible. The invariant here is relative (opening speed
//! vs mid-run speed), so it is independent of level, spawn position and units.

use crate::replay;

/// The opening of a recording must be at most this fraction of the mid-run
/// speed to count as "started at the spawn". A spawn start is ~stationary
/// (ratio ~0 during the countdown); a mid-fall start is already at speed
/// (ratio ~1).
const MAX_START_RATIO: f64 = 0.30;
/// Frames averaged at the head of the recording for the opening speed.
const HEAD_FRAMES: usize = 10;
/// A valid run must record at least this many frames and travel at least this
/// far, else the ratio is meaningless. MIN_FRAMES is above the ~183-frame
/// countdown prefix so the mid-run reference cannot sit inside it.
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

    // Mid-run speed: mean per-frame motion over the LAST third, which is past
    // any countdown (the middle third can overlap the stationary prefix).
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

    // First frame where the player is actually moving (end of the countdown):
    // the first moving faster than 10% of mid speed. The frames before it are
    // the stationary prefix.
    let eps = (mid_speed * 0.1).max(1e-3);
    let first_moving = (1..n).find(|&i| frame_speed(coords, i) >= eps).unwrap_or(n);
    let stationary_prefix = first_moving - 1;

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

/// Too short or too still to judge the start at all. (NaN travel counts.)
fn degenerate(a: &StartAnalysis) -> bool {
    a.count < MIN_FRAMES || a.travel.is_nan() || a.travel < MIN_TRAVEL
}

/// Why the recording is not a real run that begins at the stationary spawn;
/// empty when it is. The ratio alone is fooled by a late-countdown arm that
/// keeps a few stationary frames, so a real countdown prefix and real motion
/// after it are required too.
fn failures(a: &StartAnalysis) -> Vec<String> {
    let mut reasons = Vec::new();
    if degenerate(a) {
        reasons.push(format!(
            "degenerate recording (frames={}, path_len={:.2})",
            a.count, a.travel
        ));
    }
    if a.ratio.is_nan() || a.ratio >= MAX_START_RATIO {
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
    reasons
}

pub fn passes(a: &StartAnalysis) -> bool {
    failures(a).is_empty()
}

fn print_analysis(label: &str, a: &StartAnalysis) {
    println!(
        "  {label}: frames={} start_speed={:.5} mid_speed={:.5} ratio={:.3} \
         stationary_prefix={} first_moving={} path_len={:.2}",
        a.count, a.start_speed, a.mid_speed, a.ratio, a.stationary_prefix, a.first_moving, a.travel
    );
}

fn verdict(a: &StartAnalysis) -> bool {
    if degenerate(a) {
        println!(
            "*** REC-START INCONCLUSIVE: degenerate recording (frames={}, path_len={:.2}) — \
             need a real run to judge the start. ***",
            a.count, a.travel
        );
        return false;
    }
    let reasons = failures(a);
    if reasons.is_empty() {
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
    let a = analyze_start(
        &loaded.rec_coords[..count.min(loaded.rec_coords.len())],
        count,
    );
    print_analysis(path, &a);
    verdict(&a)
}
