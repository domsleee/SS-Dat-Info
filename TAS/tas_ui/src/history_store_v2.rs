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
use tas_shared::TAS_MAX_TICKS;

const MAGIC: &str = "ssb-history";
const SCHEMA: u32 = 1;
/// Legacy blob encoding: `bincode::serialize(PersistedSnapshot)`. Unbounded on
/// read, so only decoded through a size-limited reader (kept for blobs already
/// written to a user's store before the raw format existed).
const BLOB_FORMAT_BINCODE_V1: u32 = 1;
/// Current blob encoding: a tiny self-describing raw layout, bounds-checked
/// before any allocation. See `serialize_raw_blob` / `parse_raw_blob`.
const BLOB_FORMAT_RAW_V2: u32 = 2;
const CURRENT_BLOB_FORMAT: u32 = BLOB_FORMAT_RAW_V2;
const HASH_ALGO: &str = "crc32";

/// Bytes per recorded tick in a raw blob: 1 input byte + 3 little-endian f32s.
const BLOB_BYTES_PER_TICK: usize = 1 + 3 * 4;
/// Hard upper bound on a snapshot blob: a full-length recording. Any file
/// larger than this is rejected by metadata BEFORE it is read into memory, so a
/// corrupt/forged blob can never OOM the loader.
const MAX_SNAPSHOT_BYTES: u64 = (4 + TAS_MAX_TICKS * BLOB_BYTES_PER_TICK) as u64;
/// Upper bound on `manifest.json`. Real manifests are pure metadata (tens of
/// bytes per entry); even 100k entries is a few tens of MB. Anything past this
/// is treated as unusable (and so does NOT trigger blob GC) rather than read
/// into memory — a corrupt/forged manifest can't OOM the loader.
const MAX_MANIFEST_BYTES: u64 = 256 * 1024 * 1024;

