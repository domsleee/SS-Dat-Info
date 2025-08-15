# Display Config

A launcher for Supreme Snowboarding that adds QoL features and tweaks.

- Download: [Latest Release][display-config-release]
- Full guide and docs: [Display_Config wiki][display-config-docs]


[display-config-release]: https://github.com/domsleee/SS-Dat-Info/releases/latest
[display-config-docs]: https://github.com/domsleee/SS-Dat-Info/wiki/Display_Config

## Configuration

The structure of `Display_Config_Resources/Display_Config_Helper.json` is as follows:

```json
{
  "use4xFonts": true,
  "changeFov": false,
  "fovWidth": 1920,
  "fovHeight": 1080,
  "enableLogging": true,
  "makeGhostsOpaque": true,
  "matchGhostSoundsToCharacter": true,
  "disableDirectInput": true,
  "enableCustomControls": true,
  "hideBlinkingR": true,
  "showReplaySpeed": true
}
```
This json matches to the GUI shown in the [Display_Config wiki][display-config-docs].

## Building

1. Use [scoop](https://scoop.sh/) to install required tools `scoop install just task rustup bun`
2. Install [VS Community](https://visualstudio.microsoft.com/vs/community/) and tick "Desktop development with C++"

Then you can build using `just`.

## Structure

* `./Display_Config`: the launcher (`Display_Config.exe`, tauri app)
* `./Display_Config_Resources`: c++ code that runs injects the mods into `Supreme.exe` at runtime (`Injector.exe`)

## Development flow

* Run `bun tauri dev` from `./Display_Config`, when editing the UI.
* To update the `Display_Config.exe` + `Injector.exe`:
```shell
just; cp -r ./output/* C:\Games\Supreme2\
```
* To update just `Injector.exe`:
```shell
just display_config_helper; cp -r .\output\Display_Config_Resources\* C:\Games\Supreme\Display_Config_Resources\
```

## Software used for reverse engineering
* [Cheat Engine](https://www.cheatengine.org/)
* [Ghidra](https://ghidra-sre.org/)
