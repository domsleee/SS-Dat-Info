#include <windows.h>
#include <cstdio>
#include <atomic>
#include <string>
#include <thread>
#include "check.hpp"

static unsigned reads, fail_read;
static BOOL TestRead(HANDLE f, LPVOID buf, DWORD len, LPDWORD n, LPOVERLAPPED ov) {
    if (++reads == fail_read) { SetLastError(ERROR_READ_FAULT); return FALSE; }
    return ReadFile(f, buf, len, n, ov);
}
static HANDLE flushing, release_flush;
static int TestFlush(FILE* f) {
    if (flushing) {
        SetEvent(flushing);
        WaitForSingleObject(release_flush, INFINITE);
    }
    return fflush(f);
}
#define ReadFile TestRead
#define fflush TestFlush
#include "../trace_provenance.hpp"
#undef ReadFile
#undef fflush

int main() {
    wchar_t dir[MAX_PATH], unique[MAX_PATH];
    check(GetTempPathW(MAX_PATH, dir) && GetTempFileNameW(dir, L"ssb", 0, unique), "private_temp_path");
    if (g_failures) return FinishTests();
    FILE* source = nullptr;
    _wfopen_s(&source, unique, L"wb");
    check(source != nullptr, "open_hash_source");
    if (!source) return FinishTests();
    fputs("abc", source);
    fclose(source);
    const std::string sha = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    check(traceprovenance::Sha256File(unique) == sha, "complete_file_known_hash");
    reads = 0; fail_read = 2;
    check(traceprovenance::Sha256File(unique) == "unreadable", "late_read_failure_is_not_prefix_digest");
    reads = 0; fail_read = 1;
    check(traceprovenance::Sha256File(unique) == "unreadable", "first_read_failure_is_not_empty_digest");
    fail_read = 0;
    DeleteFileW(unique);

    // ASCII temp names for the production publication API.
    char tmp[MAX_PATH], name[MAX_PATH];
    check(GetTempPathA(MAX_PATH, tmp) && GetTempFileNameA(tmp, "ssb", 0, name), "private_publication_path");
    if (g_failures) return FinishTests();
    DeleteFileA(name);
    const std::string pending = std::string(name) + ".pending", complete = std::string(name) + ".ssbtrace";
    auto provisional = [&]() {
        FILE* f = nullptr;
        fopen_s(&f, pending.c_str(), "wb");
        if (f) { const char footer[16] = {'S','S','B','T','E','N','D','\n'}; fwrite(footer, 1, sizeof footer, f); }
        return f;
    };
    auto publish = [&](traceprovenance::OpenedFiles& files) {
        FILE* f = provisional();
        check(f != nullptr, "open_pending");
        return f ? files.CloseAndPublish(f, 0, pending.c_str(), complete.c_str()) : 1u;
    };
    auto cleanup = [&]() { DeleteFileA(pending.c_str()); DeleteFileA(complete.c_str()); };
    for (unsigned mode = 0; mode != 4; ++mode) {
        traceprovenance::OpenedFiles files;
        files.Note("data/a", sha);
        check(files.Snapshot() == "file " + sha + " data/a\n", "header_has_preexisting_identity");
        files.Note(mode == 1 ? "data/b" : "data/a", mode == 2 ? std::string(64, '0') : mode == 3 ? "unreadable" : sha);
        const uint32_t status = publish(files);
        check((status == 0) == (mode == 0), "only_identical_reopen_publishes");
        check((GetFileAttributesA(complete.c_str()) != INVALID_FILE_ATTRIBUTES) == (mode == 0), "late_identity_never_published");
        check(GetFileAttributesA(pending.c_str()) == INVALID_FILE_ATTRIBUTES, "failed_provenance_pending_removed");
        cleanup();
    }

    bool snapshot_races_safe = true;
    for (unsigned i = 0; i != 50; ++i) {
        traceprovenance::OpenedFiles files;
        std::atomic<bool> go{false};
        std::thread reader([&]() { while (!go.load()) SwitchToThread(); files.Note("data/racing", sha); });
        go.store(true);
        const std::string header = files.Snapshot();
        reader.join();
        const uint32_t status = publish(files);
        snapshot_races_safe &= !header.empty() || status == 32;
        cleanup();
    }
    check(snapshot_races_safe, "snapshot_race_identity_is_in_header_or_rejected");

    // Hold close inside fflush. A later Note must not slip between the final
    // generation check and rename; it completes only after publication.
    traceprovenance::OpenedFiles files;
    files.Snapshot();
    flushing = CreateEventW(nullptr, TRUE, FALSE, nullptr);
    release_flush = CreateEventW(nullptr, TRUE, FALSE, nullptr);
    std::atomic<bool> noting{false}, noted{false};
    uint32_t status = 1;
    FILE* f = provisional();
    check(f && flushing && release_flush, "publication_race_resources");
    if (f && flushing && release_flush) {
        std::thread closer([&]() { status = files.CloseAndPublish(f, 0, pending.c_str(), complete.c_str()); });
        WaitForSingleObject(flushing, INFINITE);
        std::thread reader([&]() { noting.store(true); files.Note("data/after-close", sha); noted.store(true); });
        while (!noting.load()) SwitchToThread();
        Sleep(20);
        check(!noted.load(), "open_observation_cannot_return_during_publication");
        SetEvent(release_flush);
        closer.join(); reader.join();
        check(status == 0 && !files.Snapshot().empty(), "post_publication_observation_belongs_to_next_snapshot");
    } else if (f) fclose(f);
    if (flushing) CloseHandle(flushing);
    if (release_flush) CloseHandle(release_flush);
    flushing = release_flush = nullptr;
    cleanup();
    return FinishTests();
}
