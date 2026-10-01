//! The game's own race clock against the HUD scraper (DESIGN.md "Race
//! time"). The DLL pairs every HUD player-line time with the clock read in
//! the same call, so the two are compared at one observation point: the HUD
//! formula applied to the clock must give the scraped time every time,
//! through the finish, at 1x and at a fast-forward speed. A restart must
//! reset the clock.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::race_clock::{extended_precision, hud_cs, race_ab, race_clock, ticks};
use tas_shared::{TasMode, TasSharedMemoryClient};

use crate::command_edges::cleanup;
use crate::{harness, replay};

/// A run that crosses the finish line (dialog-e2e's fixture).
const FIXTURE: &str = "FE-decent-done.tasrec";

const SPEEDS: [f32; 2] = [1.0, 8.0];
const MIN_SAMPLES: u32 = 100;

pub fn run() -> bool {
    println!("=== RACE-CLOCK: the game's race clock matches the HUD at every sample ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let mut ok = true;
    for speed in SPEEDS {
        match replay_and_compare(&mut client, speed) {
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

fn replay_and_compare(client: &mut TasSharedMemoryClient, speed: f32) -> Result<String, String> {
    harness::stop(client);
    let path = harness::fixture_path(FIXTURE)?;
    let loaded = replay::load_tasrec(&path).map_err(|e| format!("loading {FIXTURE}: {e}"))?;
    replay::write_to_shared(client, &loaded);
    client.state_mut().playback_speed = speed;
    harness::restart_play_aligned_unwatched(client)?;
    // The gate is the release; the start line comes after it.
    if let Some(clock) = race_clock(client.state()) {
        if clock.finished || clock.seconds > 0.5 {
            return Err(format!("the restart left the clock at {clock:?}"));
        }
    }
    let extended = extended_precision(client.state().fpu_control_word);
    let mut last = None;
    let (mut samples, mut mismatches) = (0u32, 0u32);
    let mut first_mismatch = String::new();
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
        let state = client.state();
        if let Some(pair) = race_ab(state) {
            if Some(pair) != last {
                last = Some(pair);
                samples += 1;
                let (cs, seconds) = pair;
                let expected = hud_cs(seconds, extended);
                if expected != cs {
                    mismatches += 1;
                    if first_mismatch.is_empty() {
                        first_mismatch =
                            format!("HUD {cs} cs vs clock {seconds} s -> {expected} cs");
                    }
                }
            }
        }
        if let Some(clock) = race_clock(state) {
            started_seen |= clock.started;
            if clock.finished && finish.is_none() {
                finish = Some(clock.seconds);
                finished_at = Some(Instant::now());
            }
        }
        thread::sleep(Duration::from_millis(1));
    }
    // The finish HUD line keeps being drawn after the replay ends.
    thread::sleep(Duration::from_millis(300));
    let final_pair = race_ab(client.state());
    harness::stop(client);
    client.state_mut().playback_speed = 1.0;
    if finish.is_some() && !harness::dismiss_finish_prompt() {
        return Err("could not answer the post-race prompt".into());
    }
    println!(
        "     {samples} HUD samples, {mismatches} differ{}",
        if first_mismatch.is_empty() {
            String::new()
        } else {
            format!(" (first: {first_mismatch})")
        }
    );

    if !started_seen {
        return Err("the clock never started".into());
    }
    let finish = finish.ok_or("the clock never finished (did the replay reach the line?)")?;
    let finish_ticks = ticks(finish, 70_000).ok_or(format!(
        "the finish time {finish} s is not a sum of 0.01f steps"
    ))?;
    let finish_cs = hud_cs(finish, extended);
    if let Some((cs, seconds)) = final_pair {
        if seconds.to_bits() == finish.to_bits() && cs != finish_cs {
            return Err(format!(
                "the finish reads {cs} cs on the HUD but {finish_cs} cs from the clock"
            ));
        }
    }
    if mismatches > 0 {
        return Err(format!(
            "{mismatches}/{samples} samples differ; first: {first_mismatch}"
        ));
    }
    if samples < MIN_SAMPLES {
        return Err(format!("only {samples} HUD samples"));
    }
    Ok(format!(
        "{samples} HUD samples all equal the clock; finish {finish} s = {finish_ticks} ticks = {finish_cs} cs on the HUD ({} precision)",
        if extended { "53-bit" } else { "24-bit" }
    ))
}
