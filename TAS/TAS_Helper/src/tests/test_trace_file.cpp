#include <windows.h>
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <string>
#include "check.hpp"

static bool fail_flush, fail_close, fail_publish, fail_cleanup;
static int TestFlush(FILE* f) {
    const int result = fflush(f);
    return fail_flush ? EOF : result;
}
static int TestClose(FILE* f) {
    const int result = fclose(f);
    return fail_close ? EOF : result;
}
static BOOL TestPublish(const char* from, const char* to, DWORD flags) {
    return fail_publish ? FALSE : MoveFileExA(from, to, flags);
}
static BOOL TestDelete(const char* path) {
    return fail_cleanup ? FALSE : DeleteFileA(path);
}
#define fflush TestFlush
#define fclose TestClose
#define MoveFileExA TestPublish
#define DeleteFileA TestDelete
#include "../trace_file.hpp"
#undef fflush
#undef fclose
#undef MoveFileExA
#undef DeleteFileA

int main() {
    char dir[MAX_PATH], unique[MAX_PATH];
    check(GetTempPathA(MAX_PATH, dir) != 0 && GetTempFileNameA(dir, "ssb", 0, unique) != 0, "private_temp_path");
    if (g_failures) return FinishTests();
    DeleteFileA(unique);
    const std::string pending = std::string(unique) + ".pending";
    const std::string complete = std::string(unique) + ".ssbtrace";
    const char trailer[16] = {'S', 'S', 'B', 'T', 'E', 'N', 'D', '\n'};
    for (unsigned failure = 0; failure < 6; ++failure) {
        fail_flush = failure == 1;
        fail_close = failure == 2 || failure == 5;
        fail_publish = failure == 3;
        fail_cleanup = failure == 5;
        FILE* f = nullptr;
        check(fopen_s(&f, pending.c_str(), "wb") == 0 && f, "open_pending");
        if (!f) break;
        check(fwrite(trailer, 1, sizeof trailer, f) == sizeof trailer, "zero_status_trailer_written");
        check(GetFileAttributesA(complete.c_str()) == INVALID_FILE_ATTRIBUTES, "not_published_while_open");
        const uint32_t status = tracefile::CloseAndPublish(f, failure == 4 ? 1 : 0, pending.c_str(), complete.c_str());
        check((status == 0) == (failure == 0), "checked_failure_status");
        check((GetFileAttributesA(complete.c_str()) != INVALID_FILE_ATTRIBUTES) == (failure == 0), "only_checked_close_publishes");
        check((GetFileAttributesA(pending.c_str()) != INVALID_FILE_ATTRIBUTES) == fail_cleanup, "pending_only_remains_if_cleanup_failed");
        check(((status & 16) != 0) == fail_cleanup, "cleanup_failure_is_reported");
        if (!failure) {
            char bytes[16] = {};
            check(fopen_s(&f, complete.c_str(), "rb") == 0 && f, "open_complete");
            if (f) {
                check(fread(bytes, 1, sizeof bytes, f) == sizeof bytes && !memcmp(bytes, trailer, sizeof bytes), "trailer_bytes_preserved");
                fclose(f);
            }
            DeleteFileA(complete.c_str());
        }
        DeleteFileA(pending.c_str());
    }
    fail_flush = fail_close = fail_publish = fail_cleanup = false;
    FILE* old = nullptr;
    fopen_s(&old, complete.c_str(), "wb");
    check(old != nullptr, "create_existing_complete");
    if (old) { fputs("original", old); fclose(old); }
    FILE* fresh = nullptr;
    fopen_s(&fresh, pending.c_str(), "wb");
    check(fresh != nullptr, "create_colliding_pending");
    if (fresh) check(tracefile::CloseAndPublish(fresh, 0, pending.c_str(), complete.c_str()) != 0, "collision_fails_closed");
    char saved[9] = {};
    fopen_s(&old, complete.c_str(), "rb");
    if (old) { fread(saved, 1, 8, old); fclose(old); }
    check(!strcmp(saved, "original"), "existing_complete_preserved");
    DeleteFileA(pending.c_str());
    DeleteFileA(complete.c_str());
    return FinishTests();
}
