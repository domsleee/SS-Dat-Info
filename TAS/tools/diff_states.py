"""Differential state RE: find bytes stable within each state but different
between states. Usage: diff_states.py a1 a2 b1 b2 [base_offset_hex]"""
import sys

a1, a2, b1, b2 = [open(p, "rb").read() for p in sys.argv[1:5]]
base = int(sys.argv[5], 16) if len(sys.argv) > 5 else 0x88000

hits = []
for i in range(min(len(a1), len(a2), len(b1), len(b2))):
    if a1[i] == a2[i] and b1[i] == b2[i] and a1[i] != b1[i]:
        hits.append(i)

# Group consecutive offsets into runs for readability
runs = []
for i in hits:
    if runs and i == runs[-1][1] + 1:
        runs[-1][1] = i
    else:
        runs.append([i, i])

print(f"{len(hits)} differing-stable bytes in {len(runs)} runs")
for s, e in runs:
    a_bytes = a1[s:e + 1].hex()
    b_bytes = b1[s:e + 1].hex()
    print(f"exe+{base + s:#x}..{base + e:#x} len={e - s + 1}  A={a_bytes}  B={b_bytes}")
