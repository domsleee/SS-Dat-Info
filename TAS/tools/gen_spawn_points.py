"""Generate per-level spawn centroids (Player_Start_Location clusters) for the
history level-backfill classifier. Levels that SHARE a spawn cluster (AE/AM/AH;
FM/FH) are printed with the same centroid — the classifier must treat a match
as ambiguous unless exactly one level owns the cluster."""
import json

CODE = {
    "ForestEasy": "FE", "ForestMedium": "FM", "ForestHard": "FH",
    "AlpineEasy": "AE", "AlpineMedium": "AM", "AlpineHard": "AH",
    "VillageEasy": "VE", "VillageMedium": "VM", "VillageHard": "VH",
}

d = json.load(open(r"C:\Users\user\git\SS-Dat-Info\analyze\src\LevelData\levelData.json"))
print("pub const SPAWN_CENTROIDS: &[(&str, [f32; 3])] = &[")
for lvl in sorted(d, key=lambda k: CODE.get(k, "zz")):
    if lvl not in CODE:
        continue
    pts = [i["position"] for i in d[lvl] if i["name"] == "Player_Start_Location"]
    cx = sum(p["x"] for p in pts) / len(pts)
    cy = sum(p["y"] for p in pts) / len(pts)
    cz = sum(p["z"] for p in pts) / len(pts)
    print(f'    ("{CODE[lvl]}", [{cx:.3f}, {cy:.3f}, {cz:.3f}]),')
print("];")

# Sanity: pairwise distances between centroids
import itertools
cents = {}
for lvl in d:
    if lvl not in CODE:
        continue
    pts = [i["position"] for i in d[lvl] if i["name"] == "Player_Start_Location"]
    cents[CODE[lvl]] = (
        sum(p["x"] for p in pts) / len(pts),
        sum(p["y"] for p in pts) / len(pts),
        sum(p["z"] for p in pts) / len(pts),
    )
print("\n# pairwise centroid distances:")
for a, b in itertools.combinations(sorted(cents), 2):
    pa, pb = cents[a], cents[b]
    dist = sum((x - y) ** 2 for x, y in zip(pa, pb)) ** 0.5
    flag = "  <-- SHARED" if dist < 60 else ""
    print(f"# {a}-{b}: {dist:.1f}{flag}")
