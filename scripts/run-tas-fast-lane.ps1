[CmdletBinding()]
param(
    [string]$RepoRoot = (Join-Path $PSScriptRoot ".."),
    [string]$ArtifactsDir = "TAS/artifacts/fast-lane/latest"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-FullPath {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Base
    )

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return [System.IO.Path]::GetFullPath($Path)
    }
    return [System.IO.Path]::GetFullPath((Join-Path $Base $Path))
}

$repoRoot = Resolve-FullPath -Path $RepoRoot -Base (Get-Location).Path
$tasRoot = Join-Path $repoRoot "TAS"
$artifactRoot = Resolve-FullPath -Path $ArtifactsDir -Base $repoRoot
$logsDir = Join-Path $artifactRoot "logs"
$tasOutputDir = Join-Path $artifactRoot "tas_test_output"
$summaryPath = Join-Path $artifactRoot "summary.md"

New-Item -ItemType Directory -Force -Path $logsDir | Out-Null
New-Item -ItemType Directory -Force -Path $tasOutputDir | Out-Null
$env:TAS_TEST_OUTPUT = $tasOutputDir

$unitGateWorkingDirectory = $tasRoot
$unitGateCommand = "cargo test --release"
if (Get-Command just -ErrorAction SilentlyContinue) {
    $unitGateWorkingDirectory = $repoRoot
    $unitGateCommand = "just test"
}
else {
    Write-Host "just not found; using fallback unit gate: cargo test --release"
}

$gates = @(
    [pscustomobject]@{
        Name = "rust-unit"
        Kind = "required"
        WorkingDirectory = $unitGateWorkingDirectory
        Command = $unitGateCommand
        RequiredPatterns = @("test result: ok\.")
    },
    [pscustomobject]@{
        Name = "mock"
        Kind = "diagnostic"
        WorkingDirectory = $tasRoot
        Command = "cargo run --release --bin tas_test -- mock"
        RequiredPatterns = @("=== Regression Summary: 15/15 passed ===")
    },
    [pscustomobject]@{
        Name = "replay-fe-decent"
        Kind = "diagnostic"
        WorkingDirectory = $tasRoot
        Command = "cargo run --release --bin tas_test -- replay recordings/FE-decent.tasrec --iterations 1 --verbose"
        RequiredPatterns = @("Result: ZERO DRIFT in all 1 iterations")
    },
    [pscustomobject]@{
        Name = "speed-reset"
        Kind = "required"
        WorkingDirectory = $tasRoot
        Command = "cargo run --release --bin tas_test -- speed-reset"
        RequiredPatterns = @("\*\*\* SPEED RESET TEST PASSED \*\*\*")
    }
)

$results = New-Object System.Collections.Generic.List[Object]
$hardGateStatus = "PASS"
$hardFailureReason = $null
$diagnosticStatus = "PASS"
$diagnosticFailures = New-Object System.Collections.Generic.List[string]

