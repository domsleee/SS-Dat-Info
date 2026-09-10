#pragma once
#include <windows.h>

// Only DLL owners hold this handle; UI clients may retain the data mapping
// after a game exits. Reject a second owner without rejecting those clients.
class MappingOwner {
    HANDLE handle_ = nullptr;
public:
    MappingOwner() = default;
    MappingOwner(const MappingOwner&) = delete;
    MappingOwner& operator=(const MappingOwner&) = delete;

    bool Acquire(const char* name) {
        if (handle_) return true;
        HANDLE handle = CreateMutexA(nullptr, FALSE, name);
        if (!handle) return false;
        if (GetLastError() == ERROR_ALREADY_EXISTS) {
            CloseHandle(handle);
            return false;
        }
        handle_ = handle;
        return true;
    }

    void Release() {
        if (handle_) {
            CloseHandle(handle_);
            handle_ = nullptr;
        }
    }

    ~MappingOwner() { Release(); }
};
