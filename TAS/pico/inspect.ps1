param([string]$Serial=$env:TAS_PICO_SERIAL, [switch]$Verify)
$ErrorActionPreference='Stop'
. "$PSScriptRoot/device.ps1"
try {
    $board = Find-PicoDevice -Serial $Serial
    if ($env:TAS_PICO_PORT -and $env:TAS_PICO_PORT -ne $board.data_port) { throw 'TAS_PICO_PORT does not belong to the selected Pico' }
    if ($Verify) { Test-PicoFiles $board $PSScriptRoot; Test-PicoAck $board }
    $board | ConvertTo-Json -Compress
} catch { [Console]::Error.WriteLine($_); exit 1 }
