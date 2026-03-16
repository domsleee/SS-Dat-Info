#pragma once
#include <filesystem>
#include <iostream>
#include "stdafx.h"

static std::string GetTasLogPath() {
    char exePath[MAX_PATH];
    GetModuleFileNameA(nullptr, exePath, MAX_PATH);
    auto basePath = std::string(exePath).substr(0, std::string(exePath).find_last_of("\\/") + 1);
    return basePath + "TAS_Helper.log";
}

void Log(const std::string& message) {
    static auto logPath = GetTasLogPath();
    std::ofstream logFile(logPath, std::ios::app);
    if (logFile) {
        logFile << message << "\n";
        logFile.flush();
    }
}
