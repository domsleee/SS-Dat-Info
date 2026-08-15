# TAS test-coverage plan (2026-06-01)

Written after a regression cascade where the test suite stayed **green** while
record, playback, and 64× catch-up all broke. Captures what tests are missing
and how to add them. (Designed via three focused sub-agents; this is the synthesis.)

## STATUS (2026-06-01, after codex critique + implementation)

Built and **validated on the good 1d32308 baseline** (all live against the running tool):

- **`rec-start`** (start-position / mid-fall) — DONE, hardened per codex. Now XYZ
  motion + cumulative path-length, `MIN_FRAMES=300`, and TWO independent gates:
  `stationary_prefix ≥ 60` AND `≥120` moving frames after the countdown (the ratio
  alone was foolable by a late-countdown arm). File-checked vs 3 known-good recordings
  (prefix 298–320, ratio 0.000) and live (605 frames, prefix 196, ratio 0.000 → PASS).
  A mid-fall arm fails on prefix≈0 AND ratio≈1 — two independent gates.
- **`catchup-speed`** (64× collapse) — DONE, new. Median `T_1x/T_64x` to the same
  splice frame; floor **8×**. Measured **32.8×** on baseline (T_1x=21.99s, T_64x=0.67s),
  re-measured **32.7×** on 2026-08-14 after trimming TRIALS 5→3 (T_1x=21.99s,
  T_64x=0.67s — the trim does not move the verdict).
  **Correction (2026-08-14):** the original claim that the splice lands bit-identical
  at 1× and 64× (end-coord delta 0.0000) no longer holds — measured delta **133.66**.
  This is NOT a speed effect: the 1× trials alone span three distinct end coords
  (spawn-bucket lottery). The test is unaffected because it asserts only the ratio,
  which stays bucket-independent; but do not rely on cross-speed end-coord equality.
- **`play-pace`** (skip-ahead) — DONE, new. 1× PLAY of a 2000-frame window must take
  ≈ native `frames×0.01s`. Measured **ratio 1.000** (19.999s vs 20.000s). Band [0.85,1.30].
