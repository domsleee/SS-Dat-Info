set shell := ["pwsh.exe", "-NoProfile", "-c"]
set unstable

# The one place the game folder is configured. `just --set supreme_folder D:\Games\Supreme deploy`
# overrides it; the live harness reads the exported SUPREME_FOLDER.
supreme_folder := 'T:\Games\SupremeORIG'
export SUPREME_FOLDER := supreme_folder
dc_profile := 'release'

[parallel]
all: display_config tas

display_config:
    cd Display_Config && just --set dc_profile {{dc_profile}} all

# Display_Config_Helper.dll and Injector.exe; deploy ships the injector.
display_config_helper:
    cd Display_Config && just display_config_helper

[parallel]
tas: tas_dll tas_rust

tas_dll:
    $vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath; \
    & "$vsPath\MSBuild\Current\Bin\MSBuild.exe" .\TAS\TAS_Helper\TAS_Helper.vcxproj /v:minimal /p:Configuration=Release /p:Platform=Win32 /m

tas_rust:
    cd TAS && cargo build --release

# Compile and run the standalone C++ policy tests (no DLL, no game).
# The script runs them hidden: a console window taking focus pauses the game.
test_dll:
    & .\TAS\tools\test_dll_hidden.ps1

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
        Write-Host "Stopping for deploy: $names"; \
        $running | Stop-Process -Force; \
        Start-Sleep -Milliseconds 500; \
    }; \
    exit 0

# Stops the game and every TAS process before copying.
deploy: tas display_config_helper stop_game
    $dest = '{{supreme_folder}}\Display_Config_Resources\TAS'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\TAS\TAS_Helper\Release\TAS_Helper.dll $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_ui.exe $dest\ -Force; \
    Copy-Item .\TAS\target\release\tas_test.exe $dest\ -Force; \
    Copy-Item .\Display_Config\Display_Config_Resources\Injector\Release\Injector.exe '{{supreme_folder}}\Display_Config_Resources\' -Force; \
    Write-Host "Deployed TAS and Injector.exe to $dest"

# Launch the game directly, then a fresh tas_ui that connects to the new instance.
relaunch:
    Start-Process -FilePath '{{supreme_folder}}\Supreme.exe' -WorkingDirectory '{{supreme_folder}}'; \
    Start-Sleep -Seconds 4; \
    $dest = '{{supreme_folder}}\Display_Config_Resources\TAS'; \
    Start-Process -FilePath "$dest\tas_ui.exe" -WorkingDirectory $dest; \
    Write-Host "Relaunched game + tas_ui (navigate into the race manually)"

# One-shot: build, stop, deploy, relaunch — the only deploy flow to use.
deploy_run: deploy relaunch

[private]
stage_display_config profile:
    Copy-Item .\Display_Config\output\Display_Config.exe '{{supreme_folder}}\' -Force; \
    $dest = '{{supreme_folder}}\Display_Config_Resources'; \
    if (!(Test-Path $dest)) { New-Item -ItemType Directory -Path $dest | Out-Null }; \
    Copy-Item .\Display_Config\output\Display_Config_Resources\* $dest\ -Force; \
    Write-Host "Deployed Display_Config ({{profile}}) to {{supreme_folder}}"

deploy_display_config: display_config stop_game (stage_display_config dc_profile)

[private]
display_config_debug:
    cd Display_Config && just --set dc_profile debug display_config display_config_helper

deploy_all: deploy deploy_display_config

deploy_all_fast: deploy display_config_debug (stage_display_config "debug")

test: tas_rust
    cd TAS && cargo test --release

test_tools:
    python -m unittest discover -s TAS/tools -p test_tools.py -v

test_smoke: tas_rust
    cd TAS && cargo run --release --bin tas_test -- smoke

test_acceptance: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance

test_regression: tas_rust
    cd TAS && cargo run --release --bin tas_test -- regression

# Starts an isolated UI and loads the checked-in FE fixture automatically.
test_cont_ui_left_spam splice="4500" iterations="5": tas_rust
    cd TAS && cargo run --release --bin tas_test -- cont-ui-left-spam --splice {{splice}} --iterations {{iterations}}

test_replay file iterations="5":
    cd TAS && cargo run --release --bin tas_test -- replay {{file}} --iterations {{iterations}} --verbose

# UI runs first: later harness stages stop the competing UI and replace the run.
test_live splice="4500" iterations="5": tas_rust
    cd TAS && cargo run --release --bin tas_test -- live --splice {{splice}} --iterations {{iterations}}

clean:
    $ErrorActionPreference = 'Stop'; \
    cd Display_Config && just clean; \
    if (Test-Path .\TAS\Release) { Remove-Item .\TAS\Release -Recurse -Force }; \
    if (Test-Path .\TAS\target) { Remove-Item .\TAS\target -Recurse -Force }
