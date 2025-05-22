#[tauri::command]
#[specta::specta]
pub fn get_log_data() -> Vec<String> {
    let log_path = crate::inject::get_log_path();

    if let Ok(file) = std::fs::read_to_string(&log_path) {
        file.lines().map(String::from).collect()
    } else {
        Vec::new()
    }
}
