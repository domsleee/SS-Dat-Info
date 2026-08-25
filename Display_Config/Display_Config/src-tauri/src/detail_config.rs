use std::fs;

use serde::{Deserialize, Serialize};

use crate::path_util::get_supreme_folder;

/// The ground renderer draws the slope as vertical strips of rows (row
/// spacing 1.2m x detail step; step=1 at ground detail 4, 2 at detail 3, ...)
/// and SKIPS any strip longer than a hardcoded row cap (Supreme_Game.dll
/// FUN_100b0ef0: `if (rows < 1 || rows > 400) skip`). At detail 4 that made
/// terrain past 400 x 1.2 = 480m vanish as "shredded"/missing triangles
/// (measured live on Alpine Easy: 480 clean / 500 shredded; the game's own UI
/// capped distance at 450 for the same reason). Display_Config_Helper's
/// "Extended render distance" fix (extendRenderDistance.hpp, default ON)
/// patches the cap 400 -> 500, which lifts detail 4 to 500 x 1.2 = 600m.
///
/// Enforce a backstop here, at the single choke point every write goes
/// through, assuming the (default-on) patch: distance is clamped, keeping the
/// user's chosen tessellation quality, and the UI mirror in
/// RenderDistanceRow.vue surfaces the toggle-aware limit:
pub fn max_safe_render_distance(ground_detail: i32) -> i32 {
    match ground_detail {
        4 => 600,  // 500 rows x 1.2m (patched cap; 480 unpatched)
        3 => 1200, // 500 rows x 2.4m; also the overall sanity ceiling
        _ => 1200, // detail <= 2: row spacing >= 4.8m, cap never binds below 1200
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
    if detail_config.render_distance.is_some_and(|d| d > cap) {
        detail_config.render_distance = Some(cap);
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
