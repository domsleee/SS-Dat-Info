## Context

`SSB Inspect` already has the main pieces involved in this change:

- `main.rs` tracks transport actions, mode transitions, and the existing history push points.
- `recording.rs` owns `RecordingHistory`, `RecordingSnapshot`, file save/load helpers, and `SegmentTracker`.
- `history_store.rs` persists visible history metadata to disk, but not enough recording payload to restore after a crash.

Today the visible history model is updated when `REC` or `CONT` is armed through `AutoSave("Before REC")` and `AutoSave("Before CONT")`. That captures the pre-edit state, not the completed recording session. The task in `tasks/history-track.md` instead wants history entries that reflect what the user just recorded, no new entries for `PLAY` -> `STOP`, and a way to recover recent progress after a crash.

## Goals / Non-Goals

**Goals:**

- Add history entries for completed `REC` and `CONT` sessions when recording stops and new ticks were captured.
- Generate user-facing labels from TAS time at 100 ticks per second, including both segment length and total length for continues.
- Keep `PLAY` -> `STOP` and zero-length recording attempts out of the visible history list.
- Persist a recoverable recording checkpoint outside the visible history list so recent recording progress can be restored after a crash.

**Non-Goals:**

- Redesign the existing history panel layout or undo/redo model.
- Replace the regular `Save Recording` / `Load Recording` workflow.
- Guarantee byte-for-byte recovery of every in-flight frame if a crash happens between checkpoint writes.

## Decisions

### 1. Record visible session history on `REC` -> `OFF`, not when `REC` or `CONT` is armed

Add explicit session-tracking state in `TasApp` that records:

- whether the active session was a fresh `REC` or a `CONT`
- the tick the session started from
- the latest observed `recorded_count`

When the app sees a mode transition from recording to off, it will compare the final `recorded_count` with the session start tick:

- if the count increased, push a new history snapshot for the final state with a generated session label
- if the count did not increase, do nothing

This reuses the current mode-transition handling in `main.rs` and keeps the visible history aligned with what the user just captured.

Alternatives considered:

- Keep pushing `Before REC` / `Before CONT` snapshots and infer completion later.
  Rejected because the visible entry is still anchored to the wrong user action.
- Push entries on every tick while recording.
  Rejected because it would flood the visible history list with low-value entries.

### 2. Use a shared 100 Hz label formatter for recording sessions

Add a helper in `recording.rs` that converts tick counts into a human-readable base-100 duration string. The formatter should produce labels consistent with the task examples:

- `Recorded 23:03`
- `Continued 23:03, total 53:03`

The same helper should be used for:

- visible history entries
- any recovery metadata shown to the user
- tests that validate label text

Alternatives considered:

- Keep generic labels such as `Before REC` or `Snapshot`.
  Rejected because they do not tell the user what was recorded.
- Show raw tick counts only.
  Rejected because the task explicitly asks for time-style labels.

### 3. Keep crash recovery separate from the visible history list

Do not model crash-recovery checkpoints as visible history entries. Instead, add a recovery store that writes a recoverable recording snapshot to disk while `REC` or `CONT` is active and again when recording stops successfully.

The recovery payload should contain enough data to restore the latest checkpoint:

- recording contents, preferably using the existing `RecordingFile` format
- segment metadata
- the label metadata needed to rehydrate the matching session entry after recovery

On the next launch after an unclean shutdown, `SSB Inspect` should detect the latest recovery checkpoint and expose it as a restore candidate rather than silently mixing background autosaves into the normal history panel.

Alternatives considered:

- Extend `history_store.json` with raw snapshot buffers.
  Rejected because it duplicates the recording file format and makes manual recovery harder.
- Add background autosave checkpoints directly into the visible history list.
  Rejected because the list should represent intentional user-visible milestones, not internal safety writes.

### 4. Recovery writes should be debounced and atomic

Recovery should track recording progress closely without rewriting the checkpoint file on every tick. Persist only when the recording length advances and a short debounce window has elapsed, then force one final checkpoint on `REC` -> `OFF`.

Writes should go through a temp file and atomic replace so a crash during persistence does not leave a partially written recovery file.

Alternatives considered:

- Write the recovery file on every tick.
  Rejected because repeated full-file rewrites could become expensive for long recordings.
- Persist only on stop.
  Rejected because it does not help when the crash happens before the user presses stop.

## Risks / Trade-offs

- [Session classification drifts from transport intent] -> Mitigation: track explicit `REC` vs `CONT` session kind instead of inferring it only from `continue_from_frame`.
- [Recovery files fall slightly behind the exact crash point] -> Mitigation: debounce writes by a short interval and always flush on recording stop.
- [Recovery write corruption leaves no usable checkpoint] -> Mitigation: use temp-file writes plus atomic replace.
- [History labels and recovery labels diverge] -> Mitigation: generate both through the same formatter and session-completion helper.

## Migration Plan

- Introduce the session-tracking helper state in `TasApp` and reroute the current `Before REC` / `Before CONT` history pushes through the new completion-based flow.
- Add recovery persistence alongside the existing history metadata autosave without changing the user-facing save/load file format.
- On startup, detect any pending recovery checkpoint from the previous unclean session and surface it through the app's existing restore flow.
- If the recovery store fails, disable only crash recovery and keep normal history behavior working.

## Open Questions

- What debounce interval gives a good balance between write volume and recovery fidelity for long recordings?
- Should the recovery candidate be surfaced automatically at startup, through the `File` menu, or both?
