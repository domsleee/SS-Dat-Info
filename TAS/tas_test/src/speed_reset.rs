//! Speed reset verification test (SSB-185).
//!
//! Validates that after using 2x speed and stopping, the game's time advance
//! constant is restored to its base value (0.01). Without the Cave 5 fix,
//! the constant stays at 0.005 after STOP, causing the game to run at 2x
//! in OFF mode — breaking ESC menus, F5 restart, and all non-TAS gameplay.
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

/// Duration for each tick rate measurement (seconds).
const MEASURE_SECS: u64 = 2;

/// Tolerance: post-stop tick rate vs baseline should be within this band.
/// 1.0 = identical to baseline. Allow 0.7–1.3 for timing jitter.
const RESET_RATIO_MIN: f64 = 0.7;
const RESET_RATIO_MAX: f64 = 1.3;

/// Tolerance: 2x REC tick rate vs baseline.
const FAST_RATIO_MIN: f64 = 1.6;
const FAST_RATIO_MAX: f64 = 2.4;

#[derive(Debug)]
pub struct SpeedResetResult {
    pub baseline_ticks: u32,
    pub fast_rec_ticks: u32,
    pub post_stop_ticks: u32,
    pub post_f5_ticks: u32,
    pub fast_ratio: f64,
    pub reset_ratio: f64,
    pub post_f5_ratio: f64,
    pub fast_pass: bool,
    pub reset_pass: bool,
    pub f5_pass: bool,
}

impl SpeedResetResult {
    pub fn all_pass(&self) -> bool {
        self.fast_pass && self.reset_pass && self.f5_pass
    }
}

/// Measure tick rate by counting Cave 2 frame_count delta over MEASURE_SECS.
/// This measures the game's actual tick processing rate, which is affected
/// by the time advance constant.
fn measure_tick_rate(client: &tas_shared::TasSharedMemoryClient) -> u32 {
    let start_fc = client.frame_count_volatile();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(MEASURE_SECS) {
        thread::sleep(Duration::from_millis(50));
    }
    let end_fc = client.frame_count_volatile();
    end_fc.saturating_sub(start_fc)
}

/// Run the speed reset verification test.
pub fn run() -> SpeedResetResult {
    println!("=== Speed Reset Verification Test (SSB-185) ===\n");

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::ensure_exclusive_runtime_ownership(&mut client, "speed reset validation failures");

    // Verify Cave 5 is hooked
    {
        let s = client.state();
        assert_eq!(s.cave5_hooked, 1, "Cave 5 must be hooked for speed test");
        assert_eq!(
            s.force_fixed_tick, 0,
            "force_fixed_tick must be 0 (would override speed)"
        );
    }

    // Phase 1: Baseline tick rate at 1x in OFF mode
    println!("--- Phase 1: Baseline tick rate (1.0x, OFF mode) ---");
    client.state_mut().playback_speed = 1.0;
    let baseline_ticks = measure_tick_rate(&client);
    println!("  Baseline: {} frames/{}s", baseline_ticks, MEASURE_SECS);
    if baseline_ticks == 0 {
        eprintln!("ERROR: No baseline ticks");
        std::process::exit(1);
    }

    // Phase 2: REC at 2.0x — verify 2x tick rate
    println!("\n--- Phase 2: REC at 2.0x speed ---");
    client.state_mut().playback_speed = 2.0;
    harness::arm_rec(&mut client);
    let fast_rec_ticks = measure_tick_rate(&client);
    let rec_count = client.state().recorded_count;
    println!(
        "  2x REC: {} frames/{}s, recorded {} ticks",
        fast_rec_ticks, MEASURE_SECS, rec_count
    );

    // Phase 3: STOP — mode goes to OFF, speed constant should reset
    println!("\n--- Phase 3: STOP (verify time constant resets) ---");
    harness::stop(&mut client);
    // Give Cave 5 a few frames to write the reset
    thread::sleep(Duration::from_millis(200));

    // Keep speed at 2.0 in shared state — the fix should still reset the
    // constant because mode == OFF. This is the key test: the DLL should
    // restore 0.01 based on mode, not speed value.
    let mode_after_stop = client.mode_volatile();
    println!("  Mode after stop: {} (expect 0=OFF)", mode_after_stop);

    // Measure tick rate in OFF mode (should be normal ~100/s, NOT 200/s)
    let post_stop_ticks = measure_tick_rate(&client);
    println!(
        "  Post-stop tick rate: {} frames/{}s",
        post_stop_ticks, MEASURE_SECS
    );

    // Phase 4: F5 restart — verify it works after 2x
    println!("\n--- Phase 4: F5 restart after 2x stop ---");
    client.state_mut().playback_speed = 1.0; // reset speed
    let f5_ok = harness::restart_and_stabilize(&client);
    println!("  F5 restart: {}", if f5_ok { "OK" } else { "FAILED" });

    let post_f5_ticks = if f5_ok {
        let t = measure_tick_rate(&client);
        println!("  Post-F5 tick rate: {} frames/{}s", t, MEASURE_SECS);
        t
    } else {
        0
    };

    // Compute ratios
    let fast_ratio = fast_rec_ticks as f64 / baseline_ticks as f64;
    let reset_ratio = post_stop_ticks as f64 / baseline_ticks as f64;
    let post_f5_ratio = if post_f5_ticks > 0 {
        post_f5_ticks as f64 / baseline_ticks as f64
    } else {
        0.0
    };

    let fast_pass = (FAST_RATIO_MIN..=FAST_RATIO_MAX).contains(&fast_ratio);
    let reset_pass = (RESET_RATIO_MIN..=RESET_RATIO_MAX).contains(&reset_ratio);
    let f5_pass = f5_ok && (RESET_RATIO_MIN..=RESET_RATIO_MAX).contains(&post_f5_ratio);

    let result = SpeedResetResult {
        baseline_ticks,
        fast_rec_ticks,
        post_stop_ticks,
        post_f5_ticks,
        fast_ratio,
        reset_ratio,
        post_f5_ratio,
        fast_pass,
        reset_pass,
        f5_pass,
    };

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

    if result.all_pass() {
        println!("\n*** SPEED RESET TEST PASSED ***");
    } else {
        println!("\n*** SPEED RESET TEST FAILED ***");
    }

    result
}
