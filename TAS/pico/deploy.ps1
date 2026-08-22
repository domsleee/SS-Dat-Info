# deploy.ps1 - flash this repo's Pico firmware onto the board.
#
# The point of this script is that the REPO is the source of truth, not the
# device. Drag-and-drop editing is how the firmware ended up existing only on a
# flash drive and in one uncommitted file, with the committed version missing
# Escape entirely for months.
#
#   .\TAS\pico\deploy.ps1              # find CIRCUITPY, show diff, ask, copy
#   .\TAS\pico\deploy.ps1 -Check       # compare only, change nothing
#   .\TAS\pico\deploy.ps1 -Force       # skip the confirmation
#
# Run -Check before a test session. If it reports a difference, something edited
# the board behind the repo's back and the next test run is not running what you
# think it is.

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

# code.py and boot.py only. test.py is a HOST-side tool (it drives the board over
# serial from the PC) and must NOT be copied to the device.
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
    # Byte compare rather than Get-FileHash: that cmdlet needs PowerShell 4.0+,
    # and this must run under whatever `powershell` happens to be on PATH.
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
    Write-Host "Diff it before assuming the repo is the newer side - the board has"
    Write-Host "historically carried fixes that were never committed."
    exit 2
}

if (-not $Force) {
    Write-Host ""
    Write-Host "About to overwrite on ${dest}: $($differs -join ', ')" -ForegroundColor Yellow
    Write-Host "If the DEVICE has changes the repo does not, they are lost. Diff first."
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
