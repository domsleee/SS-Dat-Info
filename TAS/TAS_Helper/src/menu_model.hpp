#pragma once
// The MENU MODEL - pure logic shared by the DLL (caves/menu_state.hpp) and the
// unit tests (tests/test_menu_model.cpp): the item snapshot, the JSON document
// an agent reads, and the target matching a command uses. No Windows headers;
// no game pointer is dereferenced here (menu_state.hpp fills the snapshot).
#include <cstdint>
#include <cstring>
#include <string>

namespace menumodel {

constexpr uint32_t kMaxItems = 24;
constexpr uint32_t kNameMax = 32;
constexpr uint32_t kLabelMax = 40;

struct MenuItem {
    uint32_t comp = 0;              // the UI_Component - valid for THIS visit of the page only
    char name[kNameMax] = {};       // UI_Component name (+0x10): the stable id; may be empty
    char label[kLabelMax] = {};     // the button's text line text (what the screen shows)
    uint8_t enabled = 0, visible = 0, focused = 0;
};

struct MenuSnapshot {
    uint32_t selector = 0xFFFFFFFFu;  // index into items of the focused one; none = 0xFFFFFFFF
    uint32_t count = 0;
    uint32_t container = 0;           // the items' parent UI_Container (diagnostics)
    MenuItem items[kMaxItems];
};

inline bool EqualsIgnoreCase(const char* a, const char* b) {
    for (;; a++, b++) {
        const unsigned char ca = (unsigned char)*a, cb = (unsigned char)*b;
        const unsigned char fa = (ca >= 'A' && ca <= 'Z') ? (unsigned char)(ca + 32) : ca;
        const unsigned char fb = (cb >= 'A' && cb <= 'Z') ? (unsigned char)(cb + 32) : cb;
        if (fa != fb) return false;
        if (!ca) return true;
    }
}

// Labels and names are printable ASCII (the reader enforces it), so only the
// two JSON metacharacters need escaping.
inline void AppendJsonString(std::string& out, const char* s) {
    out += '"';
    for (; *s; s++) {
        if (*s == '"' || *s == '\\') out += '\\';
        out += *s;
    }
    out += '"';
}

// {"screen":..,"sel":N|null,"items":[{"label":..,"id":..,"en":b,"vis":b},..]}
// An empty screen name yields an empty string: no menu, no document.
inline std::string BuildDoc(const MenuSnapshot& s, const char* screen) {
    if (!screen || !screen[0]) return {};
    std::string d;
    d.reserve(512);
    d += "{\"screen\":";
    AppendJsonString(d, screen);
    d += ",\"sel\":";
    d += (s.selector == 0xFFFFFFFFu) ? std::string("null") : std::to_string(s.selector);
    d += ",\"items\":[";
    for (uint32_t i = 0; i < s.count; i++) {
        const auto& it = s.items[i];
        if (i) d += ',';
        d += "{\"label\":";
        AppendJsonString(d, it.label);
        d += ",\"id\":";
        AppendJsonString(d, it.name);
        d += ",\"en\":";
        d += it.enabled ? "true" : "false";
        d += ",\"vis\":";
        d += it.visible ? "true" : "false";
        d += '}';
    }
    d += "]}";
    return d;
}

// Which item a command names. The stable id wins (case-insensitively); then
// the visible label, case-insensitively. A target shaped like an id ("ID_...")
// never falls back to a label, an empty target matches nothing (some buttons
// have no id), and there is no prefix matching. Returns the index or -1.
inline int FindTarget(const MenuSnapshot& s, const char* target) {
    if (!target || !target[0]) return -1;
    for (uint32_t i = 0; i < s.count; i++)
        if (s.items[i].name[0] && EqualsIgnoreCase(s.items[i].name, target)) return (int)i;
    if ((target[0] == 'I' || target[0] == 'i') && (target[1] == 'D' || target[1] == 'd') && target[2] == '_') return -1;
    for (uint32_t i = 0; i < s.count; i++)
        if (s.items[i].label[0] && EqualsIgnoreCase(s.items[i].label, target)) return (int)i;
    return -1;
}

}  // namespace menumodel
