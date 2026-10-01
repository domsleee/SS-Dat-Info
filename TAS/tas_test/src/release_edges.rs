//! Releasing keys and real key-ups around a replay (DESIGN.md "REC observes,
//! PLAY writes").
//!
//! release-pending: an injected (pre-v56) take whose last ticks leave a press
//! queued behind others (the observer applies one event per key per tick and
//! the event after an applied one waits), ending on a neutral mask: RIGHT,
//! LEFT, RIGHT, neutral leaves RIGHT's press queued. When PLAY ends, nothing
//! may stay held, in the observer or the key buffer; the same for a held
//! take. The tails come from a brute-force search of the observer's rules.
//!
//! release-on-arm: the same pending press, with an arm (a refused CONT, or a
//! REC over the replay) landing on the tick after the neutral mask, while the
//! press is still queued: the arm's release must cover it too.
//!
//! play-enter-spam: Enter tapped on the real keyboard throughout a replay of
//! an injected take. Its presses are blocked and its releases pass; they must
//! not reach the observer's queue, where one event per tick would push the
//! replay's own events later. The replay must stay bit for bit.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{
    TasCommand, TasMode, TasSharedMemoryClient, TAS_INPUT_MODEL_HELD, TAS_INPUT_MODEL_INJECTED,
};

use crate::command_edges::{cleanup, load_fixture};
use crate::gamemem::GameMemory;
use crate::harness;

const TAILS: [&[u8]; 2] = [
    &[0, 0, 0, 0x02, 0x01, 0x02, 0x00],
    &[0, 0, 0, 0x03, 0x00, 0x01, 0x00],
];
const LENGTH: u32 = 1200;

pub fn run_pending() -> bool {
    println!(
        "=== RELEASE-PENDING: nothing stays held when a replay ends with queued presses ===\n"
    );
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = GameMemory::attach()
        .ok_or_else(|| "cannot read the game's memory".to_string())
        .and_then(|memory| {
            for tail in TAILS {
                for model in [TAS_INPUT_MODEL_INJECTED, TAS_INPUT_MODEL_HELD] {
                    play_tail(&mut client, &memory, model, tail)?;
                }
            }
            Ok(())
        });
    cleanup(&mut client);
    report("RELEASE-PENDING", result)
}

fn play_tail(
    client: &mut TasSharedMemoryClient,
    memory: &GameMemory,
    model: u32,
    tail: &[u8],
) -> Result<(), String> {
    harness::stop(client);
    load_fixture(client)?;
    {
        let s = client.state_mut();
        let start = LENGTH as usize - tail.len();
        s.input_log[start..LENGTH as usize].copy_from_slice(tail);
        s.recorded_count = LENGTH;
        s.input_model = model;
        s.playback_speed = 4.0;
    }
    if !harness::restart_and_stabilize_inprocess(client) {
        return Err("restart failed".into());
    }
    harness::arm_play(client);
    let deadline = Instant::now() + Duration::from_secs(60);
    while client.mode_volatile() != TasMode::Off as u32 {
        if Instant::now() > deadline {
            return Err("the replay did not end".into());
        }
        thread::sleep(Duration::from_millis(5));
    }
    client.state_mut().playback_speed = 1.0;
    thread::sleep(Duration::from_millis(300));
    let observer = memory.observer_tas_mask()?;
    let buffer = memory.held_tas_mask()?;
    if observer != 0 || buffer != 0 {
        return Err(format!(
            "model {model}, tail {tail:?}: after the replay ended the observer holds {observer:#04x} and the key buffer {buffer:#04x}"
        ));
    }
    println!("  model {model}, tail {tail:?}: the replay ended with nothing held");
    Ok(())
}

pub fn run_on_arm() -> bool {
    println!(
        "=== RELEASE-ON-ARM: an arm on the tick a press is still queued leaves nothing held ===\n"
    );
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = GameMemory::attach()
        .ok_or_else(|| "cannot read the game's memory".to_string())
        .and_then(|memory| {
            for arm in [TasCommand::ArmContinue, TasCommand::ArmRec] {
                arm_on_pending(&mut client, &memory, arm)?;
            }
            Ok(())
        });
    cleanup(&mut client);
    report("RELEASE-ON-ARM", result)
}

