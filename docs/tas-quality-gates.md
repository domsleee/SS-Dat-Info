# TAS quality gates

The TAS workspace has two test surfaces: hosted CI, which runs the Rust, C++
and Python suites without a game, and the live suite, which drives
`Supreme.exe` with the DLL injected on the developer's machine. This page is
the complete contract for both: every `tas_test` mode, the environment
variables, the artifacts and the recovery steps. Build and deploy commands are
in the [TAS README](../TAS/README.md).

## CI lane

The `tas:test` job in `.github/workflows/ci.yaml` runs on every pull request
and on pushes to `main`:

```
cd TAS && cargo fmt --all --check
cd TAS && cargo clippy --workspace --all-targets -- -D warnings
python -m unittest discover -s TAS/tools -p test_tools.py -v
cd TAS && cargo test --release
MSBuild TAS/TAS_Helper/TAS_Helper.vcxproj /p:Configuration=Release /p:Platform=Win32
./TAS/tools/test_dll_hidden.ps1
```

`just test`, `just test_tools` and `just test_dll` run the same steps locally.
Windows Rust unit tests use private unnamed mappings, never the running game's
shared memory. The C++ policy suites compile the pure gate-policy headers
(input gate, replay capture, gate alignment, level path, rider identity, menu
model, race timer, setup config) into standalone executables; the script runs
them hidden because a console window taking focus pauses a running game.

### Captured UI regression

`TAS/tas_ui/src/tests/data/cont-splice-4500/` backs the ordinary Rust test
`captured_ui_break_at_4500_exercises_production_banner`
(`cargo test --release -p tas_ui cont_splice`):

- `recording.tasrec` is the saved FE recording, history entry 2431 (5,032 ticks).
- `play-tail.bin` is 4,500 captured playback XYZ samples (54 KB) whose coverage
  starts at recording tick 945. Intermediate differences reproduce the false
  banner while the actual splice endpoint matches.
- `capture.json` records the binary layouts, alignment and capture provenance.

The test drives that capture through the production drift scanner and banner,
including the PLAY-to-REC transition and a divergent-endpoint negative control.
`cont-ui-left-spam` covers the same bug against the real game.

## Live lane

Live tests control the game and replace its active recording. Save your run
first, use one injected game and one test controller at a time, and do not
play manually during a test. Prerequisites:

- Windows, Rust, and Supreme Snowboarding with the current TAS DLL deployed
  (`just deploy_run`).
- A Pico HID board running the firmware in `TAS/pico/` for the modes marked
  Pico below. The harness writes only on mask changes and the firmware
  releases every key 500 ms after the last byte, so the regression baselines
  include that release.
- `SUPREME_FOLDER` and `REVIVE_SUPREME_SCRIPT` in the environment so the
  harness can launch the game, or a game already running with `NO_REVIVE=1`.
- The game on Forest Easy, unless `TAS_TEST_LEVEL` says otherwise.

Every mode requires exit code `0` in addition to its pass signature. Run a mode
from `TAS/` with `cargo run --release --bin tas_test -- <mode>`, or from the
repository root through the `just test_*` recipes named below. Modes marked
Pico drive steering or a keypress through the board; `acceptance`,
`cont-ui-left-spam` and `live` fail fast without it, the others log
`Cannot open COM7` and wait through the steering window instead, which makes
their drift verdict vacuous.

### Suites

| Mode | Pico | Pass signature | What it checks |
|---|---|---|---|
| `live [--recording PATH] [--splice N] [--iterations N]` | yes | `Live suite report: .../summary.json`, every stage `passed` | Runs `cont-ui-left-spam`, then `acceptance`, then `regression` as child processes; a failing stage stops the rest. `just test_live [splice] [iterations]`. |
| `cont-ui-left-spam [--recording PATH] [--splice N] [--iterations N]` | yes | `CONT UI LEFT-SPAM PASSED` | Launches an isolated `tas_ui.exe`, loads the captured 4500 recording, sends F12 and physical LEFT taps, checks first-attempt resume and zero splice mismatch on each of N splices, stops with F11. `just test_cont_ui_left_spam`. |
| `acceptance [N]` | yes | `*** ACCEPTANCE TEST PASSED ***` per run, `=== Acceptance: N/N runs passed ===` | Three phases: unsteered baseline REC, Pico-steered REC that must differ from it, PLAY that must match the steered REC. `just test_acceptance`. |
| `regression` | yes | `=== Regression Summary: 15/15 passed ===` | 15 scripted steering patterns, each REC then PLAY with zero drift; writes the CSV and certificate. `just test_regression`. |

