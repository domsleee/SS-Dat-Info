# TAS system design

How the Supreme Snowboarding TAS (tool-assisted speedrun) tooling works: what
each piece does, how a run is recorded and replayed, and why the design looks
the way it does. For build and deploy
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
   (`game_addresses.hpp`). Every patch is at a fixed offset into its module
   (the game's DLLs are relocated at load, so offsets, not addresses), so any
   other build is refused.
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

After a restart there's a countdown at the spawn point, and the first moving
tick isn't at the same arm-relative tick every time: it depends on when the
arm lands relative to the restart, and on whether a Time Attack ghost is
racing. If input were counted from the arm, a gate one tick later would
shift every input by a tick and the run would drift.

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
  → wait until the DLL is OFF and has taken the command
  → RESTART → wait until the restart has finished
  → write the splice tick and gate → ARM → wait for arm_generation to move
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
6. The game clock is still behind wall time from the catch-up. The splice
   tells the tick cave to drain that backlog as a single tick on the next
   frame, at any resume speed, so recording starts at its own pace with no
   burst (`tas_test cont-resume-pace`).
7. The UI sees REC, starts a new session, and the player carries on live.

If anything goes wrong, `cont_suppress_input` is also cleared on abort,
cancel and disconnect. As backstops, the DLL clears it after 5 s with the
game loop frozen, and the UI clears it if it's been left set for 10 s with
nothing armed (a check it skips while `tas_test` is running).

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

The code calls its patches "caves" (code caves).

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
| **Cycle cave** | `Supreme::Cycle` | Runs once per tick. Applies commands and records or replays that tick's input. |
| **Key-handler cave** | key handlers | Blocks real key events during REC and PLAY, so input only enters through the cycle cave, and whenever a CONT is in flight (`cont_suppress_input`), even with the mode OFF. |
| **Observer cave** | observer `BB3B10` | Blocks real observer calls during REC and while a CONT is in flight. |
| **Tick cave** | tick loop in `Supreme.exe` | Decides how many ticks run each rendered frame: speed control, pause catch-up, parking at a splice. |
| **Replay capture** | the game's replay recorder | Finds the human player's object so the cycle cave can read its position. Ignores AI and ghost riders. |

Each tick, the cycle cave:

1. Applies a pending command (arm, stop, restart), steps an in-progress F5
   restart, and stops an armed mode if the level has been replaced.
2. In a CONT whose approved splice is already due, switches to REC before
   this tick's input is chosen, so this tick is recorded, not replayed.
3. In REC, reads the keyboard; in PLAY, reads the logged input for this tick.
4. Writes that input into the key buffer and calls the observer for each
   key that changed.
5. Logs the input (REC) and the position (REC and PLAY).
6. After a PLAY tick, checks again whether a CONT splice can complete.

(`tas_test`'s `play-pace` check confirms playback advances 100 ticks per
second at 1x, i.e. one cycle-cave call per tick.)

**REC injects too.** You might expect REC to just watch the game. It
doesn't: real key events are blocked during REC as well, and the cycle cave reads the
keyboard itself (`GetAsyncKeyState`) and injects the result exactly as PLAY
would. REC and PLAY then deliver input at the same point in the same tick,
so there's no one-tick difference between recording and replaying.

Exceptions to the blocking: Escape always gets through (pause menu, abort).
The REC/PLAY block lifts after the game loop has been still for 250 ms
(paused, or in a dialog) so menus stay usable. The CONT block
(`cont_suppress_input`) doesn't lift when paused.

**Position capture.** Each tick the cycle cave copies the boarder's X/Y/Z into
`rec_coords` (REC) or `play_coords` (PLAY). If a read fails, the tick still
counts and `capture_ok` drops to 0 for the rest of that arm; the watcher
treats that as a failed check. The flag isn't saved with the recording.

### Speed control (tick cave)

Each rendered frame the game works out how many ticks are owed (time since
its clock was last advanced, times 100), runs them, and advances its clock by
0.01 s per tick. The tick cave changes speed by changing only that 0.01. At 0.005,
each tick pays back half the time it should, so twice as many are owed and
the game runs at 2x; at 0.04 it runs at 0.25x.

- The menus read the same constant, so the DLL points the game loop's reads
  at a private copy instead of changing the original. (If that can't be done,
  it changes the original and logs that the menu will run fast.)
- The game's limit of 20 ticks per frame is raised to 64 for fast-forward.
  At 1x and slower the original 20 still applies.
- After a pause the game would run the whole paused time as a burst of
  ticks. The tick cave runs one tick that absorbs the gap instead (this path
  bypasses the 20-tick limit).

### F5 restart

A replay must start from a freshly restarted level, so the DLL restarts it
with the game's own F5 handling (`RESTART`, `caves/f5_restart_cave.hpp`); no real
key press or window focus is needed. Once per loop iteration, before the tick
batch, the game reads F5 from the key buffer and, if the race accepts a
restart, rebuilds the level on the spot. Two hooks follow it:

- **Accept** (`Supreme.exe+0x25C3F`, the accepted-F5 branch) takes the key
  back up the moment the game acts on it, for a real key too. The game would
  otherwise restart again on every poll that still sees F5 down.
- **Done** (`Supreme_Game+0x14199F`, after `Set_Game_Mode` has rebuilt the
  player and its recorder) marks the restart complete. `restart_state`
  becomes 2 on the next tick.

Until Done, the cycle cave keeps F5 down, so a restart the game refused is taken on
its next poll. A STOP lets go of the key. `tas_test restart-stress` runs
hundreds of restarts in one session.

### Stopping

A STOP command ends either mode. PLAY also stops at the end of the
recording, and REC when the buffer is full. Leaving the race (quitting to the
menu, switching track) stops an armed mode too. Every stop releases the keys
the DLL was holding, so the boarder doesn't keep steering; pausing doesn't
stop anything.

### The race lifecycle

`caves/lifecycle_cave.hpp` follows the game's own transition points, all on the
game thread:

| Hook | Where | Job |
| --- | --- | --- |
| **Launch** | `Supreme.exe+0x25BD7`, after the race loop enters the level | Identifies the track and publishes it; `game_in_game` = 1. Rider and renderer stamps refresh on the race's first tick. |
| **Stop** | `Supreme::Stop` (`Supreme_Game+0x1408F0`) | Stops an armed mode while the level still exists; publishes "no race"; `game_in_game` = 0. |
| **Pump** | the game's message pump (`Supreme.exe+0x55920`) | Runs in every state, including the menus, the pause menu and dialogs where `Supreme::Cycle` doesn't. Consumes STOP there, clears a stale input block, expires menu commands and writes the DLL's queued log lines. |

The level is the area from the game's level resource path plus the
difficulty from the game's setup object (`level_context.hpp`), because some
tracks share a path: Village Hard loads `.../Tracks/easy/...`. It is
identified at every launch, so switching between two such tracks is seen
like any other. A DLL injected into a race already running identifies it on
the race's first tick.

- **Rider**: character and stance.
- **Renderer**: which renderer plugin loaded. (The cycle cave separately publishes
  the game thread's x87 control word, which gives the precision.)

Two more readers:

- **Race timer** (`race_timer_cave.hpp`): hooks the HUD text renderer and reads the
  on-screen race time, so the UI shows exactly what the game shows.
- **Menu reader** (`menu_cave.hpp`): publishes the current menu page as a
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
  `RESTART`, `IDLE` means the restart has begun; `restart_state` 2 means the
  game has rebuilt the level (see F5 restart).
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
  the DLL is stopped. A few flags are written by both sides: the controller
  sets the alignment fields and the DLL clears them, and
  `cont_suppress_input` is set and cleared by the UI and also by the DLL
  (`STOP_FOR_RESTART` sets it, a plain STOP clears it).
- The command word is written last. The controller writes the parameters
  first and the command last; the DLL writes all resulting status first and
  resets the command last. The command is written and read with atomic
  operations, so whoever sees it change also sees the writes before it. This
  orders a command and its results; it doesn't make the other live status
  fields one consistent snapshot.
- Fields read as a group (level context, rider, race time, menu document) are
  protected by **seqlocks**: the writer makes a counter odd, writes, then
  makes it even; a reader retries until it sees the same even value before
  and after, and reports "unknown" after 64 failed tries. The level id is
  trusted only when its scan matches the currently loaded level, not the one
  before.
- `arm_generation` is bumped as the last write of every arm (REC, PLAY or
  CONT), even a refused one. Until it changes, `mode` and `playback_pos` still
  describe the *previous* session, so the controller waits for it before it
  reports an arm done or judges a replay.
- STOP must work while the game loop is frozen. The cycle cave consumes it while a
  race ticks and the message-pump hook everywhere else; both run on the game
  thread, so all DLL-side state has a single writer thread.

Nothing stops the UI and the harness driving the same game at once; the
harness closes any running `tas_ui` before it starts. (`tas_ui` does refuse
a second instance of itself.) The menu reader uses a separate command channel
of its own (sequence, target, acknowledgement, result).

## SSB Inspect (tas_ui)

The UI's responsibilities:

- **Transport**: REC, PLAY, CONT and STOP buttons, also on F9 to F12 while
  the game has focus. Each start runs the transport cycle above.
- **Sessions**: when REC starts it opens a session; when REC stops, including
  when the DLL stops it because the race was left, it saves the take to
  history.
- **Editing**: inputs appear as held-key spans on a timeline
  (`panels/timeline.rs`), or in a TMInterface-style text script
  (`324-372 press left`, ticks, end exclusive) that reloads on save
  (`panels/input_script.rs`). Edits apply only while stopped; the UI stops a
  running session first.
- **Finish line**: start and finish trigger planes come from level data
  (`start_line.rs`) for the nine main tracks and Practice. While recording on a known track, the UI sends STOP once the run
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
incompatible version (wrong magic, schema, blob format or hash), the blobs are kept as
`*.tasrec.orphaned` for manual recovery. All writes go through one background
worker that coalesces bursts; its `flush` reports whether the data actually
reached disk.

The data directory is `SSB_INSPECT_DATA_DIR` if set, otherwise `data/` next
to `tas_ui.exe` when it's deployed (its folder is inside
`Display_Config_Resources`), otherwise `~/.ssb-inspector`. Settings are the
exception: without the variable they're stored next to the exe in every
build.

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
UI falls back to the checkpoint. It releases an input block its own cycle
left set; a block left by another controller that died mid-cycle is cleared
by the 10 s backstop.

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
  serial faults), and a PowerShell test of Pico discovery.
