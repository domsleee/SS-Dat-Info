//! Is the Player's countdown a fixed anchor for the gate?
//!
//! The countdown float counts ticks since the rider's reset; the rider is
//! released on the tick it passes 3.0. Each aligned PLAY here samples it
//! alongside playback_pos: their difference is where the arm landed relative
//! to the reset (expected to vary), and the gate's distance from the release
//! tick must be identical on every restart for the countdown to replace the
//! movement gate.

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::command_edges::{cleanup, load_fixture};
use crate::gamemem::GameMemory;
use crate::harness;

const RUNS: usize = 15;
/// Ticks after reset on which the rider is released (t first exceeds 3.0).
const RELEASE_TICK: i64 = 302;

pub fn run() -> bool {
    println!("=== COUNTDOWN-ANCHOR: does the countdown fix the gate? ===\n");
    let mut client = harness::ensure_game_running();
    let result = measure(&mut client);
    cleanup(&mut client);
    match result {
        Ok(true) => true,
        Ok(false) => false,
        Err(e) => {
            eprintln!("ERROR: {e}");
            false
        }
    }
}

fn measure(client: &mut tas_shared::TasSharedMemoryClient) -> Result<bool, String> {
    harness::stop_competing_tas_ui_writer();
    harness::stop(client);
    let memory = GameMemory::attach().ok_or("cannot read the game's memory")?;
    let mut arm_offsets = Vec::new();
    let mut gate_gaps = Vec::new();
    let (_, log_cursor) = client.state().read_log_entries(0);
    let mut captures_ok = true;
    for run in 1..=RUNS {
        load_fixture(client)?;
        client.state_mut().playback_speed = 1.0;
        let (_rec_gate, live_gate) = harness::restart_play_aligned_unwatched(client)?;
        // Sample (countdown ticks - playback_pos) many times; a sample taken
        // mid-tick can be off by one, so keep the most common value.
        let mut offsets: HashMap<i64, u32> = HashMap::new();
        let deadline = Instant::now() + Duration::from_millis(400);
        while Instant::now() < deadline {
            let player = client.state().player_ptr;
            let pos = client.playback_pos_volatile() as i64;
            if let Ok(ticks) = memory.countdown_ticks(player) {
                if client.playback_pos_volatile() as i64 == pos {
                    *offsets.entry(ticks as i64 - pos).or_default() += 1;
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
        let offset = offsets
            .iter()
            .max_by_key(|(_, n)| **n)
            .map(|(o, _)| *o)
            .ok_or("no countdown samples")?;
        // Release tick in playback_pos space, and the gate's distance from it.
        let release_pos = RELEASE_TICK - offset;
        let gap = live_gate as i64 - release_pos;
        println!(
            "  run {run:2}: arm offset {offset} ticks after reset, live gate {live_gate}, release at pos {release_pos}, gate - release = {gap}   (samples {offsets:?})"
        );
        arm_offsets.push(offset);
        gate_gaps.push(gap);
        // The DLL drops capture_ok when its countdown prediction misses the gate.
        captures_ok &= client.state().capture_ok == 1;
        harness::stop(client);
    }
    let distinct = |v: &[i64]| {
        let mut d = v.to_vec();
        d.sort();
        d.dedup();
        d
    };
    let offsets = distinct(&arm_offsets);
    let gaps = distinct(&gate_gaps);
    println!("\n  arm offsets seen: {offsets:?}");
    println!("  gate - release seen: {gaps:?}");
    let (entries, _) = client.state().read_log_entries(log_cursor);
    let mismatches = entries
        .iter()
        .filter(|(_, _, text)| text.contains("differs from the countdown"))
        .count();
    println!("  DLL prediction mismatches: {mismatches}, capture_ok throughout: {captures_ok}");
    if mismatches > 0 || !captures_ok {
        println!(
            "*** COUNTDOWN-ANCHOR FAILED: the DLL's predicted gate missed the observed gate ***"
        );
        return Ok(false);
    }
    if gaps.len() == 1 {
        println!(
            "*** COUNTDOWN-ANCHOR PASSED: the gate is always {} tick(s) after release across {RUNS} restarts ***",
            gaps[0]
        );
        Ok(true)
    } else {
        println!("*** COUNTDOWN-ANCHOR FAILED: the gate moved relative to the countdown ***");
        Ok(false)
    }
}
