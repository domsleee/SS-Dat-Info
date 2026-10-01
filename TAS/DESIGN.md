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
jump, SHIFT) and the boarder's position. The input is the keys the game
itself held for that tick's physics, read from its keyboard observer (see
"REC observes, PLAY writes" below). Up to 65,536 ticks fit, about 10 min
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
- The live gate is known before it happens. The countdown is a float on the
  rider (`[player+0x154]`) that starts at 0 when the rider is reset and grows
  by 0.01 every tick; the rider is released on the 302nd tick, and that tick
  is exactly the gate, with or without ghosts (`tas_test countdown-anchor`:
  the arm lands 4-5 ticks after the reset without ghosts and 14-15 with, and
  the gate is the release tick every time). On an aligned replay's first
  tick the DLL reads the countdown and predicts the gate.
- Replay tick `p` plays recorded input `p - gate + gate_align_rec` from the
  first tick, countdown included. A tick before the recording started plays
  no input.
- The DLL still notes the observed gate (`gate_index`) where the boarder first
  moves. If it ever differs from the prediction, that is an error: it is
  logged and the capture is marked failed.

If the countdown cannot be read (the rider is already released when the
replay arms), the DLL falls back to the observed gate: before it fires, the
replay holds the recording's gate input from 8 ticks before the recording's
gate, which can replace key changes the recording made in that window.
Recordings need no conversion: a recording's gate is its release tick, found
from its positions as before.

A recording that never moves has no gate. It's replayed from the arm, with no
alignment and no check.

### Checking the replay (the watcher)

The gate fixes timing, but not everything about the spawn is visible. So the
controller watches: it compares the replay's positions to the recording's,
gate-relative, bit for bit, for up to 1,024 ticks after the gate (fewer if the
recording or the CONT prefix is shorter). If they differ, or a position read
failed, the cycle aborts: a replay must match, so a divergence is a TAS bug,
not bad luck to retry. The abort names the first differing tick and both
positions, and the UI saves both trajectories over the window to
`diagnostics/divergence-<unix time>.json`. (Aligned replays used to be
retried up to 30 times; 84 watched replays in `live-full` needed none.)
After 1,024 matching ticks the replay is accepted. Later
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

1. The UI saves the current speed and switches to the catch-up speed. The
   transport cycle registers the UI as the TAS's owner (see "Controller
   ownership"), then stops with `STOP_FOR_RESTART`, which sets
   `cont_suppress_input`: the DLL ignores every live key except Escape until
   the cycle ends, including through the restart and countdown. A key held
   during the restart would otherwise change the spawn. (Aligned PLAY does
   the same.)
2. The transport cycle restarts and arms CONT. The DLL replays the prefix,
   gate-aligned. It trims fast-forward tick batches so they stop exactly at
   the splice. If the splice is past the gate, then until the live gate
   fires (only without a countdown prediction), batches stop at the start
   of the 8-tick pre-gate window and then run at most one tick per frame, so
   a batch can't overshoot a splice just past the gate.
3. **The splice waits for approval.** If playback reaches the splice before
   the watcher has approved, the DLL runs zero ticks per frame ("parks")
   until the controller sets `cont_splice_approved`. The controller approves
   when the watcher's check passes (or, for a splice inside the countdown,
   when the spawn position matches). A slow or crashed controller delays the
   splice; it can't let an unchecked prefix through.
4. On approval the transport cycle is done and releases ownership, which
   clears `cont_suppress_input`. For a splice far into the run, the watcher approves
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

The block belongs to the owner, with no timeout: it ends when the cycle
releases ownership (done, abort or cancel) or when the owning process exits,
which also stops the TAS.

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
live game. PLAY and CONT refuse a take whose rider or x87 precision is known
to differ, naming the menu screen or the renderer setting that fixes it;
armed anyway, the watcher would report the divergence as a TAS bug. The UI
can't fix a mismatch: stance and character are chosen in the game's menus,
and the renderer at launch.

Separately, the DLL's own hooks must not disturb the x87 registers, or they
would change the physics themselves. The per-tick hooks save and restore
them.

## Inside the DLL

The code calls its patches "caves" (code caves).

### How game input works, and how the DLL feeds it

A real keypress travels: Windows message → the game's key handlers (key down
at `+3940`, key up at `+3980`), which set a byte in the game's key buffer →
`BB3B10`, which notifies the keyboard **observer**, which must be told about
the change for steering to work. The observer is the EXE's `TC_Kbd_Impl`
(`[[[Supreme.exe+0x889C4]+0x14]+0x1AC]`): `BB3B10` only queues the event, and
the race loop applies queued events once per tick into a held-key array at
`+0x38`, which is what steering reads. The key buffer alone does not steer.
The array is cleared at every race (re)start. `tas_test`'s `gamemem` reads
both arrays from outside the process.

