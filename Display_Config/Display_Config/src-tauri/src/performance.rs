use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::inject::get_display_config_resources_path;

#[tauri::command]
#[specta::specta]
pub fn log_startup_time(startup_time_info: StartupTimeInfo) -> Result<String, String> {
    if std::env::var("DISPLAY_CONFIG_LOG").is_err() {
        return Ok("Skipped since DISPLAY_CONFIG_LOG is not set".into());
    }

    let log_path = get_display_config_resources_path().join("Display_Config.perf.log");
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!(
        "[{}]: onMounted1: {:.2}ms, onMounted2: {:.2}ms, createApp: {:.2}ms, mountApp: {:.2}ms, registerPlugins: {:.2}ms, totalStartup: {:.2}ms\n",
        timestamp,
        startup_time_info.on_mounted_1,
        startup_time_info.on_mounted_2,
        startup_time_info.create_app_time,
        startup_time_info.mount_app_time,
        startup_time_info.register_plugins_time,
        startup_time_info.total_startup_time
    );

    if let Err(e) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut file| file.write_all(log_entry.as_bytes()))
    {
        eprintln!("Failed to write performance log: {}", e);
    }

    Ok(log_path.to_string_lossy().into_owned())
}

#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StartupTimeInfo {
    pub on_mounted_1: f64,
    pub on_mounted_2: f64,
    pub create_app_time: f64,
    pub mount_app_time: f64,
    pub register_plugins_time: f64,
    pub total_startup_time: f64,
}
