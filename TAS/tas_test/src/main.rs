#![allow(dead_code)]
//! TAS test runner CLI for Supreme Snowboarding.
//!
//! Modes:
//!   smoke       — Basic REC/PLAY without F5 alignment
//!   f5          — F5-aligned straight-line REC/PLAY (zero-drift baseline)
//!   regression  — 15-case regression suite with CSV output
//!   acceptance  — 3-phase acceptance test (baseline, steered REC, PLAY)
//!   speed       — Playback speed verification (0.25x, 1x, 2x)
//!   speed-reset — Speed reset verification (2x stop restores normal)
//!   drift-speed — Drift-at-speed verification (2x same-speed, 1x/2x cross-speed)
//!   replay      — Load .tasrec file and replay N times, checking drift each time
//!   cont-reliability — CONT splice reliability test at long frame offsets

mod acceptance;
mod benchmark;
mod cache;
mod certificate;
mod cont_reliability;
mod cont_splice_frame;
mod cont_stress;
mod cont_restart_race;
mod drift;
mod drift_speed;
mod f5_probe;
mod gates;
mod harness;
mod patterns;
mod catchup_speed;
mod escape_speedup;
mod fe_cont_reliability;
mod fe10065_cont;
mod fe_cont_stress;
mod pause_resume;
mod play_pace;
mod rec_start;
mod refresh_recording;
mod regression;
mod reliability;
mod replay;
mod restart_probe;
mod snapshot_probe;
mod snapshot_cont;
mod cont_hijack;
mod save_reload;
mod steer_impact;
mod stop_play_flake;
mod speed;
mod speed_reset;

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match mode {
        "smoke" => run_smoke_test(),
        "f5" => run_f5_aligned_test(),
        "segment" => run_segment_test(),
        "regression" => {
            let out = output_dir();
            let cache_dir = out.join("regression_cache");
            let csv_path = out.join("regression_results.csv");
            let cert_path = out.join("regression_certificate.json");
            let results = regression::run(&cache_dir, &csv_path);
            certificate::write_regression_certificate(&results, &csv_path, &cert_path);
            let passed = results.iter().filter(|r| r.all_gates_pass).count();
            std::process::exit(if passed == results.len() { 0 } else { 1 });
        }
        "acceptance" => {
            let out = output_dir();
            let cert_path = out.join("acceptance_certificate.json");
            const ITERATIONS: u32 = 5;
            let mut last_result = None;
            let mut passed = 0u32;
            for i in 1..=ITERATIONS {
                println!("\n========== Acceptance run {}/{} ==========", i, ITERATIONS);
                let result = acceptance::run();
                if result.all_pass() {
                    passed += 1;
                    println!("Acceptance run {}/{} PASSED", i, ITERATIONS);
                } else {
                    println!("Acceptance run {}/{} FAILED — aborting", i, ITERATIONS);
                    last_result = Some(result);
                    break;
                }
                last_result = Some(result);
            }
            if let Some(result) = last_result.as_ref() {
                certificate::write_acceptance_certificate(result, &cert_path);
            }
            println!(
                "\n=== Acceptance: {}/{} runs passed ===",
                passed, ITERATIONS
            );
            std::process::exit(if passed == ITERATIONS { 0 } else { 1 });
        }
        "speed" => {
            let result = speed::run();
            std::process::exit(if result.all_pass() { 0 } else { 1 });
        }
        "speed-reset" => {
            let result = speed_reset::run();
            std::process::exit(if result.all_pass() { 0 } else { 1 });
        }
        "drift-speed" => {
            let result = drift_speed::run();
            std::process::exit(if result.all_pass() { 0 } else { 1 });
        }
        "save-reload" => {
            // End-to-end test: record → save to disk → kill game → revive →
            // reload from disk → replay → verify zero drift. The one workflow
            // tas_ui actually exercises that no other test mode covers.
            let ok = save_reload::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "pause-resume" => {
            // Record → start playback → Escape (pause) → wait → Escape
            // (resume — game fast-forwards) → finish playback → verify the
            // first 1000 frames replay with zero drift across the pause
            // boundary.
            let ok = pause_resume::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "steer-impact" => {
            // Injected steering must move the player (BB3B10 arg4 / inject-path
            // guard). Focus-free + warmup-free → fails on stale arg4, passes
            // once arg4 is sourced live from game memory.
            let ok = steer_impact::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "rec-start" => {
            // Start-position regression guard: a fresh recording must BEGIN at
            // the stationary spawn (capturing the countdown / pre-timer inputs),
            // not mid-fall. `--file <path>` analyzes a saved .tasrec read-only.
            let mut file: Option<String> = None;
            let mut i = 2;
            while i < args.len() {
                if args[i] == "--file" && i + 1 < args.len() {
                    file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let ok = match file {
                Some(f) => rec_start::check_file(&f),
                None => rec_start::run(),
            };
            std::process::exit(if ok { 0 } else { 1 });
        }
        "catchup-speed" => {
            // Catch-up SPEED guard: median time-to-splice at 1× and 64×, assert
            // the ratio (64× must stay much faster than native). The ratio
            // cancels the harness's ~2× wall-clock environment, so it is safe to
            // assert where an absolute wall-time would not be.
            let ok = catchup_speed::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "play-pace" => {
            // Playback-pace guard: 1× PLAY of a fixed frame window must take
            // ~native wall time (frames×0.01s). Catches "skip-ahead" (PLAY
            // running faster than real time). 1× PLAY is clock-gated, so unlike
            // most timing here the absolute pace is a sound assertion.
            let ok = play_pace::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "escape-speedup" => {
            // Press Escape → wait 20s → press Escape → wait 5s. Verify the
            // ticker (cave2 frame_count) behaves sensibly: ~0 ticks during
            // the pause window, normal ~100 tps during the resume window.
            // Catches the originally reported "fast-forward on resume" bug.
            let ok = escape_speedup::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
"cont-restart-race" => {
            // Verifies the cave2 contract that tas_ui's Stop→Restart
            // serialisation depends on: confirms (1) sending Stop + Restart
            // back-to-back loses the Stop, so cave2 still sees REC/PLAY
            // mode when ArmContinue arrives, and (2) sending Stop, waiting
            // for mode==OFF, then sending Restart cleanly gets ArmContinue
            // accepted. If half 2 fails, tas_ui's CONT-twice fix is broken.
            let ok = cont_restart_race::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "fe-cont-stress" => {
            // Speed sweep for CONT splice against FE-tremendous, using
            // reference-comparison (first splice's prefix is the truth)
            // instead of strict bit-match-vs-rec_coords. Reports the
            // fastest speed where N iterations match the reference.
            let mut speeds: Vec<f32> = vec![1.0, 12.0, 32.0, 64.0, 128.0, 256.0];
            // Optional override: tas_test fe-cont-stress 512 1024 ...
            let extra: Vec<f32> = args
                .iter()
                .skip(2)
                .filter_map(|s| s.parse::<f32>().ok())
                .collect();
            if !extra.is_empty() {
                speeds = extra;
            }
            let ok = fe_cont_stress::run(&speeds);
            std::process::exit(if ok { 0 } else { 1 });
        }
        "fe-cont-reliability" => {
            // CONT splice against TAS/recordings/FE-tremendous.tasrec at
            // frame 2200, 5 iterations at 12x catchup. Pinned variant of
            // cont-reliability for the specific recording the user cares
            // about; passes only if all 5/5 splices are zero-drift.
            let ok = fe_cont_reliability::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "fe10065-cont" => {
            // CONT splice against TAS/recordings/FE-10065.tasrec at frame 6200,
            // 12 iterations at 64x. Pinned to the user's real case; passes only
            // if all splices are zero-drift AND the resume lands within a few
            // frames of the splice (the Problem B resume-timing guard).
            let ok = fe10065_cont::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "stop-play-flake" => {
            // Load FE-tremendous, PLAY → STOP at a varying mid-playback
            // frame → PLAY again, verify zero drift over the first 1000
            // frames of the second playback. 10 iterations with stop
            // points spread across the recording to surface intermittent
            // post-STOP replay drift.
            let ok = stop_play_flake::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "f5-probe" => {
            let mut iterations = 50u32;
            let mut i = 2;
            while i < args.len() {
                if (args[i] == "--iterations" || args[i] == "-n")
                    && i + 1 < args.len()
                {
                    iterations = args[i + 1].parse().unwrap_or(50);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            f5_probe::run(iterations);
            std::process::exit(0);
        }
        "benchmark" => {
            let mut config = benchmark::BenchmarkConfig::default();
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--repeats" | "-n" => {
                        config.repeats = args
                            .get(i + 1)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(config.repeats);
                        i += 2;
                    }
                    "--frames" | "-f" => {
                        config.measure_frames = args
                            .get(i + 1)
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(config.measure_frames);
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let result = benchmark::run(config);
            std::process::exit(if result.is_ok() { 0 } else { 1 });
        }
        "replay" => {
            let path = args.get(2).unwrap_or_else(|| {
                eprintln!("Usage: tas_test replay <path.tasrec> [--iterations N] [--verbose]");
                std::process::exit(1);
            });
            let mut iterations = 5u32;
            let mut verbose = false;
            let mut no_match = false;
            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--iterations" | "-n" => {
                        iterations = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(5);
                        i += 2;
                    }
                    "--verbose" | "-v" => {
                        verbose = true;
                        i += 1;
                    }
                    "--no-match" => {
                        no_match = true;
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let report = replay::run(path, iterations, verbose, no_match);
            // Exit non-zero if ANY iteration: had drift, didn't complete
            // playback, or (unless --no-match was requested) failed position
            // matching. The previous "drift only" check was misleading
            // because compute_drift inspects only the frames that played —
            // a run that never started returns 0.0 drift and would have
            // exited 0, hiding real failures from CI.
            let any_failure = report.results.iter().any(|r| {
                let drifted = r.max_drift_x > 0.0 || r.max_drift_z > 0.0;
                let pos_failed = !no_match && !r.position_matched;
                drifted || !r.playback_complete || pos_failed
            });
            std::process::exit(if any_failure { 1 } else { 0 });
        }
        "refresh-tasrec" => {
            let source = args.get(2).unwrap_or_else(|| {
                eprintln!("Usage: tas_test refresh-tasrec <source.tasrec> <out.tasrec>");
                std::process::exit(1);
            });
            let out = args.get(3).unwrap_or_else(|| {
                eprintln!("Usage: tas_test refresh-tasrec <source.tasrec> <out.tasrec>");
                std::process::exit(1);
            });
            match refresh_recording::run(source, out) {
                Ok(()) => std::process::exit(0),
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    std::process::exit(1);
                }
            }
        }
        "reliability" => {
            let mut iterations = 10u32;
            let mut speed = 12.0f32;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--iterations" | "-n" => {
                        iterations = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(10);
                        i += 2;
                    }
                    "--speed" | "-s" => {
                        speed = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(12.0);
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let report = reliability::run(iterations, speed);
            std::process::exit(if report.all_pass() { 0 } else { 1 });
        }
        "restart-probe" => {
            // Measure the in-process F5 restart timeline (F5-press -> teleport
            // -> first motion) to tell whether the short first-moving is a
            // truncated countdown or an arm-timing offset.
            let ok = restart_probe::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "snapshot-probe" => {
            // PROTOTYPE: validate in-DLL writable-memory snapshot/restore as the
            // instant-CONT mechanism (libTAS/TMInterface design). Go/no-go for
            // the state-snapshot project.
            let ok = snapshot_probe::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "snapshot-cont" => {
            // PROTOTYPE: snapshot-based CONT — establish a frame-0 spawn snapshot
            // (one-time F5 lottery), then RESTORE it instead of F5 for every
            // replay → zero rerolls (bucket lottery eliminated).
            let ok = snapshot_cont::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "cont-hijack" => {
            // Bug #2 regression: a continue_from_frame set during a plain PLAY
            // must NOT hijack the replay into REC (g_cave2_contArmed gate).
            let ok = cont_hijack::run();
            std::process::exit(if ok { 0 } else { 1 });
        }
        "gamestate" => {
            // Validate the game_in_game exposure: revive (in-game) + inject, then
            // print state a few times (game_in_game should read 1 in-game).
            let client = harness::ensure_game_running();
            for _ in 0..6 {
                harness::print_status(&client);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        "cont-reliability" => {
            let mut iterations = 10u32;
            let mut speed = 12.0f32;
            let mut splice = 2200u32; // FE-tremendous's proven splice frame
            let mut file: Option<String> = None;
            let mut synthetic = false;
            let mut profile = cont_reliability::BaselineInputProfile::Taps;
            let mut tap_ticks: Option<u32> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--iterations" | "-n" => {
                        iterations = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(10);
                        i += 2;
                    }
                    "--speed" | "-s" => {
                        speed = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(12.0);
                        i += 2;
                    }
                    "--splice" => {
                        splice = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(2400);
                        i += 2;
                    }
                    "--file" => {
                        file = args.get(i + 1).cloned();
                        i += 2;
                    }
                    "--synthetic" => {
                        // Force a fresh REC baseline (bucket-compatible with the
                        // current in-process restart) instead of the FE default.
                        synthetic = true;
                        i += 1;
                    }
                    "--profile" => {
                        let raw = args.get(i + 1).map(|s| s.as_str()).unwrap_or("");
                        profile = match cont_reliability::BaselineInputProfile::parse(raw) {
                            Some(p) => p,
                            None => {
                                eprintln!(
                                    "ERROR: invalid --profile '{}'; expected 'taps' or 'sweep'",
                                    raw
                                );
                                std::process::exit(1);
                            }
                        };
                        i += 2;
                    }
                    "--tap-ticks" => {
                        let parsed = args.get(i + 1).and_then(|s| s.parse::<u32>().ok());
                        tap_ticks = Some(parsed.unwrap_or(8).max(1));
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            // Default to the real FE-tremendous recording (the user's actual
            // workflow) when no --file is given — a real steered run makes the
            // drift check meaningful. Falls back to a synthetic baseline only if
            // the recording can't be located.
            let file = if synthetic {
                None
            } else {
                file.or_else(|| {
                    let exe_dir = std::env::current_exe()
                        .ok()
                        .and_then(|p| p.parent().map(PathBuf::from))
                        .unwrap_or_else(|| PathBuf::from("."));
                    let candidates = [
                        exe_dir.join("../../..").join("TAS/recordings/FE-tremendous.tasrec"),
                        PathBuf::from("TAS/recordings/FE-tremendous.tasrec"),
                        PathBuf::from("recordings/FE-tremendous.tasrec"),
                    ];
                    candidates
                        .iter()
                        .find(|p| p.exists())
                        .map(|p| p.to_string_lossy().into_owned())
                })
            };
            match &file {
                Some(p) => println!("  Baseline: real recording {} (default)", p),
                None => println!("  Baseline: synthetic (FE-tremendous not found)"),
            }
            let report = cont_reliability::run(
                iterations,
                speed,
                splice,
                file.as_deref(),
                profile,
                tap_ticks,
            );
            std::process::exit(if report.all_pass() { 0 } else { 1 });
        }
        "cont-stress" => {
            // The user's exact CONT flow: slow play speed, fast catch-up,
            // repeated F12. Measures the reroll distribution.
            let mut iterations = 5u32;
            let mut splice = 1000u32;
            let mut catchup = 64.0f32;
            let mut record_speed = 0.5f32;
            let mut max_median = 8u32;
            let mut file: Option<String> = None;
            let mut restart = "inprocess".to_string();
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--iterations" | "-n" => {
                        iterations = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(5);
                        i += 2;
                    }
                    "--restart" => {
                        restart = args.get(i + 1).cloned().unwrap_or_else(|| "inprocess".into());
                        i += 2;
                    }
                    "--splice" => {
                        splice = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                        i += 2;
                    }
                    "--catchup" => {
                        catchup = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(64.0);
                        i += 2;
                    }
                    "--record-speed" => {
                        record_speed = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0.5);
                        i += 2;
                    }
                    "--max-median-rerolls" => {
                        max_median = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(8);
                        i += 2;
                    }
                    "--file" => {
                        file = args.get(i + 1).cloned();
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let ok = cont_stress::run(
                iterations,
                splice,
                catchup,
                record_speed,
                max_median,
                file.as_deref(),
                &restart,
            );
            std::process::exit(if ok { 0 } else { 1 });
        }
        "cont-splice-frame" => {
            // "Continue continues on the correct frame": CONT at N must splice
            // EXACTLY at frame N. Realistic profile — catch up fast (64x) then
            // record slow (0.25x), so the splice frame is precise while catch-up
            // stays quick.
            let mut splice = 1000u32;
            let mut catchup = 64.0f32;
            let mut record_speed = 0.25f32;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--splice" => {
                        splice = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                        i += 2;
                    }
                    "--catchup" => {
                        catchup = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(64.0);
                        i += 2;
                    }
                    "--record-speed" => {
                        record_speed = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0.25);
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            let ok = cont_splice_frame::run(splice, catchup, record_speed);
            std::process::exit(if ok { 0 } else { 1 });
        }
        _ => {
            println!("Usage: tas_test <mode>");
            println!();
            println!("Modes:");
            println!("  smoke       Basic REC/PLAY without F5 alignment");
            println!("  f5          F5-aligned straight-line zero-drift check");
            println!("  segment     Multi-segment CONT zero-drift test (requires Pico HID)");
            println!("  regression  15-case regression suite (requires Pico HID)");
            println!("  acceptance  3-phase acceptance test (requires Pico HID)");
            println!("  speed       Playback speed verification (0.25x, 1x, 2x)");
            println!("  speed-reset Speed reset verification (2x stop restores normal)");
            println!("  drift-speed Drift-at-speed verification (2x same, 1x/2x cross)");
            println!("  f5-probe    F5 bucket characterization (records starting positions)");
            println!("  benchmark   Cave hook perf benchmark (frame-window repeats)");
            println!(
                "  reliability N consecutive REC+PLAY cycles at Nx speed (default 10x at 12x)"
            );
            println!(
                "  cont-reliability CONT splice reliability (default 10x, splice 2400 @ 12x; profile=taps)"
            );
            println!(
                "  cont-splice-frame CONT splices on the EXACT requested frame (default splice 1000 @ 0.25x)"
            );
            println!("  replay      Load .tasrec file and play back N times (drift check)");
            println!("  refresh-tasrec Re-record a .tasrec baseline from live runtime");
            println!();
            println!("Options for replay:");
            println!("  --iterations N  Number of playback iterations (default: 5)");
            println!("  --verbose       Show drift details for every iteration");
            println!();
            println!("Usage for refresh-tasrec:");
            println!("  refresh-tasrec <source.tasrec> <out.tasrec>");
            println!();
            println!("Options for cont-reliability:");
            println!("  --file PATH     Load baseline from .tasrec instead of fresh REC");
            println!("  --profile NAME  Synthetic baseline profile: taps (default) or sweep");
            println!("  --tap-ticks N   Tick hold per tap for profile=taps (default: 8)");
            println!();
            println!("Options for benchmark:");
            println!("  --repeats N     Number of benchmark repeats (default: 3)");
            println!("  --frames N      Frames per scenario window (default: 600)");
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

/// Multi-segment E2E zero-drift test (SSB-131).
///
/// Strategy: Use F5 position matching via ARM_PLAY to find a matching position,
/// then immediately reuse that F5 for ARM_CONTINUE (no second restart needed).
///
/// Phase 1: REC segment 0 with LEFT steering
/// Phase 2: F5-match via PLAY (establishes matching position), then ARM_CONTINUE
///          for segment 1 with RIGHT steering
/// Phase 3: PLAY full recording, verify zero drift at segment boundary
fn run_segment_test() {
    println!("=== Multi-Segment E2E Zero-Drift Test (SSB-131) ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // ---- Phase 1: REC segment 0 with LEFT steering ----
    println!("\n--- Phase 1: REC segment 0 (LEFT steering) ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive for Phase 1");
        std::process::exit(1);
    }

    harness::arm_rec(&mut client);
    println!("  Recording with LEFT steering via Pico HID...");

    // Drive LEFT for 200 ticks, then neutral for 100 ticks (total 300)
    let seg0_steps = patterns::build_from_explicit(&[
        ("LEFT", tas_shared::input_bits::LEFT, 200),
        ("NEUTRAL", 0x00, 100),
    ]);
    drive_pico_steps(&seg0_steps);

    let seg0_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Segment 0 recorded: {} ticks", seg0_count);

    if seg0_count < 200 {
        eprintln!("ERROR: Too few ticks in segment 0 (need >= 200)");
        std::process::exit(1);
    }

    let rec_start = client.state().rec_coords[0];
    println!(
        "  REC start: ({:.4}, {:.4}, {:.4})",
        rec_start[0], rec_start[1], rec_start[2]
    );
    println!("  Segments before CONT: {}", client.state().segment_count);

    let splice_frame: u32 = 200;

    // ---- Phase 2: CONT from frame 200 ----
    // Strategy: F5-restart loop until position matches, then ARM_CONTINUE
    // (not ARM_PLAY). This uses the same matching logic but starts CONT directly.
    println!(
        "\n--- Phase 2: CONT from frame {} (RIGHT steering) ---",
        splice_frame
    );

    // Set continue_from_frame before starting CONT retries
    let matched = harness::restart_continue_and_splice(
        &mut client,
        rec_start,
        splice_frame,
        30, // more retries
    );
    if matched.is_none() {
        eprintln!("ERROR: Could not position-match for CONT after retries");
        std::process::exit(1);
    }

    // Now we're in REC mode at the splice point — send RIGHT steering
    println!("  Recording segment 1 with RIGHT steering via Pico HID...");
    let seg1_steps = patterns::build_from_explicit(&[
        ("RIGHT", tas_shared::input_bits::RIGHT, 200),
        ("NEUTRAL", 0x00, 100),
    ]);
    drive_pico_steps(&seg1_steps);

    let total_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Total recorded after CONT: {} ticks", total_count);
    println!("  Segment count: {}", client.state().segment_count);

    // Print segment boundaries
    let state = client.state();
    for i in 0..state.segment_count as usize {
        let b = &state.segment_boundaries[i];
        println!(
            "  Boundary[{}]: frame={} input_log_offset={}",
            i, b.frame, b.input_log_offset
        );
    }

    // Verify input log has both LEFT and RIGHT
    let mut has_left = false;
    let mut has_right = false;
    for i in 0..total_count as usize {
        if state.input_log[i] & tas_shared::input_bits::LEFT != 0 {
            has_left = true;
        }
        if state.input_log[i] & tas_shared::input_bits::RIGHT != 0 {
            has_right = true;
        }
    }
    println!(
        "  Input log check: has_left={} has_right={} (both expected)",
        has_left, has_right
    );

    if !has_left || !has_right {
        eprintln!("ERROR: Input log missing expected LEFT or RIGHT inputs");
        std::process::exit(1);
    }

    // ---- Phase 3: PLAY full recording ----
    println!(
        "\n--- Phase 3: PLAY full recording ({} ticks) ---",
        total_count
    );

    if !harness::restart_play_and_match(&mut client, rec_start, harness::START_MATCH_RETRIES) {
        eprintln!("WARNING: Could not match position for PLAY (continuing anyway)");
    }
    let play_ok = harness::wait_playback(&client, total_count);
    if !play_ok {
        eprintln!("WARNING: Playback did not complete normally");
    }

    // ---- Results & Verification ----
    harness::print_results(&client);

    let state = client.state();

    // Check drift at segment boundary specifically
    if splice_frame < total_count {
        let sf = splice_frame as usize;
        let boundary_drift_x =
            (state.rec_coords[sf][0] as f64 - state.play_coords[sf][0] as f64).abs();
        let boundary_drift_z =
            (state.rec_coords[sf][2] as f64 - state.play_coords[sf][2] as f64).abs();
        println!("\n--- Segment Boundary (frame {}) ---", splice_frame);
        println!(
            "  REC[{}]:  ({:.6}, {:.6}, {:.6})",
            sf, state.rec_coords[sf][0], state.rec_coords[sf][1], state.rec_coords[sf][2]
        );
        println!(
            "  PLAY[{}]: ({:.6}, {:.6}, {:.6})",
            sf, state.play_coords[sf][0], state.play_coords[sf][1], state.play_coords[sf][2]
        );
        println!(
            "  Boundary drift: X={:.9} Z={:.9}",
            boundary_drift_x, boundary_drift_z
        );
        if boundary_drift_x == 0.0 && boundary_drift_z == 0.0 {
            println!("  Boundary check: PASS (zero discontinuity)");
        } else {
            println!("  Boundary check: FAIL (drift at segment boundary)");
        }
    }

    // 4-gate assessment
    let assessment = gates::run_gates(state, total_count);
    assessment.print_summary();

    // Print final verdict
    println!("\n=== SEGMENT TEST VERDICT ===");
    if assessment.all_pass() {
        println!("*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***");
        println!(
            "  {} ticks, {} segments, splice at frame {}",
            total_count, state.segment_count, splice_frame
        );
    } else {
        println!("*** MULTI-SEGMENT ZERO-DRIFT TEST FAILED ***");
    }

    std::process::exit(if assessment.all_pass() { 0 } else { 1 });
}

/// Drive Pico HID through a sequence of pattern steps (delegates to harness).
fn drive_pico_steps(steps: &[patterns::PatternStep]) {
    harness::drive_pico_steps(steps, None);
}

fn run_smoke_test() {
    println!("=== Smoke Test ===");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

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
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

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
    if !harness::restart_play_and_match(&mut client, rec_start, harness::START_MATCH_RETRIES) {
        eprintln!("WARNING: Could not match REC position for PLAY (continuing anyway)");
    }
    // Playback is already running from restart_play_and_match
    harness::wait_playback(&client, rec_count);

    // Debug: coordinate comparison and drift analysis
    let s = client.state();
    let n = rec_count as usize;
    println!("\n--- Starting position comparison ---");
    println!(
        "  REC[0]: ({:.6}, {:.6}, {:.6})",
        s.rec_coords[0][0], s.rec_coords[0][1], s.rec_coords[0][2]
    );
    println!(
        "  PLAY[0]: ({:.6}, {:.6}, {:.6})",
        s.play_coords[0][0], s.play_coords[0][1], s.play_coords[0][2]
    );
    println!(
        "  Initial offset: dx={:.9} dy={:.9} dz={:.9}",
        (s.rec_coords[0][0] as f64 - s.play_coords[0][0] as f64).abs(),
        (s.rec_coords[0][1] as f64 - s.play_coords[0][1] as f64).abs(),
        (s.rec_coords[0][2] as f64 - s.play_coords[0][2] as f64).abs()
    );

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
