set shell := ["pwsh.exe", "-NoProfile", "-c"]
set unstable

# Default game folder — override with: just --set supreme_folder 'C:\path\to\game'
supreme_folder := 'T:\Games\SupremeORIG'

# Display_Config build profile — 'debug' (fast, default) or 'release' (slow, optimised)
# Display_Config is a launcher UI, debug is fast enough. Override with: just --set dc_profile release deploy_all
dc_profile := 'debug'

# ── Build Everything ──────────────────────────────────────────────

# Build all components (Display_Config + TAS) — in parallel
[parallel]
all: display_config tas

# ── Display Config ────────────────────────────────────────────────

# Build Display_Config (Tauri app + helper DLL)
display_config:
    cd Display_Config && just --set dc_profile {{dc_profile}} all

# ── TAS ───────────────────────────────────────────────────────────

# Build all TAS components (C++ DLL + Rust UI/test) — in parallel
[parallel]
tas: tas_dll tas_rust

# Build TAS_Helper.dll + Injector.exe (C++ / MSBuild)
tas_dll:
    $vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath; \
    Import-Module "$vsPath\Common7\Tools\Microsoft.VisualStudio.DevShell.dll"; \
    Enter-VsDevShell -VsInstallPath $vsPath -SkipAutomaticLocation; \
    msbuild .\TAS\TAS_Helper\TAS_Helper.vcxproj /v:minimal /p:Configuration=Release /m; \
    msbuild .\TAS\Injector\Injector.vcxproj /v:minimal /p:Configuration=Release /m

# Build TAS Rust workspace (tas_ui + tas_test + tas_shared)
tas_rust:
    cd TAS && cargo build --release

# ── Deploy ────────────────────────────────────────────────────────

# Deploy TAS to the game folder (creates TAS_Helper/ next to Supreme.exe)
deploy: tas
    $ErrorActionPreference = 'Stop'; \
    $running = @(Get-Process -Name Supreme,Supreme_v1.035,Display_Config,'display-config',tas_ui,tas_test -ErrorAction SilentlyContinue | Sort-Object Id -Unique); \
    if ($running.Count -gt 0) { \
        $names = ($running | ForEach-Object { "$($_.ProcessName) [$($_.Id)]" }) -join ', '; \
        $running | Stop-Process -Force; \
        Start-Sleep -Milliseconds 500; \
        Write-Host "Stopped running processes for deploy: $names"; \
    }; \
    $dest = '{{supreme_folder}}\TAS_Helper'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\TAS\TAS_Helper\Release\TAS_Helper.dll $dest\ -Force; \
    Copy-Item .\TAS\Injector\Release\Injector.exe $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_ui.exe $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_test.exe $dest\ -Force; \
    Write-Host "Deployed TAS to $dest"

# Deploy Display_Config to the game folder
deploy_display_config: display_config
    $ErrorActionPreference = 'Stop'; \
    $running = @(Get-Process -Name Supreme,Supreme_v1.035,Display_Config,'display-config',tas_ui,tas_test -ErrorAction SilentlyContinue | Sort-Object Id -Unique); \
    if ($running.Count -gt 0) { \
        $names = ($running | ForEach-Object { "$($_.ProcessName) [$($_.Id)]" }) -join ', '; \
        $running | Stop-Process -Force; \
        Start-Sleep -Milliseconds 500; \
        Write-Host "Stopped running processes for deploy: $names"; \
    }; \
    Copy-Item .\Display_Config\output\Display_Config.exe '{{supreme_folder}}\' -Force; \
    $dest = '{{supreme_folder}}\Display_Config_Resources'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\Display_Config\output\Display_Config_Resources\* $dest\ -Force; \
    Write-Host "Deployed Display_Config to {{supreme_folder}}"

# Deploy everything
deploy_all: deploy deploy_display_config

# ── Test ──────────────────────────────────────────────────────────

# Run Rust unit tests (no game or hardware required)
test: tas_rust
    cd TAS && cargo test --release

# Run TAS mock tests (requires game + DLL injected, no Pico HID)
test_mock: tas_rust
    cd TAS && cargo run --release --bin tas_test -- mock

# Run TAS smoke test (no hardware required)
test_smoke: tas_rust
    cd TAS && cargo run --release --bin tas_test -- smoke

# Run full TAS acceptance test (requires Pico HID + running game)
test_acceptance: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance

# Run TAS regression suite (requires Pico HID + running game)
test_regression: tas_rust
    cd TAS && cargo run --release --bin tas_test -- regression

# Run TAS fast lane gates (hard: unit + speed-reset; diagnostic: mock + FE-decent replay)
test_fast_lane artifacts_dir='TAS/artifacts/fast-lane/latest':
    pwsh -NoProfile -File .\scripts\run-tas-fast-lane.ps1 -ArtifactsDir '{{artifacts_dir}}'

# Replay a .tasrec file N times checking for drift (requires running game)
test_replay file iterations="5":
    cd TAS && cargo run --release --bin tas_test -- replay {{file}} --iterations {{iterations}} --verbose

# Run all live hardware E2E tests (acceptance + regression, requires Pico HID + running game)
test_live: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance && \
    cargo run --release --bin tas_test -- regression

# ── Clean ─────────────────────────────────────────────────────────

# Clean all build artifacts
clean:
    $ErrorActionPreference = 'Stop'; \
    cd Display_Config && just clean; \
    if (Test-Path .\TAS\Release) { Remove-Item .\TAS\Release -Recurse -Force }; \
    if (Test-Path .\TAS\target) { Remove-Item .\TAS\target -Recurse -Force }