- `cargo fmt` and Clippy, warnings as errors.

CI also builds the DLL, the injector and the Rust binaries and checks the
release layout.

### Live (developer machine)

`tas_test` drives the real game. `just test_live` is a short gate.
`just test_live_full` runs 24 stages (`tas_test/src/live_suite.rs`): replay,
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
- **Holding F5 restarts the level on every input poll.** Only
  `f5_restart_cave.hpp` writes F5, and its accept hook takes the key up the
  moment the game acts on it.
- **The game's DLLs are always relocated**, so a byte signature must stop
  before any absolute address inside the instruction it checks.
- **Writes into game memory must stay inside the target object's
  allocation.** Check offsets against the allocation size in the decompiled
  source (the keyboard object is 0x38 bytes); a stray write corrupts
  whatever follows it and crashes much later, somewhere else.
- **SafetyHook mid-hooks don't save the x87 state.** A hook can sit in the
  middle of the game's physics with live values on the FPU stack, so every
  mid-hook body runs between FSAVE and FRSTOR (`fpu_safe_hook.hpp`). FSAVE
  also re-initialises the FPU, so code inside a hook runs at 64-bit
  precision; FRSTOR puts the game's own precision back before its physics
  continues. The game's control word is read from the saved image, not with
  `fnstcw` inside the hook. Injected input was checked against native keys on
  a precision-sensitive fence hit on Village Medium: bit-identical.
- **`Supreme::Cycle` doesn't run at the menus, the pause menu or dialogs.**
  Anything that must work there belongs in the message-pump hook.
- **The decompiled source mislabels arguments and control flow.** Ghidra
  drops register arguments (`Supreme::Enter` takes its start info on the
  stack, with objects in ECX/EDX) and marks allocations as not returning;
  check a hook site against the disassembly.
- **Two controllers on one game will confuse each other.** The command slot
  and `arm_generation` assume one.
- **History blobs aren't `.tasrec` files**, despite the extension.

## Known limits

- Works only with Supreme Snowboarding v1.035.
- One game with the DLL at a time (the shared memory name is fixed).
- Recordings are at most 65,536 ticks (about 10 min 55 s).
- An in-process restart waits for the game to take F5, which it doesn't do
  in the pause menu or a dialog; the controller's timeout then gives up.
- The watcher checks the first 1,024 ticks after the gate; later divergence
  is only visible in the drift banner.
- Editing inputs keeps the original take's positions, so an edit inside the
  watched window makes PLAY reroll until it gives up.
