// Geometry Comparison Tool
// Compares game-extracted geometry with Three.js generated geometry
// This helps identify EXACTLY where the differences are

import * as THREE from 'three';

interface ExtractedGeometry {
  vertexCount: number;
  triangleCount: number;
  positions: Float32Array;
  uvs: Float32Array;
  metadata: {
    bounds: {
      x: { min: number; max: number };
      y: { min: number; max: number };
      z: { min: number; max: number };
    };
    uvBounds: {
      u: { min: number; max: number };
      v: { min: number; max: number };
    };
  };
}

interface ComparisonResult {
  match: boolean;
  differences: string[];
  vertexMatches: number;
  vertexMismatches: number;
  uvMatches: number;
  uvMismatches: number;
  positionTolerance: number;
  uvTolerance: number;
}

export function compareGeometry(
  gameData: ExtractedGeometry,
  threeJsGeometry: THREE.BufferGeometry,
  options: {
    positionTolerance?: number;
    uvTolerance?: number;
    verbose?: boolean;
  } = {}
): ComparisonResult {
  const positionTolerance = options.positionTolerance ?? 0.001;
  const uvTolerance = options.uvTolerance ?? 0.001;
  const verbose = options.verbose ?? false;

  const result: ComparisonResult = {
    match: true,
    differences: [],
    vertexMatches: 0,
    vertexMismatches: 0,
    uvMatches: 0,
    uvMismatches: 0,
    positionTolerance,
    uvTolerance,
  };

  // Get Three.js data
  const threePositions = threeJsGeometry.getAttribute('position');
  const threeUVs = threeJsGeometry.getAttribute('uv');

  // Compare counts
  if (!threePositions) {
    result.match = false;
    result.differences.push('❌ Three.js geometry has no position attribute');
    return result;
  }

  if (!threeUVs) {
    result.match = false;
    result.differences.push('❌ Three.js geometry has no UV attribute');
    return result;
  }

  const threeVertexCount = threePositions.count;
  const gameVertexCount = gameData.vertexCount;

  console.log('\n=== GEOMETRY COMPARISON ===\n');
  console.log(`Game vertices: ${gameVertexCount}`);
  console.log(`Three.js vertices: ${threeVertexCount}`);
  console.log(`Tolerances: position=${positionTolerance}, uv=${uvTolerance}\n`);

  if (threeVertexCount !== gameVertexCount) {
    result.match = false;
    result.differences.push(
      `⚠️  Vertex count mismatch: Game=${gameVertexCount}, Three.js=${threeVertexCount}`
    );
  }

  // Compare positions and UVs
  const minCount = Math.min(threeVertexCount, gameVertexCount);

  for (let i = 0; i < minCount; i++) {
    // Compare positions
    const gameX = gameData.positions[i * 3];
    const gameY = gameData.positions[i * 3 + 1];
    const gameZ = gameData.positions[i * 3 + 2];

    const threeX = threePositions.getX(i);
    const threeY = threePositions.getY(i);
    const threeZ = threePositions.getZ(i);

    const positionDist = Math.sqrt(
      Math.pow(gameX - threeX, 2) +
        Math.pow(gameY - threeY, 2) +
        Math.pow(gameZ - threeZ, 2)
    );

    if (positionDist < positionTolerance) {
      result.vertexMatches++;
    } else {
      result.vertexMismatches++;
      if (verbose || result.vertexMismatches <= 10) {
        const msg = `Vertex ${i} position: Game=(${gameX.toFixed(2)}, ${gameY.toFixed(2)}, ${gameZ.toFixed(2)}) vs Three.js=(${threeX.toFixed(2)}, ${threeY.toFixed(2)}, ${threeZ.toFixed(2)}) [dist=${positionDist.toFixed(4)}]`;
        result.differences.push(msg);
        if (verbose) console.log(`❌ ${msg}`);
      }
    }

    // Compare UVs
    const gameU = gameData.uvs[i * 2];
    const gameV = gameData.uvs[i * 2 + 1];

    const threeU = threeUVs.getX(i);
    const threeV = threeUVs.getY(i);

    const uvDist = Math.sqrt(Math.pow(gameU - threeU, 2) + Math.pow(gameV - threeV, 2));

    if (uvDist < uvTolerance) {
      result.uvMatches++;
    } else {
      result.uvMismatches++;
      if (verbose || result.uvMismatches <= 10) {
        const msg = `Vertex ${i} UV: Game=(${gameU.toFixed(4)}, ${gameV.toFixed(4)}) vs Three.js=(${threeU.toFixed(4)}, ${threeV.toFixed(4)}) [dist=${uvDist.toFixed(6)}]`;
        result.differences.push(msg);
        if (verbose) console.log(`❌ ${msg}`);
      }
    }
  }

  // Summary
  console.log('\n=== COMPARISON RESULTS ===\n');

  if (result.vertexMismatches === 0) {
    console.log(`✅ All ${result.vertexMatches} vertex positions match!`);
  } else {
    console.log(
      `❌ Position mismatches: ${result.vertexMismatches}/${minCount} (${((result.vertexMismatches / minCount) * 100).toFixed(1)}%)`
    );
    result.match = false;
  }

  if (result.uvMismatches === 0) {
    console.log(`✅ All ${result.uvMatches} UVs match!`);
  } else {
    console.log(
      `❌ UV mismatches: ${result.uvMismatches}/${minCount} (${((result.uvMismatches / minCount) * 100).toFixed(1)}%)`
    );
    result.match = false;
  }

  // Show first few differences
  if (result.differences.length > 0 && !verbose) {
    console.log('\nFirst differences:');
    result.differences.slice(0, 10).forEach((diff) => console.log(`  ${diff}`));
    if (result.differences.length > 10) {
      console.log(`  ... and ${result.differences.length - 10} more`);
    }
  }

  console.log('');

  return result;
}

