# Updates are maintenance operations: stop game/TAS controllers first.
param([switch]$Check, [switch]$Force, [switch]$Recover,
      [string]$Drive, [string]$Serial=$env:TAS_PICO_SERIAL)
$ErrorActionPreference='Stop'
. "$PSScriptRoot/device.ps1"

function Enter-PicoConsole($Board) {
    $port = Open-PicoPort $Board.console_port
    try {
        $ready=$false
        for ($attempt=0; $attempt -lt 8; $attempt++) {
            $port.Write([byte[]]@(3),0,1)
            Start-Sleep -Milliseconds 200
            $port.Write("`r`n")
            Start-Sleep -Milliseconds 200
            if ($port.ReadExisting() -match '>>> ') { $ready=$true; break }
        }
        if (!$ready) { throw 'Console did not reach the REPL prompt within its deadline' }
        $port.DiscardInBuffer()
        $port.Write("print('TAS_UPDATE_READY')`r`n")
        Start-Sleep -Milliseconds 200
        $response = $port.ReadExisting()
        if ($response -notmatch "(?m)^TAS_UPDATE_READY\r?$") { throw "No live REPL response: $response" }
        return $port
    } catch { Close-PicoPort $port; throw }
}
function Reset-PicoConsole($Board) {
    $port = Enter-PicoConsole $Board
    try { $port.Write("import microcontroller; microcontroller.reset()`r`n") }
    finally { Close-PicoPort $port }
    Start-Sleep -Seconds 3
}
function Wait-Pico($Identity) {
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    do {
        try { return Find-PicoDevice -Serial $Identity }
        catch { $lastFailure=$_; Start-Sleep -Milliseconds 300 }
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "Pico $Identity did not reappear: $lastFailure"
}
$console=$null
try {
    if ($Check -and $Recover) { throw '-Check cannot be combined with -Recover' }
    $board = Find-PicoDevice -Serial $Serial -Drive $Drive
    Write-Host "Pico $($board.serial): $($board.drive), data $($board.data_port), console $($board.console_port)"
    if ($Check) {
        Test-PicoFiles $board $PSScriptRoot
        Write-Host 'PASS: both files match (read-only check; no HID commands sent)'
        exit 0
    }
    if (Get-Process tas_ui,tas_test,Supreme -ErrorAction SilentlyContinue) { throw 'Close game and TAS controllers before updating firmware' }
    if (!$Force -and (Read-Host "Update Pico $($board.serial)? Type yes") -ne 'yes') { throw 'Cancelled' }
    $backup = Join-Path $PSScriptRoot "../artifacts/pico-updates/$($board.serial)/$([guid]::NewGuid())"
    New-Item -ItemType Directory -Path $backup | Out-Null
    foreach ($name in 'boot.py','code.py','boot_out.txt') {
        $path = Join-Path $board.drive $name
        if (Test-Path -LiteralPath $path) { Copy-Item -LiteralPath $path -Destination (Join-Path $backup $name) }
    }
    Write-Host "Backup: $backup"
    if ($Recover) {
        # Exact composite USB device only. Run this script with sudo for this option.
        & pnputil /restart-device $board.instance_id
        if ($LASTEXITCODE -ne 0) { throw 'USB restart failed; run sudo pwsh -File deploy.ps1 -Recover -Force' }
        $board = Wait-Pico $board.serial
    }
    # Full reset clears stale device-level storage protection. Pausing in REPL then
    # prevents auto-reload from executing partially copied code or mixed boot/code.
    Reset-PicoConsole $board
    $board = Wait-Pico $board.serial
    $console = Enter-PicoConsole $board
    foreach ($name in 'boot.py','code.py') {
        $board = Find-PicoDevice -Serial $board.serial
        $source = Join-Path $PSScriptRoot $name
        $target = Join-Path $board.drive $name
        $bytes = [System.IO.File]::ReadAllBytes($source)
        $staged = "$target.$([guid]::NewGuid()).tmp"
        $file = [System.IO.File]::Open($staged,[System.IO.FileMode]::CreateNew,[System.IO.FileAccess]::Write,[System.IO.FileShare]::None)
        try { $file.Write($bytes,0,$bytes.Length); $file.Flush($true) } finally { $file.Dispose() }
        if ((Get-FileHash -LiteralPath $source).Hash -ne (Get-FileHash -LiteralPath $staged).Hash) { throw "Staged $name hash mismatch" }
        Move-Item -LiteralPath $staged -Destination $target -Force
    }
    Test-PicoFiles $board $PSScriptRoot
    $console.Write("import microcontroller; microcontroller.reset()`r`n")
    Close-PicoPort $console; $console=$null
    Start-Sleep -Seconds 3
    $board = Wait-Pico $board.serial
    Test-PicoFiles $board $PSScriptRoot
    Test-PicoAck $board
    Write-Host "PASS: Pico $($board.serial) updated, rebooted, files verified and live release acknowledged"
} catch {
    [Console]::Error.WriteLine("Pico update FAILED: $_")
    [Console]::Error.WriteLine('No success is claimed. If copying began, restore the printed backup before restarting a partial update.')
    exit 1
} finally {
    if ($console) { Close-PicoPort $console }
}
