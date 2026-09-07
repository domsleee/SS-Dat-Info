# deploy.ps1 - flash this repo's Pico firmware (code.py, boot.py) onto the board.
#
#   .\TAS\pico\deploy.ps1              # find CIRCUITPY, show diff, ask, copy
#   .\TAS\pico\deploy.ps1 -Check       # compare only, change nothing
#   .\TAS\pico\deploy.ps1 -Force       # skip the confirmation
#
# Run -Check before a test session: a difference means the board is not running
# the committed firmware.

param(
    [switch]$Check,
    [switch]$Force,
    [string]$Drive
)

$ErrorActionPreference = 'Stop'
$src = Split-Path -Parent $MyInvocation.MyCommand.Path

function Find-Circuitpy {
    if ($Drive) { return $Drive }
    $vol = Get-Volume | Where-Object { $_.FileSystemLabel -eq 'CIRCUITPY' } | Select-Object -First 1
    if (-not $vol -or -not $vol.DriveLetter) { return $null }
    return "$($vol.DriveLetter):"
}

$dest = Find-Circuitpy
if (-not $dest) {
    Write-Host "CIRCUITPY volume not found." -ForegroundColor Red
    Write-Host "The board may be unplugged, or boot.py may have disabled the USB drive."
    Write-Host "Recovery: CircuitPython safe mode bypasses boot.py; BOOTSEL reflashes."
    exit 1
}
Write-Host "CIRCUITPY: $dest"

# test.py runs on the PC and is not copied.
$files = @('code.py', 'boot.py')
$differs = @()

foreach ($f in $files) {
    $a = Join-Path $src $f
    $b = Join-Path $dest $f
    if (-not (Test-Path $b)) {
        Write-Host "  $f : MISSING on device" -ForegroundColor Yellow
        $differs += $f
        continue
    }
    # Byte compare: Get-FileHash needs PowerShell 4.0+ and this runs under
    # whatever `powershell` is on PATH.
    $ba = [System.IO.File]::ReadAllBytes($a)
    $bb = [System.IO.File]::ReadAllBytes($b)
    $same = $ba.Length -eq $bb.Length
    if ($same) {
        for ($i = 0; $i -lt $ba.Length; $i++) {
            if ($ba[$i] -ne $bb[$i]) { $same = $false; break }
        }
    }
    if ($same) {
        Write-Host "  $f : identical" -ForegroundColor Green
    } else {
        Write-Host "  $f : DIFFERS" -ForegroundColor Yellow
        $differs += $f
    }
}

if ($differs.Count -eq 0) {
    Write-Host "Device matches the repo." -ForegroundColor Green
    exit 0
}

if ($Check) {
    Write-Host ""
    Write-Host "Device does NOT match the repo: $($differs -join ', ')" -ForegroundColor Red
    Write-Host "Run without -Check to copy the repo's files to the board."
    exit 2
}

if (-not $Force) {
    Write-Host ""
    Write-Host "About to overwrite on ${dest}: $($differs -join ', ')" -ForegroundColor Yellow
    $ans = Read-Host "Type 'yes' to copy"
    if ($ans -ne 'yes') { Write-Host "Aborted."; exit 1 }
}

foreach ($f in $differs) {
    Copy-Item (Join-Path $src $f) (Join-Path $dest $f) -Force
    Write-Host "  copied $f" -ForegroundColor Green
}

Write-Host ""
Write-Host "Done. CircuitPython restarts code.py on write, so the board re-runs"
Write-Host "immediately; the CDC port may drop briefly while it does."
