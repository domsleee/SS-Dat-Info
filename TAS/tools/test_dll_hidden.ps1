# Compile + run the two C++ unit suites with EVERY child window HIDDEN.
#
# Why: this runs from the pre-commit hook (via `just test_dll`). Launched from
# a context without an inherited console (GUI git clients, background agents),
# `cmd /c` and the console test exes each allocate a NEW console window - and a
# console taking foreground PAUSES the windowed game. That froze an in-game
# suite mid-replay when a commit ran during a test battery (2026-08-24).
# Start-Process -WindowStyle Hidden allocates the console SW_HIDE: no window,
# no foreground steal, in every launch context. Output is captured to temp
# files and replayed so the hook's log looks exactly as before.
$ErrorActionPreference = 'Stop'

function Invoke-Hidden {
    param([string]$File, [string[]]$ArgList)
    $out = [System.IO.Path]::GetTempFileName()
    $err = [System.IO.Path]::GetTempFileName()
    $params = @{
        FilePath               = $File
        WindowStyle            = 'Hidden'
        Wait                   = $true
        PassThru               = $true
        RedirectStandardOutput = $out
        RedirectStandardError  = $err
    }
    if ($ArgList -and $ArgList.Count -gt 0) { $params.ArgumentList = $ArgList }
    $p = Start-Process @params
    # Write-Host, NOT the pipeline: everything a function emits becomes its
    # return value in PowerShell, and the caller compares that value to 0 —
    # log lines in the pipeline would turn every success into a false failure.
    Get-Content $out | ForEach-Object { Write-Host $_ }
    Get-Content $err | ForEach-Object { Write-Host $_ }
    Remove-Item $out, $err -Force -ErrorAction SilentlyContinue
    return $p.ExitCode
}

$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vsOut = [System.IO.Path]::GetTempFileName()
$p = Start-Process -FilePath $vsWhere -ArgumentList '-latest', '-property', 'installationPath' `
    -WindowStyle Hidden -Wait -PassThru -RedirectStandardOutput $vsOut
if ($p.ExitCode -ne 0) { throw 'vswhere failed' }
$vsPath = (Get-Content $vsOut | Select-Object -First 1).Trim()
Remove-Item $vsOut -Force -ErrorAction SilentlyContinue
if (-not $vsPath) { throw 'vswhere returned no installation path' }

$suites = @(
    @{ Src = '.\TAS\TAS_Helper\src\tests\test_input_gate.cpp'; Exe = 'tas_test_input_gate.exe'; Name = 'input_gate' },
    @{ Src = '.\TAS\TAS_Helper\src\tests\test_level_path.cpp'; Exe = 'tas_test_level_path.exe'; Name = 'level_path' },
    @{ Src = '.\TAS\TAS_Helper\src\tests\test_replay_capture.cpp'; Exe = 'tas_test_replay_capture.exe'; Name = 'replay_capture' },
    @{ Src = '.\TAS\TAS_Helper\src\tests\test_race_timer.cpp'; Exe = 'tas_test_race_timer.exe'; Name = 'race_timer' }
)
foreach ($t in $suites) {
    $exe = Join-Path $env:TEMP $t.Exe
    # A temp batch file instead of `cmd /c "<quoted> && <quoted>"`: Start-
    # Process re-quotes the joined argument and cmd's quote-stripping rules
    # then mangle the inner quotes ('C:\Program' is not recognized...). A
    # single batch-file path has no nesting to mangle.
    $batch = Join-Path $env:TEMP "tas_build_$($t.Name).cmd"
    @(
        "@call `"$vsPath\Common7\Tools\VsDevCmd.bat`" -arch=x64 -no_logo"
        "@cl /nologo /EHsc /std:c++17 /Fe:`"$exe`" /Fo:`"$env:TEMP\\`" $($t.Src)"
        "@exit /b %errorlevel%"
    ) | Set-Content -Path $batch -Encoding ascii
    $rc = Invoke-Hidden 'cmd.exe' @('/c', $batch)
    Remove-Item $batch -Force -ErrorAction SilentlyContinue
    if ($rc -ne 0) { throw "compile failed ($($t.Name))" }
    if ((Invoke-Hidden $exe @()) -ne 0) { throw "$($t.Name) tests failed" }
}
Write-Host 'test_dll: all C++ suites PASS (all children ran hidden)'
