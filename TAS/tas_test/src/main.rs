//! TAS test runner CLI for Supreme Snowboarding.
//!
//! [`MODES`] is the one list of modes: it is the dispatcher and the source of
//! `tas_test help`, so a mode cannot exist without its help line. Every mode
//! exits 0 on pass, 1 on fail and 2 on a command-line error.

mod acceptance;
mod catchup_speed;
mod certificate;
mod cli;
mod cont_cases;
mod cont_hijack;
mod cont_reliability;
mod cont_restart_race;
mod cont_ui;
mod dialog_e2e;
mod drift;
mod drift_speed;
mod f5;
mod gate_align;
mod gates;
mod harness;
mod level_seq;
mod live_suite;
mod load;
mod menu;
mod patterns;
mod pause_resume;
mod play_judge;
mod play_pace;
mod rec_repro;
mod rec_start;
mod refresh_recording;
mod regression;
mod reliability;
mod replay;
mod save_reload;
mod segment;
mod shm;
mod smoke;
mod speed;
mod speed_reset;
mod steer_impact;
mod stop_play_flake;
mod video_rate;
mod win32;

use std::path::PathBuf;

use cli::{flag, pair, switch, usage_error, Flags, Spec};

struct Mode {
    name: &'static str,
    /// Argument synopsis printed after the name.
    usage: &'static str,
    summary: &'static str,
    /// Receives the arguments after the mode name; returns pass/fail.
    run: fn(&[String]) -> bool,
}

