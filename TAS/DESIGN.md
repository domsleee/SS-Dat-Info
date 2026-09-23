# TAS system design

How the Supreme Snowboarding TAS (tool-assisted speedrun) tooling works: what
each piece does, how a run is recorded and replayed, and why the design looks
the way it does. It covers the work added in PR #41. For build and deploy
commands see the [TAS README](README.md); for the test contract see
[TAS quality gates](../docs/tas-quality-gates.md).

## What it does

A player can:

- **REC**: record a run. Every game tick, the pressed keys and the boarder's
  X/Y/Z position are logged.
- **PLAY**: restart the level and replay a recording, aiming to reproduce the
  original run bit for bit.
- **CONT** (continue): replay a recording up to a chosen tick at high speed,
  then switch to recording from exactly that point. This is how a run is
  improved piece by piece: keep the good start, redo the rest.
- **Edit** the inputs on a timeline or in a text script, keep an undo/redo
  history of every take, and recover a take if the game or the UI crashes.

There are no savestates. The game can't save and restore its physics state,
so every PLAY and CONT restarts the level and re-simulates from the first
tick with the recorded inputs. Most of the design exists to make that
re-simulation match the original exactly.

### Terms

- **Tick**: one step of the game's physics. The game runs 100 ticks per
  second of game time. Several ticks can run between two rendered frames (or
  none, when paused). Some fields and comments say "frame" when they mean
  tick.
- **Controller**: the program driving the game, either the UI or the test
  harness. Only one may drive a game at a time.
- **Arm**: tell the DLL to start REC, PLAY or CONT.
- **Transport cycle**: the stop, restart, arm and (for a replay) check
  sequence that starts REC, PLAY or CONT, including any retries.
- **Gate**: the first tick where the boarder leaves the spawn point.
- **Splice**: the tick where a CONT switches from replaying to recording.

## The pieces

```
 Display_Config (launcher)        tas_ui.exe  "SSB Inspect"         tas_test.exe
 "Enable TAS" checkbox            egui desktop app                  live test harness
        |                                 |                                |
        | Injector.exe                    |  shared memory                 | shared memory
        v                                 v  Local\SupremeTAS              v  + Pico HID keys
 +------------------------------------------------------------------------------+
 | Supreme.exe (32-bit, v1.035)                                                 |
 |   TAS_Helper.dll: hooks in the game loop and input pipeline,                 |
 |   level/rider/renderer detection, race timer, menu reader                   |
 +------------------------------------------------------------------------------+
```

| Component | Language | Role |
| --- | --- | --- |
| `TAS_Helper/` | C++ (x86 DLL) | Injected into the game. Records and injects input, captures position, controls game speed. |
| `tas_shared/` | Rust | The shared-memory protocol, plus logic the UI and harness must agree on: the transport state machine, gate alignment, level/rider/renderer decoding, the menu channel. |
| `tas_ui/` | Rust (egui) | The desktop app "SSB Inspect": transport buttons, timeline editor, history, crash recovery. |
| `tas_codec/` | Rust | The `.tasrec` file format, shared by UI and harness. |
| `tas_test/` | Rust | Live test harness that drives the real game. |
| `pico/` | CircuitPython | Firmware for a Raspberry Pi Pico acting as a USB keyboard for live tests. |
| `Display_Config` changes | Rust/Vue/C++ | The "Enable TAS" launcher option, and an injector that runs the DLL's initializer. |

### Launch