fn blob_format_v1() -> u32 {
    BLOB_FORMAT_BINCODE_V1
}

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
    /// Level code (e.g. "FE") the entry was created on; None = unknown/legacy.
    pub level: Option<String>,
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
    /// Level code (e.g. "FE"); None on manifests written before this field.
    pub level: Option<String>,
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
    /// Level code (e.g. "FE"). Absent on older manifests → None.
    #[serde(default)]
    level: Option<String>,
    created_at_iso: String,
    /// `None` for marker entries with no blob.
    size: Option<u64>,
    checksum: Option<u32>,
    /// Encoding of this entry's blob. Per-entry (not global) so a store can
    /// hold legacy bincode blobs and new raw blobs side by side — each one
    /// self-describing. Absent on manifests written before the raw format
    /// existed, where every blob was bincode → default to v1.
    #[serde(default = "blob_format_v1")]
    blob_format: u32,
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
    /// Encoding of the on-disk blob (so a reused/preserved legacy blob keeps
    /// its `blob_format` in the rewritten manifest rather than being mislabeled
    /// as the current raw format).
    format: u32,
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
                max_manifest_id = max_manifest_id.max(me.entry_id);
                // Only blob-bearing rows (checksum present) protect an
                // `<id>.tasrec` from orphan GC. A marker row (checksum None)
                // must NOT shield a stray blob at the same id — tracking
                // "manifest row ids" instead of "blob ids" leaked such blobs.
                if me.checksum.is_some() {
                    referenced.insert(me.entry_id);
                }

                let blob_info = BlobInfo {
                    size: me.size.unwrap_or(0),
                    checksum: me.checksum.unwrap_or(0),
                    format: me.blob_format,
                };
                let (snapshot, available) = match me.checksum {
                    None => (None, true), // marker entry, no blob expected
                    Some(expected) => {
                        match read_blob(&dir, me.entry_id, expected, me.size, me.blob_format) {
                            Ok(snap) => {
                                blobs.insert(me.entry_id, blob_info);
                                (Some(snap), true)
                            }
                            Err(BlobError::Missing) => {
                                warnings.push(format!(
                                    "history entry {} ('{}') blob missing — kept but unavailable",
                                    me.entry_id, me.name
                                ));
                                unavailable.insert(me.entry_id, blob_info);
                                (None, false)
                            }
                            Err(BlobError::Corrupt) => {
                                // Quarantine so we don't re-hit it every launch.
                                quarantine_blob(&dir, me.entry_id);
                                warnings.push(format!(
                                    "history entry {} ('{}') blob corrupt — quarantined, unavailable",
                                    me.entry_id, me.name
                                ));
                                unavailable.insert(me.entry_id, blob_info);
                                (None, false)
                            }
                        }
                    }
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
                    level: me.level.clone(),
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
            let (size, checksum, blob_format) = match &e.snapshot {
                None => {
                    // No bytes in hand. If this id is a snapshot entry whose
                    // blob is currently missing/corrupt, preserve its manifest
                    // blob reference (don't demote it to a marker) so it can
                    // recover if the file reappears — keeping its original
                    // encoding so a preserved legacy blob isn't mislabeled.
                    match self.unavailable.get(&e.entry_id) {
                        Some(info) => (Some(info.size), Some(info.checksum), info.format),
                        None => (None, None, CURRENT_BLOB_FORMAT),
                    }
                }
                Some(snap) => {
                    let info = if let Some(info) = self.blobs.get(&e.entry_id) {
                        *info
                    } else {
                        let info = self.write_blob(e.entry_id, snap)?;
                        self.blobs.insert(e.entry_id, info);
                        outcome.blobs_written += 1;
                        info
                    };
                    (Some(info.size), Some(info.checksum), info.format)
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
                level: e.level.clone(),
                created_at_iso: e.created_at_iso.clone(),
                size,
                checksum,
                blob_format,
            });
        }

        // 2. Publish the manifest atomically (this is the commit point).
        let manifest = Manifest {
            magic: MAGIC.to_string(),
            schema: SCHEMA,
            blob_format: CURRENT_BLOB_FORMAT,
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

        // Track only blob-bearing ids (those whose manifest row carries a
        // checksum), so the GC keep-set never includes marker ids. Mirrors the
        // load-time rule in `open_in`.
        self.referenced = manifest
            .entries
            .iter()
            .filter(|m| m.checksum.is_some())
            .map(|m| m.entry_id)
            .collect();
        Ok(outcome)
    }

    fn write_blob(&self, id: u64, snapshot: &PersistedSnapshot) -> Result<BlobInfo, String> {
        // New blobs use the bounded raw layout (never bincode). crc32 still
        // guards accidental corruption.
        let bytes = serialize_raw_blob(snapshot);
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
        Ok(BlobInfo {
            size,
            checksum,
            format: CURRENT_BLOB_FORMAT,
        })
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
    /// Drain barrier. The worker answers with the durability result of the most
    /// recent persist (Ok only if the manifest actually committed). Callers gate
    /// recovery-checkpoint clearing on this — never on mere enqueue.
    Flush(Sender<Result<(), String>>),
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
                // Durability of the LATEST persist, carried across loop
                // iterations so a flush that arrives in a later batch still
                // reports a prior failure. `Ok` only after a committed manifest.
                let mut last_result: Result<(), String> = Ok(());
                while let Ok(msg) = rx.recv() {
                    // Coalesce: keep only the newest desired state (each job is
                    // the full set; the store diffs it against disk), and answer
                    // every flush in the batch after writing.
                    let mut latest: Option<(Vec<StoredEntry>, Option<u64>, u64)> = None;
                    let mut acks: Vec<Sender<Result<(), String>>> = Vec::new();
                    let mut next = Some(msg);
                    while let Some(m) = next {
                        match m {
                            WriteMsg::Persist {
                                entries,
                                current_entry_id,
                                next_entry_id,
                            } => latest = Some((entries, current_entry_id, next_entry_id)),
                            WriteMsg::Flush(a) => acks.push(a),
                        }
                        next = rx.try_recv().ok();
                    }
                    if let Some((entries, current, next_id)) = latest {
                        last_result = match store.persist(&entries, current, next_id) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                eprintln!("[history v2] persist failed: {}", e);
                                Err(e)
                            }
                        };
                    }
                    for ack in acks {
                        let _ = ack.send(last_result.clone());
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

    /// Enqueue a persist. Returns `true` if the job reached the worker; `false`
    /// means the worker is gone (so the caller must NOT treat the revision as
    /// persisted and should retry later).
    pub fn persist(
        &self,
        entries: Vec<StoredEntry>,
        current_entry_id: Option<u64>,
        next_entry_id: u64,
    ) -> bool {
        match &self.tx {
            Some(tx) => tx
                .send(WriteMsg::Persist {
                    entries,
                    current_entry_id,
                    next_entry_id,
                })
                .is_ok(),
            None => false,
        }
    }

    /// Block until every queued write has hit disk, returning the durability
    /// result of the most recent persist. `Ok(())` means the manifest actually
    /// committed; `Err` means the latest persist failed (disk full / permission
    /// / rename) — the caller must keep any recovery checkpoint in that case.
    pub fn flush(&self) -> Result<(), String> {
        let Some(tx) = &self.tx else {
            return Ok(());
        };
        let (a, r) = channel();
        if tx.send(WriteMsg::Flush(a)).is_err() {
            return Err("history writer thread is gone".to_string());
        }
        r.recv()
            .unwrap_or_else(|_| Err("history writer dropped flush ack".to_string()))
    }
}