- **`fe-cont-stress`** — its bogus-looking `effective_x` column replaced with an
  honest in-sweep `vs_1x` speedup (measured 1× wall ÷ this speed's wall).

### Suite-wide oracle change (2026-08-14): drift now includes Y

`DriftResult::is_zero()` / `is_within()` and `compute_drift_between` compared **X and Z
only** — every drift-based mode was blind to vertical position. A regression that changed
height while preserving the ground track (jump arcs, terrain following) reported bit-exact
"zero drift". Y is now compared alongside X/Z, and the 13 call sites that had inlined
`max_drift_x == 0.0 && max_drift_z == 0.0` instead of calling `is_zero()` were routed
through it (they would otherwise have silently kept the old two-axis oracle).

Verified live: Y drift is **0.000000000** on `f5`, `replay` ×3, `cont-splice-frame`, and
`fe10065-cont` 8/8 at both 64× and 256× — so the tightening costs nothing on a healthy
build — and **15.24** on `smoke`, which is deliberately not F5-aligned. Y is a live,
discriminating signal, not a dead axis.

Still X/Z-only: the **DLL's** `state.max_drift_x/z` that `tas_ui` displays
(`shared_state.hpp:104`). Fixing that needs a shared-state version bump, so it is left
as follow-up — the test oracle and the on-screen number now differ.

### Empirical corrections to this plan's own assumptions

- **The 2× confound is a REC-phase artifact only.** 1× PLAY/catch-up is **clock-gated**
  and runs at *exactly* native: `playback_pos` advances at **100 frames/sec** at 1×
  (2199 frames→21.986s; 2000→19.999s). So for **1× PLAY, absolute pace IS assertable**
  (play-pace does exactly this). The "never assert absolute wall-clock" rule below
  applies to REC and to cross-speed comparisons, not to clock-gated 1× PLAY.
- **codex's "frames ≠ ticks, so frames/100 is invalid" is empirically false here.** At
  1× one render cycle = one 0.01s physics tick (100/sec, measured). So `speed.rs`,
  `speed_reset.rs`, and `harness.rs:820` (`splice_frame*10ms` baseline) are **correct**
  and were intentionally left untouched. The old `effective_x` formula also gave the
  right answer (32.8×); the refactor was cosmetic, not a fix.

### Open / dropped

- **`rec-repro`** (run-quality / scuffing) — NOT built; needs a design decision. codex:
  trajectory-only catches *nondeterministic* scuffing but a *smooth-but-wrong* restart
  passes it. Robust ground truth needs a committed golden trajectory or DLL restart-ready
  telemetry. K=5 is too small (adaptive to 20–30 buckets). See the question to the user.
- **`ghost-policy`** axis — DROPPED for v6: the ghost-flag shared-state bytes and DLL
  behavior it assumed do not exist at this commit (reverted with the ghost-era work).
- **DLL tick telemetry** (cave5 `tick_count` etc.) — NOT needed for catch-up or play-pace
  (wall-time ratios suffice, proven above). Only relevant if rec-repro goes the telemetry route.

## Root cause: the suite is a single-axis oracle

Nearly every mode (`f5`, `regression`, `acceptance`, `replay`, `cont-reliability`,
`fe-cont-*`, `drift-speed`, `save-reload`, `stop-play-flake`) ultimately asserts one
thing: `drift::compute_drift(rec_coords, play_coords).is_zero()` — REC vs PLAY
coordinate bit-match (the **determinism** axis). Every regression in this episode
was **self-consistent** and therefore invisible to drift:

- REC armed mid-fall → REC and PLAY both start mid-fall → drift 0.
- REC armed before physics settled (scuffed) → still reproducible → drift 0.
- Playback "skipped ahead" (wrong wall-clock speed) → path reproduced, drift 0.
- 64× catch-up collapsed to ~native → prefix still correct → drift 0.

Fix = **axis separation**: six axes, each with its own oracle and ground truth.

## The 2× environment confound (design constraint)

The harness runs the game at **~2× real time** (a 6.15 s wall-clock REC accumulated
1203 physics ticks = 12.03 s of simulated time at the documented 0.01 s/tick). This
is pre-existing and environment-specific (unfocused/uncapped), NOT the user's focused
session. **Rule: never assert an absolute wall-clock duration.** Use only:
- **same-env ratios** (REC-vs-PLAY wall, or T₁ₓ/T₆₄ₓ) — the 2× cancels; or
- **DLL-exposed integer invariants** (tick counts, a "clock-forced" counter) — unitless.

## Axes → guarding test → oracle

| Axis | Test | Exact assertion | Ground truth | 2×-safe? |
|---|---|---|---|---|
| Start-position (mid-fall) | `rec-start` (exists; harden) | `count≥120 && travel≥1 && start_speed/mid_speed < 0.30`; stationary-prefix length ≥ 60 | head vs mid per-frame motion of `rec_coords` | yes (ratio) |
| Run-quality (scuffed) | **`rec-repro`** (new) | same fixed input from K=5 *independent* restarts; a clean ≥3/5 bucket must agree (normalized) to <2.0u, prefix <0.05u, identical prefix `esi_log` | REC-vs-REC across restarts | yes |
| Playback-speed (skip-ahead) | `play-pace` (new) | `T_rec_window / T_play_window ∈ [0.7,1.6]` at 1× | REC-vs-PLAY wall over same frame window | yes (ratio) |
| | (stronger, optional) `play-pace-clockforce` | plain PLAY: `cave6_force_count == 0`; forced control: `≈ playback_pos` | DLL counter | yes (count) |
| Catch-up-speed (64× collapse) | `fe-cont-stress` **+ assert** | add `effective_x ≥ FLOOR` (e.g. ≥8×) to the pass condition; or `catchup-speed`: `T₁ₓ/T₆₄ₓ ≥ 8` | speed ratio | yes (ratio) |
| Determinism | drift suite (exists) | `compute_drift().is_zero()` — **X/Y/Z since 2026-08-14** | bit-match | yes |
| Ghost | **`ghost-policy`** (new) | default: ghost flag bytes == 0 during TAS, == saved on STOP; `native`/`enable` as labeled | ghost-flag bytes (expose via shared state) | yes (state) |

## Two sharpest techniques

1. **Catch a scuffed run when drift is 0**: a scuffed restart is self-consistent but
   **not reproducible across fresh restarts**. Record the same fixed input from several
   independent restarts; clean → they agree (+ identical countdown `esi_log`), scuffed →
   same spawn bucket yet divergent trajectory. No golden file needed (`rec-repro`).
   Optional stronger anchor: a committed golden `.tasrec` (`rec-golden`) compared
   shift-invariant on the spawn bucket but shape-strict.
2. **Beat the 2× confound with ratios / counters**, never absolute seconds.

## Two latent gaps found (NOT caused by this session — already in the tree)

- **`fe_cont_stress.rs` computes `effective_x` (catch-up speed) but never asserts it**
  — `run()` only gates on `eventual_matched == iterations` (~line 433). That is exactly
  the unguarded 64×-collapse hole. Fix: fold an `effective_x >= FLOOR` check into the
  pass condition (calibrate FLOOR on a known-good DLL *in this harness*).
- **Ghost policy has zero automated coverage** — suppress/restore lives only in
  `cave2.hpp` (`ConfigureGhostPolicy`/`SuppressGhostSimulation`/`RestoreGhostSimulation`,
  `GetGhostFlags`). Add a `ghost-policy` test reading the flag bytes before/during/after.

## One optional DLL change
`cave6_force_count` (volatile u32, appended to `TasSharedState`, version 15→16, size pin
78,522,568→78,522,572, reset in `CMD_ARM_PLAY`/`CMD_ARM_CONTINUE`, incremented in cave6's
force branch). Lets `play-pace-clockforce` assert "clock not forced during plain PLAY"
directly — env-immune. Not required: the harness-only `play-pace` ratio catches the same
bug without DLL churn.

## Recommended order (all harness-only except the optional DLL counter)
1. ~~Assert `effective_x` in `fe-cont-stress`~~ — **SUPERSEDED** by `catchup-speed`,
   which asserts the same 64×-collapse property on a dedicated ratio. `fe-cont-stress`
   keeps its speedup column as an informational sweep on purpose (see the comment in
   `fe_cont_stress.rs`); adding a second assertion there would duplicate the gate.
2. `rec-repro` — the scuffing catcher.
3. `play-pace` — the skip-ahead catcher (REC-vs-PLAY ratio).
4. Harden `rec-start`; add `ghost-policy`.
5. **Mandatory pre-deploy gate**: one test per axis must pass before any `TAS_Helper.dll`
   is copied to the game folder. Minimal Tier-1 set: `rec-start`, `regression`,
   `clock-time`/`play-pace`, `speed`, `speed-reset`, `fe-cont-reliability`,
   `fe-cont-stress`(+floor), `ghost-policy`. "If the F5-timing change had to clear those,
   it could not have shipped green."

Re-baseline (don't trust hardcoded tps) for the semi-absolute existing tests
(`escape-speedup`, `pause-resume` resume-window bands) so they hold on both the 2× harness
and a focused 1× machine.
