#include <windows.h>
#include <tlhelp32.h>
#include <iostream>
#include <filesystem>

DWORD FindProcess(const wchar_t* name) {
    DWORD processId = 0;
    PROCESSENTRY32W entry = { sizeof(entry) };
    HANDLE snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if (snap == INVALID_HANDLE_VALUE) {
        std::cout << "INVALID HANDLE VALUE, EXITING\n";
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
    LPVOID mem = VirtualAllocEx(hProc, 0, MAX_PATH, MEM_COMMIT, PAGE_READWRITE);
    WriteProcessMemory(hProc, mem, dll.c_str(), dll.length() + 1, 0);
    CreateRemoteThread(hProc, 0, 0, (LPTHREAD_START_ROUTINE)LoadLibraryA, mem, 0, 0);
}

int main() {
    DWORD pid = FindProcess(L"Supreme_v1.035.exe");
    if (!pid) pid = FindProcess(L"Supreme.exe");

    auto dllPath = std::filesystem::current_path() / "Display_Config_Helper.dll";
    // "C:\\Games\\Supreme\\Display_Config_Resources\\Display_Config_Helper.dll"
    std::cout << "Injecting " << dllPath.string() << " into process " << pid << "\n";
    if (pid) Inject(pid, dllPath.string());
    //if (pid) //Inject(pid, "C:\\Games\\Supreme\\Display_Config_Resources\\Display_Config_Helper.dll");
    else {
        std::cout << "Process not found\n";
        return 1;
    }
    std::cout << "Injection successful\n";
    return 0;
}