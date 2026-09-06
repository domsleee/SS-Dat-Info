//! One live workflow: production UI first, then harness-owned game tests.
use std::{
    path::PathBuf,
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(serde::Serialize)]
struct Stage {
    name: &'static str,
    args: Vec<String>,
    status: &'static str,
    error: Option<String>,
    elapsed_seconds: f64,
}

fn plan(ui_args: &[String]) -> Vec<Stage> {
    [
        (
            "ui-left-spam",
            [vec!["cont-ui-left-spam".into()], ui_args.to_vec()].concat(),
        ),
        ("acceptance", vec!["acceptance".into()]),
        ("regression", vec!["regression".into()]),
    ]
    .into_iter()
    .map(|(name, args)| Stage {
        name,
        args,
        status: "not-run",
        error: None,
        elapsed_seconds: 0.0,
    })
    .collect()
}

fn execute(stages: &mut [Stage], mut run: impl FnMut(&Stage) -> Result<(), String>) -> bool {
    for stage in stages {
        let start = Instant::now();
        let result = run(stage);
        stage.elapsed_seconds = start.elapsed().as_secs_f64();
        match result {
            Ok(()) => stage.status = "passed",
            Err(error) => {
                stage.status = "failed";
                stage.error = Some(error);
                return false;
            }
        }
    }
    true
}

pub fn run(args: &[String], output: PathBuf) -> Result<(), String> {
    // Reject malformed UI arguments before creating any child process.
    crate::cont_ui::validate_options(args)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();
    let directory = output.join(format!("live-{stamp}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut stages = plan(args);
    let summary = directory.join("summary.json");
    std::fs::write(&summary, serde_json::to_vec_pretty(&stages).unwrap())
        .map_err(|e| e.to_string())?;
    let passed = execute(&mut stages, |stage| {
        let stage_dir = directory.join(stage.name);
        std::fs::create_dir(&stage_dir).map_err(|e| e.to_string())?;
        let log_path = stage_dir.join("output.log");
        let log = std::fs::File::create(&log_path).map_err(|e| e.to_string())?;
        println!("START {} (output: {})", stage.name, log_path.display());
        let mut command = Command::new(&executable);
        command
            .args(&stage.args)
            .env("TAS_TEST_OUTPUT", &stage_dir)
            .stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
            .stderr(Stdio::from(log));
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }
        let status = command.status().map_err(|e| e.to_string())?;
        println!("END {}: {}", stage.name, status);
        if status.success() {
            Ok(())
        } else {
            Err(format!("{status}; see {}", log_path.display()))
        }
    });
    std::fs::write(&summary, serde_json::to_vec_pretty(&stages).unwrap())
        .map_err(|e| e.to_string())?;
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
    fn ui_is_mandatory_and_precedes_harness_ownership() {
        let args = vec![
            "--log".into(),
            "ui.log".into(),
            "--splice".into(),
            "4500".into(),
        ];
        let mut stages = plan(&args);
        let mut ran = Vec::new();
        assert!(execute(&mut stages, |stage| {
            ran.push(stage.name);
            Ok(())
        }));
        assert_eq!(ran, ["ui-left-spam", "acceptance", "regression"]);
        assert_eq!(&stages[0].args[1..], &args);
    }
    #[test]
    fn any_failure_stops_later_stages_and_marks_report() {
        for failure in 0..3 {
            let mut stages = plan(&[]);
            let mut calls = 0;
            assert!(!execute(&mut stages, |_| {
                let failed = calls == failure;
                calls += 1;
                if failed {
                    Err("test failure".into())
                } else {
                    Ok(())
                }
            }));
            assert_eq!(calls, failure + 1);
            assert_eq!(stages[failure].status, "failed");
            assert!(stages[failure + 1..]
                .iter()
                .all(|stage| stage.status == "not-run"));
        }
    }
}
