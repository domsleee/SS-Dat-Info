use std::path::PathBuf;

use crate::{
    config_parser::{parse_config, ConfigEntry},
    config_writer::write_config_to_file,
    path_util::get_supreme_folder,
};

#[tauri::command]
pub fn set_first_player_type(character: String) -> Result<(), String> {
    let character = character.trim().to_lowercase();
    if get_first_player_type()? == character {
        return Ok(());
    }

    let mut config = parse_config(get_player_types_path()).map_err(|e| e.to_string())?;
    let idx = config
        .iter()
        .position(|x| {
            get_character_name(x)
                .map(|s| s == character)
                .unwrap_or(false)
        })
        .ok_or_else(|| format!("Character {character} not found"))?;

    let entry = config.remove(idx);
    config.insert(0, entry);

    write_config_to_file(get_player_types_path(), &config)
        .map_err(|e| format!("Failed to write player types: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_first_player_type() -> Result<String, String> {
    // return Ok("HEY!".to_string());
    let config: Vec<ConfigEntry> =
        parse_config(get_player_types_path()).map_err(|e| e.to_string())?;
    get_character_name(&config[0])
}

fn get_character_name(entry: &ConfigEntry) -> Result<String, String> {
    let mesh_value = entry
        .entries
        .iter()
        .find(|x| x.0 == "mesh_config_file")
        .map(|x| x.1.clone())
        .ok_or_else(|| "mesh_config_file not found".to_string())?;

    // pattern: data/characters/vincent/plrmshcnf.txt
    // extract the character name "vincent"
    mesh_value
        .replace("\"", "")
        .split("data/characters/")
        .nth(1)
        .ok_or_else(|| format!("{} needs to start with data/characters", mesh_value).to_string())?
        .split('/')
        .next()
        .ok_or_else(|| "Invalid path format".to_string())
        .map(|s| s.to_string())
}

pub fn get_player_types_path() -> PathBuf {
    get_supreme_folder()
        .join("Data")
        .join("Characters")
        .join("player_types.txt")
}
