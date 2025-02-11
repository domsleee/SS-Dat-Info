param(
    [Parameter(Mandatory=$true)]
    [string]$version
)
$packageJsonDir = Join-Path -Resolve $PSScriptRoot ..\Display_Config
$cargoTomlFile = Join-Path -Resolve $PSScriptRoot "..\Display_Config\src-tauri\Cargo.toml"

Write-Output "`nUpdating  $(Join-Path $packageJsonDir "package.json")..."
cd $packageJsonDir
bunx tauri-version $version --no-git --no-lock
cd ..
