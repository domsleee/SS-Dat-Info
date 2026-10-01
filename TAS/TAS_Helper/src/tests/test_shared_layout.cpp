// The C++ half of the cross-language layout check: builds the wire manifest
// from shared_layout.hpp's pins plus the shared constants, and compares it
// with tas_shared/shared_layout.txt, which tas_shared's layout test checks the
// Rust side against. A mismatch (or --print) prints this side's manifest.

#include "../shared_state.hpp"
#include "check.hpp"
#include <cstring>
#include <fstream>
#include <string>
#include <vector>

// Wire type names, in the manifest's Rust-like spelling.
template <class T> struct Wire;
template <> struct Wire<uint8_t> { static std::string Name() { return "u8"; } };
template <> struct Wire<char> { static std::string Name() { return "u8"; } };
template <> struct Wire<uint32_t> { static std::string Name() { return "u32"; } };
template <> struct Wire<float> { static std::string Name() { return "f32"; } };
template <> struct Wire<TasSegmentBoundary> { static std::string Name() { return "TasSegmentBoundary"; } };
template <> struct Wire<TasLogEntry> { static std::string Name() { return "TasLogEntry"; } };
template <class T, size_t N> struct Wire<T[N]> {
    static std::string Name() { return "[" + Wire<T>::Name() + ";" + std::to_string(N) + "]"; }
};

// Values both sides define; the manifest names them by their C++ names.
#define TAS_LAYOUT_CONSTS(X) \
    X(TAS_SHARED_VERSION) X(TAS_MENU_DOC_MAX) X(TAS_MENU_CMD_TARGET_MAX) X(TAS_CRASH_MODULE_MAX) \
    X(TAS_LEVEL_PATH_MAX) X(TAS_MENU_SCREEN_MAX) X(TAS_MAX_TICKS) X(TAS_MAX_SEGMENTS) \
    X(TAS_LOG_RING_SIZE) X(TAS_LOG_ENTRY_SIZE) \
    X(CMD_IDLE) X(CMD_ARM_REC) X(CMD_ARM_PLAY) X(CMD_STOP) X(CMD_ARM_CONTINUE) X(CMD_RESTART) \
    X(CMD_STOP_FOR_RESTART) X(CMD_TEST_FAULT) \
    X(MODE_OFF) X(MODE_REC) X(MODE_PLAY) \
    X(INPUT_LEFT) X(INPUT_RIGHT) X(INPUT_UP) X(INPUT_DOWN) X(INPUT_JUMP) X(INPUT_SHIFT) \
    X(TAS_INPUT_MODEL_INJECTED) X(TAS_INPUT_MODEL_HELD) \
    X(TAS_RACE_CLOCK_STARTED) X(TAS_RACE_CLOCK_FINISHED) \
    X(ARG4_SOURCE_TIME_CURRENT) \
    X(LOG_DEBUG) X(LOG_INFO) X(LOG_WARN) X(LOG_ERROR) \
    X(TAS_MENU_CMD_ACTIVATE) X(TAS_MENU_CMD_FOCUS) X(TAS_MENU_CMD_UP) X(TAS_MENU_CMD_DOWN) \
    X(TAS_MENU_CMD_LEFT) X(TAS_MENU_CMD_RIGHT) X(TAS_MENU_CMD_TRIGGER) \
    X(TAS_MENU_RESULT_OK) X(TAS_MENU_RESULT_NO_MENU) X(TAS_MENU_RESULT_NOT_FOUND) \
    X(TAS_MENU_RESULT_DISABLED) X(TAS_MENU_RESULT_BAD_KIND) X(TAS_MENU_RESULT_FAULT) \
    X(TAS_MENU_RESULT_NOT_FOCUSABLE) X(TAS_MENU_RESULT_STALE_PAGE) X(TAS_MENU_RESULT_EXPIRED) \
    X(TAS_OWNER_ACQUIRE) X(TAS_OWNER_RELEASE) \
    X(TAS_OWNER_RESULT_OWNED) X(TAS_OWNER_RESULT_RELEASED) X(TAS_OWNER_RESULT_BUSY) \
    X(TAS_OWNER_RESULT_NO_PROCESS) X(TAS_OWNER_RESULT_WRONG_PROCESS) X(TAS_OWNER_RESULT_NOT_OWNER) \
    X(TAS_OWNER_RESULT_BAD_KIND) \
    X(TAS_GAME_CALL_NONE) X(TAS_GAME_CALL_MENU_TRIGGER) X(TAS_GAME_CALL_MENU_MOVE) \
    X(TAS_GAME_CALL_MENU_FOCUS) X(TAS_GAME_CALL_MENU_ACTIVE) X(TAS_GAME_CALL_TIME_CURRENT) \
    X(TAS_GAME_CALL_TEST_FAULT) X(TAS_GAME_CALL_OBSERVER_FLUSH) \
    X(TAS_RENDERER_UNKNOWN) X(TAS_RENDERER_DIRECTX6) X(TAS_RENDERER_DIRECTX7) X(TAS_RENDERER_OPENGL) \
    X(TAS_RENDERER_GLIDE3X) X(TAS_RENDERER_SOFTWARE2) \
    X(TAS_CHARACTER_UNKNOWN) X(TAS_CHARACTER_KEITH) X(TAS_CHARACTER_VINCENT) X(TAS_CHARACTER_AKIKO) \
    X(TAS_CHARACTER_KARL) X(TAS_CHARACTER_MIKE) X(TAS_CHARACTER_ULRIKA) X(TAS_CHARACTER_OTHER)

