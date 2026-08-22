//! Is `first_moving` COMPUTABLE at arm time, instead of observable only after
//! replaying the whole countdown?
//!
//! The judge's bucket fingerprint is the recording's first-moving frame, and to
//! see a replay's you have to replay to it — ~298 ticks of a stationary boarder.
//! Replaying that fast (the speed handover) makes the wait cheap; it does not
//! remove it. Removing it needs the answer to a different question: is the frame
//! DETERMINED by something already known when the replay is armed?
//!
//! The model says yes. The countdown is a fixed-length gate started by the F5
//! restart, and `first_moving` is counted from the ARM — so
//!
//!     first_moving  =  (restart_done_tick + K)  -  arm_consumed_tick
//!                   =   gate_tick              -  arm_consumed_tick
//!
//! with K the countdown length in ticks. Every term on the right is known before
//! a single tick is replayed, PROVIDED K is actually constant. Earlier work
//! measured a related total at {182, 183, 184} — deterministic to ±1 — but never
//! measured the gate against the restart directly, and never tested whether
//! moving the arm moves `first_moving` by exactly the same amount.
//!
//! That is what this does. It runs cycles at DELIBERATELY VARIED arm offsets. If
//! the model holds:
//!
//!   * `gate_tick - restart_done_tick` is the same K regardless of the offset —
//!     the countdown does not care when we armed
//!   * `first_moving` moves one-for-one AGAINST the arm offset
//!   * `first_moving == gate_tick - arm_consumed_tick` exactly
//!
//! If K is constant, the judge never has to replay the countdown: it can compute
//! the bucket at arm time, reject a wrong one before replaying a single tick,
//! and — better — CHOOSE the arm tick that produces the bucket it wants.
//!
//! If K is not constant, that is the answer too, and it is worth knowing
//! precisely how it scatters rather than inferring it from `first_moving`, which
//! confounds the countdown with our own arm jitter.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasSharedMemoryClient;

use crate::harness;

/// Arm offsets to sweep, in milliseconds after the restart completes. Spread
/// well past one tick (10ms) so a one-for-one trade shows up as a clear slope
/// rather than as noise.
const ARM_DELAYS_MS: &[u64] = &[0, 5, 10, 20, 30, 40];
const GATE_TIMEOUT_SECS: u64 = 20;

#[derive(Debug, Clone, Copy)]
struct Cycle {
    delay_ms: u64,
    restart_tick: u32,
    arm_tick: u32,
    gate_tick: u32,
    first_moving: u32,
}

impl Cycle {
    /// The countdown's own length: restart -> gate. The model's constant.
    fn k(&self) -> i64 {
        self.gate_tick as i64 - self.restart_tick as i64
    }
    /// How late we armed, in ticks.
    fn arm_offset(&self) -> i64 {
        self.arm_tick as i64 - self.restart_tick as i64
    }
    /// What the model predicts first_moving should be, from the arm alone.
    fn predicted(&self, k: i64) -> i64 {
        k - self.arm_offset()
    }
}

pub fn run(iterations: u32) -> bool {
    println!("=== Is the countdown gate predictable at arm time? ===");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));
    harness::focus_game();

    let mut cycles: Vec<Cycle> = Vec::new();
    for round in 1..=iterations {
        for &delay_ms in ARM_DELAYS_MS {
            match one_cycle(&mut client, delay_ms) {
                Some(c) => {
                    println!(
                        "  round {} delay {:>2}ms: restart={} arm={} gate={} | K=gate-restart={} armOff={} fm={}",
                        round, delay_ms, c.restart_tick, c.arm_tick, c.gate_tick,
                        c.k(), c.arm_offset(), c.first_moving
                    );
                    cycles.push(c);
                }
                None => println!("  round {} delay {:>2}ms: no gate within budget", round, delay_ms),
            }
            harness::stop(&mut client);
            thread::sleep(Duration::from_millis(120));
        }
    }

    if cycles.len() < 4 {
        println!("\nnot enough cycles completed to conclude anything");
        return false;
    }
    analyse(&cycles)
}

