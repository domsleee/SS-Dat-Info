#include <windows.h>
#include "log.hpp"
#include "fixes.hpp"
#include "matchGhostSoundsToCharacter.hpp"
#include "external/json.hpp"
#include "PathUtil.hpp"
#include "disableDirectInput.hpp"
#include <fstream>
using json = nlohmann::json;

void run() {
    Log("Processing");

    auto jsonPath = GetInResourceDir("Display_Config_Helper.json");
    if (!std::filesystem::exists(jsonPath)) {
        Log("Display_Config_Helper.json not found");
        return;
    }

    std::ifstream jsonFile(jsonPath);
    auto data = json::parse(jsonFile);

    if (data.value("changeFov", false)) {
        int fovWidth = data["fovWidth"];
        int fovHeight = data["fovHeight"];
        Log(std::format("Changing FOV to {}x{}", fovWidth, fovHeight));
        DoFovFix(fovWidth, fovHeight);
    }

    if (data.value("use4xFonts", false)) {
        Log("Applying 4x font fix");
        Do4xFont();
    }

    if (data.value("enableLogging", false)) {
        Log("Enable logging");
        EnableLogging();
    }

    if (data.value("makeGhostsOpaque", false)) {
        Log("Make ghosts opaque");
        DoMakeGhostsOpaque();
    }

    if (data.value("matchGhostSoundsToCharacter", false)) {
        Log("Match ghost sounds to character");
        DoMatchGhostSoundsToCharacter();
    }

    if (data.value("disableDirectInput", false)) {
        Log("Disable direct input (fixes some start-up crashes, but joysticks will no longer work)");
        DoDisableDirectInput();
    }
}

BOOL APIENTRY DllMain(HMODULE, DWORD reason, LPVOID) {
    if (reason != DLL_PROCESS_ATTACH)
    {
        Log("Ignored...");
        return TRUE;
    }

    try {
        run();
    }
    catch (const std::exception& e) {
        Log(std::format("Error: {}", e.what()));
        return TRUE;
    }
    Log("Finished.");
    return TRUE;
}
