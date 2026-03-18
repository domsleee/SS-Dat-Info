//! TAS test runner CLI for Supreme Snowboarding.
//!
//! Modes:
//!   smoke       — Basic REC/PLAY without F5 alignment
//!   f5          — F5-aligned straight-line REC/PLAY (zero-drift baseline)
//!   regression  — 15-case regression suite with CSV output
//!   acceptance  — 3-phase acceptance test (baseline, steered REC, PLAY)
//!   mock        — Regression suite with mock input (no Pico hardware)

mod acceptance;
mod cache;
mod certificate;
mod drift;
mod gates;
mod harness;
mod patterns;
mod regression;

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "smoke" => run_smoke_test(),
        "f5" => run_f5_aligned_test(),
        "regression" => {
            let out = output_dir();
            let cache_dir = out.join("regression_cache");
            let csv_path = out.join("regression_results.csv");
            let cert_path = out.join("regression_certificate.json");
            let results = regression::run(false, &cache_dir, &csv_path);
            certificate::write_regression_certificate(&results, false, &csv_path, &cert_path);
            let passed = results.iter().filter(|r| r.all_gates_pass).count();
            std::process::exit(if passed == results.len() { 0 } else { 1 });
        }
        "acceptance" => {
            let out = output_dir();
            let cert_path = out.join("acceptance_certificate.json");
            let result = acceptance::run(false);
            certificate::write_acceptance_certificate(&result, &cert_path);
            std::process::exit(if result.all_pass() { 0 } else { 1 });
        }
        "mock" => {
            let out = output_dir();
            let cache_dir = out.join("mock_cache");
            let csv_path = out.join("mock_results.csv");
            let cert_path = out.join("mock_certificate.json");
            let results = regression::run(true, &cache_dir, &csv_path);
            certificate::write_regression_certificate(&results, true, &csv_path, &cert_path);
            let passed = results.iter().filter(|r| r.all_gates_pass).count();
            std::process::exit(if passed == results.len() { 0 } else { 1 });
        }
        _ => {
            println!("Usage: tas_test <mode>");
            println!();
            println!("Modes:");
            println!("  smoke       Basic REC/PLAY without F5 alignment");
            println!("  f5          F5-aligned straight-line zero-drift check");
            println!("  regression  15-case regression suite (requires Pico HID)");
            println!("  acceptance  3-phase acceptance test (requires Pico HID)");
            println!("  mock        Regression suite with mock input (no hardware)");
            println!();
            println!("Exit code: 0 = all pass, 1 = some failed");
        }
    }
}

fn output_dir() -> PathBuf {
    // Default output next to the binary, or use TAS_TEST_OUTPUT env var
    std::env::var("TAS_TEST_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from("."))
        })
}

fn run_smoke_test() {
    println!("=== Smoke Test ===");
    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Cave 2 not firing");
        std::process::exit(1);
    }

    // REC 3s
    println!("\n--- REC 3s ---");
    harness::arm_rec(&mut client);
    std::thread::sleep(std::time::Duration::from_secs(3));
    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("Recorded {} ticks", rec_count);

    // PLAY
    println!("--- PLAY ---");
    harness::arm_play(&mut client);
    harness::wait_playback(&client, rec_count);
    harness::print_results(&client);
}

fn run_f5_aligned_test() {
    println!("=== F5-Aligned Zero-Drift Test ===");
    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Cave 2 not firing");
        std::process::exit(1);
    }

    // Phase 1: F5 + REC (straight line)
    println!("\n--- Phase 1: F5 + REC ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive after F5");
        std::process::exit(1);
    }

    harness::arm_rec(&mut client);
    println!("Recording 5s (straight line)...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("Recorded {} ticks", rec_count);

    if rec_count == 0 {
        eprintln!("ERROR: No ticks recorded");
        std::process::exit(1);
    }

    let s = client.state();
    let last = (rec_count - 1) as usize;
    println!(
        "First REC coord: ({:.4}, {:.4}, {:.4})",
        s.rec_coords[0][0], s.rec_coords[0][1], s.rec_coords[0][2]
    );
    println!(
        "Last REC coord [{}]: ({:.4}, {:.4}, {:.4})",
        last, s.rec_coords[last][0], s.rec_coords[last][1], s.rec_coords[last][2]
    );

    // Capture REC start position for matching
    let rec_start = client.state().rec_coords[0];

    // Phase 2: F5 + PLAY (match REC starting position via play_coords[0])
    println!("\n--- Phase 2: F5 + PLAY ---");
    if !harness::restart_play_and_match(&mut client, rec_start, 20) {
        eprintln!("WARNING: Could not match REC position for PLAY (continuing anyway)");
    }
    // Playback is already running from restart_play_and_match
    harness::wait_playback(&client, rec_count);

    // Debug: coordinate comparison and drift analysis
    let s = client.state();
    let n = rec_count as usize;
    println!("\n--- Starting position comparison ---");
    println!("  REC[0]: ({:.6}, {:.6}, {:.6})", s.rec_coords[0][0], s.rec_coords[0][1], s.rec_coords[0][2]);
    println!("  PLAY[0]: ({:.6}, {:.6}, {:.6})", s.play_coords[0][0], s.play_coords[0][1], s.play_coords[0][2]);
    println!("  Initial offset: dx={:.9} dy={:.9} dz={:.9}",
        (s.rec_coords[0][0] as f64 - s.play_coords[0][0] as f64).abs(),
        (s.rec_coords[0][1] as f64 - s.play_coords[0][1] as f64).abs(),
        (s.rec_coords[0][2] as f64 - s.play_coords[0][2] as f64).abs());

    // Drift progression at sample frames
    println!("\n--- Drift progression ---");
    for &frame in &[0, 5, 10, 50, 100, 200, 500, 1000, 1500] {
        if frame < n {
            let dx = (s.rec_coords[frame][0] as f64 - s.play_coords[frame][0] as f64).abs();
            let dz = (s.rec_coords[frame][2] as f64 - s.play_coords[frame][2] as f64).abs();
            println!("  [{}] dx={:.9} dz={:.9}", frame, dx, dz);
        }
    }

    harness::print_results(&client);

    // 4-gate assessment
    let assessment = gates::run_gates_straight(client.state(), rec_count);
    assessment.print_summary();

    std::process::exit(if assessment.all_pass() { 0 } else { 1 });
}
