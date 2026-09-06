# Supreme TAS E2E tests

These tests control the real game and can replace its active recording. Save the
run first, stop other tools that write to shared memory, and do not play manually
during a test. Unit tests do not require or control the game.

## Prerequisites

- Windows, Rust, and Supreme Snowboarding with the current TAS DLL.
- A Pico HID board running the firmware in `TAS/pico/` for input-driven tests.
- The `revive-supreme` launcher from `cheatengine-mcp-bridge`, with Nushell (`nu`)
  available. The harness uses it to launch the game; `NO_REVIVE=1` disables
  automatic launching so an existing session must be available.
- Forest Easy, unless deliberately selecting another track with `TAS_TEST_LEVEL`.

## Complete live workflow

Save your run, then run `just test_live` from the repository root. The test
establishes a game session, closes competing UI instances, and launches a fresh UI
with the original "UI break at 4500" recording and From set to 4500. History, settings and
logs are isolated under the test artifacts. To choose another splice, use
`just test_live 2200`.

The required stages are UI F12/LEFT-spam, acceptance, then regression. The UI test
runs first because subsequent harness tests take ownership, close the UI, and
replace the active recording. A missing UI, missing splice verdict, or any failing
stage fails the workflow; later stages are not run. Setup is never silently skipped.

Each invocation produces a
`live-<timestamp>-<pid>/summary.json`, per-stage `output.log`, and the existing
acceptance/regression artifacts under `TAS_TEST_OUTPUT` (default: beside tas_test).
The 4500 captured-data regression continues to run in the ordinary unit suite/CI.

## CONT reliability

Run from `TAS/`:

```powershell
cargo run --release --bin tas_test -- cont-reliability --iterations 1 --splice 2400 --speed 32 --profile taps --tap-ticks 8
```

For repeated coverage, use `--iterations 10`. Test other catch-up speeds with
`--speed 64` and `--speed 100`.

Success requires exit code 0, the `CONT RELIABILITY PASSED` summary, zero measured
drift, and passing coverage and forward-progress checks. A bucket match alone is
not proof that the full replayed prefix matches.

When testing a UI indicator, exercise the actual UI as well: command-line replay
checks cannot verify what it renders. Include polls during catch-up and after the
PLAY-to-REC transition, not just a completed coordinate comparison.

## UI F12 with LEFT spam

The test prepares its own UI and saved recording, with Pico auto-connect disabled
in that UI so the harness can drive physical input through `TAS_PICO_PORT`
(default `COM7`). It closes competing UI instances and replaces the active game
recording. Run from the repository root:

```powershell
just test_cont_ui_left_spam
```

This invokes `tas_test cont-ui-left-spam --splice 4500 --iterations 5`.
Use `--recording <path.tasrec>` on the CLI to select another fixture. It launches
the `tas_ui.exe` beside `tas_test.exe`; `just` builds both first. The child UI is
closed on success or failure, and its logs and isolated data are retained under
`TAS_TEST_OUTPUT` (default: beside the test binary).
It sends F12 through the UI, verifies physical LEFT down/up transitions, checks
first-attempt resume and an explicit zero splice mismatch, then stops via F11.
Retries, missing verdicts, focus loss, process exit and nonzero splice differences
fail the test. Historical `CONT prefix difference` diagnostics alone do not.

The 4500 regression capture lives under `TAS/tas_ui/src/tests/data/cont-splice-4500/`.
Its metadata records that it is a replay tail, starting at recording tick 945.
`cargo test --release -p tas_ui cont_splice` drives that capture through the
production scanner/banner, including the PLAY-to-REC transition and a divergent
endpoint negative control. This runs offline in CI; the LEFT-spam case is live-only.

## Troubleshooting

- **Cave 2 not firing:** confirm the current DLL is injected and the game is in a race.
- **CONT retries exhausted:** check the track, loaded recording and restart state.
- **Drift:** keep the failing recording and logs; check the rider and physics-mode
  stamps before retrying. Do not treat a clean retry as explaining the failure.

See [TAS test coverage and commands](tas-quality-gates.md) for the other test modes
and artifact locations.
