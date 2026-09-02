param([string]$Keys = "", [int]$DelayMs = 160)
Add-Type @"
using System; using System.Runtime.InteropServices;
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
function Scan($n){ switch($n){ "UP"{,@(0x48,$true)} "DOWN"{,@(0x50,$true)} "LEFT"{,@(0x4B,$true)} "RIGHT"{,@(0x4D,$true)} "ENTER"{,@(0x1C,$false)} "ESC"{,@(0x01,$false)} "F5"{,@(0x3F,$false)} "SPACE"{,@(0x39,$false)} "TAB"{,@(0x0F,$false)} "F9"{,@(0x43,$false)} "F10"{,@(0x44,$false)} "F11"{,@(0x57,$false)} "F12"{,@(0x58,$false)} default{$null} } }
$p=Get-Process Supreme -EA SilentlyContinue|Select-Object -First 1; if(-not $p){"NO GAME";exit 1}
$h=$p.MainWindowHandle
$fg=[K]::GetForegroundWindow();$ft=[K]::GetWindowThreadProcessId($fg,[IntPtr]::Zero);$mt=[K]::GetCurrentThreadId()
[K]::AttachThreadInput($mt,$ft,$true)|Out-Null;[K]::ShowWindow($h,9)|Out-Null;[K]::BringWindowToTop($h)|Out-Null;[K]::SetForegroundWindow($h)|Out-Null;[K]::AttachThreadInput($mt,$ft,$false)|Out-Null
Start-Sleep -Milliseconds 350
foreach($k in ($Keys -split ",")){ $n=$k.Trim().ToUpper(); if($n -eq ""){continue}; $sc=Scan $n; if($null -eq $sc){"?? $n";continue}; $e=if($sc[1]){0x1}else{0x0}; [K]::keybd_event(0,[byte]$sc[0],[uint32](0x8 -bor $e),[UIntPtr]::Zero); Start-Sleep -Milliseconds 15; [K]::keybd_event(0,[byte]$sc[0],[uint32](0x8 -bor $e -bor 0x2),[UIntPtr]::Zero); Start-Sleep -Milliseconds $DelayMs }
"sent"
