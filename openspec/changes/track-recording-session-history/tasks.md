## 1. Session History Entries

- [x] 1.1 Add explicit `REC` / `CONT` session tracking in `TAS/tas_ui/src/main.rs` so visible history entries are created on completed recording stop events instead of on `Before REC` / `Before CONT` arm actions
- [x] 1.2 Add 100 Hz duration formatting and completed-session label helpers in `TAS/tas_ui/src/recording.rs`, and use them to create `Recorded ...` and `Continued ..., total ...` history entries while skipping `PLAY` -> `STOP` and zero-length stops
- [x] 1.3 Extend `main.rs` and `recording.rs` tests to cover fresh recording, continue recording, play-stop no-op behavior, and session label formatting

## 2. Crash Recovery Checkpoints

- [x] 2.1 Implement a recovery checkpoint store that persists the latest recoverable recording state, segment metadata, and session label context with debounced atomic writes while `REC` / `CONT` is making progress
- [x] 2.2 Wire startup detection and restore flow for pending recovery checkpoints so `SSB Inspect` can expose and rehydrate the latest persisted recording after an unclean shutdown
- [x] 2.3 Add automated tests for recovery checkpoint writes, unchanged-write skipping, atomic replacement behavior, and recovery metadata loading

## 3. Verification

- [x] 3.1 Run `cargo test -p tas_ui` and fix regressions introduced by the session-history and recovery changes
- [ ] 3.2 Perform a manual `SSB Inspect` smoke test covering `REC` -> `STOP`, `CONT` -> `STOP`, `PLAY` -> `STOP`, and crash-recovery restore behavior