foreach ($gate in $gates) {
    $start = Get-Date
    $logPath = Join-Path $logsDir "$($gate.Name).log"
    $status = "PASS"
    $failure = ""
    $exitCode = 0

    Write-Host ""
    Write-Host ("[{0}] START {1}" -f $start.ToString("s"), $gate.Name)
    Write-Host ("Kind: {0}" -f $gate.Kind)
    Write-Host ("Command: {0}" -f $gate.Command)
    Write-Host ("Log: {0}" -f $logPath)

    Push-Location $gate.WorkingDirectory
    try {
        & pwsh -NoProfile -Command $gate.Command 2>&1 | Tee-Object -FilePath $logPath
        $exitCode = $LASTEXITCODE
        if ($exitCode -ne 0) {
            throw "gate exited with code $exitCode"
        }

        foreach ($pattern in $gate.RequiredPatterns) {
            if (-not (Select-String -Path $logPath -Pattern $pattern -Quiet)) {
                throw "missing expected pass signature: $pattern"
            }
        }
    }
    catch {
        $failure = $_.Exception.Message
        if ($gate.Kind -eq "diagnostic") {
            $status = "DIAGNOSTIC_FAIL"
            $diagnosticStatus = "WARN"
            $diagnosticFailures.Add(("{0}: {1}" -f $gate.Name, $failure)) | Out-Null
        }
        else {
            $status = "FAIL"
            $hardGateStatus = "FAIL"
            if (-not $hardFailureReason) {
                $hardFailureReason = "{0}: {1}" -f $gate.Name, $failure
            }
        }
    }
    finally {
        Pop-Location
    }

    $end = Get-Date
    $duration = New-TimeSpan -Start $start -End $end
    Write-Host ("[{0}] END {1} status={2} duration={3}" -f $end.ToString("s"), $gate.Name, $status, $duration)
    if ($status -eq "FAIL") {
        Write-Host ("Failure: {0}" -f $failure)
    }
    elseif ($status -eq "DIAGNOSTIC_FAIL") {
        Write-Host ("Diagnostic warning: {0}" -f $failure)
    }

    $results.Add([pscustomobject]@{
        Name = $gate.Name
        Kind = $gate.Kind
        Command = $gate.Command
        Status = $status
        Duration = $duration
        LogFile = $logPath
        Failure = $failure
    })

    if ($status -eq "FAIL") {
        break
    }
}

$summary = New-Object System.Collections.Generic.List[string]
$summary.Add("# TAS Fast Lane Summary")
$summary.Add("")
$summary.Add(("- Generated: {0}" -f (Get-Date).ToString("s")))
$summary.Add(("- Hard gates: **{0}**" -f $hardGateStatus))
$summary.Add(("- Diagnostics: **{0}**" -f $diagnosticStatus))
$summary.Add(('- TAS_TEST_OUTPUT: `{0}`' -f $tasOutputDir))
$summary.Add("")
$summary.Add("| Gate | Kind | Command | Status | Duration | Log |")
$summary.Add("|---|---|---|---|---|---|")

foreach ($result in $results) {
    $relativeLog = $result.LogFile.Replace($artifactRoot, "").TrimStart('\')
    $summary.Add(('| `{0}` | `{1}` | `{2}` | **{3}** | `{4}` | `{5}` |' -f $result.Name, $result.Kind, $result.Command, $result.Status, $result.Duration, $relativeLog))
}

$artifactCandidates = @(
    "mock_results.csv",
    "mock_certificate.json",
    "regression_results.csv",
    "regression_certificate.json",
    "acceptance_certificate.json"
)
$presentArtifacts = @()
foreach ($artifact in $artifactCandidates) {
    $artifactPath = Join-Path $tasOutputDir $artifact
    if (Test-Path $artifactPath) {
        $presentArtifacts += $artifact
    }
}

$summary.Add("")
$summary.Add("## Captured Artifacts")
if ($presentArtifacts.Count -eq 0) {
    $summary.Add('- No certificate/csv artifacts detected in `tas_test_output`.')
}
else {
    foreach ($artifact in $presentArtifacts) {
        $summary.Add(('- `{0}`' -f $artifact))
    }
}

if ($hardFailureReason) {
    $summary.Add("")
    $summary.Add("## Hard Gate Failure")
    $summary.Add(('- `{0}`' -f $hardFailureReason))
}

if ($diagnosticFailures.Count -gt 0) {
    $summary.Add("")
    $summary.Add("## Diagnostic Warnings")
    foreach ($diagnosticFailure in $diagnosticFailures) {
        $summary.Add(('- `{0}`' -f $diagnosticFailure))
    }
}

$summary | Set-Content -Path $summaryPath
Write-Host ""
Write-Host ("Summary written: {0}" -f $summaryPath)

if ($hardGateStatus -ne "PASS") {
    exit 1
}
