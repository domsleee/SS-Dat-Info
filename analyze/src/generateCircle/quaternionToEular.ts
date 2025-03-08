import { Quaternion, Rotation } from "./types";

/**
 * Convert Euler angles to Quaternion (Left-handed system)
 * @param {Rotation} euler - Euler angles in degrees
 * @returns {Quaternion} Quaternion
 */
export function eulerToQuaternion(euler: Rotation): Quaternion {
  // Convert degrees to radians
  const x = (euler.rotX * Math.PI) / 180;
  const y = (euler.rotY * Math.PI) / 180;
  const z = (euler.rotZ * Math.PI) / 180;

  // Calculate half angles
  const cx = Math.cos(x / 2);
  const sx = Math.sin(x / 2);
  const cy = Math.cos(y / 2);
  const sy = Math.sin(y / 2);
  const cz = Math.cos(z / 2);
  const sz = Math.sin(z / 2);

  // Compute quaternion components - left-handed system
  return {
    w: cx * cy * cz + sx * sy * sz,
    x: sx * cy * cz - cx * sy * sz,
    y: cx * sy * cz + sx * cy * sz,
    z: cx * cy * sz - sx * sy * cz,
  };
}

/**
 * Convert Quaternion to Euler angles (Left-handed system)
 * @param {Quaternion} q - Quaternion
 * @returns {Rotation} Euler angles in degrees
 */
export function quaternionToEuler(q: Quaternion): Rotation {
  // Normalize the quaternion
  const norm = Math.sqrt(q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z);
  const w = q.w / norm;
  const x = q.x / norm;
  const y = q.y / norm;
  const z = q.z / norm;

  // Calculate Euler angles - left-handed system
  const rotX = (Math.atan2(2 * (w * x + y * z), 1 - 2 * (x * x + y * y)) * 180) / Math.PI;

  // Handle gimbal lock
  let rotY = 0;
  const sinp = 2 * (w * y - z * x);
  if (Math.abs(sinp) >= 1) {
    rotY = Math.sign(sinp) * 90; // Use 90 degrees if in gimbal lock
  } else {
    rotY = (Math.asin(sinp) * 180) / Math.PI;
  }

  const rotZ = (Math.atan2(2 * (w * z + x * y), 1 - 2 * (y * y + z * z)) * 180) / Math.PI;

  return { rotX, rotY, rotZ };
}
