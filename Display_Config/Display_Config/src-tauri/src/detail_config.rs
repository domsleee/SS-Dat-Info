use std::fs;

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[tauri::command]
pub fn write_detail_config(detail_config: DetailConfig) -> Result<(), String> {
    let rd_config = get_supreme_folder().join("Data").join("Detail_Config.txt");

    let content = fs::read_to_string(&rd_config)
        .map_err(|e| format!("Failed to read {rd_config:?}: {}", e))?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let configs = [
        (
            "visibility_cube_width",
            detail_config.render_distance.to_string(),
        ),
        ("ground_detail", detail_config.ground_detail.to_string()),
    ];

    let mut found = vec![false; configs.len()];

    // Update existing settings
    for line_idx in 0..lines.len() {
        for (config_idx, (key, value)) in configs.iter().enumerate() {
            if lines[line_idx].starts_with(key) {
                lines[line_idx] = format!("{}\t= {};", key, value);
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

    fs::write(&rd_config, lines.join("\n"))
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailConfig {
    pub render_distance: i32,
    pub ground_detail: i32,
}
