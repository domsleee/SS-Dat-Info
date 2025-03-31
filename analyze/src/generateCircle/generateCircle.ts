import { Quaternion } from "./types";

export function generateCircle(
  _config: Partial<Config> = {}
): Array<CircleEntry> {
  const config = {
    name: "Audience_4",
    count: 10,
    radius: 90,
    uniform_scale: 0.2,
    centerLoc: { x: 248.844, y: -45.5154, z: 1096.15 },
    quat: { w: 1, x: 0, y: 0, z: 0 },
    ..._config
  };

  const result: Array<CircleEntry> = [];
  
  for (let i = 0; i < config.count; i++) {
    // Position around the circle
    const angle = (i / config.count) * 2 * Math.PI;
    
    // Calculate base position in the XY plane
    const localX = config.radius * Math.cos(angle);
    const localY = config.radius * Math.sin(angle);
    const localZ = 0;
    
    // Apply quaternion rotation to the offset
    // Use the base quaternion to rotate the offset points
    const rotatedOffset = rotatePointByQuaternion(
      { x: localX, y: localY, z: localZ }, 
      config.quat
    );
    
    // Calculate the facing quaternion (tangential to circle)
    const facingAngle = angle + Math.PI/2;
    const zRotQuat = {
      w: Math.cos(facingAngle / 2),
      x: 0,
      y: 0,
      z: Math.sin(facingAngle / 2)
    };
    
    // Combine the base quaternion with the facing rotation
    const facingQuat = multiplyQuaternions(config.quat, zRotQuat);
    
    // Round values for precision
    const offset = {
      x: roundZero(roundThousandth(rotatedOffset.x)),
      y: roundZero(roundThousandth(rotatedOffset.y)),
      z: roundZero(roundThousandth(rotatedOffset.z))
    };
    
    const roundedQuat = {
      w: roundZero(facingQuat.w),
      x: roundZero(facingQuat.x),
      y: roundZero(facingQuat.y),
      z: roundZero(facingQuat.z)
    };
    
    result.push({
      name: config.name,
      loc: config.centerLoc,
      uniform_scale: config.uniform_scale,
      quat: roundedQuat,
      offset: offset
    });
  }

  return result;
}

// Rotate a point using a quaternion
function rotatePointByQuaternion(
  point: {x: number, y: number, z: number}, 
  q: Quaternion
): {x: number, y: number, z: number} {
  // Convert point to quaternion form (w=0)
  const pointQuat = { w: 0, x: point.x, y: point.y, z: point.z };
  
  // q * point * q^-1
  const qInv = {
    w: q.w,
    x: -q.x,
    y: -q.y,
    z: -q.z
  };
  
  const rotated = multiplyQuaternions(
    multiplyQuaternions(q, pointQuat),
    qInv
  );
  
  return { x: rotated.x, y: rotated.y, z: rotated.z };
}

// Quaternion multiplication function
function multiplyQuaternions(q1: Quaternion, q2: Quaternion): Quaternion {
  return {
    w: q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
    x: q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
    y: q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
    z: q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w
  };
}

// Round to 3 decimal places (thousandths)
function roundThousandth(num: number): number {
  return Math.round(num * 1000) / 1000;
}

// Round very small values to zero
function roundZero(num: number, epsilon = 1e-12): number {
  return Math.abs(num) < epsilon ? 0 : num;
}

export interface CircleEntry {
  name: string;
  loc: {x: number, y: number, z: number};
  quat: {w: number, x: number, y: number, z: number};
  offset: {x: number, y: number, z: number};
  uniform_scale: number;
}

interface Config {
  name: string;
  count: number;
  radius: number;
  centerLoc: { x: number, y: number, z: number };
  quat: Quaternion;
  uniform_scale: number;
}