#pragma once
#include <windows.h>
#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include "safe_read.hpp"
#include "trace_file.hpp"

// Pointer census for the trace (ssb-recomp re/harness.md "Pointer census").
// Diagnostic only, on only with ssb_census.txt next to TAS_Helper.dll. On
// selected ticks every 4-aligned dword of every captured region that points at
// committed, readable memory gets a `ptr` line in <trace stem>.census: what it
// points at (module + RVA + section, MSVC vtable; msvcrt/process heap block
// base and size; other private memory; mapped; the game thread's stack) and
// the MSVC RTTI class of the object there. Reads never fault (SafeCopy).
// C++17: the standalone tests build this header (tests/test_census.cpp).
namespace census {

// ---- address chains: base[>off]...[+off], the region syntax -----------------
struct Chain {
    std::string base;
    std::vector<uint32_t> derefs;
    uint32_t add = 0;
};

// False when a >off follows a nonzero +off (the +off comes last).
inline bool ParseChain(const std::string& c, Chain* out) {
    size_t i = c.find_first_of(">+");
    out->base = c.substr(0, i);
    out->derefs.clear();
    out->add = 0;
    while (i != std::string::npos) {
        const char op = c[i];
        const size_t j = c.find_first_of(">+", i + 1);
        const uint32_t v =
            (uint32_t)strtoul(c.substr(i + 1, j == std::string::npos ? j : j - i - 1).c_str(), nullptr, 0);
        if (op == '>') {
            if (out->add) return false;
            out->derefs.push_back(v);
        } else {
            out->add = v;
        }
        i = j;
    }
    return true;
}

// The chain from `base` (0 when any pointer on the way is null or unreadable).
inline uint32_t Resolve(const Chain& c, uint32_t base) {
    uint32_t p = base;
    for (uint32_t d : c.derefs) p = p ? SafeRead32(p + d) : 0;
    return p ? p + c.add : 0;
}

// ---- options: ssb_census.txt --------------------------------------------------
struct TickRange {
    int32_t lo, hi;
};

inline bool ParseRanges(const std::string& s, std::vector<TickRange>* out) {
    size_t i = 0;
    while (i < s.size()) {
        size_t j = s.find(',', i);
        if (j == std::string::npos) j = s.size();
        const std::string item = s.substr(i, j - i);
        char* end = nullptr;
        const long lo = strtol(item.c_str(), &end, 0);
        long hi = lo;
        if (end == item.c_str()) return false;
        if (*end == '-') {
            const char* h = end + 1;
            hi = strtol(h, &end, 0);
            if (end == h) return false;
        }
        if (*end || hi < lo) return false;
        out->push_back({(int32_t)lo, (int32_t)hi});
        i = j + 1;
    }
    return !out->empty();
}

inline bool InRanges(const std::vector<TickRange>& r, int32_t t) {
    for (const TickRange& x : r)
        if (t >= x.lo && t <= x.hi) return true;
    return false;
}

// A `guard` (page-guarded range) or `hw` (data breakpoint, 4 bytes) target;
// chain base `body` (the human Body at its physics step entry), `player` or a
// module name.
struct Spec {
    std::string text;
    Chain chain;
    uint32_t len = 4;
};

struct Options {
    uint32_t every = 500;            // census every Nth record (0: only the fixed samples)
    bool readset = false;            // page-guard read-set windows
    uint32_t rs_first = 3;           // the first N physics entries
    uint32_t rs_every = 250;         // then every Nth entry (0: none)
    std::vector<TickRange> rs_ticks; // plus these rec_ticks
    uint32_t max_exceptions = 100000;  // per window; above it the window bails out
    std::vector<Spec> guards;
    std::vector<Spec> hw;
    std::vector<TickRange> hw_ticks;
    std::vector<TickRange> probe_ticks;  // contact probe at SG+0x9972E (trace.hpp)
    std::string summary;             // the accepted lines, `;`-joined, for headers
};

// Lines: `every N`, `readset`, `readset_first N`, `readset_every N`,
// `readset_ticks A-B,C`, `max_exceptions N`, `guard CHAIN LEN`, `hw CHAIN`
// (at most 4), `hw_ticks A-B,C`, `probe_ticks A-B,C`. Any readset_* or guard
// line turns the read-set on. False with *err set on anything else.
inline bool ParseOptions(const std::string& text, Options* o, std::string* err) {
    *o = Options{};
    size_t i = 0;
    while (i < text.size()) {
        size_t j = text.find('\n', i);
        if (j == std::string::npos) j = text.size();
        std::string line = text.substr(i, j - i);
        i = j + 1;
        const size_t hash = line.find('#');
        if (hash != std::string::npos) line.resize(hash);
        char key[32] = {}, a[256] = {}, b[64] = {};
        const int n = sscanf_s(line.c_str(), "%31s %255s %63s", key, (unsigned)sizeof key, a, (unsigned)sizeof a, b,
                               (unsigned)sizeof b);
        if (n <= 0) continue;
        const std::string k = key;
        char* end = nullptr;
        const uint32_t v = n >= 2 ? (uint32_t)strtoul(a, &end, 0) : 0;
        const bool num = n == 2 && end && !*end;
        bool ok = true;
        if (k == "every" && num) o->every = v;
        else if (k == "readset" && n == 1) o->readset = true;
        else if (k == "readset_first" && num) o->readset = true, o->rs_first = v;
        else if (k == "readset_every" && num) o->readset = true, o->rs_every = v;
        else if (k == "readset_ticks" && n == 2) o->readset = true, ok = ParseRanges(a, &o->rs_ticks);
        else if (k == "max_exceptions" && num && v) o->max_exceptions = v;
        else if (k == "hw_ticks" && n == 2) ok = ParseRanges(a, &o->hw_ticks);
        else if (k == "probe_ticks" && n == 2) ok = ParseRanges(a, &o->probe_ticks);
        else if ((k == "guard" && n == 3) || (k == "hw" && n == 2)) {
            Spec s;
            s.text = a;
            ok = ParseChain(a, &s.chain) && !s.chain.base.empty();
            if (k == "guard") {
                s.len = (uint32_t)strtoul(b, &end, 0);
                ok = ok && !*end && s.len;
                o->readset = true;
                o->guards.push_back(s);
            } else {
                ok = ok && (s.chain.add & 3) == 0 && o->hw.size() < 4;
                o->hw.push_back(s);
            }
        } else ok = false;
        if (!ok) {
            *err = "bad line: " + line;
            return false;
        }
        const size_t a0 = line.find_first_not_of(" \t"), a1 = line.find_last_not_of(" \t\r");
        if (!o->summary.empty()) o->summary += ';';
        o->summary += line.substr(a0, a1 - a0 + 1);
    }
    if (o->hw.empty() != o->hw_ticks.empty()) {
        *err = "hw and hw_ticks go together";
        return false;
    }
    return true;
}

// Census samples: record 0 (the arm's first PLAY tick), every Nth record, and
// the first tick whose human Body stepped (`firstEntry`: its ENTRY and POST
// points; its PRE ran before the step, so the next record's PRE).
inline bool CensusAt(const Options& o, uint32_t record, uint32_t firstEntry, bool pre) {
    if (record == 0 || (o.every && record % o.every == 0)) return true;
    if (firstEntry == UINT32_MAX) return false;
    return pre ? record == firstEntry + 1 : record == firstEntry;
}

// Read-set windows, by physics entry index (0 = the first tick the human Body
// stepped) and rec_tick.
// Anything that writes <stem>.readset.
inline bool WantsReadset(const Options& o) { return o.readset || !o.hw.empty() || !o.probe_ticks.empty(); }

inline bool ReadsetAt(const Options& o, uint32_t entry, int32_t recTick) {
    return o.readset && (entry < o.rs_first || (o.rs_every && entry % o.rs_every == 0) || InRanges(o.rs_ticks, recTick));
}

// ---- MSVC RTTI ------------------------------------------------------------------
// ".?AVBody@Supreme_Snowboarding@Housemarque@@" -> "Housemarque::Supreme_Snowboarding::Body".
// Template names (with '?') stay mangled after the prefix.
inline std::string Demangle(const char* n) {
    if (strncmp(n, ".?AV", 4) && strncmp(n, ".?AU", 4)) return "";
    std::string s = n + 4;
    if (s.size() >= 2 && s.compare(s.size() - 2, 2, "@@") == 0) s.resize(s.size() - 2);
    if (s.empty()) return "";
    if (s.find('?') != std::string::npos) return s;
    std::string out;
    size_t end = s.size();
    while (true) {
        const size_t at = s.rfind('@', end - 1);
        const size_t from = at == std::string::npos ? 0 : at + 1;
        if (!out.empty()) out += "::";
        out += s.substr(from, end - from);
        if (at == std::string::npos || at == 0) break;
        end = at;
    }
    return out;
}

// M: Read(addr, dst, n), ReadOnlyImage(addr), Code(addr). A complete object
// locator (x86): signature 0, offset, cdOffset, TypeDescriptor*, hierarchy*.
// The TypeDescriptor (vftable*, spare, name) lives in writable .data.
template <class M>
std::string ColClass(const M& m, uint32_t col) {
    uint32_t c[5];
    if (!m.ReadOnlyImage(col) || !m.Read(col, c, sizeof c) || c[0] != 0) return "";
    char name[256];
    for (uint32_t k = 0; k < sizeof name; k++) {
        if (!m.Read(c[3] + 8 + k, &name[k], 1)) return "";
        if (!name[k]) return Demangle(name);
    }
    return "";
}

// The class of the object at `obj`: its dword 0 is a vtable in a module's
// read-only data, and the dword before the vtable is its locator.
template <class M>
std::string ObjectClass(const M& m, uint32_t obj) {
    uint32_t vt = 0, col = 0;
    if (!m.Read(obj, &vt, 4) || !m.ReadOnlyImage(vt) || !m.Read(vt - 4, &col, 4)) return "";
    return ColClass(m, col);
}

// `target` inside a vtable: walking back over code pointers reaches a slot
// whose preceding dword is a valid locator. *start = that slot 0.
template <class M>
bool VtableAt(const M& m, uint32_t target, uint32_t* start, std::string* cls) {
    uint32_t p = target & ~3u;
    for (int k = 0; k < 1024; k++, p -= 4) {
        uint32_t fn = 0, col = 0;
        if (!m.Read(p, &fn, 4) || !m.Code(fn)) return false;
        if (m.Read(p - 4, &col, 4)) {
            const std::string c = ColClass(m, col);
            if (!c.empty()) {
                *start = p;
                *cls = c;
                return true;
            }
        }
    }
    return false;
}

inline bool Query(uint32_t a, MEMORY_BASIC_INFORMATION* mi) {
    return a >= 0x10000 && VirtualQuery((const void*)(uintptr_t)a, mi, sizeof *mi) == sizeof *mi &&
           mi->State == MEM_COMMIT;
}

struct LiveMem {
    bool Read(uint32_t a, void* d, uint32_t n) const { return SafeCopy(a, d, n); }
    // The image page's protection (low byte), 0 outside images.
    static DWORD ImageProtect(uint32_t a) {
        MEMORY_BASIC_INFORMATION mi;
        return Query(a, &mi) && mi.Type == MEM_IMAGE ? mi.Protect & 0xFF : 0;
    }
    bool ReadOnlyImage(uint32_t a) const {
        const DWORD p = ImageProtect(a);
        return p == PAGE_READONLY || p == PAGE_EXECUTE_READ;
    }
    bool Code(uint32_t a) const {
        return ImageProtect(a) & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY);
    }
};

