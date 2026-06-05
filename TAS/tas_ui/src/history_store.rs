use crate::recording::{PersistedHistory, RecordingHistory};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread::JoinHandle;
use std::time::SystemTime;

pub struct HistoryStore {
    path: PathBuf,
    last_len: usize,
    last_current_index: Option<usize>,
    // Background writer: the UI thread only clones the history (to_persisted)
    // and hands it off here; the worker does the expensive JSON serialize +
    // disk write. Without this, a 130MB history serializes for ~3.6s (debug)
    // on the UI thread every time the undo stack changes (e.g. REC stop).
    writer_tx: Option<Sender<WriteMsg>>,
    _writer: Option<JoinHandle<()>>,
}

enum WriteMsg {
    Write(PersistedHistory),
    // Carries an ack channel; the worker replies once every queued write has
    // hit disk. Used by tests and by Drop (flush on exit).
    Flush(Sender<()>),
}

pub struct LoadedHistory {
    pub path: PathBuf,
    pub history: PersistedHistory,
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

        let path = session_dir.join("history.json");
        let (writer_tx, writer_rx) = channel::<WriteMsg>();
        let writer_path = path.clone();
        let writer = std::thread::Builder::new()
            .name("history-writer".into())
            .spawn(move || {
                while let Ok(msg) = writer_rx.recv() {
                    // Drain everything currently queued: keep only the newest
                    // payload (coalesce bursts so we don't write the same big
                    // file N times) and remember a flush ack to answer last.
                    let mut latest: Option<PersistedHistory> = None;
                    let mut ack: Option<Sender<()>> = None;
                    let mut next = Some(msg);
                    while let Some(m) = next {
                        match m {
                            WriteMsg::Write(p) => latest = Some(p),
                            WriteMsg::Flush(a) => ack = Some(a),
                        }
                        next = writer_rx.try_recv().ok();
                    }
                    if let Some(payload) = latest {
                        match serde_json::to_string_pretty(&payload) {
                            Ok(json) => {
                                if let Err(e) = write_atomic(&writer_path, json.as_bytes()) {
                                    eprintln!("[history] background write failed: {}", e);
                                }
                            }
                            Err(e) => eprintln!("[history] serialize failed: {}", e),
                        }
                    }
                    if let Some(ack) = ack {
                        let _ = ack.send(());
                    }
                }
            })
            .map_err(|e| format!("failed to spawn history writer thread: {}", e))?;

        Ok(Self {
            path,
            // Force first write so the file appears immediately.
            last_len: usize::MAX,
            last_current_index: None,
            writer_tx: Some(writer_tx),
            _writer: Some(writer),
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

        // Clone the history (cheap-ish memcpy) on the UI thread, then hand the
        // expensive serialize + disk write to the background writer.
        let payload = history.to_persisted();
        if let Some(tx) = &self.writer_tx {
            let _ = tx.send(WriteMsg::Write(payload));
        }

        self.last_len = len;
        self.last_current_index = current_index;
        Ok(true)
    }

    /// Block until every queued history write has hit disk. Called by Drop
    /// (so the latest undo state survives app exit) and by tests.
    pub fn flush(&self) {
        if let Some(tx) = &self.writer_tx {
            let (ack_tx, ack_rx) = channel();
            if tx.send(WriteMsg::Flush(ack_tx)).is_ok() {
                let _ = ack_rx.recv();
            }
        }
    }
}

impl Drop for HistoryStore {
    fn drop(&mut self) {
        self.flush();
        // Drop the sender so the worker's recv() returns Err and the loop ends,
        // then join so the thread is gone before we return.
        self.writer_tx = None;
        if let Some(worker) = self._writer.take() {
            let _ = worker.join();
        }
    }
}

// Write to a sibling temp file then rename, so a crash mid-write can never
// leave a truncated history.json behind. Runs on the background writer thread.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let tmp = path.with_extension(format!("json.{}.tmp", nonce));
    std::fs::write(&tmp, bytes)
        .map_err(|e| format!("failed to write temp {}: {}", tmp.display(), e))?;
    if let Err(rename_err) = std::fs::rename(&tmp, path) {
        // Windows refuses rename-over-existing; remove then retry.
        if path.exists() && std::fs::remove_file(path).is_ok() {
            std::fs::rename(&tmp, path)
                .map_err(|e| format!("failed to finalize {}: {}", path.display(), e))?;
            return Ok(());
        }
        let _ = std::fs::remove_file(&tmp);
        return Err(format!("failed to rename into {}: {}", path.display(), rename_err));
    }
    Ok(())
}

