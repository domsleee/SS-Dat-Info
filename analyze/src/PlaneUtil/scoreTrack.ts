import { GameObject } from "../GameConfigParser";
import { PositionXYZ } from "../generateCircle/types";
import { getLevels } from "../LevelData/levels";
import { AnalyzeResult, RowData } from "../types";
import { calculateDistance } from "./distanceUtil";
import { getPlaneCollisions } from "./getPlaneCollisions";
import { LevelPlaneCollision, PlaneCollisionInfo, ScoreEveryLevel, TrackScoreData } from "./types";

export function getTrackScore(analyzeResult: AnalyzeResult): TrackScoreData {
  const allCollisions = getPlaneCollisions(analyzeResult.coords!.rows);
  
  const everyLevelScored = scoreEveryLevel(analyzeResult, allCollisions);
  return { allCollisions, everyLevelScored };
}

function scoreEveryLevel(analyzeResult: AnalyzeResult, planeCollisions: Array<LevelPlaneCollision>): ScoreEveryLevel {
  let levelScores: ScoreEveryLevel = [];
  for (const level of planeCollisions) {
    const nearestStartPoint = getNearestPlayerStartLocation(level.name, analyzeResult.coords!.rows);

    const levelCollisions: Array<PlaneCollisionInfo> = [];
    let largestObjectNameScore = 0;
    for (const collision of level.collisions) {
      const objectNameScore = getObjectNameScore(collision.objectName);
      if (collision.distance <= 90 && objectNameScore > largestObjectNameScore) {
        largestObjectNameScore = objectNameScore;
        levelCollisions.push(collision);
      }
    }

    const scoreData: ScoreData = {
        nearestStartDist: nearestStartPoint.dist,
        startPlaneDiffMs: getMsDiff(analyzeResult, levelCollisions, 'Start_Point'),
        checkpoint1DiffMs: getMsDiff(analyzeResult, levelCollisions, 'Check_Point_1'),
        finishPointDiffMs: getMsDiff(analyzeResult, levelCollisions, 'Finish_Point'),
        levelCollisions,
    }
    levelScores.push({name: level.name, scoreData, score: getScore(scoreData)})
  }

  levelScores = levelScores.sort((a, b) => b.score - a.score);
  return levelScores;
}

function getMsDiff(analyzeResult: AnalyzeResult, levelCollisions: Array<PlaneCollisionInfo>, objectName: string): number | undefined {
    const collision = levelCollisions.filter(t => t.objectName === objectName)?.at(0);
    if (!collision) return undefined;

    const crossStartPlusStartDelayMs = analyzeResult.lagBeforeStartMs + analyzeResult.startMs;
    if (objectName === 'Start_Point') {
        return Math.abs(collision.frame2 * 10 - crossStartPlusStartDelayMs);
    }
    if (objectName === 'Check_Point_1') {
        return Math.abs(collision.frame2 * 10 - (crossStartPlusStartDelayMs + analyzeResult.checkpoint1Ms))
    }
    if (objectName === 'Finish_Point') {
        return Math.abs(collision.frame2 * 10 - (crossStartPlusStartDelayMs + analyzeResult.displayedMs))
    }

    throw new Error(`Unhandled ${objectName}`)
}

function getNearestPlayerStartLocation(levelName: string, rows: Array<RowData>) {
    const levelData = getLevels().filter(t => t.name === levelName);
    if (levelData.length !== 1) {
        throw new Error(`Expected to find only one level with levelName: ${levelName}, found ${levelData.length}`)
    }

    const objects = levelData[0].objects;
    const startLocations = objects.filter(t => t.type === 'Player_Start_Location');
    if (startLocations.length === 0) {
        throw new Error("No start locations");
    }

    let nearest = startLocations[0];
    const replayStartPos: PositionXYZ = {x: rows[0].x, y: rows[0].y, z: rows[0].z};
    for (const startLocation of startLocations) {
        const pos1 = getLocationFromGameObject(nearest);
        const pos2 = getLocationFromGameObject(startLocation);
        const d1 = calculateDistance(replayStartPos, pos1);
        const d2 = calculateDistance(replayStartPos, pos2);
        if (d2 < d1) {
            nearest = startLocation;
        }
    }

    const pos = getLocationFromGameObject(nearest);
    const dist = calculateDistance(replayStartPos, pos);
    return {
        gameObject: nearest,
        pos,
        dist
    }
}

function getLocationFromGameObject(gameObject: GameObject): PositionXYZ {
    const loc = gameObject.properties["loc"] as Array<string>;
    return { x: parseFloat(loc[0]), y: parseFloat(loc[1]), z: parseFloat(loc[2]) }
}

function getObjectNameScore(objectName: string) {
  switch (objectName) {
    case "Start_Point":
      return 1;
    case "Check_Point_1":
      return 2;
    case "Check_Point_2":
      return 3;
    case "Finish_Point":
      return 4;
    default:
      return 0;
  }
}

function getScore(scoreData: ScoreData) {
    return (scoreData.nearestStartDist <= 5 ? 1 : 0)
        + (scoreData.startPlaneDiffMs !== undefined && scoreData.startPlaneDiffMs <= 10 ? 1 : 0)
        + (scoreData.checkpoint1DiffMs !== undefined && scoreData.checkpoint1DiffMs <= 10 ? 1 : 0)
        + (scoreData.finishPointDiffMs !== undefined && scoreData.finishPointDiffMs <= 10 ? 1 : 0)
}

interface ScoreData {
    nearestStartDist: number;
    startPlaneDiffMs: number | undefined;
    checkpoint1DiffMs: number | undefined;
    finishPointDiffMs: number | undefined;
    levelCollisions: Array<PlaneCollisionInfo>;
}
