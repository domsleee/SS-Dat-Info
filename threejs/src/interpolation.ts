import { calculateDistance } from "dat-analyze/src/PlaneUtil/distanceUtil";
import { PlaneCollisionInfo } from "dat-analyze/src/PlaneUtil/types";

export function getInterpolatedFrame(plane: PlaneCollisionInfo): number {
  const { frame1, p1, p2, intersection } = plane;
  const totalDistance = calculateDistance(p1, p2);
  if (totalDistance === 0) {
    throw new Error("p1 and p2 are the same point, cannot interpolate frame");
  }
  const intersectionDistance = calculateDistance(p1, intersection);
  const t = intersectionDistance / totalDistance;
  return frame1 + t;
}