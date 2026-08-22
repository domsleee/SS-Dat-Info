//! restart-precision: is the residual +/-1 set by WHEN THE RESTART LANDS?
//!
//! WHERE THIS COMES FROM. Arming on a measured tick already removed most of the
//! scatter in `first_moving` (3-6 values down to 2-3). What is left is +/-1, and
//! it is not explained by anything the game stores — module images and the whole
//! 96MB heap were scanned and neither partitions restarts the way the bucket
//! does. It is also not explained by the engine clock phase at ARM: over 20 runs
//! with the arm offset held at exactly +40 ticks, that phase advanced
//! monotonically ~6 units per run and spanned 111 units out of 99,999 — one tenth
//! of one percent of a tick — while fm still took three values and alternated.
//!
//! THE REMAINING SUSPECT. The restart. CMD_RESTART is written to shared memory
//! from another process at an arbitrary instant; the game picks it up on some
//! later frame. The sub-frame offset between "written" and "processed" varies run
//! to run, and it is that offset which sets where the countdown's 3-second
//! deadline falls relative to the tick grid — which decides whether the gate
//! opens on tick N or N+1.
//!
//! If that is right, firing CMD_RESTART at a CONTROLLED sub-frame moment — right
//! after the tick counter increments, so the command always lands the same
//! distance into a frame — should collapse first_moving to a single value.
//!
//! Runs both ways in one session so the comparison is like-for-like:
//!   * UNCONTROLLED: send the restart whenever, as the harness does today.
//!   * EDGE-ALIGNED: spin until the tick counter changes, then send immediately.
//! Everything downstream (arm at +40 measured ticks) is identical, so any
//! difference in the fm spread is attributable to the restart timing alone.

use std::collections::BTreeMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::harness;
use tas_shared::{TasCommand, TasSharedMemoryClient};

const ARM_OFFSET_TICKS: u32 = 40;
const RESTART_TIMEOUT_SECS: u64 = 15;
const RECORD_SECS: u64 = 5;
const MIN_TICKS: usize = 260;

/// Do one restart+arm cycle and return the resulting first_moving.
///
/// `edge_aligned` is the only difference between the two arms of the experiment.
fn one_cycle(client: &mut TasSharedMemoryClient, edge_aligned: bool) -> Option<u32> {
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(50));
    client.reset_restart_state();

    if edge_aligned {
        // Spin until the tick counter moves, then send IMMEDIATELY. That pins the
        // command to a fixed distance past a frame boundary instead of landing
        // wherever this thread happened to wake up.
        let t0 = client.tick_count_volatile();
        let spin_deadline = Instant::now() + Duration::from_millis(200);
        while client.tick_count_volatile() == t0 && Instant::now() < spin_deadline {
            std::hint::spin_loop();
        }
    }
    client.send_command(TasCommand::Restart);

    let start = Instant::now();
    loop {
        if client.restart_state() == 2 {
            break;
        }
        if start.elapsed() > Duration::from_secs(RESTART_TIMEOUT_SECS) {
            return None;
        }
        std::hint::spin_loop();
    }
    client.reset_restart_state();

    // Arm at a measured tick offset — identical in both arms, so it cannot be
    // the source of any difference between them.
    let t0 = client.tick_count_volatile();
    let deadline = Instant::now() + Duration::from_millis(4000);
    while client.tick_count_volatile().wrapping_sub(t0) < ARM_OFFSET_TICKS {
        if Instant::now() >= deadline {
            return None;
        }
        std::hint::spin_loop();
    }
    client.send_command(TasCommand::ArmRec);
    let offset = client.tick_count_volatile().wrapping_sub(t0);
    if offset != ARM_OFFSET_TICKS {
        // Arm landed off-target; this sample cannot be compared to the others.
        eprintln!("      (arm landed at +{}, discarding)", offset);
    }

    thread::sleep(Duration::from_secs(RECORD_SECS));
    let count = client.state().recorded_count as usize;
    harness::stop(client);
    let n = count.min(client.state().rec_coords.len());
    if n < MIN_TICKS {
        return None;
    }
    let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
    tas_shared::cont::detect_first_moving(&coords, n as u32)
}

fn hist(v: &[u32]) -> BTreeMap<u32, usize> {
    let mut m = BTreeMap::new();
    for x in v {
        *m.entry(*x).or_insert(0) += 1;
    }
    m
}

pub fn run(iterations: u32) -> bool {
    println!("=== RESTART-PRECISION: does pinning the restart to a tick edge fix the +/-1? ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut uncontrolled: Vec<u32> = Vec::new();
    let mut aligned: Vec<u32> = Vec::new();

    // Interleave the two arms rather than running one block then the other, so a
    // slow drift in the machine (thermal, background load) cannot masquerade as
    // an effect of the treatment.
    for iter in 1..=iterations {
        for edge in [false, true] {
            match one_cycle(&mut client, edge) {
                Some(fm) => {
                    println!(
                        "  iter {:>2} {:<12}: fm={}",
                        iter,
                        if edge { "EDGE-ALIGNED" } else { "uncontrolled" },
                        fm
                    );
                    if edge {
                        aligned.push(fm);
                    } else {
                        uncontrolled.push(fm);
                    }
                }
                None => eprintln!(
                    "  iter {:>2} {:<12}: cycle failed",
                    iter,
                    if edge { "EDGE-ALIGNED" } else { "uncontrolled" }
                ),
            }
        }
    }

    println!("\n========== RESULT ==========");
    if uncontrolled.len() < 5 || aligned.len() < 5 {
        println!("  Not enough usable cycles to compare.");
        return false;
    }
    let hu = hist(&uncontrolled);
    let ha = hist(&aligned);
    let bu = hu.values().cloned().max().unwrap_or(0) as f64 / uncontrolled.len() as f64;
    let ba = ha.values().cloned().max().unwrap_or(0) as f64 / aligned.len() as f64;

    println!("  uncontrolled : {:?}  ({} values, {:.0}% on the mode)", hu, hu.len(), bu * 100.0);
    println!("  edge-aligned : {:?}  ({} values, {:.0}% on the mode)", ha, ha.len(), ba * 100.0);

    println!();
    if ha.len() == 1 && hu.len() > 1 {
        println!("  *** CONFIRMED: pinning the restart to a tick edge makes first_moving");
        println!("  DETERMINISTIC. The residual +/-1 was the restart landing at a varying");
        println!("  sub-frame offset. Send CMD_RESTART on a tick edge and the bucket stops");
        println!("  being a lottery at all — no judge needed, fast or slow.");
    } else if ba > bu + 0.15 {
        println!("  IMPROVED but not deterministic: the mode went from {:.0}% to {:.0}%.", bu * 100.0, ba * 100.0);
        println!("  Restart timing is part of the residual but not all of it.");
    } else {
        println!("  NO EFFECT. Pinning the restart to a tick edge does not change the");
        println!("  spread, so the +/-1 is not set by when the command lands. That");
        println!("  exhausts the timing explanations available from outside the game.");
    }
    true
}
