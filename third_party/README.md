# Vendored third-party code

Libraries compiled into `Display_Config/Display_Config_Resources/Display_Config_Helper`.

To use them from a project, import `third_party.props` once (not per configuration).
It adds this directory to the include path:

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
| `safetyhook/safetyhook.hpp`, `safetyhook/safetyhook.cpp` | https://github.com/cursey/safetyhook | v0.5.3 (`safetyhook-amalgamated.zip` release asset), plus the local patch below | Boost Software License 1.0 (`safetyhook/LICENSE`) |
| `safetyhook/Zydis.h`, `safetyhook/Zydis.c` | https://github.com/zyantific/zydis | v4.0.0 amalgamation, which bundles Zycore-C v1.4.1. `Zydis.c` has been reformatted (brace style only) and includes `"Zydis.h"` rather than `<Zydis.h>` | MIT (`safetyhook/LICENSE.zydis`, `safetyhook/LICENSE.zycore`) |
| `nlohmann/json.hpp` | https://github.com/nlohmann/json | v3.12.0 (`json.hpp` release asset, unmodified) | MIT (`nlohmann/LICENSE.MIT`) |

## Local patches

- `safetyhook/safetyhook.cpp`: checks for `"Zydis.h"` before `"Zydis/Zydis.h"`, so it always
  uses the Zydis next to it, even when another Zydis (e.g. from vcpkg) is on the include path.

## Updating

Replace the files with the new release, update the version column, and reapply the local patches.
