# SSB Inspect — TAS Implementation Plan

> Migrate the zero-drift TAS pipeline from Cheat Engine Lua/ASM to a native
> DLL + egui architecture. Build a world-class tool-assisted speedrun
> environment for Supreme Snowboarding.

## Status

- **Current**: CE-based TAS with 5 code caves, Pico HID input, zero drift proven
- **Target**: Standalone `TAS_Helper.dll` + `SSB Inspect` egui UI, no CE dependency
- **Proven config**: inject_mode=6, force_direct=2, fft=0, self_capture=false, Pico HID
- **Name**: SSB Inspect (renamed from "Supreme TAS" per board feedback)

---

## 1. Architecture Overview

```
 Supreme.exe process                         Separate process
+-----------------------------------+       +-------------------------------------+
| TAS_Helper.dll (C++/SafetyHook)   |<----->| SSB Inspect (Rust/egui)             |
|                                   |  IPC  |                                     |
| Cave 1C: handler gate             |       | Transport bar (REC/PLAY/STOP/CONT) |
| Cave 1D: BB3B10 observer gate     |       | Input timeline editor               |
| Cave 2:  record/play engine       |       | Drift monitor (live graph)          |
| Cave 5:  fixed tick override      |       | Telemetry dashboard                 |
| tasState: shared memory block      |       | Trajectory viewer (2D top-down)     |
|                                   |       | Config panel (collapsed by default) |
| FSAVE/FRSTOR FPU preservation     |       | Pico HID serial (opt., auto-detect) |
| Integer-width memcpy (no x87)     |       | Log panel + dump-to-file            |
| Deferred logging (no format in cb)|       | Undo ring (5 deep)                  |
+-----------------------------------+       | Save/load manager                   |
         |                                  | Savestate browser (future)          |
         | LoadLibraryA                     +-------------------------------------+
         |                                           |
   Injector.exe                               serialport crate (optional)
   (reuse existing)                                  |
                                              Pico 2 (optional)
```

### Components

| # | Component | Language | Build | Ships as |
|---|-----------|----------|-------|----------|
| 1 | `TAS_Helper.dll` | C++20 | MSBuild (v143) | Win32 DLL |
| 2 | `SSB Inspect` | Rust | Cargo | Standalone exe |
| 3 | `Injector.exe` | C++ | MSBuild (reuse) | Console exe |
| 4 | Test runner (`tas_test`) | Rust | Cargo (same workspace) | CLI binary |
| 5 | Shared types (`tas_shared`) | Rust | Cargo (same workspace) | Library crate |

### Why these choices

- **C++ DLL, not Rust DLL**: Display_Config already has SafetyHook, pattern scanning, and a working vcxproj for Win32 DLL injection. Porting those to Rust would double the work for no benefit inside the game process.
- **egui, not Tauri**: Tauri doesn't work well on Linux. egui via eframe is pure Rust, cross-platform, and has no webview dependency.
- **Separate process, not overlay**: Crash isolation. If the UI dies, the game keeps running. IPC via named shared memory is fast enough (sub-millisecond).
- **Reuse Injector.exe**: Same `CreateToolhelp32Snapshot` + `LoadLibraryA` pattern. Just point it at TAS_Helper.dll instead of Display_Config_Helper.dll.
- **Dark theme by default**: egui dark mode. Clean, professional look. No bright flashes during long sessions.

---

## 2. IPC: Named Shared Memory

```
CreateFileMapping(INVALID_HANDLE_VALUE, ..., "Local\\SupremeTAS", sizeof(TasSharedState))
```

**DLL side** (C++): Creates the mapping on DllMain attach. Owns the tasState layout.
**UI side** (Rust): Opens the mapping. Reads state, writes commands.

### Shared state layout

```c
struct TasSharedState {
    // -- Command region (UI writes, DLL reads) --
    uint32_t command;           // 0=idle, 1=arm_rec, 2=arm_play, 3=stop, 4=arm_continue
    uint32_t inject_mode;       // 6 (proven), future modes
    uint32_t force_fixed_tick;  // 0=off, 2=deterministic
    uint32_t force_direct;      // 2=BB3B10 direct calls
    uint32_t self_capture;      // 0=false (arm-wait gate)
    uint32_t use_rec_msg_args;  // 1=use recorded wrapper args
    uint32_t input_source;      // 0=DI buffer sample
    uint32_t continue_from_frame; // frame N for Continue Record (0=disabled)
    uint32_t step_mode;         // 0=normal, 1=pause-after-each-frame
    uint32_t step_one_frame;    // UI sets to 1, DLL clears after advancing
    uint32_t playback_speed;    // 100=1x, 50=0.5x, 25=0.25x, 200=2x, 400=4x
    uint32_t savestate_cmd;     // 0=none, 1=save, 2=load
    uint32_t savestate_slot;    // 0-9 slot index

    // -- Status region (DLL writes, UI reads) --
    uint32_t mode;              // 0=off, 1=rec, 2=play
    uint32_t recorded_count;    // ticks recorded
    uint32_t playback_pos;      // current playback tick
    uint32_t prev_mask;         // last input bitmask
    uint32_t cave2_injecting;   // playback injection active
    float    player_x, player_y, player_z;
    float    max_drift_x, max_drift_z;

    // -- Telemetry (DLL writes, UI reads) --
    float    velocity_x, velocity_y, velocity_z;  // per-frame velocity
    float    speed;             // magnitude of velocity vector
    float    yaw;               // player heading (degrees)
    float    pitch;             // player pitch (degrees)
    uint32_t tick_count;        // ticks this frame (Cave 5 reports)
    uint32_t frame_counter;     // game's global frame counter
    float    elapsed_time;      // game elapsed time (seconds)

    // -- Input log (DLL reads/writes) --
    uint8_t  input_log[65536];  // per-tick bitmask

    // -- Coordinate logs --
    float    rec_coords[65536][3];   // X,Y,Z per tick (REC)
    float    play_coords[65536][3];  // X,Y,Z per tick (PLAY)

    // -- Diagnostics --
    uint32_t bb3b10_call_count;
    uint32_t handler_block_count;
    uint32_t event_count;
    uint32_t transition_count;       // input mask changes this run
    uint32_t version;                // build version for compatibility check

    // -- Log ring buffer (DLL writes, UI reads) --
    char     log_buffer[65536];      // circular text log
    uint32_t log_write_pos;          // DLL advances this
    uint32_t log_read_pos;           // UI advances this
};
```

