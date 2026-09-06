"""Inspect level markers or generate the Rust start/finish/spawn tables."""
import argparse
from collections import Counter
from itertools import combinations
import json
from pathlib import Path

DEFAULT_INPUT = Path(__file__).resolve().parents[2] / "analyze/src/LevelData/levelData.json"
CODE = {
    "ForestEasy": "FE", "ForestMedium": "FM", "ForestHard": "FH",
    "AlpineEasy": "AE", "AlpineMedium": "AM", "AlpineHard": "AH",
    "VillageEasy": "VE", "VillageMedium": "VM", "VillageHard": "VH",
}


def generate(data, kind):
    marker = {"start": "Start_Point", "finish": "Finish_Point", "spawn": "Player_Start_Location"}[kind]
    spawn = kind == "spawn"
    name = "SPAWN_CENTROIDS" if spawn else f"{kind.upper()}_POINTS"
    point_type = "[f32; 3]" if spawn else "&[[f32; 3]]"
    lines = [f"pub const {name}: &[(&str, {point_type})] = &["]
    centroids = {}
    for level in sorted(data, key=lambda level: CODE.get(level, "zz")):
        if level not in CODE:
            continue
        points = [tuple(item["position"][axis] for axis in "xyz")
                  for item in data[level] if item["name"] == marker]
        if spawn:
            if not points:
                raise ValueError(f"{level} has no {marker} markers")
            centroid = tuple(sum(values) / len(points) for values in zip(*points))
            centroids[CODE[level]] = centroid
            value = "[" + ", ".join(f"{v:.3f}" for v in centroid) + "]"
        else:
            value = "&[" + ", ".join("[" + ", ".join(f"{v:.3f}" for v in p) + "]" for p in points) + "]"
        lines.append(f'    ("{CODE[level]}", {value}),')
    lines.append("];")
    if spawn:
        lines.append("\n# pairwise centroid distances:")
        for a, b in combinations(sorted(centroids), 2):
            distance = sum((x - y) ** 2 for x, y in zip(centroids[a], centroids[b])) ** 0.5
            flag = "  <-- SHARED" if distance < 60 else ""
            lines.append(f"# {a}-{b}: {distance:.1f}{flag}")
    return "\n".join(lines)


def inspect(data, kind):
    for level, items in data.items():
        if kind == "finish":
            print(level, dict(Counter(item["name"] for item in items)))
        for item in items:
            name = item["name"]
            if not (kind in name.lower() or kind == "finish" and "end" in name.lower()):
                continue
            p = item["position"]
            position = f"({p['x']:.2f},{p['y']:.2f},{p['z']:.2f})"
            if kind == "start":
                q = item["quat"]
                print(f"{level:14s} {name:14s} pos={position} "
                      f"quat=({q['w']:.4f},{q['x']:.4f},{q['y']:.4f},{q['z']:.4f})")
            else:
                print(f"   {name}: {position}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=DEFAULT_INPUT)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("generate").add_argument("kind", choices=("start", "finish", "spawn"))
    commands.add_parser("inspect").add_argument("kind", choices=("start", "finish"))
    args = parser.parse_args()
    with args.input.open(encoding="utf-8") as source:
        data = json.load(source)
    if args.command == "generate":
        print(generate(data, args.kind))
    else:
        inspect(data, args.kind)


if __name__ == "__main__":
    main()