// ---- heap blocks --------------------------------------------------------------
// The game allocates through msvcrt (Supreme_Game, HMG_Kernel, HMG_HTL_2,
// HMG_3DE and sr.dll import operator new/malloc/free from MSVCRT.dll and no
// Heap*/VirtualAlloc of their own; sr.dll's srHeap is layered on those), so
// game objects are busy blocks of msvcrt's heap (_get_heap_handle). Every
// process heap is walked, so a block anywhere is found; `heap` names which.
struct Block {
    uint32_t base, size, heap;
};

struct HeapIndex {
    std::vector<Block> blocks;
    std::vector<HANDLE> heaps;
    HANDLE crt = nullptr;
    bool truncated = false;
    size_t cap = 1 << 18;

    // Under each heap's lock, into reserved storage: nothing here may
    // allocate from a heap being walked.
    void Build() {
        if (!crt) {
            using GetHeap = intptr_t(__cdecl*)();
            HMODULE m = GetModuleHandleA("msvcrt.dll");
            auto f = m ? (GetHeap)GetProcAddress(m, "_get_heap_handle") : nullptr;
            crt = f ? (HANDLE)f() : nullptr;
        }
        HANDLE hs[256];
        const DWORD n = GetProcessHeaps(256, hs);
        heaps.assign(hs, hs + (n < 256 ? n : 256));
        blocks.clear();
        if (blocks.capacity() < cap) blocks.reserve(cap);
        truncated = false;
        for (uint32_t i = 0; i < heaps.size() && !truncated; i++) {
            const BOOL locked = HeapLock(heaps[i]);
            PROCESS_HEAP_ENTRY e{};
            while (HeapWalk(heaps[i], &e)) {
                if (!(e.wFlags & PROCESS_HEAP_ENTRY_BUSY)) continue;
                if (blocks.size() == blocks.capacity()) {
                    truncated = true;
                    break;
                }
                blocks.push_back({(uint32_t)(uintptr_t)e.lpData, (uint32_t)e.cbData, i});
            }
            if (locked) HeapUnlock(heaps[i]);
        }
        if (truncated) cap *= 2;   // the next build has room
        std::sort(blocks.begin(), blocks.end(), [](const Block& a, const Block& b) { return a.base < b.base; });
    }

