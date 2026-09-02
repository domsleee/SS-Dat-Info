#include <windows.h>
#include <tlhelp32.h>
#include <iostream>
#include <fstream>
#include <filesystem>
#include <cstring>

static std::ofstream g_log;

void Log(const std::string& msg) {
    std::cout << msg << "\n";
    if (g_log.is_open()) {
        g_log << msg << "\n";
        g_log.flush();
    }
}

DWORD FindProcess(const wchar_t* name) {
    DWORD processId = 0;
    PROCESSENTRY32W entry = { sizeof(entry) };
    HANDLE snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if (snap == INVALID_HANDLE_VALUE) {
        Log("INVALID HANDLE VALUE, EXITING");
        return 0;
    }

    if (Process32FirstW(snap, &entry)) {
        do {
            if (wcsstr(entry.szExeFile, name)) {
                processId = entry.th32ProcessID;
                break;
            }
        } while (Process32NextW(snap, &entry));
    }

    CloseHandle(snap);
    return processId;
}

bool Inject(DWORD pid, const std::string& dll) {
    HANDLE hProc = OpenProcess(PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_CREATE_THREAD, 0, pid);
    if (!hProc) {
        Log("Failed to open process " + std::to_string(pid) + " (error " + std::to_string(GetLastError()) + ")");
        return false;
    }
    Log("OpenProcess OK (handle=" + std::to_string(reinterpret_cast<uintptr_t>(hProc)) + ")");

    const SIZE_T pathBytes = dll.size() + 1;
    LPVOID mem = VirtualAllocEx(
        hProc, nullptr, pathBytes, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (!mem) {
        Log("VirtualAllocEx failed (error " + std::to_string(GetLastError()) + ")");
        CloseHandle(hProc);
        return false;
    }
    Log("VirtualAllocEx OK (addr=" + std::to_string(reinterpret_cast<uintptr_t>(mem)) + ")");

    SIZE_T bytesWritten = 0;
    BOOL wrote = WriteProcessMemory(
        hProc, mem, dll.c_str(), pathBytes, &bytesWritten);
    bool pathWritten = wrote && bytesWritten == pathBytes;
    Log("WriteProcessMemory: " + std::string(pathWritten ? "OK" : "FAILED") +
        " (" + std::to_string(bytesWritten) + "/" + std::to_string(pathBytes) + " bytes)");
    if (!pathWritten) {
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }

    HANDLE hThread = CreateRemoteThread(hProc, 0, 0, (LPTHREAD_START_ROUTINE)LoadLibraryA, mem, 0, 0);
    if (!hThread) {
        Log("CreateRemoteThread FAILED (error " + std::to_string(GetLastError()) + ")");
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }

    Log("CreateRemoteThread OK, waiting...");
    DWORD wait = WaitForSingleObject(hThread, 5000);
    if (wait != WAIT_OBJECT_0) {
        Log("LoadLibraryA remote thread timed out/failed (wait=" + std::to_string(wait) + ")");
        // The remote thread may still be reading the path. Do not free it out
        // from underneath that thread on timeout.
        CloseHandle(hThread);
        CloseHandle(hProc);
        return false;
    }

    DWORD remoteModule = 0;
    if (!GetExitCodeThread(hThread, &remoteModule) || remoteModule == 0) {
        Log("LoadLibraryA returned NULL — DLL failed to load");
        CloseHandle(hThread);
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }
    CloseHandle(hThread);
    VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);

    // TAS_Helper keeps DllMain loader-lock-safe. If the loaded DLL exports an
    // explicit initializer, resolve its RVA without running local DllMain and
    // invoke it only after the target's LoadLibrary call has returned.
    bool requiresExplicitInit = _stricmp(
        std::filesystem::path(dll).filename().string().c_str(), "TAS_Helper.dll") == 0;
    HMODULE localModule = LoadLibraryExA(dll.c_str(), nullptr, DONT_RESOLVE_DLL_REFERENCES);
    if (!localModule && requiresExplicitInit) {
        Log("Could not inspect TAS_Helper.dll exports (error " +
            std::to_string(GetLastError()) + ")");
        CloseHandle(hProc);
        return false;
    }
    if (localModule) {
        FARPROC localInit = GetProcAddress(localModule, "TAS_Initialize");
        if (localInit) {
            uintptr_t initRva = reinterpret_cast<uintptr_t>(localInit)
                - reinterpret_cast<uintptr_t>(localModule);
            auto remoteInit = reinterpret_cast<LPTHREAD_START_ROUTINE>(
                static_cast<uintptr_t>(remoteModule) + initRva);
            HANDLE initThread = CreateRemoteThread(hProc, nullptr, 0, remoteInit, nullptr, 0, nullptr);
            if (!initThread) {
                Log("TAS_Initialize CreateRemoteThread failed (error " +
                    std::to_string(GetLastError()) + ")");
                FreeLibrary(localModule);
                CloseHandle(hProc);
                return false;
            }
            DWORD initWait = WaitForSingleObject(initThread, 10000);
            DWORD initResult = 0;
            bool initialized = initWait == WAIT_OBJECT_0
                && GetExitCodeThread(initThread, &initResult)
                && initResult != 0;
            CloseHandle(initThread);
            if (!initialized) {
                Log("TAS_Initialize failed/timed out (wait=" + std::to_string(initWait) +
                    " result=" + std::to_string(initResult) + ")");
                FreeLibrary(localModule);
                CloseHandle(hProc);
                return false;
            }
            Log("TAS_Initialize completed successfully");
        } else if (requiresExplicitInit) {
            Log("TAS_Helper.dll does not export TAS_Initialize");
            FreeLibrary(localModule);
            CloseHandle(hProc);
            return false;
        }
        FreeLibrary(localModule);
    }

    CloseHandle(hProc);
    return true;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Usage: Injector.exe <dll-path>\n";
        return 1;
    }

    // Open log file next to Injector.exe
    auto exePath = std::filesystem::path(argv[0]).parent_path();
    auto logPath = exePath / "Injector.log";
    g_log.open(logPath, std::ios::app);
    Log("--- Injector started ---");
    Log("argc=" + std::to_string(argc) + " argv[1]=" + std::string(argv[1]));

    DWORD pid = FindProcess(L"Supreme_v1.035.exe");
    if (!pid) pid = FindProcess(L"Supreme.exe");

    std::filesystem::path dllPath(argv[1]);
    if (dllPath.is_relative()) {
        dllPath = std::filesystem::current_path() / dllPath;
    }

    Log("DLL path: " + dllPath.string());
    Log("Target PID: " + std::to_string(pid));

    if (!std::filesystem::exists(dllPath)) {
        Log("ERROR: DLL file does not exist at " + dllPath.string());
        return 1;
    }

    if (pid) {
        if (!Inject(pid, dllPath.string())) {
            Log("Injection failed");
            return 1;
        }
        Log("Injection complete");
    } else {
        Log("Supreme.exe process not found");
        return 1;
    }
    return 0;
}
