//! Undo/redo recording history: entries, per-level filtering, pinning, the soft
//! cap, and the bridge to the on-disk v2 history store.

use super::{FinishStamp, IdentityStamps, RecordingSnapshot};
use crate::history_store::BlobRef;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tas_shared::TasSharedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryEntryKind {
    Snapshot,
    SaveMarker,
    LoadSnapshot,
}

/// Where an entry's recording lives. Entries loaded from the v2 store start
/// `OnDisk`: the panel only needs the metadata, and a restore reads the blob
/// then. A resident snapshot costs a fixed 852 KB (fixed-size buffers), so
/// keeping every entry resident would cost hundreds of MB.
pub(crate) enum SnapshotSlot {
    /// Marker entry (save/load landmark): nothing to restore.
    Marker,
    /// Resident. `on_disk` is set once the store holds this exact blob, which
    /// makes the resident copy droppable (`demote_resident_except`).
    Loaded {
        snapshot: RecordingSnapshot,
        on_disk: Option<BlobRef>,
    },
    /// Restorable; read from the store on demand.
    OnDisk(BlobRef),
    /// Blob missing/corrupt: visible but inert. The store keeps the blob
    /// reference itself (so a reappearing file recovers), nothing is needed here.
    Unavailable,
}

impl SnapshotSlot {
    fn restorable(&self) -> bool {
        matches!(self, SnapshotSlot::Loaded { .. } | SnapshotSlot::OnDisk(_))
    }

    fn loaded(&self) -> Option<&RecordingSnapshot> {
        match self {
            SnapshotSlot::Loaded { snapshot, .. } => Some(snapshot),
            _ => None,
        }
    }
}

pub struct HistoryEntry {
    /// Stable, monotonic, never-reused id (the storage identity — NOT the
    /// positional index). Assigned by `RecordingHistory` on push.
    pub entry_id: u64,
    /// Pinned entries are exempt from cap-eviction and never GC'd — durable
    /// named checkpoints that survive across sessions.
    pub pinned: bool,
    /// User-given name (via rename). When set, it's shown instead of the
    /// auto-generated `label`. `None` = use the auto label/duration.
    pub custom_name: Option<String>,
    pub label: String,
    /// Full local-tz creation time. Used for date grouping in the panel.
    pub created_at: chrono::DateTime<chrono::Local>,
    pub kind: HistoryEntryKind,
    /// Session start tick: 0 for REC entries (recording from the beginning)
    /// or for markers, the resume tick for CONT entries.
    pub start_tick: u32,
    /// `recorded_count` at push time. Zero on markers.
    pub end_tick: u32,
    /// First tick where the recorded position diverged from `rec_coords[0]`
    /// — the "race-start" landmark. `None` for markers and snapshots with no
    /// detected movement.
    pub first_moving: Option<u32>,
    /// Race time in centiseconds when the session ended by crossing the
    /// finish line (the HUD timer the finish-line watch auto-stopped at).
    /// `None` = the session was stopped by hand. Drives the
    /// "Finish m:ss.cc" label and the flag in the panel.
    pub finish_time_cs: Option<u32>,
    /// `true` when `finish_time_cs` came from the HUD timer (exact); `false`
    /// when it was derived from the recording's start-line / finish-line
    /// crossings (within ~0.05 s of the game's timer - shown with a "~").
    pub finish_time_exact: bool,
    /// Level code (e.g. "FE") the entry was created on, from the DLL's live
    /// level_id at push time. `None` when the level was unknown (menu). Used by
    /// the panel's per-level filter.
    pub level: Option<String>,
    /// Physics-mode stamp at push time (`tas_shared::physics_mode_label`,
    /// e.g. `OpenGL/53-bit`). None = unknown / pre-stamp entry. Restoring an
    /// entry under a different mode replays different physics.
    pub physics: Option<String>,
    /// Rider stamp at push time (`tas_shared::rider_label`, e.g.
    /// `Vincent · goofy`). None = unknown / pre-stamp entry. Restoring an
    /// entry recorded as another character or stance replays different
    /// physics.
    pub rider: Option<String>,
    /// Raw identity words behind the stamps, for exact save round trips.
    pub stamps: IdentityStamps,
    snapshot: SnapshotSlot,
}

impl HistoryEntry {
    fn from_snapshot(label: String, kind: HistoryEntryKind, snapshot: RecordingSnapshot) -> Self {
        let now = chrono::Local::now();
        let end_tick = snapshot.recorded_count;
        let first_moving =
            tas_shared::align::detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            created_at: now,
            kind,
            // start_tick is overwritten by `with_session` for CONT entries
            // that know their resume point; REC entries leave it at 0.
            start_tick: 0,
            end_tick,
            first_moving,
            finish_time_cs: None, // set by push_completed_session for finished runs
            finish_time_exact: false,
            level: None,   // stamped from live_level by RecordingHistory on push
            physics: None, // stamped from live_physics by RecordingHistory on push
            rider: None,   // stamped from live_rider by RecordingHistory on push
            stamps: IdentityStamps::default(), // stamped from live_stamps on push
            snapshot: SnapshotSlot::Loaded {
                snapshot,
                on_disk: None,
            },
        }
    }

    fn marker(label: String, kind: HistoryEntryKind) -> Self {
        let now = chrono::Local::now();
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            created_at: now,
            kind,
            start_tick: 0,
            end_tick: 0,
            first_moving: None,
            finish_time_cs: None,
            finish_time_exact: false,
            level: None,   // stamped from live_level by RecordingHistory on push
            physics: None, // stamped from live_physics by RecordingHistory on push
            rider: None,   // stamped from live_rider by RecordingHistory on push
            stamps: IdentityStamps::default(),
            snapshot: SnapshotSlot::Marker,
        }
    }

    fn with_session(mut self, start_tick: u32, end_tick: u32) -> Self {
        self.start_tick = start_tick;
        self.end_tick = end_tick;
        self
    }

    pub fn can_restore(&self) -> bool {
        self.snapshot.restorable()
    }
}

pub struct RecordingHistory {
    capacity: usize,
    entries: Vec<HistoryEntry>,
    current_index: Option<usize>,
    /// Next stable entry id to hand out. Authoritative + monotonic; never
    /// reused. Restored (and bumped past) on load.
    next_entry_id: u64,
    /// Bumped on every mutation (structural / metadata / cursor) so the app can
    /// cheaply detect "history changed, re-persist" without diffing.
    revision: u64,
    /// Current level code (e.g. "FE") from the DLL's live level_id; the app
    /// refreshes it every frame. Stamped onto each entry at push time so the
    /// panel can filter history per level. None = unknown/menu.
    live_level: Option<String>,
    /// Set between a level change and the scan publishing the new track, so
    /// "we don't know yet" is distinguishable from "we are on this track".
    level_resolving: bool,
    /// v2 store directory; where `OnDisk` entries are read from on restore.
    blob_dir: Option<PathBuf>,
    /// Problems hit while reading blobs on demand (the app logs and clears).
    warnings: Vec<String>,
    /// Live physics-mode stamp from the DLL (renderer + x87 precision),
    /// refreshed by the app every frame; stamped onto pushed entries.
    live_physics: Option<String>,
    /// Live rider stamp (character · stance), see `set_live_rider`.
    live_rider: Option<String>,
    /// Live raw identity words, refreshed alongside the label stamps;
    /// stamped onto pushed entries for exact save round trips.
    live_stamps: Option<IdentityStamps>,
}