    const Block* Find(uint32_t a) const {
        auto it = std::upper_bound(blocks.begin(), blocks.end(), a,
                                   [](uint32_t v, const Block& b) { return v < b.base; });
        if (it == blocks.begin()) return nullptr;
        --it;
        return a - it->base < (it->size ? it->size : 1) ? &*it : nullptr;
    }

    std::string Name(uint32_t i) const {
        if (i < heaps.size() && heaps[i] == crt) return "crt";
        if (i < heaps.size() && heaps[i] == GetProcessHeap()) return "process";
        return "heap" + std::to_string(i);
    }
};

// ---- describing a target --------------------------------------------------------
inline std::string Hex(uint32_t v) {
    char b[16];
    snprintf(b, sizeof b, "0x%x", v);
    return b;
}

// "Supreme_Game.dll+0x3570", or the bare address outside every module.
inline std::string ModRva(uint32_t a) {
    HMODULE m = nullptr;
    char path[MAX_PATH];
    if (!GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                            (LPCSTR)(uintptr_t)a, &m) ||
        !GetModuleFileNameA(m, path, MAX_PATH))
        return Hex(a);
    const char* base = strrchr(path, '\\');
    return std::string(base ? base + 1 : path) + "+" + Hex(a - (uint32_t)(uintptr_t)m);
}

