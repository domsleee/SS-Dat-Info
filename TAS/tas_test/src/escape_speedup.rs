//! Escape pause/resume speedup test.
//!
//! Catches the originally reported bug: pressing Escape pauses the game,
//! and pressing Escape again resumes — but with TAS caves loaded, the
//! game "fast-forwards" on resume to consume the accumulated wall-clock
//! time, producing a visible speedup that breaks any manual gameplay or
//! playback that crosses the pause boundary.
//!
//! WHICH COUNTER. Fast-forward means the game runs MORE PHYSICS TICKS PER
//! RENDER FRAME. So the counter that can see it is `tick_count` (cave5 adds
//! its per-frame tick count to it), not `frame_count` (cave2 bumps that once
//! per Supreme::Cycle, i.e. once per rendered frame). This test used to judge
//! on frame_count alone, which measures the render rate — a machine that
//! renders the pause menu at 163 fps instead of 100 failed it while
//! simulating perfectly correctly, and a genuine 20-second fast-forward would
//! have slipped through it unseen. Both are sampled now; the verdict is on
//! ticks.
//!
//! Test sequence:
//!   1. Game live, not in REC/PLAY mode.
//!   2. Press Escape → should pause: both counters stop incrementing.
//!   3. Hold pause for 20s (so a fast-forward would owe ~2000 ticks).
//!   4. Press Escape → should resume at the game's normal ~100 ticks/s.
//!   5. Hold resume for 5s, sampling every second.
//!
//! Pass criteria:
//!   * pause window: tick delta ≈ 0 (pause actually engaged);
//!   * resume window: ~500 ticks over 5s, not 2000+;
//!   * FIRST resume second contains no burst — that is where a drained
//!     backlog would land, and a whole-window total can hide a one-second
//!     spike inside an otherwise normal average.
//!
//! Note on simulated Escape: keybd_event with a hardware scancode is the
//! best we can do programmatically. If it fails to pause (e.g. focus
//! issues, the game ignoring synthetic input), the test reports that the
//! pause never engaged — which is itself useful information, and the
//! resume-phase check is moot since there was nothing to fast-forward
//! through.

use std::thread;
use std::time::Duration;

use crate::harness;

const PAUSE_DURATION_SECS: u64 = 20;
const RESUME_DURATION_SECS: u64 = 5;
/// Acceptable tick count during the pause window. With pause working this is
/// flat zero; 100 is generous slop for the moment either side of the keypress.
const MAX_PAUSE_TICKS: u32 = 100;
/// Resume window expectations: at 1x the game simulates ~100 ticks/s, so 5s is
/// ~500. Draining a 20s backlog would add up to ~2000 on top. The band is wide
/// enough to survive a slow frame or two and still nowhere near a catchup.
const RESUME_NORMAL_TICK_MIN: u32 = 300;
const RESUME_NORMAL_TICK_MAX: u32 = 900;
/// A backlog drain lands in the first second after resume. Normal is ~100. This
/// is only a secondary, absolute plausibility guard - the real check is
/// MAX_RATE_RATIO below, measured against this machine.
const MAX_FIRST_SECOND_TICKS: u32 = 250;
/// THE ACTUAL ASSERTION. Every one-second slice after the resume must stay
/// within this multiple of the pre-pause baseline.
///
/// Absolute bands alone are not enough, and the gap was real: an implementation
/// that drained the backlog by advancing the wrong accumulator left the sim
/// running 1.63x fast forever after a pause (baseline 100 ticks/s, resume 163),
/// and 163 ticks/s clears BOTH a 300..=900 five-second band (815) and a 250
/// first-second cap. Only a comparison against the baseline catches it. 1.30 sits
/// well clear of observed jitter (99..102 against a 100 baseline) and well under
/// the 1.63 it has to reject.
const MAX_RATE_RATIO: f64 = 1.30;
/// ...and the game must not come back stalled either.
const MIN_RATE_RATIO: f64 = 0.70;
/// Baseline sanity: below this the machine is not simulating and the ratio test
/// would be meaningless, so say so rather than silently passing everything.
const MIN_PLAUSIBLE_BASELINE_TPS: u32 = 40;
/// Baseline sampled before the pause, so the verdict can compare resume against
/// how this machine actually runs rather than against a hardcoded 100 ticks/s.
const BASELINE_SECS: u64 = 4;

