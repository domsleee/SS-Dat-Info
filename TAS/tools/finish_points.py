import json
from collections import Counter

d = json.load(open(r"C:\Users\user\git\SS-Dat-Info\analyze\src\LevelData\levelData.json"))
for lvl, items in d.items():
    names = Counter(i["name"] for i in items)
    print(lvl, dict(names))
    for i in items:
        if "finish" in i["name"].lower() or "end" in i["name"].lower():
            p = i["position"]
            print(f"   {i['name']}: ({p['x']:.2f},{p['y']:.2f},{p['z']:.2f})")
