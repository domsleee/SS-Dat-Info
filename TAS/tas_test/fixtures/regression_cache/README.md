These `.tas` files are trusted live regression baselines consumed by `cargo run --release --bin tas_test -- mock`.

They were copied from a validated `regression_cache` run and are intentionally checked into the repo so clean CI runners can compare PLAY output against real `rec_coords` data instead of zero-filled mock caches.

Each file name uses the canonical ordinal from `build_cases()` in `TAS/tas_test/src/regression.rs`:

- `01_l.tas`
- `02_r.tas`
- ...
- `15_shift_left_right.tas`

Filtered reruns must preserve those ordinals so `tas_test -- mock` continues to load the same authoritative live baseline for each case.

Refresh process:

1. Run `cargo run --release --bin tas_test -- regression` against a known-good live runtime.
2. Copy the resulting `regression_cache/*.tas` files into this directory.
3. Re-run `cargo run --release --bin tas_test -- mock` and the fast lane before committing.
