import type { Plane, PositionXYZ, LineSegment } from "../generateCircle/types";

export function doesLineSegmentCollideWithPlane(plane: Plane, segment: LineSegment): PositionXYZ | undefined {
  return doesLineSegmentCollideWithPlaneWithNormal({...plane, normal: getPlaneNormal(plane)}, segment);
}

export interface PlaneWithNormal extends Plane {
  normal: PositionXYZ;
}

// 100% of exits go through e5, but some early exits are left in anyways.
// export const tracing = {e1: 0, e2: 0, e3: 0, e4: 0, e5: 0};
export function doesLineSegmentCollideWithPlaneWithNormal(plane: PlaneWithNormal, segment: LineSegment): PositionXYZ | undefined {
  const { normal } = plane;
  const normalLength = Math.sqrt(normal.x * normal.x + normal.y * normal.y + normal.z * normal.z);
  if (normalLength < 1e-6) {
    // tracing.e2++;
    return undefined;
  }

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

  if (Math.abs(dotNormalDirection) < 1e-6) {
    // tracing.e3++;
    return undefined;
  }

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
    // tracing.e4++;
    return undefined;
  }

  const intersection = {
    x: segment.p1.x + t * direction.x,
    y: segment.p1.y + t * direction.y,
    z: segment.p1.z + t * direction.z,
  };

  // tracing.e5++;
  return intersection;
}

export function getPlaneNormal(plane: Plane): PositionXYZ {
  return {
    x: 2 * (plane.quat.x * plane.quat.z + plane.quat.w * plane.quat.y),
    y: 2 * (plane.quat.y * plane.quat.z - plane.quat.w * plane.quat.x),
    z: 1 - 2 * (plane.quat.x * plane.quat.x + plane.quat.y * plane.quat.y),
  };
}
