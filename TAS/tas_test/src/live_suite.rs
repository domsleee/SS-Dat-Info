//! Ordered live regression plans. UI owns the game first; menu navigation runs last.
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(serde::Serialize)]
struct Stage {
    name: &'static str,
    args: Vec<String>,
    status: &'static str,
    error: Option<String>,
    elapsed_seconds: f64,
}

fn stage(name: &'static str, args: &[&str]) -> Stage {
    Stage {
        name,
        args: std::iter::once(name)
            .chain(args.iter().copied())
            .map(str::to_string)
            .collect(),
        status: "not-run",
        error: None,
        elapsed_seconds: 0.0,
    }
}

// @ arguments resolve through the same fixture locator as individual modes.
const FULL_CASES: &[(&str, &[&str])] = &[
    ("smoke", &[]),
    ("f5", &[]),
    ("segment", &[]),
    ("replay", &["@FE-tremendous.tasrec"]),
    ("reliability", &[]),
    ("drift-speed", &[]),
    ("pause-resume", &[]),
    ("stop-play-flake", &[]),
    ("rec-start", &[]),
    ("rec-repro", &[]),
    ("steer-impact", &[]),
    ("speed", &[]),
    ("speed-reset", &[]),
    ("catchup-speed", &[]),
    ("play-pace", &[]),
    ("play-judge", &[]),
    ("fe-cont-reliability", &[]),
    ("fe10065-cont", &[]),
    ("cont-hijack", &[]),
    ("cont-restart-race", &[]),
    ("cont-input-protection", &[]),
    ("gate-align", &[]),
    ("level-seq", &[]),
    ("save-reload", &[]),
    ("dialog-e2e", &[]),
];

// Every mode is either run or explicitly accounted for. New modes must update this contract.
#[cfg(test)]
const EXCLUDED: &[(&str, &str)] = &[
    ("live", "suite entry point"),
    ("live-full", "suite entry point"),
    (
        "cont-reliability",
        "covered by the two named CONT case presets",
    ),
    ("refresh-tasrec", "baseline-writing utility"),
    ("load", "manual UI loading"),
    ("shm", "manual diagnostics"),
    ("menu", "navigation exercised by dialog-e2e"),
    ("gamestate", "manual diagnostics"),
    ("video-rate", "screen sampling exercised by dialog-e2e"),
];

fn plan(ui_args: &[String], full: bool) -> Vec<Stage> {
    let mut ui = stage("cont-ui-left-spam", &[]);
    ui.args.extend_from_slice(ui_args);
    let mut stages = vec![ui, stage("acceptance", &[]), stage("regression", &[])];
    if full {
        stages.extend(FULL_CASES.iter().map(|(name, args)| stage(name, args)));
    }
    stages
}

fn execute(
    stages: &mut [Stage],
    mut run: impl FnMut(&Stage) -> Result<(), String>,
    mut publish: impl FnMut(&[Stage]) -> Result<(), String>,
) -> Result<bool, String> {
    publish(stages)?;
    for index in 0..stages.len() {
        stages[index].status = "running";
        publish(stages)?;
        let start = Instant::now();
        let result = run(&stages[index]);
        let stage = &mut stages[index];
        stage.elapsed_seconds = start.elapsed().as_secs_f64();
        stage.status = if result.is_ok() { "passed" } else { "failed" };
        stage.error = result.err();
        let failed = stage.error.is_some();
        publish(stages)?;
        if failed {
            return Ok(false);
        }
    }
    Ok(true)
}

fn wait_child(child: &mut Child, timeout: Duration) -> Result<ExitStatus, String> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if start.elapsed() < timeout => std::thread::sleep(Duration::from_millis(50)),
            result => {
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    // Exact owned child tree, never an image-name kill.
                    let _ = Command::new("taskkill")
                        .args(["/F", "/T", "/PID", &child.id().to_string()])
                        .creation_flags(0x08000000)
                        .output();
                }
                let _ = child.kill();
                let _ = child.wait();
                return Err(match result {
                    Err(error) => format!("Cannot wait for child: {error}"),
                    _ => format!("Stage exceeded {} seconds", timeout.as_secs()),
                });
            }
        }
    }
}

