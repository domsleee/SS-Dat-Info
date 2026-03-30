## Why

`SSB Inspect` currently exposes edit history through disconnected UI pieces: `Undo` lives in the transport bar, `Redo` is being added separately, and `Save` / `Load` only appear in the file menu and log output. That makes TAS editing harder to reason about because users cannot see the recording session as a single, navigable history the way they can in Photoshop.

## What Changes

- Add a dedicated History panel to `SSB Inspect` that presents recording actions as a Photoshop-style vertical history stack with a visible current selection.
- Replace the current undo-only snapshot stack with a history model that supports undo, redo, and selecting prior history states from the same source of truth.
- Record `Save to file` and `Load from file` actions in the History panel so file operations appear alongside edit actions instead of only in the bottom log.
- Add panel visibility and persistence controls so the History panel behaves like the existing optional `SSB Inspect` panels.

## Capabilities

### New Capabilities
- `ssb-inspect-history`: Present TAS recording history as a Photoshop-style panel that tracks edit states and file actions.

### Modified Capabilities

## Impact

- Affected code: `TAS/tas_ui/src/main.rs`, `TAS/tas_ui/src/panels/transport.rs`, `TAS/tas_ui/src/recording.rs`, `TAS/tas_ui/src/settings.rs`
- Likely new code: `TAS/tas_ui/src/panels/history.rs` and related history state types/tests
- Systems: egui panel layout, recording snapshot management, file save/load flows, keyboard shortcuts for undo/redo
