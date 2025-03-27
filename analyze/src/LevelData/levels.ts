import { LevelDataEntry } from "./types";
import levelData from "../LevelData/levelData.json"

type ParsedLevels = Array<{name: string, entries: Array<LevelDataEntry>}>;

let levels: ParsedLevels | undefined = undefined;
export function getLevels(): ParsedLevels {
  if (!levels) {
    levels = [];
    for (const [k, v] of Object.entries(levelData)) {
      levels.push({
        name: k,
        entries: v,
      });
    }
  }
  return levels;
}
