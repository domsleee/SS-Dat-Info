import type { Plane, PositionXYZ, LineSegment } from "../generateCircle/types";

export function doesLineSegmentCollideWithPlane(plane: Plane, segment: LineSegment): PositionXYZ | undefined {
  // Extract normal from quaternion (transform the world up vector by quaternion)
  const normal = {
    x: 2 * (plane.quat.x * plane.quat.z + plane.quat.w * plane.quat.y),
    y: 2 * (plane.quat.y * plane.quat.z - plane.quat.w * plane.quat.x),
    z: 1 - 2 * (plane.quat.x * plane.quat.x + plane.quat.y * plane.quat.y),
  };

  const normalLength = Math.sqrt(normal.x * normal.x + normal.y * normal.y + normal.z * normal.z);
  if (normalLength < 1e-6) return undefined;

  const normalizedNormal = {
    x: normal.x / normalLength,
    y: normal.y / normalLength,
    z: normal.z / normalLength,
  };

  const direction = {
    x: segment.p2.x - segment.p1.x,
    y: segment.p2.y - segment.p1.y,
    z: segment.p2.z - segment.p1.z,
  };

  const dotNormalDirection =
    normalizedNormal.x * direction.x + normalizedNormal.y * direction.y + normalizedNormal.z * direction.z;

  if (Math.abs(dotNormalDirection) < 1e-6) return undefined;

  const toStart = {
    x: segment.p1.x - plane.position.x,
    y: segment.p1.y - plane.position.y,
    z: segment.p1.z - plane.position.z,
  };

  const dotNormalToStart =
    normalizedNormal.x * toStart.x + normalizedNormal.y * toStart.y + normalizedNormal.z * toStart.z;

  const t = -dotNormalToStart / dotNormalDirection;

  if (t < 0 || t > 1) {
    // If the intersection is outside the segment, return undefined
    return undefined;
  }

  const intersection = {
    x: segment.p1.x + t * direction.x,
    y: segment.p1.y + t * direction.y,
    z: segment.p1.z + t * direction.z,
  };

  return intersection;
}

