export interface PositionXYZ {
  x: number;
  y: number;
  z: number;
}

export interface Quaternion {
  w: number;
  x: number;
  y: number;
  z: number;
}

export interface Plane {
  position: PositionXYZ;
  quat: Quaternion;
}

export interface LineSegment {
  p1: PositionXYZ;
  p2: PositionXYZ;
}

export interface Rotation {
  rotX: number;
  rotY: number;
  rotZ: number;
}