/// RIGHT, LEFT, RIGHT, neutral at ticks T..T+3 leaves RIGHT's press queued
/// after the Update of tick T+4; the arm is processed at that tick's cycle
/// cave entry. The replay slows to 0.05x (a tick per 200 ms) around it so the
/// command lands in that tick.
fn arm_on_pending(
    client: &mut TasSharedMemoryClient,
    memory: &GameMemory,
    arm: TasCommand,
) -> Result<(), String> {
    const T: u32 = 900;
    harness::stop(client);
    load_fixture(client)?;
    {
        let s = client.state_mut();
        for i in T as usize - 20..LENGTH as usize {
            s.input_log[i] = 0;
        }
        s.input_log[T as usize..T as usize + 4].copy_from_slice(&[0x02, 0x01, 0x02, 0x00]);
        s.recorded_count = LENGTH;
        s.input_model = TAS_INPUT_MODEL_INJECTED;
        s.continue_from_frame = 100;
        s.playback_speed = 4.0;
    }
    if !harness::restart_and_stabilize_inprocess(client) {
        return Err("restart failed".into());
    }
    harness::arm_play(client);
    let deadline = Instant::now() + Duration::from_secs(60);
    let wait_pos = |client: &TasSharedMemoryClient, pos: u32| -> Result<(), String> {
        while client.playback_pos_volatile() < pos {
            if Instant::now() > deadline || client.mode_volatile() != TasMode::Play as u32 {
                return Err(format!("the replay never reached tick {pos}"));
            }
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    };
    wait_pos(client, T - 30)?;
    client.state_mut().playback_speed = 0.05;
    wait_pos(client, T + 4)?;
    let at = client.playback_pos_volatile();
    client.send_command(arm);
    thread::sleep(Duration::from_millis(1000));
    // Before any STOP: its own release would hide what the arm left.
    let observer = memory.observer_tas_mask()?;
    let buffer = memory.held_tas_mask()?;
    harness::stop(client);
    client.state_mut().playback_speed = 1.0;
    thread::sleep(Duration::from_millis(300));
    if at != T + 4 {
        return Err(format!("{arm:?}: sent at tick {at}, not {}; rerun", T + 4));
    }
    if observer != 0 || buffer != 0 {
        return Err(format!(
            "{arm:?} on the pending tick: the observer holds {observer:#04x}, the key buffer {buffer:#04x}"
        ));
    }
    println!("  {arm:?} on the tick RIGHT's press was queued: nothing held afterwards");
    Ok(())
}

pub fn run_enter_spam() -> bool {
    println!("=== PLAY-ENTER-SPAM: real Enter taps leave an injected replay bit for bit ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = enter_spam(&mut client);
    cleanup(&mut client);
    report("PLAY-ENTER-SPAM", result)
}

fn enter_spam(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    harness::stop(client);
    load_fixture(client)?;
    let count = client.state().recorded_count.min(2000);
    client.state_mut().recorded_count = count;
    client.state_mut().input_model = TAS_INPUT_MODEL_INJECTED;
    client.state_mut().playback_speed = 1.0;
    let rec: Vec<[f32; 3]> = client.state().rec_coords[..count as usize].to_vec();
    harness::focus_game();
    let (rec_gate, live_gate) = harness::restart_play_aligned_unwatched(client)?;
    // Tap Enter every 70 ms while the replay runs.
    let running = Arc::new(AtomicBool::new(true));
    let flag = running.clone();
    let tapper = thread::spawn(move || -> Result<u32, String> {
        let mut keys = harness::PicoKeys::open_checked()?;
        let mut taps = 0;
        while flag.load(Ordering::Relaxed) {
            if !keys.send(0xFC) {
                return Err("Pico Enter failed".into());
            }
            thread::sleep(Duration::from_millis(30));
            if !keys.send(0xFF) {
                return Err("Pico release failed".into());
            }
            taps += 1;
            thread::sleep(Duration::from_millis(40));
        }
        Ok(taps)
    });
    let end = live_gate + (count - rec_gate);
    let finished = harness::wait_playback(client, end);
    running.store(false, Ordering::Relaxed);
    let taps = tapper.join().map_err(|_| "the Pico thread panicked")??;
    if !finished {
        return Err("the replay did not finish".into());
    }
    let play = &client.state().play_coords;
    let recorded = &rec[rec_gate as usize..count as usize];
    let replayed = &play[live_gate as usize..];
    let mismatches = recorded
        .iter()
        .zip(replayed)
        .filter(|(r, p)| r.map(f32::to_bits) != p.map(f32::to_bits))
        .count();
    if mismatches > 0 {
        return Err(format!(
            "{mismatches} ticks differ with Enter tapped {taps} times"
        ));
    }
    println!(
        "  {} ticks from the gate replayed bit for bit with Enter tapped {taps} times",
        count - rec_gate
    );
    Ok(())
}

fn report(name: &str, result: Result<(), String>) -> bool {
    match result {
        Ok(()) => {
            println!("\n*** {name} PASSED ***");
            true
        }
        Err(e) => {
            eprintln!("\n*** {name} FAILED: {e} ***");
            false
        }
    }
}
