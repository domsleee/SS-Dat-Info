//! arm-precision: is the F5 "lottery" actually our own arm jitter?
//!
//! WHAT THE SCANS ESTABLISHED. Nothing the game stores — not one slot of the
//! 2.6MB of module images, not one of 1.3M surviving heap candidates — predicts
//! which tick the boarder leaves spawn on. But correlating against the
//! ARM-INDEPENDENT quantity (ticks from a fixed reference to the gate, rather
//! than from ARM) gave a very different picture:
//!
//!     restart->gate totals over 14 restarts: {182, 183, 184}, twelve of them 183
//!
//! The countdown is essentially DETERMINISTIC — 183 ticks, +/-1. So first_moving
//! is not being scattered by the game; it is being scattered by WHEN WE ARM.
//!
//! THE SUSPECT. `transport::arm_settle_ms` computes a target delay and the
//! controller then SLEEPS it. A sleep is accurate to the OS scheduler, several
//! ticks at best, and every tick of sleep error moves first_moving by one. That
//! is a self-inflicted lottery: the documented first-try rate is 19% and the mean
//! is 3.88 attempts, against a game whose countdown only varies +/-1.
//!
//! THE TEST. Arm at a MEASURED tick offset instead of a slept one: poll
//! tick_count (which cave5 publishes every frame) and fire ARM the instant it
//! reaches the target. If the scatter is ours, first_moving should collapse onto
//! one or two values; if it is the game's, it will stay spread and this idea is
//! wrong.
//!
//! Reports the distribution both ways so the two cannot be confused.

use std::collections::BTreeMap;
use std::thread;
use std::time::{Duration, Instant};

use crate::harness;
use tas_shared::TasCommand;

/// Ticks after the restart reference to fire ARM. Any fixed value works — what
/// matters is hitting the SAME one every time.
const TARGET_OFFSET_TICKS: u32 = 40;
/// Give up if the tick counter never reaches the target.
const WAIT_TIMEOUT_MS: u64 = 4000;
const RECORD_SECS: u64 = 5;
const MIN_TICKS: usize = 260;

pub fn run(iterations: u32) -> bool {
    println!("=== ARM-PRECISION: is the bucket lottery our own arm jitter? ===\n");
    println!(
        "  Arming at a measured +{} ticks rather than sleeping to it.\n",
        TARGET_OFFSET_TICKS
    );

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut fms: Vec<u32> = Vec::new();
    let mut offsets: Vec<u32> = Vec::new();
    let mut totals: Vec<u32> = Vec::new();

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed", iter);
            continue;
        }
        let t0 = client.tick_count_volatile();
        let target = t0.wrapping_add(TARGET_OFFSET_TICKS);

        // Tight poll. No sleep: sleeping is the very thing under test, and a 1ms
        // sleep is a tenth of a tick anyway — spinning briefly is cheaper than
        // being wrong about what we are measuring.
        let deadline = Instant::now() + Duration::from_millis(WAIT_TIMEOUT_MS);
        let mut reached = false;
        while Instant::now() < deadline {
            if client.tick_count_volatile().wrapping_sub(t0) >= TARGET_OFFSET_TICKS {
                reached = true;
                break;
            }
            std::hint::spin_loop();
        }
        if !reached {
            eprintln!("  iter {}: tick target never reached", iter);
            continue;
        }
        // Fire immediately — no focus_game here, it costs ~200ms and would
        // reintroduce exactly the jitter being measured.
        client.send_command(TasCommand::ArmRec);
        let armed_at = client.tick_count_volatile();
        let actual_offset = armed_at.wrapping_sub(t0);
        let _ = target;

        thread::sleep(Duration::from_secs(RECORD_SECS));
        let count = client.state().recorded_count as usize;
        harness::stop(&mut client);

        let n = count.min(client.state().rec_coords.len());
        let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
        let fm = match tas_shared::cont::detect_first_moving(&coords, n as u32) {
            Some(f) if n >= MIN_TICKS => f,
            _ => {
                eprintln!("  iter {}: unusable recording (ticks={})", iter, n);
                continue;
            }
        };

        println!(
            "  iter {:>2}: armed at +{} ticks (target +{})  fm={}  total={}",
            iter,
            actual_offset,
            TARGET_OFFSET_TICKS,
            fm,
            fm + actual_offset
        );
        fms.push(fm);
        offsets.push(actual_offset);
        totals.push(fm + actual_offset);
    }

    report(&fms, &offsets, &totals)
}

fn hist(vals: &[u32]) -> BTreeMap<u32, usize> {
    let mut m = BTreeMap::new();
    for v in vals {
        *m.entry(*v).or_insert(0) += 1;
    }
    m
}

fn report(fms: &[u32], offsets: &[u32], totals: &[u32]) -> bool {
    println!("\n========== RESULT ==========");
    if fms.len() < 6 {
        println!("  Only {} usable runs — not enough to judge.", fms.len());
        return false;
    }

    let fh = hist(fms);
    let oh = hist(offsets);
    let th = hist(totals);

    println!("  arm offset achieved : {:?}", oh);
    println!("  first_moving        : {:?}", fh);
    println!("  restart->gate total : {:?}", th);

    let best = fh.values().cloned().max().unwrap_or(0);
    let rate = best as f64 / fms.len() as f64;
    println!(
        "\n  most common first_moving hit {}/{} = {:.0}% of runs",
        best,
        fms.len(),
        rate * 100.0
    );

    println!();
    if oh.len() == 1 && fh.len() <= 2 {
        println!("  CONFIRMED. Arming on a measured tick lands the SAME offset every time,");
        println!("  and first_moving collapses to one or two adjacent values. The scatter");
        println!("  was the sleep, not the game — so the fix is not a faster judge but a");
        println!("  precise arm: poll tick_count and fire on the target tick. The residual");
        println!("  is the game's own +/-1 rounding and cannot be removed, only judged.");
    } else if oh.len() > 1 {
        println!("  The arm offset itself still varies ({:?}), so this run did not actually", oh.keys());
        println!("  test the hypothesis — the poll is not landing on a fixed tick. Tighten");
        println!("  the wait before drawing any conclusion about the game.");
    } else {
        println!("  NOT confirmed. The arm landed on a fixed tick every time and yet");
        println!("  first_moving still spread across {} values. That puts the variance back", fh.len());
        println!("  in the game, and a precise arm cannot fix it.");
    }
    true
}
