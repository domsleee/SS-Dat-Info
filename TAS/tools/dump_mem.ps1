param(
    [string]$Offset = "0x88000",   # offset from module base
    [int]$Length = 0x1800,
    [string]$Out = "dump.bin",
    [string]$Module = ""           # "" = Supreme.exe main module
)
Add-Type @"
using System; using System.Runtime.InteropServices;
public class RPM {
  [DllImport("kernel32.dll")] public static extern IntPtr OpenProcess(uint a, bool i, int p);
  [DllImport("kernel32.dll")] public static extern bool ReadProcessMemory(IntPtr h, IntPtr a, byte[] b, int n, out IntPtr r);
  [DllImport("kernel32.dll")] public static extern bool CloseHandle(IntPtr h);
}
"@
$p = Get-Process Supreme -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $p) { Write-Error "NO GAME"; exit 1 }
if ($Module -eq "") {
    $base = $p.MainModule.BaseAddress
} else {
    $m = $p.Modules | Where-Object { $_.ModuleName -ieq $Module } | Select-Object -First 1
    if (-not $m) { Write-Error "module $Module not found"; exit 1 }
    $base = $m.BaseAddress
}
$addr = [IntPtr]([int64]$base + [Convert]::ToInt64($Offset, 16))
$h = [RPM]::OpenProcess(0x10, $false, $p.Id)  # PROCESS_VM_READ
$buf = New-Object byte[] $Length
$page = New-Object byte[] 0x1000
$read = [IntPtr]::Zero
$ok = 0
for ($o = 0; $o -lt $Length; $o += 0x1000) {
    $n = [Math]::Min(0x1000, $Length - $o)
    $pa = [IntPtr]([int64]$addr + $o)
    if ([RPM]::ReadProcessMemory($h, $pa, $page, $n, [ref]$read)) {
        [Array]::Copy($page, 0, $buf, $o, $n)
        $ok += $n
    }  # unmapped pages stay zero-filled
}
[RPM]::CloseHandle($h) | Out-Null
[IO.File]::WriteAllBytes($Out, $buf)
"dumped $ok/$Length readable bytes from exe+$Offset to $Out"
