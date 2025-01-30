use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainerSettings {
    pub use4x_fonts: bool,
    pub change_fov: bool,
    pub fov_width: Option<i32>,
    pub fov_height: Option<i32>,
}

#[tauri::command]
pub async fn run_inject(trainer_settings: TrainerSettings) -> Result<String, String> {
    let supreme_folder = get_supreme_folder();
    let display_config_resources = supreme_folder.join("Display_Config_Resources");
    let settings_path = display_config_resources.join("Display_Config_Helper.json");
    let settings_file = std::fs::File::create(settings_path).unwrap();

    let writer = std::io::BufWriter::new(&settings_file);
    serde_json::to_writer_pretty(writer, &trainer_settings).unwrap();

    let injector_path = display_config_resources.join("Injector.exe");
    let status = Command::new(injector_path)
        .current_dir(&display_config_resources)
        .status()
        .map_err(|err| format!("Failed to spawn process: {err}"))?;
    if status.success() {
        Ok("seems good".to_string())
    } else {
        Err("Failed to inject".to_string())
    }
}