impl RecordingHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            entries: Vec::with_capacity(capacity.max(1)),
            current_index: None,
            next_entry_id: 1,
            revision: 0,
            live_level: None,
            level_resolving: false,
            blob_dir: None,
            warnings: Vec::new(),
            live_physics: None,
            live_rider: None,
            live_stamps: None,
        }
    }

    /// Refresh the physics-mode stamp given to subsequently pushed entries.
    pub fn set_live_physics(&mut self, label: Option<String>) {
        if label.is_some() && self.live_physics != label {
            self.live_physics = label;
        }
    }

    pub fn live_physics(&self) -> Option<&str> {
        self.live_physics.as_deref()
    }

    /// Live rider stamp (character · stance). Like the physics stamp, an
    /// unknown live value never erases a known one.
    pub fn set_live_rider(&mut self, label: Option<String>) {
        if label.is_some() && self.live_rider != label {
            self.live_rider = label;
        }
    }

    pub fn live_rider(&self) -> Option<&str> {
        self.live_rider.as_deref()
    }

    /// Refresh the raw identity words given to subsequently pushed entries.
    /// Unknown halves never erase known ones (same rule as the labels).
    pub fn set_live_stamps(&mut self, stamps: IdentityStamps) {
        let cur = self.live_stamps.get_or_insert_with(IdentityStamps::default);
        if stamps.renderer_id.is_some() {
            cur.renderer_id = stamps.renderer_id;
        }
        if stamps.fpu_control_word.is_some() {
            cur.fpu_control_word = stamps.fpu_control_word;
        }
        if stamps.rider_character.is_some() {
            cur.rider_character = stamps.rider_character;
        }
        if stamps.rider_stance.is_some() {
            cur.rider_stance = stamps.rider_stance;
        }
    }

    /// Where lazily-loaded entries read their blobs from. Set before
    /// `apply_loaded`; without it an `OnDisk` entry cannot be restored.
    pub fn set_blob_dir(&mut self, dir: PathBuf) {
        self.blob_dir = Some(dir);
    }

    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }
    /// Make entry `index` resident (reading its blob if needed) and return it.
    /// A blob that fails to read turns the entry `Unavailable` and records a
    /// warning; the cursor is NOT moved here so a failed restore leaves the
    /// selection where it was.
    fn load_slot(&mut self, index: usize) -> Option<&RecordingSnapshot> {
        let blob = match &self.entries[index].snapshot {
            SnapshotSlot::Loaded { .. } => return self.entries[index].snapshot.loaded(),
            SnapshotSlot::OnDisk(blob) => *blob,
            SnapshotSlot::Marker | SnapshotSlot::Unavailable => return None,
        };
        let id = self.entries[index].entry_id;
        let loaded = self
            .blob_dir
            .as_deref()
            .ok_or_else(|| "history store directory unknown".to_string())
            .and_then(|dir| crate::history_store::load_blob(dir, id, blob))
            .and_then(RecordingSnapshot::from_persisted);
        match loaded {
            Ok(snapshot) => {
                self.entries[index].snapshot = SnapshotSlot::Loaded {
                    snapshot,
                    on_disk: Some(blob),
                };
                self.entries[index].snapshot.loaded()
            }
            Err(e) => {
                self.warnings.push(format!(
                    "history entry {} ('{}') cannot be restored: {}",
                    id, self.entries[index].label, e
                ));
                self.entries[index].snapshot = SnapshotSlot::Unavailable;
                None
            }
        }
    }

    /// Drop resident copies the store already holds, except `keep` and the
    /// current entry. Restores are rare user actions and a blob re-reads in
    /// about a millisecond, so "at most the one you just used" is enough.
    fn demote_resident_except(&mut self, keep: usize) {
        let current = self.current_index;
        for (i, e) in self.entries.iter_mut().enumerate() {
            if i == keep || current == Some(i) {
                continue;
            }
            if let SnapshotSlot::Loaded {
                on_disk: Some(blob),
                ..
            } = &e.snapshot
            {
                let blob = *blob;
                e.snapshot = SnapshotSlot::OnDisk(blob);
            }
        }
    }

    /// The writer committed `id`'s blob: its resident copy is now droppable,
    /// and is dropped unless it is the current entry.
    pub fn mark_durable(&mut self, id: u64, blob: BlobRef) {
        let Some(i) = self.entries.iter().position(|e| e.entry_id == id) else {
            return;
        };
        if let SnapshotSlot::Loaded { on_disk, .. } = &mut self.entries[i].snapshot {
            if on_disk.is_none() {
                *on_disk = Some(blob);
            }
        }
        if self.current_index != Some(i) {
            if let SnapshotSlot::Loaded {
                on_disk: Some(blob),
                ..
            } = &self.entries[i].snapshot
            {
                let blob = *blob;
                self.entries[i].snapshot = SnapshotSlot::OnDisk(blob);
            }
        }
    }

    /// Refresh the level code stamped onto subsequently pushed entries and
    /// used by the panel's per-level view. STICKY: `None` (menu / unknown) is
    /// ignored so the panel keeps showing the track you were just on — you're
    /// almost always between restarts of the same level, and entries pushed
    /// at the menu (e.g. a save marker right after a run) still belong to it.
    /// Not a history mutation — does not bump the revision.
    pub fn set_live_level(&mut self, level: Option<&str>) {
        if let Some(code) = level {
            if self.live_level.as_deref() != Some(code) {
                self.live_level = Some(code.to_owned());
            }
            // A concrete reading ends any transition.
            self.level_resolving = false;
        }
    }

    /// We are in a level context whose track has not been identified yet.
    ///
    /// Driven by the DLL's `level_epoch` / `level_scan_epoch` pair: level_scan
    /// bumps the epoch when the level path changes or becomes unavailable, and
    /// the scan epoch catches up once the new track is identified. Until then
    /// the old track must not be asserted: a wrong tag is worse than no tag,
    /// because an untagged entry is visibly untagged and a mis-stamped one is
    /// not.
    ///
    /// Idempotent; called every frame while unresolved.
    pub fn enter_resolving(&mut self) {
        self.live_level = None;
        self.level_resolving = true;
    }

    /// True between a level change and the scan publishing the new track. The
    /// panel should say so rather than assert a track it cannot currently know.
    pub fn level_is_resolving(&self) -> bool {
        self.level_resolving
    }

    /// The level code new entries are currently stamped with (None = unknown).
    pub fn live_level(&self) -> Option<&str> {
        self.live_level.as_deref()
    }

    /// Backfill level tags on entries persisted before tagging existed, by
    /// classifying each snapshot's spawn position (rec_coords[0]). Only
    /// unambiguous spawns are tagged (the classifier refuses shared clusters),
    /// so a wrong tag can't hide an entry from its real level. Returns the
    /// number of entries tagged.
    pub fn backfill_levels<F>(&mut self, classify: F) -> usize
    where
        F: Fn(&[f32; 3]) -> Option<&'static str>,
    {
        let mut tagged = 0;
        for i in 0..self.entries.len() {
            if self.entries[i].level.is_some() {
                continue;
            }
            // Only the spawn coordinate is needed. For an on-disk entry read
            // the blob transiently rather than making it resident: this runs
            // once per untagged entry, and the tag is persisted afterwards.
            let spawn = match &self.entries[i].snapshot {
                SnapshotSlot::Loaded { snapshot, .. } => {
                    (snapshot.recorded_count > 0).then_some(snapshot.rec_coords[0])
                }
                SnapshotSlot::OnDisk(blob) => self.blob_dir.as_deref().and_then(|dir| {
                    crate::history_store::load_blob(dir, self.entries[i].entry_id, *blob)
                        .ok()
                        .and_then(|ps| ps.rec_coords.first().copied())
                }),
                SnapshotSlot::Marker | SnapshotSlot::Unavailable => None,
            };
            let Some(spawn) = spawn else {
                continue;
            };
            if let Some(code) = classify(&spawn) {
                self.entries[i].level = Some(code.to_string());
                tagged += 1;
            }
        }
        if tagged > 0 {
            self.bump(); // persist the new tags
        }
        tagged
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_entry_id;
        self.next_entry_id = self
            .next_entry_id
            .checked_add(1)
            .expect("entry_id overflow");
        id
    }

    /// Monotonic change counter — compare across frames to know if a persist is
    /// needed. Rename and pin bump this even though len/cursor are unchanged.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    fn bump(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn next_entry_id(&self) -> u64 {
        self.next_entry_id
    }

    /// Raise the next-id allocator to at least `floor`. Used on startup so an
    /// EMPTY load (a valid empty manifest, or a corrupt manifest that preserved
    /// blobs) still advances past any id that already exists on disk — otherwise
    /// the next new entry restarts at 1 and collides with a preserved blob.
    pub fn adopt_id_floor(&mut self, floor: u64) {
        if floor > self.next_entry_id {
            self.next_entry_id = floor;
            self.bump();
        }
    }

    pub fn push_snapshot(&mut self, state: &TasSharedState, label: impl Into<String>) -> bool {
        self.push_snapshot_data(RecordingSnapshot::from_state(state), label)
    }

    pub fn push_snapshot_data(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            None,
            None,
            None,
        )
    }

    /// Like `push_snapshot_data` but records the session's `start_tick` /
    /// `end_tick` on the entry so the panel can render "from <tick> ·
    /// <in-game time>" without parsing the label string. `stamps` overrides
    /// the live identity for takes that were not just recorded (edits of a
    /// loaded take); `None` stamps the live words (fresh sessions).
    pub fn push_snapshot_data_with_session(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
        start_tick: u32,
        end_tick: u32,
        stamps: Option<IdentityStamps>,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            Some((start_tick, end_tick)),
            None,
            stamps,
        )
    }

    /// A REC / CONT session that just ended. `finish` is the race time when
    /// the finish-line watch stopped it (the entry then shows the flag and
    /// "Finish m:ss.cc", whatever it is later renamed to); `None` for a
    /// session stopped by hand.
    pub fn push_completed_session(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
        start_tick: u32,
        end_tick: u32,
        finish: Option<FinishStamp>,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            Some((start_tick, end_tick)),
            finish,
            None,
        )
    }

    /// A loaded file's identity comes from its header, not the live game —
    /// the caller passes the take's stamps (`None` only when the header had
    /// none, which stays unknown rather than backfilling live).
    pub fn push_loaded_snapshot(
        &mut self,
        state: &TasSharedState,
        path: &Path,
        stamps: Option<IdentityStamps>,
    ) -> bool {
        let label = format!("Load: {}", short_file_label(path));
        self.push_snapshot_entry(
            RecordingSnapshot::from_state(state),
            label,
            HistoryEntryKind::LoadSnapshot,
            None,
            None,
            stamps,
        )
    }

    /// Record a save marker without changing the current restored state.
    /// `stamps` carries the saved take's identity into the auto-pushed
    /// "Current recording" entry when the buffer holds a loaded take;
    /// `None` keeps the live stamps (a genuinely new recording).
    pub fn push_save_marker(
        &mut self,
        state: &TasSharedState,
        path: &Path,
        stamps: Option<IdentityStamps>,
    ) {
        let pushed = self.current_index.is_none()
            && state.recorded_count > 0
            && self.push_snapshot(state, "Current recording");
        if let Some((id, stamps)) = pushed
            .then(|| self.current_entry_id().zip(stamps))
            .flatten()
        {
            self.set_stamps(id, stamps);
        }
        let label = format!("Save: {}", short_file_label(path));
        let mut marker = HistoryEntry::marker(label, HistoryEntryKind::SaveMarker);
        marker.entry_id = self.alloc_id();
        marker.level = self.live_level.clone();
        if let Some(current) = self.current_index {
            // Save belongs to the current visible state without changing selection.
            let insert_at = (current + 1).min(self.entries.len());
            self.entries.insert(insert_at, marker);
            self.enforce_capacity();
        } else {
            self.entries.push(marker);
            self.enforce_capacity();
        }
        self.bump();
    }

    /// Whether entry `i` belongs to the track we are currently on — the SAME
    /// rule the history panel filters by (matching level, or untagged).
    ///
    /// Undo, redo and restore all check it, because they index entries
    /// directly and must not restore another track's recording over the
    /// live buffer.
    pub fn entry_on_current_level(&self, i: usize) -> bool {
        if i >= self.entries.len() {
            return false;
        }
        // While resolving the track is unknown, so nothing qualifies.
        if self.level_resolving {
            return false;
        }
        match (self.live_level.as_deref(), self.entries[i].level.as_deref()) {
            (Some(want), Some(have)) => want == have,
            // Untagged entries (made before tagging, or while the track was
            // unknown) are allowed everywhere: hiding them would strand the
            // user's history. The guarantee is "nothing tagged with another
            // track", not "nothing unknown".
            _ => true,
        }
    }

    pub fn undo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let prev = (0..current)
            .rev()
            .find(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))?;
        // Load BEFORE moving the cursor: an unreadable blob must not leave the
        // selection on an entry that just turned inert.
        self.load_slot(prev)?;
        self.current_index = Some(prev);
        self.demote_resident_except(prev);
        self.bump();
        self.entries[prev].snapshot.loaded()
    }

    pub fn redo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let next = ((current + 1)..self.entries.len())
            .find(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))?;
        self.load_slot(next)?;
        self.current_index = Some(next);
        self.demote_resident_except(next);
        self.bump();
        self.entries[next].snapshot.loaded()
    }

    pub fn restore_index(&mut self, index: usize) -> Option<&RecordingSnapshot> {
        if index >= self.entries.len() {
            return None;
        }
        // The index comes from the UI, so enforce the per-level rule here
        // rather than relying on the panel's row filter.
        if !self.entry_on_current_level(index) {
            return None;
        }
        self.load_slot(index)?;
        self.current_index = Some(index);
        self.demote_resident_except(index);
        self.bump();
        self.entries[index].snapshot.loaded()
    }

    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// Clear the "current" pointer so no row is highlighted. Does not
    /// touch any entry data. Used by the panel when the user clicks
    /// empty space to deselect.
    pub fn clear_selection(&mut self) {
        self.current_index = None;
        self.bump();
    }

    // ===== v2 store bridge =====

    /// Convert the in-memory history to the store's entry list (row order).
    pub fn to_stored_entries(&self) -> Vec<crate::history_store::StoredEntry> {
        self.entries
            .iter()
            .map(|e| crate::history_store::StoredEntry {
                meta: crate::history_store::EntryMeta {
                    entry_id: e.entry_id,
                    name: e.label.clone(),
                    user_name: e.custom_name.clone(),
                    pinned: e.pinned,
                    kind: e.kind,
                    start_tick: e.start_tick,
                    end_tick: e.end_tick,
                    first_moving: e.first_moving,
                    finish_time_cs: e.finish_time_cs,
                    finish_time_exact: e.finish_time_exact,
                    level: e.level.clone(),
                    physics: e.physics.clone(),
                    rider: e.rider.clone(),
                    stamps: e.stamps.clone(),
                    created_at_iso: e.created_at.to_rfc3339(),
                },
                // Bytes travel only for snapshots the store does not have
                // yet. On-disk and durable entries send `None`; the store
                // keeps their blob reference (ids are never reused).
                snapshot: match &e.snapshot {
                    SnapshotSlot::Loaded {
                        snapshot,
                        on_disk: None,
                    } => Some(snapshot.to_persisted()),
                    _ => None,
                },
            })
            .collect()
    }

    /// The stable id of the current (selected) entry, if any.
    pub fn current_entry_id(&self) -> Option<u64> {
        self.current_index.map(|i| self.entries[i].entry_id)
    }

    /// Tag an entry with the track it belongs to.
    ///
    /// Used for recovered checkpoints, which carry the level they were
    /// recorded on; the live level is unknown at startup. `None` leaves the
    /// entry untagged, i.e. visible on every track.
    pub fn set_level(&mut self, entry_id: u64, level: Option<String>) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.level == level {
            return false;
        }
        e.level = level;
        self.bump();
        true
    }

    /// Stamp the rider a recovered entry was recorded as (the checkpoint
    /// carries it; the live rider is unknown during startup). See set_level.
    pub fn set_rider(&mut self, entry_id: u64, rider: Option<String>) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.rider == rider {
            return false;
        }
        e.rider = rider;
        self.bump();
        true
    }

    /// Stamp the physics mode a recovered entry was recorded under (the
    /// checkpoint carries it; the live physics is unknown during startup).
    /// See set_level.
    pub fn set_physics(&mut self, entry_id: u64, physics: Option<String>) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.physics == physics {
            return false;
        }
        e.physics = physics;
        self.bump();
        true
    }

    /// Stamp the raw identity words a recovered entry was recorded with (the
    /// checkpoint carries them; the live words are unknown during startup).
    /// See set_level.
    pub fn set_stamps(&mut self, entry_id: u64, stamps: IdentityStamps) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.stamps == stamps {
            return false;
        }
        e.stamps = stamps;
        self.bump();
        true
    }

    pub fn set_pinned(&mut self, entry_id: u64, pinned: bool) -> bool {
        let mut changed = false;
        let found = if let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) {
            if e.pinned != pinned {
                e.pinned = pinned;
                changed = true;
            }
            true
        } else {
            false
        };
        if changed {
            self.bump();
            if !pinned {
                // Unpinning can push the unpinned count back over the cap.
                self.enforce_capacity();
            }
        }
        found
    }

    /// Set (or clear, if blank) the user-given name of an entry. Leaves the
    /// auto `label` (and its duration) intact; the name is shown instead.
    pub fn rename(&mut self, entry_id: u64, name: impl Into<String>) -> bool {
        let name = name.into();
        let new = {
            let t = name.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        };
        if let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) {
            if e.custom_name != new {
                e.custom_name = new;
                self.bump();
            }
            true
        } else {
            false
        }
    }

    /// Change the soft cap (max unpinned entries) and trim immediately. Used
    /// by the config-panel setting.
    pub fn set_capacity(&mut self, capacity: usize) {
        let cap = capacity.max(1);
        if cap != self.capacity {
            self.capacity = cap;
            self.enforce_capacity();
            self.bump();
        }
    }

    /// Rebuild the in-memory history from a v2-store load. Unavailable entries
    /// (snapshot == None but kind expects one) come in inert (can't restore).
    pub fn apply_loaded(
        &mut self,
        loaded: Vec<crate::history_store::LoadedEntry>,
        current_entry_id: Option<u64>,
        next_entry_id_floor: u64,
    ) {
        let mut entries = Vec::with_capacity(loaded.len());
        for le in loaded {
            let snapshot = match (le.snapshot, le.blob, le.available) {
                (Some(ps), blob, _) => match RecordingSnapshot::from_persisted(ps) {
                    Ok(snapshot) => SnapshotSlot::Loaded {
                        snapshot,
                        on_disk: blob,
                    },
                    Err(_) => {
                        if blob.is_some() {
                            SnapshotSlot::Unavailable
                        } else {
                            SnapshotSlot::Marker
                        }
                    }
                },
                (None, Some(blob), true) => SnapshotSlot::OnDisk(blob),
                (None, Some(_), false) => SnapshotSlot::Unavailable,
                (None, None, _) => SnapshotSlot::Marker,
            };
            let meta = le.meta;
            let created_at = chrono::DateTime::parse_from_rfc3339(&meta.created_at_iso)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local))
                .unwrap_or_else(chrono::Local::now);
            entries.push(HistoryEntry {
                entry_id: meta.entry_id,
                pinned: meta.pinned,
                custom_name: meta.user_name,
                label: meta.name,
                created_at,
                kind: meta.kind,
                start_tick: meta.start_tick,
                end_tick: meta.end_tick,
                first_moving: meta.first_moving,
                finish_time_cs: meta.finish_time_cs,
                finish_time_exact: meta.finish_time_exact,
                level: meta.level,
                physics: meta.physics,
                rider: meta.rider,
                stamps: meta.stamps,
                snapshot,
            });
        }
        let max_id_plus_1 = entries.iter().map(|e| e.entry_id + 1).max().unwrap_or(1);
        self.entries = entries;
        self.next_entry_id = self
            .next_entry_id
            .max(next_entry_id_floor)
            .max(max_id_plus_1);
        // Respect the store's already-resolved cursor exactly (it fell back to
        // the nearest available entry, or None). Do NOT silently jump to newest.
        self.current_index = current_entry_id
            .and_then(|id| self.entries.iter().position(|e| e.entry_id == id))
            .filter(|&i| self.entries[i].can_restore());
        // A lowered cap (e.g. settings changed between sessions) trims on load.
        self.enforce_capacity_preserving_none();
        self.bump();
    }

    pub fn undo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        (0..current)
            .filter(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))
            .count()
    }

    pub fn redo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        ((current + 1)..self.entries.len())
            .filter(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))
            .count()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// Evict to the cap (see `evict_to_cap`), then select the newest
    /// restorable entry if nothing is selected.
    fn enforce_capacity(&mut self) {
        self.evict_to_cap();
        if self.current_index.is_none() {
            self.current_index = self
                .entries
                .iter()
                .enumerate()
                .rev()
                .find_map(|(idx, entry)| entry.can_restore().then_some(idx));
        }
    }

    /// Enforce the cap but leave a `None` cursor as `None` (used on load, where
    /// the store already resolved the cursor and a deliberate `None` must stand).
    fn enforce_capacity_preserving_none(&mut self) {
        self.evict_to_cap();
    }

    /// Soft cap: `capacity` bounds the UNPINNED count. Pinned entries and the
    /// current entry are never evicted; evicts the oldest unpinned, non-current
    /// entry until the unpinned count fits (so total can exceed `capacity` when
    /// there are many pins — pins win).
    fn evict_to_cap(&mut self) {
        loop {
            let unpinned = self.entries.iter().filter(|e| !e.pinned).count();
            if unpinned <= self.capacity {
                break;
            }
            let cur = self.current_index;
            let victim = self
                .entries
                .iter()
                .enumerate()
                .find(|(i, e)| !e.pinned && cur != Some(*i))
                .map(|(i, _)| i);
            let Some(victim) = victim else {
                break; // only pinned and/or the current entry remain
            };
            self.entries.remove(victim);
            self.current_index = self
                .current_index
                .map(|idx| if idx > victim { idx - 1 } else { idx });
        }
    }

    fn push_snapshot_entry(
        &mut self,
        snapshot: RecordingSnapshot,
        label: String,
        kind: HistoryEntryKind,
        session: Option<(u32, u32)>,
        finish: Option<FinishStamp>,
        stamps: Option<IdentityStamps>,
    ) -> bool {
        if snapshot.recorded_count == 0 {
            return false;
        }

        let mut entry = HistoryEntry::from_snapshot(label, kind, snapshot);
        entry.entry_id = self.alloc_id();
        entry.level = self.live_level.clone();
        entry.physics = self.live_physics.clone();
        entry.rider = self.live_rider.clone();
        entry.stamps = stamps
            .or_else(|| self.live_stamps.clone())
            .unwrap_or_default();
        entry.finish_time_cs = finish.map(|f| f.cs);
        entry.finish_time_exact = finish.is_some_and(|f| f.exact);
        if let Some((start_tick, end_tick)) = session {
            entry = entry.with_session(start_tick, end_tick);
        }
        self.entries.push(entry);
        let newest = self.entries.len() - 1;
        self.current_index = Some(newest);
        // The previous take is no longer current: if the store already holds
        // its blob, its resident 852 KB copy can go.
        self.demote_resident_except(newest);
        self.enforce_capacity();
        self.bump();
        true
    }
}

