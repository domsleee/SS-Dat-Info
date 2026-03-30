## Context

`SSB Inspect` currently keeps edit-state behavior in a few separate places:

- `TAS/tas_ui/src/recording.rs` owns `RecordingSnapshot`, `UndoRing`, segment tracking, and the save/load dialog helpers.
- `TAS/tas_ui/src/main.rs` owns application state, keyboard shortcuts, and file-menu wiring.
- `TAS/tas_ui/src/settings.rs` persists optional panel visibility and playback defaults.

The current implementation supports a bounded undo stack, but it does not expose a visible history model, does not retain redo state, and does not surface file operations in the same editing flow. The existing tests are already organized around module-local state and action checks: `recording.rs` tests snapshot/file behavior, `main.rs` tests shortcut routing and app flags, and `tas_test` covers larger runtime flows. The history feature should extend that pattern rather than introduce brittle GUI snapshot tests.

## Goals / Non-Goals

**Goals:**

- Introduce a single bounded history model that drives undo, redo, and direct history selection.
- Add a Photoshop-style `History` panel that shows ordered recording actions with a visible current selection.
- Show successful `Save to file` and `Load from file` operations in the same history list as edit actions.
- Persist `History` panel visibility like the existing optional panels.
- Add local automated tests for history cursor behavior, markers, shortcut wiring, and settings persistence.

**Non-Goals:**

- Reproduce Photoshop's UI pixel-for-pixel or add a full docking/layout system.
- Keep an unbounded history of recording states.
- Replace the existing bottom log panel for diagnostics and error output.
- Expand hardware-driven `tas_test` flows unless the feature introduces runtime behavior that cannot be covered with unit tests.

## Decisions

### 1. Replace the undo-only ring with a bounded recording history model

Create a new history type in `recording.rs` that supersedes `UndoRing` as the single source of truth for edit history. It should keep the current preallocated snapshot strategy for restorable recording states, but add:

- a cursor for the currently selected state
- ordered history entries with user-facing labels
- support for both restorable state entries and lightweight marker entries

Restorable entries own or reference a captured `RecordingSnapshot`. Marker entries represent non-mutating actions such as `Save to file` and point at the currently selected snapshot instead of duplicating recording data.

Rationale:

- A Photoshop-style panel needs a stable current pointer, not just a pop-only stack.
- Undo and redo become cursor moves over the same structure instead of separate mechanisms.
- Save markers can appear in the panel without multiplying memory usage.

Alternatives considered:

- Keep `UndoRing` and add a separate event log panel.
  Rejected because it would not support direct state selection or a single visible history cursor.
- Store an unbounded `Vec` of full snapshots.
  Rejected because recording snapshots are large and the existing code intentionally bounds memory.

### 2. Add a dedicated optional `History` side panel

Implement the UI as a new `panels/history.rs` module and expose it through the `View` menu plus persisted settings, matching the existing optional-panel pattern. Place it in a side panel so the history list reads like Photoshop's vertical history stack without overloading the bottom log.

The panel should:

- show an empty-state message before any history exists
- render entries in chronological order with the current entry highlighted
- allow selecting a prior restorable entry
- keep filename/path detail compact in the row label and/or tooltip

Rationale:

- A dedicated side panel is closer to the user's requested mental model than the current log panel.
- It avoids mixing operational diagnostics with edit history.

Alternatives considered:

- Reuse the bottom log panel.
  Rejected because log output is noisy, non-stateful, and not suitable for direct selection.
- Add history as another collapsing section in the central area.
  Rejected because it competes with the timeline and drift panels instead of behaving like a persistent editing tool.

### 3. Treat save and load differently in the history model

Successful `Save to file` should append a marker entry that references the current snapshot and includes file context in the label. Successful `Load from file` should create a new current state entry from the loaded recording and add a corresponding load label.

This keeps the semantics coherent:

- save is visible in history but does not change the recording state
- load both changes the recording state and establishes a new current point in history

If a user selects a save marker, the panel should resolve it to the referenced snapshot so selection remains intuitive.

Alternatives considered:

- Ignore save/load in history and keep them log-only.
  Rejected because it fails the requested Photoshop-style workflow.
- Snapshot on save as if it were a distinct edit state.
  Rejected because it duplicates an unchanged recording for no behavioral gain.

### 4. Fold redo into the same model, even if its UI lands separately

This change should not build a new history panel on top of a temporary undo-only structure. Whether redo lands in a separate in-flight change or inside this one, both directions should use the same history cursor and entry list.

That means:

- transport actions should expose redo through the shared history model
- keyboard shortcut tests in `main.rs` should validate redo once the action exists
- direct panel selection should restore the same states undo/redo traverses

Rationale:

- Shipping history first and consolidating later would create avoidable rework and state-sync bugs.

### 5. Follow the existing module-local testing strategy

Add tests where the current code already tests similar behavior:

- `recording.rs`: history push/undo/redo/select behavior, branching after undo, save markers, load entries, capacity eviction
- `main.rs`: redo shortcut routing, new panel toggle defaults/persistence, and any history-related action mapping helpers
- `settings.rs`: persisted `show_history` serialization if helpers are added there
- `panels/history.rs`: pure helper logic only, if label formatting or selection mapping is extracted

Keep `tas_test` unchanged unless the implementation adds a runtime contract that cannot be validated through module tests.

## Risks / Trade-offs

- [Large snapshot memory footprint] -> Mitigation: keep history bounded and represent save events as markers instead of duplicate snapshots.
- [Redo work lands concurrently] -> Mitigation: define the shared history API first and route both undo and redo through it.
- [Marker entries make selection semantics unclear] -> Mitigation: make marker selection resolve to the associated snapshot and keep labels explicit (`Save`, `Load`, `Undo`, `Redo`).
- [Another side panel reduces workspace width] -> Mitigation: keep the panel optional, collapsible, and persisted like existing panels.
- [Cancelled or failed file dialogs pollute history] -> Mitigation: only append history entries on successful save/load; keep failures in the bottom log only.

## Migration Plan

- Add a new persisted setting for `show_history`.
- Replace `UndoRing` usage in `TasApp` with the new bounded history model.
- Route transport actions, file-menu actions, and shortcut handlers through shared history entry helpers.
- If separate redo code already exists by implementation time, rebase it onto the shared history model rather than keeping parallel state.

No on-disk migration is required because history remains session-local.

## Open Questions

- Which redo shortcut should be canonical on Windows: `Ctrl+Y`, `Ctrl+Shift+Z`, or both?
- What bounded history depth best balances memory and usefulness for TAS editing?
- Should the panel rows show the full path, file name only, or file name with the full path in hover text?
