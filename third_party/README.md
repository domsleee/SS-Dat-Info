# Vendored third-party code

One copy, shared by `TAS/TAS_Helper` and
`Display_Config/Display_Config_Resources/Display_Config_Helper`; each vcxproj
adds the directory to `AdditionalIncludeDirectories` and compiles the `.c`/`.cpp`
from here, so sources include `<safetyhook.hpp>` / `<json.hpp>`.

| Directory | Upstream | Version | License |
|---|---|---|---|
| `safetyhook/` (`safetyhook.hpp`, `safetyhook.cpp`) | https://github.com/cursey/safetyhook | amalgamated build, vendored 2025-03-08 (commit db6d286 of this repo); the amalgamation carries no version macro | Boost Software License 1.0 (`safetyhook/LICENSE`) |
| `safetyhook/` (`Zydis.h`, `Zydis.c`) | https://github.com/zyantific/zydis | 4.0.0 (`ZYDIS_VERSION` in `Zydis.h`), amalgamated; safetyhook needs it for instruction-length decoding | MIT (`safetyhook/LICENSE.zydis`) |
| `json/json.hpp` | https://github.com/nlohmann/json | 3.11.3 (`NLOHMANN_JSON_VERSION_*`), single header; Display_Config_Helper only | MIT (SPDX header in the file) |

Update by replacing the files with a fresh amalgamation and recording the new
version here.
