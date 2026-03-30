# SSB-275 Deep Investigation (2026-03-31)

## Scope

Issue: random crashes reported on:

- `Record` flow
- `Load .tasrec` -> `Play` flow

Objective for this retry: reproduce and isolate real crash conditions vs runtime stalls/hangs.

## Environment

- Runtime reset used before live runs (`revive-supreme` sequence)
- Process under test: `Supreme.exe` (with TAS helper hooks active)
- Test driver: `TAS/target/release/tas_test.exe`

## Reproduction Matrix

### 1) Replay path with default position matching

Command:

```powershell
tas_test.exe replay C:\Users\user\.ssb-inspector\recovery\recovery_checkpoint.tasrec --iterations 1 --verbose
```

Observed:

- Deterministic long run (~331s) due repeated F5 restart retries for start-position matching (`20` retries).
- Position never matched.
- Final result exited with failure (drift + early playback exit), but **no Windows crash event** during this run.

Key output pattern:

- recurring `play_coords[0] offset: dx~6.4 dz~959-960`
- `WARNING: Could not match position after 20 retries`
- `Playback exited PLAY early ...`

Interpretation:

- This is a deterministic **hang/timeout-like failure mode**, not a hard process crash.
- User perception can be "it crashed" because command appears stuck for minutes.

### 2) Replay path without matching (fast path)

Command:

```powershell
tas_test.exe replay C:\Users\user\.ssb-inspector\recovery\recovery_checkpoint.tasrec --iterations 5 --verbose --no-match
```

Observed:

- Runs complete normally.
- Large drift every iteration (expected due no start-position alignment).
- No process crash events.

### 3) Repeated smoke cycles (REC/PLAY)

Command:

```powershell
tas_test.exe smoke   # repeated in loop
```

Observed:

- First 6 runs passed.
- Run 7 failed with:
  - `Liveness: 0 frames/500ms`
  - `ERROR: Cave 2 not firing`
- `Supreme.exe` remained alive and responsive.
- No Windows crash events at failure time.

Interpretation:

- Deterministic **stalled runtime state** can appear after repeated REC/PLAY cycles.
- Not equivalent to hard process termination.

## Windows Crash Log Correlation

Queried `Application Error` events for `Supreme.exe` (last 7 days):

- `7x` `sr.dll + 0x0001357f` with `0xc0000005`
- `1x` `sr.dll + 0x00026839` with `0xc0000005`
- `1x` `sr.dll + 0x00026839` with `0xc000041d`
- additional `unknown` module crashes also present

Conclusion:

- Real hard crashes do exist historically, but in this retry session the deterministic repros were stalls/hangs, not fresh hard-crash events.

## Root-Cause Split

1. **Hard crash class (historical):**
   - Access violations in `sr.dll` (`0x1357f`, `0x26839`).
   - Engine-module fault site, not directly proven by current retry steps.

2. **Stall/hang class (reproduced this session):**
   - Replay path can spend minutes in position-match retries and still fail.
   - REC/PLAY loops can enter `cave2 not firing` state without process crash.

## Practical Guidance

- Treat replay start-match loops as bounded-risk diagnostics; they can look like crash to users.
- If `Liveness: 0` or `cave2 not firing`, recover runtime state first (`revive-supreme`) before further conclusions.
- Distinguish:
  - "Process crashed" = Windows `Application Error` event exists.
  - "Runtime stalled" = process alive, no crash event, frame_count not advancing.

## Artifacts

- `TAS/target/replay-full.out.log`
- `TAS/target/replay-full.err.log`
- `TAS/target/smoke-investigation/smoke-*.log`

