import { PositionXYZ } from "../generateCircle/types";

export function calculateDistance(p1: PositionXYZ, p2: PositionXYZ): number {
    const dx = p2.x - p1.x;
    const dy = p2.y - p1.y;
    const dz = p2.z - p1.z;
  
    return Math.sqrt(dx * dx + dy * dy + dz * dz);
  }