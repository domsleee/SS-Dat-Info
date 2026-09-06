"""Find bytes stable within each state but different between two states."""
import argparse
from pathlib import Path


def stable_runs(a1, a2, b1, b2, a_base, b_base, max_value=255):
    lo = max(a_base, b_base)
    hi = min(a_base + min(len(a1), len(a2)), b_base + min(len(b1), len(b2)))
    runs = []
    for address in range(lo, hi):
        a, b = address - a_base, address - b_base
        if a1[a] == a2[a] and b1[b] == b2[b] and a1[a] != b1[b] and max(a1[a], b1[b]) <= max_value:
            if runs and address == runs[-1][1] + 1:
                runs[-1][1] = address
            else:
                runs.append([address, address])
    return runs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("a1", "a2", "b1", "b2"):
        parser.add_argument(name, type=Path)
    parser.add_argument("base", nargs="?", type=lambda s: int(s, 16), default=0x88000,
                        help="common base offset in hex (default: 88000)")
    parser.add_argument("--a-base", type=lambda s: int(s, 16), help="state A base in hex")
    parser.add_argument("--b-base", type=lambda s: int(s, 16), help="state B base in hex")
    parser.add_argument("--max-value", type=lambda s: int(s, 0), default=255,
                        help="only compare values at most this number (e.g. 0xf)")
    args = parser.parse_args()
    if not 0 <= args.max_value <= 255:
        parser.error("--max-value must be between 0 and 255")
    a1, a2, b1, b2 = (getattr(args, name).read_bytes() for name in ("a1", "a2", "b1", "b2"))
    a_base = args.base if args.a_base is None else args.a_base
    b_base = args.base if args.b_base is None else args.b_base
    runs = stable_runs(a1, a2, b1, b2, a_base, b_base, args.max_value)
    print(f"{sum(e - s + 1 for s, e in runs)} differing-stable bytes in {len(runs)} runs")
    for s, e in runs:
        a = a1[s - a_base:e - a_base + 1].hex()
        b = b1[s - b_base:e - b_base + 1].hex()
        print(f"exe+{s:#x}..{e:#x} len={e - s + 1}  A={a}  B={b}")


if __name__ == "__main__":
    main()