static std::vector<std::string> Manifest() {
    std::vector<std::string> m;
#define STRUCT_LINE(S, size, align) \
    m.push_back("struct " #S " " + std::to_string(sizeof(S)) + " " + std::to_string(alignof(S)));
#define FIELD_LINE(S, f, T, off) \
    m.push_back("field " #S "." #f " " + Wire<T>::Name() + " " + std::to_string(offsetof(S, f)) + " " + \
                std::to_string(sizeof(T)) + " " + std::to_string(alignof(T)));
#define CONST_LINE(name) m.push_back("const " #name " " + std::to_string((uint64_t)(name)));
    TAS_LAYOUT_STRUCTS(STRUCT_LINE)
    TAS_LAYOUT_FIELDS(FIELD_LINE)
    m.push_back(std::string("const TAS_SHARED_MEMORY_NAME ") + TAS_SHARED_MEMORY_NAME);
    TAS_LAYOUT_CONSTS(CONST_LINE)
#undef STRUCT_LINE
#undef FIELD_LINE
#undef CONST_LINE
    return m;
}

// shared_layout.txt without comments and blank lines. The suite is compiled
// from its full path, so __FILE__ locates the repo.
static std::vector<std::string> CheckedIn(bool* found) {
    std::string path = __FILE__;
    path = path.substr(0, path.find_last_of("\\/") + 1) + "../../../tas_shared/shared_layout.txt";
    std::ifstream in(path);
    *found = in.is_open();
    std::vector<std::string> lines;
    for (std::string line; std::getline(in, line);) {
        while (!line.empty() && (line.back() == '\r' || line.back() == ' ')) line.pop_back();
        if (!line.empty() && line[0] != '#') lines.push_back(line);
    }
    return lines;
}

int main(int argc, char** argv) {
    std::printf("shared_layout tests:\n");
    std::vector<std::string> ours = Manifest();
    bool found = false;
    std::vector<std::string> file = CheckedIn(&found);
    check(found, "shared_layout_txt_found");

    size_t i = 0;
    while (i < ours.size() && i < file.size() && ours[i] == file[i]) i++;
    bool same = found && ours.size() == file.size() && i == ours.size();
    check(same, "manifest_matches_shared_layout_txt");
    if (found && !same) {
        std::printf("  first difference at entry %zu:\n    C++:  %s\n    file: %s\n", i,
                    i < ours.size() ? ours[i].c_str() : "<end>", i < file.size() ? file[i].c_str() : "<end>");
    }
    if (!same || (argc > 1 && std::strcmp(argv[1], "--print") == 0)) {
        for (const std::string& line : ours) std::printf("%s\n", line.c_str());
    }
    return FinishTests();
}
