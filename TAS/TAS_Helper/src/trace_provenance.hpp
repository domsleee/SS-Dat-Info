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

// The header is fixed once PLAY starts. Any new identity until publication
// invalidates that trace; a repeat read of identical content is harmless.
class OpenedFiles {
    std::mutex mutex;
    std::map<std::string, std::set<std::string>> files;
    uint64_t generation = 0, snapshot = 0;
public:
    void Note(const std::string& path, const std::string& sha) {
        const std::lock_guard<std::mutex> guard(mutex);
        if (files[path].insert(sha).second) ++generation;
    }

    std::string Snapshot() {
        const std::lock_guard<std::mutex> guard(mutex);
        std::string out;
        for (const auto& [path, shas] : files)
            for (const auto& sha : shas) out += "file " + sha + " " + path + "\n";
        snapshot = generation;
        return out;
    }

    uint32_t CloseAndPublish(FILE* file, uint32_t status, const char* pending, const char* complete) {
        // Serialize the final check AND publication with Note. An open whose
        // observation waits here cannot return to game code until we finish.
        const std::lock_guard<std::mutex> guard(mutex);
        if (generation != snapshot) status |= 32;
        return tracefile::CloseAndPublish(file, status, pending, complete);
    }
};

} // namespace traceprovenance
