//! File-per-entry persistent history store (v2). See TAS/README.md.
//!
//! Layout: `<dir>/<entry_id>.tasrec` (one immutable blob per snapshot entry)
//! plus `<dir>/manifest.json` (ordered metadata + cursor + next_entry_id).
//!
//! Identity is the stable `entry_id` (NOT a positional index). Blobs are
//! id-named (no content hashing): snapshots are ~always unique, so dedup buys
//! nothing and hashing would force a canonical blob format + refcount GC.
//! Integrity is a per-entry crc32 checksum stored in the manifest.

use crate::recording::{HistoryEntryKind, PersistedSnapshot};
use crate::worker::CoalescingWriter;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tas_shared::TAS_MAX_TICKS;

const MAGIC: &str = "ssb-history";
const SCHEMA: u32 = 1;
/// Blob encoding: a tiny self-describing raw layout, bounds-checked before
/// any allocation. See `serialize_raw_blob` / `parse_raw_blob`.
const CURRENT_BLOB_FORMAT: u32 = 2;
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

/// Everything the store knows about an entry besides its recording. The same
/// shape travels in (`StoredEntry`), out (`LoadedEntry`) and through the
/// manifest on disk.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct EntryMeta {
    pub entry_id: u64,
    pub name: String,
    /// User-given name (rename); `None` = use the auto `name`/duration.
    #[serde(default)]
    pub user_name: Option<String>,
    pub pinned: bool,
    pub kind: HistoryEntryKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub first_moving: Option<u32>,
    /// Race time (centiseconds) of a session that ended at the finish line.
    #[serde(default)]
    pub finish_time_cs: Option<u32>,
    /// Whether that time is the HUD timer's (exact) or geometry-derived.
    #[serde(default)]
    pub finish_time_exact: bool,
    /// Level code (e.g. "FE") the entry was created on; None = unknown.
    #[serde(default)]
    pub level: Option<String>,
    /// Physics-mode stamp (`tas_shared::physics_mode_label`); None = unknown.
    #[serde(default)]
    pub physics: Option<String>,
    /// Rider stamp (`tas_shared::rider_label`); None = unknown.
    #[serde(default)]
    pub rider: Option<String>,
    /// Raw identity words behind the stamps, for exact save round trips.
    /// `serde(default)` so pre-stamp manifests still load (as unknown).
    #[serde(default)]
    pub stamps: crate::recording::IdentityStamps,
    pub created_at_iso: String,
}

/// One entry the caller wants persisted. `snapshot == None` => a marker entry
/// (e.g. SaveMarker) that carries no blob, or a snapshot entry whose blob the
/// store already holds.
#[derive(Clone, PartialEq, Debug)]
pub struct StoredEntry {
    pub meta: EntryMeta,
    pub snapshot: Option<PersistedSnapshot>,
}

/// One entry as loaded from disk. `available == false` means a snapshot entry
/// whose blob was missing/corrupt — kept visible but inert (cannot restore).
#[derive(Clone, PartialEq, Debug)]
pub struct LoadedEntry {
    pub meta: EntryMeta,
    /// Inline bytes: `Some` for available snapshot entries under an eager
    /// open, `None` under the lazy production open (see `blob`).
    pub snapshot: Option<PersistedSnapshot>,
    /// The on-disk blob backing this entry (`None` for markers). Under lazy
    /// loading this is what a later `load_blob` needs.
    pub blob: Option<BlobRef>,
    pub available: bool,
}

#[derive(Debug, Default)]
pub struct LoadResult {
    pub entries: Vec<LoadedEntry>,
    pub current_entry_id: Option<u64>,
    pub next_entry_id: u64,
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ManifestEntry {
    #[serde(flatten)]
    meta: EntryMeta,
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

/// Identity of an immutable on-disk blob (`<id>.tasrec`): enough to read and
/// verify it later (`load_blob`) without holding its contents in memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlobRef {
    pub size: u64,
    pub checksum: u32,
}

pub struct HistoryStoreV2 {
    dir: PathBuf,
    /// Entry ids currently referenced by the on-disk manifest.
    referenced: HashSet<u64>,
    /// Blob info for ids that currently have a blob on disk (lets us skip
    /// re-serializing immutable blobs and carry checksums into the manifest).
    blobs: HashMap<u64, BlobRef>,
    /// Ids whose manifest entry references a blob that was missing/corrupt at
    /// load. We preserve their blob metadata across re-persists so a transient
    /// disappearance doesn't permanently demote a snapshot entry to a marker
    /// (and so the entry recovers if the file reappears).
    unavailable: HashMap<u64, BlobRef>,
}

impl HistoryStoreV2 {
    /// Open without reading blob contents: entries come back with
    /// `snapshot: None` and `blob: Some(..)`, and a blob is read when it is
    /// actually restored (`load_blob`).
    pub fn open_lazy(dir: PathBuf) -> Result<(Self, LoadResult), String> {
        Self::open_in_with(dir, false)
    }

