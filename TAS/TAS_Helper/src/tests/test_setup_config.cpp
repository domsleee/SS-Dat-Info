// Unit tests for the exact game-config pointer chain used by level and stance
// detection. Pure logic, no live process or Windows APIs.

#include "../setup_config_parse.hpp"
#include "../level_path_parse.hpp"
#include <cstdio>
#include <cstring>
#include <string>
#include <unordered_map>

static int g_failures = 0;

static void check(bool condition, const char* name) {
    std::printf("  %-4s %s\n", condition ? "ok" : "FAIL", name);
    if (!condition) g_failures++;
}

struct Memory {
    std::unordered_map<uint32_t, unsigned char> bytes;

    void Put(uint32_t address, const void* source, uint32_t size) {
        const auto* source_bytes = static_cast<const unsigned char*>(source);
        for (uint32_t i = 0; i < size; i++) bytes[address + i] = source_bytes[i];
    }

    void PutU32(uint32_t address, uint32_t value) { Put(address, &value, sizeof value); }

    void PutString(uint32_t object, uint32_t data, const char* value) {
        const uint32_t length = static_cast<uint32_t>(std::strlen(value));
        const uint32_t header[4] = {0, data, length, 31};
        Put(object, header, sizeof header);
        Put(data, value, length);
    }

    bool Read(uint32_t address, void* destination, uint32_t size) const {
        auto* destination_bytes = static_cast<unsigned char*>(destination);
        for (uint32_t i = 0; i < size; i++) {
            const uint64_t current = static_cast<uint64_t>(address) + i;
            if (current > UINT32_MAX) return false;
            auto found = bytes.find(static_cast<uint32_t>(current));
            if (found == bytes.end()) return false;
            destination_bytes[i] = found->second;
        }
        return true;
    }
};

struct Fixture {
    static constexpr uint32_t EXE = 0x00400000;
    static constexpr uint32_t STATE = 0x02000000;
    static constexpr uint32_t CONFIG = 0x02100000;

    Memory memory;

    Fixture(const char* area, const char* difficulty, uint32_t stance,
            const char* character = "Keith", const char* controller = "Keyboard") {
        using namespace setupconfig;
        memory.PutU32(EXE + MAIN_STATE_PTR_RVA, STATE);
        memory.PutU32(STATE + CONFIG_PTR_OFFSET, CONFIG);
        memory.PutString(CONFIG + AREA_STRING, 0x02200000, area);
        memory.PutString(CONFIG + DIFFICULTY_STRING, 0x02200100, difficulty);
        memory.PutString(CONFIG + CHARACTER_STRING, 0x02200200, character);
        memory.PutString(CONFIG + CONTROLLER_STRING, 0x02200300, controller);
        memory.PutU32(CONFIG + STANCE, stance);
    }

    bool Read(setupconfig::Values* values, uint32_t* address = nullptr) const {
        auto reader = [this](uint32_t source, void* destination, uint32_t size) {
            return memory.Read(source, destination, size);
        };
        return setupconfig::Read(EXE, reader, values, address);
    }
};

static int DetectedLevel(const Fixture& fixture, int path_area) {
    setupconfig::Values values;
    if (!fixture.Read(&values)) return -1;
    return levelpath::LevelIdFrom(path_area, values.area, values.difficulty);
}

int main() {
    using namespace setupconfig;
    std::printf("setup_config tests:\n");

    {
        Fixture fixture("Forest", "Easy", 0);
        Values values;
        uint32_t address = 0;
        check(fixture.Read(&values, &address), "exact EXE -> state -> config chain resolves");
        check(address == Fixture::CONFIG, "reader reports the config object selected by the chain");
        check(values.valid && std::strcmp(values.area, "Forest") == 0 &&
                  std::strcmp(values.difficulty, "Easy") == 0,
              "area and difficulty come from their exact config offsets");
        check(DetectedLevel(fixture, 0) == 0, "Forest Easy resolves end-to-end to level 0");
        check(values.stance == 0, "regular stance is read from config +0x140");
    }

    {
        Fixture fixture("Forest", "Medium", 1);
        Values values;
        check(DetectedLevel(fixture, 0) == 1, "Forest Medium resolves end-to-end to level 1");
        check(fixture.Read(&values) && values.stance == 1,
              "goofy stance is read from config +0x140");
    }

    {
        // Village Hard intentionally uses Village's easy shadow path. The path
        // supplies area 2; the config must supply Hard to avoid mislabelling it.
        Fixture fixture("Village", "Hard", 0);
        check(DetectedLevel(fixture, 2) == 8,
              "Village Hard resolves to level 8 even when its asset path says easy");
    }

    {
        Fixture fixture("Forest", "Easy", 2);
        Values values;
        check(fixture.Read(&values) && values.valid && values.stance == STANCE_UNKNOWN,
              "invalid stance becomes unknown without breaking level detection");
        check(DetectedLevel(fixture, 0) == 0,
              "a bad stance cannot make an otherwise valid level unresolved");
    }

    {
        Fixture fixture("Forest", "Easy", 0);
        fixture.memory.PutU32(Fixture::STATE + CONFIG_PTR_OFFSET, 0);
        Values values;
        check(!fixture.Read(&values), "missing config pointer is unresolved despite valid decoy heap data");
    }

    {
        Fixture fixture("Forest", "Easy", 0, "Keith", "NotAController");
        Values values;
        check(!fixture.Read(&values), "unknown controller rejects a coincidental string layout");
    }

    {
        Fixture fixture("Forest", "Easy", 0);
        const uint32_t torn_header[4] = {0, 0x02200000, 0xFFFFFFFFu, 0xFFFFFFFFu};
        fixture.memory.Put(Fixture::CONFIG + AREA_STRING, torn_header, sizeof torn_header);
        Values values;
        check(!fixture.Read(&values), "torn string length is rejected before copying");
    }

    {
        Memory memory;
        Values values;
        auto reader = [&memory](uint32_t source, void* destination, uint32_t size) {
            return memory.Read(source, destination, size);
        };
        check(!setupconfig::Read(0xFFF80000u, reader, &values),
              "EXE base plus global RVA overflow is rejected");
    }

    if (g_failures == 0) {
        std::printf("ALL PASS\n");
        return 0;
    }
    std::printf("%d FAILED\n", g_failures);
    return 1;
}
