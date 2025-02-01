use crate::path_util::get_supreme_folder;

#[tauri::command]
pub fn open_log_file() -> Result<(), String> {
    let log_file = get_supreme_folder().join("HMGEHLOG.TXT");
    if log_file.exists() {
        opener::open(log_file).map_err(|err| format!("Failed to open log file: {err}"))?;
        return Ok(());
    }
    Err(format!("Log file not found {log_file:?}"))
}
