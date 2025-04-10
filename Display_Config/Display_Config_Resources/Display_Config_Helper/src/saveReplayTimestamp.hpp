#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include <string>
#include <regex>
#include <ctime>

#pragma once
static std::string modifiedPath;

void DoSaveReplayToTimestamp() {
    const auto module = GetModuleHandleA("Supreme_Game.dll");
    if (!module) {
        Log("DoSaveReplayToTimestamp: Failed to find Supreme_Game.dll");
        return;
    }

    std::uint8_t* saveReplayAddress = Memory::PatternScan(module, "00 8B 42 04 6A 22 89 4C 04 30 53 8D 4C 24 38"); // Just before PUSH EBX, Supreme_Game.dll+9FF00
                                                                  //"FF 15 84 36 E0 00 8B 54 24 2C 8B 0D 18 36 E0 00 8B 42 04 6A 22 89 4C 04 30 53"
    if (!saveReplayAddress) {
        Log("DoSaveReplayToTimestamp: Couldn't find setPlayerNameAddress");
        return;
    }

    saveReplayAddress += 0xA;
    Log(std::format("DoSaveReplayToTimestamp: Address is Supreme_Game.dll+{:x}", saveReplayAddress - (std::uint8_t*)module));

    static safetyhook::MidHook saveReplayHook{};
    saveReplayHook = safetyhook::create_mid(saveReplayAddress, [](safetyhook::Context& ctx) {
        static std::unordered_map<std::string, std::string> characterNames;
        char* replayLocation = reinterpret_cast<char*>(ctx.ebx); // example "Data/Levels/Alpine/Tracks/Easy/Replays/Replay.dat"
        modifiedPath = std::string(replayLocation);
        Log(std::format("Existing string {}", replayLocation));

        static const std::string REPLAY_FILENAME = "Replay.dat";
        size_t pos = modifiedPath.find(REPLAY_FILENAME);
        if (pos != std::string::npos) {
            time_t now = time(0);
            struct tm timeinfo;
            localtime_s(&timeinfo, &now);
            char timeBuffer[100];
            strftime(timeBuffer, sizeof(timeBuffer), "%Y-%m-%d-%H%M%S-Replay.dat", &timeinfo);
            modifiedPath.replace(pos, REPLAY_FILENAME.length(), timeBuffer);
            Log(std::format("Modified string {}", modifiedPath));
        }
        ctx.ebx = reinterpret_cast<uintptr_t>(modifiedPath.c_str());
    });
    Log("DoSaveReplayToTimestamp: Fix applied");
}
