import json

d = json.load(open(r"C:\Users\user\git\SS-Dat-Info\analyze\src\LevelData\levelData.json"))
for lvl, items in d.items():
    starts = [i for i in items if "start" in i["name"].lower()]
    for s in starts:
        p = s["position"]
        q = s["quat"]
        print(
            f"{lvl:14s} {s['name']:14s} pos=({p['x']:.2f},{p['y']:.2f},{p['z']:.2f}) "
            f"quat=({q['w']:.4f},{q['x']:.4f},{q['y']:.4f},{q['z']:.4f})"
        )
