use std::fs::File;
use std::io::Write;

use crate::path_util::get_supreme_folder;

#[tauri::command]
#[specta::specta]
pub fn write_language(language: String) -> Result<(), String> {
    let language_file = get_supreme_folder().join("Saved_Data").join("language.txt");

    // write to the language_file
    let mut file = File::create(&language_file)
        .map_err(|e| format!("Failed to create language file: {language_file:?} {}", e))?;
    write!(file, "{}", language).map_err(|e| format!("Failed to write language: {}", e))
}