/// One sample of both counters, taken as close together as we can manage.
#[derive(Clone, Copy)]
struct Sample {
    ticks: u32,
    frames: u32,
}

fn sample(client: &tas_shared::TasSharedMemoryClient) -> Sample {
    Sample {
        ticks: client.tick_count_volatile(),
        frames: client.frame_count_volatile(),
    }
}

pub fn run() -> bool {
    println!("=== Escape Pause/Resume Speedup Test ===\n");

    let client = harness::ensure_game_running();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Game not alive");
        return false;
    }

    let start = sample(&client);
    println!(
        "  Initial tick_count: {}   frame_count: {}",
        start.ticks, start.frames
    );

    // ---- Phase 0: baseline BEFORE any pause ----
    // Without this the test cannot tell "the pause left the game running fast"
    // from "the game was running at this rate the whole time" — and those want
    // completely different fixes.
    println!(
        "\n--- Phase 0: {}s baseline BEFORE pausing ---",
        BASELINE_SECS
    );
    let base_start = sample(&client);
    thread::sleep(Duration::from_secs(BASELINE_SECS));
    let base_end = sample(&client);
    let base_ticks = base_end.ticks.saturating_sub(base_start.ticks);
    let base_frames = base_end.frames.saturating_sub(base_start.frames);
    let base_tps = base_ticks / BASELINE_SECS as u32;
    println!(
        "  baseline: {} ticks/s, {} fps ({:.2} ticks per frame)",
        base_tps,
        base_frames / BASELINE_SECS as u32,
        if base_frames > 0 {
            base_ticks as f64 / base_frames as f64
        } else {
            0.0
        }
    );

    // ---- Phase 1: Pause ----
    println!("\n--- Phase 1: Press Escape (pause) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Could not send Escape");
        return false;
    }
    // Give the pause handler a moment to engage
    thread::sleep(Duration::from_millis(250));
    let at_pause = sample(&client);
    println!(
        "  after pause sent: tick_count={}  frame_count={}",
        at_pause.ticks, at_pause.frames
    );

    println!(
        "\n--- Phase 2: Hold pause for {}s, sampling every second ---",
        PAUSE_DURATION_SECS
    );
    let mut prev = at_pause;
    for second in 1..=PAUSE_DURATION_SECS {
        thread::sleep(Duration::from_secs(1));
        let now = sample(&client);
        println!(
            "  t={:>2}s  ticks={} (+{})   frames={} (+{})",
            second,
            now.ticks,
            now.ticks.saturating_sub(prev.ticks),
            now.frames,
            now.frames.saturating_sub(prev.frames)
        );
        prev = now;
    }
    let end_pause = sample(&client);
    let pause_ticks = end_pause.ticks.saturating_sub(at_pause.ticks);
    println!(
        "\n  Pause window total: {} ticks over {}s ({} ticks/s), {} render frames",
        pause_ticks,
        PAUSE_DURATION_SECS,
        pause_ticks / PAUSE_DURATION_SECS as u32,
        end_pause.frames.saturating_sub(at_pause.frames)
    );

    // ---- Phase 3: Resume ----
    println!("\n--- Phase 3: Press Escape (resume) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Could not send Escape for resume");
        return false;
    }
    let at_resume = sample(&client);

    println!(
        "\n--- Phase 4: Hold resume for {}s, sampling every second ---",
        RESUME_DURATION_SECS
    );
    let mut prev = at_resume;
    let mut first_second_ticks = 0u32;
    let mut max_second_ticks = 0u32;
    let mut min_second_ticks = u32::MAX;
    for second in 1..=RESUME_DURATION_SECS {
        thread::sleep(Duration::from_secs(1));
        let now = sample(&client);
        let dt = now.ticks.saturating_sub(prev.ticks);
        if second == 1 {
            first_second_ticks = dt;
        }
        if dt > max_second_ticks {
            max_second_ticks = dt;
        }
        if dt < min_second_ticks {
            min_second_ticks = dt;
        }
        println!(
            "  t={:>2}s  ticks={} (+{})   frames={} (+{})",
            second,
            now.ticks,
            dt,
            now.frames,
            now.frames.saturating_sub(prev.frames)
        );
        prev = now;
    }
    let end_resume = sample(&client);
    let resume_ticks = end_resume.ticks.saturating_sub(at_resume.ticks);
    let resume_frames = end_resume.frames.saturating_sub(at_resume.frames);
    println!(
        "\n  Resume window total: {} ticks over {}s ({} ticks/s), {} render frames ({} fps)",
        resume_ticks,
        RESUME_DURATION_SECS,
        resume_ticks / RESUME_DURATION_SECS as u32,
        resume_frames,
        resume_frames / RESUME_DURATION_SECS as u32
    );

    // ---- Phase 5: Verdict ----
    println!("\n=== TICKER VERDICT ===");
    let pause_ok = pause_ticks <= MAX_PAUSE_TICKS;
    let resume_ok = (RESUME_NORMAL_TICK_MIN..=RESUME_NORMAL_TICK_MAX).contains(&resume_ticks);
    let burst_ok = first_second_ticks <= MAX_FIRST_SECOND_TICKS;

    // The baseline-relative check, applied to EVERY one-second slice rather than
    // to the window total: a burst hides inside an otherwise normal average, and
    // Escape handling can push it out of the first slice into the second.
    let baseline_ok = base_tps >= MIN_PLAUSIBLE_BASELINE_TPS;
    let max_allowed = (base_tps as f64 * MAX_RATE_RATIO) as u32;
    let min_allowed = (base_tps as f64 * MIN_RATE_RATIO) as u32;
    let rate_ok = baseline_ok && max_second_ticks <= max_allowed && min_second_ticks >= min_allowed;

    println!(
        "  Pause window:  {} ticks (expected <= {}) — {}",
        pause_ticks,
        MAX_PAUSE_TICKS,
        if pause_ok {
            "PASS (game paused)"
        } else {
            "FAIL (game kept ticking — pause didn't engage)"
        }
    );
    println!(
        "  Resume window: {} ticks (expected {}..={}) — {}",
        resume_ticks,
        RESUME_NORMAL_TICK_MIN,
        RESUME_NORMAL_TICK_MAX,
        if resume_ok {
            "PASS (normal speed)"
        } else if resume_ticks > RESUME_NORMAL_TICK_MAX {
            "FAIL (catchup detected — game fast-forwarded after resume)"
        } else {
            "FAIL (ticks too low — game still paused?)"
        }
    );
    println!(
        "  First second:  {} ticks (expected <= {}) — {}",
        first_second_ticks,
        MAX_FIRST_SECOND_TICKS,
        if burst_ok {
            "PASS (no backlog burst)"
        } else {
            "FAIL (backlog drained INTO the sim — that is the speedup)"
        }
    );

    println!(
        "  Rate vs baseline: per-second {}..{} ticks against a {} baseline (allowed {}..{}) - {}",
        min_second_ticks,
        max_second_ticks,
        base_tps,
        min_allowed,
        max_allowed,
        if !baseline_ok {
            "INCONCLUSIVE (baseline too low to judge)"
        } else if max_second_ticks > max_allowed {
            "FAIL (game is simulating faster than before the pause)"
        } else if min_second_ticks < min_allowed {
            "FAIL (game is simulating slower than before the pause)"
        } else {
            "PASS (same rate as before the pause)"
        }
    );

    let pass = pause_ok && resume_ok && burst_ok && rate_ok;
    if pass {
        println!("\n*** ESCAPE SPEEDUP TEST PASSED: ticker makes sense across pause+resume ***");
    } else {
        println!("\n*** ESCAPE SPEEDUP TEST FAILED ***");
    }
    pass
}
