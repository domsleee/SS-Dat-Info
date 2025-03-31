import type { Plane, PositionXYZ } from "../generateCircle/types";

export interface PlaneLineSegmentCollision {
  distance: number;
  intersection: PositionXYZ;
}

export interface PlaneCollisionInfo {
  objectName: string;
  distance: number;
  intersection: PositionXYZ;
  plane: Plane;
  p1: PositionXYZ;
  p2: PositionXYZ;
  frame1: number;
  frame2: number;
}

export interface LevelPlaneCollision {
  name: string;
  collisions: Array<PlaneCollisionInfo>;
}

export interface EveryLevelScoredData {
  allCollisions: Array<LevelPlaneCollision>;
  levelScores: Array<LevelScore>;
}

export interface LevelScore {
  name: string;
  scoreData: ScoreData;
  score: number;
}

export interface ScoreData {
  nearestStartDist: number;
  startPlaneDiffMs: number | undefined;
  checkpoint1DiffMs: number | undefined;
  finishPointDiffMs: number | undefined;
  checkpoint2Exists: boolean;

  firstValidStartPointCollision: PlaneCollisionInfo | undefined;
  firstValidCheckPoint1Collision: PlaneCollisionInfo | undefined;
  firstValidFinishPointCollision: PlaneCollisionInfo | undefined;
  allCheckPoint2Collisions: Array<PlaneCollisionInfo>;
  validCheckPoint2Collisions: Array<PlaneCollisionInfo>;
}