    /// Open reading, verifying and returning every blob.
    #[cfg(test)]
    pub fn open_eager(dir: PathBuf) -> Result<(Self, LoadResult), String> {
        Self::open_in_with(dir, true)
    }

    fn open_in_with(dir: PathBuf, eager: bool) -> Result<(Self, LoadResult), String> {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("failed to create history dir {}: {}", dir.display(), e))?;

        let mut warnings = Vec::new();
        let disk_ids = list_blob_ids(&dir);
        let manifest = read_manifest(&dir, &mut warnings);

        let mut entries = Vec::new();
        let mut referenced = HashSet::new();
        let mut blobs = HashMap::new();
        let mut unavailable: HashMap<u64, BlobRef> = HashMap::new();
        let mut max_manifest_id = 0u64;
        let mut stored_next = 0u64;
        let mut persisted_current = None;

        if let Some(m) = &manifest {
            stored_next = m.next_entry_id;
            persisted_current = m.current_entry_id;
            for me in &m.entries {
                let id = me.meta.entry_id;
                max_manifest_id = max_manifest_id.max(id);
                // Only blob-bearing rows (checksum present) protect an
                // `<id>.tasrec` from orphan GC. A marker row (checksum None)
                // must NOT shield a stray blob at the same id.
                if me.checksum.is_some() {
                    referenced.insert(id);
                }

                let blob_info = BlobRef {
                    size: me.size.unwrap_or(0),
                    checksum: me.checksum.unwrap_or(0),
                };
                let (snapshot, available, blob) = match me.checksum {
                    None => (None, true, None), // marker entry, no blob expected
                    Some(expected) => {
                        let read = if eager {
                            read_blob(&dir, id, expected, me.size).map(Some)
                        } else {
                            // Existence + size only. Checksum and parse happen
                            // on first restore (`load_blob`), which quarantines
                            // a corrupt blob exactly as this path would.
                            stat_blob(&dir, id, me.size).map(|()| None)
                        };
                        match read {
                            Ok(snap) => {
                                blobs.insert(id, blob_info);
                                (snap, true, Some(blob_info))
                            }
                            Err(BlobError::Missing) => {
                                warnings.push(format!(
                                    "history entry {} ('{}') blob missing — kept but unavailable",
                                    id, me.meta.name
                                ));
                                unavailable.insert(id, blob_info);
                                (None, false, Some(blob_info))
                            }
                            Err(BlobError::Corrupt) => {
                                // Quarantine so we don't re-hit it every launch.
                                quarantine_blob(&dir, id);
                                warnings.push(format!(
                                    "history entry {} ('{}') blob corrupt — quarantined, unavailable",
                                    id, me.meta.name
                                ));
                                unavailable.insert(id, blob_info);
                                (None, false, Some(blob_info))
                            }
                        }
                    }
                };

                entries.push(LoadedEntry {
                    meta: me.meta.clone(),
                    snapshot,
                    blob,
                    available,
                });
            }
        }

