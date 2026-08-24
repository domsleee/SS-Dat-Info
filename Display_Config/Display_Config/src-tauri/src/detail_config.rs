use std::fs;

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

/// The engine caps VISIBLE terrain at 65,536 vertices — a 16-bit-addressable
/// vertex pool in the mesh build (the GL draw calls themselves already use
/// GL_UNSIGNED_INT, so the wall is upstream). Past the budget, wrapped slots
/// draw the far patches as shredded/missing triangles (reported on Alpine Easy at
/// distance 600 + detail 4, and reproduced live: shredding starts between 480
/// and 500 at detail 4, exactly where 17x17-vertex patches exhaust the index
/// space; at detail 3 the same 600m renders clean, as the budget predicts).
/// The game's own UI capped distance at 450 for the same reason.
///
/// Enforce the pair here, at the single choke point every write goes through.
/// Distance is clamped (keeping the user's chosen tessellation quality); the
/// UI surfaces the cap so raising Ground Detail visibly lowers the max.
/// Measured-safe caps with margin, per ground_detail level:
pub fn max_safe_render_distance(ground_detail: i32) -> i32 {
    match ground_detail {
        4 => 480,  // 289 verts/patch -> budget exhausts ~482 (measured 480 ok / 500 shredded)
        3 => 600,  // 169 verts/patch -> ~630 theoretical; 600 verified clean in-game
        2 => 900,  // 81 verts/patch
        _ => 1200, // detail 1/0: far below budget at any sane distance
    }
}

#[tauri::command]
#[specta::specta]
pub fn write_detail_config(detail_config: DetailConfig) -> Result<(), String> {
    let mut detail_config = detail_config;
    // Clamp against the CURRENT file's ground_detail when the write doesn't
    // carry one, so a distance-only update still respects the budget.
    let gd = match detail_config.ground_detail {
        Some(g) => g,
        None => read_detail_config()
            .ok()
            .and_then(|kv| {
                kv.iter()
                    .find(|(k, _)| k == "ground_detail")
                    .and_then(|(_, v)| v.parse().ok())
            })
            .unwrap_or(4),
    };
    let cap = max_safe_render_distance(gd);
    if let Some(d) = detail_config.render_distance {
        if d > cap {
            detail_config.render_distance = Some(cap);
        }
    }
    let detail_config_path = get_detail_config_path();

    let content = fs::read_to_string(&detail_config_path)
        .map_err(|e| format!("Failed to read {detail_config_path:?}: {e}"))?;

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
                *line = format!("{key}\t= {value};");
                found[config_idx] = true;
                break;
            }
        }
    }

    // Append missing settings
    for (config_idx, (key, value)) in configs.iter().enumerate() {
        if !found[config_idx] {
            lines.push(format!("{key}\t= {value};"));
        }
    }

    fs::write(&detail_config_path, lines.join("\n"))
        .map_err(|e| format!("Failed to write config file: {e}"))?;

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
        .map_err(|e| format!("Failed to read {detail_config_path:?}: {e}"))?;

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
