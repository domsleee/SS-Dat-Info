/* eslint-disable */
// Compare board UV generation with extracted game data
// This validates our UV algorithm against the real game values

import { boardLod0MeshData } from './boardLod0CompleteData';

interface UVComparisonResult {
  vertexIndex: number;
  position: { x: number; y: number; z: number };
  gameUV: { u: number; v: number };
  generatedUV: { u: number; v: number };
  difference: { u: number; v: number };
  error: number;
}

export function compareBoardUVs(
    positions: Float32Array,
    generatedUVs: Float32Array,
): {
    results: UVComparisonResult[];
    stats: {
        maxError: number;
        avgError: number;
        matchCount: number;
        totalCount: number;
    };
}
{
  const results: UVComparisonResult[] = [];
  const gamePositions = boardLod0MeshData.vertices;
  const gameUVs = boardLod0MeshData.uvs;
  
  console.log('\n=== BOARD UV COMPARISON ===');
  console.log(`Game data: ${gamePositions.length / 3} vertices`);
  console.log(`Generated: ${positions.length / 3} vertices`);
  
  // Match vertices by position
  const tolerance = 0.001;
  let totalError = 0;
  let matchCount = 0;
  
  for (let i = 0; i < positions.length / 3; i++) {
    const pos = {
      x: positions[i * 3],
      y: positions[i * 3 + 1],
      z: positions[i * 3 + 2]
    };
    
    // Find matching vertex in game data
    let bestMatch = -1;
    let bestDist = Infinity;
    
    for (let j = 0; j < gamePositions.length / 3; j++) {
      const gamePosX = gamePositions[j * 3];
      const gamePosY = gamePositions[j * 3 + 1];
      const gamePosZ = gamePositions[j * 3 + 2];
      
      const dist = Math.sqrt(
        Math.pow(pos.x - gamePosX, 2) +
        Math.pow(pos.y - gamePosY, 2) +
        Math.pow(pos.z - gamePosZ, 2)
      );
      
      if (dist < bestDist) {
        bestDist = dist;
        bestMatch = j;
      }
    }
    
    if (bestMatch !== -1 && bestDist < tolerance) {
      const gameU = gameUVs[bestMatch * 2];
      const gameV = gameUVs[bestMatch * 2 + 1];
      const genU = generatedUVs[i * 2];
      const genV = generatedUVs[i * 2 + 1];
      
      const diffU = Math.abs(genU - gameU);
      const diffV = Math.abs(genV - gameV);
      const error = Math.sqrt(diffU * diffU + diffV * diffV);
      
      results.push({
        vertexIndex: i,
        position: pos,
        gameUV: { u: gameU, v: gameV },
        generatedUV: { u: genU, v: genV },
        difference: { u: diffU, v: diffV },
        error
      });
      
      totalError += error;
      matchCount++;
    }
  }
  
  const maxError = Math.max(...results.map(r => r.error));
  const avgError = totalError / matchCount;
  
  console.log(`\nMatched: ${matchCount}/${positions.length / 3} vertices`);
  console.log(`Max error: ${maxError.toFixed(6)}`);
  console.log(`Avg error: ${avgError.toFixed(6)}`);
  
  // Show worst mismatches
  const sorted = [...results].sort((a, b) => b.error - a.error);
  console.log('\nTop 10 worst mismatches:');
  console.log('Index | Game UV              | Generated UV         | Error');
  console.log('------|---------------------|---------------------|-------');
  
  for (let i = 0; i < Math.min(10, sorted.length); i++) {
    const r = sorted[i];
    console.log(
      `[${r.vertexIndex.toString().padStart(3)}] | ` +
      `(${r.gameUV.u.toFixed(4)}, ${r.gameUV.v.toFixed(4)}) | ` +
      `(${r.generatedUV.u.toFixed(4)}, ${r.generatedUV.v.toFixed(4)}) | ` +
      `${r.error.toFixed(6)}`
    );
  }
  
  // Check UV ranges
  const genUMin = Math.min(...Array.from(generatedUVs).filter((_, i) => i % 2 === 0));
  const genUMax = Math.max(...Array.from(generatedUVs).filter((_, i) => i % 2 === 0));
  const genVMin = Math.min(...Array.from(generatedUVs).filter((_, i) => i % 2 === 1));
  const genVMax = Math.max(...Array.from(generatedUVs).filter((_, i) => i % 2 === 1));
  
  console.log('\nUV Range Comparison:');
  console.log('         | Game Data            | Generated');
  console.log('---------|---------------------|---------------------');
  console.log(
    `U range  | [${boardLod0MeshData.metadata.uvRanges.u[0].toFixed(4)}, ` +
    `${boardLod0MeshData.metadata.uvRanges.u[1].toFixed(4)}] | ` +
    `[${genUMin.toFixed(4)}, ${genUMax.toFixed(4)}]`
  );
  console.log(
    `V range  | [${boardLod0MeshData.metadata.uvRanges.v[0].toFixed(4)}, ` +
    `${boardLod0MeshData.metadata.uvRanges.v[1].toFixed(4)}] | ` +
    `[${genVMin.toFixed(4)}, ${genVMax.toFixed(4)}]`
  );
  
  // Check if V is negative
  const hasNegativeV = genVMin < 0;
  console.log(`\nGenerated V is negative: ${hasNegativeV ? '✅ YES' : '❌ NO'}`);
  
  return {
    results,
    stats: {
      maxError,
      avgError,
      matchCount,
      totalCount: positions.length / 3
    }
  };
}

// Analysis function to check parameters
export function analyzeCurrentParameters(aspectRatio: number) {
  console.log('\n=== CURRENT PARAMETER ANALYSIS ===');
  
  const isCube = true; // Current condition in runMeshViewer.ts
  const isElongated = aspectRatio > 3.0;
  
  console.log(`Aspect ratio: ${aspectRatio.toFixed(2)}:1`);
  console.log(`Cube detection: ${isCube}`);
  console.log(`Elongated detection: ${isElongated}`);
  
  if (isCube) {
    console.log('\n✅ Using CUBE parameters:');
    console.log('  scale_v = 3.0');
    console.log('  offset_v = 0.0');
    console.log('  vScaleMultiplier = 1.0');
    console.log('\n⚠️  This should work for Editor_Cross but NOT boards!');
  } else if (isElongated) {
    console.log('\n❌ Using BOARD parameters (WRONG):');
    console.log('  scale_v = 0.686');
    console.log('  offset_v = 0.080');
    console.log('  vScaleMultiplier = 0.70');
    console.log('\n⚠️  These values are INCORRECT based on extraction!');
    console.log('  Should be: scale_v = 3.0, offset_v = -3.0');
  }
  
  console.log('\n=== EXTRACTED GAME PARAMETERS ===');
  console.log('Based on board_lod0 memory extraction:');
  console.log('  scale_u = 1.0 (NOT 3.0!)');
  console.log('  scale_v = 1.0 (NOT 3.0!)');
  console.log('  offset_u = 0.0');
  console.log('  offset_v = -1.0 (NOT -3.0!)');
  console.log('  UV ranges: U=[0,1], V=[-1,0]');
  console.log('\n❌ DIFFERENT from Editor_Cross (which uses 3x3 tiling)!');
}