        // Corrupt-store quarantine: a present-but-unusable manifest with
        // surviving blobs is a data-loss trap. The next persist would publish
        // a valid EMPTY manifest, and the launch after that would GC every
        // surviving recording as an orphan — with no new recording or explicit
        // deletion in between. Move the orphans out of GC reach BEFORE
        // anything can publish, and set the bad manifest aside for forensics.
        // (A missing manifest is a fresh store: strays there are still GC'd,
        // tested by crash_after_blob_before_manifest_is_clean.)
        let manifest_present = dir.join("manifest.json").exists();
        if manifest.is_none() && manifest_present && !disk_ids.is_empty() {
            quarantine_unmanifested_blobs(&dir, &disk_ids, &mut warnings);
            set_aside_unusable_manifest(&dir, &mut warnings);
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
        } else if !disk_ids.is_empty() && !manifest_present {
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
    ) -> Result<(), String> {
        let desired_ids: HashSet<u64> = entries.iter().map(|e| e.meta.entry_id).collect();

        // 1. Write blobs for new snapshot-bearing ids (immutable, write-once).
        let mut manifest_entries = Vec::with_capacity(entries.len());
        for e in entries {
            let id = e.meta.entry_id;
            let info = match &e.snapshot {
                None => {
                    // No bytes in hand. Two legitimate reasons: the entry was
                    // loaded lazily (its immutable blob is on disk and this
                    // store knows it), or its blob is missing/corrupt. Either
                    // way keep the manifest's blob reference — demoting to a
                    // marker would GC a perfectly good blob in the lazy case,
                    // and would stop a missing one from recovering if the file
                    // reappears. Ids are never reused, so a known id without
                    // bytes can only mean one of these.
                    self.blobs
                        .get(&id)
                        .or_else(|| self.unavailable.get(&id))
                        .copied()
                }
                Some(snap) => Some(match self.blobs.get(&id) {
                    Some(info) => *info,
                    None => {
                        let info = self.write_blob(id, snap)?;
                        self.blobs.insert(id, info);
                        info
                    }
                }),
            };
            manifest_entries.push(ManifestEntry {
                meta: e.meta.clone(),
                size: info.map(|i| i.size),
                checksum: info.map(|i| i.checksum),
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
                let _ = std::fs::remove_file(blob_path(&self.dir, id));
            }
        }

        // Track only blob-bearing ids (those whose manifest row carries a
        // checksum), so the GC keep-set never includes marker ids. Mirrors the
        // load-time rule in `open_in_with`.
        self.referenced = manifest
            .entries
            .iter()
            .filter(|m| m.checksum.is_some())
            .map(|m| m.meta.entry_id)
            .collect();
        Ok(())
    }

    fn write_blob(&self, id: u64, snapshot: &PersistedSnapshot) -> Result<BlobRef, String> {
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
        Ok(BlobRef { size, checksum })
    }

    /// Reference for a blob this store currently holds on disk (loaded or
    /// written by it); `None` for markers, unavailable ids, and unknown ids.
    pub fn blob_ref(&self, id: u64) -> Option<BlobRef> {
        self.blobs.get(&id).copied()
    }
}

/// Default v2 store directory: `<data root>/history/`.
pub fn default_history_dir() -> PathBuf {
    crate::settings::data_root_dir().join("history")
}

struct PersistJob {
    entries: Vec<StoredEntry>,
    current_entry_id: Option<u64>,
    next_entry_id: u64,
    revision: u64,
}

/// Background writer: owns the store on a worker thread so the serialize +
/// disk writes never touch the UI thread. The UI thread only clones
/// `to_stored_entries()` and hands it off; bursts coalesce to the latest.
pub struct HistoryWriter {
    inner: CoalescingWriter<PersistJob>,
    durable_revision: Arc<AtomicU64>,
    failed_revision: Arc<AtomicU64>,
    /// Blob references for inline snapshots the worker has committed and the
    /// UI has not yet collected (`take_durable_blobs`).
    durable_blobs: Arc<Mutex<Vec<(u64, BlobRef)>>>,
}

impl HistoryWriter {
    /// Open (loading existing data lazily) and spawn the writer thread.
    pub fn open(dir: PathBuf) -> Result<(Self, LoadResult), String> {
        let (mut store, load) = HistoryStoreV2::open_lazy(dir)?;
        let durable_revision = Arc::new(AtomicU64::new(0));
        let failed_revision = Arc::new(AtomicU64::new(0));
        let durable_blobs: Arc<Mutex<Vec<(u64, BlobRef)>>> = Arc::default();
        let worker_durable_revision = Arc::clone(&durable_revision);
        let worker_failed_revision = Arc::clone(&failed_revision);
        let worker_durable_blobs = Arc::clone(&durable_blobs);
        let inner = CoalescingWriter::spawn("history", move |job: PersistJob| {
            match store.persist(&job.entries, job.current_entry_id, job.next_entry_id) {
                Ok(()) => {
                    // Every inline snapshot in this job is now an immutable
                    // blob on disk: tell the UI so it can drop its resident
                    // copy (see RecordingHistory::mark_durable). Published
                    // BEFORE the revision so a reader that sees the revision
                    // can collect them.
                    let refs: Vec<(u64, BlobRef)> = job
                        .entries
                        .iter()
                        .filter(|e| e.snapshot.is_some())
                        .filter_map(|e| {
                            store
                                .blob_ref(e.meta.entry_id)
                                .map(|b| (e.meta.entry_id, b))
                        })
                        .collect();
                    if !refs.is_empty() {
                        if let Ok(mut pending) = worker_durable_blobs.lock() {
                            pending.extend(refs);
                        }
                    }
                    worker_durable_revision.store(job.revision, Ordering::Release);
                    worker_failed_revision.store(0, Ordering::Release);
                    Ok(())
                }
                Err(e) => {
                    worker_failed_revision.store(job.revision, Ordering::Release);
                    Err(e)
                }
            }
        })?;
        Ok((
            Self {
                inner,
                durable_revision,
                failed_revision,
                durable_blobs,
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
        revision: u64,
    ) -> bool {
        // Clear the previous failure before retrying the same revision, so
        // callers can distinguish a new failed attempt.
        self.failed_revision.store(0, Ordering::Release);
        self.inner.submit(PersistJob {
            entries,
            current_entry_id,
            next_entry_id,
            revision,
        })
    }

    pub fn durable_revision(&self) -> u64 {
        self.durable_revision.load(Ordering::Acquire)
    }

    pub fn failed_revision(&self) -> u64 {
        self.failed_revision.load(Ordering::Acquire)
    }

    /// Blob references for snapshots the worker has committed since the last
    /// call (in persist order). The UI hands them to
    /// `RecordingHistory::mark_durable`.
    pub fn take_durable_blobs(&self) -> Vec<(u64, BlobRef)> {
        self.durable_blobs
            .lock()
            .map(|mut pending| std::mem::take(&mut *pending))
            .unwrap_or_default()
    }

    /// Block until every queued write has hit disk, returning the durability
    /// result of the most recent persist. `Ok(())` means the manifest actually
    /// committed; `Err` means the latest persist failed (disk full / permission
    /// / rename) — the caller must keep any recovery checkpoint in that case.
    pub fn flush(&self) -> Result<(), String> {
        self.inner.flush()
    }

    /// Persist failures since the last call, for the UI log.
    pub fn take_errors(&self) -> Vec<String> {
        self.inner.take_errors()
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
) -> Result<PersistedSnapshot, BlobError> {
    let path = blob_path(dir, id);

    // Bound BEFORE reading: reject by file metadata so a giant/forged blob can
    // never be slurped into memory. A size that disagrees with the manifest is
    // treated as corruption (tamper or truncation).
    stat_blob(dir, id, expected_size)?;

    let bytes = std::fs::read(&path).map_err(|_| BlobError::Missing)?;
    if crc32fast::hash(&bytes) != expected_checksum {
        return Err(BlobError::Corrupt);
    }

    let snapshot = parse_raw_blob(&bytes).ok_or(BlobError::Corrupt)?;
    // Reject semantically-invalid snapshots HERE (in the store) so the bridge's
    // `from_persisted(..).ok()` never has to drop one — which would silently
    // demote a present-but-bad blob to a marker on the next persist.
    snapshot.validate().map_err(|_| BlobError::Corrupt)?;
    Ok(snapshot)
}

/// Existence + size check; the checksum is deferred to `load_blob` under a
/// lazy open.
fn stat_blob(dir: &Path, id: u64, expected_size: Option<u64>) -> Result<(), BlobError> {
    let meta = std::fs::metadata(blob_path(dir, id)).map_err(|_| BlobError::Missing)?;
    let len = meta.len();
    if len > MAX_SNAPSHOT_BYTES {
        return Err(BlobError::Corrupt);
    }
    if let Some(sz) = expected_size {
        if len != sz {
            return Err(BlobError::Corrupt);
        }
    }
    Ok(())
}

/// Read and verify one blob on demand (the lazy counterpart of an eager
/// open's read). A corrupt blob is quarantined here, exactly as at eager load.
pub fn load_blob(dir: &Path, id: u64, blob: BlobRef) -> Result<PersistedSnapshot, String> {
    match read_blob(dir, id, blob.checksum, Some(blob.size)) {
        Ok(snapshot) => Ok(snapshot),
        Err(BlobError::Missing) => Err(format!("blob {} missing", blob_path(dir, id).display())),
        Err(BlobError::Corrupt) => {
            quarantine_blob(dir, id);
            Err(format!(
                "blob {} corrupt (quarantined)",
                blob_path(dir, id).display()
            ))
        }
    }
}

/// Raw blob layout: `u32le recorded_count | input_log[count] | (f32le x,y,z)[count]`.
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
    let (chunks, _) = bytes[input_end..].as_chunks::<12>();
    for chunk in chunks {
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

/// Move `<id>.tasrec` blobs with no usable manifest out of GC reach: renamed
/// to `<id>.tasrec.orphaned`, which neither `list_blob_ids` nor the orphan GC
/// touches. The bytes stay on disk for manual re-import; the store never
/// deletes what it cannot account for.
fn quarantine_unmanifested_blobs(dir: &Path, ids: &[u64], warnings: &mut Vec<String>) {
    let mut kept = 0u64;
    for &id in ids {
        let from = blob_path(dir, id);
        let to = dir.join(format!("{}.tasrec.orphaned", id));
        match std::fs::rename(&from, &to) {
            Ok(()) => kept += 1,
            Err(e) => warnings.push(format!(
                "cannot preserve orphan blob {}: {}",
                from.display(),
                e
            )),
        }
    }
    if kept > 0 {
        warnings.push(format!(
            "unusable manifest — {} recording blob(s) preserved as *.tasrec.orphaned (not GC'd); re-import manually",
            kept
        ));
    }
}

/// Rename a present-but-unusable `manifest.json` aside so the next persist
/// starts clean instead of re-triggering quarantine on an empty store.
/// Best-effort: if the rename fails the corrupt file stays and the next open
/// simply quarantines nothing (no blobs left) and warns again.
fn set_aside_unusable_manifest(dir: &Path, warnings: &mut Vec<String>) {
    if std::fs::rename(dir.join("manifest.json"), dir.join("manifest.corrupt.json")).is_err() {
        warnings.push("unusable manifest could not be set aside".to_string());
    }
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
    if manifest.magic != MAGIC
        || manifest.schema != SCHEMA
        || manifest.blob_format != CURRENT_BLOB_FORMAT
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
    let pos = entries.iter().position(|e| e.meta.entry_id == target)?;
    // Restorable = an available snapshot entry, whether its bytes were read
    // (eager) or only referenced (lazy). Markers have neither.
    let restorable = |e: &LoadedEntry| e.available && (e.snapshot.is_some() || e.blob.is_some());
    if restorable(&entries[pos]) {
        return Some(target);
    }
    // search backward, then forward, for the nearest restorable entry
    for i in (0..pos).rev() {
        if restorable(&entries[i]) {
            warnings.push(format!(
                "current entry {} unavailable — resolved to {}",
                target, entries[i].meta.entry_id
            ));
            return Some(entries[i].meta.entry_id);
        }
    }
    for e in entries.iter().skip(pos + 1) {
        if restorable(e) {
            warnings.push(format!(
                "current entry {} unavailable — resolved to {}",
                target, e.meta.entry_id
            ));
            return Some(e.meta.entry_id);
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

    fn meta(id: u64, name: &str, pinned: bool, kind: HistoryEntryKind, count: u32) -> EntryMeta {
        EntryMeta {
            entry_id: id,
            name: name.to_string(),
            user_name: None,
            pinned,
            kind,
            start_tick: 0,
            end_tick: count,
            first_moving: None,
            finish_time_cs: None,
            finish_time_exact: false,
            level: None,
            physics: None,
            rider: None,
            stamps: crate::recording::IdentityStamps::default(),
            created_at_iso: "2026-06-06T00:00:00+00:00".to_string(),
        }
    }

    fn entry(id: u64, name: &str, pinned: bool, count: u32) -> StoredEntry {
        StoredEntry {
            meta: meta(id, name, pinned, HistoryEntryKind::Snapshot, count),
            snapshot: Some(snap(count, id as u8)),
        }
    }

    fn marker(id: u64, name: &str) -> StoredEntry {
        StoredEntry {
            meta: meta(id, name, false, HistoryEntryKind::SaveMarker, 0),
            snapshot: None,
        }
    }

    fn mtime(path: &Path) -> std::time::SystemTime {
        std::fs::metadata(path).unwrap().modified().unwrap()
    }

    #[test]
    fn history_writer_advances_revision_only_after_durable_commit() {
        let dir = tmp_dir("writer_revision");
        let (writer, _) = HistoryWriter::open(dir.clone()).unwrap();
        assert!(writer.persist(vec![entry(1, "A", false, 3)], Some(1), 2, 42));
        writer.flush().unwrap();
        assert_eq!(writer.durable_revision(), 42);
        assert_eq!(writer.failed_revision(), 0);
        assert!(writer.take_errors().is_empty());
        drop(writer);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn history_writer_reports_failed_revision_without_marking_it_durable() {
        let dir = tmp_dir("writer_failed_revision");
        let (writer, _) = HistoryWriter::open(dir.clone()).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(writer.persist(vec![entry(1, "A", false, 3)], Some(1), 2, 7));
        assert!(writer.flush().is_err());
        assert_eq!(writer.durable_revision(), 0);
        assert_eq!(writer.failed_revision(), 7);
        assert_eq!(
            writer.take_errors().len(),
            1,
            "the failure reaches the UI log"
        );
    }

    #[test]
    fn lazy_open_defers_blobs_and_persist_keeps_them() {
        let dir = tmp_dir("lazy");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let entries = vec![
            entry(1, "A", false, 3),
            entry(2, "B", true, 5),
            marker(3, "saved"),
        ];
        store.persist(&entries, Some(2), 4).unwrap();
        drop(store);

        let (mut lazy, res) = HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 3);
        assert!(res.entries[0].snapshot.is_none() && res.entries[0].available);
        let blob1 = res.entries[0].blob.expect("lazy row carries its blob ref");
        assert!(res.entries[2].blob.is_none(), "marker has no blob");
        assert_eq!(lazy.blob_ref(1), Some(blob1));
        assert_eq!(res.current_entry_id, Some(2));

        // The bridge re-persists lazily-loaded rows WITHOUT bytes: the blob
        // reference and file must survive (not demoted to a marker + GC'd).
        let before = std::fs::read(blob_path(&dir, 1)).unwrap();
        let again = vec![
            StoredEntry {
                snapshot: None,
                ..entry(1, "A", false, 3)
            },
            StoredEntry {
                snapshot: None,
                ..entry(2, "B", true, 5)
            },
            marker(3, "saved"),
        ];
        lazy.persist(&again, Some(2), 4).unwrap();
        assert_eq!(std::fs::read(blob_path(&dir, 1)).unwrap(), before);
        assert!(blob_path(&dir, 2).exists());

        // On-demand read returns the original bytes; an eager reopen still
        // sees every snapshot.
        assert_eq!(
            load_blob(&dir, 1, blob1).unwrap(),
            entries[0].snapshot.clone().unwrap()
        );
        assert_loads_as(&dir, &entries, Some(2));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_blob_quarantines_corrupt_blob() {
        let dir = tmp_dir("lazy_corrupt");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 4)], Some(1), 2)
            .unwrap();
        drop(store);
        let (_lazy, res) = HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        assert!(res.entries[0].available, "lazy open only stats the file");
        let blob = res.entries[0].blob.unwrap();
        // Same size, different bytes: passes the lazy stat, fails the checksum.
        let p = blob_path(&dir, 1);
        let mut bytes = std::fs::read(&p).unwrap();
        bytes[4] ^= 0xFF;
        std::fs::write(&p, &bytes).unwrap();
        assert!(load_blob(&dir, 1, blob).is_err());
        assert!(!p.exists(), "corrupt blob quarantined on demand");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Assert the loaded store matches the desired entries + cursor exactly.
    fn assert_loads_as(dir: &Path, desired: &[StoredEntry], cursor: Option<u64>) {
        let (_store, res) = HistoryStoreV2::open_eager(dir.to_path_buf()).unwrap();
        assert_eq!(res.entries.len(), desired.len(), "entry count");
        for (got, want) in res.entries.iter().zip(desired.iter()) {
            assert_eq!(got.meta, want.meta);
            assert_eq!(
                got.snapshot, want.snapshot,
                "snapshot for {}",
                want.meta.entry_id
            );
            assert!(got.available);
        }
        assert_eq!(res.current_entry_id, cursor);
    }

    #[test]
    fn roundtrip_empty() {
        let dir = tmp_dir("empty");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store.persist(&[], None, 0).unwrap();
        assert_loads_as(&dir, &[], None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn roundtrip_entries_and_marker() {
        let dir = tmp_dir("rt");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let entries = vec![
            entry(1, "A", false, 3),
            entry(2, "B", true, 5),
            marker(3, "saved"),
        ];
        store.persist(&entries, Some(2), 4).unwrap();
        assert_loads_as(&dir, &entries, Some(2));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Blobs are immutable: an append writes only the new blob, and a
    /// re-persist of the same state touches none.
    #[test]
    fn append_writes_only_the_new_blob() {
        let dir = tmp_dir("append");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut entries = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        store.persist(&entries, Some(2), 3).unwrap();
        let (t1, t2) = (mtime(&blob_path(&dir, 1)), mtime(&blob_path(&dir, 2)));
        std::thread::sleep(std::time::Duration::from_millis(20));
        entries.push(entry(3, "C", false, 3));
        store.persist(&entries, Some(3), 4).unwrap();
        assert!(blob_path(&dir, 3).exists());
        store.persist(&entries, Some(3), 4).unwrap();
        assert_eq!(mtime(&blob_path(&dir, 1)), t1);
        assert_eq!(mtime(&blob_path(&dir, 2)), t2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn removed_entry_deletes_blob() {
        let dir = tmp_dir("rm");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        store.persist(&entries, Some(2), 3).unwrap();
        let trimmed = vec![entry(2, "B", false, 3)];
        store.persist(&trimmed, Some(2), 3).unwrap();
        assert!(!blob_path(&dir, 1).exists());
        assert!(blob_path(&dir, 2).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn marker_has_no_blob() {
        let dir = tmp_dir("marker");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store.persist(&[marker(1, "saved")], None, 2).unwrap();
        assert!(!blob_path(&dir, 1).exists());
        assert_loads_as(&dir, &[marker(1, "saved")], None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_blob_marks_unavailable_no_crash() {
        let dir = tmp_dir("missing");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
            .unwrap();
        // Yank a blob from under the store.
        std::fs::remove_file(blob_path(&dir, 1)).unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 2, "bad entry still listed");
        assert!(!res.entries[0].available);
        assert!(res.entries[1].available);
        assert!(!res.warnings.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_blob_quarantined() {
        let dir = tmp_dir("corrupt");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3)], Some(1), 2)
            .unwrap();
        std::fs::write(blob_path(&dir, 1), b"garbage").unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
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
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 3)], Some(1), 2)
            .unwrap();
        // Stray blob not referenced by the manifest.
        std::fs::write(blob_path(&dir, 99), b"orphan").unwrap();
        let (_s, _res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(!blob_path(&dir, 99).exists(), "orphan GC'd");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn next_entry_id_never_reuses_and_respects_disk() {
        let dir = tmp_dir("nextid");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        // next_entry_id stored as 100 even though max id is 2.
        store
            .persist(&[entry(2, "A", false, 3)], Some(2), 100)
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert_eq!(
            res.next_entry_id, 100,
            "authoritative stored next_entry_id wins"
        );
        // A stray higher-id blob must also bump next_entry_id past it.
        std::fs::write(blob_path(&dir, 250), b"x").unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
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

    /// Apply random op sequences to a reference model AND the store; after
    /// every op, reopen from disk and assert the loaded state equals the model
    /// exactly.
    #[test]
    fn fuzz_disk_always_matches_model() {
        let dir = tmp_dir("fuzz");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut model: Vec<StoredEntry> = Vec::new();
        let mut current: Option<u64> = None;
        let mut next_id: u64 = 1;
        let mut rng = Rng(0x9e3779b97f4a7c15);

        for _ in 0..200 {
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
                        let removed = model.remove(idx).meta.entry_id;
                        if current == Some(removed) {
                            current = None;
                        }
                    }
                }
                4 => {
                    if !model.is_empty() {
                        let idx = rng.below(model.len() as u64) as usize;
                        model[idx].meta.name = format!("r{}", rng.next() % 1000);
                    }
                }
                5 => {
                    if !model.is_empty() {
                        let idx = rng.below(model.len() as u64) as usize;
                        model[idx].meta.pinned = !model[idx].meta.pinned;
                    }
                }
                _ => {
                    // move cursor to a random restorable entry (or None)
                    let cands: Vec<u64> = model
                        .iter()
                        .filter(|e| e.snapshot.is_some())
                        .map(|e| e.meta.entry_id)
                        .collect();
                    current = if cands.is_empty() {
                        None
                    } else {
                        Some(cands[rng.below(cands.len() as u64) as usize])
                    };
                }
            }

            store.persist(&model, current, next_id).unwrap();
            assert_loads_as(&dir, &model, current);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Fault-injection: hammer the store with random blob deletion/corruption
    /// and manifest corruption between persists. Opening must NEVER panic or
    /// error, available entries must still round-trip, and ids must never
    /// regress (preserved blobs are reused).
    #[test]
    fn fault_injection_never_crashes_and_recovers() {
        let dir = tmp_dir("faults");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut rng = Rng(0x00C0FFEE);
        let mut next_id = 1u64;

        for _ in 0..50 {
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

            let (s2, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
            assert!(
                res.next_entry_id >= *ids.last().unwrap(),
                "next_entry_id {} regressed below last issued id {}",
                res.next_entry_id,
                ids.last().unwrap()
            );
            for e in &res.entries {
                if e.available {
                    assert_eq!(e.meta.kind, HistoryEntryKind::Snapshot);
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
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let committed = vec![entry(1, "A", false, 3), entry(2, "B", false, 3)];
        store.persist(&committed, Some(2), 3).unwrap();
        // Simulate "blob written, then crash before manifest publish".
        std::fs::write(blob_path(&dir, 3), b"halfwritten").unwrap();

        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 2, "only committed entries");
        assert_eq!(res.entries[0].meta.entry_id, 1);
        assert_eq!(res.entries[1].meta.entry_id, 2);
        assert!(!blob_path(&dir, 3).exists(), "orphan blob GC'd");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A leftover manifest.json.tmp (crash during a previous publish) must be
    /// ignored — the committed manifest.json still loads cleanly.
    #[test]
    fn stray_manifest_tmp_is_ignored() {
        let dir = tmp_dir("straytmp");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let entries = vec![entry(1, "A", false, 3)];
        store.persist(&entries, Some(1), 2).unwrap();
        std::fs::write(dir.join("manifest.json.tmp"), b"{ garbage").unwrap();
        assert_loads_as(&dir, &entries, Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cursor_resolves_when_current_unavailable() {
        let dir = tmp_dir("cursor");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
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
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert_eq!(res.current_entry_id, Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_manifest_preserves_blobs() {
        let dir = tmp_dir("corruptman");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
            .unwrap();
        let blob1 = std::fs::read(blob_path(&dir, 1)).unwrap();
        let blob2 = std::fs::read(blob_path(&dir, 2)).unwrap();
        // Corrupt the manifest — blobs must be quarantined out of GC reach,
        // not just left for the next persist to orphan-GC.
        std::fs::write(dir.join("manifest.json"), b"{ not valid json").unwrap();
        let (mut store2, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(res.entries.is_empty(), "unusable manifest -> no entries");
        assert!(
            res.warnings.iter().any(|w| w.contains("preserved")),
            "quarantine warning, got: {:?}",
            res.warnings
        );
        assert!(
            !blob_path(&dir, 1).exists(),
            "quarantined blob no longer at its live path"
        );
        // App persists its (empty) history, then the UI closes and reopens:
        // the replacement manifest must not cost the quarantined bytes.
        store2.persist(&[], None, res.next_entry_id).unwrap();
        let (_s3, res3) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(res3.entries.is_empty());
        assert_eq!(
            std::fs::read(dir.join("1.tasrec.orphaned")).unwrap(),
            blob1,
            "quarantined bytes survive persist + reopen"
        );
        assert_eq!(
            std::fs::read(dir.join("2.tasrec.orphaned")).unwrap(),
            blob2,
            "quarantined bytes survive persist + reopen"
        );
        assert!(
            dir.join("manifest.corrupt.json").exists(),
            "bad manifest set aside, not silently overwritten"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unavailable_entry_not_demoted_to_marker_on_repersist() {
        let dir = tmp_dir("undemote");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(
                &[entry(1, "A", false, 3), entry(2, "B", false, 3)],
                Some(2),
                3,
            )
            .unwrap();
        std::fs::remove_file(blob_path(&dir, 1)).unwrap(); // blob 1 disappears

        let (mut store2, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
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
        let (_s3, res3) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(
            !res3.entries[0].available,
            "preserved as missing-blob entry, not demoted to a marker"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A blob whose on-disk length disagrees with the manifest (tamper /
    /// truncation / a forged-huge file) is rejected as corrupt by metadata —
    /// it is never slurped into memory, so it cannot OOM the loader.
    #[test]
    fn blob_size_mismatch_rejected_without_reading() {
        let dir = tmp_dir("sizemismatch");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&[entry(1, "A", false, 4)], Some(1), 2)
            .unwrap();
        // Grow the blob so its length no longer matches the manifest `size`.
        let mut bytes = std::fs::read(blob_path(&dir, 1)).unwrap();
        bytes.extend_from_slice(&[0u8; 32]);
        std::fs::write(blob_path(&dir, 1), &bytes).unwrap();

        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(
            !res.entries[0].available,
            "size-mismatched blob is unavailable"
        );
        assert!(dir.join("1.tasrec.corrupt").exists(), "quarantined");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A marker row (no blob) must NOT shield a stray `<id>.tasrec` from
    /// orphan GC just because it shares the id.
    #[test]
    fn marker_row_does_not_protect_stray_blob() {
        let dir = tmp_dir("markerstray");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store.persist(&[marker(1, "saved")], None, 2).unwrap();
        std::fs::write(blob_path(&dir, 1), b"stray").unwrap();
        let (_s, _res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert!(
            !blob_path(&dir, 1).exists(),
            "stray blob at a marker id must be GC'd, not protected"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Blobs are the raw layout (`4 + count * 13` bytes) and round-trip
    /// bit-exactly (f32 via to/from_le_bytes).
    #[test]
    fn blobs_use_raw_format() {
        let dir = tmp_dir("rawfmt");
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let e = entry(1, "A", false, 5);
        store.persist(std::slice::from_ref(&e), Some(1), 2).unwrap();
        let len = std::fs::metadata(blob_path(&dir, 1)).unwrap().len();
        assert_eq!(len, (4 + 5 * BLOB_BYTES_PER_TICK) as u64);
        assert_loads_as(&dir, &[e], Some(1));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Manifests written before a field existed still load (missing optional
    /// fields default), and per-entry keys the store no longer writes are
    /// ignored.
    #[test]
    fn older_manifest_rows_load_with_defaults() {
        let dir = tmp_dir("oldrows");
        std::fs::create_dir_all(&dir).unwrap();
        let bytes = serialize_raw_blob(&snap(2, 1));
        std::fs::write(blob_path(&dir, 1), &bytes).unwrap();
        let manifest = serde_json::json!({
            "magic": MAGIC, "schema": SCHEMA, "blob_format": CURRENT_BLOB_FORMAT,
            "hash_algo": HASH_ALGO, "next_entry_id": 2, "current_entry_id": 1,
            "entries": [{
                "entry_id": 1, "name": "A", "pinned": false, "kind": "snapshot",
                "start_tick": 0, "end_tick": 2, "first_moving": null,
                "created_at_iso": "2026-06-06T00:00:00+00:00",
                "size": bytes.len(), "checksum": crc32fast::hash(&bytes), "blob_format": 2
            }]
        });
        std::fs::write(dir.join("manifest.json"), manifest.to_string()).unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        assert_eq!(res.entries.len(), 1);
        assert!(res.entries[0].available);
        assert_eq!(res.entries[0].meta.level, None);
        assert_eq!(res.entries[0].meta.user_name, None);
        assert_eq!(res.entries[0].snapshot.as_ref().unwrap(), &snap(2, 1));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
