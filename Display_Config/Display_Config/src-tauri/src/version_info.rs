#[tauri::command]
#[specta::specta]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