### Drift and replay

| Mode | Pico | Pass signature | What it checks |
|---|---|---|---|
| `smoke` | no | `*** SMOKE TEST PASSED ***` | Pipeline liveness (~40 s): ticks captured, playback ran to completion, player moved in REC and PLAY. REC and PLAY start from different spawns here, so liveness is the whole verdict. `just test_smoke`. |
| `f5` | no | `=== Overall: ALL GATES PASS ===` | F5-aligned straight-line REC then PLAY through the gate checks. |
| `segment` | yes | `*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***` | Two-segment CONT: LEFT segment, F5-matched CONT into a RIGHT segment, full replay with zero drift at the boundary. |
| `replay <file.tasrec> [--iterations N] [--verbose] [--no-match]` | no | `Result: ZERO DRIFT in all N iterations` | Loads a `.tasrec` and replays it N times; drift, incomplete playback or a failed start match fails. `just test_replay FILE`. |
| `reliability [--iterations N] [--speed X]` | yes | `*** RELIABILITY TEST PASSED: N/N zero drift at Xx ***` | N consecutive steered REC+PLAY cycles at one speed (default 10 at 12x). |
| `drift-speed` | yes | `*** DRIFT-AT-SPEED TEST PASSED ***` | REC 2x/PLAY 2x and REC 1x/PLAY 2x both replay with zero drift. |
| `save-reload` | yes | `*** SAVE/RELOAD/REPLAY PASSED: zero drift across game restart ***` | Steered REC, save to disk, kill and relaunch the game, reload, replay, zero drift. |
| `pause-resume` | yes | `*** PAUSE/RESUME REPLAY PASSED: ...` | Escape pause and resume during PLAY; the first 1000 frames stay bit-identical. |
| `stop-play-flake` | no | `*** STOP+PLAY FLAKE TEST PASSED: N/N iterations matched the reference ...` | PLAY, STOP at varying frames, PLAY again; ten second playbacks match a reference over their first 1000 frames. |
| `rec-start [--file PATH]` | no | `*** REC-START OK: recording begins at the spawn ...` | A fresh recording starts at the stationary spawn with the countdown, not mid-fall; `--file` judges a saved `.tasrec` instead. |
| `rec-repro` | yes | `*** REC-REPRO OK: N transitions match in order+mask ...` | The same driven input recorded twice yields the same transitions within tolerance. |
| `steer-impact` | no | `*** STEER-IMPACT OK: ...` | Injected steering moves the player, and only with a live Kernel::Time stamp on the injection. |
| `refresh-tasrec <source.tasrec> <out.tasrec>` | no | `Saved refreshed baseline to PATH` | Replays a recording in the live game and writes it back with the freshly captured coordinates. |

### Speed and timing

| Mode | Pico | Pass signature | What it checks |
|---|---|---|---|
| `speed` | no | `*** SPEED TEST PASSED ***` | 0.25x and 2x tick counts scale against 1x; speed is back at 1x afterwards. |
| `speed-reset` | no | `*** SPEED RESET TEST PASSED ***` | After 2x REC and STOP the OFF-mode tick rate and F5 are back to normal. |
| `catchup-speed` | no | `*** CATCH-UP SPEED OK: 64× catch-up is Nx faster than 1× ...` | Median time-to-splice at 64x versus 1x stays above the required ratio. |
| `play-pace` | no | `*** PLAY-PACE OK: 1× PLAY ran at N% of native wall time ...` | 1x PLAY of a fixed frame window takes native wall time. |
| `play-judge [--iterations N]` | no | `*** PLAY JUDGE HANDOVER PASSED ***` | A judged PLAY replays the countdown at catch-up speed and hands back at `first_moving + 1` with the run unchanged. |
| `video-rate [secs] [--at X Y]` | no | rate summary | Distinct frames per second reaching the screen, measured from outside the process; works with no DLL injected. |
| `dialog-e2e` | yes | `*** DIALOG-E2E PASSED: save-dialog and menu behave at native speed end to end ***` | Real finishes with a Pico Escape on the save dialog in PLAY and after a CONT splice, then the main menu speed; the quit navigates by menu-document ids after one physical ESC. |

### CONT

