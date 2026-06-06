//! File-per-entry persistent history store (v2). See HISTORY_STORE_PLAN.md.
//!
//! Layout: `<dir>/<entry_id>.tasrec` (one immutable blob per snapshot entry)
//! plus `<dir>/manifest.json` (ordered metadata + cursor + next_entry_id).
//!
//! Identity is the stable `entry_id` (NOT a positional index). Blobs are
//! id-named (no content hashing): snapshots are ~always unique, so dedup buys
//! nothing and hashing would force a canonical blob format + refcount GC.
//! Integrity is a per-entry crc32 checksum stored in the manifest.

// Not yet wired into the app (Phase 2); silence until then.
#![allow(dead_code)]

use crate::recording::{HistoryEntryKind, PersistedSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread::JoinHandle;

const MAGIC: &str = "ssb-history";
const SCHEMA: u32 = 1;
const BLOB_FORMAT: u32 = 1;
const HASH_ALGO: &str = "crc32";

/// One entry the caller wants persisted. `snapshot == None` => a marker entry
/// (e.g. SaveMarker) that carries no blob.
#[derive(Clone, PartialEq, Debug)]
pub struct StoredEntry {
    pub entry_id: u64,
    pub name: String,
    /// User-given name (rename); `None` = use the auto `name`/duration.
    pub user_name: Option<String>,
    pub pinned: bool,
    pub kind: HistoryEntryKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub first_moving: Option<u32>,
    pub created_at_iso: String,
    pub snapshot: Option<PersistedSnapshot>,
}

/// One entry as loaded from disk. `available == false` means a snapshot entry
/// whose blob was missing/corrupt — kept visible but inert (cannot restore).
#[derive(Clone, PartialEq, Debug)]
pub struct LoadedEntry {
    pub entry_id: u64,
    pub name: String,
    pub user_name: Option<String>,
    pub pinned: bool,
    pub kind: HistoryEntryKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub first_moving: Option<u32>,
    pub created_at_iso: String,
    pub snapshot: Option<PersistedSnapshot>,
    pub available: bool,
}

#[derive(Debug, Default)]
pub struct LoadResult {
    pub entries: Vec<LoadedEntry>,
    pub current_entry_id: Option<u64>,
    pub next_entry_id: u64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct PersistOutcome {
    pub blobs_written: usize,
    pub blobs_deleted: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct ManifestEntry {
    entry_id: u64,
    name: String,
    #[serde(default)]
    user_name: Option<String>,
    pinned: bool,
    kind: HistoryEntryKind,
    start_tick: u32,
    end_tick: u32,
    first_moving: Option<u32>,
    created_at_iso: String,
    /// `None` for marker entries with no blob.
    size: Option<u64>,
    checksum: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    magic: String,
    schema: u32,
    blob_format: u32,
    hash_algo: String,
    next_entry_id: u64,
    current_entry_id: Option<u64>,
    entries: Vec<ManifestEntry>,
}

#[derive(Clone, Copy)]
struct BlobInfo {
    size: u64,
    checksum: u32,
}

pub struct HistoryStoreV2 {
    dir: PathBuf,
    /// Entry ids currently referenced by the on-disk manifest.
    referenced: HashSet<u64>,
    /// Blob info for ids that currently have a blob on disk (lets us skip
    /// re-serializing immutable blobs and carry checksums into the manifest).
    blobs: HashMap<u64, BlobInfo>,
    /// Ids whose manifest entry references a blob that was missing/corrupt at
    /// load. We preserve their blob metadata across re-persists so a transient
    /// disappearance doesn't permanently demote a snapshot entry to a marker
    /// (and so the entry recovers if the file reappears).
    unavailable: HashMap<u64, BlobInfo>,
}

impl HistoryStoreV2 {
    pub fn open_in(dir: PathBuf) -> Result<(Self, LoadResult), String> {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("failed to create history dir {}: {}", dir.display(), e))?;

        let mut warnings = Vec::new();
        let disk_ids = list_blob_ids(&dir);
        let manifest = read_manifest(&dir, &mut warnings);

        let mut entries = Vec::new();
        let mut referenced = HashSet::new();
        let mut blobs = HashMap::new();
        let mut unavailable: HashMap<u64, BlobInfo> = HashMap::new();
        let mut max_manifest_id = 0u64;
        let mut stored_next = 0u64;
        let mut persisted_current = None;

        if let Some(m) = &manifest {
            stored_next = m.next_entry_id;
            persisted_current = m.current_entry_id;
            for me in &m.entries {
                referenced.insert(me.entry_id);
                max_manifest_id = max_manifest_id.max(me.entry_id);

                let (snapshot, available) = match me.checksum {
                    None => (None, true), // marker entry, no blob expected
                    Some(expected) => match read_blob(&dir, me.entry_id, expected) {
                        Ok(snap) => {
                            blobs.insert(
                                me.entry_id,
                                BlobInfo {
                                    size: me.size.unwrap_or(0),
                                    checksum: expected,
                                },
                            );
                            (Some(snap), true)
                        }
                        Err(BlobError::Missing) => {
                            warnings.push(format!(
                                "history entry {} ('{}') blob missing — kept but unavailable",
                                me.entry_id, me.name
                            ));
                            unavailable.insert(
                                me.entry_id,
                                BlobInfo {
                                    size: me.size.unwrap_or(0),
                                    checksum: expected,
                                },
                            );
                            (None, false)
                        }
                        Err(BlobError::Corrupt) => {
                            // Quarantine so we don't re-hit it every launch.
                            quarantine_blob(&dir, me.entry_id);
                            warnings.push(format!(
                                "history entry {} ('{}') blob corrupt — quarantined, unavailable",
                                me.entry_id, me.name
                            ));
                            unavailable.insert(
                                me.entry_id,
                                BlobInfo {
                                    size: me.size.unwrap_or(0),
                                    checksum: expected,
                                },
                            );
                            (None, false)
                        }
                    },
                };

                entries.push(LoadedEntry {
                    entry_id: me.entry_id,
                    name: me.name.clone(),
                    user_name: me.user_name.clone(),
                    pinned: me.pinned,
                    kind: me.kind,
                    start_tick: me.start_tick,
                    end_tick: me.end_tick,
                    first_moving: me.first_moving,
                    created_at_iso: me.created_at_iso.clone(),
                    snapshot,
                    available,
                });
            }
        }

        // GC: delete blob files not referenced by the manifest — but ONLY when
        // we actually loaded a usable manifest. If the manifest is absent or
        // unparseable we must NOT delete blobs (that would turn a transiently
        // bad manifest into permanent data loss).
        if manifest.is_some() {
            for id in &disk_ids {
                if !referenced.contains(id) {
                    let p = blob_path(&dir, *id);
                    if std::fs::remove_file(&p).is_ok() {
                        warnings.push(format!("removed orphan blob {}", p.display()));
                    }
                }
            }
        } else if !disk_ids.is_empty() {
            warnings.push(format!(
                "no usable manifest — {} blob(s) preserved (not GC'd)",
                disk_ids.len()
            ));
        }

        let max_disk_id = disk_ids.iter().copied().max().unwrap_or(0);
        let next_entry_id = stored_next
            .max(max_manifest_id.saturating_add(1))
            .max(max_disk_id.saturating_add(1));

        // Resolve the cursor by row order: if the persisted current entry is
        // unavailable, fall back to the nearest previous available entry, else
        // the nearest next, else none.
        let current_entry_id = resolve_current(&entries, persisted_current, &mut warnings);

        Ok((
            Self {
                dir,
                referenced,
                blobs,
                unavailable,
            },
            LoadResult {
                entries,
                current_entry_id,
                next_entry_id,
                warnings,
            },
        ))
    }

    /// Incrementally sync the on-disk store to `entries` (in row order).
    /// Writes blobs for newly-appearing snapshot ids, deletes blobs for removed
    /// ids, and atomically replaces the manifest.
    pub fn persist(
        &mut self,
        entries: &[StoredEntry],
        current_entry_id: Option<u64>,
        next_entry_id: u64,
    ) -> Result<PersistOutcome, String> {
        let desired_ids: HashSet<u64> = entries.iter().map(|e| e.entry_id).collect();
        let mut outcome = PersistOutcome::default();

        // 1. Write blobs for new snapshot-bearing ids (immutable, write-once).
        let mut manifest_entries = Vec::with_capacity(entries.len());
        for e in entries {
            let (size, checksum) = match &e.snapshot {
                None => {
                    // No bytes in hand. If this id is a snapshot entry whose
                    // blob is currently missing/corrupt, preserve its manifest
                    // blob reference (don't demote it to a marker) so it can
                    // recover if the file reappears.
                    match self.unavailable.get(&e.entry_id) {
                        Some(info) => (Some(info.size), Some(info.checksum)),
                        None => (None, None),
                    }
                }
                Some(snap) => {
                    if let Some(info) = self.blobs.get(&e.entry_id) {
                        (Some(info.size), Some(info.checksum))
                    } else {
                        let info = self.write_blob(e.entry_id, snap)?;
                        self.blobs.insert(e.entry_id, info);
                        outcome.blobs_written += 1;
                        (Some(info.size), Some(info.checksum))
                    }
                }
            };
            manifest_entries.push(ManifestEntry {
                entry_id: e.entry_id,
                name: e.name.clone(),
                user_name: e.user_name.clone(),
                pinned: e.pinned,
                kind: e.kind,
                start_tick: e.start_tick,
                end_tick: e.end_tick,
                first_moving: e.first_moving,
                created_at_iso: e.created_at_iso.clone(),
                size,
                checksum,
            });
        }

        // 2. Publish the manifest atomically (this is the commit point).
        let manifest = Manifest {
            magic: MAGIC.to_string(),
            schema: SCHEMA,
            blob_format: BLOB_FORMAT,
            hash_algo: HASH_ALGO.to_string(),
            next_entry_id,
            current_entry_id,
            entries: manifest_entries,
        };
        write_manifest_atomic(&self.dir, &manifest)?;

        // 3. Only AFTER the manifest no longer references them, delete blobs.
        let removed: Vec<u64> = self
            .referenced
            .iter()
            .copied()
            .filter(|id| !desired_ids.contains(id))
            .collect();
        for id in removed {
            self.unavailable.remove(&id);
            if self.blobs.remove(&id).is_some() {
                let p = blob_path(&self.dir, id);
                if std::fs::remove_file(&p).is_ok() {
                    outcome.blobs_deleted += 1;
                }
            }
        }

        self.referenced = desired_ids;
        Ok(outcome)
    }

    fn write_blob(&self, id: u64, snapshot: &PersistedSnapshot) -> Result<BlobInfo, String> {
        let bytes = bincode::serialize(snapshot)
            .map_err(|e| format!("failed to serialize blob {}: {}", id, e))?;
        let checksum = crc32fast::hash(&bytes);
        let size = bytes.len() as u64;
        let final_path = blob_path(&self.dir, id);

        // Immutable blobs are never overwritten. A pre-existing file at this id
        // is a stray (ids are never reused) — quarantine it before writing.
        if final_path.exists() {
            quarantine_blob(&self.dir, id);
        }
        let tmp = self.dir.join(format!("{}.tasrec.tmp", id));
        std::fs::write(&tmp, &bytes)
            .map_err(|e| format!("failed to write blob tmp {}: {}", tmp.display(), e))?;
        std::fs::rename(&tmp, &final_path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp);
            format!("failed to finalize blob {}: {}", final_path.display(), e)
        })?;
        Ok(BlobInfo { size, checksum })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }
}