// Helper to analyze UV patterns
export function analyzeUVPattern(geometry: THREE.BufferGeometry): void {
  const uvs = geometry.getAttribute('uv');
  if (!uvs) {
    console.log('❌ No UV attribute found');
    return;
  }

  let minU = Infinity,
    maxU = -Infinity;
  let minV = Infinity,
    maxV = -Infinity;

  for (let i = 0; i < uvs.count; i++) {
    const u = uvs.getX(i);
    const v = uvs.getY(i);

    minU = Math.min(minU, u);
    maxU = Math.max(maxU, u);
    minV = Math.min(minV, v);
    maxV = Math.max(maxV, v);
  }

  const rangeU = maxU - minU;
  const rangeV = maxV - minV;

  console.log('\n=== UV ANALYSIS ===\n');
  console.log(`U range: [${minU.toFixed(4)}, ${maxU.toFixed(4)}] (span: ${rangeU.toFixed(4)})`);
  console.log(`V range: [${minV.toFixed(4)}, ${maxV.toFixed(4)}] (span: ${rangeV.toFixed(4)})`);

  const estimatedScaleU = Math.round(rangeU);
  const estimatedScaleV = Math.round(Math.abs(rangeV));

  console.log(`\nEstimated tiling: ${estimatedScaleU}x${estimatedScaleV}`);

  if (Math.abs(estimatedScaleU - 3) < 0.1 && Math.abs(estimatedScaleV - 3) < 0.1) {
    console.log('Pattern: ✅ Editor_Cross-like (3x3 tiling)');
  } else if (Math.abs(estimatedScaleU - 1) < 0.1 && Math.abs(estimatedScaleV - 1) < 0.1) {
    console.log('Pattern: ✅ Board-like (1x1 tiling)');
  } else {
    console.log('Pattern: ⚠️  Unknown pattern');
  }

  console.log('');
}

// Helper to find duplicate vertices (for UV seaming analysis)
export function findDuplicateVertices(
  geometry: THREE.BufferGeometry,
  positionTolerance: number = 0.001
): Array<{ index1: number; index2: number; uvDiff: number }> {
  const positions = geometry.getAttribute('position');
  const uvs = geometry.getAttribute('uv');
  const duplicates: Array<{ index1: number; index2: number; uvDiff: number }> = [];

  if (!positions || !uvs) return duplicates;

  // Only check first 100 vertices to avoid O(n²) explosion
  const checkCount = Math.min(100, positions.count);

  for (let i = 0; i < checkCount; i++) {
    const x1 = positions.getX(i);
    const y1 = positions.getY(i);
    const z1 = positions.getZ(i);
    const u1 = uvs.getX(i);
    const v1 = uvs.getY(i);

    for (let j = i + 1; j < checkCount; j++) {
      const x2 = positions.getX(j);
      const y2 = positions.getY(j);
      const z2 = positions.getZ(j);
      const u2 = uvs.getX(j);
      const v2 = uvs.getY(j);

      const posDist = Math.sqrt(
        Math.pow(x2 - x1, 2) + Math.pow(y2 - y1, 2) + Math.pow(z2 - z1, 2)
      );
      const uvDist = Math.sqrt(Math.pow(u2 - u1, 2) + Math.pow(v2 - v1, 2));

      if (posDist < positionTolerance && uvDist > positionTolerance) {
        duplicates.push({ index1: i, index2: j, uvDiff: uvDist });
      }
    }
  }

  return duplicates;
}

// Helper to export Three.js geometry in same format as game extraction
export function exportThreeJsGeometry(
  geometry: THREE.BufferGeometry
): ExtractedGeometry {
  const positions = geometry.getAttribute('position');
  const uvs = geometry.getAttribute('uv');

  if (!positions || !uvs) {
    throw new Error('Geometry missing position or UV attributes');
  }

  const posArray = new Float32Array(positions.count * 3);
  const uvArray = new Float32Array(uvs.count * 2);

  let minX = Infinity,
    maxX = -Infinity;
  let minY = Infinity,
    maxY = -Infinity;
  let minZ = Infinity,
    maxZ = -Infinity;
  let minU = Infinity,
    maxU = -Infinity;
  let minV = Infinity,
    maxV = -Infinity;

  for (let i = 0; i < positions.count; i++) {
    const x = positions.getX(i);
    const y = positions.getY(i);
    const z = positions.getZ(i);
    const u = uvs.getX(i);
    const v = uvs.getY(i);

    posArray[i * 3] = x;
    posArray[i * 3 + 1] = y;
    posArray[i * 3 + 2] = z;

    uvArray[i * 2] = u;
    uvArray[i * 2 + 1] = v;

    minX = Math.min(minX, x);
    maxX = Math.max(maxX, x);
    minY = Math.min(minY, y);
    maxY = Math.max(maxY, y);
    minZ = Math.min(minZ, z);
    maxZ = Math.max(maxZ, z);
    minU = Math.min(minU, u);
    maxU = Math.max(maxU, u);
    minV = Math.min(minV, v);
    maxV = Math.max(maxV, v);
  }

  return {
    vertexCount: positions.count,
    triangleCount: positions.count / 3,
    positions: posArray,
    uvs: uvArray,
    metadata: {
      bounds: {
        x: { min: minX, max: maxX },
        y: { min: minY, max: maxY },
        z: { min: minZ, max: maxZ },
      },
      uvBounds: {
        u: { min: minU, max: maxU },
        v: { min: minV, max: maxV },
      },
    },
  };
}
