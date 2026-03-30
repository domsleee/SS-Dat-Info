use crate::recording::{HistoryEntryKind, RecordingHistory};
use serde::Serialize;
use std::path::{Path, PathBuf};

pub struct HistoryStore {
    path: PathBuf,
    last_len: usize,
    last_current_index: Option<usize>,
}

impl HistoryStore {
    pub fn new() -> Result<Self, String> {
        Self::new_in_root(default_history_root_dir())
    }

    fn new_in_root(root: PathBuf) -> Result<Self, String> {
        let session_dir = root.join(session_dir_name());
        std::fs::create_dir_all(&session_dir).map_err(|e| {
            format!(
                "failed to create history session dir {}: {}",
                session_dir.display(),
                e
            )
        })?;

        Ok(Self {
            path: session_dir.join("history.json"),
            // Force first write so the file appears immediately.
            last_len: usize::MAX,
            last_current_index: None,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn persist_if_changed(&mut self, history: &RecordingHistory) -> Result<bool, String> {
        let len = history.len();
        let current_index = history.current_index();

        if self.last_len == len && self.last_current_index == current_index {
            return Ok(false);
        }

        let payload = PersistedHistory::from_history(history);
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("failed to serialize history: {}", e))?;
        std::fs::write(&self.path, json).map_err(|e| {
            format!(
                "failed to write history file {}: {}",
                self.path.display(),
                e
            )
        })?;

        self.last_len = len;
        self.last_current_index = current_index;
        Ok(true)
    }
}

fn session_dir_name() -> String {
    chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string()
}

fn default_history_root_dir() -> PathBuf {
    user_home_dir()
        .map(|home| home.join(".ssb-inspector"))
        .unwrap_or_else(|| PathBuf::from(".ssb-inspector"))
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

#[derive(Serialize)]
struct PersistedHistory {
    version: u32,
    saved_at: String,
    current_index: Option<usize>,
    entries: Vec<PersistedHistoryEntry>,
}

impl PersistedHistory {
    fn from_history(history: &RecordingHistory) -> Self {
        let current_index = history.current_index();
        let entries = history
            .entries()
            .iter()
            .enumerate()
            .map(|(index, entry)| PersistedHistoryEntry {
                index,
                timestamp: entry.timestamp.clone(),
                label: entry.label.clone(),
                kind: kind_name(entry.kind),
                can_restore: entry.can_restore(),
                is_current: current_index == Some(index),
            })
            .collect();

        Self {
            version: 1,
            saved_at: chrono::Local::now().to_rfc3339(),
            current_index,
            entries,
        }
    }
}

#[derive(Serialize)]
struct PersistedHistoryEntry {
    index: usize,
    timestamp: String,
    label: String,
    kind: &'static str,
    can_restore: bool,
    is_current: bool,
}

fn kind_name(kind: HistoryEntryKind) -> &'static str {
    match kind {
        HistoryEntryKind::Snapshot => "snapshot",
        HistoryEntryKind::SaveMarker => "save_marker",
        HistoryEntryKind::LoadSnapshot => "load_snapshot",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tas_shared::zeroed_boxed;

    fn unique_temp_root(prefix: &str) -> PathBuf {
        let nonce = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default();
        std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), nonce))
    }

    fn sample_history() -> RecordingHistory {
        let mut history = RecordingHistory::new(8);
        let mut state = zeroed_boxed();
        state.recorded_count = 1;

        state.input_log[0] = 0x01;
        state.rec_coords[0] = [1.0, 0.0, 0.0];
        assert!(history.push_snapshot(&state, "A"));

        state.input_log[0] = 0x02;
        state.rec_coords[0] = [2.0, 0.0, 0.0];
        assert!(history.push_snapshot(&state, "B"));

        history.push_save_marker(&state, Path::new("C:\\temp\\run.tasrec"));
        history
    }

    #[test]
    fn persist_if_changed_writes_history_json() {
        let root = unique_temp_root("tas_ui_history_store_write");
        std::fs::create_dir_all(&root).unwrap();

        let mut store = HistoryStore::new_in_root(root.clone()).unwrap();
        let history = sample_history();

        assert!(store.persist_if_changed(&history).unwrap());
        let json = std::fs::read_to_string(store.path()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["version"], 1);
        assert_eq!(value["entries"].as_array().unwrap().len(), 3);
        assert_eq!(value["entries"][0]["kind"], "snapshot");
        assert_eq!(value["entries"][2]["kind"], "save_marker");

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn persist_if_changed_skips_unchanged_state() {
        let root = unique_temp_root("tas_ui_history_store_unchanged");
        std::fs::create_dir_all(&root).unwrap();

        let mut store = HistoryStore::new_in_root(root.clone()).unwrap();
        let mut history = sample_history();

        assert!(store.persist_if_changed(&history).unwrap());
        assert!(!store.persist_if_changed(&history).unwrap());

        let _ = history.undo();
        assert!(store.persist_if_changed(&history).unwrap());

        let _ = std::fs::remove_dir_all(root);
    }
}