/// Default v2 store directory: `<history-root>/history/`.
pub fn default_history_dir() -> PathBuf {
    crate::history_store::default_history_root_dir().join("history")
}

/// If there is no v2 manifest yet but a legacy `history.json` exists, copy a
/// timestamped backup into the v2 dir and return the parsed legacy history for
/// the caller to apply + persist. Returns `None` if already on v2 or there's no
/// legacy data. Non-destructive: the original legacy files are left in place.
pub fn migrate_legacy(
    v2_dir: &Path,
) -> Result<Option<(PathBuf, crate::recording::PersistedHistory)>, String> {
    migrate_legacy_in(&crate::history_store::default_history_root_dir(), v2_dir)
}

fn migrate_legacy_in(
    legacy_root: &Path,
    v2_dir: &Path,
) -> Result<Option<(PathBuf, crate::recording::PersistedHistory)>, String> {
    if v2_dir.join("manifest.json").exists() {
        return Ok(None); // already on v2 (incl. a deliberately-empty store)
    }
    let loaded = match crate::history_store::load_latest_history_from_root(legacy_root)? {
        Some(l) => l,
        None => return Ok(None), // no legacy data to migrate
    };
    std::fs::create_dir_all(v2_dir)
        .map_err(|e| format!("failed to create v2 dir {}: {}", v2_dir.display(), e))?;
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let backup = v2_dir.join(format!("legacy-backup-{}.json", ts));
    std::fs::copy(&loaded.path, &backup).map_err(|e| {
        format!(
            "failed to back up legacy history {} -> {}: {}",
            loaded.path.display(),
            backup.display(),
            e
        )
    })?;
    Ok(Some((backup, loaded.history)))
}

