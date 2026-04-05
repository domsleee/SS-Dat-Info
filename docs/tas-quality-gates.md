# TAS Quality Gate Suite

Canonical gate matrix for preventing TAS regressions in `TAS/tas_test`.

## Lane Definitions

- `Fast lane (pre-commit)`: deterministic, no game process, no Pico HID.
- `CI-on-push lane`: automated runtime checks (game + DLL/hooks), no human interaction.
- `Manual live-runtime lane`: hardware/game-process dependent validation requiring operator control (Pico HID and/or revive cadence).

## Global Runtime Rules

- Runtime commands run from `C:\Users\user\git\SS-Dat-Info\TAS`.
- Default artifact directory is the `tas_test` binary directory. For stable paths, set `TAS_TEST_OUTPUT` first:
  - `$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'`
- All gates require exit code `0` in addition to the pass signatures listed below.

## Test Inventory and Lane Classification

| Mode | Command | Lane | Pass Signature(s) | Required Artifacts |
|---|---|---|---|---|
| Rust unit tests | `just test` | Fast lane | `test result: ok.` for all crates | Console log |
| `mock` | `cargo run --release --bin tas_test -- mock` | CI-on-push | `=== Regression Summary: 15/15 passed ===` | `mock_results.csv`, `mock_certificate.json` |
| `replay` | `cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose` | CI-on-push | `Result: ZERO DRIFT in all 1 iterations` | Console log |
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
| `cont-reliability` (file baseline) | `cargo run --release --bin tas_test -- cont-reliability --file recordings/FE-decent.tasrec --splice 2400 --iterations 10 --speed 12` | CI-on-push | `*** CONT RELIABILITY PASSED: 10/10 splice cycles clean ***` | Console log |
| `cont-reliability` (synthetic baseline, stress) | `cargo run --release --bin tas_test -- cont-reliability --iterations 10 --splice 2400 --speed 32 --profile taps --tap-ticks 8` | Manual live-runtime | `*** CONT RELIABILITY PASSED: 10/10 splice cycles clean ***` and summary row columns `cover=ok`, `fwd=ok` | Console log |

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

All three commands must pass.

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

## Recommended Minimum Always-Run Set

Balanced for high signal with bounded runtime cost:

1. `just test` on every commit (fast, deterministic).
2. On every push (runtime automation worker), run:
   - `tas_test -- mock`
   - `tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose`
   - `tas_test -- speed-reset`

Rationale:

- `mock` catches regression-pattern drift and gate breakage quickly.
- `replay` catches deterministic playback drift against known-good data.
- `speed-reset` protects the historically high-impact Cave 5 OFF-mode regression.
- Full live-runtime suite stays in manual lane to control cost and hardware contention.
