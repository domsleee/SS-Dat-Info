# History Store v2 — Persistent, Named, Pinnable Undo History

## Problem
The undo history (`RecordingHistory`) is persisted as a single `history.json`
rewritten on **every** undo-stack change. It grew to **134 MB / 191 entries**
(each a full recording snapshot: `input_log` + `rec_coords`) and froze the UI
~3.6 s per change. The UI freeze is already fixed (background writer, commit
`c7a2501`); this plan fixes the remaining problems: file size, write
amplification, unbounded cross-session growth — and adds **name + pin** and a
revised recovery flow.

## Confirmed requirement
User reopens the app and undoes into **previous sessions'** edits → the deep
stack must persist across sessions. So: keep persistence, store it efficiently.

## Decision: file-per-entry, id-named (NO content hashing)
Each entry is an immutable snapshot; max a few hundred; load-all at startup;
append-mostly writes. A DB (SQLite/redb) is overkill at this scale (codex).
Content-addressing (hashing) was rejected: our snapshots are essentially always
unique, so dedup buys ~nothing, and it forced a canonical deterministic blob
format + refcounted GC. We already need a stable id for the cursor, so name
blobs by that id.

---

## 1. Identity model (Phase 0 — define before any storage code)
- **`entry_id: u64`** — stable, monotonic, assigned on creation, **never
  reused** (evicted ids must never come back, or a pinned old id + reused id
  collide). Gaps are fine.
- **`next_entry_id` is authoritative** and persisted in the manifest. On load:
  `next_entry_id = max(stored_next_entry_id, max_manifest_id + 1,
  max_disk_blob_id + 1)`, with **checked arithmetic + hard error on overflow**.
  (Don't derive it from `max(id)+1` alone — pins can leave only old low ids.)
- **Blob** = `<entry_id>.tasrec`, written **exactly once**, immutable, via the
  existing temp+rename writer — **but blob creation must NOT overwrite** an
  existing `<id>.tasrec`. If a stray `K.tasrec` exists for the id we're about to
  allocate, bump past it or quarantine it first. (`std::fs::rename` replaces
  targets — correct for the manifest, wrong for immutable blob creation.)
- Content is irrelevant to identity, so the `timestamp: now` inside `.tasrec`
  is harmless (no canonical-format requirement).
- **Save markers / no-snapshot entries carry NO blob** (`blob: None`); they
  must not retain an `id.tasrec` just because their id is in the manifest.
- **Cursor** = `current_entry_id: Option<u64>` (NOT a positional index).
  Resolved on load by **manifest row order** (save markers can sit mid-stack
  after undo, so id order ≠ visual order).
- **GC** = on load, delete blob files whose id ∉ manifest. No refcounting
  (ids are never shared).
- **Integrity** = per-entry `{ size, checksum }` (crc32 over the exact bytes
  written, pure-Rust `crc32fast`) in the manifest; recompute on load; mismatch
  ⇒ delete/rename the bad blob + warn (no elaborate quarantine UI for v1).
  crc32 is sufficient for the local accidental-corruption threat model;
  BLAKE3 later only if tamper-resistance is ever wanted.

## 2. On-disk layout — `~/.ssb-inspector/history/`
- `<entry_id>.tasrec` — one immutable blob per entry.
- `manifest.json` — small, atomic-replaced, **debounced ~250 ms**:
  ```
  { magic: "ssb-history", schema: 1, blob_format: 1, hash_algo: "crc32",
    next_entry_id: u64,
    current_entry_id: Option<u64>,
    entries: [ { entry_id, name, pinned: bool, kind, start_tick, end_tick,
                 first_moving, created_at_iso, size, checksum } ] }
  ```
  Header fields let us reject incompatible data safely.

## 3. Write path (keeps the background writer)
- **Dirty tracking** (the `(len, current_entry_id)` early-out is INVALID —
  rename/pin change neither length nor cursor): use a single `revision: u64`
  bumped on any structural/metadata/cursor change, or separate
  structural/meta/cursor dirty flags. Persist when `revision` advanced.
- On change: write any **new** entry's blob (atomic, once, non-overwriting),
  then publish the manifest (atomic). Deletes happen **after** the manifest no
  longer references the file. GC orphans on load.
- **Manifest atomic replace**: `std::fs::rename` over the existing manifest
  (temp file in the **same directory**) is sufficient — it replaces the target
  (`MoveFileExW`/`SetFileInformationByHandle`, `MOVEFILE_REPLACE_EXISTING`).
  **Drop the remove-then-rename fallback** for the manifest (that fallback has a
  missing-window). Optional `File::sync_all` on the temp before rename **only if
  power-loss durability** is wanted (process-crash safety ≠ power-loss
  durability). Crash test asserts old-or-new, never missing.
- **Cursor is navigation state**: `current_entry_id` is NOT force-synced on
  every undo/redo; persisted lazily (debounced + on clean shutdown). Crash loss
  of cursor position is explicitly accepted; resolved on next load.

## 4. Cap (user setting) + pin
- `Settings.history_cap: usize` (default 150), editable in the config panel.
- **Soft cap**: keep ALL pinned entries + the newest N **unpinned** + **never
  evict the current entry**. If pinned count ≥ cap, pins win (effective count
  can exceed cap; surface a notice rather than dropping pins).
- Eviction removes oldest **unpinned, non-current** entries; their blobs are
  deleted (after manifest publish).
- **Pin** = per-entry bool, exempt from eviction + GC.
- **Name** = per-entry editable label; rename = manifest write only (no blob).

## 5. Undo semantics
- **CONFIRMED (codex, from the existing tests):** `RecordingHistory` is
  **append-after-undo** — undoing then pushing **appends** (len grows, nothing
  truncated); save markers can be inserted mid-stack. The fuzz model mirrors
  this exactly. **No branch-truncation is introduced.**

## 6. Missing / corrupt blobs (never crash)
- Availability is **derived from disk on load**, not persisted as truth.
- A missing/corrupt entry stays **visible but inert** — behaves like a save
  marker: skipped by undo/redo, cannot become `current`. Non-blocking warning
  (log + subtle UI notice).
- Corrupt blob (checksum mismatch) ⇒ quarantine/delete the file (keep the
  manifest row) so the same corruption isn't re-hit every launch.
- If the persisted `current_entry_id` is unavailable: resolve to nearest
  previous valid entry, else nearest next, else none — and warn. Never silently
  jump to newest.

## 7. Recovery banner "Name & Pin"
- Current: `⚠ Recovery from {time}` + Restore / Discard.
- Add `name: [____]` + **Name & Pin** = persist the recovered snapshot as a
  **pinned, named** history entry **without** restoring live state (Restore
  already mutates live state).
- Works with **no game connection** (recovery already holds the snapshot).
- Clear `pending_recovery` **only after** the history write is flushed/acked
  (else a crash after Name & Pin loses both the recovery file and the new
  entry). Respect the existing "pending_recovery is sacred" guard. Apply the
  same flush-before-clear rule to **Restore** if Restore writes a history entry
  before clearing recovery.
- **Idempotency across "history flushed, recovery-clear crashed":** stamp the
  new entry with a `source_recovery_id`; on next launch, if that recovery id is
  already represented in history, don't re-add it. (Or explicitly accept +
  document a possible duplicate pinned entry.)
