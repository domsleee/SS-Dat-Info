# TAS development

`TAS_Helper` is the injected native DLL, `tas_shared` defines its shared-memory
protocol, `tas_ui` (SSB Inspect) is the desktop UI, and `tas_test` is the live
game harness. The hooking library shared with `Display_Config_Helper` lives in
`third_party/safetyhook/` (safetyhook with its Zydis amalgamation).

## Quick start

From the repository root:

```
just deploy_run       # build DLL, tas_ui, tas_test and the injector; deploy; relaunch the game
just test             # Rust unit and captured-fixture regressions (the CI surface)
just test_tools       # offline Python helper tests
just test_dll         # native C++ policy tests
just test_live        # real game: UI LEFT-spam, acceptance, regression
```

`just deploy_run` is the one deploy flow. It stops the game and every TAS
process, copies `TAS_Helper.dll`, `tas_ui.exe`, `tas_test.exe` and
`Injector.exe` into the game folder and relaunches the game with a fresh UI.
The game folder is the `supreme_folder` variable at the top of the root
`justfile` (`just --set supreme_folder D:\Games\Supreme deploy_run` overrides
it), exported to the harness as `SUPREME_FOLDER`. Live tests control the game
and replace its recording, so save your run first. The full test contract,
every `tas_test` mode, the environment variables, artifacts and
troubleshooting are in [TAS quality gates](../docs/tas-quality-gates.md).

`TAS/Cargo.lock` is committed on purpose: the binaries ship, so every build
resolves the same dependency versions.

## Recordings

`recordings/*.tasrec` are the recordings the live modes replay and splice.
`FE-tremendous.tasrec` (4,696 ticks) is the default `cont-reliability`
baseline and drives `fe-cont-reliability`, `stop-play-flake`,
`catchup-speed`, `play-pace` and `cont-restart-race`; `FE-10065.tasrec`
(7,162 ticks, 221 segments) drives `fe10065-cont`, `gate-align`,
`play-judge` and `cont-hijack`; `FE-decent-done.tasrec` (8,149 ticks, crosses
the finish line) drives `dialog-e2e`. A `.tasrec` is a little-endian `u32`
header length, a JSON header, then the per-tick input bytes and XYZ
coordinates. Headers written by the current `tas_ui` carry `renderer`,
`fpu_control_word`, `character` and `stance`, the stamps the UI compares with
the live game because DirectX (24-bit) and OpenGL (53-bit) precision,
character and stance each change the physics. The three committed files were
saved before those stamps existed (header versions 5 and 6), so their renderer
and rider are unknown; the loaders ignore header fields they do not know.
When a physics change moves a trajectory, `tas_test refresh-tasrec
<source.tasrec> <out.tasrec>` replays the recording in the live game and
writes it back with the freshly captured coordinates.

## Offline inspection tools

Run these from the repository root. They read files, not the live game.

| Tool | Purpose |
| --- | --- |
| `tools/pe_inspect.py FILE sections` | Image layout and section permissions |
| `tools/pe_inspect.py FILE imports` | Imported DLLs |
| `tools/pe_inspect.py FILE exports --filter Time Clock Tick` | Exports, optionally filtered by name |
| `tools/pe_inspect.py FILE disasm 3b10 3d10 --stop-at-ret` | x86/x64 disassembly; hexadecimal RVAs, exclusive end |
| `tools/pe_inspect.py FILE callers 3940` | x86 call/jump byte-scan candidates and raw pointers |
| `tools/level_points.py generate start` | Rust start-line table; also `finish` and `spawn` |
| `tools/level_points.py inspect start` | Marker positions/orientations; `finish` includes object counts |

Prefix the tool paths above with `python TAS/`. PE inspection requires `pefile`;
disassembly additionally requires `capstone`. Install with
`python -m pip install pefile capstone`. The caller scan reports every byte
pattern that decodes as a call or jump to the target, so confirm each hit in
the disassembly.

Level tools default to the repository's `analyze/src/LevelData/levelData.json`;
override with `--input PATH` before the subcommand. Generated tables go to stdout.
Practice rows in `tas_ui/src/start_line.rs` are maintained by hand because the
JSON has no Practice data. Spawn output also reports shared clusters, which must
not be classified as a unique level.

`tools/keys.ps1 -Keys "ESC,DOWN,ENTER"` sends scan-code key presses to the game
window; `dialog-e2e` uses it for its quit sequence. `tools/test_dll_hidden.ps1`
compiles and runs the C++ policy tests with hidden windows so they cannot take
focus from the game. The Pico firmware and its deployment have their own
[README](pico/README.md).

## History store

`tas_ui` keeps run history in `tas_ui/src/history_store_v2.rs`:
`manifest.json` stores ordered metadata and the current entry ID, with immutable
ID-named `.tasrec` blobs and per-entry CRC32 checksums. Metadata edits leave the
recording blobs untouched. Its tests and the recording/UI tests cover migration,
recovery and eviction. The store lives under `SSB_INSPECT_DATA_DIR` when that
is set, otherwise in `data/` next to a deployed `tas_ui.exe`, or
`~/.ssb-inspector` for a dev build.
