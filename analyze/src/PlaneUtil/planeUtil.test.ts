import { test } from "bun:test";
import { doesLineSegmentCollideWithPlane } from "./planeUtil";
import { expect } from "bun:test";
import { describe } from "bun:test";
import { eulerToQuaternion } from "../generateCircle/quaternionToEular";
import { Plane } from "../generateCircle/types";
import "../generateCircle/toBeCloseToStructure";
import { calculateDistance } from "./distanceUtil";

describe("planeUtil", () => {
  test("basic test - collides", () => {
    const plane: Plane = {
      position: { x: 200, y: 200, z: 1000 },
      quat: { w: 1, x: 0, y: 0, z: 0 },
    };
    const p1 = { x: 200, y: 200, z: 800 };
    const p2 = { x: 200, y: 200, z: 1200 };

    const r = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
    expect(r).toEqual({ x: 200, y: 200, z: 1000 });
  });

  test("basic test - does not collide", () => {
    const plane: Plane = {
      position: { x: 200, y: 200, z: 1000 },
      quat: { w: 1, x: 0, y: 0, z: 0 },
    };
    const p1 = { x: 200, y: 200, z: 1100 };
    const p2 = { x: 200, y: 200, z: 1200 };

    const r = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
    expect(r).toBeUndefined();
  });

  test("rotated 30 degrees about the x axis", () => {
    const quat = eulerToQuaternion({ rotX: 30, rotY: 0, rotZ: 0 });
    const plane: Plane = {
      position: { x: 0, y: 0, z: 0 },
      quat,
    };
    const p1 = { x: 0, y: 25.981, z: 14.9 };
    const p2 = { x: 0, y: 25.981, z: 15.1 };
    const r = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
    expect(r).toBeCloseToStructure({ x: 0, y: 25.981, z: 15 });
    expect(calculateDistance(r!, plane.position)).toBeCloseTo(30);
  });

  test("rotated -30 degrees about the y axis", () => {
    const quat = eulerToQuaternion({ rotX: 0, rotY: -30, rotZ: 0 });
    const plane: Plane = {
      position: { x: 0, y: 0, z: 0 },
      quat,
    };
    const p1 = { x: 77.942, y: 0, z: 44.9 };
    const p2 = { x: 77.942, y: 0, z: 45.1 };
    const r = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
    expect(r).toBeCloseToStructure({ x: 77.942, y: 0, z: 45 });
  });

  test("rotated 30 degrees about the z axis", () => {
    const quat = eulerToQuaternion({ rotX: 0, rotY: 0, rotZ: 30 });
    const plane: Plane = {
      position: { x: 0, y: 0, z: 0 },
      quat,
    };
    const p1 = { x: 25.981, y: 15, z: 0.1 };
    const p2 = { x: 25.981, y: 15, z: -0.1 };

    const r = doesLineSegmentCollideWithPlane(plane, { p1, p2 });
    expect(r).toBeCloseToStructure({ x: 25.981, y: 15, z: 0 });
  });
});