Each key event carries a game timestamp, and the observer applies an event
only once the tick's clock has reached its stamp; an event stamped in the
future holds back everything queued behind it. The stamp is one 64-bit QPC
count, and the tick's clock trails wall time. So the DLL stamps injected
events one whole window of the high half (2^32 counts, about 7 minutes)
before the game's current time, with the low half zero
(`injection_stamp.hpp`): always in the past, even just after the high half
rolls over, when a stamp of the current high half would be ahead of the
trailing clock and land a tick or more late. The observer also applies at
most one event per key per tick, and skips the event right after one it
applies until the next tick, so two keys changed on the same tick reach
steering one tick apart.

### The hooks

| Hook | Where | Job |
| --- | --- | --- |
| **Cycle cave** | `Supreme::Cycle` | Runs once per tick. Applies commands and records or replays that tick's input. |
| **Key-handler cave** | key handlers | Blocks real key events during PLAY, so input only comes from the recording, and whenever a CONT is in flight (`cont_suppress_input`), even with the mode OFF. |
| **Observer cave** | observer `BB3B10` | Blocks real observer calls while a CONT is in flight. |
| **Tick cave** | tick loop in `Supreme.exe` | Decides how many ticks run each rendered frame: speed control, pause catch-up, parking at a splice. |
| **Replay capture** | the game's replay recorder | Finds the human player's object so the cycle cave can read its position. Ignores AI and ghost riders. |

Each tick, the cycle cave:

1. Applies a pending command (arm, stop, restart) and steps an in-progress F5
   restart. An arm first releases whatever keys the DLL is holding, even when
   it arrives over a live REC or PLAY without a STOP, or is refused.
2. In a CONT whose approved splice is already due, switches to REC before
   this tick's input is chosen, so this tick is recorded, not replayed.
3. In REC, reads the keys the observer holds for this tick; in PLAY, reads
   the logged input for this tick.
4. In PLAY, writes that input into the key buffer and the observer's held
   keys (a take from before v56: calls the observer for each key that
   changed instead).
5. Logs the input (REC) and the position (REC and PLAY).
6. After a PLAY tick, checks again whether a CONT splice can complete.

(`tas_test`'s `play-pace` check confirms playback advances 100 ticks per
second at 1x, i.e. one cycle-cave call per tick.)

**REC observes, PLAY writes** (`held_keys.hpp`). The race loop runs the
observer's Update and then `Supreme::Cycle` each tick, so at the cycle cave
the observer's held-key array is exactly what this tick's physics will read.
REC lets real key events through and records that array; the game handles
the keys as it always would (pause menu included), and the take holds what
steering saw, taps shorter than a tick and all. PLAY writes the recorded
keys into the same array at the same point, after that tick's Update, so no
queue, stamp or one-event-per-tick rule stands between the recording and the
physics. Nothing listens to the observer's events; gameplay polls the array
(`Is_Pressed`), and for jump and shift it ORs three key codes each
(0x27..0x29, 0x24..0x26), so a bit reads as held when any of them is and
PLAY sets all three.

A fresh REC only observes: after its restart the game holds what real key
events have given it since, as it would without the TAS (in a race the game
ignores Windows' key repeats, so a key held through the restart counts once
it is pressed again). At a CONT splice the replay's keys are still held and
the player's were blocked, so the first REC tick applies the observer's
queue (the game's own flush), then sets the held keys and the key buffer to
the physical keyboard (`GetAsyncKeyState`, only while the game has focus)
using the codes real key events set (either Ctrl is 0x27, either Shift
0x24), so the player's key-ups release them. REC passes only the six
recorded keys (plus Escape, and menus while paused); a key the take could
not hold could change the run. Releases of other keys always pass (one
pressed in a menu must come back up); during a running PLAY such a release
is applied to the held array directly rather than queued, since a queued
event among the replay's would push one of them a tick later. Escape is the
exception: its press always gets through, and the pause menu must see it
come up (`tas_test pause-resume`).

Releasing the DLL's keys (STOP, an arm over a live session, the end of
PLAY) applies the observer's queue first and then clears, in the held array
and the key buffer, every key PLAY pressed since its arm (an injected take
can still have a press queued for a key its last mask no longer holds), so
no queued event can leave a key down (`tas_test release-pending`).

Takes carry their model (`input_model`: file header, history entry,
recovery checkpoint; absent = injected). Takes made before v56 were
recorded by injecting the keyboard through the observer's queue, so their
masks take effect a tick later and two keys changed together land a tick
apart; PLAY replays them that way unchanged. A CONT from such a take
converts the replayed prefix to the held model at the splice, from the held
keys its replay actually produced, so every new take is one model.