Lock-free single-writer design: UI writes command region only, DLL writes status/telemetry region only. Atomic uint32_t for `command` and `mode` fields. No mutex needed.

---

## 3. DLL: TAS_Helper.dll

### 3.1 Hook points (port from CE ASM)

| Cave | Hook site | CE offset | Purpose |
|------|-----------|-----------|---------|
| 1C | HMG_Cetsup_Win32.dll+0x3940/+0x3980 | +3940, +3980 | Block external handler calls during REC and PLAY (symmetric) |
| 1D | HMG_Cetsup_Win32.dll+0x3B10 | BB3B10 | Gate observer notification for timing symmetry |
| 2 | Supreme_Game.dll+0x13FE40 | Supreme::Cycle | Main REC/PLAY engine |
| 5 | Tick computation site | +25C81/+426000 | Force fixed tick count |

All hooks use SafetyHook `create_mid()` with the same pattern-scan signatures already proven in Display_Config. The existing helper.hpp `PatternScan()` function works unchanged.

**Critical implementation details**:
- **FSAVE/FRSTOR**: SafetyHookMid doesn't save x87 FPU state. Cave 2 and Cave 5 must wrap their bodies in FSAVE/FRSTOR to prevent FPU corruption.
- **Integer-width memcpy**: Float field clearing must use integer-width `memcpy`, not x87 instructions, to avoid contaminating the FPU state.
- **Deferred logging**: No `Log()`, `format()`, or `sprintf()` calls inside hook callbacks. These can steal focus or cause reentrancy. Write to the shared memory log ring buffer instead.
- **Symmetric Cave 1C**: Cave 1C blocks handler dispatch during BOTH REC and PLAY (not just PLAY). Only passes when `cave2_injecting=1` (Cave 2's direct BB3B10 calls) or `mode=OFF`.

### 3.2 BB3B10 direct call (critical for zero drift)

During PLAY, Cave 2 must call BB3B10 directly on input transitions:

```cpp
// Pseudo-code for the critical PLAY path
void on_transition(uint8_t bit, bool pressed) {
    auto keyboard = read_pointer(SG_BASE + 0x1D5450, 0x530);
    auto key_table = (uint32_t*)(SG_BASE + 0x9AD8);
    uint32_t keyIndex = key_table[vk_for_bit(bit)];

    // Set cave2_injecting so Cave 1C/1D pass through
    shared->cave2_injecting = 1;

    // Call BB3B10(keyboard+0x18, keyIndex, pressed, 0, 0x588)
    auto bb3b10 = (BB3B10_fn)(SG_BASE + 0x3B10);
    bb3b10(keyboard + 0x18, keyIndex, pressed ? 1 : 0, 0, 0x588);

    shared->cave2_injecting = 0;
}
```

Without this call, buffer-write-only produces ~41 unit drift during turns. The BB3B10 observer notification drives physics state updates that the buffer alone does not trigger.

### 3.3 DI buffer write

```cpp
// Write input mask to DirectInput keyboard buffer
auto buffer = read_pointer(SG_BASE + 0x1D5450, 0x530 + 0x30);
// Game key codes: UP=0x38, DOWN=0x39, LEFT=0x3A, RIGHT=0x3B
for each bit in mask:
    buffer[game_keycode] = pressed ? 0x80 : 0x00;

// Write byte input flags on keyboard object
// keyboard+455=LEFT, +456=RIGHT, +457=UP, +458=DOWN
// keyboard+440=SHIFT, +441=JUMP

// Write action_state at [SG+1D5450]+offset
// LEFT=+0xC4, UP=+0xC8, RIGHT=+0xCC, DOWN=+0xD0
```

### 3.4 Coexistence with Display_Config_Helper.dll

Both DLLs can be loaded simultaneously. They hook different sites:
- Display_Config hooks: FOV, fonts, ghost opacity, DirectInput creation, replay speed
- TAS hooks: Cave 1C/1D (handler gate), Cave 2 (Supreme::Cycle), Cave 5 (tick)

No overlap. Both use SafetyHook's allocator which handles multiple near-code allocations.

**Future consideration**: Merge TAS into Display_Config_Helper.dll as a feature flag. Not for v1 -- keep separation for cleaner development and testing.

### 3.5 Frame sequence and timing

Each game frame executes in this order:

1. `Handle_OS_Messages` — processes Windows message queue
2. `Poll_Controllers` — reads DirectInput state
3. `Supreme::Cycle` — game update (Cave 2 hooks here)

During REC (mode 6):
1. Pico HID (or real keyboard) sends a keystroke through the normal OS pipeline
2. Windows delivers it -> DirectInput updates the buffer
3. Cave 1D blocks the natural `+1830 -> +3940 -> BB3B10` chain
4. Cave 2 fires at Supreme::Cycle time, reads the DI buffer, records the 6-bit mask + player XYZ
5. On input transitions, Cave 2 calls BB3B10 directly with `cave2_injecting=1`

During PLAY:
1. Cave 2 fires at Supreme::Cycle time, reads the recorded mask for frame N
2. Writes DI buffer + byte input flags + action_state
3. On transitions, calls BB3B10 directly with `cave2_injecting=1`
4. Cave 1C blocks any external handler calls

This achieves timing symmetry: BB3B10 fires from Cave 2 in both modes, at the same point in the frame.

### 3.6 Telemetry extraction

The DLL extracts real-time telemetry from game memory and writes it to the shared state:

```cpp
// Player position (already captured for drift checking)
float x = readFloat(playerPtr + 0xF8);
float y = readFloat(playerPtr + 0xFC);
float z = readFloat(playerPtr + 0x100);

// Physics sub-object
auto physics = readPointer(playerPtr + 0x110);
// Previous position for velocity calculation
float prev_x = readFloat(physics + 0x3C0);
float prev_y = readFloat(physics + 0x3C4);
float prev_z = readFloat(physics + 0x3C8);

// Velocity = current - previous (per tick)
shared->velocity_x = x - prev_x;
shared->velocity_y = y - prev_y;
shared->velocity_z = z - prev_z;
shared->speed = sqrt(vx*vx + vy*vy + vz*vz);
```

Yaw and pitch extraction requires additional reverse engineering of the player's rotation matrix or quaternion. Candidate offsets to investigate:
- Player object +0x108..+0x10C (rotation fields)
- Physics sub-object +0x3AC..+0x3B0 (angular velocity candidates)

### 3.7 Playback speed control

The DLL modifies the tick timing to achieve variable playback speed:

| Speed | Ticks per frame | Method |
|-------|-----------------|--------|
| 0.25x | 1 every 4 frames | Skip 3 of every 4 Cave 2 calls |
| 0.5x | 1 every 2 frames | Skip every other Cave 2 call |
| 1x | Normal | No modification |
| 2x | 2 per frame | Force 2 ticks (same as fft=2) |
| 4x | 4 per frame | Force 4 ticks per frame |

Implementation: Cave 5 reads `playback_speed` from shared state and adjusts the tick count accordingly. Only active during PLAY mode.

### 3.8 Savestate system (future, hard)

Snapshot and restore game memory to allow instant jump to any frame.

**Approach**: Dump all writable memory regions of Supreme.exe + loaded DLLs to a file. On restore, write them back. This is inherently fragile:
- Heap pointers may be invalidated
- DirectInput/DirectDraw device state is not restorable from memory alone
- Thread state (stacks, TLS) must be captured
- File handles, sockets, and OS resources can't be serialized

**Practical alternative**: "Replay to frame N" — start from frame 0 and fast-forward using recorded inputs at maximum speed. With 4x playback, reaching frame 1000 takes ~4 seconds. This is simpler and proven correct.

**Hybrid approach**: Combine fast-forward with periodic save points. Save game state every 500 frames during REC. On load, restore the nearest save point and replay forward. Reduces load time to <1 second for any frame.

---

## 4. SSB Inspect (egui UI)

### 4.1 Workspace layout

```
SS-Dat-Info/
  TAS/
    Cargo.toml          # workspace root
    tas_ui/
      Cargo.toml
      src/main.rs       # eframe app
      src/app.rs        # app state, update loop
      src/panels/
        transport.rs    # REC/PLAY/STOP/CONT buttons + status
        timeline.rs     # input timeline editor
        drift.rs        # drift monitor graph
        telemetry.rs    # speed, position, heading dashboard
        trajectory.rs   # 2D top-down path viewer
        config.rs       # injection config (collapsed by default)
        pico.rs         # Pico HID panel (auto-detect)
        log_panel.rs    # scrolling diagnostic log
        savestate.rs    # savestate browser (future)
      src/recording.rs  # save/load .tas files
      src/undo.rs       # undo ring buffer (5 deep)
      src/ipc.rs        # shared memory client
    tas_test/
      Cargo.toml
      src/main.rs       # test runner CLI
    tas_shared/
      Cargo.toml        # shared types (TasSharedState repr)
      src/lib.rs
```

### 4.2 UI panels

#### Transport bar
- **REC** (red circle): Arm recording. Game must be focused. F5 restarts the run.
- **PLAY** (green triangle): Arm playback. F5 restarts, DLL replays the input log.
- **STOP** (white square): Stop recording or playback immediately.
- **CONT** (blue arrow): Continue Record from frame N. Replays 0..N then switches to live REC.
- **Status indicator**: Shows current mode (IDLE / REC frame 234 / PLAY frame 234/1000 / CONT).
- **Frame counter**: Current frame / total frames. Click to jump to a frame (during PLAY).

#### Input timeline editor
- Horizontal strip showing input masks over time.
- Color-coded lanes per input: LEFT=blue, RIGHT=red, UP=green, DOWN=orange, JUMP=yellow, SHIFT=purple.
- Scrollable and zoomable (mouse wheel = zoom, drag = scroll).
- **Click to edit**: Click a cell to toggle an input on/off for that frame. Edits are live — the recording is modified in place.
- **Drag to paint**: Hold and drag to paint inputs across multiple frames.
- **Selection**: Click+drag to select a range of frames. Copy/paste/delete selection.
- **Markers**: Right-click to add named markers ("perfect turn", "jump point", "splice here").
- **Cursor line**: Vertical line showing current playback position (during PLAY).
- **Transition indicators**: Small arrows at frames where inputs change (edges).

#### Drift monitor
- Live X/Z drift graph during playback. Y-axis = drift in game units. X-axis = frame number.
- Red zero-line. Green trace for Z drift, blue trace for X drift.
- Max drift readout in corner.
- In normal operation this should always be flat at zero. Non-zero drift means something is wrong.

#### Telemetry dashboard
- **Speed**: Current velocity magnitude (game units/tick). Line graph over time.
- **Position**: X, Y, Z coordinates. Updated per frame.
- **Heading**: Yaw angle (degrees). Compass widget showing direction.
- **Altitude**: Y coordinate as a vertical bar.
- **Tick count**: Ticks this frame (should be consistent).
- **Frame rate**: Frames per second (derived from frame counter).
- Inspired by TrackMania Interface-style telemetry overlays.

#### Trajectory viewer (2D top-down)
- Renders the player's X/Z path as a line on a 2D canvas.
- During PLAY: shows REC path (faded) and PLAY path (solid) simultaneously.
- Zoom to fit or manual zoom/pan.
- Color gradient along the path shows speed (blue=slow, red=fast).
- Dot at current position with heading indicator.
- Grid lines for scale reference.
- Click any point on the path to jump to that frame in the timeline.
- **Ghost comparison**: Load a second recording and overlay its path in a different color.

#### Config panel (collapsed by default)
- inject_mode, force_fixed_tick, force_direct, self_capture, use_rec_msg_args, input_source.
- Dropdowns/toggles matching proven config.
- **Warning**: Collapsed by default. These are debug/development settings. The proven zero-drift config is the default — changing these risks breaking drift. Show a warning banner when expanding.

#### Pico HID panel (optional, auto-detect)
- COM port selector (auto-scans for Pico devices).
- Connection status: Connected / Disconnected / HID Dead.
- Health check button (sends LEFT via serial, checks GAKS).
- Soft reconnect button (0xFD D+ pullup toggle).
- Hidden by default. Auto-shows when a Pico is detected.

#### Log panel
- Scrolling text log from DLL diagnostics (via shared memory ring buffer).
- Severity color coding: INFO=white, WARN=yellow, ERROR=red.
- Filter by severity.
- Search within logs.
- Export to file button.

### 4.3 Input without Pico (most users)

Most users won't have a Pico 2. The UI supports recording via normal keyboard input:

- **Keyboard recording**: During REC, the DLL samples the DI buffer directly (input_source=0). Whatever key the OS delivers to the game's DirectInput works.
- **Without CE, keyboard input IS symmetric**: The whole reason Pico was needed is that CE's `keyDown()` only sets GAKS, not DirectInput. With CE removed, real keyboard input goes through DirectInput naturally. Pico becomes optional (useful for automation, not required for zero drift).
- **Mock input mode** (test harness): The test runner can write input masks directly to shared memory's input_log, skipping the HID layer entirely.

### 4.4 Pico serial (optional feature)

For automated/scripted recording, the egui UI can optionally talk to the Pico:

```rust
// tas_ui/src/pico.rs
use serialport;

pub struct PicoHid {
    port: Box<dyn SerialPort>,
}

impl PicoHid {
    pub fn open(port_name: &str) -> Result<Self> { ... }
    pub fn send_mask(&mut self, mask: u8) -> Result<()> { ... }
    pub fn send_f5(&mut self) -> Result<()> { ... }  // bit 6
    pub fn soft_reconnect(&mut self) -> Result<()> { ... }  // 0xFD
    pub fn hard_reset(&mut self) -> Result<()> { ... }  // 0xFE
    pub fn is_alive(&self) -> bool { ... }
    pub fn is_hid_alive(&self) -> bool { ... }  // send LEFT, check GAKS
    pub fn ensure_alive(&mut self) -> Result<()> { ... }  // auto-recover
}
```

### 4.5 Recording file format (.tas)

```
Header (JSON, length-prefixed):
{
    "version": 2,
    "tool": "SSB Inspect",
    "game": "Supreme Snowboarding v1.035",
    "created": "2026-03-17T12:00:00Z",
    "frame_count": 2776,
    "transition_count": 15,
    "config": {
        "inject_mode": 6,
        "force_direct": 2,
        "force_fixed_tick": 0,
        "input_source": 0
    },
    "markers": [
        {"frame": 120, "label": "first turn"},
        {"frame": 450, "label": "jump section"}
    ],
    "notes": "Clean run, no mistakes. Left-right pattern through trees."
}

Body (binary):
  [input_log: frame_count bytes, one mask per frame]
  [rec_coords: frame_count * 12 bytes, 3 floats (X,Y,Z) per frame]
```

Compressed with zstd for storage. Typical 2000-frame recording: ~30KB compressed.

### 4.6 Save/load and undo

- **Auto-save**: Before every REC, CONT, or destructive edit, auto-save current recording to undo ring.
- **Undo ring**: Last 5 saves. Ctrl+Z to restore previous. Ring wraps — oldest save is overwritten.
- **Manual save**: File > Save As. Writes `.tas` file to disk.
- **Load**: File > Open. Loads `.tas` file, replaces current recording.
- **Recent files**: File > Recent. Last 10 opened/saved files.

### 4.7 Dump to file

File > Dump Diagnostics. Writes a timestamped text file containing:
- Recording metadata (frames, transitions, config)
- Input transition table (frame, mask, description)
- Drift analysis (per-section breakdown)
- Coordinate table (REC vs PLAY, delta per frame)
- Full diagnostic log

Replaces the CE-only `print()` dump approach.

### 4.8 Keyboard shortcuts

| Shortcut | Action |
|----------|--------|
| F5 | Restart run (same as game) |
| F6 | Toggle REC |
| F7 | Toggle PLAY |
| F8 | Stop |
| F9 | Continue Record |
| Space | Pause / Resume (during PLAY) |
| . (period) | Step one frame (when paused) |
| , (comma) | Step back one frame (requires replay-to-frame) |
| Ctrl+Z | Undo |
| Ctrl+S | Save recording |
| Ctrl+O | Open recording |
| +/- | Zoom timeline in/out |
| 1-5 | Playback speed (0.25x, 0.5x, 1x, 2x, 4x) |

---

## 5. Advanced Features

### 5.1 Continue Record (splice recording)

The core workflow for TAS routing:

1. Record a run (frames 0..N)
2. Watch the replay. Identify frame K where you want to change the input.
3. Click CONT with `continue_from_frame = K`.
4. DLL replays frames 0..K automatically (at 4x speed).
5. At frame K, DLL switches to live REC. You play from that point.
6. New inputs overwrite frames K+1 onward.
7. Previous recording (0..N) is saved to undo ring.

This is the fundamental TAS editing loop. Combined with frame stepping, it allows frame-perfect input placement.

### 5.2 Frame stepping and scrubbing

- **Pause**: During PLAY, press Space to pause at the current frame.
- **Step forward**: Press `.` to advance exactly one frame. DLL processes one tick.
- **Step backward**: Press `,`. This requires replaying from frame 0 to frame N-1 (fast, ~1ms per frame at 4x). The DLL does this internally.
- **Scrub**: Drag the timeline cursor to any frame. DLL replays to that frame at maximum speed.
- **Breakpoints**: Set a frame breakpoint. PLAY auto-pauses when it reaches that frame.

### 5.3 Input macros

Save commonly used input sequences as named macros:

- **Quick turn left**: LEFT held for 12 frames
- **Jump left**: LEFT + JUMP for 8 frames, then LEFT for 20 frames
- **Shift boost**: SHIFT for 3 frames, release for 1 frame, repeat

Macros can be pasted into the timeline at any frame position. Useful for consistent trick execution.

### 5.4 Ghost comparison

Load a second `.tas` recording and overlay it:

- **Timeline**: Second recording shown as a faded row above/below the primary.
- **Trajectory**: Second path rendered in a different color.
- **Speed graph**: Both speeds overlaid for comparison.
- **Drift display**: Shows the drift between the two recordings (not just REC vs PLAY).

Use cases:
- Compare two routing attempts to see which is faster.
- A/B test a single input change.
- Compare human play vs TAS.

### 5.5 Telemetry (TrackMania Interface inspired)

Real-time game state readout, inspired by TrackMania's TMInterface:

| Metric | Source | Display |
|--------|--------|---------|
| Speed | `sqrt(vx^2 + vy^2 + vz^2)` | Speedometer gauge + line graph |
| Position X/Y/Z | Player +0xF8/+0xFC/+0x100 | 3 readouts + trajectory view |
| Heading (yaw) | Player rotation (needs RE) | Compass widget |
| Altitude (Y) | Player +0xFC | Vertical bar |
| Ticks/frame | Cave 5 report | Counter |
| Time | Game elapsed | Clock |
| Distance traveled | Cumulative path length | Odometer |

The telemetry panel is collapsible. Power users can pop it out as a separate window.

### 5.6 Trajectory analysis

Beyond the 2D viewer, dedicated analysis tools:

- **Optimal line finder**: Highlight the straightest path between two points. Compare actual path to theoretical optimal.
- **Turn radius estimator**: Calculate the turning radius at each point. Identify if turns are too sharp (wasting speed) or too wide (wasting distance).
- **Sector splits**: Define sectors (start gate, checkpoint, finish). Show time per sector. Compare between recordings.
- **Path length**: Total distance traveled. Less distance = faster time (usually).

### 5.7 Input analysis

Post-recording analysis tools:

- **Input histogram**: Count frames spent in each state (neutral, left, right, jump, shift). Pie chart or bar graph.
- **Transition density**: Frames between transitions. Identifies "busy" vs "calm" sections.
- **Input efficiency**: Frames where input was held but had no effect (e.g., LEFT while already at max turn rate).
- **Symmetry check**: Compare left-turn frames vs right-turn frames. Asymmetry may indicate suboptimal routing.

### 5.8 Multi-segment recording

For full-run TAS across multiple stages or checkpoints:

- **Segment list**: Each segment is a separate `.tas` file. The segment list defines the order.
- **Auto-chain**: Finish one segment, auto-start recording the next.
- **Segment timing**: Individual segment times + total run time.
- **Segment editor**: Reorder, delete, or replace individual segments without affecting others.

### 5.9 Recording annotations

- **Named markers**: Place named markers at any frame ("first turn", "jump spot", "risky section").
- **Notes**: Free-text notes attached to a recording (stored in the .tas header).
- **Tags**: Categorize recordings (#routing, #optimization, #backup, #final).
- **Timestamp**: Auto-recorded creation and modification times.

### 5.10 In-game UI overlay (alternative/complement to separate window)

An alternative to the separate egui window: render the TAS UI directly inside the game window as an overlay.

**Approach**: Hook Supreme Snowboarding's DirectDraw rendering pipeline to inject UI elements after the game's frame is drawn but before the flip/present call.

**Implementation options**:

| Option | Library | Language | Notes |
|--------|---------|----------|-------|
| Dear ImGui + DDraw hook | Dear ImGui | C++ (in DLL) | Most proven approach. ReShade, Special K, and most game overlays use ImGui. Hook `IDirectDrawSurface::Flip` or `Blt`. |
| egui inside DLL | egui + softbuffer | Rust (in DLL) | Would require a Rust DLL or FFI bridge. Less proven for game overlays. |
| GDI overlay | Win32 GDI | C++ (in DLL) | Simple text/rectangles via `GetDC` on the DirectDraw surface. Low quality but easy. |
| Transparent window | Win32 | Separate process | Layered window positioned over game window. No hooking needed but fragile positioning. |

**Recommended**: Dear ImGui via DirectDraw hook in TAS_Helper.dll. This is the same pattern used by virtually every game overlay tool. ImGui is lightweight, immediate-mode (like egui), and has battle-tested DirectDraw/D3D backends.

**Key design questions**:
1. **Input routing**: When overlay is visible, should game input be suppressed? Toggle overlay with a hotkey (e.g., F10). When active, ImGui consumes keyboard/mouse; when hidden, all input goes to game.
2. **Crash risk**: Rendering inside the game process means UI bugs can crash the game. Mitigate with careful error handling and a "safe mode" that disables the overlay.
3. **Performance**: ImGui is very lightweight. Even on a 1999 DirectDraw game, the overhead is negligible.
4. **Dual mode**: Support both in-game overlay AND separate window. Some features (like the full timeline editor) are better in a big window. The overlay shows compact telemetry + transport controls.

**Overlay layout** (compact, in-game):
```
+------------------------------------------+
| [REC] [PLAY] [STOP]  Frame: 234/1000    |
| Speed: 12.4  Drift: 0.000  Ticks: 2     |
| X: 1234.5  Z: 5678.9  Yaw: 45.2        |
+------------------------------------------+
```

**Full window layout** (separate egui process):
- Timeline editor, drift graph, trajectory viewer, config panel, log panel
- Everything that needs space or mouse interaction

This gives users the best of both worlds: quick status in-game, full editing power in the separate window.

**Research needed** (assigned to [SSB-74](/SSB/issues/SSB-74)):
- DirectDraw hook specifics for Supreme Snowboarding
- Input routing implementation
- Dear ImGui DirectDraw backend availability/maturity

---

## 6. Test Harness

### 6.1 Test runner CLI

```
tas_test --mode regression --config mode6.toml
tas_test --mode acceptance --steering LRLRL --hold-ticks 56
tas_test --mode smoke  # quick sanity check
tas_test --mode mock   # regression with simulated input (no Pico)
tas_test --mode f5     # F5-aligned zero-drift baseline
```

### 6.2 Test architecture

```
tas_test/
  src/
    main.rs
    harness.rs       # orchestrates REC/PLAY cycles via shared memory
    patterns.rs      # steering pattern DSL (LRLRL, hold/gap ticks)
    gates.rs         # 4-gate verification logic
    drift.rs         # coordinate comparison, drift calculation
    regression.rs    # regression suite (port from _drift_regression_suite.lua)
    acceptance.rs    # 3-phase acceptance test (port from _full_drift_test.lua)
    cache.rs         # recording cache file I/O (compatible format)
    liveness.rs      # game alive check (frame counter advancing)
```

### 6.3 Four verification gates (mandatory before any human testing)

| Gate | Check | Pass condition |
|------|-------|----------------|
| 0 | Z-coordinate reference | 18 reference frames (50-900) match expected values |
| 1 | REC movement | transitions > 0, firstInput != -1 |
| 2 | PLAY movement | steering visibly works (coord deltas present) |
| 3 | Zero drift | maxDriftX == 0.0, maxDriftZ == 0.0 |

**Rule**: Always run synthetic verification BEFORE asking the user to test hardware. Never ship a build that fails any gate.

### 6.4 Regression suite

Port the 15 test cases from `_drift_regression_suite.lua`:
- `01_jump_flip_down/up`, `01_jump_hold/turn_left`
- `01_lrlrl` (and phase/ret/retprobe/stage/upret variants)
- `01_shift_left`, `02_shift_right`, `03_shift_left_right`
- `04_jump_tap`, `05_jump_hold`

Each case: scripted steering pattern -> REC -> PLAY -> drift check -> CSV output.

### 6.5 Acceptance test (3-phase)

1. **Baseline**: Straight-line recording. Verify zero drift with no steering.
2. **Steered**: Left-right-left-right-left pattern with turns. Verify zero drift with steering.
3. **Full overlap**: Verify coordinate arrays are bit-identical between REC and PLAY.

Pass criteria: `steering=PASS, replay_steered=PASS, zero_drift=PASS`.

### 6.6 Mock input for CI

The test runner writes input masks directly into shared memory without needing Pico or real keyboard. This enables:
- Headless CI testing (game must still be running)
- Deterministic input patterns
- No hardware dependency

### 6.7 Determinism certificate

After a successful regression run, the test runner outputs a determinism certificate:

```
=== DETERMINISM CERTIFICATE ===
Tool:     SSB Inspect v1.0.0
DLL:      TAS_Helper.dll v0x26040100
Date:     2026-04-01T12:00:00Z
Tests:    15/15 passed
Drift:    0.000000000 (all tests)
Config:   inject_mode=6, force_direct=2, fft=0
Verdict:  DETERMINISTIC
===
```

This proves the recording is 100% reproducible.

---

## 7. Phase Plan

### Phase 1: Foundation (DONE)

**Goal**: Prove SafetyHook can hook the same sites, prove shared memory IPC works, scaffold projects.

| Task | Description | Status |
|------|-------------|--------|
| 1.1 | Create vcxproj for TAS_Helper.dll | DONE |
| 1.2 | Implement named shared memory | DONE |
| 1.3 | Verify SafetyHook on Cave 2 site | DONE |
| 1.4 | Verify SafetyHook on Cave 1C/1D sites | DONE |
| 1.5 | Scaffold Cargo workspace | DONE |
| 1.6 | Test DLL coexistence | DONE |

### Phase 2: Core DLL (port caves)

**Goal**: All 4 caves working, zero-drift REC/PLAY via shared memory commands.

| Task | Description |
|------|-------------|
| 2.1 | Port Cave 5: fixed tick override with FSAVE/FRSTOR |
| 2.2 | Port Cave 2: REC path (GAKS sample, coord capture, input log write) |
| 2.3 | Port Cave 2: PLAY path (input log read, DI buffer write, action_state write, byte flags) |
| 2.4 | Port Cave 2: BB3B10 direct call on transitions (force_direct=2) |
| 2.5 | Port Cave 1C: symmetric handler blocking (REC and PLAY) |
| 2.6 | Port Cave 1D: BB3B10 observer gate (blocks during REC mode 6 + PLAY) |
| 2.7 | Implement command interface: arm_rec, arm_play, arm_continue, stop, step |
| 2.8 | Continue Record mode: replay 0..N, auto-switch to REC at continue_from_frame |
| 2.9 | Frame step mode: pause-after-each-frame, step_one_frame command |
| 2.10 | Telemetry extraction: velocity, speed, position to shared state |
| 2.11 | Playback speed control via Cave 5 tick manipulation |
| 2.12 | Log ring buffer: DLL writes diagnostics, UI reads |
| 2.13 | Validate zero drift with test harness (4 gates) |

**Exit criteria**: REC via real keyboard -> PLAY via DLL -> maxDriftX=0.0, maxDriftZ=0.0. Continue Record produces zero drift up to the splice point.

### Phase 3: SSB Inspect UI

**Goal**: Functional UI that replaces the CE Lua TAS-ui.

| Task | Description |
|------|-------------|
| 3.1 | eframe scaffold with dark theme, transport bar (REC/PLAY/STOP/CONT) |
| 3.2 | Shared memory reader with live status display |
| 3.3 | Config panel (collapsed by default, warning banner) |
| 3.4 | Input timeline visualization (color-coded, scrollable, zoomable) |
| 3.5 | Input timeline editing (click-to-toggle, drag-to-paint, selection) |
| 3.6 | Drift monitor (live graph with zero-line) |
| 3.7 | Telemetry dashboard (speed, position, heading) |
| 3.8 | Trajectory viewer (2D top-down, REC+PLAY overlay) |
| 3.9 | Pico HID serial integration (auto-detect, health check) |
| 3.10 | Recording save/load (.tas format with zstd compression) |
| 3.11 | Undo ring (5 deep, auto-save before destructive ops) |
| 3.12 | Log panel (ring buffer reader, severity filter, search) |
| 3.13 | Dump to file (diagnostics + transitions + drift + coords) |
| 3.14 | Playback speed control (0.25x / 0.5x / 1x / 2x / 4x) |
| 3.15 | Frame stepping (pause, step forward, step backward, scrub) |
| 3.16 | Recording markers and annotations |
| 3.17 | Keyboard shortcuts (see section 4.8) |
| 3.18 | Ghost comparison (load second recording, overlay) |

**Exit criteria**: Full REC/PLAY/CONT workflow through egui UI, with save/load, undo, timeline editing, telemetry, and dump-to-file.

### Phase 4: Test infrastructure

**Goal**: Automated regression + acceptance testing, CI-ready.

| Task | Description |
|------|-------------|
| 4.1 | Port steering pattern DSL from Lua |
| 4.2 | Port 4-gate verification |
| 4.3 | Port regression suite (15 test cases) |
| 4.4 | Port 3-phase acceptance test |
| 4.5 | Mock input mode (no hardware) |
| 4.6 | CSV output for regression results |
| 4.7 | Determinism certificate output |
| 4.8 | End-to-end validation: full suite green |

**Exit criteria**: `tas_test --mode regression` passes all 15 cases with zero drift.

### Phase 5: Polish + integration

| Task | Description |
|------|-------------|
| 5.1 | Integrate into Display_Config launcher (optional TAS toggle) |
| 5.2 | Error handling: game crash recovery, DLL unload safety |
| 5.3 | Cross-platform egui build verification (Windows primary, Linux build check) |
| 5.4 | Documentation: README, user guide |
| 5.5 | Input macros (save/load common sequences) |
| 5.6 | Multi-segment recording support |
| 5.7 | Input analysis tools (histogram, transition density, symmetry) |
| 5.8 | Trajectory analysis (optimal line, turn radius, sector splits) |

### Phase 6: Advanced (future, separate issues)

| Task | Description | Difficulty |
|------|-------------|------------|
| 6.1 | Save states (memory snapshot/restore) | Very hard |
| 6.2 | Yaw/pitch extraction (rotation matrix RE) | Medium |
| 6.3 | 3D trajectory viewer (WebGPU or egui 3D) | Hard |
| 6.4 | DLL merger (TAS into Display_Config) | Medium |
| 6.5 | Online recording repository | Medium |
| 6.6 | Video export with stats overlay | Hard |
| 6.7 | Netplay determinism validation | Hard |

---

## 8. Key Addresses and Constants

These are the game addresses the DLL must hook or read. All relative to module base.

### Supreme_Game.dll (SG)

| Symbol | Offset | Purpose |
|--------|--------|---------|
| Supreme::Cycle | +0x13FE40 | Cave 2 hook site |
| Tick clamp | +0x25C81 / +0x426000 | Cave 5 hook site |
| Player base | [SG+0x1D5450] | Player/keyboard root pointer |
| DI buffer | [SG+0x1D5450]+0x530+0x30 | 256-byte keyboard buffer |
| Keyboard object | [SG+0x1D5450]+0x530 | Byte input flags base |
| Byte flags (arrows) | keyboard+0x455..+0x458 | LEFT/RIGHT/UP/DOWN |
| Byte flags (modifiers) | keyboard+0x440, +0x441 | SHIFT, JUMP |
| Action state | [SG+0x1D5450]+0xC4..+0xD0 | LEFT/UP/RIGHT/DOWN flags |
| VK lookup table | SG+0x9AD8 | 4 bytes/entry, VK -> game keyIndex |
| Player X | [player]+0xF8 | X coordinate |
| Player Y | [player]+0xFC | Y coordinate |
| Player Z | [player]+0x100 | Z coordinate |
| Physics sub-object | [player]+0x110 | Physics pointer |
| Physics position | [physics]+0x3B4/+0x3B8/+0x3BC | X/Y/Z |
| Physics prev pos | [physics]+0x3C0/+0x3C4/+0x3C8 | Previous X/Y/Z |
| Frame counter | (per-cave tracking) | Game frame number |
| `__ftol` | 0x0046204C | Float-to-int (can return garbage for large values) |

### HMG_Cetsup_Win32.dll

| Symbol | Offset | Purpose |
|--------|--------|---------|
| Key handler (down) | +0x3940 | Cave 1C hook (keyDown path) |
| Key handler (up) | +0x3980 | Cave 1C hook (keyUp path) |
| BB3B10 observer | +0x3B10 | Cave 1D hook + direct call target |
| WinMsg context | +0x1830 | Message dispatch entry |

### Game key codes

| Key | VK | Game keyIndex | Bit |
|-----|-----|---------------|-----|
| LEFT | 0x25 | 0x3A | 0 |
| RIGHT | 0x27 | 0x3B | 1 |
| UP | 0x26 | 0x38 | 2 |
| DOWN | 0x28 | 0x39 | 3 |
| JUMP (CTRL) | 0x11/0xA2 | 0x27 | 4 |
| SHIFT | 0x10/0xA0 | 0x24 | 5 |
| F5 (restart) | 0x74 | — | 6 (Pico only) |

### Input bitmask protocol

Single byte bitmask used in input_log, Pico serial, and shared memory:

```
bit 0 = LEFT     (0x01)
bit 1 = RIGHT    (0x02)
bit 2 = UP       (0x04)
bit 3 = DOWN     (0x08)
bit 4 = JUMP     (0x10)
bit 5 = SHIFT    (0x20)
bit 6 = F5       (0x40, Pico only)
bit 7 = reserved
0xFF = release all (Pico special)
0xFE = hard MCU reset (Pico special)
0xFD = soft USB reconnect (Pico special)
```

---

## 9. Why Zero Drift Works

Three conditions must hold for zero drift:

### 1. Input path parity

REC and PLAY must drive the same game subsystems. In both modes, Cave 2:
- Writes the DI buffer
- Sets byte input flags on the keyboard object
- Sets action_state dwords
- Calls BB3B10 on transitions

All three subsystems are driven at the same point in the frame, from the same code.

### 2. Timing parity

Both REC and PLAY issue BB3B10 calls from inside `Supreme::Cycle`, via Cave 2.

During REC, Cave 1D blocks the natural `+3940 -> BB3B10` chain. BB3B10 only fires from Cave 2's explicit calls on input transitions.

During PLAY, Cave 1C blocks external dispatch. Cave 2 calls BB3B10 directly.

Result: BB3B10 fires from Cave 2 in both modes, at the same timing, with the same number of calls per transition.

### 3. Frame alignment

The arm-wait gate ensures PLAY starts at the exact same frame offset as REC. The game's tick computation is deterministic at a given framerate. Physics timeline aligns perfectly.

### Why previous approaches failed

| Approach | Drift | Root cause |
|----------|-------|------------|
| CE keyDown | ~5 units | Sets GAKS but not DirectInput. REC/PLAY input path asymmetry. |
| Buffer-only writes | ~41 units | Missing BB3B10 observer notifications. Physics state not updated. |
| Asymmetric Cave 1C (PLAY only) | ~0.5-1.4 units | +3940 fires during REC at Handle_OS_Messages time, Cave 2 fires at Cycle time. One-frame timing offset on transition frames. |
| PostMessage | ~variable | Doesn't reach DirectInput. Timing controlled by OS message queue. |
| Variable tick counts | ~variable | Different tick counts between REC/PLAY frames. Non-deterministic physics. |

---

## 10. Risk Register

| Risk | Impact | Mitigation |
|------|--------|------------|
| SafetyHook can't hook Cave 2 site (push ebp prologue) | Blocks everything | Phase 1.3 validates this first (DONE, it works). Fallback: manual jmp patch. |
| Two DLLs conflict on SafetyHook allocator | Both DLLs broken | Phase 1.6 tests coexistence (DONE, no conflict). Fallback: merge into single DLL. |
| Named shared memory permissions on non-admin | UI can't connect | Use "Local\\" prefix (session-scoped). Both processes run as same user. |
| Keyboard input drift without Pico | Zero-drift broken | CE synthetic keys caused drift, not real keyboard. Real keyboard goes through DirectInput same as Pico. |
| Game updates break pattern-scanned addresses | Hooks fail silently | Same risk Display_Config already handles. Pattern scan returns nullptr, DLL logs error and skips. |
| Linux egui build | Won't compile serialport/Windows IPC | Phase 5.3. Conditional compilation: UI builds on Linux without IPC or Pico features. DLL is Windows-only. |
| FPU state corruption in hooks | Subtle drift or crashes | FSAVE/FRSTOR around all hook bodies. Integer-width memcpy for float clearing. No x87 in callbacks. |
| Focus stealing from log calls | Game loses focus, input breaks | Deferred logging only. Write to ring buffer, never call printf/format/Log in callbacks. |
| Undo ring overflow | Lost work | Auto-save to disk before ring wraps. Configurable ring depth (default 5). |
| Savestate memory corruption | Game crash | Start with fast-forward approach. Savestates are Phase 6 experimental only. |

---

## 11. Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| SafetyHook | (amalgamated, in-tree) | Code cave hooking |
| Zydis | (amalgamated, in-tree) | x86 instruction analysis for SafetyHook |
| nlohmann/json | (in-tree) | Config file parsing |
| eframe/egui | latest | UI framework |
| egui_plot | latest | Drift graph, speed graph, telemetry charts |
| serialport | latest | Pico HID serial (optional) |
| windows-sys | latest | Named shared memory, process APIs |
| zstd | latest | Recording file compression |
| chrono | latest | Timestamps for recordings and logs |

---

## 12. Success Criteria

1. **Zero drift**: REC via keyboard -> PLAY via DLL -> maxDriftX=0.0, maxDriftZ=0.0
2. **No CE dependency**: Entire workflow runs without Cheat Engine
3. **No Pico required**: Regular keyboard recording achieves zero drift
4. **Regression green**: All 15 test cases pass
5. **Acceptance green**: 3-phase acceptance test passes (steering + replay_steered + zero_drift)
6. **Cross-platform UI**: egui compiles and runs on Windows and Linux (DLL is Windows-only)
7. **Save/Load**: Recordings persist to disk and can be restored across sessions
8. **Continue Record**: Replay-to-frame-N then splice into live recording with zero drift up to splice point
9. **Undo**: Auto-save ring buffer protects against accidental overwrites
10. **Input timeline editing**: Frame-level click-to-edit with visual feedback
11. **Telemetry**: Live speed, position, and heading readouts during REC and PLAY
12. **Trajectory viewer**: 2D top-down path comparison between REC and PLAY
13. **Frame stepping**: Single-frame advance and reverse
14. **Determinism certificate**: Automated proof that recordings are reproducible
15. **Ghost comparison**: Load and overlay a second recording for A/B analysis

---

## 13. Design Feedback (2026-03-17)

Board review of egui UI screenshots:

### Accepted
- **Rename**: "Supreme TAS" -> "SSB Inspect" (all references)
- **Dark theme**: Default to egui dark mode
- **Fix icons**: Green squares in hook status are broken -- replace with proper indicators
- **Config panel**: inject_mode/force_fixed_tick/etc are debug-only. Collapse by default.
- **Drift graph**: Keep for debugging. In normal operation drift should always be zero.

### Priority: Test Suite
Port the CE test suite to the native DLL pipeline. Pico HID for recording, DLL injection for playback, zero drift verification.

### Future Work (separate issues)
- **SSB-65**: Save states -- snapshot/restore game memory. Very hard. Backlog.
- **SSB-66**: TM Interface-style telemetry -- speed, x, y, z, yaw in UI. Needs RE work. Backlog.

---

## Appendix A: Glossary

| Term | Definition |
|------|-----------|
| **Cave** | A code hook injected into the running game process. Named by convention (Cave 1C, 2, etc.). |
| **BB3B10** | The game's observer notification function at HMG_Cetsup_Win32.dll+0x3B10. Notifies the game engine of key press/release events. Essential for steering physics. |
| **DI buffer** | DirectInput keyboard state buffer. 256 bytes, one per game key code. 0x80 = pressed, 0x00 = released. |
| **GAKS** | GetAsyncKeyState — Windows API that returns real-time key state. Used by Cave 2 REC path for input sampling. |
| **Arm-wait gate** | Synchronization mechanism that ensures PLAY starts at exactly the same frame offset as REC. DLL waits for the correct frame counter value before latching playback. |
| **Transition** | A frame where the input bitmask changes from the previous frame. BB3B10 is called on transitions. |
| **Pico HID** | Raspberry Pi Pico 2 (RP2350) configured as a USB HID keyboard. Sends real keystrokes that go through the full OS -> DirectInput pipeline. |
| **Zero drift** | The state where REC coordinates and PLAY coordinates are bit-identical for every frame. maxDriftX = 0.0, maxDriftZ = 0.0. |
| **Continue Record** | A workflow where the DLL replays frames 0..N, then switches to live recording from frame N+1. Allows re-recording from any point without losing earlier frames. |
| **Force direct** | Configuration flag (force_direct=2) that makes Cave 2 call BB3B10 directly on input transitions, rather than relying on the natural message pump chain. |
| **Inject mode** | Configuration that selects the input injection strategy. Mode 6 is proven zero-drift. |

## Appendix B: File Inventory

| File | Location | Purpose |
|------|----------|---------|
| `TAS-caves.asm` | `cheatengine-mcp-bridge/tasks/` | CE auto-assembler caves (reference implementation) |
| `TAS-ui.lua` | `cheatengine-mcp-bridge/tasks/` | CE Lua UI (being replaced by SSB Inspect) |
| `code.py` | `cheatengine-mcp-bridge/tasks/pico/` | Pico 2 firmware (USB HID keyboard) |
| `serial.lua` | `cheatengine-mcp-bridge/tasks/pico/` | CE Lua serial interface to Pico |
| `_full_drift_test.lua` | `cheatengine-mcp-bridge/tasks/drift-fix/` | 3-phase acceptance test harness |
| `_drift_regression_suite.lua` | `cheatengine-mcp-bridge/tasks/drift-fix/` | Regression test runner (15 cases) |
| `zero_drift_solution.md` | `cheatengine-mcp-bridge/tasks/drift-fix/docs/` | Technical design of the zero-drift solution |
| `acceptance_criteria.md` | `cheatengine-mcp-bridge/tasks/drift-fix/docs/` | Success contract |
| `tas_state_map.md` | `cheatengine-mcp-bridge/tasks/drift-fix/docs/` | tasState memory layout |
| `supreme-snowboarding-memory.md` | `cheatengine-mcp-bridge/tasks/archive/` | Comprehensive RE notes (329 lines) |
| `tas.md` | `SS-Dat-Info/tasks/` | This document |
| `Cargo.toml` | `SS-Dat-Info/TAS/` | Rust workspace root |
| `tas_ui/` | `SS-Dat-Info/TAS/` | SSB Inspect egui application |
| `tas_test/` | `SS-Dat-Info/TAS/` | Test runner CLI |
| `tas_shared/` | `SS-Dat-Info/TAS/` | Shared IPC types |
