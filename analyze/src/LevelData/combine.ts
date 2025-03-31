import { GameConfigParser, GameObject } from "../GameConfigParser"
import { existsSync,readdirSync, writeFileSync } from "fs";
import { join } from "path";
import { readFile } from 'fs/promises';
import { LevelDataEntry } from "./types";
import { Plane } from "../generateCircle/types";

export async function combine() {
  const entries: Record<string, Array<LevelDataEntry>> = {};
  const dir = join(__dirname, "Raw", "Original");
  const textFiles = readdirSync(dir).filter(file => file.endsWith('.txt'));
  for (const textFileName of textFiles) {
    const fullPath = join(dir, textFileName);
    const data = []
    const objects = new GameConfigParser(await readFile(fullPath, 'utf8')).parse();
    const tsFileName = fullPath.replace(".txt", ".raw.ts");
    if (existsSync(tsFileName)) {
      const rawPoints = (await import(tsFileName)).rawPoints;
      if (rawPoints.length !== objects.length) throw new Error("??");
      for (let i = 0; i < rawPoints.length; i++) {
        const object = objects[i];
        const rawPoint = rawPoints[i];
        data.push({
          ...getFromGameObject(object),
          position: { x: rawPoint[1][0], y: rawPoint[1][1], z: rawPoint[1][2] },
        })
      }
    } else {
      for (const object of objects) {
        data.push(getFromGameObject(object));
      }
    }
    entries[textFileName.replace(".txt", "")] = data;
  }

  // console.log(entries);
  writeFileSync(join(__dirname, "levelData.json"), JSON.stringify(entries, null, 2));
}

function getFromGameObject(object: GameObject): LevelDataEntry {
  return {
    name: object.type,
    uniform_scale: parseFloat(object.properties["uniform_scale"] as string),
    ...getPlaneFromGameObject(object),
  }
}

function getPlaneFromGameObject(obj: GameObject): Plane {
  const loc = obj.properties["loc"] as Array<string>;
  const quat = obj.properties["quat"] as [string, [string, string, string]];
  return {
    position: { x: parseFloat(loc[0]), y: parseFloat(loc[1]), z: parseFloat(loc[2]) },
    quat: { w: parseFloat(quat[0]), x: parseFloat(quat[1][0]), y: parseFloat(quat[1][1]), z: parseFloat(quat[1][2]) },
  };
}