PLAY blocks real key events. Escape always gets through (pause menu,
abort), and the block lifts while a menu runs over a stalled race so menus
stay usable. The CONT block (`cont_suppress_input`) doesn't lift when
paused.

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
  bypasses the 20-tick limit). That tick is physics-neutral: physics steps a
  constant dt per tick whatever the advance, and the only thing the drain
  changes is the input-gate time of that one tick.

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

A RESTART abandons an armed session: it applies a STOP first, keeping any
input protection (`cont_suppress_input`). Left armed, a CONT would splice into
REC on the rebuilt level. During a race the message pump also consumes
RESTART, because a CONT parked at its splice runs no ticks and the cycle cave
never sees the command (`tas_test restart-while-parked`).

### Stopping

A STOP command ends either mode. PLAY also stops at the end of the
recording, and REC when the buffer is full. Leaving the race (quitting to the
menu, switching track) stops an armed mode too. Every stop releases the keys
the DLL was holding, so the boarder doesn't keep steering; so does every arm
and a RESTART (`tas_test arm-over-live-release`, `cont-refuse-release`).
Pausing doesn't stop anything.

### The race lifecycle

`caves/lifecycle_cave.hpp` follows the game's own transition points, all on the
game thread:

| Hook | Where | Job |
| --- | --- | --- |
| **Launch** | `Supreme.exe+0x25BD7`, after the race loop enters the level | Identifies the track and publishes it; `game_in_game` = 1. Rider and renderer stamps refresh on the race's first tick. |
| **Stop** | `Supreme::Stop` (`Supreme_Game+0x1408F0`) | Stops an armed mode while the level still exists; publishes "no race"; `game_in_game` = 0. |
| **Pump** | the game's message pump (`Supreme.exe+0x55920`) | Runs in every state, including the menus, the pause menu and dialogs where `Supreme::Cycle` doesn't. Checks the owning controller is alive and answers ownership requests, consumes STOP there (and RESTART during a race), expires menu commands, keeps the crash record's module table current and writes the DLL's queued log lines. |

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

- **Race timer** (`race_timer_cave.hpp`): publishes the game's own race
  clock every clock tick: the player's timer
  object at `[player+0xB8]` holds the elapsed time as a float at `+0x0C`
  (0.01f added per Player update from the start line to the finish line)
  and started/finished bytes at `+0x10`/`+0x11`. The HUD formats that float
  as `MM:SS:CC` with truncating conversions, so the float, not a tick count,
  is the official time: it drifts (60,000 ticks read 600.27 s), and the tick
  count is recovered by replaying the float sum (`tas_shared::race_clock`).
  The UI shows the clock's time through the HUD formula. A HUD-text scraper
  once confirmed that formula against every on-screen time, at 1x and 8x,
  through the finish; it is retired, and `tas_test race-clock` now checks
  that the clock starts, never runs backward and finishes on a tick sum.
- **Finish line** (`finish_cave.hpp`): the game's Finish_Point crossing
  callback calls the rider's race timer finish (`SG+0x7F9D0`), which decides
  whether the finish counts (a missed checkpoint voids the time) and latches
  it once. A hook right after that decision (`SG+0x7FA33`: `bl` = counts,
  `esi` = the timer, `[esi+0x1C]` = its Player) publishes, for the human
  rider only (ghosts and AI finish through the same code), the tick it
  finished in (the recorded index in REC, the playback index in PLAY), the
  mode, whether it counts and the final time (`race_finish_*`). It runs in
  that tick's physics, inside `Supreme::Cycle`. The UI stops a REC on a
  finish that happened in REC, and labels the take with that time; it no
  longer tests positions against level geometry, which differs between
  tracks and game versions. `tas_test finish-line` checks the tick is the
  same at 1x and 8x, the time is the race clock's, the replay's positions are
  past FE's finish plane on the next tick, and a CONT's REC reports its own
  finish.
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
  sets the alignment fields and the DLL clears them. `cont_suppress_input`
  is set by `STOP_FOR_RESTART` from the owning controller and cleared by a
  plain STOP, by the owner's release and by the owner's exit.
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

A transport cycle registers its process as the owner first, and a second
controller's cycle is refused while the owner lives. Commands sent outside
a cycle aren't checked, so the harness still closes any running `tas_ui`
before it starts. (`tas_ui` refuses a second instance of itself.) The menu
reader uses a separate command channel of its own (sequence, target,
acknowledgement, result).

### Controller ownership

A controller that dies mid-cycle would otherwise leave the TAS armed, keys
held and live input blocked, with nothing to notice. So the DLL tracks which
process owns the TAS (`controller_owner.hpp`, `caves/owner_cave.hpp`):

