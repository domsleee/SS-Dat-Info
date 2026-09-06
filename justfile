set shell := ["pwsh.exe", "-NoProfile", "-c"]
set unstable

supreme_folder := 'T:\Games\SupremeORIG'
dc_profile := 'release'

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

# Compile + run the standalone C++ unit tests (pure gate-policy logic, no DLL).
# Uses the VS dev shell's cl.exe via vswhere + VsDevCmd.
test_dll:
    # All children run HIDDEN (see the script) — a console window popping here
    # steals foreground from the game, and a deactivated game PAUSES. That
    # froze an in-game suite mid-replay when a commit ran this hook.
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
    Copy-Item .\Display_Config\Display_Config_Resources\Injector\Release\Injector.exe '{{supreme_folder}}\Display_Config_Resources\' -Force; \
    Write-Host "Deployed TAS (+ Injector.exe: TAS_Helper's DllMain no longer initializes, the injector calls TAS_Initialize) to $dest"

# Launch the game (direct exe, NO scripted navigation — scripted nav can land
# the engine in its demo/attract state) and a fresh tas_ui AFTER it. Order
# matters: the DLL maps shared memory at game start; tas_ui must connect to
# the new instance, never reuse a stale one.
relaunch:
    Start-Process -FilePath '{{supreme_folder}}\Supreme.exe' -WorkingDirectory '{{supreme_folder}}'; \
    Start-Sleep -Seconds 4; \
    $dest = '{{supreme_folder}}\Display_Config_Resources\TAS'; \
    Start-Process -FilePath "$dest\tas_ui.exe" -WorkingDirectory $dest; \
    Write-Host "Relaunched game + tas_ui (navigate into the race manually)"

# One-shot: build, stop, deploy, relaunch — the only deploy flow to use.
deploy_run: deploy relaunch

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

test_tools:
    python -m unittest discover -s TAS/tools -p test_tools.py -v

test_smoke: tas_rust
    cd TAS && cargo run --release --bin tas_test -- smoke

test_acceptance: tas_rust
    cd TAS && cargo run --release --bin tas_test -- acceptance

test_regression: tas_rust
    cd TAS && cargo run --release --bin tas_test -- regression

# Requires the deployed UI with a saved FE run loaded and From set to splice.
test_cont_ui_left_spam log splice="2200" iterations="5":
    cd TAS && cargo run --release --bin tas_test -- cont-ui-left-spam --log '{{log}}' --splice {{splice}} --iterations {{iterations}}

test_replay file iterations="5":
    cd TAS && cargo run --release --bin tas_test -- replay {{file}} --iterations {{iterations}} --verbose

# UI runs first: later harness stages stop the competing UI and replace the run.
test_live splice="4500" iterations="5" log=(supreme_folder + '\Display_Config_Resources\TAS\data\tas_ui.log'): tas_rust
    cd TAS && cargo run --release --bin tas_test -- live --log '{{log}}' --splice {{splice}} --iterations {{iterations}}

clean:
    $ErrorActionPreference = 'Stop'; \
    cd Display_Config && just clean; \
    if (Test-Path .\TAS\Release) { Remove-Item .\TAS\Release -Recurse -Force }; \
    if (Test-Path .\TAS\target) { Remove-Item .\TAS\target -Recurse -Force }
