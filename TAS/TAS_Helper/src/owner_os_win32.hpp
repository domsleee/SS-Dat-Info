#pragma once
#include <windows.h>
#include "controller_owner.hpp"

// The real OS calls behind OwnerTracker.
namespace owner_os {

inline void* Open(uint32_t pid) {
    return OpenProcess(SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
}

inline bool Created(void* handle, uint64_t* filetime) {
    FILETIME created{}, exited{}, kernel{}, user{};
    if (!GetProcessTimes(handle, &created, &exited, &kernel, &user)) return false;
    *filetime = ((uint64_t)created.dwHighDateTime << 32) | created.dwLowDateTime;
    return true;
}

inline OwnerLiveness Poll(void* handle) {
    switch (WaitForSingleObject(handle, 0)) {
        case WAIT_TIMEOUT: return OwnerLiveness::Alive;
        case WAIT_OBJECT_0: return OwnerLiveness::Exited;
        default: return OwnerLiveness::Unknown;
    }
}

inline void Close(void* handle) { CloseHandle(handle); }

inline constexpr OwnerOs kWin32{&Open, &Created, &Poll, &Close};

}  // namespace owner_os
