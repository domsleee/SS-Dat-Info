# Display Config

Example `Display_Config_Helper.json`:

```json
{
  "use4xFonts": true,
  "changeFov": false,
  "fovWidth": 1920,
  "fovHeight": 1080,
  "enableLogging": true,
  "makeGhostsOpaque": true,
  "matchGhostSoundsToCharacter": true
}
```

Notes:
* `use4xFonts` currently requires changing the font textures yourself 

## Building

1. Install [scoop](https://scoop.sh/)
2. Install everything using scoop `scoop install just task rustup bun`
3. Install [VS Community](https://visualstudio.microsoft.com/vs/community/) and tick ""Desktop development with C++"

Then you can build using `just`.

Development flow:
* Using `bun tauri dev` inside `Display_Config`, to configure the UI
* To update the injection code:
> just display_config_helper && cp -r .\output\Display_Config_Resources\* C:\Games\Supreme\Display_Config_Resources\

Software used to find what to replace:
* [Cheat Engine](https://www.cheatengine.org/)
* [Ghidra](https://ghidra-sre.org/)