/// Background writer: owns the store on a worker thread so the expensive
/// serialize + disk writes never touch the UI thread. The UI thread only
/// clones `to_stored_entries()` and hands it off. Bursts coalesce to the latest.
pub struct HistoryWriter {
    tx: Option<Sender<WriteMsg>>,
    worker: Option<JoinHandle<()>>,
}

enum WriteMsg {
    Persist {
        entries: Vec<StoredEntry>,
        current_entry_id: Option<u64>,
        next_entry_id: u64,
    },
    Flush(Sender<()>),
}

impl HistoryWriter {
    /// Open (loading existing data) and spawn the writer thread.
    pub fn open(dir: PathBuf) -> Result<(Self, LoadResult), String> {
        let (store, load) = HistoryStoreV2::open_in(dir)?;
        let (tx, rx) = channel::<WriteMsg>();
        let worker = std::thread::Builder::new()
            .name("history-v2-writer".into())
            .spawn(move || {
                let mut store = store;
                while let Ok(msg) = rx.recv() {
                    // Coalesce: keep only the newest desired state (each job is
                    // the full set; the store diffs it against disk), and answer
                    // the latest flush after writing.
                    let mut latest: Option<(Vec<StoredEntry>, Option<u64>, u64)> = None;
                    let mut ack: Option<Sender<()>> = None;
                    let mut next = Some(msg);
                    while let Some(m) = next {
                        match m {
                            WriteMsg::Persist {
                                entries,
                                current_entry_id,
                                next_entry_id,
                            } => latest = Some((entries, current_entry_id, next_entry_id)),
                            WriteMsg::Flush(a) => ack = Some(a),
                        }
                        next = rx.try_recv().ok();
                    }
                    if let Some((entries, current, next_id)) = latest {
                        if let Err(e) = store.persist(&entries, current, next_id) {
                            eprintln!("[history v2] persist failed: {}", e);
                        }
                    }
                    if let Some(ack) = ack {
                        let _ = ack.send(());
                    }
                }
            })
            .map_err(|e| format!("failed to spawn history writer: {}", e))?;
        Ok((
            Self {
                tx: Some(tx),
                worker: Some(worker),
            },
            load,
        ))
    }

