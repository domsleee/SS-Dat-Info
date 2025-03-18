import { test } from "bun:test";
import { doesLineSegmentCollideWithPlane } from "./planeUtil";
import { expect } from "bun:test";
import { describe } from "bun:test";

describe("planeUtil", () => {
  test("basic test - collides", () => {
    const r = doesLineSegmentCollideWithPlane(
      {
        position: {
          x: 200,
          y: 200,
          z: 1000,
        },
        quat: {
          w: 1,
          x: 0,
          y: 0,
          z: 0,
        },
      },
      {
        p1: {
          x: 200,
          y: 200,
          z: 800,
        },
        p2: {
          x: 200,
          y: 200,
          z: 1200,
        },
      }
    );
    expect(r!.distance).toBeCloseTo(0.00);
    expect(r!.intersection).toEqual({ x: 200, y: 200, z: 1000 });
  });
  test("basic test - does not collide", () => {
    const r = doesLineSegmentCollideWithPlane(
      {
        position: {
          x: 200,
          y: 200,
          z: 1000,
        },
        quat: {
          w: 1,
          x: 0,
          y: 0,
          z: 0,
        },
      },
      {
        p1: {
          x: 200,
          y: 200,
          z: 1100,
        },
        p2: {
          x: 200,
          y: 200,
          z: 1200,
        },
      }
    );
    expect(r).toBeUndefined();
  });
});
