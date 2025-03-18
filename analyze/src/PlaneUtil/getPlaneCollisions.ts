
import type { Plane, PositionXYZ } from "../generateCircle/types";
import { doesLineSegmentCollideWithPlane } from "../PlaneUtil/planeUtil";
import { RowData } from "../types";
import type { LevelPlaneCollision } from "./types";
import { GameObject } from "../GameConfigParser";
import { getLevels } from "../LevelData/levels";

export function getPlaneCollisions(rows: Array<RowData>): Array<LevelPlaneCollision> {
  const result: Array<LevelPlaneCollision> = [];

  for (const level of getLevels()) {
    const planeTypes = ['Check_Point_1', 'Check_Point_2', 'Start_Point', 'Finish_Point'];
    const planeGameObjects = level.objects.filter(t => planeTypes.includes(t.type));
    const levelPlaneCollision: LevelPlaneCollision = {name: level.name, collisions: []};
    
    for (let i = 0; i < rows.length - 2; ++i) {
      for (const planeGameObject of planeGameObjects) {
        const plane = getPlaneFromGameObject(planeGameObject);
        const p1: PositionXYZ = { x: rows[i].x, y: rows[i].y, z: rows[i].z };
        const p2: PositionXYZ = { x: rows[i + 1].x, y: rows[i + 1].y, z: rows[i + 1].z };
        const collision = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
        if (collision !== undefined) {
          levelPlaneCollision.collisions.push({
            ...collision,
            p1,
            p2,
            frame1: i,
            frame2: i + 1,
            objectName: planeGameObject.type,
            gameObject: planeGameObject,
            plane: plane
          });
        }
      }
    }
    result.push(levelPlaneCollision)
  }

  return result;
}

function getPlaneFromGameObject(obj: GameObject): Plane {
  const loc = obj.properties["loc"] as Array<string>;
  const quat = obj.properties["quat"] as [string, [string, string, string]];
  return {
    position: { x: parseFloat(loc[0]), y: parseFloat(loc[1]), z: parseFloat(loc[2]) },
    quat: { w: parseFloat(quat[0]), x: parseFloat(quat[1][0]), y: parseFloat(quat[1][1]), z: parseFloat(quat[1][2]) },
  };
}
