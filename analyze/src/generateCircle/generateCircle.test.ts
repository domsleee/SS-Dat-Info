import { describe, expect, test } from "bun:test";
import { generateCircle } from "./generateCircle";
import { eulerToQuaternion, quaternionToEuler } from "./quaternionToEular";
import "./toBeCloseToStructure";

describe("generateCircle", () => {
  test("parallel with x and y axis", () => {
    const circle = generateCircle({ name: "Audience_4", centerLoc: { x: 100, y: 200, z: 300 }, count: 4, radius: 1 });
    const offsets = circle.map((circle) => circle.offset);
    expect(offsets).toBeCloseToStructure([
      { x: 1, y: 0, z: 0 },
      { x: 0, y: 1, z: 0 },
      { x: -1, y: 0, z: 0 },
      { x: 0, y: -1, z: 0 },
    ]);
    expect(new Set(circle.map((circle) => circle.loc))).toEqual(new Set([{ x: 100, y: 200, z: 300 }]));
    expect(new Set(circle.map((circle) => circle.name))).toEqual(new Set(["Audience_4"]));
  });

  test("rotated -30 degrees about the y axis", () => {
    const quat = eulerToQuaternion({ rotX: 0, rotY: -30, rotZ: 0 });
    // look at https://claude.ai/share/14a26c8b-df5b-4ec8-843d-675a0ff4ef43
    const circle = generateCircle({
      centerLoc: { x: 0, y: 0, z: 0 },
      count: 4,
      uniform_scale: 0.2,
      quat,
    });
    const offsets = circle.map((circle) => circle.offset);
    expect(offsets).toBeCloseToStructure([
      {
        x: 77.942,
        y: 0,
        z: 45,
      },
      {
        x: 0,
        y: 90,
        z: 0,
      },
      {
        x: -77.942,
        y: 0,
        z: -45,
      },
      {
        x: 0,
        y: -90,
        z: 0,
      },
    ]);

    const rotations = circle.map((circle) => quaternionToEuler(circle.quat));
    expect(rotations).toBeCloseToStructure([
      {
        // right
        rotX: -30,
        rotY: 0,
        rotZ: 90,
      },
      {
        // bottom
        rotX: 0,
        rotY: 30,
        rotZ: 180,
      },
      {
        rotX: 30,
        rotY: 0,
        rotZ: -90,
      },
      {
        // top
        rotX: 0,
        rotY: -30,
        rotZ: 0,
      },
    ]);
  });

  test("rotated 30 degrees about the x axis", () => {
    const quat = eulerToQuaternion({ rotX: 30, rotY: 0, rotZ: 0 });
    const circle = generateCircle({
      centerLoc: { x: 0, y: 0, z: 0 },
      count: 4,
      uniform_scale: 0.2,
      radius: 30,
      quat,
    });
    const offsets = circle.map((circle) => circle.offset);
    expect(offsets).toBeCloseToStructure([
      { x: 30, y: 0, z: 0 },
      { x: 0, y: 25.981, z: 15 },
      { x: -30, y: 0, z: 0 },
      { x: 0, y: -25.981, z: -15 },
    ]);

    const rotations = circle.map((circle) => quaternionToEuler(circle.quat));
    expect(rotations).toBeCloseToStructure([
      {
        // right
        rotX: 0,
        rotY: -30,
        rotZ: 90,
      },
      {
        // bottom
        rotX: -30,
        rotY: 0,
        rotZ: 180,
      },
      {
        // left
        rotX: 0,
        rotY: 30,
        rotZ: -90,
      },
      {
        // top
        rotX: 30,
        rotY: 0,
        rotZ: 0,
      },
    ]);
  });

  test("rotated 30 degrees about the z axis", () => {
    const quat = eulerToQuaternion({ rotX: 0, rotY: 0, rotZ: 30 });
    const circle = generateCircle({
      centerLoc: { x: 0, y: 0, z: 0 },
      count: 4,
      uniform_scale: 0.2,
      radius: 30,
      quat,
    });
    const offsets = circle.map((circle) => circle.offset);
    expect(offsets).toBeCloseToStructure([
      { x: 25.981, y: 15, z: 0 },
      { x: -15, y: 25.981, z: 0 },
      { x: -25.981, y: -15, z: 0 },
      { x: 15, y: -25.981, z: 0 },
    ]);

    const rotations = circle.map((circle) => quaternionToEuler(circle.quat));
    expect(rotations).toBeCloseToStructure([
      {
        // right
        rotX: 0,
        rotY: 0,
        rotZ: 120,
      },
      {
        // bottom
        rotX: 0,
        rotY: 0,
        rotZ: -150,
      },
      {
        // left
        rotX: 0,
        rotY: 0,
        rotZ: -60,
      },
      {
        // top
        rotX: 0,
        rotY: 0,
        rotZ: 30,
      },
    ]);
  });

});
