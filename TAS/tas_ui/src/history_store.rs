//! Legacy (v1) `history.json` loader — kept ONLY for the one-time migration
//! into the v2 file-per-entry store (see `history_store_v2`). The v1 writer
//! (the JSON background writer) is gone; only reading remains.

use crate::recording::PersistedHistory;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct LoadedHistory {
    pub path: PathBuf,
    pub history: PersistedHistory,
}

/// Load the newest legacy `history.json` under `root` (by mtime), healing
/// pre-`created_at_iso` entries so day-grouping stays correct.
pub(crate) fn load_latest_history_from_root(
    root: &Path,
) -> Result<Option<LoadedHistory>, String> {
    if !root.exists() {
        return Ok(None);
    }

    let mut latest: Option<(SystemTime, PathBuf)> = None;
    let dir_entries = std::fs::read_dir(root)
        .map_err(|e| format!("failed to read history root {}: {}", root.display(), e))?;

    for dir_entry in dir_entries.flatten() {
        let path = dir_entry.path();
        if !path.is_dir() {
            continue;
        }
        let history_path = path.join("history.json");
        if !history_path.is_file() {
            continue;
        }

        let modified = history_path
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let replace = latest
            .as_ref()
            .map(|(current_time, _)| modified > *current_time)
            .unwrap_or(true);
        if replace {
            latest = Some((modified, history_path));
        }
    }

    let Some((modified, path)) = latest else {
        return Ok(None);
    };

    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("failed to read persisted history {}: {}", path.display(), e))?;
    let mut history: PersistedHistory = serde_json::from_str(&json)
        .map_err(|e| format!("failed to parse persisted history {}: {}", path.display(), e))?;

    // Heal entries that pre-date the `created_at_iso` field: fill from the
    // file's mtime + the entry's HH:MM:SS, and walk any future-dated timestamp
    // (from an earlier broken migration) back a day at a time.
    let modified_date: chrono::DateTime<chrono::Local> = modified.into();
    let date_str = modified_date.format("%Y-%m-%d").to_string();
    let offset_seconds = chrono::Local::now().offset().local_minus_utc();
    let offset_sign = if offset_seconds >= 0 { '+' } else { '-' };
    let abs = offset_seconds.unsigned_abs();
    let offset_str = format!("{}{:02}:{:02}", offset_sign, abs / 3600, (abs % 3600) / 60);
    let now = chrono::Local::now();
    let future_slop = chrono::Duration::minutes(5);
    for entry in &mut history.entries {
        if entry.created_at_iso.is_empty() {
            entry.created_at_iso = format!("{}T{}{}", date_str, entry.timestamp, offset_str);
        }
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&entry.created_at_iso) {
            let mut dt = parsed.with_timezone(&chrono::Local);
            let mut healed = false;
            while dt > now + future_slop {
                dt -= chrono::Duration::days(1);
                healed = true;
            }
            if healed {
                entry.created_at_iso = dt.to_rfc3339();
            }
        }
    }

    Ok(Some(LoadedHistory { path, history }))
}

/// Root directory for all per-install TAS data (history, recovery, recordings,
/// logs, screenshots). Resolution order:
///   1. `SSB_INSPECT_DATA_DIR` env override (tests / custom setups).
///   2. **Per-game folder** — when tas_ui is deployed at
///      `<game>/Display_Config_Resources/TAS/tas_ui.exe`, data lives in a
///      `data/` subfolder next to the exe (`…/TAS/data`). Keeping it a level
///      below the binaries avoids the case-insensitive `tas`/`TAS` collision on
///      Windows and keeps data out of the deploy/binary folder, while each game
///      install's recordings/history stay separate (different builds ship
///      different tracks).
///   3. `~/.ssb-inspector` — dev / un-deployed fallback.
pub fn default_history_root_dir() -> PathBuf {
    if let Some(over) = std::env::var_os("SSB_INSPECT_DATA_DIR") {
        return PathBuf::from(over);
    }
    if let Some(dir) = game_data_root_from_exe() {
        return dir;
    }
    user_home_dir()
        .map(|home| home.join(".ssb-inspector"))
        .unwrap_or_else(|| PathBuf::from(".ssb-inspector"))
}

/// `<game>/Display_Config_Resources/TAS/data` when the running exe is deployed
/// inside a `Display_Config_Resources/TAS` folder; `None` otherwise (e.g. a dev
/// build under `target/release`, so it doesn't write a stray `data` dir there).
/// The `Display_Config_Resources` parent check is the deployment guard; the data
/// itself goes in a `data/` subfolder of the exe dir (not a `tas` sibling, which
/// collides with `TAS` on case-insensitive Windows).
fn game_data_root_from_exe() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let exe_dir = exe.parent()?; // …/Display_Config_Resources/TAS
    let parent = exe_dir.parent()?; // …/Display_Config_Resources
    if parent
        .file_name()?
        .to_str()?
        .eq_ignore_ascii_case("Display_Config_Resources")
    {
        Some(exe_dir.join("data"))
    } else {
        None
    }
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .or_else(|| {
            let drive = std::env::var_os("HOMEDRIVE")?;
            let path = std::env::var_os("HOMEPATH")?;
            let mut buf = PathBuf::from(drive);
            buf.push(path);
            Some(buf)
        })
}
