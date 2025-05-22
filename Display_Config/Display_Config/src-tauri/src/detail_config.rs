use std::fs;

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[tauri::command]
#[specta::specta]
pub fn write_detail_config(detail_config: DetailConfig) -> Result<(), String> {
    let detail_config_path = get_detail_config_path();

    let content = fs::read_to_string(&detail_config_path)
        .map_err(|e| format!("Failed to read {detail_config_path:?}: {}", e))?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let configs: Vec<(String, String)> = [
        (
            "visibility_cube_width",
            detail_config.render_distance.map(|n| n.to_string()),
        ),
        (
            "ground_detail",
            detail_config.ground_detail.map(|n| n.to_string()),
        ),
    ]
    .into_iter()
    .filter_map(|(key, opt_value)| opt_value.map(|value| (key.to_string(), value)))
    .collect();

    let mut found = vec![false; configs.len()];

    // Update existing settings
    for line in &mut lines {
        for (config_idx, (key, value)) in configs.iter().enumerate() {
            if line.starts_with(key) {
                *line = format!("{}\t= {};", key, value);
                found[config_idx] = true;
                break;
            }
        }
    }

    // Append missing settings
    for (config_idx, (key, value)) in configs.iter().enumerate() {
        if !found[config_idx] {
            lines.push(format!("{}\t= {};", key, value));
        }
    }

    fs::write(&detail_config_path, lines.join("\n"))
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

fn get_detail_config_path() -> std::path::PathBuf {
    get_supreme_folder().join("Data").join("Detail_Config.txt")
}

#[tauri::command]
#[specta::specta]
pub fn read_detail_config() -> Result<Vec<(String, String)>, String> {
    let detail_config_path = get_detail_config_path();
    let mut res = Vec::new();
    let content = fs::read_to_string(&detail_config_path)
        .map_err(|e| format!("Failed to read {detail_config_path:?}: {}", e))?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    for line in lines.iter_mut() {
        let mut split = line.split('=');
        if let (Some(key), Some(val)) = (split.next(), split.next()) {
            res.push((
                key.trim().to_string(),
                val.trim().trim_end_matches(';').to_string(),
            ));
        }
    }

    Ok(res)
}

#[derive(Debug, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DetailConfig {
    pub render_distance: Option<i32>,
    pub ground_detail: Option<i32>,
}
