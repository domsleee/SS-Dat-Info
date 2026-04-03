# SSB-296 Hook Performance + Gameplay Impact Audit

Date: 2026-04-03
Scope: `TAS/TAS_Helper/src/caves/*` hook callbacks and nearby shared-state logic.

## Executive Summary

- The hooks are intentionally gameplay-affecting during REC/PLAY. That is required for zero drift.
- The two highest-frequency hooks are `Cave2_MidCallback` and `Cave5_MidCallback`; both run every game frame and both execute `FSAVE/FRSTOR`.
- The largest avoidable overhead is in Cave 2/Cave 5 hot paths, not in install-time SafetyHook patching.
- In `MODE_OFF`, hooks still do non-trivial work each frame (telemetry reads in Cave 2, tick clamp/reset logic in Cave 5).

## Hook Inventory (frequency and cost)

## Cave 2 (`Supreme::Cycle`) — highest impact

Reference: `TAS/TAS_Helper/src/caves/cave2.hpp`

- Hook install: `create_mid` at line ~528.
- Per-frame callback: `Cave2_MidCallback` (`fsave`/`frstor`) at lines ~511-515.
- Always-on per-frame work before mode gate:
  - `frame_count++` at line ~356.
  - replay/player pointer refresh + telemetry reads/writes at lines ~358-399.
  - command processing at line ~401.
  - mode gate (`MODE_OFF` early return) only at line ~421.
- REC hot path:
  - 6x `GetAsyncKeyState` per frame (`SampleGAKS`, lines ~71-78).
  - writes DI buffer + action state every frame (`WriteDIBuffer` line ~99, `WriteActionState` line ~116, called at ~440/~442).
  - transition BB3B10 calls (`CallBB3B10OnTransitions`, line ~130, called at ~445).
- PLAY hot path mirrors REC writes and transition notifications (~468-479).

Assessment:

- This is the dominant CPU cost center because it is frame-frequency and contains the most branching/memory traffic.
- It also intentionally replaces native input timing path (by design).

## Cave 5 (tick override)

Reference: `TAS/TAS_Helper/src/caves/cave5.hpp`

- Hook install: `create_mid` at ~97.
- Per-frame callback: `Cave5_MidCallback` with `fsave`/`frstor` at ~40-70.
- Always writes `ctx.esi` clamp/override each callback (~46-55).
- When active mode, writes tick-advance constant each frame (`*g_tickAdvancePtr = 0.01 / playback_speed` at ~62-63).
- In `MODE_OFF`, explicitly restores base constant every frame (~64-66).

Assessment:

- Second hottest path. Cost is moderate but persistent.
- It can influence gameplay timing globally by design (especially if other systems try to alter the same constant).

## Cave 1C / 1D (input gates)

References:

- `cave1c.hpp`: block external key handlers unless `MODE_OFF` or `cave2_injecting` (~34/~43).
- `cave1d.hpp`: block BB3B10 in REC mode 6 unless `cave2_injecting` (~34-43).

Assessment:

- Event-driven, not frame-driven.
- Low runtime cost versus Cave 2/5.
- Critical for deterministic timing symmetry and therefore intentionally gameplay-affecting in REC/PLAY.

## Replay capture hook

Reference: `cave1_replay.hpp`

- `create_mid` at ~28.
- Captures `replay_ptr` from `ECX` when idle or pointer unset (~37).

Assessment:

- Very small callback body.
- Low risk/cost.

## Gameplay Impact Audit

## Intended gameplay changes (required for TAS)

- Cave 1C blocks natural handler path in REC/PLAY and lets Cave 2 drive input timing.
- Cave 1D blocks natural BB3B10 path in REC mode 6 and allows Cave 2 direct transition calls.
- Cave 2 writes DI buffer/action-state and calls BB3B10 directly on transitions.
- Cave 5 modifies effective tick handling and playback-speed behavior.

These are deliberate and required to preserve REC/PLAY symmetry.

## Residual behavior changes outside active TAS mode

- Cave 2 still performs telemetry/pointer work every frame before `MODE_OFF` early return.
- Cave 5 still clamps tick and restores base tick-advance constant each callback.

These do not appear to break normal flow by design, but they are still non-zero behavioral footprint when idle.

## Candidate Performance Experiments

1. Add cycle counters around each hook callback.
   - Capture per-hook call count + total cycles + max cycles.
   - Expose in shared state and UI diagnostics panel.
2. Measure idle overhead (`MODE_OFF`) with TAS helper loaded vs unloaded.
   - KPI: frame-rate proxy (`frame_count` delta / second) and CPU usage.
3. Measure active REC overhead by mode.
   - Compare no-input, constant-input, and high-transition patterns.
4. A/B Cave 2 telemetry block.
   - Toggle "telemetry every frame" vs "telemetry every N frames when MODE_OFF".
5. A/B Cave 5 speed write policy.
   - Current: write tick-advance every callback.
   - Candidate: write only when effective value changes.

## Candidate Optimizations (low risk first)

1. Move Cave 2 mode gate earlier for non-essential telemetry work when `MODE_OFF`.
2. Avoid duplicate player-memory reads in Cave 2 (current code reads/upserts live position and then captures coords again in REC/PLAY).
3. Cache computed tick-advance value in Cave 5 and only write on value changes.
4. Add optional compile/runtime flag to disable heavy diagnostics in production runs.

## Notes

- Install-time SafetyHook overhead is negligible relative to frame-loop callback overhead.
- `FlushPendingLog()` in Cave 2 is currently not referenced; deferred file logging path appears effectively dormant.