With **Enable TAS** ticked, the launcher's Play button runs `Injector.exe
TAS_Helper.dll` once the game is running
(`Display_Config/Display_Config/src-tauri/src/inject.rs`). The injector loads
the DLL into `Supreme.exe`, then calls its exported `TAS_Initialize` in a
second remote thread, so the real setup doesn't run under the Windows loader
lock. It exits non-zero if either step fails. The launcher then waits up to
5 s for the shared memory to appear and starts `tas_ui.exe`. (The DLL creates
the shared memory before it installs hooks, so the injector's exit code is
the real success signal.)

`TAS_Initialize` (`TAS_Helper/src/main.cc`):

1. Checks that the game modules are exactly the expected v1.035 build
   (`game_addresses.hpp`). Every patch is at a fixed address, so any other
   build is refused.
2. Creates the shared memory. A named mutex, `Local\SupremeTAS.Owner`, stops
   a second injected game from taking it over.
3. Installs the five core hooks. They are all-or-nothing: if any fails, all
   are rolled back and initialization fails.
4. Starts the extras (level scanner, race timer, menu reader). These are
   best-effort and only log if they can't start.
5. Pins the DLL in memory so it can't be unloaded while the game holds
   pointers into it. If pinning fails, it logs a warning.

## How a replay stays exact

This section explains the ideas. The DLL and shared-memory sections below
explain the machinery.

### Recording

In REC the DLL logs, every tick, one byte of input (LEFT, RIGHT, UP, DOWN,
jump, SHIFT) and the boarder's position. Up to 65,536 ticks fit, about 10 min
55 s. The positions aren't needed to replay the run; they are the reference
a replay is checked against.

### Replaying from the gate

After a restart there's a countdown at the spawn point, and it isn't the same
length every time: it depends on exactly when the arm lands relative to the
restart (up to four ticks apart in tests). If input were counted from the
arm, a countdown one tick longer would shift every input by a tick and the
run would drift.

So input is counted from the **gate**, the first tick where the boarder's
position differs from the spawn (`gate_alignment.hpp`,
`tas_shared/src/align.rs`):

- Before arming, the controller finds the recording's gate from its positions
  and writes it to the DLL (`gate_align_rec`).
- The DLL notes the live replay's gate (`gate_index`) at the end of the tick
  where the boarder first moves.
- From then on, replay tick `p` plays recorded input
  `p - gate_index + gate_align_rec`.
- Before the live gate, the replay plays the recording's own inputs, up to 8
  ticks before the recording's gate. From there until the live gate fires,
  it holds whatever input the recording had at its gate. The input for the
  gate tick has to be chosen before anyone knows it's the gate tick, and
  holding it early makes it land wherever the gate falls.

The hold has a cost: any key changes the recording made in those 8 ticks
reached the game's key observer during recording but are replaced during
replay, which can cause divergence. The UI warns when a recording has such
changes.

A recording that never moves has no gate. It's replayed from the arm, with no
alignment and no check.

### Checking the replay (the watcher)

The gate fixes timing, but not everything about the spawn is visible. So the
controller watches: it compares the replay's positions to the recording's,
gate-relative, bit for bit, for up to 1,024 ticks after the gate (fewer if the
recording or the CONT prefix is shorter). If they differ, or a position read
failed, it restarts and tries again (a **reroll**), up to 30 times, then
gives up. After 1,024 matching ticks the replay is accepted. Later
divergence isn't caught by the watcher. The UI's drift banner compares X/Z
positions as a separate diagnostic.

### The transport cycle

`tas_shared/src/transport.rs` implements the stop, restart, arm, check
sequence. It drives every REC, PLAY and CONT started from the UI, and the
harness's tests of that same workflow (some harness tests send commands
directly instead):

```
STOP (STOP_FOR_RESTART for an aligned replay)
  → wait until the DLL is OFF and has taken the command → 50 ms
  → RESTART → wait until the restart has finished → 10 ms
  → write the splice tick and gate → ARM
  → REC, or no gate: done
  → otherwise watch: compare positions → done | reroll (back to STOP) | give up
