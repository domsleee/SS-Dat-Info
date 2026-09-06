//! Live F12 regression through the deployed UI, with real Pico LEFT taps.
//! Unlike harness-driven CONT, this never writes commands to shared memory.
use std::{
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    time::{Duration, Instant},
};

struct Options {
    log: PathBuf,
    splice: u32,
    iterations: u32,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut result = Options {
        log: PathBuf::new(),
        splice: 2200,
        iterations: 5,
    };
    for pair in args.chunks(2) {
        if pair.len() != 2 {
            return Err("Each option needs a value".into());
        }
        match pair[0].as_str() {
            "--log" => result.log = pair[1].as_str().into(),
            "--splice" => result.splice = pair[1].parse().map_err(|_| "Invalid splice")?,
            "--iterations" => {
                result.iterations = pair[1].parse().map_err(|_| "Invalid iterations")?
            }
            other => return Err(format!("Unknown option: {other}")),
        }
    }
    if result.log.as_os_str().is_empty()
        || !(1..=65535).contains(&result.splice)
        || !(1..=100).contains(&result.iterations)
    {
        return Err("Use --log <running UI log> --splice <1..65535> --iterations <1..100>".into());
    }
    Ok(result)
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
    let config = options(args)?;
    #[cfg(windows)]
    {
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
        // Do not use harness::connect: that stops competing UI writers. Here
        // the deployed UI is precisely the controller being tested.
        let client = TasSharedMemoryClient::open()?;
        if client.mode_volatile() != TasMode::Off as u32
            || !client.command_idle()
            || client.state().recorded_count < config.splice
            || client.state().level_id != 0
        {
            return Err(
                "Load the saved FE run in the UI, set From to --splice, and STOP before testing"
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
    fn cli_requires_log_and_bounded_parameters() {
        assert!(options(&[]).is_err());
        for args in [
            vec!["--log", "test.log", "--iterations", "0"],
            vec!["--log", "test.log", "--splice", "65536"],
            vec!["--unknown", "x"],
        ] {
            assert!(options(&args.into_iter().map(str::to_string).collect::<Vec<_>>()).is_err());
        }
        assert!(options(&["--log".into(), "test.log".into()]).is_ok());
    }
}