inline std::string SectionOf(uint32_t module, uint32_t rva) {
    const auto* dos = (const IMAGE_DOS_HEADER*)(uintptr_t)module;
    const auto* nt = (const IMAGE_NT_HEADERS32*)(uintptr_t)(module + dos->e_lfanew);
    const IMAGE_SECTION_HEADER* s = IMAGE_FIRST_SECTION(nt);
    for (unsigned i = 0; i < nt->FileHeader.NumberOfSections; i++, s++) {
        const uint32_t size = s->Misc.VirtualSize ? s->Misc.VirtualSize : s->SizeOfRawData;
        if (rva - s->VirtualAddress < size) {
            char name[9] = {};
            memcpy(name, s->Name, 8);
            return name;
        }
    }
    return "header";
}

// The target of a pointer-sized value, or "" when it is not committed,
// readable memory. `stackBase`: the allocation base of the calling (game)
// thread's stack. *block: the heap block it lies in, if any.
inline std::string Describe(uint32_t v, const HeapIndex& hx, uint32_t stackBase, const Block** block = nullptr) {
    if (block) *block = nullptr;
    MEMORY_BASIC_INFORMATION mi;
    if (!Query(v, &mi) || (mi.Protect & (PAGE_NOACCESS | PAGE_GUARD)) || !mi.Protect) return "";
    const LiveMem mem;
    std::string s;
    const uint32_t alloc = (uint32_t)(uintptr_t)mi.AllocationBase;
    if (mi.Type == MEM_IMAGE) {
        const std::string mr = ModRva(v);
        const size_t plus = mr.find('+');
        s = "image " + mr.substr(0, plus) + " " + mr.substr(plus + 1) + " " + SectionOf(alloc, v - alloc);
        uint32_t start = 0;
        std::string cls;
        if (VtableAt(mem, v, &start, &cls)) s += " vtable " + cls + " +" + Hex(v - start);
    } else if (mi.Type == MEM_MAPPED) {
        return "mapped " + Hex(alloc) + " +" + Hex(v - alloc);
    } else if (alloc == stackBase) {
        return "stack";
    } else if (const Block* b = hx.Find(v)) {
        if (block) *block = b;
        s = "heap " + hx.Name(b->heap) + " " + Hex(b->base) + " " + Hex(b->size) + " +" + Hex(v - b->base);
        if (b->base != v) {
            const std::string bc = ObjectClass(mem, b->base);
            if (!bc.empty()) s += " base_rtti " + bc;
        }
    } else {
        s = "private " + Hex(alloc) + " +" + Hex(v - alloc);
    }
    const std::string c = ObjectClass(mem, v);
    if (!c.empty()) s += " rtti " + c;
    return s;
}

