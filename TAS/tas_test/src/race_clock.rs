//! The game's own race clock (DESIGN.md "Race time"). Replaying a run that
//! crosses the finish, at 1x and at a fast-forward speed, the clock must
//! start, never run backward, and finish on a sum of 0.01f steps. A restart
//! must reset it.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::race_clock::{extended_precision, hud_cs, race_clock, ticks};
use tas_shared::{TasMode, TasSharedMemoryClient};

use crate::harness::{self, cleanup};

/// A run that crosses the finish line (dialog-e2e's fixture).
const FIXTURE: &str = "FE-decent-done.tasrec";

const SPEEDS: [f32; 2] = [1.0, 8.0];
const MIN_SAMPLES: u32 = 100;

pub fn run() -> bool {
    println!("=== RACE-CLOCK: the game's race clock runs from start to finish ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let mut ok = true;
    for speed in SPEEDS {
        match replay_and_check(&mut client, speed) {
            Ok(summary) => println!("  {speed}x: {summary}"),
            Err(e) => {
                eprintln!("  {speed}x: FAILED: {e}");
                ok = false;
            }
        }
    }
    cleanup(&mut client);
    if ok {
        println!("\n*** RACE-CLOCK PASSED ***");
    } else {
        println!("\n*** RACE-CLOCK FAILED ***");
    }
    ok
}

fn replay_and_check(client: &mut TasSharedMemoryClient, speed: f32) -> Result<String, String> {
    harness::stop(client);
    harness::load_fixture(client, FIXTURE)?;
    client.state_mut().playback_speed = speed;
    harness::restart_play_aligned_unwatched(client)?;
    // The gate is the release; the start line comes after it.
    if let Some(clock) = race_clock(client.state()) {
        if clock.finished || clock.seconds > 0.5 {
            return Err(format!("the restart left the clock at {clock:?}"));
        }
    }
    let extended = extended_precision(client.state().fpu_control_word);
    let mut last: Option<f32> = None;
    let (mut samples, mut backward) = (0u32, 0u32);
    let mut first_backward = String::new();
    let mut finish: Option<f32> = None;
    let mut started_seen = false;
    let mut finished_at: Option<Instant> = None;
    let deadline = Instant::now() + Duration::from_secs(240);
    // The finish line freezes the engine at the post-race prompt, so stop
    // watching a second after the clock finishes.
    while client.mode_volatile() == TasMode::Play as u32
        && Instant::now() < deadline
        && finished_at.is_none_or(|t| t.elapsed() < Duration::from_secs(1))
    {
        if let Some(clock) = race_clock(client.state()) {
            started_seen |= clock.started;
            if clock.started && last != Some(clock.seconds) {
                if let Some(prev) = last.filter(|&prev| clock.seconds < prev) {
                    backward += 1;
                    if first_backward.is_empty() {
                        first_backward = format!("{prev} s -> {} s", clock.seconds);
                    }
                }
                last = Some(clock.seconds);
                samples += 1;
            }
            if clock.finished && finish.is_none() {
                finish = Some(clock.seconds);
                finished_at = Some(Instant::now());
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    harness::stop(client);
    client.state_mut().playback_speed = 1.0;
    if finish.is_some() && !harness::dismiss_finish_prompt() {
        return Err("could not answer the post-race prompt".into());
    }

    if !started_seen {
        return Err("the clock never started".into());
    }
    let finish = finish.ok_or("the clock never finished (did the replay reach the line?)")?;
    let finish_ticks = ticks(finish, 70_000).ok_or(format!(
        "the finish time {finish} s is not a sum of 0.01f steps"
    ))?;
    if backward > 0 {
        return Err(format!(
            "the clock ran backward {backward} time(s); first: {first_backward}"
        ));
    }
    if samples < MIN_SAMPLES {
        return Err(format!("only {samples} clock samples"));
    }
    Ok(format!(
        "{samples} clock samples, none backward; finish {finish} s = {finish_ticks} ticks = {} cs on the HUD ({} precision)",
        hud_cs(finish, extended),
        if extended { "53-bit" } else { "24-bit" }
    ))
}
