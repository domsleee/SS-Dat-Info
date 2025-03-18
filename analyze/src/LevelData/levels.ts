import { GameConfigParser, GameObject } from "../GameConfigParser";
import { LEVELS } from "./levelData";

type ParsedLevels = Array<{ name: string; objects: Array<GameObject> }>;

let levels: ParsedLevels | undefined = undefined;
export function getLevels(): ParsedLevels {
  if (!levels) {
    levels = [];
    for (const level of LEVELS) {
      levels.push({
        name: level.name,
        objects: new GameConfigParser(level.raw).parse(),
      });
    }
  }
  return levels;
}
