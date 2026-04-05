# TAS Quality Gate Suite

Canonical gate matrix for preventing TAS regressions in `TAS/tas_test`.

## Lane Definitions

- `Fast lane (pre-commit)`: deterministic, no game process, no Pico HID.
- `CI-on-push lane`: automated runtime checks (game + DLL/hooks), no human interaction.
- `Manual live-runtime lane`: hardware/game-process dependent validation requiring operator control (Pico HID and/or revive cadence).

## FE-decent Compatibility Rule

- `recordings/FE-decent.tasrec` is a compatibility/diagnostic artifact, not a shared hard baseline for both `replay` and file-backed `cont-reliability`.
- File-backed FE-decent commands should record evidence and warnings, but they must not decide pass/fail for automation or release gates until separate replay-vs-CONT baseline semantics are defined.

## Mock Regression Baseline Rule

- `tas_test -- mock` is currently a diagnostic gate, not a hard release/CI gate.
- The active blocker is start-match stability, not drift semantics: on runs that position-match successfully, full-window replay drift and normalized active-input-window drift both collapse to zero.
- Baseline source order is:
  1. sibling `regression_cache/*.tas` produced by a validated `tas_test -- regression` run
  2. checked-in fallback fixtures under `TAS/tas_test/fixtures/regression_cache/*.tas`
- Baseline filenames use the stable canonical ordinals `01_...` through `15_...`. Focused reruns via `TAS_TEST_CASE_FILTER` must keep those ordinals so filtered mock runs still load the same authoritative baselines.
- Promotion criteria: only restore `mock` to required after it demonstrates stable `=== Regression Summary: 15/15 passed ===` behavior on a clean runtime, with the recorded per-case start-offset / active-window artifacts staying at zero on matched runs.
- Mock artifacts should record `first_input_tick`, frame-0 offsets, active-input-window length, and normalized active-window drift for triage.

## Global Runtime Rules

- Runtime commands run from `C:\Users\user\git\SS-Dat-Info\TAS`.
- Default artifact directory is the `tas_test` binary directory. For stable paths, set `TAS_TEST_OUTPUT` first:
  - `$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'`
- All gates require exit code `0` in addition to the pass signatures listed below.

## Test Inventory and Lane Classification

| Mode | Command | Lane | Pass Signature(s) | Required Artifacts |
|---|---|---|---|---|
| Rust unit tests | `just test` | Fast lane | `test result: ok.` for all crates | Console log |
| `mock` | `cargo run --release --bin tas_test -- mock` | CI-on-push (diagnostic only) | `=== Regression Summary: 15/15 passed ===` | `mock_results.csv`, `mock_certificate.json` |
| `replay` (FE-decent compatibility) | `cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose` | CI-on-push (diagnostic only) | `Result: ZERO DRIFT in all 1 iterations` | Console log |
| `speed` | `cargo run --release --bin tas_test -- speed` | CI-on-push | `*** SPEED TEST PASSED ***` | Console log |
| `speed-reset` | `cargo run --release --bin tas_test -- speed-reset` | CI-on-push | `*** SPEED RESET TEST PASSED ***` | Console log |
| `smoke` | `cargo run --release --bin tas_test -- smoke` | CI-on-push | No explicit PASS banner; gate is exit code `0` with no `ERROR:` lines | Console log |
| `f5` | `cargo run --release --bin tas_test -- f5` | CI-on-push | `=== Overall: ALL GATES PASS ===` | Console log |
| `benchmark` (non-gate diagnostic) | `cargo run --release --bin tas_test -- benchmark --repeats 3 --frames 600` | CI-on-push (informational) | Produces summary table only; do not block on this mode | Benchmark log |
| `acceptance` | `cargo run --release --bin tas_test -- acceptance` | Manual live-runtime | `VERDICT: steering=PASS replay_steered=PASS zero_drift=PASS` and `*** ACCEPTANCE TEST PASSED ***` | `acceptance_certificate.json`, console log |
| `regression` | `cargo run --release --bin tas_test -- regression` | Manual live-runtime | `=== Regression Summary: 15/15 passed ===` | `regression_results.csv`, `regression_certificate.json` |
| `drift-speed` | `cargo run --release --bin tas_test -- drift-speed` | Manual live-runtime | `*** DRIFT-AT-SPEED TEST PASSED ***` | Console log |
| `reliability` | `cargo run --release --bin tas_test -- reliability --iterations 10 --speed 12` | Manual live-runtime | `*** RELIABILITY TEST PASSED: 10/10 zero drift at 12x ***` | Console log |
| `segment` | `cargo run --release --bin tas_test -- segment` | Manual live-runtime | `*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***` | Console log |
| `cont-reliability` (FE-decent file compatibility) | `cargo run --release --bin tas_test -- cont-reliability --file recordings/FE-decent.tasrec --splice 2400 --iterations 10 --speed 12` | Manual live-runtime (diagnostic only) | `*** CONT RELIABILITY PASSED: 10/10 splice cycles clean ***` | Console log |
| `cont-reliability` (synthetic baseline, stress) | `cargo run --release --bin tas_test -- cont-reliability --iterations 10 --splice 2400 --speed 32 --profile taps --tap-ticks 8` | Manual live-runtime (hard gate) | `*** CONT RELIABILITY PASSED: 10/10 splice cycles clean ***` and summary row columns `cover=ok`, `fwd=ok` | Console log |