```

It's a non-blocking stepper: each `step()` does at most one transition. The UI
steps it from its frame loop and the harness from a poll loop, so both run the
same sequence. The playback speed is rewritten on every step because the
restart resets it. The UI abandons a cycle after 180 s.

### CONT: replay, check, then record

CONT is restart → replay the prefix fast and check it → resume recording at
the splice. For a recording with a gate:

1. The UI saves the current speed and switches to the catch-up speed. It sets
   `cont_suppress_input`, which makes the DLL ignore every live key except
   Escape until the cycle ends, including through the restart and countdown.
   A key held during the restart would otherwise change the spawn. (Aligned
   PLAY sets it too.)
2. The transport cycle restarts and arms CONT. The DLL replays the prefix,
   gate-aligned. It trims fast-forward tick batches so they stop exactly at
   the splice. If the splice is past the gate, then until the live gate
   fires, batches stop at the start of the 8-tick pre-gate window and then
   run at most one tick per frame, so a batch can't overshoot a splice just
   past the gate.
3. **The splice waits for approval.** If playback reaches the splice before
   the watcher has approved, the DLL runs zero ticks per frame ("parks")
   until the controller sets `cont_splice_approved`. The controller approves
   when the watcher's check passes (or, for a splice inside the countdown,
   when the spawn position matches). A slow or crashed controller delays the
   splice; it can't let an unchecked prefix through.
4. On approval the transport cycle is done and the UI clears
   `cont_suppress_input`. For a splice far into the run, the watcher approves
   long before playback gets there; PLAY mode keeps live keys out until then.
5. At the splice, the DLL cuts the recording to the splice tick, switches to
   the saved normal speed, marks a segment boundary and switches to REC. The
   recording keeps the original's tick numbering: the new take is the old
   prefix followed by new input.
6. The DLL moves the game clock forward to "now" without running the ticks
   the catch-up owed, so recording starts in real time with no burst.
7. The UI sees REC, starts a new session, and the player carries on live.

If anything goes wrong, `cont_suppress_input` is also cleared on abort,
cancel and disconnect. As backstops, the DLL clears it after 5 s with the
game loop frozen, and the UI clears it if it's been left set for 10 s with
nothing armed.

Only a successful CONT arm can splice. The permission flag lives inside the
DLL, so a stray splice value in shared memory can't turn a plain replay into
a recording.

### What else must match

The same inputs give the same run only if the physics are the same. Two
things change them, so the DLL detects them and recordings are stamped with
them:

- **Rider**: character and stance.
- **Renderer**: DirectX runs the game's x87 floating-point math at 24-bit
  precision, OpenGL at 53-bit, so every calculation rounds differently.

The UI's status card shows a mismatch between the loaded recording and the
live game, and starting PLAY or CONT also logs advice on a rider mismatch.
The UI can't fix a mismatch: stance and character are chosen in the game's
menus, and the renderer at launch.

Separately, the DLL's own hooks must not disturb the x87 registers, or they
would change the physics themselves. The per-tick hooks save and restore
them.

## Inside the DLL

The patches are called "caves" (code caves) for historical reasons.

### How game input works, and how the DLL feeds it

A real keypress travels: Windows message → the game's key handlers (key down
at `+3940`, key up at `+3980`), which set a byte in the game's key buffer →
`BB3B10`, the keyboard **observer**, which must be told about the change for
steering to work.

Each key event carries a game timestamp, and the observer silently ignores
events stamped before the current race. So the DLL stamps injected events
with the game's own current time. It zeroes the low half of the stamp, which
the observer doesn't check, so a recording and its replay produce identical
stamps (as long as both fall in the same roughly 7-minute window of the high
half).

### The hooks

| Hook | Where | Job |
| --- | --- | --- |
| **Cave 2** | `Supreme::Cycle` | Runs once per tick. Applies commands and records or replays that tick's input. |
| **Cave 1C** | key handlers | Blocks real key events during REC and PLAY, so input only enters through Cave 2. |
| **Cave 1D** | observer `BB3B10` | Blocks real observer calls during REC and while a CONT is in flight. |
| **Cave 5** | tick loop in `Supreme.exe` | Decides how many ticks run each rendered frame: speed control, pause catch-up, parking at a splice. |
| **Replay capture** | the game's replay recorder | Finds the human player's object so Cave 2 can read its position. Ignores AI and ghost riders. |

Each tick, Cave 2:

1. Applies a pending command (arm, stop, restart).
2. In a CONT whose approved splice is already due, switches to REC before
   doing anything else, so this tick is recorded, not replayed.
3. In REC, reads the keyboard; in PLAY, reads the logged input for this tick.
4. Writes that input into the key buffer and the key state, and calls the
   observer for each key that changed.
5. Logs the input (REC) and the position (REC and PLAY).
6. After a PLAY tick, checks again whether a CONT splice can complete.

(`tas_test`'s `play-pace` check confirms playback advances 100 ticks per
second at 1x, i.e. one Cave 2 call per tick.)

**REC injects too.** You might expect REC to just watch the game. It
doesn't: real key events are blocked during REC as well, and Cave 2 reads the
keyboard itself (`GetAsyncKeyState`) and injects the result exactly as PLAY
would. REC and PLAY then deliver input at the same point in the same tick,
so there's no one-tick difference between recording and replaying.

Exceptions to the blocking: Escape always gets through (pause menu, abort).
The REC/PLAY block lifts after the game loop has been still for 250 ms
(paused, or in a dialog) so menus stay usable. The CONT block
(`cont_suppress_input`) doesn't lift when paused.

**Position capture.** Each tick Cave 2 copies the boarder's X/Y/Z into
`rec_coords` (REC) or `play_coords` (PLAY). If a read fails, the tick still
counts and `capture_ok` drops to 0 for the rest of that arm; the watcher
treats that as a failed check. The flag isn't saved with the recording.

### Speed control (Cave 5)

Each rendered frame the game works out how many ticks are owed (time since
its clock was last advanced, times 100), runs them, and advances its clock by
0.01 s per tick. Cave 5 changes speed by changing only that 0.01. At 0.005,
each tick pays back half the time it should, so twice as many are owed and
the game runs at 2x; at 0.04 it runs at 0.25x.

- The menus read the same constant, so the DLL points the game loop's reads
  at a private copy instead of changing the original. (If that can't be done,
  it changes the original and logs that the menu will run fast.)
- The game's limit of 20 ticks per frame is raised to 64 for fast-forward.
  At 1x the original 20 still applies.
- After a pause the game would run the whole paused time as a burst of
  ticks. Cave 5 runs one tick that absorbs the gap instead.

### F5 restart

A replay must start from a freshly restarted level, so the DLL restarts it by
pressing F5 in the key buffer (`RESTART`). No real key press or window focus
is needed. F5 must be released as soon as the game acts on it: the game
restarts again on every input poll that sees F5 down, and overlapping
restarts corrupt the heap and crash the game later. The replay-capture hook
sees the new player object the restart creates and releases F5 right then;
a 25 ms timer and a 30-tick cap are fallbacks (`restart_release.hpp`).

### Stopping

A STOP command ends either mode. PLAY also stops at the end of the
recording, and REC when the buffer is full. The DLL stops an armed mode when
a running tick finds the level has been replaced (a new player root). If the
game loop stays frozen instead, as at the main menu, the UI sends STOP after
5 s. Every stop releases the keys the DLL was holding, so the boarder doesn't
keep steering.

The game loop doesn't run at the menus. Anything that must work there runs on
the DLL's background thread instead of in a game-loop hook: consuming a STOP,
noticing a level change, clearing a stale input block.

### Detecting the level, rider and renderer

A background thread (`level_scan.hpp`, about 10 Hz) publishes:

- **Level**: the area from the game's current level resource path (whose
  change also signals a level load), and the difficulty from the game's setup
  object, because some tracks share assets.
- **Rider**: character and stance.
- **Renderer**: which renderer plugin loaded. (Cave 2 separately publishes
  the game thread's x87 control word, which gives the precision.)

Two more readers:

- **Race timer** (`race_timer.hpp`): hooks the HUD text renderer and reads the
  on-screen race time, so the UI shows exactly what the game shows.
- **Menu reader** (`menu_state.hpp`): publishes the current menu page as a
  small JSON document (items, labels, ids) and carries out commands (focus,
  activate, up/down) through the game's own menu code, on the menu's own
  thread. A command names the page it was read from and is refused if the
  page has changed; unconsumed commands expire after 3 s. The live tests use
  it to navigate menus without screen scraping.

## Shared memory

The DLL and the Rust programs talk only through one ~1.6 MB named mapping,
`Local\SupremeTAS`. Its layout is `TasSharedState` in
`TAS_Helper/src/shared_state.hpp`, mirrored field for field in
`tas_shared/src/state.rs`. Both sides pin the total size and key offsets (C++
with `static_assert`, Rust in a unit test), and a version number is bumped on
any change, so a mismatched DLL/UI pair fails loudly instead of misreading
memory.

What's in it:

- **The command slot**: the controller writes one command (`ARM_REC`,
  `ARM_PLAY`, `ARM_CONTINUE`, `STOP`, `RESTART`, `STOP_FOR_RESTART`) and the
  DLL resets it to `IDLE` once applied. There's no queue: a new command
  overwrites an unconsumed one, so controllers wait for `IDLE`. For
  `RESTART`, `IDLE` means the restart has begun; `restart_state` reaching 2
  means it's done.
- **Status**: `mode` (OFF, REC or PLAY; CONT is PLAY until the splice),
  `recorded_count`, `playback_pos`, live position and velocity.
- **The recording itself**: `input_log`, `rec_coords` and `play_coords`. The
  recording lives here, not inside the DLL: loading a file copies it into
  these arrays, and editing inputs writes straight into `input_log` while the
  game is stopped.
- **Context**: level, rider, renderer and precision, race time, menu page.
- **A log ring** the DLL writes and the UI displays.

### Concurrency

Two processes share this memory without locks, so it relies on conventions:

- The controller writes commands and their parameters; the DLL writes
  status. The UI writes the recording arrays and `recorded_count` only while
  the DLL is stopped. A few flags (`cont_suppress_input`, the alignment
  fields) are set by the controller and cleared by the DLL.
- The command word is written last. The controller writes the parameters
  first and the command last; the DLL writes all resulting status first and
  resets the command last. The command is written and read with atomic
  operations, so whoever sees it change also sees the writes before it. This
  orders a command and its results; it doesn't make the other live status
  fields one consistent snapshot.
- Fields read as a group (level context, rider, race time, menu document) are
  protected by **seqlocks**: the writer makes a counter odd, writes, then
  makes it even; a reader retries until it sees the same even value before
  and after, and reports "unknown" after a few failed tries. The level id is
  trusted only when its scan matches the currently loaded level, not the one
  before.
- `arm_generation` is bumped as the last write of every PLAY or CONT arm,
  even a refused one. Until it changes, `mode` and `playback_pos` still
  describe the *previous* replay, so the watcher waits for it.
- STOP must work while the game loop is frozen. The game-loop hook and the
  background thread can both consume it, and an atomic claim ensures only one
  does.

Nothing enforces a single controller. The harness closes any running
`tas_ui` before it starts. The menu reader uses a separate command channel
of its own (sequence, target, acknowledgement, result).

## SSB Inspect (tas_ui)

The UI's responsibilities:

- **Transport**: REC, PLAY, CONT and STOP buttons, also on F9 to F12 while
  the game has focus. Each start runs the transport cycle above.
- **Sessions**: when REC starts it opens a session; when REC stops it saves
  the take to history. It stops REC/PLAY if the game loop has been frozen for
  5 s (usually the player quitting to the menu; a long pause also counts).
- **Editing**: inputs appear as held-key spans on a timeline
  (`panels/timeline.rs`), or in a TMInterface-style text script
  (`324-372 press left`, ticks, end exclusive) that reloads on save
  (`panels/input_script.rs`). Edits apply only while stopped; the UI stops a
  running session first.
- **Finish line**: start and finish trigger planes come from level data
  (`start_line.rs`, generated for the nine main tracks, Practice added by
  hand). While recording on a known track, the UI sends STOP once the run
  crosses the finish, since the run-out is never wanted. The STOP lands a few
  ticks after the crossing.
- **Drift banner** (`drift_scan.rs`): compares replay and recording X/Z
  positions as they arrive and shows where a replay drifted. It's a
  diagnostic, separate from the watcher.
- **Default file names**: level code plus run length in centiseconds from
  the gate to the end, e.g. `FE-5876.tasrec` for 58.76 s on Forest Easy.
- **History, crash recovery and relaunch**, below.

**Editing changes inputs, not positions.** The stored positions stay those
of the original take, and the watcher keeps comparing against them. An edit
that changes the run within the first 1,024 ticks after the gate makes PLAY
reroll until it gives up. A CONT is only affected by edits in the prefix it
replays (including the 8 ticks before the gate).

### Recording files (`.tasrec`)

```
[u32 header length][JSON header][input bytes, 1 per tick][f32 X, Y, Z per tick]
```

All numbers are little-endian. The JSON header holds `recorded_count` (which
sets both body lengths), segments, and the rider and physics stamps. Loaders
ignore unknown header fields. `tas_codec` owns the format: it rejects
oversized headers (over 1 MiB), truncated bodies and non-finite positions,
and writes files atomically. Old input-only files still load in the UI
(positions zeroed); the test harness requires positions.

### History

Every take and every edit becomes a history entry with undo/redo, pinning,
per-level filtering, and a soft cap of 500 unpinned entries by default
(`recording/history.rs`). On disk it's under `<data dir>/history/`
(`history_store.rs`):

- `manifest.json`: the ordered entries (labels, level, stamps, CRC32
  checksums), the cursor, and the next entry id.
- One immutable `<id>.tasrec` per entry that holds a recording (markers,
  such as save points, have none). Despite the extension, these are **not**
  `.tasrec` files: they're a bare `u32 count | inputs | f32 X, Y, Z` with no
  JSON header.

Saving writes new blobs first, then the manifest, then deletes blobs nothing
references any more. Metadata edits rewrite only the manifest. Blobs are
verified when restored; a corrupt one is quarantined, never deleted, and its
id is never reused. If the manifest itself is unusable or from an
incompatible version (such as an older bincode store), the blobs are kept as
`*.tasrec.orphaned` for manual recovery. All writes go through one background
worker that coalesces bursts; its `flush` reports whether the data actually
reached disk.

The data directory is `SSB_INSPECT_DATA_DIR` if set, otherwise `data/` next
to a deployed `tas_ui.exe`, or `~/.ssb-inspector` for a dev build.

### Crash recovery

While recording, a background writer saves the whole take to
`recovery_checkpoint.tasrec` (recording and session details in one atomic
file). Each save rewrites the whole take, so saves get less frequent as it
grows: at most every 1.5 s for the first 12,000 ticks (2 minutes), every 5 s
up to 48,000 ticks, then every 10 s. A crash normally loses at most one
interval; more if the UI stalls or a save fails.

When REC stops, the take goes into history, and the checkpoint is deleted
only once history confirms the write reached disk. If the history write
fails, the checkpoint is kept and comes back as a pinned entry on the next
start. STOP doesn't save a final checkpoint, so that entry can miss the last
few seconds.

### Relaunch

If the game dies or restarts while the UI stays open (`relaunch.rs`), the UI
still has the old shared memory mapped. If it still holds the dead game's
data, the UI saves the take from it. If the new DLL has already reset it, the
UI falls back to the checkpoint. It also clears input blocks left by a
controller that died mid-cycle.

A relaunch usually produces two signals, in either order: the game's process
id changes, and the DLL's tick counter goes backwards. The UI handles them as
one event. At the main menu the counter may never have moved, so the process
id alone has to catch it.

## Testing

### Offline (CI)

`just check_all` runs locally and in the `tas:test` CI job:

- Rust unit tests. The transport cycle runs against a fake game;
  Windows tests use private shared memory, never the game's.
- Standalone x86 C++ tests of the DLL's decision logic (input blocking, gate
  alignment, replay capture, level path, rider, menu model, race timer,
  setup parsing). They run with hidden windows, because a console window
  taking focus pauses the game.
- Python tests for the level tools and the Pico firmware (simulated USB and
  serial faults).
- `cargo fmt` and Clippy, warnings as errors.

CI also builds the DLL, the injector and the Rust binaries and checks the
release layout.

### Live (developer machine)

`tas_test` drives the real game. `just test_live` is a short gate.
`just test_live_full` runs 23 stages (`tas_test/src/live_suite.rs`): replay,
reliability, pause and resume, speed and pace, CONT variants, input
protection, level sequence, save and reload, menu dialogs and more.
`just test_live_soak` repeats the same checks more times. Live tests replace
the game's current recording, so save your run first.

**Why a Pico.** REC reads the keyboard with `GetAsyncKeyState`, which reports
physical key state. Messages posted to the game window don't change it, so a
test of REC needs real key presses. A Raspberry Pi Pico 2 acting as a USB
keyboard provides them: its keys reach Windows and the game exactly like a
person's. Normal use doesn't need one, since PLAY injects inside the game
and the DLL presses F5 itself.

The harness sends the Pico one byte per change of held keys over its serial
data port, plus a few reserved bytes for Enter, reset and a health check.
Keys release on their own 500 ms after the last byte, so a crashed harness
can't leave a key held; the harness resends held keys every 200 ms to keep
long holds going. The [Pico README](pico/README.md) has the protocol and the
update procedure.

The recordings in `recordings/` are the fixtures the live tests replay and
splice.

## Pitfalls

- **Positions are compared as raw float bits.** Anything that changes
  rounding (renderer, rider, a hook that disturbs the FPU) breaks replays.
- **An injected key with the wrong timestamp is silently ignored**, and one
  stamped in the future makes later keys look out of order and get dropped.
- **Holding F5 crashes the game later**, not immediately.
- **Two controllers on one game will confuse each other.** The command slot
  and `arm_generation` assume one.
- **History blobs aren't `.tasrec` files**, despite the extension.

## Known limits

- Works only with Supreme Snowboarding v1.035.
- One game with the DLL at a time (the shared memory name is fixed).
- Recordings are at most 65,536 ticks (about 10 min 55 s).
- Switching between two tracks that share a level path (Village Easy and
  Village Hard) changes no path; detection relies on the game loop pausing
  during the load instead.
- The watcher checks the first 1,024 ticks after the gate; later divergence
  is only visible in the drift banner.
