import { PositionXYZ } from "../generateCircle/types";
import { getLevels } from "../LevelData/levels";
import { AnalyzeResult, RowData } from "../types";
import { calculateDistance } from "./distanceUtil";
import { getPlaneCollisions } from "./getPlaneCollisions";
import { LevelPlaneCollision as LevelPlaneCollisionGroup, LevelScore, PlaneCollisionInfo, ScoreData, EveryLevelScoredData } from "./types";

export const PLANE_RADIUS = 90;
export const NEAREST_START_POINT_DIST = 2;
export const MAX_SCORE = 5;

export function getEveryLevelScored(analyzeResult: AnalyzeResult): EveryLevelScoredData {
  const levelPlaneCollisionGroups = getPlaneCollisions(analyzeResult.coords!.rows);
  const levelScores = scoreEveryLevel(analyzeResult, levelPlaneCollisionGroups);
  return { allCollisions: levelPlaneCollisionGroups, levelScores };
}

function scoreEveryLevel(
  analyzeResult: AnalyzeResult,
  levelPlaneCollisionGroups: Array<LevelPlaneCollisionGroup>
): Array<LevelScore> {
  let levelScores: Array<LevelScore> = [];
  for (const levelPlaneCollisionGroup of levelPlaneCollisionGroups) {
    const nearestStartPoint = getNearestPlayerStartLocation(levelPlaneCollisionGroup.name, analyzeResult.coords!.rows);

    const firstValidStartPointCollision = getValidCollisions(levelPlaneCollisionGroup, "Start_Point")?.at(0);
    const firstValidCheckPoint1Collision = getValidCollisions(levelPlaneCollisionGroup, "Check_Point_1")?.at(0);
    const firstValidFinishPointCollision = getValidCollisions(levelPlaneCollisionGroup, "Finish_Point")?.at(0);
    const allCheckPoint2Collisions = levelPlaneCollisionGroup.collisions.filter((t) => t.objectName === "Check_Point_2");
    const validCheckPoint2Collisions = allCheckPoint2Collisions
      .filter(t => t.distance <= PLANE_RADIUS)
      .filter(t => !firstValidStartPointCollision || firstValidStartPointCollision.frame2 <= t.frame1)
      .filter(t => !firstValidCheckPoint1Collision || firstValidCheckPoint1Collision.frame2 <= t.frame1)
      .filter(t => !firstValidFinishPointCollision || t.frame2 <= firstValidFinishPointCollision.frame2);

    const scoreData: ScoreData = {
      nearestStartDist: nearestStartPoint.dist,
      startPlaneDiffMs: getMsDiff(analyzeResult, firstValidStartPointCollision, "Start_Point"),
      checkpoint1DiffMs: getMsDiff(analyzeResult, firstValidCheckPoint1Collision, "Check_Point_1"),
      finishPointDiffMs: getMsDiff(analyzeResult, firstValidFinishPointCollision, "Finish_Point"),
      checkpoint2Exists: validCheckPoint2Collisions.length > 0,
      firstValidStartPointCollision,
      firstValidCheckPoint1Collision,
      firstValidFinishPointCollision,
      allCheckPoint2Collisions,
      validCheckPoint2Collisions
    };

    const scoreFlags = getScoreFlags(scoreData);
    const score = Object.values(scoreFlags).filter(t => t).length;
    levelScores.push({ name: levelPlaneCollisionGroup.name, scoreData, score });
  }

  levelScores = levelScores.sort((a, b) => b.score - a.score);
  return levelScores;
}

function getValidCollisions(
  levelPlaneCollisionGroup: LevelPlaneCollisionGroup,
  objectName: string
): Array<PlaneCollisionInfo> {
  return levelPlaneCollisionGroup.collisions.filter((t) => t.objectName === objectName && t.distance <= PLANE_RADIUS);
}

export function getMsDiff(
  analyzeResult: AnalyzeResult,
  collision: PlaneCollisionInfo | undefined,
  objectName: string
): number | undefined {
  if (!collision) return undefined;
  const { crossStartPlusStartDelayMs, totalTimeToFinishMs, checkpoint1TotalMs } = analyzeResult.timingDataFromHeader;

  switch (objectName) {
    case "Start_Point": return collision.frame2 * 10 - crossStartPlusStartDelayMs;
    case "Check_Point_1": return collision.frame2 * 10 - checkpoint1TotalMs;
    case "Finish_Point": return collision.frame2 * 10 - totalTimeToFinishMs;
    default: throw new Error(`Unhandled ${objectName}`);
  }
}

function getNearestPlayerStartLocation(levelName: string, rows: Array<RowData>) {
  const levelData = getLevels().filter((t) => t.name === levelName);
  if (levelData.length !== 1) {
    throw new Error(`Expected to find only one level with levelName: ${levelName}, found ${levelData.length}`);
  }

  const entries = levelData[0].entries;
  const startLocations = entries.filter((t) => t.name === "Player_Start_Location");
  if (startLocations.length === 0) {
    throw new Error("No start locations");
  }

  let nearest = startLocations[0];
  const replayStartPos: PositionXYZ = { x: rows[0].x, y: rows[0].y, z: rows[0].z };
  for (const startLocation of startLocations) {
    const pos1 = nearest.position;
    const pos2 = startLocation.position;
    const d1 = calculateDistance(replayStartPos, pos1);
    const d2 = calculateDistance(replayStartPos, pos2);
    if (d2 < d1) {
      nearest = startLocation;
    }
  }

  const pos = nearest.position;
  const dist = calculateDistance(replayStartPos, pos);
  return {
    gameObject: nearest,
    pos,
    dist,
  };
}

function getScoreFlags(scoreData: ScoreData) {
  return {
    hasPlayerStartLocation: scoreData.nearestStartDist <= NEAREST_START_POINT_DIST,
    hasStartPlane: scoreData.startPlaneDiffMs !== undefined && Math.abs(scoreData.startPlaneDiffMs) <= 10,
    hasCheckPoint1: scoreData.checkpoint1DiffMs !== undefined && Math.abs(scoreData.checkpoint1DiffMs) <= 10,
    hasFinishPoint: scoreData.finishPointDiffMs !== undefined && Math.abs(scoreData.finishPointDiffMs) <= 10,
    hasCheckPoint2: scoreData.checkpoint2Exists,
  };
}

export function getScoreBreakdown(scoreData: ScoreData): Array<[string, { valid: boolean }]> {
  const flags = getScoreFlags(scoreData);
  const notes = [];
  notes.push([`Has Player Start Location ≤${NEAREST_START_POINT_DIST}m`, { valid: flags.hasPlayerStartLocation }]);
  notes.push([`Has Start_Point ≤10ms, ≤${PLANE_RADIUS}m`, { valid: flags.hasStartPlane }]);
  notes.push([`Has Check_Point_1 ≤10ms, ≤${PLANE_RADIUS}m`, { valid:flags.hasCheckPoint1 }]);
  notes.push([`Has Finish_Point ≤10ms, ≤${PLANE_RADIUS}m`, { valid: flags.hasFinishPoint }]);
  notes.push([`Has Check_Point_2 ≤${PLANE_RADIUS}m`, { valid: flags.hasCheckPoint2 }]);
  return notes as unknown as Array<[string, { valid: boolean }]>;
}
