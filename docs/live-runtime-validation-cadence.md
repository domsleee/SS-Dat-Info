# Live Runtime Validation Cadence (Hardware E2E)

Operational cadence and runbook for hardware-backed Supreme TAS validation (`tas_test` against live game runtime).

## Ownership

- Founding Engineer owns execution of manual live-runtime gates and evidence capture.
- CEO owns queue progression, prioritization, and go/no-go decisions when failures occur.
- Board reviews only when escalation requires strategy or scope changes.

## Cadence Recommendation

| Trigger | Required Suite | Owner | Exit Criteria |
|---|---|---|---|
| Daily (business days) | Sentinel live check: `cont-reliability --iterations 10 --splice 2400 --speed 32 --profile taps --tap-ticks 8` | Founding Engineer | PASS banner + zero drift + `cover=ok` + `fwd=ok` |
| Before release/handoff/runtime-significant merge | Full manual live-runtime lane from `docs/tas-quality-gates.md` | Founding Engineer | All commands exit `0` and pass signatures present |
| Weekly hardening (or after crash-heavy week) | Sentinel check at `--speed 32`, `--speed 64`, `--speed 100` | Founding Engineer | All three speed tiers pass with zero drift |
| Any runtime-sensitive change (shared memory, REC/PLAY/CONT flow, speed/restart/drift logic) | Full manual live-runtime lane before issue close | Founding Engineer | Validation evidence attached to issue comment |

## Preconditions (Mandatory)

1. Ensure exclusive runtime control:
   - Close extra runtime writers/readers that can interfere with shared memory.
   - Keep only the intended test path active.
2. Start from clean runtime state:
   - `Set-Location C:\Users\user\git\cheatengine-mcp-bridge`
   - `$env:NO_CE = '1'` (unless CE is explicitly required for the task)
   - `nu skills/revive-supreme/scripts/revive-supreme.nu`
3. Set stable output location:
   - `Set-Location C:\Users\user\git\SS-Dat-Info\TAS`
   - `$env:TAS_TEST_OUTPUT = 'C:\Users\user\git\SS-Dat-Info\TAS\artifacts\latest'`
4. Confirm game is fully in-run before executing test commands.

If the game crashes or runtime state is stale, run `revive-supreme` again and restart the sequence from step 1.

## Execution Runbook

Use this wrapper to capture wall-clock start/end/duration plus command logs:

```powershell
function Invoke-TasGate {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Command
    )

    $start = Get-Date
    Write-Host ("[{0}] START {1}" -f $start.ToString("s"), $Name)
    Invoke-Expression $Command
    $exit = $LASTEXITCODE
    $end = Get-Date
    $duration = New-TimeSpan -Start $start -End $end
    Write-Host ("[{0}] END {1} exit={2} duration={3}" -f $end.ToString("s"), $Name, $exit, $duration)

    if ($exit -ne 0) {
        throw "Gate failed: $Name (exit $exit)"
    }
}
```

Example sentinel execution:

```powershell
Invoke-TasGate -Name "cont-reliability-32x" -Command "cargo run --release --bin tas_test -- cont-reliability --iterations 10 --splice 2400 --speed 32 --profile taps --tap-ticks 8"
```

For release/handoff validation, execute the full command set from `docs/tas-quality-gates.md` under "Manual Live-Runtime Lane".

## Required Evidence Per Run

- Wall-clock lines for every gate: `START`, `END`, `exit`, `duration`.
- Pass signatures from command output (exact banners where defined).
- Drift summary lines:
  - `max_drift_x = ...`
  - `max_drift_z = ...`
  - summary row with `cover` / `fwd` for `cont-reliability`.
- Artifact paths produced by `tas_test` (certificates/csv/logs as applicable).
- Screenshot evidence only when UI behavior is part of the validation scope or when investigating anomalies.

## Failure Handling and Escalation

1. On first failure, perform one clean retry after `revive-supreme`.
2. If failure repeats, mark the issue `blocked` and post a concise evidence comment mentioning `@CEO` with:
   - failing command(s)
   - pass signature(s) missing
   - drift values observed
   - artifact/log path
   - whether issue should pause queue progression
3. Do not advance to the next live runtime issue unless the CEO explicitly advances the queue.

## Completion Comment Template

```md
## Live Runtime Validation Update

- Cadence tier: <daily sentinel | weekly hardening | release/handoff>
- Commands run:
  - `<exact command 1>`
  - `<exact command 2>`
- Result: <PASS/FAIL>
- Key signatures:
  - `<signature line>`
- Drift:
  - `max_drift_x = ...`
  - `max_drift_z = ...`
- Artifacts: `<path(s)>`
- Queue recommendation for @CEO: <advance next runtime issue | hold queue and investigate>
```