fn preflight(stages: &mut [Stage], full: bool, executable: &Path) -> Result<(), String> {
    if !executable.with_file_name("tas_ui.exe").is_file() {
        return Err("Build tas_ui beside tas_test".into());
    }
    for stage in stages.iter_mut() {
        for arg in &mut stage.args {
            if let Some(name) = arg.strip_prefix('@') {
                *arg = crate::harness::fixture_path(name)?
                    .canonicalize()
                    .map_err(|e| e.to_string())?
                    .to_string_lossy()
                    .into_owned();
            }
        }
    }
    for pair in stages[0].args[1..].chunks(2) {
        if pair[0] == "--recording" {
            crate::replay::load_tasrec(Path::new(&pair[1]))?;
        }
    }
    if full {
        // These are also consumed internally by named cases, not passed in argv.
        for name in [
            "FE-tremendous.tasrec",
            "FE-10065.tasrec",
            "FE-decent-done.tasrec",
        ] {
            crate::replay::load_tasrec(&crate::harness::fixture_path(name)?)?;
        }
        if std::env::var("NO_REVIVE").is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true")) {
            return Err("live-full includes save-reload and cannot use NO_REVIVE=1".into());
        }
        let script = std::env::var("REVIVE_SUPREME_SCRIPT")
            .map_err(|_| "Set REVIVE_SUPREME_SCRIPT for save-reload")?;
        if !Path::new(&script).is_file() {
            return Err("REVIVE_SUPREME_SCRIPT does not exist".into());
        }
        if !Command::new("nu")
            .arg("--version")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return Err("Nushell (nu) is required for save-reload revival".into());
        }
        let folder = std::env::var("SUPREME_FOLDER").map_err(|_| "Set SUPREME_FOLDER")?;
        for relative in [
            "Supreme.exe",
            "Display_Config_Resources/Injector.exe",
            "Display_Config_Resources/TAS/TAS_Helper.dll",
        ] {
            if !Path::new(&folder).join(relative).is_file() {
                return Err(format!("Missing deployed {relative}"));
            }
        }
        let built = crate::harness::repo_path("TAS/TAS_Helper/Release/TAS_Helper.dll")?;
        let deployed = Path::new(&folder).join("Display_Config_Resources/TAS/TAS_Helper.dll");
        if std::fs::read(built).map_err(|e| e.to_string())?
            != std::fs::read(deployed).map_err(|e| e.to_string())?
        {
            return Err(
                "Deployed TAS_Helper.dll differs from the local build; deploy before live-full"
                    .into(),
            );
        }
        // Filters make a nominal full suite incomplete.
        if std::env::var("TAS_TEST_CASE_FILTER").is_ok_and(|v| !v.trim().is_empty()) {
            return Err("Unset TAS_TEST_CASE_FILTER for live-full".into());
        }
    }
    if let Ok(client) = tas_shared::TasSharedMemoryClient::open() {
        if tas_shared::resolved_level_id(client.state()) != Some(0) {
            return Err("Live suite requires a resolved Forest Easy race".into());
        }
    }
    crate::harness::PicoKeys::open_checked()?;
    Ok(())
}

pub fn run(args: &[String], output: PathBuf) -> Result<(), String> {
    crate::cont_ui::validate_options(args)?;
    run_plan(plan(args, false), false, output)
}

pub fn run_full(output: PathBuf) -> Result<(), String> {
    run_plan(plan(&[], true), true, output)
}

