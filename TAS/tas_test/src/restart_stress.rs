//! In-process restart stress: fire many F5 restarts in one game session and
//! time each one. The DLL holds F5 until the game's own restart has rebuilt
//! the level, so every restart must complete; a slow or missing one is a
//! regression in that handshake. About a third of a second per restart, so a
//! few hundred run in the time of a handful of game relaunches.

use std::thread;
use std::time::{Duration, Instant};

use crate::harness;

/// Waits between restarts, cycled, so restarts land in the countdown, just
/// after the start and well into the run.
const SETTLE_MS: &[u64] = &[50, 400, 1500, 3000];

pub fn run(count: u32) -> bool {
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    let mut took_ms = Vec::with_capacity(count as usize);
    let mut failed = 0u32;
    let start = Instant::now();
    for i in 0..count {
        let t0 = Instant::now();
        if harness::restart_inprocess(&mut client) {
            took_ms.push(t0.elapsed().as_millis() as u64);
        } else {
            failed += 1;
            println!("  restart {}: never completed", i + 1);
        }
        thread::sleep(Duration::from_millis(
            SETTLE_MS[i as usize % SETTLE_MS.len()],
        ));
    }
    took_ms.sort_unstable();
    let pick = |q: usize| {
        took_ms
            .get(q * took_ms.len().saturating_sub(1) / 100)
            .copied()
    };
    println!(
        "{count} restarts in {:.0} s, {failed} failed; restart took ms: min {:?}, median {:?}, max {:?}",
        start.elapsed().as_secs_f64(),
        pick(0),
        pick(50),
        pick(100),
    );
    if failed == 0 {
        println!("*** RESTART-STRESS PASSED ***");
        true
    } else {
        println!("*** RESTART-STRESS FAILED: {failed} restarts never completed ***");
        false
    }
}
