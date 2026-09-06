//! Live F12 regression through the deployed UI, with real Pico LEFT taps.
//! CONT always goes through F12. Shared-memory commands are only used for setup
//! and emergency STOP after the owned UI has exited.
use std::{
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    time::{Duration, Instant},
};

struct Options {
    log: PathBuf,
    recording: Option<PathBuf>,
    splice: u32,
    iterations: u32,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut result = Options {
        log: PathBuf::new(),
        recording: None,
        splice: 4500,
        iterations: 5,
    };
    for pair in args.chunks(2) {
        if pair.len() != 2 {
            return Err("Each option needs a value".into());
        }
        match pair[0].as_str() {
            "--recording" => result.recording = Some(pair[1].as_str().into()),
            "--splice" => result.splice = pair[1].parse().map_err(|_| "Invalid splice")?,
            "--iterations" => {
                result.iterations = pair[1].parse().map_err(|_| "Invalid iterations")?
            }
            other => return Err(format!("Unknown option: {other}")),
        }
    }
    if !(1..=65535).contains(&result.splice) || !(1..=100).contains(&result.iterations) {
        return Err("Use --recording <tasrec> --splice <1..65535> --iterations <1..100>".into());
    }
    Ok(result)
}

// The same original history capture used by the offline banner regression.
const ORIGINAL_RECORDING: &[u8] =
    include_bytes!("../../tas_ui/src/tests/data/cont-splice-4500/recording.tasrec");

fn original_recording_file() -> Vec<u8> {
    let count = u32::from_le_bytes(ORIGINAL_RECORDING[..4].try_into().unwrap());
    assert_eq!(ORIGINAL_RECORDING.len(), 4 + count as usize * 13);
    // History blobs have no file metadata. Add the normal file envelope using
    // the test's standard injection configuration, without inventing a rider
    // or renderer stamp. Copy the captured inputs and XYZ bytes verbatim.
    let metadata = serde_json::to_vec(&serde_json::json!({
        "version": 1, "recorded_count": count, "inject_mode": 6,
        "force_fixed_tick": 0, "force_direct": 2, "input_source": 0,
        "max_drift_x": 0.0, "max_drift_z": 0.0, "timestamp": "2026-09-05",
        "notes": "UI break at 4500: original history entry 2431; metadata envelope added for live UI loading"
    })).unwrap();
    let mut file = (metadata.len() as u32).to_le_bytes().to_vec();
    file.extend_from_slice(&metadata);
    file.extend_from_slice(&ORIGINAL_RECORDING[4..]);
    file
}

fn verdict(log: &str, splice: u32) -> Result<bool, String> {
    if [
        "CONT bucket reroll",
        "CONT aborted",
        "CONT gave up",
        "Game process not found",
        "DRIFT",
    ]
    .iter()
    .any(|text| log.contains(text))
    {
        return Err("Retry, drift or runtime failure in UI log".into());
    }
    let resumes = log
        .matches(&format!(
            "CONT resumed at frame {splice} after 1 bucket attempt "
        ))
        .count();
    if resumes > 1 {
        return Err("More than one CONT was triggered".into());
    }
    let marker = format!("CONT splice {splice}: ");
    let mut checked = false;
    for line in log.lines() {
        if let Some((_, values)) = line.split_once(&marker) {
            let numbers: Vec<f32> = values
                .split_whitespace()
                .filter_map(|word| {
                    word.split_once('=')
                        .and_then(|(_, value)| value.parse().ok())
                })
                .collect();
            if numbers.len() != 2 || numbers.iter().any(|n| !n.is_finite() || *n != 0.0) {
                return Err(format!("Nonzero or invalid splice verdict: {line}"));
            }
            checked = true;
        }
    }
    let shortcut = log.contains("Global F12 (in-game): CONT") || log.contains("Shortcut: F12 CONT");
    Ok(shortcut && resumes == 1 && checked)
}

fn new_log(path: &std::path::Path, offset: u64) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    if file.metadata().map_err(|e| e.to_string())?.len() < offset {
        return Err("UI log was truncated".into());
    }
    file.seek(SeekFrom::Start(offset))
        .map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    // A concurrently appended last line is not a complete verdict yet.
    if let Some(end) = text.rfind('\n') {
        text.truncate(end + 1);
    } else {
        text.clear();
    }
    Ok(text)
}

pub fn run(args: &[String]) -> Result<(), String> {
    let mut config = options(args)?;
    #[cfg(windows)]
    {
        let _ui = live::prepare(&mut config)?;
        live::run(&config)
    }
    #[cfg(not(windows))]
    {
        let _ = config;
        Err("Live UI tests require Windows".into())
    }
}

pub fn validate_options(args: &[String]) -> Result<(), String> {
    options(args).map(|_| ())
}

