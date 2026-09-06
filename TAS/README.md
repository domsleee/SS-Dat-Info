# TAS development

`TAS_Helper` is the injected native DLL, `tas_shared` defines its shared-memory
protocol, `tas_ui` is the desktop UI, and `tas_test` is the live game harness.

## Tests

From the repository root:

```
just test             # Rust unit and captured-fixture regressions
just test_tools       # Offline Python helpers; standard library only
just test_dll         # Native C++ policy tests
just test_live        # Real game: CONT UI regression, acceptance, regression
```

Live tests control the game and replace recordings. Save your run first and
follow the [E2E guide](../docs/e2e-test-guide.md) for deployment, Pico and UI setup.
The [quality gates](../docs/tas-quality-gates.md) describe each test's contract.
Replay equality alone cannot prove correct recording startup or playback speed;
those need the separate `rec-start`, `play-pace` and `catchup-speed` checks.

The `tas_ui/src/tests/data/cont-splice-4500` fixture supports the ordinary Rust
test `captured_ui_break_at_4500_exercises_production_banner`:

- `recording.tasrec` is the saved FE recording.
- `play-tail.bin` is 4,500 captured playback XYZ samples (54 KB), not a duplicate
  recording. Its coverage starts at recording tick 945. Intermediate differences
  reproduce the false banner while the actual splice endpoint matches.
- `capture.json` records the binary layouts, alignment and capture provenance.

This fixture checks the production UI drift functions offline; it does not
replace the live `cont-ui-left-spam` stage or certify a full zero-drift replay.

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
| `tools/diff_states.py A1 A2 B1 B2` | Bytes stable within each state but different between states |

Prefix the tool paths above with `python TAS/`. PE inspection requires `pefile`;
disassembly additionally requires `capstone`. Install with
`python -m pip install pefile capstone`. Caller results are candidates, not
validated instruction boundaries.

Level tools default to the repository's `analyze/src/LevelData/levelData.json`;
override with `--input PATH` before the subcommand. Generated tables go to stdout.
Practice rows in `tas_ui/src/start_line.rs` are maintained separately because the
JSON has no Practice data. Spawn output also reports shared clusters, which must
not be classified as a unique level.

Memory diffs default to base offset `88000` (hex). An optional fifth positional
argument sets a common base; `--a-base HEX --b-base HEX` aligns differently based
dumps. Add `--max-value 0xf` to restrict both states to small values.

## Live utilities and history

`tools/dump_mem.ps1` and `probe_actions.ps1` read live process memory.
`tas_shm.ps1` can also send commands; `keys.ps1` sends keyboard input. These are
manual diagnostics, not substitutes for the Rust live suite.
`test_dll_hidden.ps1` runs native tests without stealing game focus.
Pico firmware and deployment have their own [README](pico/README.md).

History persistence is implemented in `tas_ui/src/history_store_v2.rs`:
`manifest.json` stores ordered metadata and the current entry ID, with immutable
ID-named `.tasrec` blobs and per-entry CRC32 checksums. Metadata edits do not
require rewriting recording blobs. See its tests and the recording/UI tests for
migration, recovery and eviction behavior. The former history and test-coverage
implementation plans have been retired; Git retains their historical details.
