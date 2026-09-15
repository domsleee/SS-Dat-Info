#include <windows.h>
#include <tlhelp32.h>
#include <iostream>
#include <fstream>
#include <filesystem>

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
        Log("OpenProcess(" + std::to_string(pid) + ") failed (error " + std::to_string(GetLastError()) + ")");
        return false;
    }

    const SIZE_T pathBytes = dll.size() + 1;
    LPVOID mem = VirtualAllocEx(
        hProc, nullptr, pathBytes, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (!mem) {
        Log("VirtualAllocEx failed (error " + std::to_string(GetLastError()) + ")");
        CloseHandle(hProc);
        return false;
    }

    SIZE_T bytesWritten = 0;
    BOOL wrote = WriteProcessMemory(
        hProc, mem, dll.c_str(), pathBytes, &bytesWritten);
    if (!wrote || bytesWritten != pathBytes) {
        Log("WriteProcessMemory failed (" + std::to_string(bytesWritten) + "/" +
            std::to_string(pathBytes) + " bytes, error " + std::to_string(GetLastError()) + ")");
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }

    HANDLE hThread = CreateRemoteThread(hProc, 0, 0, (LPTHREAD_START_ROUTINE)LoadLibraryA, mem, 0, 0);
    if (!hThread) {
        Log("CreateRemoteThread(LoadLibraryA) failed (error " + std::to_string(GetLastError()) + ")");
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }

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
        Log("LoadLibraryA returned NULL - the DLL failed to load in the target");
        CloseHandle(hThread);
        VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
        CloseHandle(hProc);
        return false;
    }
    CloseHandle(hThread);
    VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);

    CloseHandle(hProc);
    return true;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Usage: Injector.exe <dll-path>\n";
        return 1;
    }

    // One log per run, next to Injector.exe.
    auto exePath = std::filesystem::path(argv[0]).parent_path();
    g_log.open(exePath / "Injector.log", std::ios::trunc);

    std::filesystem::path dllPath(argv[1]);
    if (dllPath.is_relative()) {
        dllPath = std::filesystem::current_path() / dllPath;
    }
    if (!std::filesystem::exists(dllPath)) {
        Log("DLL not found: " + dllPath.string());
        return 1;
    }

    DWORD pid = FindProcess(L"Supreme_v1.035.exe");
    if (!pid) pid = FindProcess(L"Supreme.exe");
    if (!pid) {
        Log("Supreme.exe is not running");
        return 1;
    }

    if (!Inject(pid, dllPath.string())) {
        Log("Injection of " + dllPath.string() + " into PID " + std::to_string(pid) + " failed");
        return 1;
    }
    Log("Injected " + dllPath.string() + " into PID " + std::to_string(pid));
    return 0;
}
