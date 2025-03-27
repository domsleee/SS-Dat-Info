import { GameObject } from "../GameConfigParser";
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

export interface LevelScore {
  levelName: string;
  score: number;
  planeCollisionInfos: Array<PlaneCollisionInfo>;
}

export interface LevelPlaneCollision {
  name: string;
  collisions: Array<PlaneCollisionInfo>;
}

export interface TrackScoreData {
  allCollisions: Array<LevelPlaneCollision>;
  everyLevelScored: ScoreEveryLevel;
}

export interface ScoreData {
  nearestStartDist: number;
  startPlaneDiffMs: number | undefined;
  checkpoint1DiffMs: number | undefined;
  finishPointDiffMs: number | undefined;
  levelCollisions: Array<PlaneCollisionInfo>;
}


export type ScoreEveryLevel = Array<{name: string, scoreData: ScoreData, score: number}>;
