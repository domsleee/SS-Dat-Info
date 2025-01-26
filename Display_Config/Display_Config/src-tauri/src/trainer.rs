use std::process::ExitStatus;

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
pub async fn run_trainer(trainer_settings: TrainerSettings) -> Result<(), String> {
    let supreme_folder = get_supreme_folder();
    let settings_path = supreme_folder.join("SupremeTrainer.autorun.json");
    let settings_file = std::fs::File::create(settings_path).unwrap();

    // Use pretty formatting and customize serialization
    let writer = std::io::BufWriter::new(&settings_file);
    serde_json::to_writer_pretty(writer, &trainer_settings).unwrap();

    let supreme_trainer_path = supreme_folder.join("SupremeTrainer.exe");
    let status = run_trainer_as_admin(supreme_trainer_path.to_str().unwrap());
    println!("Status: {:?}", status);
    Ok(())
}

fn run_trainer_as_admin(trainer_path: &str) -> Result<(), String> {
    let status: ExitStatus = runas::Command::new(trainer_path)
        .status()
        .map_err(|err| format!("Failed to spawn process: {err}"))?;

    // Check if the child process exited successfully
    if status.success() {
        Ok(())
    } else {
        Err(format!("Trainer exited with code: {:?}", status.code()))
    }
}
