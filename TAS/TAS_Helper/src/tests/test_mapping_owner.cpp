#include "../mapping_owner.hpp"
#include <cstdio>
#include <string>

static DWORD ChildResult(const std::string& name) {
    char exe[MAX_PATH];
    GetModuleFileNameA(nullptr, exe, MAX_PATH);
    std::string command = std::string("\"") + exe + "\" " + name;
    STARTUPINFOA startup = {};
    startup.cb = sizeof(startup);
    PROCESS_INFORMATION process = {};
    if (!CreateProcessA(nullptr, command.data(), nullptr, nullptr, FALSE,
                        CREATE_NO_WINDOW, nullptr, nullptr, &startup, &process)) return 99;
    DWORD result = 99;
    if (WaitForSingleObject(process.hProcess, 10000) == WAIT_OBJECT_0)
        GetExitCodeProcess(process.hProcess, &result);
    CloseHandle(process.hThread);
    CloseHandle(process.hProcess);
    return result;
}

int main(int argc, char** argv) {
    if (argc == 2) {
        MappingOwner owner;
        return owner.Acquire(argv[1]) ? 0 : 1;
    }
    // Never touches the production mapping or its owner name.
    const auto name = "Local\\SupremeTAS.Owner.Test." + std::to_string(GetCurrentProcessId());
    MappingOwner first, second;
    if (!first.Acquire(name.c_str()) || !first.Acquire(name.c_str())) return 1;
    if (second.Acquire(name.c_str()) || ChildResult(name) != 1) return 2;
    first.Release();
    // Child acquires and exits without explicit Release; Windows reclaims it.
    if (ChildResult(name) != 0 || !second.Acquire(name.c_str())) return 3;
    puts("mapping_owner: same-process and cross-process exclusion, release and process-exit recovery PASS");
    return 0;
}
