# Sample the keyboard-object action-state bytes via the known pointer chain:
# root = [SG+0x1D5450], kbobj = [root+0x530], actions at kb+0x440/0x441/0x455..0x458.
# Prints N samples so you can see whether something (demo replay / player) is
# driving the action bytes while the user touches nothing.
param([int]$Samples = 25, [int]$IntervalMs = 120)
Add-Type @"
using System; using System.Runtime.InteropServices;
public class RPM2 {
  [DllImport("kernel32.dll")] public static extern IntPtr OpenProcess(uint a, bool i, int p);
  [DllImport("kernel32.dll")] public static extern bool ReadProcessMemory(IntPtr h, IntPtr a, byte[] b, int n, out IntPtr r);
}
"@
$p = Get-Process Supreme -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $p) { Write-Error "NO GAME"; exit 1 }
$sg = ($p.Modules | Where-Object { $_.ModuleName -ieq "Supreme_Game.dll" } | Select-Object -First 1).BaseAddress
$h = [RPM2]::OpenProcess(0x10, $false, $p.Id)
function ReadU32($addr) {
    $b = New-Object byte[] 4; $r = [IntPtr]::Zero
    if ([RPM2]::ReadProcessMemory($h, [IntPtr][int64]$addr, $b, 4, [ref]$r)) { return [BitConverter]::ToUInt32($b, 0) }
    return 0
}
function ReadBytes($addr, $n) {
    $b = New-Object byte[] $n; $r = [IntPtr]::Zero
    [RPM2]::ReadProcessMemory($h, [IntPtr][int64]$addr, $b, $n, [ref]$r) | Out-Null
    return $b
}
for ($i = 0; $i -lt $Samples; $i++) {
    $root = ReadU32 ([int64]$sg + 0x1D5450)
    if ($root -eq 0) { "root=NULL"; Start-Sleep -Milliseconds $IntervalMs; continue }
    $kb = ReadU32 ([int64]$root + 0x530)
    if ($kb -eq 0) { "kb=NULL"; Start-Sleep -Milliseconds $IntervalMs; continue }
    $a = ReadBytes ([int64]$kb + 0x440) 2     # SHIFT, JUMP
    $d = ReadBytes ([int64]$kb + 0x455) 4     # LEFT, UP, RIGHT, DOWN
    "{0,2}: shift={1} jump={2} L={3} U={4} R={5} D={6}" -f $i,$a[0],$a[1],$d[0],$d[1],$d[2],$d[3]
    Start-Sleep -Milliseconds $IntervalMs
}