## Mandatory Gate Packs

### 1) Fast Lane (every commit, no hardware)

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info
just test
```

Fail fast if this command exits non-zero.

### 2) CI-On-Push Lane (automated runtime worker)

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info\TAS
$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'
cargo run --release --bin tas_test -- mock
cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose
cargo run --release --bin tas_test -- speed-reset
```

`speed-reset` is the only runtime hard gate in this lane today. `mock` and `replay recordings/FE-decent.tasrec` still run in this lane, but they are diagnostic-only and may warn without failing the run while start-match stability remains unresolved.

### 3) Manual Live-Runtime Lane (release/handoff gate)

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info\TAS
$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'
cargo run --release --bin tas_test -- acceptance
cargo run --release --bin tas_test -- regression
cargo run --release --bin tas_test -- drift-speed
cargo run --release --bin tas_test -- cont-reliability --iterations 10 --splice 2400 --speed 32 --profile taps --tap-ticks 8
```

For extreme-speed certification, repeat the final command at `--speed 64` and `--speed 100`.

### 4) FE-decent Compatibility Diagnostics (non-blocking)

Use these commands to preserve evidence about FE-decent behavior while replay-vs-CONT baseline semantics remain intentionally split:

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info\TAS
cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose
cargo run --release --bin tas_test -- cont-reliability --file recordings/FE-decent.tasrec --splice 2400 --iterations 1 --speed 12
```

## Recommended Minimum Always-Run Set

Balanced for high signal with bounded runtime cost:

1. `just test` on every commit (fast, deterministic).
2. On every push (runtime automation worker), run:
   - `tas_test -- mock` as a diagnostic artifact
   - `tas_test -- speed-reset`
   - collect `tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose` as a diagnostic artifact

Rationale:

- `mock` still catches regression-pattern drift and gate breakage quickly, but current start-match instability makes it unsuitable as a hard gate.
- `speed-reset` protects the historically high-impact Cave 5 OFF-mode regression.
- `replay` against FE-decent remains useful compatibility evidence, but it is not a shared hard baseline for both replay and CONT flows.
- Full live-runtime suite stays in manual lane to control cost and hardware contention.

## Automation Entry Points (SSB-300)

Single-command local/agent invocation:

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info
just test_fast_lane
```

Direct script invocation (equivalent):

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info
pwsh -NoProfile -File .\scripts\run-tas-fast-lane.ps1 -ArtifactsDir TAS/artifacts/fast-lane/latest
```

CI workflow target:

- `.github/workflows/tas-fast-lane.yaml`
- Trigger: every pull request + pushes to `main` and `tas`
- Runner: self-hosted Windows x64 runtime worker

Expected outputs per run:

- Console `START/END` lines for each gate with status + duration.
- Gate logs in `TAS/artifacts/fast-lane/latest/logs/`.
- `tas_test` artifacts in `TAS/artifacts/fast-lane/latest/tas_test_output/` (notably `mock_results.csv` and `mock_certificate.json`).
- Summary markdown at `TAS/artifacts/fast-lane/latest/summary.md` listing gate kind, status, command, duration, and log path.
- Workflow artifact upload of the full `TAS/artifacts/fast-lane/latest` directory for triage.

Fail behavior (hard gate):

- Any non-zero required gate exit code fails the run.
- Missing expected pass signatures in required gate logs also fail the run.
- Diagnostic FE-decent failures are reported as warnings and kept in artifacts, but they do not fail the run.

## Recurring Manual Live-Runtime Cadence

For schedule, preconditions, evidence requirements, and escalation policy for hardware-backed E2E runs, use:

- [`docs/live-runtime-validation-cadence.md`](live-runtime-validation-cadence.md)
