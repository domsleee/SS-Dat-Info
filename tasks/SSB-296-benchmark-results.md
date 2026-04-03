# SSB-296 Cave Benchmark Results

Date: 2026-04-03
Build: `TAS_SHARED_VERSION=6` (hook cycle counters added)
Runner: `T:\Games\SupremeORIG\TAS_Helper\tas_test.exe`

## Command

```powershell
tas_test.exe benchmark --repeats 3 --frames 500
```

## Method

- Frame-window benchmark (500 Cave2 frames per scenario) to reduce wall-clock jitter.
- 3 repeats, mean reported.
- Per-hook counters measured in DLL with `__rdtsc`:
  - `calls`
  - `cycles_total`
  - `cycles_max`
  - derived `avg cycles/call` and `cycles/frame`

## Summary (mean of 3 repeats)

## IDLE_OFF

- FPS: `100.06`
- Cave2: `513.8 cyc/call` (`max 6,390`)
- Cave5: `424.4 cyc/call` (`max 750`)
- Replay capture: `71.5 cyc/call`

## REC_NEUTRAL

- FPS: `100.02`
- Cave2: `24,053.0 cyc/call` (`max 916,331`)
- Cave5: `435.1 cyc/call` (`max 630`)
- Replay capture: `80.3 cyc/call`

## PLAY_NEUTRAL

- FPS: `100.02`
- Cave2: `744.2 cyc/call` (`max 2,040`)
- Cave5: `423.0 cyc/call` (`max 1,230`)
- Replay capture: `76.6 cyc/call`

## PLAY_HIGH_TRANSITIONS

- FPS: `100.08`
- Cave2: `15,859.1 cyc/call` (`max 717,119`)
- Cave5: `439.2 cyc/call` (`max 12,660`)
- Cave1D: `7,399.4 cyc/call` (`max 710,519`, `~999 calls / 500 frames`)
- Replay capture: `85.2 cyc/call`

## Notes

- Cave5 cost is stable and low across scenarios (~423-439 cycles/call).
- Cave2 cost is low in idle/play-neutral, high in REC and transition-heavy PLAY.
- Transition-heavy PLAY pushes Cave1D call volume and cost as expected.
- FPS stayed ~100 across all scenarios in this run, so no obvious frame-rate regression under measured load.
