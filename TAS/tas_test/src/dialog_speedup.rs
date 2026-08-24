//! Reproduce two user-reported clock bugs, with the DLL's own diagnostics:
//!
//! 1. "After finishing, if you don't press anything on the save-replay screen,
//!    it plays in fast forward for a while when you do press it."
//! 2. "The menu speed is sometimes slow and sometimes fast."
//!
//! Both are statements about the tick machinery: the engine freezes at the
//! dialog while the wall clock runs, and what happens to that backlog when the
//! engine resumes decides everything. cave5 has a one-tick drain for exactly
//! this — but it is gated on `playback_speed == 1.0`, so any other speed sends
//! the backlog through the per-frame cap instead: a sustained fast-forward.
//!
//! Method: replay a FINISHING recording to the end, let the save dialog sit
//! untouched for a while, then dismiss it and profile ticks/second for the
//! next seconds, alongside cave5's new diagnostics (raw demand, live
//! tick-advance, drain count). Run once at 1.0x and once at a non-1x speed —
//! if the speed gate is the bug, the second run bursts and the first does not.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasMode;

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-decent-done.tasrec";
/// How long to leave the dialog untouched. Long enough that the backlog is
/// unmistakable (native = 100 ticks/sec, so 15s = 1500 ticks of backlog).
const DIALOG_IDLE_SECS: u64 = 15;
/// Headroom over the EXPECTED rate before it counts as a burst. The expected
/// rate after dismissal is the playback speed times native — the tool honors
/// its speed setting on the post-race screen until auto-stop — so the bug is
/// a rate ABOVE that (the backlog replaying), not the setting itself.
const BURST_HEADROOM: f64 = 1.4;

pub fn run(speed: f32, rec: Option<&str>) -> bool {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let rel = rec.unwrap_or(RECORDING_REL);
    let candidates = [
        exe_dir.join("../../..").join(rel),
        PathBuf::from(rel),
        exe_dir.join("../../..").join("TAS/recordings").join(rel),
    ];
    let Some(path) = candidates.iter().find(|p| p.exists()) else {
        eprintln!("ERROR: couldn't locate {}", rel);
        return false;
    };
    let path = path.to_string_lossy().into_owned();
    println!(
        "=== Save-dialog fast-forward repro: {} at {}x ===",
        path, speed
    );

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    let loaded = match replay::load_tasrec(std::path::Path::new(&path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    println!("  {} ticks loaded", loaded.count);
    harness::focus_game();

    // Aligned PLAY to the end of the recording — through the finish line.
    client.state_mut().playback_speed = speed;
    if harness::restart_play_aligned_inprocess(&mut client).is_none() {
        eprintln!("ERROR: aligned PLAY failed to arm");
        return false;
    }

    // Wait for the run to reach the finish. The race finishes MID-replay and
    // the finish screen freezes the engine with mode still PLAY — playback_pos
    // just stops. So the end condition is position STABILITY (or mode leaving
    // PLAY at the aligned endpoint, whichever happens first), well past the
    // countdown.
    let deadline = Instant::now() + Duration::from_secs(240);
    let mut stable_pos = 0u32;
    let mut stable_since = Instant::now();
    loop {
        if Instant::now() > deadline {
            eprintln!("ERROR: replay never reached the finish");
            return false;
        }
        let s = client.state();
        let pos = s.playback_pos;
        if s.mode != TasMode::Play as u32 && pos > loaded.count / 2 {
            break;
        }
        if pos != stable_pos {
            stable_pos = pos;
            stable_since = Instant::now();
        } else if pos > 500 && stable_since.elapsed() > Duration::from_secs(3) {
            // Frozen at the finish screen with the dialog up.
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    println!(
        "  replay ended at pos={} race_time_cs={:#x}",
        client.state().playback_pos,
        client.state().race_time_cs
    );

    // Wait for the engine to freeze at the save dialog: frame_count stable.
    let mut frozen = false;
    let mut last_fc = client.state().frame_count;
    for _ in 0..40 {
        thread::sleep(Duration::from_millis(250));
        let fc = client.state().frame_count;
        if fc == last_fc {
            frozen = true;
            break;
        }
        last_fc = fc;
    }
    println!(
        "  engine {} at the dialog (frame_count={})",
        if frozen { "FROZEN" } else { "still cycling" },
        last_fc
    );

    // The user's scenario: touch nothing for a while.
    println!("  idling {}s with the dialog up...", DIALOG_IDLE_SECS);
    let t0 = client.state().tick_count;
    thread::sleep(Duration::from_secs(DIALOG_IDLE_SECS));
    let s = client.state();
    println!(
        "  during idle: ticks advanced {} | demand={} | tick_advance={:.5} | drains={} | speed={}",
        s.tick_count.wrapping_sub(t0),
        s.diag_demand,
        f32::from_bits(s.diag_tick_advance),
        s.diag_drain_count,
        s.playback_speed
    );

    // Dismiss and profile the next seconds, 100ms buckets.
    let drains_before = s.diag_drain_count;
    harness::dismiss_save_dialog_pub();
    let mut prev = client.state().tick_count;
    let mut buckets: Vec<u32> = Vec::new();
    let mut worst_demand = 0i32;
    for _ in 0..60 {
        thread::sleep(Duration::from_millis(100));
        let s = client.state();
        buckets.push(s.tick_count.wrapping_sub(prev));
        prev = s.tick_count;
        worst_demand = worst_demand.max(s.diag_demand);
    }
    let s = client.state();
    let drains_after = s.diag_drain_count;

    // Report per-second tick rates.
    println!("  post-dismiss ticks per 100ms (first 30 buckets):");
    println!("    {:?}", &buckets[..30.min(buckets.len())]);
    let max_bucket = buckets.iter().copied().max().unwrap_or(0);
    let burst_secs = buckets.iter().filter(|b| **b > 14).count() as f64 * 0.1;
    println!(
        "  max rate {:.0} ticks/sec | above-native for {:.1}s | worst demand seen {} | drains {} -> {}",
        max_bucket as f64 * 10.0,
        burst_secs,
        worst_demand,
        drains_before,
        drains_after
    );

    harness::stop(&mut client);
    let expected = 100.0 * f64::from(speed.max(1.0));
    let ok = (max_bucket as f64 * 10.0) <= expected * BURST_HEADROOM;
    if ok {
        println!("\n*** DIALOG-SPEEDUP PASSED: no fast-forward after dismissal ***");
    } else {
        println!(
            "
*** DIALOG-SPEEDUP FAILED: {:.1}x fast-forward (expected <= {:.0} ticks/s) ***",
            max_bucket as f64 / 10.0,
            expected * BURST_HEADROOM
        );
    }
    ok
}
