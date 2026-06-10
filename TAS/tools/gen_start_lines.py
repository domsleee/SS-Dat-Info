"""Generate the Rust start-line table for tas_ui from levelData.json."""
import json

CODE = {
    "ForestEasy": "FE", "ForestMedium": "FM", "ForestHard": "FH",
    "AlpineEasy": "AE", "AlpineMedium": "AM", "AlpineHard": "AH",
    "VillageEasy": "VE", "VillageMedium": "VM", "VillageHard": "VH",
}

d = json.load(open(r"C:\Users\user\git\SS-Dat-Info\analyze\src\LevelData\levelData.json"))
print("pub const START_POINTS: &[(&str, &[[f32; 3]])] = &[")
for lvl in sorted(d, key=lambda k: CODE.get(k, "zz")):
    if lvl not in CODE:
        continue
    pts = [i["position"] for i in d[lvl] if i["name"] == "Start_Point"]
    fmt = ", ".join(f"[{p['x']:.3f}, {p['y']:.3f}, {p['z']:.3f}]" for p in pts)
    print(f'    ("{CODE[lvl]}", &[{fmt}]),')
print("];")