pub fn load_latest_history() -> Result<Option<LoadedHistory>, String> {
    load_latest_history_from_root(&default_history_root_dir())
}

fn load_latest_history_from_root(root: &Path) -> Result<Option<LoadedHistory>, String> {
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
    let mut history: PersistedHistory = serde_json::from_str(&json).map_err(|e| {
        format!(
            "failed to parse persisted history {}: {}",
            path.display(),
            e
        )
    })?;

    // Migrate entries that pre-date the `created_at_iso` field. Without
    // this, the apply_persisted fallback assigns "today" as the date,
    // making a history loaded from yesterday's session render every
    // 21:14 entry under "Today · 25 May" even though they were
    // recorded yesterday.
    //
    // Use the history file's mtime (when it was last written) as the
    // baseline date. The session_dir name encodes when the *current*
    // tas_ui launch started, which is wrong — we want the date the
    // entries were actually recorded. mtime tracks that closely
    // because the auto-save writes the file on every history change.
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
            // Legacy entry: fill from file mtime + entry's HH:MM:SS.
            entry.created_at_iso = format!(
                "{}T{}{}",
                date_str,
                entry.timestamp,
                offset_str,
            );
        }
        // Heal earlier broken migrations that wrote future-dated
        // timestamps. A previous version of this code defaulted to
        // `Local::now()` whenever `created_at_iso` was empty, so any
        // entry recorded at e.g. 21:13 on day N got migrated into a
        // 21:13 timestamp on day N+1 (today's date with yesterday's
        // time-of-day). Those rows then bunch under "Today" even
        // though they were recorded yesterday. Entries can't legally
        // be created in the future, so if the parsed timestamp is
        // ahead of now, walk it back a day at a time until it isn't.
        if let Ok(parsed) =
            chrono::DateTime::parse_from_rfc3339(&entry.created_at_iso)
        {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
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
    fn persist_if_changed_writes_restorable_history_json() {
        let root = unique_temp_root("tas_ui_history_store_write");
        std::fs::create_dir_all(&root).unwrap();

        let mut store = HistoryStore::new_in_root(root.clone()).unwrap();
        let history = sample_history();

        assert!(store.persist_if_changed(&history).unwrap());
        store.flush();
        let json = std::fs::read_to_string(store.path()).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["version"], 2);
        assert_eq!(value["entries"].as_array().unwrap().len(), 3);
        assert_eq!(value["entries"][0]["kind"], "snapshot");
        assert_eq!(value["entries"][2]["kind"], "save_marker");
        assert_eq!(value["entries"][0]["snapshot"]["recorded_count"], 1);
        assert_eq!(value["entries"][0]["snapshot"]["input_log"][0], 1);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn load_latest_history_round_trips_snapshots() {
        let root = unique_temp_root("tas_ui_history_store_load");
        std::fs::create_dir_all(&root).unwrap();

        let mut store = HistoryStore::new_in_root(root.clone()).unwrap();
        let history = sample_history();
        assert!(store.persist_if_changed(&history).unwrap());
        store.flush();

        let loaded = load_latest_history_from_root(&root).unwrap().unwrap();
        let mut restored = RecordingHistory::new(8);
        restored.apply_persisted(loaded.history).unwrap();

        assert_eq!(restored.len(), history.len());
        let snap = restored.restore_index(0).unwrap();
        assert_eq!(snap.recorded_count, 1);
        assert_eq!(snap.input_log[0], 0x01);

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
