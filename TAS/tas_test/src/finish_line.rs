//! The finish line from the game's own Finish_Point (DESIGN.md "Race time").
//! The DLL hooks the race timer's finish, right after the game decides the
//! finish counts, and publishes the tick for the human rider only.
//!
//! PLAY: a take that finishes, at 1x and 8x. The finish must land on the same
//! tick from the gate at both speeds, count as valid, carry the race clock's final time, and
//! agree with FE's finish plane in the replay's own positions (captured at
//! Cycle entry, so the rider is past the plane one tick after the finish).
//! REC: a CONT spliced shortly before the line records through it; the finish
//! is reported in REC, after the splice.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::race_clock::{race_clock, race_finish, RaceFinish};
use tas_shared::{TasMode, TasSharedMemoryClient};

use crate::command_edges::cleanup;
use crate::{harness, replay};

const FIXTURE: &str = "FE-decent-done.tasrec";
/// FE's Finish_Point (levelData.json): the rider crosses when z rises past it
/// within the plane's half-width of x.
const FE_FINISH: [f32; 3] = [534.888, -284.278, 2_313.64];
const PLANE_RADIUS: f32 = 90.5;

pub fn run() -> bool {
    println!("=== FINISH-LINE: the finish comes from the game's Finish_Point ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = check(&mut client);
    cleanup(&mut client);
    match result {
        Ok(()) => {
            println!("\n*** FINISH-LINE PASSED ***");
            true
        }
        Err(e) => {
            eprintln!("\n*** FINISH-LINE FAILED: {e} ***");
            false
        }
    }
}

fn check(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    let slow = play_to_finish(client, 1.0)?;
    let fast = play_to_finish(client, 8.0)?;
    // Playback indices count from the arm, which lands a tick or two apart
    // run to run; from the gate the finish must be the same tick.
    let from_gate = |r: &(RaceFinish, u32, u32)| r.0.tick as i64 - r.1 as i64;
    if from_gate(&slow) != from_gate(&fast) || slow.0.seconds.to_bits() != fast.0.seconds.to_bits()
    {
        return Err(format!(
            "the finish moved with speed: 1x {:?} (gate {}) vs 8x {:?} (gate {})",
            slow.0, slow.1, fast.0, fast.1
        ));
    }
    println!(
        "  gate-relative finish at both speeds: gate+{}",
        from_gate(&slow)
    );
    rec_through_finish(client, slow.0.tick - slow.1 + slow.2)
}

fn load(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    harness::stop(client);
    let path = harness::fixture_path(FIXTURE)?;
    let loaded = replay::load_tasrec(&path).map_err(|e| format!("loading {FIXTURE}: {e}"))?;
    replay::write_to_shared(client, &loaded);
    Ok(())
}

fn seq(client: &TasSharedMemoryClient) -> u32 {
    race_finish(client.state()).map_or(0, |f| f.seq)
}

/// Wait for a finish newer than `before`, in `mode`.
fn wait_finish(
    client: &TasSharedMemoryClient,
    before: u32,
    mode: TasMode,
) -> Result<RaceFinish, String> {
    let deadline = Instant::now() + Duration::from_secs(240);
    loop {
        if let Some(f) = race_finish(client.state()) {
            if f.seq != before {
                if f.mode != mode as u32 {
                    return Err(format!("the finish came in mode {} ({f:?})", f.mode));
                }
                return Ok(f);
            }
        }
        if Instant::now() > deadline || client.mode_volatile() != mode as u32 {
            return Err(format!("no finish in {mode:?}"));
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Returns the finish, the live gate and the recording's gate.
fn play_to_finish(
    client: &mut TasSharedMemoryClient,
    speed: f32,
) -> Result<(RaceFinish, u32, u32), String> {
    load(client)?;
    client.state_mut().playback_speed = speed;
    let before = seq(client);
    let (rec_gate, live_gate) = harness::restart_play_aligned_unwatched(client)?;
    let finish = wait_finish(client, before, TasMode::Play)?;
    thread::sleep(Duration::from_millis(500));
    let clock = race_clock(client.state());
    let plane_tick = crossing_tick(&client.state().play_coords, finish.tick);
    harness::stop(client);
    client.state_mut().playback_speed = 1.0;
    if !harness::dismiss_finish_prompt() {
        return Err("could not answer the post-race prompt".into());
    }
    if !finish.valid {
        return Err(format!("{speed}x: the replay's finish missed a checkpoint"));
    }
    match clock {
        Some(c) if c.finished && c.seconds.to_bits() == finish.seconds.to_bits() => {}
        other => {
            return Err(format!(
                "{speed}x: the finish time {} s is not the race clock's ({other:?})",
                finish.seconds
            ))
        }
    }
    if plane_tick != Some(finish.tick + 1) {
        return Err(format!(
            "{speed}x: finish at tick {} but the positions pass FE's finish plane at {plane_tick:?}",
            finish.tick
        ));
    }
    println!(
        "  {speed}x PLAY: finished at tick {} (valid, {} s = the race clock), past the plane at tick {}",
        finish.tick,
        finish.seconds,
        finish.tick + 1
    );
    Ok((finish, live_gate, rec_gate))
}

/// First tick, searching from 200 before `near`, whose position is past FE's
/// finish plane.
fn crossing_tick(coords: &[[f32; 3]], near: u32) -> Option<u32> {
    let from = near.saturating_sub(200) as usize;
    (from.max(1)..coords.len().min(near as usize + 200)).find_map(|i| {
        let (prev, cur) = (coords[i - 1], coords[i]);
        (prev[2] < FE_FINISH[2]
            && cur[2] >= FE_FINISH[2]
            && (cur[0] - FE_FINISH[0]).abs() <= PLANE_RADIUS)
            .then_some(i as u32)
    })
}

/// CONT spliced 150 ticks before the recording's finish, then REC with no
/// input over the line.
fn rec_through_finish(
    client: &mut TasSharedMemoryClient,
    recorded_finish: u32,
) -> Result<(), String> {
    load(client)?;
    let splice = recorded_finish.saturating_sub(150);
    client.state_mut().playback_speed = 8.0;
    client.state_mut().cont_resume_speed = 1.0;
    let before = seq(client);
    harness::restart_continue_and_splice_inprocess(client, splice, 0)
        .ok_or("the CONT did not splice")?;
    let finish = wait_finish(client, before, TasMode::Rec)?;
    thread::sleep(Duration::from_millis(300));
    harness::stop(client);
    if !harness::dismiss_finish_prompt() {
        return Err("could not answer the post-race prompt".into());
    }
    if finish.tick <= splice {
        return Err(format!(
            "REC finish at {} is not after the splice at {splice}",
            finish.tick
        ));
    }
    println!(
        "  REC: finished at recorded tick {} after a splice at {splice} (valid={}, {} s)",
        finish.tick, finish.valid, finish.seconds
    );
    Ok(())
}
