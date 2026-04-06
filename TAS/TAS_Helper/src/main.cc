#include <windows.h>
#include "log.hpp"
#include "helper.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "caves/cave1_replay.hpp"
#include "caves/cave2.hpp"
#include "caves/cave1c.hpp"
#include "caves/cave1d.hpp"
#include "caves/cave5.hpp"

static TasSharedMemory g_sharedMem;
static GameAddresses g_addr;

namespace {

struct HookInstallResult {
    bool ok;
    bool disabled;
};

std::string ReadEnvVar(const char* name) {
    DWORD len = GetEnvironmentVariableA(name, nullptr, 0);
    if (len == 0) {
        return {};
    }

    std::string value(len, '\0');
    DWORD written = GetEnvironmentVariableA(name, value.data(), len);
    if (written == 0) {
        return {};
    }
    value.resize(written);
    return value;
}

std::string ToLowerAscii(std::string value) {
    for (char& ch : value) {
        if (ch >= 'A' && ch <= 'Z') {
            ch = static_cast<char>(ch - 'A' + 'a');
        }
    }
    return value;
}

bool HookEnabled(const char* hookKey) {
    std::string raw = ToLowerAscii(ReadEnvVar("TAS_DISABLE_HOOKS"));
    if (raw.empty()) {
        return true;
    }

    std::string key = ToLowerAscii(hookKey);
    std::string token;
    for (char ch : raw) {
        bool delimiter = ch == ',' || ch == ';' || ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n';
        if (delimiter) {
            if (token == "all" || token == key) {
                return false;
            }
            token.clear();
            continue;
        }
        token.push_back(ch);
    }

    return token != "all" && token != key;
}

template <typename Fn>
HookInstallResult InstallHookIfEnabled(const char* hookKey, const char* label, Fn&& installFn) {
    if (!HookEnabled(hookKey)) {
        Log(std::format("{}: disabled by TAS_DISABLE_HOOKS", label));
        return {false, true};
    }
    return {installFn(), false};
}

const char* HookStatusText(const HookInstallResult& result) {
    if (result.disabled) {
        return "DISABLED";
    }
    return result.ok ? "OK" : "FAILED";
}

}  // namespace

void run() {
    Log("=== TAS_Helper.dll loading (Phase 2) ===");
    Log(std::format("  sizeof(TasSharedState) = {}", sizeof(TasSharedState)));

    // Step 1: Create shared memory
    if (!g_sharedMem.Create()) {
        Log("FATAL: Failed to create shared memory mapping");
        return;
    }
    Log(std::format("Shared memory '{}' created ({} bytes)",
        TAS_SHARED_MEMORY_NAME, sizeof(TasSharedState)));

    // Step 2: Resolve game addresses
    if (!g_addr.Resolve()) {
        Log("FATAL: Failed to resolve game addresses");
        return;
    }

    auto* state = g_sharedMem.state;

    // Step 3: Install hooks
    // Order matters: Cave 1D (BB3B10 gate) must be installed before Cave 2
    // because Cave 2 calls BB3B10 directly. Cave 1C (handler gate) must be
    // installed before Cave 2 for the same reason.
    std::string disabledHooks = ReadEnvVar("TAS_DISABLE_HOOKS");
    if (!disabledHooks.empty()) {
        Log(std::format("TAS_DISABLE_HOOKS={}", disabledHooks));
    }

    HookInstallResult replay = InstallHookIfEnabled("replay", "Replay capture", [&] {
        return InstallReplayCapture(g_addr, state);
    });
    HookInstallResult cave1d = InstallHookIfEnabled("cave1d", "Cave 1D", [&] {
        return InstallCave1D(g_addr, state);
    });
    HookInstallResult cave1c = InstallHookIfEnabled("cave1c", "Cave 1C", [&] {
        return InstallCave1C(g_addr, state);
    });
    HookInstallResult cave2 = InstallHookIfEnabled("cave2", "Cave 2", [&] {
        return InstallCave2(g_addr, state);
    });
    HookInstallResult cave5 = InstallHookIfEnabled("cave5", "Cave 5", [&] {
        return InstallCave5(g_addr, state);
    });

    Log("=== Hook installation summary ===");
    Log(std::format("  Replay capture (SG+9E8F0):  {}", HookStatusText(replay)));
    Log(std::format("  Cave 1D (BB3B10 gate):      {}", HookStatusText(cave1d)));
    Log(std::format("  Cave 1C (handler gate):     {}", HookStatusText(cave1c)));
    Log(std::format("  Cave 2  (Supreme::Cycle):   {}", HookStatusText(cave2)));
    Log(std::format("  Cave 5  (fixed tick):       {}", HookStatusText(cave5)));

    if (!cave2.ok) {
        if (cave2.disabled) {
            Log("CRITICAL: Cave 2 disabled - TAS record/playback will not function");
        } else {
            Log("CRITICAL: Cave 2 hook failed - TAS will not function");
        }
    }
    if (!cave1d.ok || !cave1c.ok) {
        Log("WARNING: Gate hooks failed - input blocking won't work correctly");
    }

    Log("=== TAS_Helper.dll ready (Phase 2) ===");
}

BOOL APIENTRY DllMain(HMODULE, DWORD reason, LPVOID) {
    if (reason == DLL_PROCESS_DETACH) {
        Log("TAS_Helper.dll unloading");
        g_sharedMem.Destroy();
        return TRUE;
    }

    if (reason != DLL_PROCESS_ATTACH) {
        return TRUE;
    }

    try {
        run();
    }
    catch (const std::exception& e) {
        Log(std::format("FATAL exception: {}", e.what()));
        return TRUE;
    }

    return TRUE;
}