| Mode | Pico | Pass signature | What it checks |
|---|---|---|---|
| `cont-reliability [--iterations N] [--speed X] [--splice N] [--file PATH \| --synthetic] [--profile taps\|sweep] [--tap-ticks N]` | synthetic baseline only | `*** CONT RELIABILITY PASSED: N/N splice cycles clean ***` | Repeated CONT splices at the given frame: zero drift in the replayed prefix, replay coverage and forward progress. Defaults to FE-tremendous at splice 2200; `--synthetic` records a fresh Pico-driven baseline instead. |
| `fe-cont-reliability` | no | `*** FE-CONT-RELIABILITY PASSED ***` | FE-tremendous, five splices at 2200 at 12x. |
| `fe10065-cont` | no | `*** FE-10065 CONT PASSED ***` | FE-10065, eight splices at 6200 at each of 64x and 256x, resume overshoot at most one frame and best resume time at most 3000 ms. |
| `cont-hijack` | no | `*** BUG #2 PASSED: replay crossed frame N still in PLAY — no REC hijack ***` | A `continue_from_frame` written during a plain PLAY leaves it in PLAY. |
| `cont-restart-race` | no | `*** PASS: serialised Stop→Restart is accepted by cave2 ***` | Stop and Restart sent together lose the Stop; Stop, wait for OFF, then Restart is accepted. |
| `cont-input-protection` | no | every `PASS:` line, ending `PASS: ordinary STOP releases input protection (cont_suppress_input=0)` | Live input stays blocked through the CONT restart and STOP releases it, driven through the real transport controller. |
| `gate-align [N] [recording]` | no | `*** Gate-offset input indexing matched the control in this sample. ***` | Gate-relative input indexing reproduces the recording's run over N aligned replays. |

### Diagnostics

These run on whatever track is loaded; the track guard below is skipped.

| Mode | Output | What it does |
|---|---|---|
| `shm [--command record\|play\|stop\|restart]` | `version=... command_idle=... renderer_id=... fpu_cw=...`, rider and status lines | Version-checked shared-memory read; never launches or restarts the game. With `--command` it publishes one named command when none is pending. |
| `load <file.tasrec>` | `loaded N ticks from PATH` | Writes a recording into shared memory and exits, for arming through the real UI afterwards; refuses while the DLL is in REC or PLAY. |
| `menu [activate <id\|label> \| focus <id\|label> \| up \| down \| left \| right \| trigger]` | one JSON line `{"result":..,"doc":..}`, exit 0 on ok | Prints the menu document (page, selector, focusable items with labels and ids) or executes one command on the menu thread through the game's own entry points and prints the settled document. |
| `gamestate` | six status prints | Launches or reuses the game and prints `game_in_game` with the rest of the status. |
| `level-seq [secs]`, `level-seq watch [secs]` | `*** level-seq PASSED: the seqlock is engaged and reads are coherent ***` | The DLL publishes level context through the seqlock; `watch` logs transitions instead of asserting. |

### Track guard

Every mode outside the diagnostics table asserts that the game is on Forest
Easy before it runs and exits `1` with an explanation otherwise. On the wrong
track the spawn is on a different map, so a replay's start matcher can never
hit and burns its whole retry budget (the mode appears to hang for about ten
minutes), while a fresh REC records another course. The check waits up to 12 s
for the DLL to publish a level. Level detection is edge-driven: the DLL
re-reads the engine's setup object when the engine cycle freezes or resumes,
which covers every load and menu trip, with a 60 s re-read as a safety net
against a missed edge. A level is known within a second of a track loading.

```powershell
$env:TAS_TEST_LEVEL = 'AM'    # expect a different track
$env:TAS_TEST_LEVEL = 'any'   # disable the check for deliberate off-track work
```

Level codes are the nine Time-Attack tracks `FE FM FH AE AM AH VE VM VH` and
`PE` for Practice, which is a level in its own right. `level_id = 0xFFFFFFFF`
means the game is in a menu, mid teardown, or in Halfpipe, which the scan does
not classify.

## Environment variables

| Variable | Read by | Meaning |
|---|---|---|
| `SUPREME_FOLDER` | `tas_test` | Game folder; the harness injects `Display_Config_Resources\TAS\TAS_Helper.dll` from it after launching. Required whenever the harness has to launch the game, with no default. The root `justfile` exports it from `supreme_folder`. |
| `REVIVE_SUPREME_SCRIPT` | `tas_test` | Path to `revive-supreme.nu`, the launcher the harness runs when no game is live. The script comes from the separate `cheatengine-mcp-bridge` repository and needs Nushell (`nu`) on PATH. Required, with no default. |
| `NO_REVIVE` | `tas_test` | `1` or `true`: never launch the game; fail when none is live. |
| `TAS_TEST_LEVEL` | `tas_test` | Expected track code (default `FE`); `any` disables the track guard. |
| `TAS_TEST_OUTPUT` | `tas_test` | Artifact directory; default is next to `tas_test.exe`. |
| `TAS_PICO_PORT` | `tas_test`, `tas_ui` | Pico CDC data port (default `COM7`). |
| `SSB_INSPECT_DATA_DIR` | `tas_ui` | Root for history, recovery, recordings, settings and the session log; default is `data/` next to a deployed exe, or `~/.ssb-inspector` for a dev build. |
| `TAS_RACE_DIAG` | `TAS_Helper.dll` | `1` before launch: log every race-timer HUD line event. |
| `TAS_MENU_DIAG` | `TAS_Helper.dll` | `1` before launch: log the menu item list on every change. |
| `SSB_INSPECT_E2E_RECORDING`, `SSB_INSPECT_E2E_SPLICE` | `tas_ui` | Harness-only. `cont-ui-left-spam` sets them on the UI it launches: load this recording and set From to this splice at startup. The UI refuses them without an isolated `SSB_INSPECT_DATA_DIR`. |

