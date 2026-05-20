//! Escape pause/resume speedup test.
//!
//! Catches the originally reported bug: pressing Escape pauses the game,
//! and pressing Escape again resumes — but with TAS caves loaded, the
//! game "fast-forwards" on resume to consume the accumulated wall-clock
//! time, producing a visible speedup that breaks any manual gameplay or
//! playback that crosses the pause boundary.
//!
//! Test sequence:
//!   1. Game live, not in REC/PLAY mode.
//!   2. Press Escape → should pause: cave2's frame_count stops incrementing.
//!   3. Hold pause for 20s.
//!   4. Press Escape → should resume: frame_count should increment at the
//!      game's normal rate (~100/s at 1x speed). NOT at a fast-forward
//!      rate that drains the 20-second backlog.
//!   5. Hold resume for 5s, sampling frame_count.
//!
//! Pass criterion: the ticker "makes sense" — during the pause window,
//! frame_count delta stays near zero; during the resume window, frame_count
//! delta is close to the expected ~500 ticks (5s × ~100tps), not 2000+
//! (which would indicate fast-forward catchup).
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
/// Acceptable tick rate during the pause window. With pause working,
/// frame_count should be essentially flat (<10 ticks total over 20s).
/// We allow up to 100 to be generous about timing slop.
const MAX_PAUSE_TICKS: u32 = 100;
/// Resume window expectations: at 1x speed the game runs ~100 ticks/s.
/// Over 5s that's ~500 ticks. Fast-forward catchup would produce many
/// thousands. We accept anything below 1.5× normal as "no catchup".
const RESUME_NORMAL_TICK_MIN: u32 = 200;
const RESUME_NORMAL_TICK_MAX: u32 = 800;

pub fn run() -> bool {
    println!("=== Escape Pause/Resume Speedup Test ===\n");

    let client = harness::ensure_game_running();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Game not alive");
        return false;
    }

    let fc_start = client.frame_count_volatile();
    println!("  Initial frame_count: {}", fc_start);

    // ---- Phase 1: Pause ----
    println!("\n--- Phase 1: Press Escape (pause) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Could not send Escape");
        return false;
    }
    // Give the pause handler a moment to engage
    thread::sleep(Duration::from_millis(250));
    let fc_at_pause = client.frame_count_volatile();
    println!("  frame_count after pause sent: {}", fc_at_pause);

    println!(
        "\n--- Phase 2: Hold pause for {}s, sampling every second ---",
        PAUSE_DURATION_SECS
    );
    let mut prev = fc_at_pause;
    for second in 1..=PAUSE_DURATION_SECS {
        thread::sleep(Duration::from_secs(1));
        let now = client.frame_count_volatile();
        println!(
            "  t={:>2}s  frame_count={}  delta_this_sec={}",
            second,
            now,
            now.saturating_sub(prev)
        );
        prev = now;
    }
    let fc_end_pause = client.frame_count_volatile();
    let pause_ticks = fc_end_pause.saturating_sub(fc_at_pause);
    println!(
        "\n  Pause window total: {} ticks over {}s ({} ticks/s)",
        pause_ticks,
        PAUSE_DURATION_SECS,
        pause_ticks / PAUSE_DURATION_SECS as u32
    );

    // ---- Phase 2: Resume ----
    println!("\n--- Phase 3: Press Escape (resume) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Could not send Escape for resume");
        return false;
    }
    let fc_at_resume = client.frame_count_volatile();

    println!(
        "\n--- Phase 4: Hold resume for {}s, sampling every second ---",
        RESUME_DURATION_SECS
    );
    let mut prev = fc_at_resume;
    for second in 1..=RESUME_DURATION_SECS {
        thread::sleep(Duration::from_secs(1));
        let now = client.frame_count_volatile();
        println!(
            "  t={:>2}s  frame_count={}  delta_this_sec={}",
            second,
            now,
            now.saturating_sub(prev)
        );
        prev = now;
    }
    let fc_end_resume = client.frame_count_volatile();
    let resume_ticks = fc_end_resume.saturating_sub(fc_at_resume);
    println!(
        "\n  Resume window total: {} ticks over {}s ({} ticks/s)",
        resume_ticks,
        RESUME_DURATION_SECS,
        resume_ticks / RESUME_DURATION_SECS as u32
    );

    // ---- Phase 5: Verdict ----
    println!("\n=== TICKER VERDICT ===");
    let pause_ok = pause_ticks <= MAX_PAUSE_TICKS;
    let resume_ok = (RESUME_NORMAL_TICK_MIN..=RESUME_NORMAL_TICK_MAX).contains(&resume_ticks);

    println!(
        "  Pause window:  {} ticks (expected <= {}) — {}",
        pause_ticks,
        MAX_PAUSE_TICKS,
        if pause_ok {
            "PASS (game paused)"
        } else {
            "FAIL (game kept running — pause didn't engage)"
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

    let pass = pause_ok && resume_ok;
    if pass {
        println!("\n*** ESCAPE SPEEDUP TEST PASSED: ticker makes sense across pause+resume ***");
    } else {
        println!("\n*** ESCAPE SPEEDUP TEST FAILED ***");
    }
    pass
}