    pub fn persist(
        &self,
        entries: Vec<StoredEntry>,
        current_entry_id: Option<u64>,
        next_entry_id: u64,
    ) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(WriteMsg::Persist {
                entries,
                current_entry_id,
                next_entry_id,
            });
        }
    }

    /// Block until every queued write has hit disk.
    pub fn flush(&self) {
        if let Some(tx) = &self.tx {
            let (a, r) = channel();
            if tx.send(WriteMsg::Flush(a)).is_ok() {
                let _ = r.recv();
            }
        }
    }
}

impl Drop for HistoryWriter {
    fn drop(&mut self) {
        self.flush();
        self.tx = None;
        if let Some(w) = self.worker.take() {
            let _ = w.join();
        }
    }
}

enum BlobError {
    Missing,
    Corrupt,
}

fn blob_path(dir: &Path, id: u64) -> PathBuf {
    dir.join(format!("{}.tasrec", id))
}

fn read_blob(dir: &Path, id: u64, expected_checksum: u32) -> Result<PersistedSnapshot, BlobError> {
    let path = blob_path(dir, id);
    let bytes = std::fs::read(&path).map_err(|_| BlobError::Missing)?;
    if crc32fast::hash(&bytes) != expected_checksum {
        return Err(BlobError::Corrupt);
    }
    bincode::deserialize(&bytes).map_err(|_| BlobError::Corrupt)
}

fn quarantine_blob(dir: &Path, id: u64) {
    let from = blob_path(dir, id);
    let to = dir.join(format!("{}.tasrec.corrupt", id));
    if std::fs::rename(&from, &to).is_err() {
        let _ = std::fs::remove_file(&from);
    }
}

fn list_blob_ids(dir: &Path) -> Vec<u64> {
    let mut ids = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            // exactly "<u64>.tasrec" (not .tmp/.corrupt)
            if let Some(stem) = name.strip_suffix(".tasrec") {
                if let Ok(id) = stem.parse::<u64>() {
                    ids.push(id);
                }
            }
        }
    }
    ids
}

