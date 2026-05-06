# TAS Quality Gates

The TAS workspace has two test surfaces: hosted CI (Rust unit tests) and a local
hardware lane (everything else). The full integration suite needs `Supreme.exe`
running with hooks injected, and most of it also needs a Pico HID board, so it
runs on the developer's machine — not in CI.

## Lanes

### CI lane (hosted runner)

Runs on every pull request and on pushes to `main`/`tas` via the `tas:test` job
in `.github/workflows/ci.yaml`.

```
cd TAS && cargo test --release
```

That's the entire CI surface. No game, no Pico, no integration coverage.

### Local lane (your machine)

All other modes require a live game and most require a Pico HID. The harness
auto-launches `Supreme.exe` via `revive-supreme` (set `NO_REVIVE=1` to reuse a
session you already have running).

| Mode | Pico? | Pass signature | Notes |
|---|---|---|---|
| `smoke` | no | exit 0, no `ERROR:` lines | Quick harness sanity (~30s) |
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
| `segment` | yes | `*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***` | Two-segment CONT |

All gates require exit code `0` in addition to the pass signature.

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

## Cadence

For hardware-backed validation cadence, preconditions, and escalation, see
[`docs/live-runtime-validation-cadence.md`](live-runtime-validation-cadence.md).