## Artifacts

`TAS_TEST_OUTPUT` (default: beside `tas_test.exe`) receives
`regression_results.csv`, `regression_certificate.json` and
`regression_cache/` from `regression`; `acceptance_certificate.json` from
`acceptance`; `cont-ui-<timestamp>-<pid>/` from `cont-ui-left-spam` with the
child UI's isolated data directory and logs; and `live-<timestamp>-<pid>/`
from `live` with `summary.json` and a per-stage `output.log`, each stage's
own artifacts landing in its stage directory. `TAS/artifacts/` is git-ignored
for keeping these inside the checkout:

```powershell
$env:TAS_TEST_OUTPUT = 'C:\path\to\SS-Dat-Info\TAS\artifacts\latest'
```

## Recipes

CONT reliability against the default FE-tremendous baseline, one cycle at 32x,
from `TAS/`:

```powershell
cargo run --release --bin tas_test -- cont-reliability --iterations 1 --splice 2400 --speed 32 --profile taps --tap-ticks 8
```

Use `--iterations 10` for repeated coverage and `--speed 64` or `--speed 100`
for other catch-up speeds. Success requires exit code 0, the
`CONT RELIABILITY PASSED` summary, zero measured drift, and passing coverage
and forward-progress checks; a bucket match on its own leaves the replayed
prefix unproven. When a change touches a UI indicator, exercise the UI too and
poll it during the catch-up and after the PLAY-to-REC transition, which is
what `cont-ui-left-spam` does.

UI F12 with LEFT spam, five splices at 4500, from the repository root:

```powershell
just test_cont_ui_left_spam            # or: just test_cont_ui_left_spam 2200 5
```

The mode builds `tas_ui` beside `tas_test`, closes competing UI instances,
launches its own UI with Pico auto-connect disabled so the harness owns
`TAS_PICO_PORT`, loads the captured 4500 recording (or `--recording PATH`)
with From set to the splice, sends F12 through the UI, verifies physical LEFT
down/up transitions, checks first-attempt resume and an explicit zero splice
mismatch, then stops via F11. Retries, missing verdicts, focus loss, process
exit and nonzero splice differences fail the test; historical
`CONT prefix difference` diagnostics alone do not. The child UI is closed on
success or failure and its logs and data stay under `TAS_TEST_OUTPUT`.

The complete live workflow is `just test_live` (or `just test_live 2200`).
The UI stage runs first because the later harness stages take ownership,
close the UI and replace the active recording. A missing UI, a missing splice
verdict or any failing stage fails the workflow and the later stages stay
`not-run` in `summary.json`.

## Troubleshooting

- **Cave 2 not firing:** confirm the current DLL is injected and the game is
  in a race; `tas_test shm` shows the hook flags and the frame counter.
- **Wrong track or no level:** the guard names the track it saw. Navigate with
  `tas_test menu` and `tas_test menu activate <id>`, or set `TAS_TEST_LEVEL`.
- **CONT retries exhausted:** check the track, the loaded recording and the
  restart state.
- **Drift:** keep the failing recording and logs, and compare the rider and
  physics-mode stamps (renderer precision, character, stance) with the live
  game before retrying. A clean retry leaves the failure unexplained.
- **Reroll rate jumps:** a busy machine (a compile, or a git commit whose hook
  runs the console tests and pauses the game) turns first-try spawns into
  retries; reproduce on an idle machine before calling it a regression.
- **Pico problems:** `TAS\pico\deploy.ps1 -Check` confirms the board runs the
  committed firmware; the harness releases all keys on the port before every
  run, and a `Cannot open COM7` line in a steered mode's log means its drift
  verdict was reached without steering.
