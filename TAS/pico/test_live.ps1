param([string]$Serial=$env:TAS_PICO_SERIAL)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/device.ps1"
if (Get-Process tas_ui,tas_test,Supreme -ErrorAction SilentlyContinue) { throw 'Close game and TAS controllers before the hardware test' }
$board = Find-PicoDevice -Serial $Serial
Test-PicoFiles $board $PSScriptRoot
Add-Type @'
using System.Runtime.InteropServices;
public class PicoKeyProbe {
    [DllImport("user32.dll")] public static extern short GetAsyncKeyState(int key);
}
'@
function Left-Down { return ([PicoKeyProbe]::GetAsyncKeyState(0x25) -band 0x8000) -ne 0 }
$port = Open-PicoPort $board.data_port
try {
    Read-PicoAck $port
    $port.Write([byte[]]@(1),0,1)
    Start-Sleep -Milliseconds 100
    if (!(Left-Down)) { throw 'LEFT did not reach Windows' }
    Write-Output 'PASS: physical HID LEFT reaches Windows'
    Start-Sleep -Milliseconds 550
    if (Left-Down) { throw 'LEFT remained held after safety timeout' }
    Write-Output 'PASS: safety timeout releases LEFT'
    $port.Write([byte[]]@(1),0,1)
    Start-Sleep -Milliseconds 80
    if (!(Left-Down)) { throw 'Second LEFT did not arrive' }
    Read-PicoAck $port
    Start-Sleep -Milliseconds 80
    if (Left-Down) { throw 'Explicit release failed' }
    Write-Output 'PASS: explicit release and acknowledgement'
    for ($i=0; $i -lt 100; $i++) { Read-PicoAck $port }
    Write-Output 'PASS: 100 consecutive health acknowledgements'
    $port.Write([byte[]]@(253),0,1)
} finally {
    try { $port.Write([byte[]]@(255),0,1) } catch { }
    Close-PicoPort $port
}
Start-Sleep -Seconds 3
$board = Find-PicoDevice -Serial $board.serial
$port = Open-PicoPort $board.data_port
try { Read-PicoAck $port; Write-Output 'PASS: software reset, reopen and fresh acknowledgement' }
finally { Close-PicoPort $port }
