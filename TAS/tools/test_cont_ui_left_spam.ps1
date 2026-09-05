# Live regression: deployed tas_ui handles F12 while Pico HID repeatedly taps
# LEFT. Keep the UI running with a loaded recording and From set to Splice.
# A passing run needs every CONT to resume on its first attempt without drift.
param(
    [ValidateRange(1,100)][int]$Iterations = 5,
    [ValidateRange(1,65535)][int]$Splice = 2200,
    [ValidateRange(20,250)][int]$KeyIntervalMs = 40,
    [ValidateRange(1,30)][int]$SpamSeconds = 3,
    [string]$PicoPort = 'COM7',
    [string]$LogPath = 'T:\Games\SupremeORIG\Display_Config_Resources\TAS\data\tas_ui.log'
)
$ErrorActionPreference = 'Stop'
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class ContSpamInput {
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern short GetAsyncKeyState(int vk);
    [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
}
'@
function Press-Key([byte]$Key) {
    [ContSpamInput]::keybd_event($Key, 0, 0, [UIntPtr]::Zero)
    try { Start-Sleep -Milliseconds 100 }
    finally { [ContSpamInput]::keybd_event($Key, 0, 2, [UIntPtr]::Zero) }
}
function Read-NewLog([long]$Offset) {
    $stream = [System.IO.File]::Open($LogPath, 'Open', 'Read', 'ReadWrite')
    $stream.Seek($Offset, 'Begin') | Out-Null
    $reader = [System.IO.StreamReader]::new($stream)
    try { $reader.ReadToEnd() } finally { $reader.Dispose() }
}
$game = Get-Process -Name Supreme | Select-Object -First 1
$ui = Get-Process -Name tas_ui | Select-Object -First 1
Write-Output "Production processes: Supreme PID $($game.Id), tas_ui PID $($ui.Id)"
[ContSpamInput]::SetForegroundWindow($game.MainWindowHandle) | Out-Null
Start-Sleep -Milliseconds 300
if ([ContSpamInput]::GetForegroundWindow() -ne $game.MainWindowHandle) {
    throw 'Supreme did not take focus; no keys sent'
}
$port = $null
$failures = 0
$offset = (Get-Item -LiteralPath $LogPath).Length
try {
    $port = [System.IO.File]::Open("\\.\$PicoPort", 'Open', 'Write')
    for ($trial = 1; $trial -le $Iterations; $trial++) {
        $trialOffset = (Get-Item -LiteralPath $LogPath).Length
        $port.WriteByte(255); $port.Flush()
        Write-Output "Trial ${trial}: F12, then LEFT down/up every ${KeyIntervalMs}ms"
        Press-Key 123
        $deadline = [DateTime]::UtcNow.AddSeconds($SpamSeconds)
        $presses = 0
        while ([DateTime]::UtcNow -lt $deadline) {
            if (-not (Get-Process -Id $game.Id -ErrorAction SilentlyContinue)) {
                throw "Supreme exited during trial $trial"
            }
            if ([ContSpamInput]::GetForegroundWindow() -ne $game.MainWindowHandle) {
                throw 'Game lost focus during spam; trial is inconclusive'
            }
            $port.WriteByte(1); $port.Flush()
            Start-Sleep -Milliseconds $KeyIntervalMs
            if ([ContSpamInput]::GetAsyncKeyState(37) -ge 0) { throw 'LEFT press not observed by Windows' }
            $port.WriteByte(255); $port.Flush()
            Start-Sleep -Milliseconds $KeyIntervalMs
            if ([ContSpamInput]::GetAsyncKeyState(37) -lt 0) { throw 'LEFT release not observed by Windows' }
            $presses++
        }
        Press-Key 122
        Start-Sleep -Milliseconds 300
        $log = Read-NewLog $trialOffset
        $resumes = [regex]::Matches($log, "CONT resumed at frame $Splice after 1 bucket attempt ").Count
        $rerolls = [regex]::Matches($log, 'CONT bucket reroll').Count
        $bad = $log -match 'DRIFT|CONT aborted|CONT gave up|Game process not found'
        $passed = $resumes -eq 1 -and $rerolls -eq 0 -and -not $bad
        if (-not $passed) { $failures++ }
        Write-Output "  $(if($passed){'PASS'}else{'FAIL'}): $presses verified LEFT taps; first-attempt resumes=$resumes; rerolls=$rerolls"
        $log -split "`n" | Where-Object { $_ -match 'Global F12|CONT resumed|reroll|aborted|DRIFT|ignored|refused' }
    }
} finally {
    if ($port) { try { $port.WriteByte(255); $port.Flush() } finally { $port.Dispose() } }
    if ([ContSpamInput]::GetForegroundWindow() -eq $game.MainWindowHandle) { Press-Key 122 }
    Write-Output "Log byte offset: $offset (retained in $LogPath)"
}
if ($failures -gt 0) { throw "$failures / $Iterations production UI trials failed" }
Write-Output "PASS: $Iterations production UI trials, no retries or drift"
