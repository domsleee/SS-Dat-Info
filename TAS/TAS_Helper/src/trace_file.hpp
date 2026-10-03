#pragma once
#include <windows.h>
#include <cstdio>
#include <cstdint>

namespace tracefile {

// A zero-status trailer is provisional until the stream closes successfully.
// Publish beside the pending file, without replacing an older completed trace.
inline uint32_t CloseAndPublish(FILE* file, uint32_t status, const char* pending, const char* complete) {
    if (fflush(file)) status |= 2;
    if (fclose(file)) status |= 4;
    if (!status && !MoveFileExA(pending, complete, MOVEFILE_WRITE_THROUGH)) status |= 8;
    if (status && !DeleteFileA(pending)) status |= 16;
    return status;
}

} // namespace tracefile