const MODES: &[Mode] = &[
    Mode {
        name: "live",
        usage: "[--recording PATH] [--splice N] [--iterations N]",
        summary: "cont-ui-left-spam, then acceptance, then regression as child processes",
        run: |args| report(live_suite::run(args, output_dir()), "LIVE SUITE FAILED"),
    },
    Mode {
        name: "cont-ui-left-spam",
        usage: "[--recording PATH] [--splice N] [--iterations N]",
        summary: "Isolated tas_ui: F12 CONT with physical Pico LEFT taps, zero splice mismatch",
        run: |args| report(cont_ui::run(args), "CONT UI LEFT-SPAM FAILED"),
    },
    Mode {
        name: "acceptance",
        usage: "[N]",
        summary: "Unsteered baseline REC, Pico-steered REC that differs, PLAY that matches; N runs",
        run: run_acceptance,
    },
    Mode {
        name: "regression",
        usage: "",
        summary: "15 scripted steering patterns, REC then PLAY with zero drift; CSV + certificate",
        run: run_regression,
    },
    Mode {
        name: "smoke",
        usage: "",
        summary: "Pipeline liveness: ticks captured, playback completes, player moves (not F5-aligned)",
        run: |args| no_args(args) && smoke::run(),
    },
    Mode {
        name: "f5",
        usage: "",
        summary: "F5-aligned straight-line REC then PLAY through the gate checks",
        run: |args| no_args(args) && f5::run(),
    },
    Mode {
        name: "segment",
        usage: "",
        summary: "Two-segment CONT (LEFT, F5-matched CONT into RIGHT) with zero boundary drift",
        run: |args| no_args(args) && segment::run(),
    },
    Mode {
        name: "replay",
        usage: "<file.tasrec> [--iterations N] [--verbose] [--no-match]",
        summary: "Replay a .tasrec N times; drift, incomplete playback or a failed start match fails",
        run: run_replay,
    },
    Mode {
        name: "reliability",
        usage: "[--iterations N] [--speed X]",
        summary: "N consecutive steered REC+PLAY cycles at one speed (default 10 at 12x)",
        run: |args| {
            let flags = parse(args, &[flag("--iterations", Some("-n")), flag("--speed", Some("-s"))], 0);
            reliability::run(num(&flags, "--iterations", 10), num(&flags, "--speed", 12.0))
        },
    },
    Mode {
        name: "drift-speed",
        usage: "",
        summary: "REC 2x/PLAY 2x and REC 1x/PLAY 2x both replay with zero drift",
        run: |args| no_args(args) && drift_speed::run(),
    },
    Mode {
        name: "save-reload",
        usage: "",
        summary: "Steered REC, save, kill and relaunch the game, reload, replay with zero drift",
        run: |args| no_args(args) && save_reload::run(),
    },
    Mode {
        name: "pause-resume",
        usage: "",
        summary: "Escape pause and resume during PLAY; the replay stays bit-identical",
        run: |args| no_args(args) && pause_resume::run(),
    },
    Mode {
        name: "stop-play-flake",
        usage: "",
        summary: "PLAY, STOP at varying frames, PLAY again; second playbacks match a reference",
        run: |args| no_args(args) && stop_play_flake::run(),
    },
    Mode {
        name: "rec-start",
        usage: "[--file PATH]",
        summary: "A fresh recording starts at the stationary spawn; --file judges a saved .tasrec",
        run: |args| {
            let flags = parse(args, &[flag("--file", None)], 0);
            match flags.value("--file") {
                Some(file) => rec_start::check_file(file),
                None => rec_start::run(),
            }
        },
    },
    Mode {
        name: "rec-repro",
        usage: "",
        summary: "The same driven input recorded twice yields the same transitions",
        run: |args| no_args(args) && rec_repro::run(),
    },
    Mode {
        name: "steer-impact",
        usage: "",
        summary: "Injected steering moves the player, and only with a live Kernel::Time stamp",
        run: |args| no_args(args) && steer_impact::run(),
    },
    Mode {
        name: "refresh-tasrec",
        usage: "<source.tasrec> <out.tasrec>",
        summary: "Replay a recording live and write it back with fresh coordinates",
        run: |args| {
            let flags = parse(args, &[], 2);
            let [source, out] = flags.positional.as_slice() else {
                usage_error("refresh-tasrec needs <source.tasrec> <out.tasrec>");
            };
            report(refresh_recording::run(source, out), "ERROR")
        },
    },
    Mode {
        name: "speed",
        usage: "",
        summary: "0.25x and 2x tick counts scale against 1x; speed is back at 1x afterwards",
        run: |args| no_args(args) && speed::run(),
    },
    Mode {
        name: "speed-reset",
        usage: "",
        summary: "After 2x REC and STOP the OFF-mode tick rate and F5 are back to normal",
        run: |args| no_args(args) && speed_reset::run(),
    },
    Mode {
        name: "catchup-speed",
        usage: "",
        summary: "Median time-to-splice at 64x versus 1x stays above the required ratio",
        run: |args| no_args(args) && catchup_speed::run(),
    },
    Mode {
        name: "play-pace",
        usage: "",
        summary: "1x PLAY of a fixed frame window takes native wall time",
        run: |args| no_args(args) && play_pace::run(),
    },
    Mode {
        name: "play-judge",
        usage: "[--iterations N]",
        summary: "A judged PLAY replays the countdown at catch-up speed and hands back exactly",
        run: |args| {
            let flags = parse(args, &[flag("--iterations", Some("-n"))], 0);
            play_judge::run(num(&flags, "--iterations", 5))
        },
    },
    Mode {
        name: "video-rate",
        usage: "[secs] [--at X Y]",
        summary: "Distinct frames per second reaching the screen, measured from outside the process",
        run: |args| {
            let flags = parse(args, &[pair("--at")], 1);
            let secs = positional_num(&flags, 0);
            let region = flags.values("--at").map(|xy| {
                let at = |raw: &str| {
                    raw.parse::<i32>()
                        .unwrap_or_else(|_| usage_error(format!("--at: expected a number, got '{raw}'")))
                };
                (at(&xy[0]), at(&xy[1]))
            });
            video_rate::run(secs, region)
        },
    },
    Mode {
        name: "dialog-e2e",
        usage: "",
        summary: "Real finishes with a Pico Escape on the save dialog, then the main menu speed",
        run: |args| no_args(args) && dialog_e2e::run(),
    },
    Mode {
        name: "cont-reliability",
        usage: "[--iterations N] [--speed X] [--splice N] [--file PATH | --synthetic] [--profile taps|sweep] [--tap-ticks N]",
        summary: "Repeated CONT splices: zero prefix drift, coverage and forward progress (default FE-tremendous @2200)",
        run: run_cont_reliability,
    },
    Mode {
        name: "fe-cont-reliability",
        usage: "",
        summary: "FE-tremendous, five splices at 2200 at 12x",
        run: |args| no_args(args) && cont_cases::run(&cont_cases::FE_TREMENDOUS),
    },
    Mode {
        name: "fe10065-cont",
        usage: "",
        summary: "FE-10065, eight splices at 6200 at 64x and 256x with resume-timing limits",
        run: |args| no_args(args) && cont_cases::run(&cont_cases::FE_10065),
    },
    Mode {
        name: "cont-hijack",
        usage: "",
        summary: "A continue_from_frame written during a plain PLAY leaves it in PLAY",
        run: |args| no_args(args) && cont_hijack::run(),
    },
    Mode {
        name: "cont-restart-race",
        usage: "",
        summary: "Stop+Restart together lose the Stop; Stop, wait for OFF, then Restart is accepted",
        run: |args| no_args(args) && cont_restart_race::run(),
    },
    Mode {
        name: "cont-input-protection",
        usage: "",
        summary: "Live input stays blocked through the CONT restart and STOP releases it",
        run: |args| no_args(args) && cont_restart_race::run_input_protection(),
    },
    Mode {
        name: "gate-align",
        usage: "[N] [recording]",
        summary: "Gate-relative input indexing reproduces the run over N aligned replays",
        run: |args| {
            let flags = parse(args, &[], 2);
            let iterations = positional_num(&flags, 0).unwrap_or(8);
            gate_align::run(iterations, flags.positional.get(1).map(String::as_str))
        },
    },
    Mode {
        name: "shm",
        usage: "[--command record|play|stop|restart]",
        summary: "Version-checked shared-memory diagnostics; read-only by default",
        run: |args| report(shm::run(args), "SHM"),
    },
    Mode {
        name: "load",
        usage: "<file.tasrec>",
        summary: "Write a recording into shared memory and exit; refuses while REC/PLAY is active",
        run: |args| {
            let flags = parse(args, &[], 1);
            let Some(path) = flags.positional.first() else {
                usage_error("load needs <file.tasrec>");
            };
            load::run(path)
        },
    },
    Mode {
        name: "menu",
        usage: "[activate <id|label> | focus <id|label> | up | down | left | right | trigger]",
        summary: "Print the menu document, or drive the menu through the game's own entry points",
        run: |args| {
            let flags = parse(args, &[], 2);
            let arg = |i: usize| flags.positional.get(i).map(String::as_str);
            menu::run(arg(0), arg(1))
        },
    },
    Mode {
        name: "gamestate",
        usage: "",
        summary: "Launch or reuse the game and print the status six times (any track)",
        run: |args| {
            no_args(args) && {
                allow_any_level();
                let client = harness::ensure_game_running();
                for _ in 0..6 {
                    harness::print_status(&client);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                true
            }
        },
    },
    Mode {
        name: "level-seq",
        usage: "[secs] | watch [secs]",
        summary: "The DLL publishes level context through the seqlock; watch logs transitions instead",
        run: |args| {
            allow_any_level();
            let flags = parse(args, &[], 2);
            if flags.positional.first().map(String::as_str) == Some("watch") {
                level_seq::watch(positional_num(&flags, 1))
            } else if flags.positional.len() > 1 {
                usage_error("level-seq takes [secs] or watch [secs]");
            } else {
                level_seq::run(positional_num(&flags, 0))
            }
        },
    },
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let name = args.first().map(String::as_str).unwrap_or("help");
    if matches!(name, "help" | "--help" | "-h") {
        print_help();
        return;
    }
    let Some(mode) = MODES.iter().find(|m| m.name == name) else {
        eprintln!("ERROR: unknown mode '{name}'\n");
        print_help();
        std::process::exit(2);
    };
    let passed = (mode.run)(&args[1..]);
    std::process::exit(if passed { 0 } else { 1 });
}

fn print_help() {
    println!("Usage: tas_test <mode> [args]\n");
    println!("Modes:");
    for mode in MODES {
        let usage = if mode.usage.is_empty() {
            String::new()
        } else {
            format!(" {}", mode.usage)
        };
        println!("  {}{}", mode.name, usage);
        println!("      {}", mode.summary);
    }
    println!("\nExit code: 0 = pass, 1 = fail, 2 = command-line error");
}

fn parse(args: &[String], specs: &[Spec], max_positional: usize) -> Flags {
    cli::parse(args, specs, max_positional).unwrap_or_else(|e| usage_error(e))
}

/// Modes without arguments still reject stray ones, so a typo cannot pass
/// silently as "the default".
fn no_args(args: &[String]) -> bool {
    parse(args, &[], 0);
    true
}

fn num<T: std::str::FromStr>(flags: &Flags, name: &str, default: T) -> T {
    flags.num(name, default).unwrap_or_else(|e| usage_error(e))
}

fn positional_num<T: std::str::FromStr>(flags: &Flags, index: usize) -> Option<T> {
    flags
        .positional_num(index)
        .unwrap_or_else(|e| usage_error(e))
}

fn report(result: Result<(), String>, prefix: &str) -> bool {
    if let Err(error) = result {
        eprintln!("{prefix}: {error}");
        return false;
    }
    true
}

/// Diagnostics run on whatever track is loaded: opt out of the track guard
/// unless the caller asked for a specific one.
fn allow_any_level() {
    if std::env::var("TAS_TEST_LEVEL").is_err() {
        // SAFETY: single-threaded startup, before any harness thread.
        unsafe { std::env::set_var("TAS_TEST_LEVEL", "any") };
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

fn run_regression(args: &[String]) -> bool {
    no_args(args);
    let out = output_dir();
    let csv_path = out.join("regression_results.csv");
    let cert_path = out.join("regression_certificate.json");
    let results = regression::run(&csv_path);
    certificate::write_regression(&results, &csv_path, &cert_path);
    !results.is_empty() && results.iter().all(|r| r.all_gates_pass)
}

fn run_acceptance(args: &[String]) -> bool {
    let flags = parse(args, &[], 1);
    let iterations: u32 = positional_num(&flags, 0).unwrap_or(5);
    if iterations == 0 {
        usage_error("acceptance iteration count must be at least 1");
    }
    let cert_path = output_dir().join("acceptance_certificate.json");
    let mut last_result = None;
    let mut passed = 0u32;
    for i in 1..=iterations {
        println!(
            "\n========== Acceptance run {}/{} ==========",
            i, iterations
        );
        let result = acceptance::run();
        let ok = result.all_pass();
        last_result = Some(result);
        if ok {
            passed += 1;
            println!("Acceptance run {}/{} PASSED", i, iterations);
        } else {
            println!("Acceptance run {}/{} FAILED — aborting", i, iterations);
            break;
        }
    }
    if let Some(result) = last_result.as_ref() {
        certificate::write_acceptance(result, &cert_path);
    }
    println!(
        "\n=== Acceptance: {}/{} runs passed ===",
        passed, iterations
    );
    passed == iterations
}

fn run_replay(args: &[String]) -> bool {
    let flags = parse(
        args,
        &[
            flag("--iterations", Some("-n")),
            switch("--verbose", Some("-v")),
            switch("--no-match", None),
        ],
        1,
    );
    let Some(path) = flags.positional.first() else {
        usage_error("replay needs <path.tasrec>");
    };
    let iterations = num(&flags, "--iterations", 5u32);
    let no_match = flags.is_set("--no-match");
    let report = replay::run(path, iterations, flags.is_set("--verbose"), no_match);
    // Zero iterations would make `.any()` vacuously false.
    if report.results.is_empty() {
        eprintln!("ERROR: replay produced no iterations — nothing was verified");
        return false;
    }
    // Drift, incomplete playback and (unless --no-match) a failed start match
    // all fail; drift alone is not enough because a run that never started
    // reports 0.0 drift over zero frames.
    !report
        .results
        .iter()
        .any(|r| r.has_drift() || !r.playback_complete || (!no_match && !r.position_matched))
}

fn run_cont_reliability(args: &[String]) -> bool {
    let flags = parse(
        args,
        &[
            flag("--iterations", Some("-n")),
            flag("--speed", Some("-s")),
            flag("--splice", None),
            flag("--file", None),
            switch("--synthetic", None),
            flag("--profile", None),
            flag("--tap-ticks", None),
        ],
        0,
    );
    let iterations = num(&flags, "--iterations", 10u32);
    let speed = num(&flags, "--speed", 12.0f32);
    let splice = num(&flags, "--splice", 2200u32);
    let profile = match flags.value("--profile") {
        None => cont_reliability::BaselineInputProfile::Taps,
        Some(raw) => cont_reliability::BaselineInputProfile::parse(raw).unwrap_or_else(|| {
            usage_error(format!(
                "invalid --profile '{raw}'; expected 'taps' or 'sweep'"
            ))
        }),
    };
    let tap_ticks = flags
        .value("--tap-ticks")
        .map(|_| num(&flags, "--tap-ticks", 8u32).max(1));
    // A real steered recording makes the drift check meaningful, so the
    // default baseline is FE-tremendous; `--synthetic` records a fresh one.
    let file = if flags.is_set("--synthetic") {
        None
    } else {
        flags.value("--file").map(str::to_string).or_else(|| {
            harness::fixture_path("FE-tremendous.tasrec")
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
        })
    };
    match &file {
        Some(p) => println!("  Baseline: real recording {}", p),
        None => println!("  Baseline: synthetic"),
    }
    let report = cont_reliability::run(
        iterations,
        speed,
        splice,
        file.as_deref(),
        profile,
        tap_ticks,
    );
    report.all_pass()
}

#[cfg(test)]
mod tests {
    use super::MODES;

    #[test]
    fn mode_names_are_unique() {
        for (i, mode) in MODES.iter().enumerate() {
            assert!(
                !MODES[..i].iter().any(|m| m.name == mode.name),
                "duplicate mode {}",
                mode.name
            );
        }
    }
}