- Before its first command, a transport cycle writes its pid and process
  creation time and bumps `owner_request_seq`. On the next pump pass the
  DLL opens a handle to that process, checks the creation time (so a reused
  pid can't pass) and answers in `owner_result` / `owner_ack_seq`. A live
  owner is never displaced: another process gets BUSY. The same process may
  acquire again.
- Every pump pass the DLL polls the handle. When the owner has exited (or
  the handle stops answering), it runs the full STOP transition (keys
  released, F5 let go, splice approval and alignment cleared, protection
  lifted), drops the owner's unconsumed command and logs the stop. The handle
  identifies the process object itself, so this is exact, not a timeout.
- A cycle releases ownership when it finishes, aborts or is cancelled; the
  release clears `cont_suppress_input`. A block found set with no owner is
  cleared and logged.
- A controller that is alive but hung keeps ownership; that isn't detected.

`tas_test owner-death` kills a real owning process mid-restart, in the
countdown and mid-replay with keys held, and checks the stop; it also checks
that a reader's death changes nothing, that a second controller is refused,
and that a living owner's block holds for 12 s.

### Crashes

DLL calls into game code (`Kernel::Time::Current`, the menu's
Trigger/Up/Down/Left/Right, Request_Focus, Want_Focus, Get_Active_Component,
Get_Modal) are not wrapped in `__try`: a fault there is a bug, so it crashes
the game rather than being swallowed. Plain memory reads of game objects
that an F5 restart may have freed stay guarded; they leave nothing half done.

Each such call sets `game_call` for its duration (`crash_report.hpp`). A
vectored exception handler records a fault raised while one is in flight;
it has to be first-chance, because the game's own handler catches the fault
and holds the process open behind its kernel-error dialog. An
unhandled-exception filter, re-installed from the pump and chained to any
later one, records the rest. The record (code, module + offset from a module
table the pump refreshes every 2 s, the call, the thread) goes into shared
memory, which `tas_ui` still maps after the game dies. `game_exit_clean` is
set on a normal exit.

`tas_ui` shows a red banner with the fault as soon as a record appears, and
when the game process goes away it says how: crashed (with the record), or
gone without a clean exit and without a record (killed, or a fault nothing
saw). A `__fastfail` bypasses every handler, and nothing here writes a dump
file. `tas_test crash-report` sends a test-only command that faults inside
`Kernel::Time::Current` and checks the record.

## SSB Inspect (tas_ui)

The UI's responsibilities:

- **Transport**: REC, PLAY, CONT and STOP buttons, also on F9 to F12 while
  the game has focus. Each start runs the transport cycle above. CONT from
  frame 0, or with nothing recorded, has nothing to replay and records a
  fresh take. PLAY and CONT refuse a take recorded on another track (its
  level stamp, else its spawn position), rider or x87 precision; unknown on
  either side is allowed.
- **Sessions**: when REC starts it opens a session; when REC stops, including
  when the DLL stops it because the race was left, it saves the take to
  history.
- **Editing**: inputs appear as held-key spans on a timeline
  (`panels/timeline.rs`), or in a TMInterface-style text script
  (`324-372 press left`, ticks, end exclusive) that reloads on save
  (`panels/input_script.rs`). Edits apply only while stopped; the UI stops a
  running session first. The timeline opens on a 60 s window until the user
  zooms, pans or fits. A click on open lane space sets the CONT frame; a
  click or drag on an input span never does.
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
of the original take, so past the first changed tick they are stale. The take
carries that tick (`trajectory_ticks` in the file header and the history
entry's stamps; later edits only move it earlier), and the watcher and the
drift banner compare only up to it. An edit inside the countdown leaves only
the spawn to check. A CONT takes the replayed prefix as the new take's
positions at the splice, so the take it records is whole again.

### Recording files (`.tasrec`)

```
[u32 header length][JSON header][input bytes, 1 per tick][f32 X, Y, Z per tick]
```

All numbers are little-endian. The JSON header holds `recorded_count` (which
sets both body lengths) and the rider and physics stamps. Loaders ignore
unknown header fields, such as the `segments` list older files carry (it
was never read, and overlapped after a CONT). `tas_codec` owns the format: it rejects
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
UI falls back to the checkpoint. If the game crashed, or vanished without a
clean exit, the UI says so in a banner (see "Crashes").

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

**Why a Pico.** REC records the keys the game received from Windows, so a
test of REC needs real key presses; messages posted to the game window
aren't the same path. A Raspberry Pi Pico 2 acting as a USB
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
- An edited take is watched only up to its first edited tick until a CONT
  re-records its positions. A UI reopened on a buffer the DLL kept takes the
  stamps (edit limit included) of the selected history entry when that entry
  holds the same input; otherwise it knows none and watches the whole take.
