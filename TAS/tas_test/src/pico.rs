//! The updater and test preflight share Windows physical-device discovery.
use std::{
    process::{Command, Stdio},
    time::Duration,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Device {
    pub serial: String,
    pub instance_id: String,
    pub drive: String,
    pub data_port: String,
    pub console_port: String,
}

pub fn discover(verify: bool) -> Result<Device, String> {
    let script = crate::harness::repo_path("TAS/pico/inspect.ps1")?;
    let mut command = Command::new("pwsh");
    command.args(["-NoProfile", "-File"]).arg(script);
    if verify {
        command.arg("-Verify");
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command.spawn().map_err(|e| e.to_string())?;
    crate::live_suite::wait_child(&mut child, Duration::from_secs(30))?;
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().into());
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Invalid Pico discovery output: {e}"))
}

pub fn run() -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(&discover(true)?).map_err(|e| e.to_string())?
    );
    Ok(())
}