fn one_cycle(client: &mut TasSharedMemoryClient, delay_ms: u64) -> Option<Cycle> {
    {
        let s = client.state_mut();
        s.gate_tick = 0;
        s.gate_index = 0;
        s.restart_done_tick = 0;
        s.arm_consumed_tick = 0;
    }
    if !harness::restart_and_stabilize_inprocess(client) {
        eprintln!("    restart failed");
        return None;
    }
    // The whole point: arm at a DELIBERATELY varied offset after the restart, so
    // a countdown that ignores the arm shows up as a flat K against a sliding fm.
    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms));
    }
    harness::arm_rec(client);

    let deadline = Instant::now() + Duration::from_secs(GATE_TIMEOUT_SECS);
    while Instant::now() < deadline {
        let s = client.state();
        if s.gate_tick != 0 && s.restart_done_tick != 0 && s.arm_consumed_tick != 0 {
            return Some(Cycle {
                delay_ms,
                restart_tick: s.restart_done_tick,
                arm_tick: s.arm_consumed_tick,
                gate_tick: s.gate_tick,
                first_moving: s.gate_index,
            });
        }
        thread::sleep(Duration::from_millis(2));
    }
    None
}

fn analyse(cycles: &[Cycle]) -> bool {
    println!("\n=== ANALYSIS ({} cycles) ===", cycles.len());

    // 1. Is the countdown length constant, independent of when we armed?
    let mut ks: Vec<i64> = cycles.iter().map(|c| c.k()).collect();
    ks.sort_unstable();
    let kmin = ks[0];
    let kmax = ks[ks.len() - 1];
    println!("\n-- countdown length K = gate_tick - restart_done_tick --");
    print_histogram(&ks);
    let k_spread = kmax - kmin;
    println!("  spread: {} tick(s)", k_spread);

    // Per-delay, so a K that quietly tracks the arm cannot hide in the pooled
    // spread — that would mean the countdown is NOT independent of the arm.
    println!("\n-- K by arm delay (must NOT trend) --");
    for &d in ARM_DELAYS_MS {
        let mut v: Vec<i64> = cycles.iter().filter(|c| c.delay_ms == d).map(|c| c.k()).collect();
        if v.is_empty() {
            continue;
        }
        v.sort_unstable();
        println!(
            "  {:>2}ms: n={} K min={} max={} median={}",
            d, v.len(), v[0], v[v.len() - 1], v[v.len() / 2]
        );
    }

    // 2. Does first_moving equal gate - arm exactly? (ticks and indices 1:1)
    let mismatched = cycles
        .iter()
        .filter(|c| c.first_moving as i64 != c.gate_tick as i64 - c.arm_tick as i64)
        .count();
    println!("\n-- first_moving == gate_tick - arm_consumed_tick --");
    if mismatched == 0 {
        println!("  holds on all {} cycles", cycles.len());
    } else {
        println!(
            "  FAILS on {}/{} — ticks and replay indices do not advance 1:1",
            mismatched,
            cycles.len()
        );
    }

    // 3. The actual question: predict fm from the arm alone, using the median K.
    let k_median = ks[ks.len() / 2];
    let mut err: Vec<i64> = cycles
        .iter()
        .map(|c| c.first_moving as i64 - c.predicted(k_median))
        .collect();
    err.sort_unstable();
    println!("\n-- prediction error, fm - (K_median - arm_offset), K_median={} --", k_median);
    print_histogram(&err);
    let worst = err.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
    let within1 = err.iter().filter(|e| e.abs() <= 1).count();
    println!(
        "  worst |error| = {} tick(s); within +/-1: {}/{}",
        worst,
        within1,
        cycles.len()
    );

    println!("\n=== VERDICT ===");
    let ok = worst <= 1 && mismatched == 0;
    if ok {
        println!(
            "  first_moving IS computable at arm time to +/-{} tick(s).",
            worst
        );
        println!("  A judge can rule before replaying anything, and the arm tick can be");
        println!("  CHOSEN to land a wanted bucket instead of rerolled until it does.");
    } else {
        println!("  first_moving is NOT computable from the arm alone:");
        if mismatched > 0 {
            println!("    - replay indices do not track ticks 1:1");
        }
        if worst > 1 {
            println!(
                "    - the countdown length varies by {} ticks, so the gate is not a",
                k_spread
            );
            println!("      fixed offset from the restart");
        }
    }
    ok
}

fn print_histogram(sorted: &[i64]) {
    let mut i = 0;
    while i < sorted.len() {
        let v = sorted[i];
        let n = sorted[i..].iter().take_while(|x| **x == v).count();
        println!("  {:>6}: {} {}", v, "#".repeat(n.min(60)), n);
        i += n;
    }
}
