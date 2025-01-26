use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

#[tauri::command]
pub fn read_rd_config() -> String {
    let supreme_folder = get_supreme_folder();
    let rd_config = supreme_folder.join("rd_config.txt");

    let file = match File::open(rd_config) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let mut config = HashMap::new();

    for line in lines {
        if !line.contains('=') {
            continue;
        }
        let mut split = line.split('=');
        if let (Some(key), Some(val)) = (split.next(), split.next()) {
            config.insert(
                key.trim().to_string(),
                val.trim().trim_end_matches(';').to_string(),
            );
        }
    }

    let rd_config = RdConfig {
        api_name: config.get("api_name").unwrap_or(&String::new()).clone(),
        width: config
            .get("width")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        height: config
            .get("height")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        depth: config
            .get("depth")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        card_id: config
            .get("card_id")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        fullscreen: config
            .get("fullscreen")
            .and_then(|s| s.parse().ok())
            .unwrap_or(false),
    };

    serde_json::to_string(&rd_config)
        .unwrap_or_default()
        .replace("\"\\\"", "\"")
        .replace("\\\"\"", "\"")
}

#[tauri::command]
pub fn write_rd_config(config: RdConfig) -> Result<(), String> {
    let rd_config = get_supreme_folder().join("rd_config.txt");
    let mut file =
        File::create(&rd_config).map_err(|e| format!("Failed to create config file: {}", e))?;

    let configs = [
        format!("api_name = \"{}\";", config.api_name),
        format!("width = {};", config.width),
        format!("height = {};", config.height),
        format!("depth = {};", config.depth),
        format!("card_id = {};", config.card_id),
        format!("fullscreen = {};", if config.fullscreen { 1 } else { 0 }),
    ];

    configs
        .iter()
        .try_for_each(|line| writeln!(file, "{}", line))
        .map_err(|e| format!("Failed to write config: {}", e))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RdConfig {
    pub api_name: String,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub card_id: i32,
    pub fullscreen: bool,
}
