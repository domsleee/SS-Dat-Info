set shell := ["pwsh.exe", "-NoProfile", "-c"]
set unstable

supreme_folder := 'T:\Games\SupremeORIG'
dc_profile := 'debug'

[parallel]
all: display_config tas

display_config:
    cd Display_Config && just --set dc_profile {{dc_profile}} all

[parallel]
tas: tas_dll tas_rust

tas_dll:
    $vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath; \
    & "$vsPath\MSBuild\Current\Bin\MSBuild.exe" .\TAS\TAS_Helper\TAS_Helper.vcxproj /v:minimal /p:Configuration=Release /m

tas_rust:
    cd TAS && cargo build --release

[private]
[no-exit-message]
stop_game:
    $ErrorActionPreference = 'Stop'; \
    $running = @( \
        'Supreme','Supreme_v1.035','Display_Config','display-config','tas_ui','tas_test' | \
        ForEach-Object { Get-Process -Name $_ -ErrorAction SilentlyContinue } | \
        Where-Object { $_.Name -ne 'supreme-service' } | \
        Sort-Object Id -Unique \
    ); \
    if ($running.Count -gt 0) { \
        $names = ($running | ForEach-Object { "$($_.ProcessName) [$($_.Id)]" }) -join ', '; \
        $running | Stop-Process -Force; \
        Start-Sleep -Milliseconds 500; \
        Write-Host "Stopped running processes for deploy: $names"; \
    }; \
    exit 0

deploy: tas stop_game
    $dest = '{{supreme_folder}}\Display_Config_Resources\TAS'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\TAS\TAS_Helper\Release\TAS_Helper.dll $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_ui.exe $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_test.exe $dest\ -Force; \
    Write-Host "Deployed TAS to $dest"

deploy_display_config: display_config stop_game
    Copy-Item .\Display_Config\output\Display_Config.exe '{{supreme_folder}}\' -Force; \
    $dest = '{{supreme_folder}}\Display_Config_Resources'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\Display_Config\output\Display_Config_Resources\* $dest\ -Force; \
    Write-Host "Deployed Display_Config to {{supreme_folder}}"

deploy_all: deploy deploy_display_config

deploy_all_fast: deploy
    cd Display_Config && just --set dc_profile debug display_config display_config_helper; \
    Copy-Item .\Display_Config\output\Display_Config.exe '{{supreme_folder}}\' -Force; \
    $dest = '{{supreme_folder}}\Display_Config_Resources'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\Display_Config\output\Display_Config_Resources\* $dest\ -Force; \
    Write-Host "Deployed Display_Config (debug) to {{supreme_folder}}"

test: tas_rust
    cd TAS && cargo test --release

test_smoke: tas_rust
    cd TAS && cargo run --release --bin tas_test -- smoke

test_acceptance: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance

test_regression: tas_rust
    cd TAS && cargo run --release --bin tas_test -- regression

test_replay file iterations="5":
    cd TAS && cargo run --release --bin tas_test -- replay {{file}} --iterations {{iterations}} --verbose

test_live: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance && \
    cargo run --release --bin tas_test -- regression

clean:
    $ErrorActionPreference = 'Stop'; \
    cd Display_Config && just clean; \
    if (Test-Path .\TAS\Release) { Remove-Item .\TAS\Release -Recurse -Force }; \
    if (Test-Path .\TAS\target) { Remove-Item .\TAS\target -Recurse -Force }