fn read_manifest(dir: &Path, warnings: &mut Vec<String>) -> Option<Manifest> {
    let path = dir.join("manifest.json");
    if !path.exists() {
        return None;
    }
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            warnings.push(format!("manifest unreadable: {}", e));
            return None;
        }
    };
    let manifest: Manifest = match serde_json::from_slice(&bytes) {
        Ok(m) => m,
        Err(e) => {
            warnings.push(format!("manifest corrupt: {}", e));
            return None;
        }
    };
    if manifest.magic != MAGIC
        || manifest.schema != SCHEMA
        || manifest.blob_format != BLOB_FORMAT
        || manifest.hash_algo != HASH_ALGO
    {
        warnings.push(format!(
            "manifest incompatible (magic={}, schema={}, blob_format={}, hash_algo={}) — ignored",
            manifest.magic, manifest.schema, manifest.blob_format, manifest.hash_algo
        ));
        return None;
    }
    Some(manifest)
}

fn write_manifest_atomic(dir: &Path, manifest: &Manifest) -> Result<(), String> {
    let json = serde_json::to_vec_pretty(manifest)
        .map_err(|e| format!("failed to serialize manifest: {}", e))?;
    let tmp = dir.join("manifest.json.tmp");
    std::fs::write(&tmp, &json)
        .map_err(|e| format!("failed to write manifest tmp: {}", e))?;
    // std::fs::rename atomically replaces the destination (MoveFileExW with
    // MOVEFILE_REPLACE_EXISTING on Windows): old-or-new, never missing.
    std::fs::rename(&tmp, dir.join("manifest.json")).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("failed to publish manifest: {}", e)
    })
}

