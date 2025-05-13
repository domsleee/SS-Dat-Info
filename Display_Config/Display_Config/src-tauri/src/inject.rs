use std::{os::windows::process::CommandExt, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrainerSettings {
    #[specta(rename = "use4xFonts")]
    pub use4x_fonts: bool,
    pub change_fov: bool,
    pub fov_width: Option<i32>,
    pub fov_height: Option<i32>,
    pub enable_logging: bool,
    pub make_ghosts_opaque: bool,
    pub match_ghost_sounds_to_character: bool,
    pub disable_direct_input: bool,
    pub enable_custom_controls: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn run_inject(trainer_settings: TrainerSettings) -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");
    let settings_path = display_config_resources.join("Display_Config_Helper.json");
    let settings_file = std::fs::File::create(settings_path).unwrap();

    let log_path = get_log_path();
    if log_path.exists() {
        std::fs::remove_file(&log_path).expect("Failed to remove log file");
    }

    let writer = std::io::BufWriter::new(&settings_file);
    serde_json::to_writer_pretty(writer, &trainer_settings).unwrap();

    let injector_path = display_config_resources.join("Injector.exe");
    let status = Command::new(injector_path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW (https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
        .current_dir(&display_config_resources)
        .status()
        .map_err(|err| format!("Failed to spawn process: {err}"))?;

    if !status.success() {
        return Err("Injector.exe failed.\nDid you run using Supreme.exe?".to_string());
    }

    wait_for_finished_log(&log_path)
}

pub fn get_log_path() -> PathBuf {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");

    display_config_resources.join("Display_Config_Helper.log")
}

fn wait_for_finished_log(log_path: &PathBuf) -> Result<String, String> {
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(5);

    while start_time.elapsed() < timeout_duration {
        if let Ok(log_contents) = std::fs::read_to_string(log_path) {
            if log_contents.contains("Finished.") {
                return Ok("Finished".to_string());
            }
        }
        // Ignore error case and continue waiting
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let formatted_log_path = log_path.display().to_string().replace("\\", "/");
    Err(format!(
        "Timeout waiting for 'Finished.' in log {formatted_log_path}"
    ))
}
