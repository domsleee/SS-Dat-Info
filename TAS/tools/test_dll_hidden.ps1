# Compile and run the C++ unit suites (TAS/TAS_Helper/src/tests/test_*.cpp) with
# every child window hidden. A console taking the foreground pauses the windowed
# game, so nothing here may open one.
$ErrorActionPreference = 'Stop'

function Invoke-Hidden {
    param([string]$File, [string[]]$ArgList)
    $out = [System.IO.Path]::GetTempFileName()
    $err = [System.IO.Path]::GetTempFileName()
    $params = @{
        FilePath               = $File
        WindowStyle            = 'Hidden'
        PassThru               = $true
        RedirectStandardOutput = $out
        RedirectStandardError  = $err
    }
    if ($ArgList -and $ArgList.Count -gt 0) { $params.ArgumentList = $ArgList }
    $p = Start-Process @params
    # Wait for the compiler/test command, not unrelated descendants such as
    # Visual Studio's vctip telemetry process, which can stay alive indefinitely.
    $p.WaitForExit()
    # Write-Host, NOT the pipeline: everything a function emits becomes its
    # return value in PowerShell, and the caller compares that value to 0 -
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

$suites = Get-ChildItem -Path (Join-Path $PSScriptRoot '..\TAS_Helper\src\tests') -Filter 'test_*.cpp' |
    Sort-Object Name
if (-not $suites) { throw 'no test_*.cpp suites found' }

# A private directory per run: fixed exe/obj names in %TEMP% collide between
# concurrent runs, and a stale locked exe fails the next compile.
$work = Join-Path $env:TEMP ('tas_test_dll_' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $work | Out-Null
try {
    $i = 0
    foreach ($src in $suites) {
        $name = $src.BaseName -replace '^test_', ''
        # Numbered, not named: a 32-bit exe with "setup" in its name trips
        # Windows installer detection and refuses to start without elevation.
        $exe = Join-Path $work ('suite{0:D2}.exe' -f $i++)
        # A batch file instead of `cmd /c "<quoted> && <quoted>"`: Start-Process
        # re-quotes the joined argument and cmd's quote-stripping rules then
        # mangle the inner quotes. A single batch-file path has no nesting.
        $batch = Join-Path $work "build_$name.cmd"
        @(
            # VsDevCmd runs a bare `vswhere.exe` from a pushd'd directory; with
            # NoDefaultCurrentDirectoryInExePath set in the environment cmd
            # refuses that lookup and prints "'vswhere.exe' is not recognized".
            '@set NoDefaultCurrentDirectoryInExePath='
            # -arch=x86: the DLL is Win32-only, so the suites must see the same
            # pointer width and size_t as the shipped build.
            "@call `"$vsPath\Common7\Tools\VsDevCmd.bat`" -arch=x86 -no_logo"
            # Two backslashes: cl's argv parsing reads \" as a literal quote.
            "@cl /nologo /EHsc /std:c++17 /Fe:`"$exe`" /Fo:`"$work\\`" `"$($src.FullName)`""
            "@exit /b %errorlevel%"
        ) | Set-Content -Path $batch -Encoding ascii
        if ((Invoke-Hidden 'cmd.exe' @('/c', $batch)) -ne 0) { throw "compile failed ($name)" }
        if ((Invoke-Hidden $exe @()) -ne 0) { throw "$name tests failed" }
    }
} finally {
    Remove-Item $work -Recurse -Force -ErrorAction SilentlyContinue
}
Write-Host "test_dll: all $($suites.Count) C++ suites PASS (x86, all children ran hidden)"