- History blobs **preserve segments** (`.tasrec` already carries them); history
  restore should restore segments too (consistency with recovery restore).

## 8. Migration from legacy `history.json`
- **DECIDED (codex): import the latest parseable `history.json` ONLY. No
  multi-session merge.** The current code loads the newest legacy file by mtime
  and persists the cumulative stack, and `RecordingHistory` is
  append-after-undo — so the latest file already contains the deep stack.
  Merging all sessions would duplicate entries (no clean dedup key).
- Assign `entry_id`s **in manifest row order**, set `next_entry_id` after the
  last assigned, preserve names/timestamps/kinds, `pinned` defaults false.
- **Only run migration if no valid v2 manifest exists.**
- Fallback: if the newest legacy file is corrupt, try the next-newest parseable
  one (this is *not* "merge all").

## 9. Tests (all green standalone before integration)
Model-based fuzz (10k random op sequences; disk == reference model after every
op, using the REAL undo semantics) **plus** targeted:
- round-trip · append-writes-one-blob · idempotent-persist
- all-pinned-over-cap · pinned-middle-eviction · current-entry-never-evicted
- pinned-survives-unpinned-eviction · cap-lowered-trims-correctly
- missing-blob-no-crash · only-corrupt-entry · corrupt-current-entry
- unavailable-entry-inert-in-navigation · stale-cursor-after-eviction
- crash-after-blob-before-manifest · crash-during-manifest-replace
   (old-or-new, never missing)
- JSON migration (and multi-session if we go that route)
- flush-before-clearing-recovery · Name&Pin-failure-keeps-recovery
- rename round-trip · checksum-detects-corruption

## 10. Build order
0. **Identity/blob/manifest/cursor spec + confirm real undo semantics** (this doc).
1. **Storage layer standalone + ALL tests green.** Nothing touches the app.
2. **Wire into app**: data model gains `entry_id`, `name`, `pinned`; swap
   load/persist to the new store; migrate legacy `history.json`.
3. **UI**: history panel rename + pin toggle; recovery banner Name & Pin;
   `history_cap` in config panel.
4. **Release-verify live**: 134MB→few-MB, STOP snappy, undo/redo + recovery
   intact, pin/rename work, yank a blob ⇒ warning not crash.

Each step is a reviewable commit; the live tas_ui is untouched until step 1 is
green.

## 11. Cut for v1 (codex) — keep it lean
- **Cut** multi-session merge (latest-only import).
- **Cut** `ReplaceFileW` / atomic-write crate (`std::fs::rename` suffices).
- **Cut** elaborate quarantine UI — just delete/rename a corrupt blob + a simple
  warning.
- **Keep**: manifest header, per-entry checksum, orphan GC, never-evict-current,
  soft cap, the focused fuzz + crash tests.