fn short_file_label(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recording::test_support::*;
    use crate::recording::{finished_session_label, PersistedSnapshot};

    #[test]
    fn history_new_is_empty() {
        let history = RecordingHistory::new(4);
        assert_eq!(history.len(), 0);
        assert_eq!(history.undo_depth(), 0);
        assert_eq!(history.redo_depth(), 0);
    }

    #[test]
    fn history_push_undo_redo() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        let c = one_tick_state(0x04);

        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        assert!(history.push_snapshot(&c, "C"));
        assert_eq!(history.current_index(), Some(2));
        assert_eq!(history.undo_depth(), 2);
        assert_eq!(history.redo_depth(), 0);

        let snap = history.undo().unwrap();
        assert_eq!(snap.input_log[0], 0x02);
        assert_eq!(history.current_index(), Some(1));
        assert_eq!(history.undo_depth(), 1);
        assert_eq!(history.redo_depth(), 1);

        let snap = history.redo().unwrap();
        assert_eq!(snap.input_log[0], 0x04);
        assert_eq!(history.current_index(), Some(2));
    }

    #[test]
    fn history_branch_keeps_previous_states_append_only() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        let c = one_tick_state(0x04);
        let d = one_tick_state(0x08);

        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        assert!(history.push_snapshot(&c, "C"));
        let _ = history.undo(); // now on B
        assert_eq!(history.current_index(), Some(1));
        assert_eq!(history.redo_depth(), 1);

        assert!(history.push_snapshot(&d, "D"));
        assert_eq!(history.len(), 4);
        assert_eq!(history.current_index(), Some(3));
        assert_eq!(history.entries()[2].label, "C");
        assert_eq!(history.entries()[3].label, "D");
        assert_eq!(history.redo_depth(), 0);
    }

    #[test]
    fn history_save_marker_keeps_current_cursor() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        let before = history.current_index();

        history.push_save_marker(&b, Path::new("C:\\temp\\run.tasrec"), None);
        assert_eq!(history.len(), 3);
        assert_eq!(history.current_index(), before);
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert!(!history.entries()[2].can_restore());
    }

    #[test]
    fn history_load_snapshot_becomes_current() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let loaded = one_tick_state(0x20);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_loaded_snapshot(&loaded, Path::new("C:\\temp\\loaded.tasrec"), None));
        let current = history.current_index().unwrap();
        assert_eq!(
            history.entries()[current].kind,
            HistoryEntryKind::LoadSnapshot
        );
        assert!(history.entries()[current].label.contains("loaded.tasrec"));
        assert_eq!(history.undo_depth(), 1);
    }

    #[test]
    fn history_restore_index_skips_non_restorable_entries() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        history.push_save_marker(&b, Path::new("run.tasrec"), None);

        // Save marker cannot be restored directly.
        assert!(history.restore_index(2).is_none());
        let snap = history.restore_index(0).unwrap();
        assert_eq!(snap.input_log[0], 0x01);
        assert_eq!(history.current_index(), Some(0));
    }

    #[test]
    fn history_capacity_eviction_preserves_recent_entries() {
        let mut history = RecordingHistory::new(2);
        assert!(history.push_snapshot(&one_tick_state(0x01), "A"));
        assert!(history.push_snapshot(&one_tick_state(0x02), "B"));
        assert!(history.push_snapshot(&one_tick_state(0x04), "C"));
        assert_eq!(history.len(), 2);
        assert_eq!(history.entries()[0].label, "B");
        assert_eq!(history.entries()[1].label, "C");
        assert_eq!(history.current_index(), Some(1));
    }

    /// `adopt_id_floor` raises the allocator (never lowers it), so an empty
    /// load that nonetheless has a high stored next-id can't reissue old ids.
    #[test]
    fn adopt_id_floor_only_raises() {
        let mut h = RecordingHistory::new(8);
        h.adopt_id_floor(50);
        assert_eq!(h.next_entry_id(), 50);
        h.adopt_id_floor(10);
        assert_eq!(h.next_entry_id(), 50, "floor never lowers the allocator");
    }

    #[test]
    fn lazy_history_entries_restore_from_store_and_stay_off_heap() {
        use crate::history_store::{HistoryStore, StoredEntry};
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_lazy_hist_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mk = |id: u64, count: u32| {
            let mut input_log = vec![0u8; count as usize];
            input_log[0] = id as u8;
            StoredEntry {
                meta: crate::history_store::EntryMeta {
                    entry_id: id,
                    name: format!("take {}", id),
                    user_name: None,
                    pinned: false,
                    kind: HistoryEntryKind::Snapshot,
                    start_tick: 0,
                    end_tick: count,
                    first_moving: None,
                    finish_time_cs: None,
                    finish_time_exact: false,
                    level: None,
                    physics: None,
                    rider: None,
                    stamps: IdentityStamps::default(),
                    created_at_iso: "2026-09-02T00:00:00+10:00".to_string(),
                },
                snapshot: Some(PersistedSnapshot {
                    recorded_count: count,
                    input_log,
                    rec_coords: vec![[id as f32, 0.0, 0.0]; count as usize],
                }),
            }
        };
        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store.persist(&[mk(1, 3), mk(2, 5)], Some(2), 3).unwrap();
        drop(store);

        let (_lazy, load) = HistoryStore::open_lazy(dir.clone()).unwrap();
        let mut history = RecordingHistory::new(8);
        history.set_blob_dir(dir.clone());
        history.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(history.len(), 2);
        assert!(history.entries().iter().all(|e| e.can_restore()));
        assert!(
            history
                .entries()
                .iter()
                .all(|e| matches!(e.snapshot, SnapshotSlot::OnDisk(_))),
            "nothing resident after a lazy load"
        );
        // Re-persisting sends no bytes for on-disk entries.
        assert!(history
            .to_stored_entries()
            .iter()
            .all(|e| e.snapshot.is_none()));

        // Restore reads the blob on demand.
        let snap = history.restore_index(0).expect("restorable");
        assert_eq!(snap.recorded_count, 3);
        assert_eq!(snap.input_log[0], 1);
        assert!(matches!(
            history.entries()[0].snapshot,
            SnapshotSlot::Loaded { .. }
        ));
        // Restoring another entry drops the previous resident copy to disk.
        history.restore_index(1).expect("restorable");
        assert!(matches!(
            history.entries()[0].snapshot,
            SnapshotSlot::OnDisk(_)
        ));
        assert!(matches!(
            history.entries()[1].snapshot,
            SnapshotSlot::Loaded { .. }
        ));

        // A corrupt blob makes the entry inert with a warning; the cursor
        // stays on the entry that was current.
        let p = dir.join("1.tasrec");
        let mut bytes = std::fs::read(&p).unwrap();
        bytes[4] ^= 0xFF;
        std::fs::write(&p, &bytes).unwrap();
        assert!(history.restore_index(0).is_none());
        assert!(!history.entries()[0].can_restore());
        assert_eq!(history.current_index(), Some(1));
        assert!(!history.take_warnings().is_empty());

        // A session take stays resident until the writer reports it durable,
        // then drops once it is no longer current.
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 4;
        state.input_log[0] = 9;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take 3".to_string(),
            0,
            4,
            None
        ));
        let stored = history.to_stored_entries();
        assert!(stored[2].snapshot.is_some(), "new take travels with bytes");
        let blob = crate::history_store::BlobRef {
            size: 0,
            checksum: 0,
        };
        history.mark_durable(stored[2].meta.entry_id, blob);
        assert!(
            matches!(history.entries()[2].snapshot, SnapshotSlot::Loaded { .. }),
            "current entry stays resident"
        );
        history.restore_index(1).expect("restorable");
        assert!(matches!(
            history.entries()[2].snapshot,
            SnapshotSlot::OnDisk(_)
        ));
        assert!(history.to_stored_entries()[2].snapshot.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn history_entries_carry_the_rider_stamp_through_the_store() {
        let mut history = RecordingHistory::new(8);
        history.set_live_rider(Some("Vincent · regular".to_string()));
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 3;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take".to_string(),
            0,
            3,
            None
        ));
        assert_eq!(
            history.entries()[0].rider.as_deref(),
            Some("Vincent · regular")
        );
        history.set_live_rider(None);
        assert_eq!(
            history.live_rider(),
            Some("Vincent · regular"),
            "unknown never erases known"
        );

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.rider.as_deref(), Some("Vincent · regular"));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_rider_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) = crate::history_store::HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store::HistoryStore::open_lazy(dir.clone()).unwrap();
        assert_eq!(
            load.entries[0].meta.rider.as_deref(),
            Some("Vincent · regular")
        );
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(
            reloaded.entries()[0].rider.as_deref(),
            Some("Vincent · regular")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn finished_sessions_keep_their_race_time_through_the_store() {
        let mut history = RecordingHistory::new(8);
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 3;
        let stamp = FinishStamp {
            cs: 5334,
            exact: true,
        };
        assert!(history.push_completed_session(
            RecordingSnapshot::from_state(&state),
            finished_session_label(stamp),
            0,
            3,
            Some(stamp)
        ));
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "Recorded 0:00.03".to_string(),
            0,
            3,
            None
        ));
        assert_eq!(history.entries()[0].finish_time_cs, Some(5334));
        assert!(history.entries()[0].finish_time_exact);
        assert_eq!(history.entries()[0].label, "Finish 0:53.34");
        assert_eq!(
            history.entries()[1].finish_time_cs,
            None,
            "a hand-stopped take is not a finish"
        );

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.finish_time_cs, Some(5334));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_finish_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) = crate::history_store::HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store::HistoryStore::open_lazy(dir.clone()).unwrap();
        assert_eq!(load.entries[0].meta.finish_time_cs, Some(5334));
        assert_eq!(load.entries[1].meta.finish_time_cs, None);
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(reloaded.entries()[0].finish_time_cs, Some(5334));
        assert!(reloaded.entries()[0].finish_time_exact);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn history_entries_carry_the_physics_stamp_through_the_store() {
        let mut history = RecordingHistory::new(8);
        history.set_live_physics(Some("OpenGL/53-bit".to_string()));
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 3;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take".to_string(),
            0,
            3,
            None
        ));
        assert_eq!(
            history.entries()[0].physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        // An unknown live mode (DLL not attached / not sampled yet) never
        // erases a known one.
        history.set_live_physics(None);
        assert_eq!(history.live_physics(), Some("OpenGL/53-bit"));

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.physics.as_deref(), Some("OpenGL/53-bit"));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_physics_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) = crate::history_store::HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store::HistoryStore::open_lazy(dir.clone()).unwrap();
        assert_eq!(
            load.entries[0].meta.physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(
            reloaded.entries()[0].physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn backfill_levels_tags_unambiguous_spawns_idempotently() {
        let mut h = RecordingHistory::new(8);
        // FE-spawn recording (unique cluster → taggable).
        let mut fe = state_with_ticks(10);
        fe.rec_coords[0] = [519.2, -1401.6, 53.6];
        assert!(h.push_snapshot(&fe, "fe run"));
        // Alpine-spawn recording (shared cluster → must stay untagged).
        let mut alpine = state_with_ticks(10);
        alpine.rec_coords[0] = [642.9, -842.2, 96.5];
        assert!(h.push_snapshot(&alpine, "alpine run"));
        // Simulate pre-tagging entries.
        for e in &mut h.entries {
            e.level = None;
        }

        let rev = h.revision();
        assert_eq!(
            h.backfill_levels(crate::start_line::level_code_from_spawn),
            1
        );
        assert_eq!(h.entries()[0].level.as_deref(), Some("FE"));
        assert_eq!(h.entries()[1].level, None);
        assert!(h.revision() > rev, "tagging must mark history dirty");

        // Idempotent: second pass tags nothing, revision untouched.
        let rev = h.revision();
        assert_eq!(
            h.backfill_levels(crate::start_line::level_code_from_spawn),
            0
        );
        assert_eq!(h.revision(), rev);
    }

    /// Quitting FE and loading Forest Medium goes known(FE) -> unknown -> known(FM).
    /// During the unknown window the old level must not be asserted, or the
    /// panel filters to FE and entries pushed mid-load are stamped FE.
    #[test]
    fn level_change_does_not_carry_the_old_level_into_the_new_one() {
        let mut h = RecordingHistory::new(8);
        h.set_live_level(Some("FE"));
        assert_eq!(h.live_level(), Some("FE"));

        // Player quits to the menu and starts loading Forest Medium; the DLL
        // reports an unknown level until the new track is identified.
        h.enter_resolving();

        assert_ne!(
            h.live_level(),
            Some("FE"),
            "after the level changed underneath us, FE is no longer a fact — \
             continuing to assert it filters the panel to the wrong track and \
             mis-stamps anything pushed during the load"
        );
        assert!(
            h.level_is_resolving(),
            "the transition window should be an explicit 'resolving' state, not \
             silently rendered as the previous level"
        );

        // Anything pushed while resolving must NOT be stamped with the old level.
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "mid-load");
        assert_eq!(
            h.entries().last().and_then(|e| e.level.as_deref()),
            None,
            "an entry pushed mid-transition must be untagged, never tagged FE"
        );

        // Scan resolves: now we genuinely know.
        h.set_live_level(Some("FM"));
        assert_eq!(h.live_level(), Some("FM"));
        assert!(!h.level_is_resolving());
    }

    /// Clicking a history row must not reach across tracks. `restore_index`
    /// takes an index from the UI, so it enforces the per-level rule itself
    /// rather than relying on the panel's row filter.
    #[test]
    fn restore_index_does_not_cross_levels() {
        let mut h = RecordingHistory::new(8);

        h.set_live_level(Some("FE"));
        let mut fe = RecordingSnapshot::new_empty();
        fe.recorded_count = 10;
        h.push_snapshot_data(fe, "fe-run");

        h.set_live_level(Some("FM"));
        let mut fm = RecordingSnapshot::new_empty();
        fm.recorded_count = 20;
        h.push_snapshot_data(fm, "fm-run");

        // Index 0 is the FE entry; we are on FM.
        assert_eq!(h.entries()[0].level.as_deref(), Some("FE"));
        assert!(
            h.restore_index(0).is_none(),
            "restoring another track's entry by index must be refused, not just \
             hidden from the list"
        );
        // ...and the refusal must not have moved the selection.
        assert_ne!(h.current_index(), Some(0));

        // The FM entry restores fine.
        assert!(h.restore_index(1).is_some());

        // While resolving nothing qualifies: the track is unknown.
        h.enter_resolving();
        assert!(
            h.restore_index(1).is_none(),
            "must refuse every entry while the track is unknown"
        );
    }

    /// A recovered recording must not show on every track. Recovered
    /// checkpoints are pushed at startup, before the live level is known, so
    /// the level comes from the checkpoint instead; pinning must not exempt
    /// the entry from the per-level filter.
    #[test]
    fn a_recovered_entry_is_tagged_and_does_not_leak_across_tracks() {
        let mut h = RecordingHistory::new(8);

        // Startup ordering: the recovery push happens before any level sync.
        h.set_live_level(None);
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "⟲");
        let id = h.entries().last().unwrap().entry_id;
        assert_eq!(
            h.entries().last().unwrap().level,
            None,
            "precondition: the push itself cannot know the level"
        );

        // The checkpoint knew: it was recorded on FE.
        h.set_level(id, Some("FE".to_string()));
        h.set_pinned(id, true);

        // On Forest Medium it must be gone — pinning must not exempt it.
        h.set_live_level(Some("FM"));
        let idx = h.entries().iter().position(|e| e.entry_id == id).unwrap();
        assert!(
            !h.entry_on_current_level(idx),
            "a recovered FE recording must not show (or restore) on FM, pinned or not"
        );
        assert!(h.restore_index(idx).is_none());

        // Back on FE it is available again.
        h.set_live_level(Some("FE"));
        assert!(h.entry_on_current_level(idx));
        assert!(h.restore_index(idx).is_some());
    }

    /// A checkpoint written before the level was carried still loads, and stays
    /// visible everywhere — the honest state for one we genuinely cannot place.
    #[test]
    fn a_recovered_entry_with_no_recorded_level_stays_untagged() {
        let mut h = RecordingHistory::new(8);
        h.set_live_level(None);
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "⟲");
        let id = h.entries().last().unwrap().entry_id;

        assert!(!h.set_level(id, None), "no-op when there is nothing to set");
        h.set_live_level(Some("VH"));
        let idx = h.entries().iter().position(|e| e.entry_id == id).unwrap();
        assert!(h.entry_on_current_level(idx));
    }

    /// Untagged entries stay restorable on every track, deliberately (see
    /// `entry_on_current_level`), except while the track is resolving.
    #[test]
    fn untagged_entries_remain_restorable() {
        let mut h = RecordingHistory::new(8);

        // Pushed with no live level => untagged.
        h.set_live_level(None);
        let mut legacy = RecordingSnapshot::new_empty();
        legacy.recorded_count = 10;
        h.push_snapshot_data(legacy, "legacy-run");
        assert_eq!(h.entries()[0].level, None);

        h.set_live_level(Some("VH"));
        assert!(
            h.restore_index(0).is_some(),
            "an untagged entry must stay reachable on any track"
        );

        // But not while we do not know the track at all.
        h.enter_resolving();
        assert!(h.restore_index(0).is_none());
    }

    /// Ctrl+Z must not reach across tracks: undo/redo walk the unfiltered
    /// entry list, not the rows the panel shows.
    #[test]
    fn undo_does_not_cross_levels() {
        let mut h = RecordingHistory::new(8);

        h.set_live_level(Some("FE"));
        let mut fe = RecordingSnapshot::new_empty();
        fe.recorded_count = 10;
        h.push_snapshot_data(fe, "fe-run");

        h.set_live_level(Some("FM"));
        let mut fm = RecordingSnapshot::new_empty();
        fm.recorded_count = 20;
        h.push_snapshot_data(fm, "fm-run");

        // On FM with only one FM entry, there is nothing to undo TO. The FE
        // entry is on another track and must not be offered.
        assert_eq!(
            h.undo_depth(),
            0,
            "an FE entry must not be undo-reachable from FM"
        );
        assert!(
            h.undo().is_none(),
            "Ctrl+Z must not restore another track's recording"
        );

        // Back on FE it is reachable again.
        h.set_live_level(Some("FE"));
        assert_eq!(h.undo_depth(), 1);
    }

    #[test]
    fn set_live_level_is_sticky_across_unknown() {
        let mut h = RecordingHistory::new(8);
        assert_eq!(h.live_level(), None);
        h.set_live_level(Some("FE"));
        assert_eq!(h.live_level(), Some("FE"));
        // Menu / unknown must NOT clear the last-known level.
        h.set_live_level(None);
        assert_eq!(h.live_level(), Some("FE"));
        h.set_live_level(Some("AM"));
        assert_eq!(h.live_level(), Some("AM"));
    }

    #[test]
    fn entry_ids_unique_and_monotonic() {
        let mut h = RecordingHistory::new(16);
        for i in 0..5 {
            assert!(h.push_snapshot(&state_with_ticks(10 + i), format!("S{}", i)));
        }
        let ids: Vec<u64> = h.entries().iter().map(|e| e.entry_id).collect();
        let mut uniq = ids.clone();
        uniq.sort();
        uniq.dedup();
        assert_eq!(uniq.len(), ids.len(), "ids unique");
        assert!(ids.windows(2).all(|w| w[0] < w[1]), "ids monotonic");
    }

    #[test]
    fn soft_cap_evicts_oldest_unpinned() {
        let mut h = RecordingHistory::new(3);
        for i in 0..5 {
            h.push_snapshot(&state_with_ticks(10 + i), format!("S{}", i));
        }
        assert_eq!(h.len(), 3, "capped to 3 unpinned");
        let labels: Vec<&str> = h.entries().iter().map(|e| e.label.as_str()).collect();
        assert_eq!(labels, vec!["S2", "S3", "S4"], "oldest unpinned evicted");
    }

    #[test]
    fn pinned_survive_eviction() {
        let mut h = RecordingHistory::new(3);
        h.push_snapshot(&state_with_ticks(10), "A");
        let a_id = h.entries()[0].entry_id;
        assert!(h.set_pinned(a_id, true));
        for i in 0..5 {
            h.push_snapshot(&state_with_ticks(20 + i), format!("U{}", i));
        }
        assert!(
            h.entries().iter().any(|e| e.entry_id == a_id),
            "pinned A survived"
        );
        assert_eq!(
            h.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "unpinned still capped"
        );
    }

    #[test]
    fn current_entry_never_evicted() {
        let mut h = RecordingHistory::new(3);
        for lbl in ["A", "B", "C"] {
            h.push_snapshot(&state_with_ticks(10), lbl);
        }
        h.restore_index(0); // select the oldest
        let a_id = h.entries()[0].entry_id;
        h.capacity = 2; // tighten below the count
        h.enforce_capacity();
        assert!(
            h.entries().iter().any(|e| e.entry_id == a_id),
            "current (oldest) entry not evicted"
        );
    }

    #[test]
    fn set_capacity_trims_immediately() {
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        assert_eq!(h.len(), 6);
        h.set_capacity(3);
        assert_eq!(
            h.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "lowering the cap trims unpinned immediately"
        );
    }

    #[test]
    fn ids_never_reused_after_eviction_and_reload() {
        use crate::history_store::HistoryStore;
        let dir = std::env::temp_dir().join(format!(
            "ssb_idreuse_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(3);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        // cap 3 + 6 pushes ⇒ ids 1,2,3 were front-evicted; 4,5,6 remain.
        let evicted = [1u64, 2, 3];
        let next_before = h.next_entry_id();

        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), h.current_entry_id(), next_before)
            .unwrap();
        let (_s, res) = HistoryStore::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(3);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        // A new push after reload must get a FRESH id, never an evicted one —
        // even though the evicted ids are now "gaps" below the surviving set.
        h2.push_snapshot(&state_with_ticks(5), "new");
        let new_id = h2
            .entries()
            .iter()
            .find(|e| e.label == "new")
            .unwrap()
            .entry_id;
        assert!(
            new_id >= next_before,
            "id {} reused (next was {})",
            new_id,
            next_before
        );
        assert!(
            !evicted.contains(&new_id),
            "id {} reused an EVICTED id",
            new_id
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pinned_and_current_survive_lowered_cap_on_load() {
        use crate::history_store::HistoryStore;
        let dir = std::env::temp_dir().join(format!(
            "ssb_pinload_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        let pin_id = h.entries()[0].entry_id; // oldest — would normally evict first
        assert!(h.set_pinned(pin_id, true));
        h.restore_index(1);
        let cur_id = h.current_entry_id();

        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur_id, h.next_entry_id())
            .unwrap();
        let (_s, res) = HistoryStore::open_eager(dir.clone()).unwrap();
        // Load into a history with a cap FAR below the entry count.
        let mut h2 = RecordingHistory::new(2);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        assert!(
            h2.entries()
                .iter()
                .any(|e| e.entry_id == pin_id && e.pinned),
            "pinned entry must survive a lowered cap on load"
        );
        assert_eq!(
            h2.current_entry_id(),
            cur_id,
            "current entry must survive a lowered cap on load"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn all_pinned_over_cap_keeps_all() {
        let mut h = RecordingHistory::new(4);
        for i in 0..4 {
            h.push_snapshot(&state_with_ticks(10), format!("P{}", i));
        }
        let ids: Vec<u64> = h.entries().iter().map(|e| e.entry_id).collect();
        for id in &ids {
            h.set_pinned(*id, true);
        }
        h.capacity = 2;
        h.enforce_capacity();
        assert_eq!(h.len(), 4, "all-pinned kept despite cap 2");
    }

    #[test]
    fn rename_and_pin_bump_revision_only_on_change() {
        let mut h = RecordingHistory::new(8);
        h.push_snapshot(&state_with_ticks(5), "A");
        let id = h.entries()[0].entry_id;
        let r0 = h.revision();
        assert!(h.rename(id, "A2"));
        assert!(h.revision() > r0);
        let r1 = h.revision();
        assert!(h.set_pinned(id, true));
        assert!(h.revision() > r1);
        // no-op rename + pin must NOT bump
        let r2 = h.revision();
        h.rename(id, "A2");
        h.set_pinned(id, true);
        assert_eq!(h.revision(), r2, "no-op meta change doesn't bump");
    }

    #[test]
    fn bridge_roundtrips_through_v2_store() {
        use crate::history_store::HistoryStore;
        let dir = std::env::temp_dir().join(format!(
            "ssb_bridge_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));

        let mut h = RecordingHistory::new(16);
        h.push_snapshot(&state_with_ticks(5), "A");
        h.push_snapshot(&state_with_ticks(7), "B");
        h.push_save_marker(&state_with_ticks(7), Path::new("run.tasrec"), None);
        let b_id = h.entries()[1].entry_id;
        h.set_pinned(b_id, true);
        h.rename(b_id, "B renamed");
        h.restore_index(1);
        let cur = h.current_entry_id();

        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s2, res) = HistoryStore::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        assert_eq!(h2.len(), h.len());
        let b2 = h2.entries().iter().find(|e| e.entry_id == b_id).unwrap();
        assert_eq!(b2.custom_name.as_deref(), Some("B renamed"));
        assert!(b2.pinned);
        assert_eq!(h2.current_entry_id(), cur);
        let max_loaded_id = h2.entries().iter().map(|e| e.entry_id).max().unwrap();
        assert!(
            h2.next_entry_id() > max_loaded_id,
            "next id ({}) must be past EVERY loaded id ({}), incl. the save marker",
            h2.next_entry_id(),
            max_loaded_id
        );
        // snapshot content survives the round-trip
        let a2 = h2.entries().iter().find(|e| e.label == "A").unwrap();
        assert!(a2.can_restore());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn custom_name_roundtrips_and_clears() {
        use crate::history_store::HistoryStore;
        let dir = std::env::temp_dir().join(format!(
            "ssb_name_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(16);
        h.push_snapshot(&state_with_ticks(5), "Recorded 0:05");
        let id = h.entries()[0].entry_id;
        assert!(h.rename(id, "  my best run  ")); // trims whitespace

        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(
                &h.to_stored_entries(),
                h.current_entry_id(),
                h.next_entry_id(),
            )
            .unwrap();
        let (_s, res) = HistoryStore::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        let e = h2.entries().iter().find(|e| e.entry_id == id).unwrap();
        assert_eq!(e.custom_name.as_deref(), Some("my best run"));
        assert_eq!(
            e.label, "Recorded 0:05",
            "auto label preserved alongside name"
        );

        // Blank rename clears the custom name.
        assert!(h2.rename(id, "   "));
        assert_eq!(
            h2.entries()
                .iter()
                .find(|e| e.entry_id == id)
                .unwrap()
                .custom_name,
            None
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn push_save_marker_bumps_revision() {
        let mut h = RecordingHistory::new(8);
        h.push_snapshot(&state_with_ticks(5), "A");
        let r = h.revision();
        h.push_save_marker(&state_with_ticks(5), Path::new("x.tasrec"), None);
        assert!(h.revision() > r, "save marker must bump revision");
    }

    #[test]
    fn unpin_triggers_eviction() {
        let mut h = RecordingHistory::new(2);
        h.push_snapshot(&state_with_ticks(5), "A");
        let a = h.entries()[0].entry_id;
        h.set_pinned(a, true);
        h.push_snapshot(&state_with_ticks(5), "B");
        h.push_snapshot(&state_with_ticks(5), "C"); // current=C; A pinned + B,C unpinned
        assert_eq!(h.len(), 3);
        h.set_pinned(a, false); // now unpinned A,B,C = 3 > cap 2 -> evict oldest (A)
        assert!(
            !h.entries().iter().any(|e| e.entry_id == a),
            "unpinned-over-cap A evicted"
        );
        assert_eq!(h.entries().iter().filter(|e| !e.pinned).count(), 2);
    }

    #[test]
    fn apply_loaded_lowered_cap_trims() {
        use crate::history_store::HistoryStore;
        let dir = std::env::temp_dir().join(format!(
            "ssb_captrim_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        let cur = h.current_entry_id();
        let (mut store, _) = HistoryStore::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s, res) = HistoryStore::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(3); // smaller cap
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);
        assert_eq!(
            h2.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "lowered cap trims unpinned on load"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn history_undo_redo_full_cycle_restores_correct_data() {
        let mut history = RecordingHistory::new(8);
        let s1 = state_with_ticks(5);
        let s2 = state_with_ticks(15);
        let s3 = state_with_ticks(25);

        assert!(history.push_snapshot(&s1, "5 ticks"));
        assert!(history.push_snapshot(&s2, "15 ticks"));
        assert!(history.push_snapshot(&s3, "25 ticks"));

        // Undo to 15
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 15);

        // Undo to 5
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 5);

        // Can't undo further
        assert!(history.undo().is_none());
        assert_eq!(history.undo_depth(), 0);

        // Redo back to 15
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 15);

        // Redo back to 25
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 25);

        // Can't redo further
        assert!(history.redo().is_none());
        assert_eq!(history.redo_depth(), 0);
    }

    #[test]
    fn history_save_marker_after_undo_does_not_shift_selection() {
        let mut history = RecordingHistory::new(16);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);

        assert!(history.push_snapshot(&s1, "A"));
        assert!(history.push_snapshot(&s2, "B"));
        assert!(history.push_snapshot(&s3, "C"));

        // Undo to B
        history.undo().unwrap();
        assert_eq!(history.current_index(), Some(1));

        // Save marker should not change current_index
        history.push_save_marker(&s2, Path::new("test.tasrec"), None);
        assert_eq!(history.current_index(), Some(1)); // still on B
        assert_eq!(history.len(), 4); // A, B, SaveMarker, C
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert!(!history.entries()[2].can_restore());
    }

    #[test]
    fn history_capacity_eviction_under_undo_keeps_valid_cursor() {
        let mut history = RecordingHistory::new(3);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);
        let s4 = state_with_ticks(40);

        assert!(history.push_snapshot(&s1, "A"));
        assert!(history.push_snapshot(&s2, "B"));
        assert!(history.push_snapshot(&s3, "C"));
        assert_eq!(history.len(), 3);
        assert_eq!(history.current_index(), Some(2)); // on C

        // Undo to B
        history.undo().unwrap();
        assert_eq!(history.current_index(), Some(1)); // on B

        // Push D — eviction should happen (capacity=3, will have 4 before eviction)
        assert!(history.push_snapshot(&s4, "D"));
        assert_eq!(history.len(), 3); // A evicted
                                      // current_index should be valid and point to D
        let idx = history.current_index().unwrap();
        assert_eq!(history.entries()[idx].label, "D");
    }

    #[test]
    fn history_zero_recorded_count_is_rejected() {
        let mut history = RecordingHistory::new(8);
        let empty = tas_shared::zeroed_boxed(); // recorded_count = 0
        assert!(!history.push_snapshot(&empty, "Empty"));
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn history_restore_index_validates_bounds() {
        let mut history = RecordingHistory::new(4);
        let s1 = state_with_ticks(10);
        assert!(history.push_snapshot(&s1, "A"));

        assert!(history.restore_index(99).is_none()); // out of bounds
        assert_eq!(history.current_index(), Some(0)); // unchanged
    }

    #[test]
    fn history_mixed_workflow_rec_save_undo_load_continue() {
        // Simulates a real user workflow:
        // 1. Record 10 ticks
        // 2. Record 20 ticks
        // 3. Save file
        // 4. Undo to 10 ticks
        // 5. Record 15 ticks (continue)
        // 6. Load a file
        // All entries should be preserved.
        let mut history = RecordingHistory::new(32);
        let s10 = state_with_ticks(10);
        let s20 = state_with_ticks(20);
        let s15 = state_with_ticks(15);
        let s_loaded = state_with_ticks(50);

        // Step 1-2: Two recording sessions
        assert!(history.push_snapshot(&s10, "Recorded 0:00.10"));
        assert!(history.push_snapshot(&s20, "Recorded 0:00.20"));

        // Step 3: Save
        history.push_save_marker(&s20, Path::new("run.tasrec"), None);

        // Step 4: Undo to 10 ticks
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 10);

        // Step 5: Continue from undo point — appends, doesn't truncate
        assert!(history.push_snapshot(&s15, "Continued from 0:00.10, total 0:00.15"));

        // Step 6: Load a file
        assert!(history.push_loaded_snapshot(&s_loaded, Path::new("other.tasrec"), None));

        // Trace: [s10, s20] → save marker at idx 2 → [s10, s20, SaveMarker]
        // undo → current=0 → push s15 → [s10, s20, SaveMarker, s15] current=3
        // push loaded → [s10, s20, SaveMarker, s15, loaded] current=4
        assert_eq!(history.len(), 5);
        assert_eq!(history.entries()[0].label, "Recorded 0:00.10");
        assert_eq!(history.entries()[1].label, "Recorded 0:00.20");
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert_eq!(
            history.entries()[3].label,
            "Continued from 0:00.10, total 0:00.15"
        );
        assert!(history.entries()[4].label.contains("other.tasrec"));
        assert_eq!(history.entries()[4].kind, HistoryEntryKind::LoadSnapshot);

        // Can undo all the way back: s15(3), s20(1), s10(0) = 3 restorable before current(4)
        assert_eq!(history.undo_depth(), 3);
    }

    #[test]
    fn history_load_snapshot_is_undoable() {
        let mut history = RecordingHistory::new(8);
        let s1 = state_with_ticks(10);
        let s_loaded = state_with_ticks(50);

        assert!(history.push_snapshot(&s1, "Recording"));
        assert!(history.push_loaded_snapshot(&s_loaded, Path::new("loaded.tasrec"), None));

        // Should be able to undo back to the recording
        assert!(history.undo_depth() > 0);
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 10);

        // And redo back to the loaded file
        assert!(history.redo_depth() > 0);
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 50);
    }
}
