# Supreme TAS E2E Test Guide

This guide shows how to run the live `tas_test` E2E checks used for CONT reliability.

For recurring schedule/ownership/escalation policy, see `docs/live-runtime-validation-cadence.md`.

## Scope

Main E2E command:

```powershell
cargo run --release --bin tas_test -- cont-reliability ...
```

Covers:

- CONT splice success (`PLAY -> REC` at the splice frame)
- zero drift across replayed prefix
- replay coverage up to splice frame
- movement-quality metrics for the replayed prefix

## Prerequisites

- Windows + PowerShell
- Supreme Snowboarding installed and runnable
- `SS-Dat-Info` repo at `C:\Users\user\git\SS-Dat-Info`
- `cheatengine-mcp-bridge` repo at `C:\Users\user\git\cheatengine-mcp-bridge`
- `nu` available (for `revive-supreme`)
- Cave hooks active (`tas_test` will print hook/liveness status)

## 1) Quick Sanity Run (single tier, short)

```powershell
Set-Location C:\Users\user\git\cheatengine-mcp-bridge
$env:NO_CE = '1'
nu skills/revive-supreme/scripts/revive-supreme.nu

Set-Location C:\Users\user\git\SS-Dat-Info\TAS
cargo run --release --bin tas_test -- cont-reliability --iterations 1 --splice 2400 --speed 32 --profile taps --tap-ticks 8
```

## 2) Full E2E Reliability Sweep (32x / 64x / 100x)

Run a clean revive before each tier:

```powershell
$tasRepo = 'C:\Users\user\git\SS-Dat-Info\TAS'
$bridgeRepo = 'C:\Users\user\git\cheatengine-mcp-bridge'
$reviveScript = Join-Path $bridgeRepo 'skills/revive-supreme/scripts/revive-supreme.nu'
$speeds = @(32, 64, 100)

foreach ($speed in $speeds) {
    Set-Location $bridgeRepo
    $env:NO_CE = '1'
    nu $reviveScript

    Set-Location $tasRepo
    cargo run --release --bin tas_test -- cont-reliability --iterations 10 --splice 2400 --speed $speed --profile taps --tap-ticks 8
}
```

Expected pass result per tier:

- `*** CONT RELIABILITY PASSED: 10/10 splice cycles clean ***`
- `max_drift_x = 0.000000000`
- `max_drift_z = 0.000000000`
- `cover = ok` in summary table
- `fwd = ok` in summary table

## 3) File-Backed CONT Diagnostic (FE-decent compatibility baseline)

This command is compatibility evidence only. Do not treat it as a required release or CI gate while replay-vs-CONT baseline semantics remain split.

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info\TAS
cargo run --release --bin tas_test -- cont-reliability --file recordings/FE-decent.tasrec --splice 2400 --iterations 1 --speed 12
```

## 4) Replay-Only Compatibility Diagnostic

This command is also diagnostic-only for FE-decent. Capture the output for comparison, but do not fail a heartbeat solely on this result unless the active issue is specifically about FE-decent compatibility.

```powershell
Set-Location C:\Users\user\git\SS-Dat-Info\TAS
cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose
```

## 5) Compare Z-axis Travel vs FE-decent

Use this one-off script to inspect `rec_coords` from `FE-decent.tasrec`:

```powershell
@'
import json, struct, pathlib
path = pathlib.Path(r"C:/Users/user/git/SS-Dat-Info/TAS/recordings/FE-decent.tasrec")
d = path.read_bytes()
meta_len = int.from_bytes(d[:4], "little")
meta = json.loads(d[4:4+meta_len])
n = meta["recorded_count"]
coords_off = 4 + meta_len + n
zs = []
off = coords_off
for _ in range(n):
    off += 8
    z = struct.unpack_from("<f", d, off)[0]
    off += 4
    zs.append(z)
def stats(k):
    z = zs[:k]
    fwd = back = flat = 0
    for i in range(1, len(z)):
        dz = z[i] - z[i-1]
        if dz > 1e-4: fwd += 1
        elif dz < -1e-4: back += 1
        else: flat += 1
    return {
        "count": len(z),
        "start_z": z[0],
        "end_z": z[-1],
        "net_z": z[-1]-z[0],
        "min_z": min(z),
        "max_z": max(z),
        "range_z": max(z)-min(z),
        "steps": {"fwd": fwd, "back": back, "flat": flat},
    }
print("full", stats(len(zs)))
print("prefix_2400", stats(2400))
'@ | python -
```

## Troubleshooting

- `ERROR: Cave 2 not firing`
- Hooks are not active. Re-run revive and confirm the game is fully in-run before launching test.
- `Could not complete CONT splice after 40 retries`
- Position/anchor matching did not converge. Re-run revive and repeat the tier.
- Drift not zero
- Ensure config is at proven settings printed by test (`fft=0`, `inject_mode=6`, `force_direct=2`).
- `fwd = rev` in summary
- Prefix did not satisfy forward-progress quality gate (net forward/range/forward-step requirements).
