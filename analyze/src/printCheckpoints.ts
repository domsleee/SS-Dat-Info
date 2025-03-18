import { resolve } from "path";
import { analyzeReplay } from "./analyzeReplayFs";
import { REPLAY_FOLDER } from "./types";

export async function printCheckpoints() {
  const data = await analyzeReplay(resolve(REPLAY_FOLDER, "Village/Hard/0.56.47 Dom.dat"), true);
  // console.log(data.planeCollisionData.allCollisions.filter(t => t.name === 'VillageHard')[0].collisions)
}