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
    pub hide_blinking_r: bool,
    pub show_replay_speed: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn run_inject(trainer_settings: TrainerSettings) -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");
    let settings_path = display_config_resources.join("Display_Config_Helper.json");
    let settings_file = std::fs::File::create(settings_path).unwrap();

    let log_path = get_display_config_helper_log_path();
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

pub fn get_display_config_helper_log_path() -> PathBuf {
    get_display_config_resources_path().join("Display_Config_Helper.log")
}

pub fn get_display_config_resources_path() -> PathBuf {
    let supreme_folder = get_supreme_folder();
    supreme_folder.join("Display_Config_Resources")
}

/// Inject TAS_Helper.dll into the running Supreme.exe process.
/// Looks for TAS_Helper/Injector.exe and TAS_Helper.dll next to Display_Config_Resources.
#[tauri::command]
#[specta::specta]
pub async fn run_tas_inject() -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let tas_folder = supreme_folder.join("TAS_Helper");
    let injector_path = tas_folder.join("Injector.exe");
    let dll_path = tas_folder.join("TAS_Helper.dll");

    if !injector_path.exists() {
        return Err(format!(
            "TAS Injector not found at {}",
            injector_path.display()
        ));
    }
    if !dll_path.exists() {
        return Err(format!(
            "TAS_Helper.dll not found at {}",
            dll_path.display()
        ));
    }

    let status = Command::new(&injector_path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .current_dir(&tas_folder)
        .status()
        .map_err(|err| format!("Failed to spawn TAS Injector: {err}"))?;

    if !status.success() {
        return Err("TAS Injector.exe failed.\nIs Supreme.exe running?".to_string());
    }

    // Launch tas_ui.exe (SSB Inspect) as a detached process
    let tas_ui_path = tas_folder.join("tas_ui.exe");
    if tas_ui_path.exists() {
        let _ = Command::new(&tas_ui_path)
            .current_dir(&tas_folder)
            .creation_flags(0x00000008) // DETACHED_PROCESS
            .spawn();
    }

    Ok("TAS_Helper.dll injected".to_string())
}

fn wait_for_finished_log(log_path: &PathBuf) -> Result<String, String> {
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(5);

    while start_time.elapsed() < timeout_duration {
        if let Ok(log_contents) = std::fs::read_to_string(log_path)
            && log_contents.contains("Finished.")
        {
            return Ok("Finished".to_string());
        }
        // Ignore error case and continue waiting
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let formatted_log_path = log_path.display().to_string().replace("\\", "/");
    Err(format!(
        "Timeout waiting for 'Finished.' in log {formatted_log_path}"
    ))
}
