
import { doesLineSegmentCollideWithPlane } from "../PlaneUtil/planeUtil";
import { RowData } from "../types";
import type { LevelPlaneCollision } from "./types";
import { getLevels } from "../LevelData/levels";
import { calculateDistance } from "./distanceUtil";

export function getPlaneCollisions(rows: Array<RowData>): Array<LevelPlaneCollision> {
  const result: Array<LevelPlaneCollision> = [];
  const rowPositions = rows.map(t => ({ x: t.x, y: t.y, z: t.z }));
  const segments = Array(rows.length-1).fill(0).map((_, i) => ({ p1: rowPositions[i], p2: rowPositions[i+1] }));
  const planeTypes = ['Check_Point_1', 'Check_Point_2', 'Start_Point', 'Finish_Point'];

  for (const level of getLevels()) {
    const planeEntries = level.entries.filter(t => planeTypes.includes(t.name));
    const planeEntryPlanes = planeEntries.map(t => ({ planeName: t.name, plane: {position: t.position, quat: t.quat}}))
    const levelPlaneCollision: LevelPlaneCollision = {name: level.name, collisions: []};

    for (let i = 0; i < rows.length - 1; i++) {
      const segment = segments[i];
      for (const { planeName, plane } of planeEntryPlanes) {
        const intersection = doesLineSegmentCollideWithPlane(plane, segment);
        if (intersection !== undefined) {
          levelPlaneCollision.collisions.push({
            intersection,
            distance: calculateDistance(intersection, plane.position),
            p1: segment.p1,
            p2: segment.p2,
            frame1: i,
            frame2: i+1,
            objectName: planeName,
            plane: plane
          });
        }
      }
    }
    result.push(levelPlaneCollision)
  }

  return result;
}

