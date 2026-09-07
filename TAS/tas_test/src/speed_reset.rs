//! Speed reset verification: after 2x speed and STOP, the game's time advance
//! constant is back at its base value (0.01). Without the Cave 5 restore the
//! constant stays at 0.005 in OFF mode, so ESC menus, F5 and all non-TAS
//! gameplay run at 2x.
//!
//! Test strategy:
//!   1. Measure baseline tick rate in OFF mode (~100 ticks/s)
//!   2. REC at 2x speed, verify tick rate is ~200 ticks/s
//!   3. STOP (mode → OFF)
//!   4. Measure tick rate in OFF mode again — should match baseline, NOT 2x
//!   5. F5 restart — should work (game responds at normal speed)
//!   6. Confirm game is alive and tick rate is still normal
//!
//! Pass criteria:
//!   - Post-stop tick ratio vs baseline is within 0.7–1.3 (normal speed)
//!   - F5 restart succeeds after stop from 2x
//!   - 2x REC tick ratio vs baseline is within 1.6–2.4

use crate::harness;
use std::thread;
use std::time::{Duration, Instant};
use tas_shared::TasMode;

/// Duration for each tick rate measurement (seconds).
const MEASURE_SECS: u64 = 2;

/// Post-stop and post-F5 tick rate vs baseline; 1.0 = identical, the band
/// absorbs timing jitter.
const RESET_RATIO_MIN: f64 = 0.7;
const RESET_RATIO_MAX: f64 = 1.3;

/// 2x REC tick rate vs baseline.
const FAST_RATIO_MIN: f64 = 1.6;
const FAST_RATIO_MAX: f64 = 2.4;

/// Cave 2 frame_count delta over MEASURE_SECS: the game's actual tick
/// processing rate, which the time advance constant governs.
fn measure_tick_rate(client: &tas_shared::TasSharedMemoryClient) -> u32 {
    let start_fc = client.frame_count_volatile();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(MEASURE_SECS) {
        thread::sleep(Duration::from_millis(50));
    }
    let end_fc = client.frame_count_volatile();
    end_fc.saturating_sub(start_fc)
}

pub fn run() -> bool {
    println!("=== Speed Reset Verification Test ===\n");

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::ensure_exclusive_runtime_ownership(&mut client, "speed reset validation failures");
    if !harness::require_speed_preconditions(&client) {
        return false;
    }

    println!("--- Phase 1: Baseline tick rate (1.0x, OFF mode) ---");
    client.state_mut().playback_speed = 1.0;
    let baseline_ticks = measure_tick_rate(&client);
    println!("  Baseline: {} frames/{}s", baseline_ticks, MEASURE_SECS);
    if baseline_ticks == 0 {
        eprintln!("ERROR: No baseline ticks");
        return false;
    }

    println!("\n--- Phase 2: REC at 2.0x speed ---");
    client.state_mut().playback_speed = 2.0;
    harness::arm_rec(&mut client);
    let fast_rec_ticks = measure_tick_rate(&client);
    let rec_count = client.state().recorded_count;
    println!(
        "  2x REC: {} frames/{}s, recorded {} ticks",
        fast_rec_ticks, MEASURE_SECS, rec_count
    );

    println!("\n--- Phase 3: STOP (verify time constant resets) ---");
    harness::stop(&mut client);
    // Give Cave 5 a few frames to write the reset.
    thread::sleep(Duration::from_millis(200));

    // playback_speed stays at 2.0 in shared state on purpose: the DLL must
    // restore the constant because mode == OFF, not because the speed changed.
    let mode_after_stop = client.mode_volatile();
    println!("  Mode after stop: {} (expect 0=OFF)", mode_after_stop);

    let post_stop_ticks = measure_tick_rate(&client);
    println!(
        "  Post-stop tick rate: {} frames/{}s",
        post_stop_ticks, MEASURE_SECS
    );

    println!("\n--- Phase 4: F5 restart after 2x stop ---");
    client.state_mut().playback_speed = 1.0;
    let f5_ok = harness::restart_and_stabilize(&client);
    println!("  F5 restart: {}", if f5_ok { "OK" } else { "FAILED" });

    let post_f5_ticks = if f5_ok {
        let t = measure_tick_rate(&client);
        println!("  Post-F5 tick rate: {} frames/{}s", t, MEASURE_SECS);
        t
    } else {
        0
    };

    let fast_ratio = fast_rec_ticks as f64 / baseline_ticks as f64;
    let reset_ratio = post_stop_ticks as f64 / baseline_ticks as f64;
    let post_f5_ratio = if post_f5_ticks > 0 {
        post_f5_ticks as f64 / baseline_ticks as f64
    } else {
        0.0
    };

    let fast_pass = (FAST_RATIO_MIN..=FAST_RATIO_MAX).contains(&fast_ratio);
    // The post-stop rate only measures what its name says if STOP actually
    // returned the engine to OFF.
    let stopped_ok = mode_after_stop == TasMode::Off as u32;
    let reset_pass = stopped_ok && (RESET_RATIO_MIN..=RESET_RATIO_MAX).contains(&reset_ratio);
    let f5_pass = f5_ok && (RESET_RATIO_MIN..=RESET_RATIO_MAX).contains(&post_f5_ratio);

    println!("\n=== SPEED RESET TEST RESULTS ===");
    println!("Baseline (1x OFF): {} frames", baseline_ticks);
    println!(
        "2x REC:            {} frames (ratio {:.3}, expected {:.1}-{:.1}) — {}",
        fast_rec_ticks,
        fast_ratio,
        FAST_RATIO_MIN,
        FAST_RATIO_MAX,
        if fast_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "Post-stop (OFF):   {} frames (ratio {:.3}, expected {:.1}-{:.1}) — {}",
        post_stop_ticks,
        reset_ratio,
        RESET_RATIO_MIN,
        RESET_RATIO_MAX,
        if reset_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "Post-F5:           {} frames (ratio {:.3}, expected {:.1}-{:.1}) — {}",
        post_f5_ticks,
        post_f5_ratio,
        RESET_RATIO_MIN,
        RESET_RATIO_MAX,
        if f5_pass { "PASS" } else { "FAIL" }
    );

    let pass = fast_pass && reset_pass && f5_pass;
    if pass {
        println!("\n*** SPEED RESET TEST PASSED ***");
    } else {
        println!("\n*** SPEED RESET TEST FAILED ***");
    }
    pass
}
