# Vendored third-party code

Libraries compiled into `Display_Config/Display_Config_Resources/Display_Config_Helper`.

To use them from a project, import `third_party.props` once (not per configuration).
It adds this directory to the include path. The path is relative to the project file, so use
one `..\` per folder between the project and the repo root. For `Display_Config_Helper`, which
is three folders deep:

```xml
<ImportGroup Label="PropertySheets">
  <Import Project="..\..\..\third_party\third_party.props" />
</ImportGroup>
```

Include headers with their directory, e.g. `#include <safetyhook/safetyhook.hpp>` and
`#include <nlohmann/json.hpp>`. The project still has to compile `safetyhook/safetyhook.cpp`
and `safetyhook/Zydis.c` itself.

| Files | Upstream | Version | License |
|---|---|---|---|
| `safetyhook/safetyhook.hpp`, `safetyhook/safetyhook.cpp` | https://github.com/cursey/safetyhook | v0.7.0 (`safetyhook-amalgamated.zip` release asset, unmodified) | Boost Software License 1.0 (`safetyhook/LICENSE`) |
| `safetyhook/Zydis.h`, `safetyhook/Zydis.c` | https://github.com/zyantific/zydis, taken from safetyhook v0.7.0's `safetyhook-amalgamated-zydis.zip` release asset | Zydis v4.0.0 with Zycore-C v1.4.1. Take them from safetyhook's zip, not Zydis's own release: its `Zydis.h` starts with `#define ZYCORE_STATIC_BUILD` and `#define ZYDIS_STATIC_BUILD`, which static linking needs. `Zydis.c` has been reformatted (brace style only), plus the local patch below | MIT (`safetyhook/LICENSE.zydis`, `safetyhook/LICENSE.zycore`) |
| `nlohmann/json.hpp` | https://github.com/nlohmann/json | v3.12.0 (`json.hpp` release asset, unmodified) | MIT (`nlohmann/LICENSE.MIT`) |

## Local patches

- `safetyhook/Zydis.c`: includes `"Zydis.h"` rather than `<Zydis.h>`. `third_party.props` only
  puts `third_party` on the include path, and `<Zydis.h>` doesn't look next to `Zydis.c`.

## Updating

Replace all four `safetyhook` files with those in the new release's `safetyhook-amalgamated-zydis.zip`,
update the version column, and reapply the local patch. The brace-style reformatting doesn't need redoing.
