## Why

`SSB Inspect` already has a history panel, but it currently captures pre-action snapshots such as `Before REC` and `Before CONT` instead of the completed recording sessions users actually care about. It also writes history metadata to disk without enough recording state to recover a just-finished run after a game crash.

## What Changes

- Change history tracking so completed `REC` and `CONT` sessions add entries when recording stops, using labels that describe what was captured instead of when the action was armed.
- Suppress history entries for `PLAY` -> `STOP` cycles that do not modify the recording.
- Format session history labels with recorded durations derived from the TAS tick rate of 100 frames per second, including both segment length and total length for continues.
- Extend history autosave so the latest recoverable recording state survives a game crash and can be restored on the next `SSB Inspect` launch.

## Capabilities

### New Capabilities
- `recording-session-history`: Track completed recording sessions as user-readable history entries and persist recoverable recording state for crash recovery.

### Modified Capabilities

## Impact

- Affected code: `TAS/tas_ui/src/main.rs`, `TAS/tas_ui/src/recording.rs`, `TAS/tas_ui/src/history_store.rs`, `TAS/tas_ui/src/panels/history.rs`
- Systems: transport mode transitions, history labeling, history persistence, crash recovery flow
