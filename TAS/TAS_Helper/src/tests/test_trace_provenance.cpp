#include <windows.h>
#include <cstdio>
#include <atomic>
#include <cstring>
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
    // The "records" written before the tail; CloseAndPublish appends the late
    // section and the trailer.
    const std::string body = "records";
    auto provisional = [&](const char* mode = "wb") {
        FILE* f = nullptr;
        fopen_s(&f, pending.c_str(), mode);
        if (f && mode[0] == 'w') fwrite(body.data(), 1, body.size(), f);
        return f;
    };
    auto publish = [&](traceprovenance::OpenedFiles& files) {
        FILE* f = provisional();
        check(f != nullptr, "open_pending");
        return f ? files.CloseAndPublish(f, 0, 7, pending.c_str(), complete.c_str()) : 1u;
    };
    // The published tail's late lines, or "?" if the file is not the exact
    // body + section + trailer(7 records, status 0).
    auto published_late = [&]() -> std::string {
        FILE* f = nullptr;
        if (fopen_s(&f, complete.c_str(), "rb") || !f) return "?";
        std::string all;
        char buf[4096];
        size_t n;
        while ((n = fread(buf, 1, sizeof buf, f)) != 0) all.append(buf, n);
        fclose(f);
        const size_t head = body.size() + 12;
        if (all.size() < head + 16 || all.compare(0, body.size(), body) || all.compare(body.size(), 8, "SSBLATE\n"))
            return "?";
        uint32_t len, tail[2];
        memcpy(&len, all.data() + body.size() + 8, 4);
        if (all.size() != head + len + 16 || all.compare(head + len, 8, "SSBTEND\n")) return "?";
        memcpy(tail, all.data() + head + len + 8, 8);
        return tail[0] == 7 && tail[1] == 0 ? all.substr(head, len) : "?";
    };
    auto cleanup = [&]() { DeleteFileA(pending.c_str()); DeleteFileA(complete.c_str()); };
    const std::string other(64, '0');
    for (unsigned mode = 0; mode != 4; ++mode) {
        traceprovenance::OpenedFiles files;
        files.Note("data/a", sha, 99);   // before the trace: header only
        check(files.Snapshot() == "file " + sha + " data/a\n", "header_has_preexisting_identity");
        files.Note(mode == 1 ? "data/b" : "data/a", mode == 2 ? other : mode == 3 ? "unreadable" : sha, 5);
        files.Note("data/a", sha, 6);    // a repeat of the header identity is not late
        const uint32_t status = publish(files);
        const std::string want[] = {"", "file " + sha + " 5 data/b\n", "file " + other + " 5 data/a\n",
                                    "file unreadable 5 data/a\n"};
        check(status == 0, "late_identity_publishes");
        check(published_late() == want[mode], "late_section_names_each_new_identity_once_with_its_record");
        check(GetFileAttributesA(pending.c_str()) == INVALID_FILE_ATTRIBUTES, "pending_renamed");
        cleanup();
        // The next trace's header carries it; its own late section starts empty.
        const std::string in_next[] = {"file " + sha + " data/a\n", "file " + sha + " data/b\n",
                                       "file " + other + " data/a\n", "file unreadable data/a\n"};
        check(files.Snapshot().find(in_next[mode]) != std::string::npos, "late_identity_in_next_header");
        check(publish(files) == 0 && published_late().empty(), "next_late_section_empty");
        cleanup();
    }

    // A late section that cannot be written fails the trace (never published).
    {
        traceprovenance::OpenedFiles files;
        files.Snapshot();
        files.Note("data/late", sha, 1);
        FILE* f = provisional("wb");
        if (f) fclose(f);
        f = provisional("rb");   // writes fail on a read-only stream
        check(f != nullptr, "open_read_only_pending");
        const uint32_t status = f ? files.CloseAndPublish(f, 0, 7, pending.c_str(), complete.c_str()) : 0;
        check((status & 1) != 0, "late_section_write_failure_sets_status");
        check(GetFileAttributesA(complete.c_str()) == INVALID_FILE_ATTRIBUTES, "failed_late_section_not_published");
        check(GetFileAttributesA(pending.c_str()) == INVALID_FILE_ATTRIBUTES, "failed_late_section_pending_removed");
        cleanup();
    }

    bool snapshot_races_safe = true;
    for (unsigned i = 0; i != 50; ++i) {
        traceprovenance::OpenedFiles files;
        std::atomic<bool> go{false};
        std::thread reader([&]() { while (!go.load()) SwitchToThread(); files.Note("data/racing", sha, 3); });
        go.store(true);
        const std::string header = files.Snapshot();
        reader.join();
        const uint32_t status = publish(files);
        const std::string late = published_late();
        // Exactly one of: in the header, in the late section.
        snapshot_races_safe &= status == 0 && header.empty() == (late == "file " + sha + " 3 data/racing\n") &&
                               (header.empty() || late.empty());
        cleanup();
    }
    check(snapshot_races_safe, "snapshot_race_identity_is_in_header_xor_late_section");

    // Hold close inside fflush. A later Note must not slip into the published
    // trace unrecorded; it completes only after publication.
    traceprovenance::OpenedFiles files;
    files.Snapshot();
    flushing = CreateEventW(nullptr, TRUE, FALSE, nullptr);
    release_flush = CreateEventW(nullptr, TRUE, FALSE, nullptr);
    std::atomic<bool> noting{false}, noted{false};
    uint32_t status = 1;
    FILE* f = provisional();
    check(f && flushing && release_flush, "publication_race_resources");
    if (f && flushing && release_flush) {
        std::thread closer([&]() { status = files.CloseAndPublish(f, 0, 7, pending.c_str(), complete.c_str()); });
        WaitForSingleObject(flushing, INFINITE);
        std::thread reader([&]() { noting.store(true); files.Note("data/after-close", sha, 8); noted.store(true); });
        while (!noting.load()) SwitchToThread();
        Sleep(20);
        check(!noted.load(), "open_observation_cannot_return_during_publication");
        SetEvent(release_flush);
        closer.join(); reader.join();
        check(status == 0 && published_late().empty(), "published_trace_has_no_post_publication_identity");
        check(files.Snapshot() == "file " + sha + " data/after-close\n",
              "post_publication_observation_belongs_to_next_snapshot");
    } else if (f) fclose(f);
    if (flushing) CloseHandle(flushing);
    if (release_flush) CloseHandle(release_flush);
    flushing = release_flush = nullptr;
    cleanup();
    return FinishTests();
}
