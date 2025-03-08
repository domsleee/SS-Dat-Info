#pragma once
#include <filesystem>
#include <iostream>
#include "stdafx.h"

std::string GetInResourceDir(std::string filename) {
    char dllPath[MAX_PATH];
    GetModuleFileNameA(nullptr, dllPath, MAX_PATH);
    auto basePath = std::string(dllPath).substr(0, std::string(dllPath).find_last_of("\\/") + 1);
    auto resourceDir = basePath + "Display_Config_Resources" + "\\";
    return resourceDir + filename;
}