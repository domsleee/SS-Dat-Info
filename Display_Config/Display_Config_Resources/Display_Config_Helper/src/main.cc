#include <windows.h>
#include "log.hpp"
#include "fixes.hpp"
#include "saveReplayTimestamp.hpp"
#include "matchGhostSoundsToCharacter.hpp"
#include "customInput.hpp"
#include "external/json.hpp"
#include "PathUtil.hpp"
#include "globalState.hpp"
#include "disableDirectInput.hpp"
#include "extendRenderDistance.hpp"
#include "configCrashGuard.hpp"
#include "f5Debounce.hpp"
#include "showSpeedAndHideBlinkingR.hpp"
#include <fstream>
using json = nlohmann::json;

class DisplayConfig {
public:
    // FOV settings
    bool changeFov = false;
    int fovWidth = 0;
    int fovHeight = 0;

    // Feature flags
    bool use4xFonts = false;
    bool enableLogging = false;
    bool makeGhostsOpaque = false;
    bool matchGhostSoundsToCharacter = false;
    bool disableDirectInput = false;
    bool enableCustomControls = false;
    bool showReplaySpeed = false;
    bool hideBlinkingR = false;
    bool extendRenderDistance = true;
};

NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE_WITH_DEFAULT(DisplayConfig,
    changeFov, fovWidth, fovHeight,
    use4xFonts, enableLogging, makeGhostsOpaque,
    matchGhostSoundsToCharacter, disableDirectInput, enableCustomControls, showReplaySpeed, hideBlinkingR,
    extendRenderDistance)


void run() {
    Log("Processing");

    auto jsonPath = GetInResourceDir("Display_Config_Helper.json");
    if (!std::filesystem::exists(jsonPath)) {
        Log("Display_Config_Helper.json not found");
        return;
    }

    std::ifstream jsonFile(jsonPath);
    DisplayConfig config = json::parse(jsonFile).get<DisplayConfig>();

    if (config.changeFov) {
        Log(std::format("Changing FOV to {}x{}", config.fovWidth, config.fovHeight));
        DoFovFix(config.fovWidth, config.fovHeight);
    }

    if (config.use4xFonts) {
        Log("Applying 4x font fix");
        Do4xFont();
    }

    if (config.enableLogging) {
        Log("Enable logging");
        EnableLogging();
    }

    if (config.makeGhostsOpaque || config.enableCustomControls) {
        Log("Make ghosts opaque");
        DoMakeGhostsOpaque();
        GlobalState::ghostsOpaque = config.makeGhostsOpaque;
    }

    if (config.matchGhostSoundsToCharacter) {
        Log("Match ghost sounds to character");
        DoMatchGhostSoundsToCharacter();
    }

    if (config.disableDirectInput) {
        Log("Disable direct input (fixes some start-up crashes, but joysticks will no longer work)");
        DoDisableDirectInput();
    }

    if (config.enableCustomControls) {
        DoSaveReplayToTimestamp();
        DoCustomInput();
    }

    if (config.hideBlinkingR || config.showReplaySpeed) {
        DoShowSpeedAndHideBlinkingR(config.hideBlinkingR, config.showReplaySpeed);
    }

    if (config.extendRenderDistance) {
        // One toggle drives the render-distance patch, the F5 restart
        // debounce and the config-list crash guard: injection is gated on
        // this flag, so bundling keeps them from silently never running when
        // the box is left on (default). A held F5 level-triggers a restart
        // every engine tick, which keeps the renderer inside its multi-frame
        // scene rebuild - at 600m/detail4 that shows as serrated half-built
        // terrain. The debounce makes it one restart per press.
        DoExtendRenderDistance();
        DoF5Debounce();
        DoConfigCrashGuard();
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
