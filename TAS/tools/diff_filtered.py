"""Cross-process differential: stable within each state, different between
states, BOTH values small ints (filters heap-pointer noise).
Usage: diff_filtered.py a1 a2 a_base_hex b1 b2 b_base_hex"""
import sys

a1 = open(sys.argv[1], "rb").read()
a2 = open(sys.argv[2], "rb").read()
a_base = int(sys.argv[3], 16)
b1 = open(sys.argv[4], "rb").read()
b2 = open(sys.argv[5], "rb").read()
b_base = int(sys.argv[6], 16)

lo = max(a_base, b_base)
hi = min(a_base + len(a1), b_base + len(b1))
print(f"overlap exe+{lo:#x}..{hi:#x} ({hi - lo} bytes)")

for va in range(lo, hi):
    ai = va - a_base
    bi = va - b_base
    if a1[ai] == a2[ai] and b1[bi] == b2[bi] and a1[ai] != b1[bi]:
        if a1[ai] <= 0x0F and b1[bi] <= 0x0F:
            print(f"exe+{va:#x}: A={a1[ai]:#04x} B={b1[bi]:#04x}")
