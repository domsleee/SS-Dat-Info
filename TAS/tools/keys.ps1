# Send keys to the running game window through keybd_event scan codes.
#   powershell -NoProfile -File TAS\tools\keys.ps1 -Keys "ESC,DOWN,ENTER" -DelayMs 160
param(
    [string]$Keys = "",
    [int]$DelayMs = 160
)

Add-Type @"
using System;
using System.Runtime.InteropServices;
public class K {
  [DllImport("user32.dll")] public static extern void keybd_event(byte v, byte s, uint f, UIntPtr e);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int n);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, IntPtr p);
  [DllImport("kernel32.dll")] public static extern uint GetCurrentThreadId();
  [DllImport("user32.dll")] public static extern bool AttachThreadInput(uint a, uint b, bool x);
}
"@

# Scan code and whether the key is extended (arrows need KEYEVENTF_EXTENDEDKEY).
function Scan($name) {
    switch ($name) {
        "UP"    { ,@(0x48, $true) }
        "DOWN"  { ,@(0x50, $true) }
        "LEFT"  { ,@(0x4B, $true) }
        "RIGHT" { ,@(0x4D, $true) }
        "ENTER" { ,@(0x1C, $false) }
        "ESC"   { ,@(0x01, $false) }
        "F5"    { ,@(0x3F, $false) }
        "SPACE" { ,@(0x39, $false) }
        "TAB"   { ,@(0x0F, $false) }
        "F9"    { ,@(0x43, $false) }
        "F10"   { ,@(0x44, $false) }
        "F11"   { ,@(0x57, $false) }
        "F12"   { ,@(0x58, $false) }
        default { $null }
    }
}

$game = Get-Process Supreme -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $game) {
    "NO GAME"
    exit 1
}
$window = $game.MainWindowHandle

# Attach to the foreground thread so SetForegroundWindow is allowed to take focus.
$foreground = [K]::GetForegroundWindow()
$foregroundThread = [K]::GetWindowThreadProcessId($foreground, [IntPtr]::Zero)
$thisThread = [K]::GetCurrentThreadId()
[K]::AttachThreadInput($thisThread, $foregroundThread, $true) | Out-Null
[K]::ShowWindow($window, 9) | Out-Null
[K]::BringWindowToTop($window) | Out-Null
[K]::SetForegroundWindow($window) | Out-Null
[K]::AttachThreadInput($thisThread, $foregroundThread, $false) | Out-Null
Start-Sleep -Milliseconds 350

$KEYEVENTF_EXTENDEDKEY = 0x1
$KEYEVENTF_KEYUP = 0x2
$KEYEVENTF_SCANCODE = 0x8

foreach ($key in ($Keys -split ",")) {
    $name = $key.Trim().ToUpper()
    if ($name -eq "") {
        continue
    }
    $scan = Scan $name
    if ($null -eq $scan) {
        "?? $name"
        continue
    }
    $extended = if ($scan[1]) { $KEYEVENTF_EXTENDEDKEY } else { 0x0 }
    [K]::keybd_event(0, [byte]$scan[0], [uint32]($KEYEVENTF_SCANCODE -bor $extended), [UIntPtr]::Zero)
    Start-Sleep -Milliseconds 15
    [K]::keybd_event(0, [byte]$scan[0], [uint32]($KEYEVENTF_SCANCODE -bor $extended -bor $KEYEVENTF_KEYUP), [UIntPtr]::Zero)
    Start-Sleep -Milliseconds $DelayMs
}
"sent"
