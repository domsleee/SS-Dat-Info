use crate::{path_util::get_supreme_folder, player_types::get_player_types_path};

#[tauri::command]
#[specta::specta]
pub fn open_log_file() -> Result<(), String> {
    let log_file = get_supreme_folder().join("HMGEHLOG.TXT");
    if log_file.exists() {
        opener::open(log_file).map_err(|err| format!("Failed to open log file: {err}"))?;
        return Ok(());
    }
    Err(format!("Log file not found {log_file:?}"))
}

#[tauri::command]
#[specta::specta]
pub fn open_player_types() -> Result<(), String> {
    let player_types_file = get_player_types_path();
    if player_types_file.exists() {
        opener::open(player_types_file).map_err(|err| format!("Failed to open log file: {err}"))?;
        return Ok(());
    }
    Err(format!("Player types file not found {player_types_file:?}"))
}
