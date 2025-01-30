#pragma once
#include <filesystem>
#include <iostream>
#include "stdafx.h"
#include "PathUtil.hpp"

void Log(const std::string& message) {
    auto logPath = GetInResourceDir("Display_Config_Helper.log");
    std::ofstream logFile(logPath, std::ios::app);
    if (logFile) logFile << message << "\n";
}
