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
function Open-Pico {
    $port = [System.IO.Ports.SerialPort]::new($board.data_port,115200)
    $port.ReadTimeout=1000
    $port.WriteTimeout=1000
    $port.DtrEnable=$true
    $port.Open()
    return $port
}
function Ack($port) {
    $port.DiscardInBuffer()
    $port.Write([byte[]]@(255),0,1)
    $reply = [byte[]]::new(3)
    $count=0
    while ($count -lt 3) { $count += $port.Read($reply,$count,3-$count) }
    if ([BitConverter]::ToString($reply) -ne '5A-01-01') { throw 'Bad ACK' }
}
function Left-Down { return ([PicoKeyProbe]::GetAsyncKeyState(0x25) -band 0x8000) -ne 0 }
$port = Open-Pico
try {
    Ack $port
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
    Ack $port
    Start-Sleep -Milliseconds 80
    if (Left-Down) { throw 'Explicit release failed' }
    Write-Output 'PASS: explicit release and acknowledgement'
    for ($i=0; $i -lt 100; $i++) { Ack $port }
    Write-Output 'PASS: 100 consecutive health acknowledgements'
    $port.Write([byte[]]@(253),0,1)
} finally {
    try { $port.Write([byte[]]@(255),0,1) } catch { }
    Close-PicoPort $port
}
Start-Sleep -Seconds 3
$board = Find-PicoDevice -Serial $board.serial
$port = Open-Pico
try { Ack $port; Write-Output 'PASS: software reset, reopen and fresh acknowledgement' }
finally { Close-PicoPort $port }
