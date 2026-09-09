#pragma once
// The MENU MODEL - pure logic shared by the DLL (caves/menu_state.hpp) and the
// unit tests (tests/test_menu_model.cpp): the item snapshot, the JSON document
// an agent reads, and the target matching a command uses. No Windows headers
// and no allocation (the DLL builds the document inside a hook); no game
// pointer is dereferenced here (menu_state.hpp fills the snapshot).
#include <cstdint>

namespace menumodel {

constexpr uint32_t kMaxItems = 24;
constexpr uint32_t kNameMax = 32;
constexpr uint32_t kLabelMax = 40;

template <typename GetModal>
bool UnobscuredMenu(uint32_t page, uint32_t container, GetModal get_modal) {
    return page >= 0x10000 && container >= 0x10000 &&
           !get_modal(page) && (page == container || !get_modal(container));
}

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

// Bounded, allocation-free text writer: appends past the end are dropped and
// flagged; Finish() NUL-terminates.
struct TextWriter {
    char* out;
    uint32_t cap;
    uint32_t len = 0;
    bool overflow = false;

    void Put(char c) {
        if (len + 1 < cap) out[len++] = c;
        else overflow = true;
    }
    void Put(const char* s) {
        for (; *s; s++) Put(*s);
    }
    void PutU32(uint32_t v) {
        char digits[10];
        int n = 0;
        do {
            digits[n++] = (char)('0' + v % 10);
            v /= 10;
        } while (v);
        while (n) Put(digits[--n]);
    }
    // Labels and names are printable ASCII (the reader enforces it), so only
    // the two JSON metacharacters need escaping.
    void PutJson(const char* s) {
        Put('"');
        for (; *s; s++) {
            if (*s == '"' || *s == '\\') Put('\\');
            Put(*s);
        }
        Put('"');
    }
    const char* Finish() {
        if (cap) out[len] = 0;
        return out;
    }
};

// {"screen":..,"sel":N|null,"items":[{"label":..,"id":..,"en":b,"vis":b},..]}
// written into out[cap]. Returns the length: 0 (and an empty out) when there
// is no menu (empty screen) or the document does not fit - never a cut-off
// document.
inline uint32_t BuildDoc(const MenuSnapshot& s, const char* screen, char* out, uint32_t cap) {
    if (cap == 0) return 0;
    out[0] = 0;
    if (!screen || !screen[0]) return 0;
    TextWriter w{out, cap};
    w.Put("{\"screen\":");
    w.PutJson(screen);
    w.Put(",\"sel\":");
    if (s.selector == 0xFFFFFFFFu) w.Put("null");
    else w.PutU32(s.selector);
    w.Put(",\"items\":[");
    for (uint32_t i = 0; i < s.count; i++) {
        const auto& it = s.items[i];
        if (i) w.Put(',');
        w.Put("{\"label\":");
        w.PutJson(it.label);
        w.Put(",\"id\":");
        w.PutJson(it.name);
        w.Put(",\"en\":");
        w.Put(it.enabled ? "true" : "false");
        w.Put(",\"vis\":");
        w.Put(it.visible ? "true" : "false");
        w.Put('}');
    }
    w.Put("]}");
    if (w.overflow) {
        out[0] = 0;
        return 0;
    }
    w.Finish();
    return w.len;
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
