#pragma once
#include "trace_file.hpp"
#include <bcrypt.h>
#include <map>
#include <mutex>
#include <set>
#include <string>
#include <vector>
#pragma comment(lib, "bcrypt.lib")

namespace traceprovenance {

inline std::string Sha256Bytes(const void* data, size_t n) {
    uint8_t h[32];
    if (!BCRYPT_SUCCESS(BCryptHash(BCRYPT_SHA256_ALG_HANDLE, nullptr, 0, (PUCHAR)data, (ULONG)n, h, sizeof h)))
        return "unhashable";
    constexpr char hex[] = "0123456789abcdef";
    std::string out;
    for (uint8_t b : h) { out += hex[b >> 4]; out += hex[b & 15]; }
    return out;
}

inline std::string Sha256File(const std::wstring& path) {
    HANDLE f = CreateFileW(path.c_str(), GENERIC_READ, FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                           nullptr, OPEN_EXISTING, 0, nullptr);
    if (f == INVALID_HANDLE_VALUE) return "unreadable";
    std::vector<uint8_t> data;
    uint8_t buf[65536];
    DWORD n = 0;
    BOOL read;
    while ((read = ReadFile(f, buf, sizeof buf, &n, nullptr)) && n)
        data.insert(data.end(), buf, buf + n);
    CloseHandle(f);
    // A failed read must not masquerade as a complete hash of the prefix.
    return read ? Sha256Bytes(data.data(), data.size()) : "unreadable";
}

// Every identity (path, content) opened since injection. The header lists them
// all as of PLAY. Each identity first seen after that goes into the trace's
// late section with the record index it was first observed at. The section is
// written, closed and published under the same lock as Note, so an identity is
// either in this trace or observed after its publication (then in the next
// header). A repeat read of identical content is not a new identity.
class OpenedFiles {
    std::mutex mutex;
    std::map<std::string, std::set<std::string>> files;
    std::string late;   // "file <sha> <at> <path>\n" per identity since Snapshot
public:
    // True when this is a new identity (a new path, or new content for a path).
    // `at`: the open trace's record index (Snapshot discards earlier lines).
    bool Note(const std::string& path, const std::string& sha, uint32_t at) {
        const std::lock_guard<std::mutex> guard(mutex);
        if (!files[path].insert(sha).second) return false;
        late += "file " + sha + " " + std::to_string(at) + " " + path + "\n";
        return true;
    }

    std::string Snapshot() {
        const std::lock_guard<std::mutex> guard(mutex);
        std::string out;
        for (const auto& [path, shas] : files)
            for (const auto& sha : shas) out += "file " + sha + " " + path + "\n";
        late.clear();
        return out;
    }

    // "SSBLATE\n", u32 length, the late lines; the 16-byte trailer ("SSBTEND\n",
    // u32 records, u32 status); then checked close and publication.
    uint32_t CloseAndPublish(FILE* file, uint32_t status, uint32_t records, const char* pending,
                             const char* complete) {
        // An open whose observation waits here cannot return to game code
        // until the trace is published.
        const std::lock_guard<std::mutex> guard(mutex);
        const uint32_t n = (uint32_t)late.size();
        if (fwrite("SSBLATE\n", 1, 8, file) != 8 || fwrite(&n, 4, 1, file) != 1 ||
            fwrite(late.data(), 1, n, file) != n)
            status |= 1;
        const uint32_t tail[2] = {records, status};
        if (fwrite("SSBTEND\n", 1, 8, file) != 8 || fwrite(tail, 4, 2, file) != 2) status |= 1;
        late.clear();
        return tracefile::CloseAndPublish(file, status, pending, complete);
    }
};

} // namespace traceprovenance
