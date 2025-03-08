import { Quaternion } from "./types";

export function generateCircle(
  _config: Partial<Config> = {}
): Array<CircleEntry> {
  // Default configuration
  let config = {
    name: "Audience_4",
    count: 10,
    radius: 90,
    uniform_scale: 0.2,
    centerLoc: { x: 248.844, y: -45.5154, z: 1096.15 },
    quat: { w: 1, x: 0, y: 0, z: 0 },
    ..._config
  };

  let result: Array<CircleEntry> = [];
  
  // Extract quaternion components
  const baseW = config.quat.w;
  const baseY = config.quat.y;
  
  // The correct rotation angle for a left-handed system
  // We NEGATE the y component to get the proper tilt direction
  const halfAngle = Math.asin(-baseY);
  const fullAngle = 2 * halfAngle;
  
  for (let i = 0; i < config.count; i++) {
    // Position around the circle
    const angle = (i / config.count) * 2 * Math.PI;
    
    // Calculate base position in the XY plane
    const localX = config.radius * Math.cos(angle);
    const localY = config.radius * Math.sin(angle);
    const localZ = 0;
    
    // Apply y-axis rotation using the left-handed formula:
    // x' = x*cos(θ) - z*sin(θ)
    // z' = x*sin(θ) + z*cos(θ)
    const rotatedX = localX * Math.cos(fullAngle) - localZ * Math.sin(fullAngle);
    const rotatedY = localY;
    const rotatedZ = localX * Math.sin(fullAngle) + localZ * Math.cos(fullAngle);
    
    // Calculate the correct facing quaternion (tangential to circle)
    // For a tangential direction, we need a 90 degree offset from the position
    let facingAngle = angle + Math.PI/2;
    
    // Normalize the angle to 0-2π range for consistent quaternion calculation
    facingAngle = ((facingAngle % (2 * Math.PI)) + 2 * Math.PI) % (2 * Math.PI);
    
    // Calculate the quaternion that combines both the base y-rotation and the facing direction
    // First, create the z-rotation quaternion
    const halfFacingAngle = facingAngle / 2;
    const zRotQuat = {
      w: Math.cos(halfFacingAngle),
      x: 0,
      y: 0,
      z: Math.sin(halfFacingAngle)
    };
    
    // Multiply the z-rotation quaternion with the base quaternion
    // Using quaternion multiplication formula
    const facingQuat = multiplyQuaternions(config.quat, zRotQuat);
    
    // Round values to match expected precision
    const offset = {
      x: roundZero(roundThousandth(rotatedX)),
      y: roundZero(roundThousandth(rotatedY)),
      z: roundZero(roundThousandth(rotatedZ))
    };
    
    // Ensure small values are exactly zero
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

interface CircleEntry {
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

export function circleToString(circle: Array<CircleEntry>): string {
  return circle.map((entry) => {
    return `${entry.name} {
uniform_scale = ${entry.uniform_scale.toFixed(3)};
loc = {${entry.loc.x}, ${entry.loc.y}, ${entry.loc.z}};
level_inclusion_mask = -1;
offset = {${entry.offset.x.toFixed(3)}, ${entry.offset.y.toFixed(3)}, ${entry.offset.z.toFixed(3)}};
quat = {${entry.quat.w.toFixed(6)}, {${entry.quat.x.toFixed(6)}, ${entry.quat.y.toFixed(6)}, ${entry.quat.z.toFixed(6)}}};
melt_height = 0;
};`}).join("\n\n");
}


/*
Finish_Point {
uniform_scale	= 1;
loc	= {211.158,285.281,1897.36};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
};
*/

/*

VM
Finish_Point {
uniform_scale	= 1;
loc	= {605.56,1116.02,2583.45};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {0.999118,{0,-0.0419877,0}};
melt_height	= 0;
};

Finish_Point {
uniform_scale	= 1;
loc	= {562.33,1199.33,2580.01};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
};

Finish_Point {
uniform_scale	= 1;
loc	= {661.13,-517.818,2589.11};
level_inclusion_mask	= -1;
offset	= {0,0,0};
quat	= {1,{0,0,0}};
melt_height	= 0;
};
*/
// console.log(circleToString(generateCircle({ centerLoc: {x: 605.56, y: 1116.02, z:2583.45}, count: 4, uniform_scale: 4, quat: { w: 0.966, x: 0, y: -0.259, z: 0} })));