inline uint32_t StackBase() {
    MEMORY_BASIC_INFORMATION mi{};
    int local = 0;
    VirtualQuery(&local, &mi, sizeof mi);
    return (uint32_t)(uintptr_t)mi.AllocationBase;
}

// ---- sidecar files ----------------------------------------------------------------
// <stem>.census / <stem>.readset: written to `<path>.pending`, an `end <lines>`
// line last, then the trace's checked close and publication (never replacing
// an existing file). A reader rejects a file without the `end` line.
class Sidecar {
    FILE* f_ = nullptr;
    std::string pending_, path_;
    uint32_t status_ = 0, lines_ = 0;

public:
    bool on() const { return f_ != nullptr; }
    const std::string& path() const { return path_; }

    bool Open(const std::string& path, const std::string& header) {
        path_ = path;
        pending_ = path + ".pending";
        status_ = lines_ = 0;
        if (fopen_s(&f_, pending_.c_str(), "wb") || !f_) {
            f_ = nullptr;
            return false;
        }
        setvbuf(f_, nullptr, _IOFBF, 1 << 20);
        Put(header);
        return true;
    }

    void Put(const std::string& s) {
        if (f_ && fwrite(s.data(), 1, s.size(), f_) != s.size()) status_ |= 1;
    }

    void Line(const std::string& s) {
        Put(s);
        Put("\n");
        lines_++;
    }

    // 0 = published.
    uint32_t Close() {
        if (!f_) return 0;
        Put("end " + std::to_string(lines_) + "\n");
        const uint32_t st = tracefile::CloseAndPublish(f_, status_, pending_.c_str(), path_.c_str());
        f_ = nullptr;
        return st;
    }
};

// ---- one census sample ----------------------------------------------------------
struct View {
    const char* name;
    const uint8_t* bytes;   // the captured bytes, as the trace has them
    uint32_t len;
};

// A `ptr <tag> <region> <off> <value> <target>` line per described dword.
inline uint32_t Scan(Sidecar& out, const HeapIndex& hx, const std::string& tag, const std::vector<View>& views) {
    const uint32_t stack = StackBase();
    uint32_t n = 0;
    for (const View& r : views) {
        for (uint32_t off = 0; off + 4 <= r.len; off += 4) {
            uint32_t v;
            memcpy(&v, r.bytes + off, 4);
            if (v < 0x10000) continue;
            const std::string d = Describe(v, hx, stack);
            if (d.empty()) continue;
            out.Line("ptr " + tag + " " + r.name + " " + Hex(off) + " " + Hex(v) + " " + d);
            n++;
        }
    }
    return n;
}

} // namespace census
