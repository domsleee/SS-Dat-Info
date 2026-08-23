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

//! ==========================================================================
//! OUTCOME: measured true, SHIPPED FALSE. Do not wire this into the controller.
//! ==========================================================================
//!
//! The measurement below is real: arming on a tick does collapse first_moving
//! from 3-6 scattered values to two adjacent ones. But turning that into a
//! controller change made CONT WORSE, and the reason is worth keeping.
//!
//! The change: TransportPort gained tick_count, RestartWaitDone set a tick
//! target instead of returning Wait{ms}, and a new ArmTickWait phase polled for
//! it. All 332 unit tests passed and fe10065-cont still passed with zero drift
//! and zero overshoot — correctness was never in question. The rerolls were:
//!
//!     baseline (slept settle)   first-try 7/8, 5/8, 6/8   mean 0.2 - 0.6
//!     tick-precise arm          first-try 1/8, 3/8        mean 3.5, 2.4
//!
//! Six to ten times more rerolls. Reverted.
//!
//! WHY IT BACKFIRED. arm_settle_ms is calibrated IN MILLISECONDS against the
//! slept path — base = (OBSERVED_AT_ZERO_SETTLE - fm) * ARM_SETTLE_MS_PER_FRAME,
//! clamped to 0..48ms. Re-expressing that as ticks (div_ceil(10), so 0..5 ticks)
//! keeps the number and throws away what it was tuned against: a different
//! reference point, a different rounding, and a poll delay before the arm fires.
//! Precision the calibration was not built for is not an improvement.
//!
//! AND THE PREMISE WAS STALE. The estimate of a 3.3x win came from a recorded
//! "19% first-try, mean 3.88 attempts". The shipped path measures 0.2-0.6 mean
//! rerolls today — first-try 60-88%. There was never 3.3x of headroom; that
//! figure predates the arm calibration and should not be used again.
//!
//! What survives is the physics, and it is worth knowing: the countdown is
//! deterministic to +/-1 tick, the bucket follows the arm tick, and the shipped
//! ms calibration already lands it most of the time. If anyone revisits this,
//! the only honest path is to re-derive the calibration IN TICKS from scratch
//! against the real game — not to convert the existing constants.

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
/// Engine clock units in one 10ms tick — QPC at 10MHz, measured in-process
/// (9,999,900 units/sec => 99,999 per tick).
const TICK_UNITS: u32 = 99_999;

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
    let mut phases: Vec<u32> = Vec::new();

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed", iter);
            continue;
        }
        let t0 = client.tick_count_volatile();
        let target = t0.wrapping_add(TARGET_OFFSET_TICKS);

        // Schedule the arm INSIDE the game thread instead of racing it from here.
        //
        // Spinning outside and firing on the target tick only controlled when the
        // command was WRITTEN. cave2 consumes it on whichever Supreme::Cycle comes
        // next, and first_moving counts from CONSUMPTION — so the tick that
        // actually mattered was never pinned. arm_at_tick makes cave2 hold the
        // command until the counter reaches the target, then consume it exactly
        // there and report the tick it used.
        client.state_mut().arm_consumed_tick = 0;
        client.state_mut().arm_at_tick = target;
        client.send_command(TasCommand::ArmRec);

        // Wait for the DLL to report consumption rather than assuming it.
        let deadline = Instant::now() + Duration::from_millis(WAIT_TIMEOUT_MS);
        let mut consumed = 0u32;
        while Instant::now() < deadline {
            let c = client.state().arm_consumed_tick;
            if c != 0 {
                consumed = c;
                break;
            }
            std::hint::spin_loop();
        }
        if consumed == 0 {
            eprintln!("  iter {}: arm never consumed", iter);
            client.state_mut().arm_at_tick = 0;
            continue;
        }
        let actual_offset = consumed.wrapping_sub(t0);
        let clk_at_arm = client.state().clock_delta_lo;

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
            "  iter {:>2}: armed +{} (target +{})  fm={}  total={}  clk%tick={}",
            iter,
            actual_offset,
            TARGET_OFFSET_TICKS,
            fm,
            fm + actual_offset,
            clk_at_arm % TICK_UNITS
        );
        phases.push(clk_at_arm);
        fms.push(fm);
        offsets.push(actual_offset);
        totals.push(fm + actual_offset);
    }

    let ok = report(&fms, &offsets, &totals);
    phase_split(&fms, &phases);
    ok
}

/// THE TEST THAT MATTERS. With the arm offset held identical, any remaining
/// spread in first_moving is the game's +/-1. If the absolute clock phase at the
/// arm frame separates those groups, it is a fast judge: read it at ARM and know
/// the bucket, with no replay at all.
fn phase_split(fms: &[u32], phases: &[u32]) {
    if fms.len() != phases.len() || fms.is_empty() {
        return;
    }
    println!("\n--- absolute clock phase at ARM, grouped by first_moving ---");
    let mut by_fm: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (fm, ph) in fms.iter().zip(phases.iter()) {
        by_fm.entry(*fm).or_default().push(*ph % TICK_UNITS);
    }
    let mut ranges: Vec<(u32, u32, u32)> = Vec::new();
    for (fm, mut ph) in by_fm {
        ph.sort_unstable();
        let lo = *ph.first().unwrap();
        let hi = *ph.last().unwrap();
        println!(
            "  fm={:<5} n={:<3} phase {:>6}..{:<6}  {:?}",
            fm,
            ph.len(),
            lo,
            hi,
            ph
        );
        ranges.push((fm, lo, hi));
    }
    if ranges.len() < 2 {
        println!("\n  Only one bucket occurred — cannot tell whether phase separates them.");
        return;
    }
    // Separable if no two buckets overlap in phase.
    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| r.1);
    let mut overlap = false;
    for w in sorted.windows(2) {
        if w[0].2 >= w[1].1 {
            overlap = true;
        }
    }
    println!();
    if overlap {
        println!("  PHASE DOES NOT SEPARATE THE BUCKETS — the ranges overlap, so the same");
        println!("  phase at ARM occurs for more than one first_moving. Reading it early");
        println!("  would accept wrong buckets.");
    } else {
        println!("  *** PHASE SEPARATES THE BUCKETS CLEANLY ***");
        println!("  Each first_moving occupies its own phase band, so the bucket is");
        println!("  knowable AT ARM from a single u32 — no replay, no 3.1s wait.");
        println!("  Thresholds (midpoints between adjacent bands):");
        for w in sorted.windows(2) {
            println!(
                "    phase < {:>6} => fm {}   |   phase > {:>6} => fm {}",
                (w[0].2 + w[1].1) / 2,
                w[0].0,
                (w[0].2 + w[1].1) / 2,
                w[1].0
            );
        }
    }
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
        println!(
            "  The arm offset itself still varies ({:?}), so this run did not actually",
            oh.keys()
        );
        println!("  test the hypothesis — the poll is not landing on a fixed tick. Tighten");
        println!("  the wait before drawing any conclusion about the game.");
    } else {
        println!("  NOT confirmed. The arm landed on a fixed tick every time and yet");
        println!(
            "  first_moving still spread across {} values. That puts the variance back",
            fh.len()
        );
        println!("  in the game, and a precise arm cannot fix it.");
    }
    true
}