fn run_plan(mut stages: Vec<Stage>, full: bool, output: PathBuf) -> Result<(), String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();
    let directory = output.join(format!("live-{stamp}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let summary = directory.join("summary.json");
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let timeout = std::env::var("TAS_STAGE_TIMEOUT_SECONDS")
        .unwrap_or_else(|_| "1800".into())
        .parse::<u64>()
        .ok()
        .filter(|n| *n > 0)
        .ok_or("TAS_STAGE_TIMEOUT_SECONDS must be positive")?;
    if let Err(error) = preflight(&mut stages, full, &executable) {
        tas_codec::save_atomic(
            &summary,
            &serde_json::to_vec_pretty(&serde_json::json!({
                "preflight_error": error, "stages": stages,
            }))
            .map_err(|e| e.to_string())?,
        )?;
        return Err(format!("{error}; report: {}", summary.display()));
    }
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
    let passed = execute(
        &mut stages,
        |stage| {
            let stage_dir = directory.join(stage.name);
            std::fs::create_dir(&stage_dir).map_err(|e| e.to_string())?;
            let log_path = stage_dir.join("output.log");
            let log = std::fs::File::create(&log_path).map_err(|e| e.to_string())?;
            println!("START {} (output: {})", stage.name, log_path.display());
            let mut command = Command::new(&executable);
            command
                .args(&stage.args)
                .env("TAS_TEST_OUTPUT", &stage_dir)
                .env("TAS_SUITE_PID", std::process::id().to_string())
                .stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
                .stderr(Stdio::from(log));
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                command.creation_flags(0x08000000);
            }
            let mut child = command.spawn().map_err(|e| e.to_string())?;
            let status = wait_child(&mut child, Duration::from_secs(timeout))?;
            println!("END {}: {}", stage.name, status);
            if status.success() {
                Ok(())
            } else {
                Err(format!("{status}; see {}", log_path.display()))
            }
        },
        |stages| {
            tas_codec::save_atomic(&summary, &serde_json::to_vec_pretty(&serde_json::json!({
            "checkout_revision": revision, "full": full, "stages": stages,
            "deployment_check": if full { "on-disk DLL matches local build; loaded DLL identity not verified" } else { "not checked" },
        })).map_err(|e| e.to_string())?)
        },
    )?;
    for stage in &stages {
        println!("{}: {}", stage.name, stage.status);
    }
    println!("Live suite report: {}", summary.display());
    if passed {
        Ok(())
    } else {
        Err("Live suite failed; later stages were not run".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_mode_is_run_or_explicitly_excluded() {
        let stages = plan(&[], true);
        assert_eq!(stages[0].name, "cont-ui-left-spam");
        assert_eq!(stages.last().unwrap().name, "dialog-e2e");
        for mode in crate::MODES {
            let occurrences = stages.iter().filter(|s| s.name == mode.name).count()
                + EXCLUDED
                    .iter()
                    .filter(|(name, why)| *name == mode.name && !why.is_empty())
                    .count();
            assert_eq!(occurrences, 1, "mode {}", mode.name);
        }
        for stage in stages {
            assert!(crate::MODES.iter().any(|m| m.name == stage.name));
        }
    }
    #[test]
    fn failures_stop_the_plan_and_progress_is_published() {
        for failure in 0..3 {
            let mut stages = plan(&[], false);
            let mut calls = 0;
            let mut reports = Vec::new();
            assert!(!execute(
                &mut stages,
                |_| {
                    let fail = calls == failure;
                    calls += 1;
                    if fail {
                        Err("test failure".into())
                    } else {
                        Ok(())
                    }
                },
                |stages| {
                    reports.push(stages.iter().map(|s| s.status).collect::<Vec<_>>());
                    Ok(())
                }
            )
            .unwrap());
            assert_eq!(calls, failure + 1);
            assert_eq!(stages[failure].status, "failed");
            assert!(stages[failure + 1..].iter().all(|s| s.status == "not-run"));
            assert_eq!(reports[1][0], "running");
            assert_eq!(reports.last().unwrap()[failure], "failed");
        }
    }
    #[test]
    fn publication_failure_prevents_starting_a_child() {
        let mut stages = plan(&[], false);
        assert!(execute(
            &mut stages,
            |_| panic!("must not run"),
            |_| Err("disk full".into())
        )
        .is_err());
    }
    #[test]
    fn short_plan_forwards_ui_arguments() {
        let args = vec!["--splice".into(), "4500".into()];
        let stages = plan(&args, false);
        assert_eq!(&stages[0].args[1..], &args);
        assert_eq!(stages.len(), 3);
    }
    #[cfg(windows)]
    #[test]
    fn child_timeout_is_failure_and_reaps_the_owned_process() {
        use std::os::windows::process::CommandExt;
        let mut child = Command::new("powershell")
            .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
            .creation_flags(0x08000000)
            .spawn()
            .unwrap();
        assert!(wait_child(&mut child, Duration::from_millis(100)).is_err());
        assert!(child.try_wait().unwrap().is_some());
    }
}