impl Drop for HistoryWriter {
    fn drop(&mut self) {
        let _ = self.flush();
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

fn read_blob(
    dir: &Path,
    id: u64,
    expected_checksum: u32,
    expected_size: Option<u64>,
    blob_format: u32,
) -> Result<PersistedSnapshot, BlobError> {
    let path = blob_path(dir, id);

    // Bound BEFORE reading: reject by file metadata so a giant/forged blob can
    // never be slurped into memory. A size that disagrees with the manifest is
    // treated as corruption (tamper or truncation).
    let meta = std::fs::metadata(&path).map_err(|_| BlobError::Missing)?;
    let len = meta.len();
    if len > MAX_SNAPSHOT_BYTES {
        return Err(BlobError::Corrupt);
    }
    if let Some(sz) = expected_size {
        if len != sz {
            return Err(BlobError::Corrupt);
        }
    }

    let bytes = std::fs::read(&path).map_err(|_| BlobError::Missing)?;
    if crc32fast::hash(&bytes) != expected_checksum {
        return Err(BlobError::Corrupt);
    }

    let snapshot = match blob_format {
        BLOB_FORMAT_RAW_V2 => parse_raw_blob(&bytes).ok_or(BlobError::Corrupt)?,
        BLOB_FORMAT_BINCODE_V1 => parse_bincode_blob(&bytes).ok_or(BlobError::Corrupt)?,
        _ => return Err(BlobError::Corrupt), // unknown encoding
    };
    // Reject semantically-invalid snapshots HERE (in the store) so the bridge's
    // `from_persisted(..).ok()` never has to drop one — which would silently
    // demote a present-but-bad blob to a marker on the next persist.
    snapshot.validate().map_err(|_| BlobError::Corrupt)?;
    Ok(snapshot)
}

/// Raw blob layout (`BLOB_FORMAT_RAW_V2`):
/// `u32le recorded_count | input_log[count] | (f32le x,y,z)[count]`.
fn serialize_raw_blob(snapshot: &PersistedSnapshot) -> Vec<u8> {
    let count = snapshot.recorded_count as usize;
    let mut out = Vec::with_capacity(4 + count * BLOB_BYTES_PER_TICK);
    out.extend_from_slice(&snapshot.recorded_count.to_le_bytes());
    out.extend_from_slice(&snapshot.input_log);
    for c in &snapshot.rec_coords {
        out.extend_from_slice(&c[0].to_le_bytes());
        out.extend_from_slice(&c[1].to_le_bytes());
        out.extend_from_slice(&c[2].to_le_bytes());
    }
    out
}

/// Parse a raw blob with every length/bound checked before allocation. Returns
/// `None` on any inconsistency (caller maps to corrupt).
fn parse_raw_blob(bytes: &[u8]) -> Option<PersistedSnapshot> {
    if bytes.len() < 4 {
        return None;
    }
    let count = u32::from_le_bytes(bytes[0..4].try_into().ok()?) as usize;
    if count > TAS_MAX_TICKS {
        return None;
    }
    if bytes.len() != 4 + count * BLOB_BYTES_PER_TICK {
        return None;
    }
    let input_end = 4 + count;
    let input_log = bytes[4..input_end].to_vec();
    let mut rec_coords = Vec::with_capacity(count);
    for chunk in bytes[input_end..].chunks_exact(12) {
        rec_coords.push([
            f32::from_le_bytes(chunk[0..4].try_into().ok()?),
            f32::from_le_bytes(chunk[4..8].try_into().ok()?),
            f32::from_le_bytes(chunk[8..12].try_into().ok()?),
        ]);
    }
    Some(PersistedSnapshot {
        recorded_count: count as u32,
        input_log,
        rec_coords,
    })
}

/// Decode a legacy bincode blob through a SIZE-LIMITED reader (matching the
/// fixint encoding `bincode::serialize` produced), so a forged length prefix
/// can't allocate-huge. Only used for blobs written before the raw format.
fn parse_bincode_blob(bytes: &[u8]) -> Option<PersistedSnapshot> {
    use bincode::Options;
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(MAX_SNAPSHOT_BYTES + 1024)
        .allow_trailing_bytes()
        .deserialize::<PersistedSnapshot>(bytes)
        .ok()
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
    // Bound by metadata before reading: an oversized manifest is treated as
    // unusable (no GC of blobs) rather than slurped into memory.
    if let Ok(meta) = std::fs::metadata(&path) {
        if meta.len() > MAX_MANIFEST_BYTES {
            warnings.push(format!(
                "manifest too large ({} bytes) — ignored, blobs preserved",
                meta.len()
            ));
            return None;
        }
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
    // blob_format is a coarse top-level gate; the actual encoding is per-entry
    // (`ManifestEntry::blob_format`), so accept any known top-level value and
    // reject only the genuinely unknown/newer (forward-incompatible) ones.
    if manifest.magic != MAGIC
        || manifest.schema != SCHEMA
        || manifest.blob_format == 0
        || manifest.blob_format > CURRENT_BLOB_FORMAT
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
    std::fs::write(&tmp, &json).map_err(|e| format!("failed to write manifest tmp: {}", e))?;
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
        std::env::temp_dir().join(format!(
            "ssb_hist_v2_{}_{}_{}",
            tag,
            std::process::id(),
            nonce
        ))
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
            level: None,
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
            level: None,
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
            assert_eq!(
                got.snapshot, want.snapshot,
                "snapshot for {}",
                want.entry_id
            );
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
        let entries = vec![
            entry(1, "A", false, 3),
            entry(2, "B", true, 5),
            marker(3, "saved"),
        ];
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
        assert_eq!(
            o,
            PersistOutcome {
                blobs_written: 0,
                blobs_deleted: 0
            }
        );
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
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
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
        store
            .persist(&[entry(1, "A", false, 3)], Some(1), 2)
            .unwrap();
        std::fs::write(blob_path(&dir, 1), b"garbage").unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(!res.entries[0].available);
        assert!(
            dir.join("1.tasrec.corrupt").exists(),
            "corrupt blob quarantined"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn gc_orphan_blob_on_open() {
        let dir = tmp_dir("gc");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3)], Some(1), 2)
            .unwrap();
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
        store
            .persist(&[entry(2, "A", false, 3)], Some(2), 100)
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(
            res.next_entry_id, 100,
            "authoritative stored next_entry_id wins"
        );
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
            let n = 3 + rng.below(6);
            let ids: Vec<u64> = (0..n)
                .map(|_| {
                    let id = next_id;
                    next_id += 1;
                    id
                })
                .collect();
            let model: Vec<StoredEntry> = ids
                .iter()
                .map(|&id| entry(id, &format!("e{}", id), false, 2))
                .collect();
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
                &[
                    entry(1, "A", false, 3),
                    entry(2, "B", false, 3),
                    entry(3, "C", false, 3),
                ],
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
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
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
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
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

    /// Build a manifest referencing a single blob written by the test, so we can
    /// forge legacy/invalid blobs the public `persist` path would never emit.
    fn write_manifest_with_blob(
        dir: &Path,
        id: u64,
        bytes: &[u8],
        blob_format: u32,
        size: Option<u64>,
    ) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(blob_path(dir, id), bytes).unwrap();
        let manifest = Manifest {
            magic: MAGIC.to_string(),
            schema: SCHEMA,
            blob_format,
            hash_algo: HASH_ALGO.to_string(),
            next_entry_id: id + 1,
            current_entry_id: Some(id),
            entries: vec![ManifestEntry {
                entry_id: id,
                name: "A".to_string(),
                user_name: None,
                pinned: false,
                kind: HistoryEntryKind::Snapshot,
                start_tick: 0,
                end_tick: 4,
                first_moving: None,
                level: None,
                created_at_iso: "2026-06-06T00:00:00+00:00".to_string(),
                size: size.or(Some(bytes.len() as u64)),
                checksum: Some(crc32fast::hash(bytes)),
                blob_format,
            }],
        };
        write_manifest_atomic(dir, &manifest).unwrap();
    }

    /// The migration guarantee: blobs written by the OLD bincode format (which
    /// real user stores already contain) must still load, and must NOT be
    /// rewritten on re-persist (immutable; their per-entry format is preserved).
    #[test]
    fn legacy_bincode_blob_loads_and_is_preserved() {
        let dir = tmp_dir("legacyblob");
        let ps = snap(4, 7);
        let bytes = bincode::serialize(&ps).unwrap();
        write_manifest_with_blob(&dir, 1, &bytes, BLOB_FORMAT_BINCODE_V1, None);

        let (mut store, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(res.entries[0].available, "legacy bincode blob loads");
        assert_eq!(res.entries[0].snapshot.as_ref().unwrap(), &ps);

        // Re-persist with the loaded snapshot in hand: the store must REUSE the
        // existing blob (immutable), not rewrite it as raw.
        let before = std::fs::read(blob_path(&dir, 1)).unwrap();
        let stored = StoredEntry {
            entry_id: 1,
            name: "A".to_string(),
            user_name: None,
            pinned: false,
            kind: HistoryEntryKind::Snapshot,
            start_tick: 0,
            end_tick: 4,
            first_moving: None,
            level: None,
            created_at_iso: "2026-06-06T00:00:00+00:00".to_string(),
            snapshot: res.entries[0].snapshot.clone(),
        };
        let o = store.persist(&[stored], Some(1), 2).unwrap();
        assert_eq!(o.blobs_written, 0, "legacy blob reused, not rewritten");
        assert_eq!(
            std::fs::read(blob_path(&dir, 1)).unwrap(),
            before,
            "blob bytes unchanged"
        );

        // And it still loads after the manifest was rewritten (format preserved).
        let (_s, res2) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert_eq!(res2.entries[0].snapshot.as_ref().unwrap(), &ps);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A blob whose on-disk length disagrees with the manifest (tamper /
    /// truncation / a forged-huge file) is rejected as corrupt by metadata —
    /// it is never slurped into memory, so it cannot OOM the loader.
    #[test]
    fn blob_size_mismatch_rejected_without_reading() {
        let dir = tmp_dir("sizemismatch");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 4)], Some(1), 2)
            .unwrap();
        // Grow the blob so its length no longer matches the manifest `size`.
        let mut bytes = std::fs::read(blob_path(&dir, 1)).unwrap();
        bytes.extend_from_slice(&[0u8; 32]);
        std::fs::write(blob_path(&dir, 1), &bytes).unwrap();

        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(
            !res.entries[0].available,
            "size-mismatched blob is unavailable"
        );
        assert!(dir.join("1.tasrec.corrupt").exists(), "quarantined");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// #8: a marker row (no blob) must NOT shield a stray `<id>.tasrec` from
    /// orphan GC just because it shares the id.
    #[test]
    fn marker_row_does_not_protect_stray_blob() {
        let dir = tmp_dir("markerstray");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store.persist(&[marker(1, "saved")], None, 2).unwrap();
        std::fs::write(blob_path(&dir, 1), b"stray").unwrap();
        let (_s, _res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(
            !blob_path(&dir, 1).exists(),
            "stray blob at a marker id must be GC'd, not protected"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// #5: a blob that DESERIALIZES but is semantically bogus (bincode succeeds,
    /// but recorded_count disagrees with the vec lengths) must be rejected at
    /// load as corrupt and kept unavailable — never silently demoted to a
    /// marker on the next persist.
    #[test]
    fn invalid_bincode_blob_unavailable_not_demoted() {
        let dir = tmp_dir("invalidbincode");
        // recorded_count claims 5, but the buffers are empty → validate() fails.
        let bad = PersistedSnapshot {
            recorded_count: 5,
            input_log: vec![],
            rec_coords: vec![],
        };
        let bytes = bincode::serialize(&bad).unwrap();
        write_manifest_with_blob(&dir, 1, &bytes, BLOB_FORMAT_BINCODE_V1, None);

        let (mut store, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(
            !res.entries[0].available,
            "invalid snapshot rejected, not loaded"
        );

        // Bridge re-persists it with snapshot None (it couldn't build one): the
        // entry must stay a missing-blob Snapshot, NOT become an available marker.
        let none_entry = StoredEntry {
            snapshot: None,
            ..entry(1, "A", false, 5)
        };
        store.persist(&[none_entry], Some(1), 2).unwrap();
        let (_s, res2) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        assert!(
            !res2.entries[0].available,
            "preserved as unavailable snapshot, not demoted to a marker"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// New blobs are the raw format (format 2), and round-trip bit-exactly
    /// (f32 via to/from_le_bytes), so existing round-trip tests now exercise it.
    #[test]
    fn new_blobs_use_raw_format() {
        let dir = tmp_dir("rawfmt");
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let e = entry(1, "A", false, 5);
        store.persist(std::slice::from_ref(&e), Some(1), 2).unwrap();
        // Raw layout length is exactly 4 + count*(1 + 12).
        let len = std::fs::metadata(blob_path(&dir, 1)).unwrap().len();
        assert_eq!(len, (4 + 5 * BLOB_BYTES_PER_TICK) as u64);
        assert_loads_as(&dir, &[e], Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
