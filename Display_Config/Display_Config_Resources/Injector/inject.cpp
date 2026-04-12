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

void Inject(DWORD pid, std::string dll) {
    HANDLE hProc = OpenProcess(PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_CREATE_THREAD, 0, pid);
    if (!hProc) {
        Log("Failed to open process " + std::to_string(pid) + " (error " + std::to_string(GetLastError()) + ")");
        return;
    }
    Log("OpenProcess OK (handle=" + std::to_string(reinterpret_cast<uintptr_t>(hProc)) + ")");

    LPVOID mem = VirtualAllocEx(hProc, 0, MAX_PATH, MEM_COMMIT, PAGE_READWRITE);
    if (!mem) {
        Log("VirtualAllocEx failed (error " + std::to_string(GetLastError()) + ")");
        CloseHandle(hProc);
        return;
    }
    Log("VirtualAllocEx OK (addr=" + std::to_string(reinterpret_cast<uintptr_t>(mem)) + ")");

    BOOL wrote = WriteProcessMemory(hProc, mem, dll.c_str(), dll.length() + 1, 0);
    Log("WriteProcessMemory: " + std::string(wrote ? "OK" : "FAILED"));

    HANDLE hThread = CreateRemoteThread(hProc, 0, 0, (LPTHREAD_START_ROUTINE)LoadLibraryA, mem, 0, 0);
    if (hThread) {
        Log("CreateRemoteThread OK, waiting...");
        DWORD wait = WaitForSingleObject(hThread, 5000);
        DWORD exitCode = 0;
        GetExitCodeThread(hThread, &exitCode);
        Log("Thread finished (wait=" + std::to_string(wait) + " exitCode=0x" +
            ([](DWORD v) { char buf[16]; snprintf(buf, sizeof(buf), "%08X", v); return std::string(buf); })(exitCode) + ")");
        if (exitCode == 0) {
            Log("WARNING: LoadLibraryA returned NULL — DLL failed to load!");
        }
        CloseHandle(hThread);
    } else {
        Log("CreateRemoteThread FAILED (error " + std::to_string(GetLastError()) + ")");
    }

    VirtualFreeEx(hProc, mem, 0, MEM_RELEASE);
    CloseHandle(hProc);
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
        Inject(pid, dllPath.string());
        Log("Injection complete");
    } else {
        Log("Supreme.exe process not found");
        return 1;
    }
    return 0;
}