#[cfg(windows)]
mod live {
    use super::*;
    use std::{os::windows::process::CommandExt, thread};
    use tas_shared::{TasMode, TasSharedMemoryClient};

    pub(super) struct UiProcess(std::process::Child);
    impl Drop for UiProcess {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
            // If focus was lost, StopOnExit could not safely send F11. With
            // our UI gone, no controller can re-arm while we stop the game.
            if let Ok(mut client) = TasSharedMemoryClient::open() {
                if client.mode_volatile() != TasMode::Off as u32 || !client.command_idle() {
                    crate::harness::stop(&mut client);
                }
            }
        }
    }

    pub(super) fn prepare(config: &mut Options) -> Result<UiProcess, String> {
        let executable = std::env::current_exe()
            .map_err(|e| e.to_string())?
            .with_file_name("tas_ui.exe");
        if !executable.is_file() {
            return Err("Build tas_ui beside tas_test before running".into());
        }
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis();
        let root = crate::output_dir().join(format!("cont-ui-{stamp}-{}", std::process::id()));
        std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let recording = match &config.recording {
            Some(path) => path.canonicalize().map_err(|e| format!("Recording: {e}"))?,
            None => {
                let path = root.join("FE-UI-break-at-4500.tasrec");
                std::fs::write(&path, original_recording_file()).map_err(|e| e.to_string())?;
                path
            }
        };
        let loaded = crate::replay::load_tasrec(&recording)?;
        if config.splice > loaded.count {
            return Err("Splice outside recording".into());
        }
        let mut client = crate::harness::ensure_game_running();
        crate::harness::ensure_exclusive_runtime_ownership(&mut client, "UI LEFT-spam setup");
        if client.state().level_id != 0 {
            return Err("UI LEFT-spam requires Forest Easy".into());
        }
        println!(
            "UI fixture: {} ({} ticks)",
            recording.display(),
            loaded.count
        );
        config.log = root.join("tas_ui.log");
        let stderr = std::fs::File::create(root.join("startup.log")).map_err(|e| e.to_string())?;
        let mut ui = UiProcess(
            std::process::Command::new(executable)
                .env("SSB_INSPECT_DATA_DIR", &root)
                .env("SSB_INSPECT_E2E_RECORDING", &recording)
                .env("SSB_INSPECT_E2E_SPLICE", config.splice.to_string())
                .stderr(stderr)
                .creation_flags(0x08000000)
                .spawn()
                .map_err(|e| e.to_string())?,
        );
        println!("UI artifacts: {}", root.display());
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(status) = ui.0.try_wait().map_err(|e| e.to_string())? {
                return Err(format!(
                    "UI setup exited {status}; see {}",
                    root.join("startup.log").display()
                ));
            }
            if std::fs::read_to_string(&config.log).is_ok_and(|s| s.contains("UI E2E ready")) {
                break;
            }
            if Instant::now() >= deadline {
                return Err("UI setup timed out; see startup.log".into());
            }
            thread::sleep(Duration::from_millis(50));
        }
        Ok(ui)
    }
    #[link(name = "user32")]
    extern "system" {
        fn SetForegroundWindow(window: isize) -> i32;
        fn GetForegroundWindow() -> isize;
        fn IsWindow(window: isize) -> i32;
        fn GetAsyncKeyState(key: i32) -> i16;
        fn keybd_event(key: u8, scan: u8, flags: u32, extra: usize);
    }
    fn press(key: u8) {
        unsafe {
            keybd_event(key, 0, 0, 0);
        }
        thread::sleep(Duration::from_millis(100));
        unsafe {
            keybd_event(key, 0, 2, 0);
        }
    }
    struct StopOnExit(isize);
    impl Drop for StopOnExit {
        fn drop(&mut self) {
            if unsafe { GetForegroundWindow() == self.0 && IsWindow(self.0) != 0 } {
                press(122);
            }
        }
    }
    fn healthy(windows: &[isize; 2]) -> Result<(), String> {
        if windows
            .iter()
            .any(|window| unsafe { IsWindow(*window) == 0 })
        {
            return Err("Game or UI exited".into());
        }
        if unsafe { GetForegroundWindow() != windows[0] } {
            return Err("Game lost focus; trial inconclusive".into());
        }
        Ok(())
    }
    pub(super) fn run(config: &Options) -> Result<(), String> {
        // Setup is complete; the child UI is now the controller under test.
        let client = TasSharedMemoryClient::open()?;
        if client.mode_volatile() != TasMode::Off as u32
            || !client.command_idle()
            || client.state().recorded_count < config.splice
            || client.state().level_id != 0
        {
            return Err(
                "Automatic UI setup did not leave a stopped FE recording at the requested splice"
                    .into(),
            );
        }
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "$ErrorActionPreference='Stop'; $g=@(Get-Process Supreme); $u=@(Get-Process tas_ui); if($g.Count -ne 1 -or $u.Count -ne 1){throw 'Require exactly one game and UI'}; @($g[0].MainWindowHandle.ToInt64(),$u[0].MainWindowHandle.ToInt64()) | ConvertTo-Json -Compress"])
            .creation_flags(0x08000000).output().map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        let windows: [isize; 2] =
            serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        std::fs::metadata(&config.log).map_err(|e| e.to_string())?;
        unsafe {
            SetForegroundWindow(windows[0]);
        }
        thread::sleep(Duration::from_millis(300));
        healthy(&windows)?;
        let mut pico = crate::harness::PicoKeys::open()
            .ok_or("Verified Pico data port required (TAS_PICO_PORT)")?;
        println!(
            "Live UI LEFT-spam: {} iterations, splice {}, Pico {}",
            config.iterations,
            config.splice,
            pico.port_name()
        );
        for trial in 1..=config.iterations {
            healthy(&windows)?;
            let offset = std::fs::metadata(&config.log)
                .map_err(|e| e.to_string())?
                .len();
            if !pico.send(255) {
                return Err("Pico release failed".into());
            }
            let stop = StopOnExit(windows[0]);
            press(123); // Actual UI F12 shortcut; never ArmContinue from this process.
            let start = Instant::now();
            while start.elapsed() < Duration::from_secs(3) {
                healthy(&windows)?;
                for (mask, down) in [(1, true), (255, false)] {
                    if !pico.send(mask) {
                        return Err("Pico write failed".into());
                    }
                    thread::sleep(Duration::from_millis(40));
                    if unsafe { GetAsyncKeyState(37) < 0 } != down {
                        return Err("LEFT HID transition not observed".into());
                    }
                }
            }
            loop {
                healthy(&windows)?;
                let text = new_log(&config.log, offset)?;
                if verdict(&text, config.splice).map_err(|error| format!("{error}\n{text}"))? {
                    println!("{text}");
                    break;
                }
                if start.elapsed() > Duration::from_secs(30) {
                    return Err(format!("Trial {trial}: missing first-attempt resume or explicit splice verdict\n{text}"));
                }
                thread::sleep(Duration::from_millis(50));
            }
            drop(stop); // F11 through the UI.
            let deadline = Instant::now() + Duration::from_secs(3);
            while client.mode_volatile() != TasMode::Off as u32 || !client.command_idle() {
                healthy(&windows)?;
                if Instant::now() >= deadline {
                    return Err("UI STOP was not acknowledged".into());
                }
                thread::sleep(Duration::from_millis(20));
            }
            println!("PASS trial {trial}: physical LEFT taps, first attempt, explicit zero splice mismatch");
        }
        println!("CONT UI LEFT-SPAM PASSED");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_fixture_is_the_original_4500_capture_without_modified_samples() {
        assert!(options(&[]).unwrap().recording.is_none());
        let file = original_recording_file();
        let header_len = u32::from_le_bytes(file[..4].try_into().unwrap()) as usize;
        let meta: serde_json::Value = serde_json::from_slice(&file[4..4 + header_len]).unwrap();
        assert_eq!(meta["recorded_count"], 5032);
        assert_eq!(&file[4 + header_len..], &ORIGINAL_RECORDING[4..]);
        assert_eq!(options(&[]).unwrap().splice, 4500);
    }
    #[test]
    fn resume_alone_cannot_pass_and_transients_are_diagnostic() {
        let resume = "Global F12 (in-game): CONT\nCONT resumed at frame 4500 after 1 bucket attempt — bucket matched\n";
        assert!(!verdict(resume, 4500).unwrap());
        assert!(verdict(&format!("{resume}CONT prefix difference first at tick 1831\nCONT splice 4500: X=0.000000000 Z=0.000000000\n"), 4500).unwrap());
        for bad in [
            "CONT splice 4500: X=0.5 Z=0",
            "CONT splice 4500: X=NaN Z=0",
            "CONT bucket reroll",
            "DRIFT",
        ] {
            assert!(verdict(&format!("{resume}{bad}"), 4500).is_err());
        }
        assert!(!verdict(&format!("{resume}CONT splice 2200: X=0 Z=0"), 4500).unwrap());
    }
    #[test]
    fn cli_has_self_contained_defaults_and_bounded_parameters() {
        assert!(options(&[]).is_ok());
        for args in [
            vec!["--iterations", "0"],
            vec!["--splice", "65536"],
            vec!["--unknown", "x"],
        ] {
            assert!(options(&args.into_iter().map(str::to_string).collect::<Vec<_>>()).is_err());
        }
        assert!(options(&["--recording".into(), "test.tasrec".into()]).is_ok());
    }
}