/// Resolve the persisted cursor against availability, by row order: keep it if
/// available; else nearest previous available, else nearest next, else none.
fn resolve_current(
    entries: &[LoadedEntry],
    persisted: Option<u64>,
    warnings: &mut Vec<String>,
) -> Option<u64> {
    let target = persisted?;
    let pos = entries.iter().position(|e| e.entry_id == target);
    let Some(pos) = pos else {
        return None; // cursor pointed at a now-absent entry
    };
    let restorable = |e: &LoadedEntry| e.available && e.snapshot.is_some();
    if restorable(&entries[pos]) {
        return Some(target);
    }
    // search backward, then forward, for the nearest restorable entry
    for i in (0..pos).rev() {
        if restorable(&entries[i]) {
            warnings.push(format!(
                "current entry {} unavailable — resolved to {}",
                target, entries[i].entry_id
            ));
            return Some(entries[i].entry_id);
        }
    }
    for e in entries.iter().skip(pos + 1) {
        if restorable(e) {
            warnings.push(format!(
                "current entry {} unavailable — resolved to {}",
                target, e.entry_id
            ));
            return Some(e.entry_id);
        }
    }
    warnings.push(format!(
        "current entry {} unavailable — no restorable entry left",
        target
    ));
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(tag: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("ssb_hist_v2_{}_{}_{}", tag, std::process::id(), nonce))
    }

    fn snap(count: u32, fill: u8) -> PersistedSnapshot {
        PersistedSnapshot {
            recorded_count: count,
            input_log: vec![fill; count as usize],
            rec_coords: vec![[fill as f32, 0.0, 1.0]; count as usize],
        }
    }

    fn entry(id: u64, name: &str, pinned: bool, count: u32) -> StoredEntry {
        StoredEntry {
            entry_id: id,
            name: name.to_string(),
            user_name: None,
            pinned,
            kind: HistoryEntryKind::Snapshot,
            start_tick: 0,
            end_tick: count,
            first_moving: None,
            created_at_iso: "2026-06-06T00:00:00+00:00".to_string(),
            snapshot: Some(snap(count, id as u8)),
        }
    }

    fn marker(id: u64, name: &str) -> StoredEntry {
        StoredEntry {
            entry_id: id,
            name: name.to_string(),
            user_name: None,
            pinned: false,
            kind: HistoryEntryKind::SaveMarker,
            start_tick: 0,
            end_tick: 0,
            first_moving: None,
            created_at_iso: "2026-06-06T00:00:00+00:00".to_string(),
            snapshot: None,
        }
    }

    /// Assert the loaded store matches the desired entries + cursor exactly.
    fn assert_loads_as(dir: &Path, desired: &[StoredEntry], cursor: Option<u64>) {
        let (_store, res) = HistoryStoreV2::open_in(dir.to_path_buf()).unwrap();
        assert_eq!(res.entries.len(), desired.len(), "entry count");
        for (got, want) in res.entries.iter().zip(desired.iter()) {
            assert_eq!(got.entry_id, want.entry_id);
            assert_eq!(got.name, want.name);
            assert_eq!(got.pinned, want.pinned);
            assert_eq!(got.kind, want.kind);
            assert_eq!(got.snapshot, want.snapshot, "snapshot for {}", want.entry_id);
            assert!(got.available);
        }
        assert_eq!(res.current_entry_id, cursor);
    }

    #[test]
    fn roundtrip_empty() {
        let dir = tmp_dir("empty");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store.persist(&[], None, 0).unwrap();
        assert_loads_as(&dir, &[], None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn roundtrip_entries_and_marker() {
        let dir = tmp_dir("rt");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3), entry(2, "B", true, 5), marker(3, "saved")];
        store.persist(&entries, Some(2), 4).unwrap();
        assert_loads_as(&dir, &entries, Some(2));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn append_writes_one_blob() {
        let dir = tmp_dir("append");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut entries = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        let o = store.persist(&entries, Some(2), 3).unwrap();
        assert_eq!(o.blobs_written, 2);
        entries.push(entry(3, "C", false, 3));
        let o = store.persist(&entries, Some(3), 4).unwrap();
        assert_eq!(o.blobs_written, 1, "append should write exactly one blob");
        assert_eq!(o.blobs_deleted, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn idempotent_persist_writes_nothing() {
        let dir = tmp_dir("idem");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3)];
        store.persist(&entries, Some(1), 2).unwrap();
        let o = store.persist(&entries, Some(1), 2).unwrap();
        assert_eq!(o, PersistOutcome { blobs_written: 0, blobs_deleted: 0 });
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn removed_entry_deletes_blob() {
        let dir = tmp_dir("rm");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        store.persist(&entries, Some(2), 3).unwrap();
        let trimmed = vec![entry(2, "B", false, 3)];
        let o = store.persist(&trimmed, Some(2), 3).unwrap();
        assert_eq!(o.blobs_deleted, 1);
        assert!(!blob_path(&dir, 1).exists());
        assert!(blob_path(&dir, 2).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn marker_has_no_blob() {
        let dir = tmp_dir("marker");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store.persist(&[marker(1, "saved")], None, 2).unwrap();
        assert!(!blob_path(&dir, 1).exists());
        assert_loads_as(&dir, &[marker(1, "saved")], None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_blob_marks_unavailable_no_crash() {
        let dir = tmp_dir("missing");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3), entry(2, "B", false, 3)], Some(2), 3)
            .unwrap();
        // Yank a blob from under the store.
        std::fs::remove_file(blob_path(&dir, 1)).unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 2, "bad entry still listed");
        assert!(!res.entries[0].available);
        assert!(res.entries[1].available);
        assert!(!res.warnings.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_blob_quarantined() {
        let dir = tmp_dir("corrupt");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store.persist(&[entry(1, "A", false, 3)], Some(1), 2).unwrap();
        std::fs::write(blob_path(&dir, 1), b"garbage").unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(!res.entries[0].available);
        assert!(dir.join("1.tasrec.corrupt").exists(), "corrupt blob quarantined");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn gc_orphan_blob_on_open() {
        let dir = tmp_dir("gc");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store.persist(&[entry(1, "A", false, 3)], Some(1), 2).unwrap();
        // Stray blob not referenced by the manifest.
        std::fs::write(blob_path(&dir, 99), b"orphan").unwrap();
        let (_s, _res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(!blob_path(&dir, 99).exists(), "orphan GC'd");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn next_entry_id_never_reuses_and_respects_disk() {
        let dir = tmp_dir("nextid");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        // next_entry_id stored as 100 even though max id is 2.
        store.persist(&[entry(2, "A", false, 3)], Some(2), 100).unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(res.next_entry_id, 100, "authoritative stored next_entry_id wins");
        // A stray higher-id blob must also bump next_entry_id past it.
        std::fs::write(blob_path(&dir, 250), b"x").unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(res.next_entry_id >= 251, "next id past stray disk blob");
        let _ = std::fs::remove_dir_all(&dir);
    }

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn below(&mut self, n: u64) -> u64 {
            self.next() % n.max(1)
        }
    }

    /// The centerpiece "it can't drift" test: apply random op sequences to a
    /// reference model AND the store; after every op, reopen from disk and
    /// assert the loaded state equals the model exactly.
    #[test]
    fn fuzz_disk_always_matches_model() {
        let dir = tmp_dir("fuzz");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut model: Vec<StoredEntry> = Vec::new();
        let mut current: Option<u64> = None;
        let mut next_id: u64 = 1;
        let mut rng = Rng(0x9e3779b97f4a7c15);

        for _ in 0..1500 {
            match rng.below(7) {
                0 | 1 => {
                    if model.len() < 20 {
                        let id = next_id;
                        next_id += 1;
                        model.push(entry(id, &format!("e{}", id), false, 1 + (id % 4) as u32));
                    }
                }
                2 => {
                    if model.len() < 20 {
                        let id = next_id;
                        next_id += 1;
                        model.push(marker(id, &format!("m{}", id)));
                    }
                }
                3 => {
                    if !model.is_empty() {
                        let idx = rng.below(model.len() as u64) as usize;
                        let removed = model.remove(idx).entry_id;
                        if current == Some(removed) {
                            current = None;
                        }
                    }
                }
                4 => {
                    if !model.is_empty() {
                        let idx = rng.below(model.len() as u64) as usize;
                        model[idx].name = format!("r{}", rng.next() % 1000);
                    }
                }
                5 => {
                    if !model.is_empty() {
                        let idx = rng.below(model.len() as u64) as usize;
                        model[idx].pinned = !model[idx].pinned;
                    }
                }
                _ => {
                    // move cursor to a random restorable entry (or None)
                    let cands: Vec<u64> = model
                        .iter()
                        .filter(|e| e.snapshot.is_some())
                        .map(|e| e.entry_id)
                        .collect();
                    current = if cands.is_empty() {
                        None
                    } else {
                        Some(cands[rng.below(cands.len() as u64) as usize])
                    };
                }
            }

            store.persist(&model, current, next_id).unwrap();

            let (_verify, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
            assert_eq!(res.entries.len(), model.len(), "len");
            for (got, want) in res.entries.iter().zip(model.iter()) {
                assert_eq!(got.entry_id, want.entry_id);
                assert_eq!(got.name, want.name, "name id={}", want.entry_id);
                assert_eq!(got.pinned, want.pinned, "pin id={}", want.entry_id);
                assert_eq!(got.kind, want.kind);
                assert_eq!(got.snapshot, want.snapshot, "snap id={}", want.entry_id);
                assert!(got.available);
            }
            assert_eq!(res.current_entry_id, current, "cursor");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Fault-injection: hammer the store with random blob deletion/corruption
    /// and manifest corruption between persists. `open_in` must NEVER panic or
    /// error, available entries must still round-trip, and a clean reopen after
    /// the faults must recover the full model (preserved blobs are reused).
    #[test]
    fn fault_injection_never_crashes_and_recovers() {
        let dir = tmp_dir("faults");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut rng = Rng(0x00C0FFEE);
        let mut next_id = 1u64;

        for _ in 0..200 {
            let n = 3 + rng.below(6) as u64;
            let ids: Vec<u64> = (0..n).map(|_| { let id = next_id; next_id += 1; id }).collect();
            let model: Vec<StoredEntry> =
                ids.iter().map(|&id| entry(id, &format!("e{}", id), false, 2)).collect();
            store.persist(&model, Some(ids[0]), next_id).unwrap();

            // Inject a fault.
            match rng.below(5) {
                0 => {
                    let _ = std::fs::remove_file(blob_path(&dir, ids[rng.below(n) as usize]));
                }
                1 => {
                    let _ = std::fs::write(blob_path(&dir, ids[rng.below(n) as usize]), b"corrupt");
                }
                2 => {
                    let _ = std::fs::write(dir.join("manifest.json"), b"{ not valid json");
                }
                3 => {
                    let _ = std::fs::write(dir.join("manifest.json.tmp"), b"junk");
                }
                _ => {}
            }

            // Must not panic/error, and ids must never regress below what we've
            // handed out (no id reuse even after corruption — blobs are kept).
            let (s2, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
            assert!(
                res.next_entry_id >= *ids.last().unwrap(),
                "next_entry_id {} regressed below last issued id {}",
                res.next_entry_id,
                ids.last().unwrap()
            );
            for e in &res.entries {
                if e.available {
                    assert_eq!(e.kind, HistoryEntryKind::Snapshot);
                }
            }
            store = s2;
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Crash after a blob write but before the manifest commit: the new blob is
    /// an orphan (manifest still references the old set) and must be GC'd, with
    /// the committed state intact. No half-written entry leaks in.
    #[test]
    fn crash_after_blob_before_manifest_is_clean() {
        let dir = tmp_dir("crashblob");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let committed = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        store.persist(&committed, Some(2), 3).unwrap();
        // Simulate "blob written, then crash before manifest publish".
        std::fs::write(blob_path(&dir, 3), b"halfwritten").unwrap();

        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 2, "only committed entries");
        assert_eq!(res.entries[0].entry_id, 1);
        assert_eq!(res.entries[1].entry_id, 2);
        assert!(!blob_path(&dir, 3).exists(), "orphan blob GC'd");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A leftover manifest.json.tmp (crash during a previous publish) must be
    /// ignored — the committed manifest.json still loads cleanly.
    #[test]
    fn stray_manifest_tmp_is_ignored() {
        let dir = tmp_dir("straytmp");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3)];
        store.persist(&entries, Some(1), 2).unwrap();
        std::fs::write(dir.join("manifest.json.tmp"), b"{ garbage").unwrap();
        assert_loads_as(&dir, &entries, Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cursor_resolves_when_current_unavailable() {
        let dir = tmp_dir("cursor");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3), entry(3, "C", false, 3)],
                Some(2),
                4,
            )
            .unwrap();
        // Current (id 2) becomes unavailable -> resolve to nearest previous (1).
        std::fs::remove_file(blob_path(&dir, 2)).unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(res.current_entry_id, Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_manifest_preserves_blobs() {
        let dir = tmp_dir("corruptman");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3), entry(2, "B", false, 3)], Some(2), 3)
            .unwrap();
        // Corrupt the manifest — must NOT trigger GC of the blobs.
        std::fs::write(dir.join("manifest.json"), b"{ not valid json").unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(blob_path(&dir, 1).exists(), "blob 1 preserved");
        assert!(blob_path(&dir, 2).exists(), "blob 2 preserved");
        assert!(res.entries.is_empty(), "unusable manifest -> no entries");
        assert!(res.warnings.iter().any(|w| w.contains("preserved")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unavailable_entry_not_demoted_to_marker_on_repersist() {
        let dir = tmp_dir("undemote");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3), entry(2, "B", false, 3)], Some(2), 3)
            .unwrap();
        std::fs::remove_file(blob_path(&dir, 1)).unwrap(); // blob 1 disappears

        let (mut store2, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(!res.entries[0].available, "1 unavailable on load");

        // App re-persists: entry 1 returns with snapshot None (bridge couldn't
        // load it), but it is STILL a Snapshot-kind entry.
        let e1_none = StoredEntry {
            snapshot: None,
            ..entry(1, "A", false, 3)
        };
        store2
            .persist(&[e1_none, entry(2, "B", false, 3)], Some(2), 3)
            .unwrap();

        // Reopen: 1 must remain a missing-blob snapshot (available == false),
        // NOT a marker (which would be available == true).
        let (_s3, res3) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(
            !res3.entries[0].available,
            "preserved as missing-blob entry, not demoted to a marker"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn migrate_legacy_backs_up_and_returns_history() {
        let root = tmp_dir("legacyroot");
        let session = root.join("2026-06-06-00-00-00");
        std::fs::create_dir_all(&session).unwrap();
        // Minimal legacy history.json (PersistedHistory JSON shape).
        let legacy = serde_json::json!({
            "version": 2, "saved_at": "x", "current_index": 0,
            "entries": [ {
                "label": "A", "timestamp": "00:00:00",
                "created_at_iso": "2026-06-06T00:00:00+00:00",
                "kind": "snapshot", "start_tick": 0, "end_tick": 3, "first_moving": null,
                "snapshot": { "recorded_count": 3, "input_log": [1,2,3],
                              "rec_coords": [[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]] }
            } ]
        });
        std::fs::write(session.join("history.json"), legacy.to_string()).unwrap();

        let v2 = tmp_dir("v2dir");
        let (backup, persisted) = migrate_legacy_in(&root, &v2)
            .unwrap()
            .expect("should migrate legacy data");
        assert!(backup.exists(), "legacy backup copied");
        assert_eq!(persisted.entries.len(), 1);
        assert_eq!(persisted.entries[0].label, "A");

        // Once a v2 manifest exists, migration must no-op (don't re-import).
        std::fs::write(v2.join("manifest.json"), b"{}").unwrap();
        assert!(migrate_legacy_in(&root, &v2).unwrap().is_none());

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&v2);
    }
}
