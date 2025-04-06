#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include <string>
#include <regex>

#pragma once

void DoMatchGhostSoundsToCharacter() {
    const auto module = GetModuleHandleA("Supreme_Game.dll");
    if (!module) {
        Log("DoMatchGhostSoundsToCharacter: Failed to find Supreme_Game.dll");
        return;
    }

    std::uint8_t* setPlayerNameAddress = Memory::PatternScan(module, "8B CE E8 1F 5F FF FF 8B CE E8 D8 59 FF FF 8B 4C 24 10");
    if (!setPlayerNameAddress) {
        Log("DoMatchGhostSoundsToCharacter: Couldn't find setPlayerNameAddress");
        return;
    }

    Log(std::format("DoMatchGhostSoundsToCharacter: Address is Supreme_Game.dll+{:x}", setPlayerNameAddress - (std::uint8_t*)module));

    static safetyhook::MidHook setNameHook{};
    setNameHook = safetyhook::create_mid(setPlayerNameAddress, [](safetyhook::Context& ctx) {
        static std::unordered_map<std::string, std::string> characterNames;
        char* playerMeshConfig = *reinterpret_cast<char**>(ctx.esi + 0x64); // example "data/characters/vincent/plrmshcnf.txt"
        if (!playerMeshConfig) {
            Log("DoMatchGhostSoundsToCharacter: ERROR: playerMeshConfig is null");
            return;
        }

        std::string configPath(playerMeshConfig);
        std::regex pattern(R"(data/characters/([^/]+)/plrmshcnf\.txt)");
        std::smatch matches;

        if (std::regex_match(configPath, matches, pattern) && matches.size() > 1) {
            std::string characterKey = matches[1].str();

            if (!characterNames.contains(characterKey)) {
                characterNames[characterKey] = characterKey;
            }
            *reinterpret_cast<const char**>(ctx.esp) = characterNames[characterKey].c_str();
        }
        else {
            Log("DoMatchGhostSoundsToCharacter: ERROR: regex match failed for " + std::string(playerMeshConfig ? playerMeshConfig : "null"));
        }
    });
    Log("DoMatchGhostSoundsToCharacter: Fix applied");
}

