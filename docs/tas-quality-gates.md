# TAS Quality Gates

The TAS workspace has two test surfaces: hosted CI (Rust tests and native C++
tests) and local game integration tests. The integration suite needs `Supreme.exe`
running with hooks injected, and most of it also needs a Pico HID board, so it
runs on the developer's machine — not in CI.

## Lanes

### CI lane (hosted runner)

Runs on every pull request and on pushes to `main` via the `tas:test` job
in `.github/workflows/ci.yaml`.

```
cd TAS && cargo test --release
```

CI also builds the Win32 native DLL and runs the eight C++ policy suites via
`TAS/tools/test_dll_hidden.ps1`. No game or Pico is involved. Windows Rust unit
tests use private unnamed mappings, never the running game's shared memory.
Offline Python helper tests also run in this job (`just test_tools` locally).
Tool usage and captured-fixture provenance are in the [TAS reference](../TAS/README.md).

### Local lane (your machine)

Live tests control the game and can replace its recording. Save your run first.
Use only one injected game and one test controller at a time. The harness
auto-launches `Supreme.exe` via `revive-supreme` (set `NO_REVIVE=1` to reuse a
session you already have running).

| Mode | Pico? | Pass signature | Notes |
|---|---|---|---|
| `smoke` | no | `*** SMOKE TEST PASSED ***` | Pipeline liveness (~40s): ticks captured, playback ran to completion, player moved in both REC and PLAY. Drift is deliberately **not** asserted — smoke is not F5-aligned, so REC and PLAY start from different spawns. |
| `f5` | no | `=== Overall: ALL GATES PASS ===` | F5-aligned zero-drift baseline |
| `speed` | no | `*** SPEED TEST PASSED ***` | 0.25x / 1x / 2x playback |
| `speed-reset` | no | `*** SPEED RESET TEST PASSED ***` | Cave-5 OFF-mode regression guard |
| `benchmark` | no | summary table | Cave-hook perf timing (informational) |
| `replay <file>` | no | `Result: ZERO DRIFT in all N iterations` | Replay a `.tasrec` |
| `acceptance` | yes | `*** ACCEPTANCE TEST PASSED ***` | 3-phase E2E |
| `regression` | yes | `=== Regression Summary: 15/15 passed ===` | 15-case drift suite |
| `drift-speed` | yes | `*** DRIFT-AT-SPEED TEST PASSED ***` | Drift across speeds |
| `reliability` | yes | `*** RELIABILITY TEST PASSED ***` | Repeated REC/PLAY cycles |
| `cont-reliability` | yes | `*** CONT RELIABILITY PASSED ***` | Splice stability |
| `save-reload` | yes | `*** SAVE/RELOAD/REPLAY PASSED ***` | Cross-session record → save to disk → kill game → revive → reload → replay → zero drift. Exercises the same persistence path tas_ui uses; catches any state lost across game restart. |
| `segment` | yes | `*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***` | Two-segment CONT |

All gates require exit code `0` in addition to the pass signature.

### Track guard

Every mode asserts the game is on **Forest Easy** before it runs, and exits `1`
with an explanation otherwise. This is not cosmetic: on the wrong track the spawn
is on a different map, so a replay's start matcher can never hit its target and
burns its whole retry budget (a mode appears to hang for ~10 minutes), while a
fresh REC silently records another course. The check waits up to 12s for the
DLL's level-scan thread to publish a track, since it re-scans about every 1.5s.

```powershell
$env:TAS_TEST_LEVEL = 'AM'    # expect a different track
$env:TAS_TEST_LEVEL = 'any'   # disable the check (deliberate off-track work)
```

`level_id = 0xFFFFFFFF` means the game is **not on one of the nine Tracks** — a
menu, or a mode the scan does not cover (Practice, Halfpipe). `gamestate` opts
out automatically so it stays usable as a diagnostic precisely when the session
is in that state.

## Recipes

```powershell
# CI surface (also fine to run locally)
just test

# Local validation before push (Pico required)
just test_acceptance
just test_regression
```

Set `TAS_TEST_OUTPUT` if you want artifacts in a stable location:

```powershell
$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'
```

Otherwise artifacts (CSVs, certificates) land next to the binary.

See the [E2E test guide](e2e-test-guide.md) for CONT reliability commands and
troubleshooting.
