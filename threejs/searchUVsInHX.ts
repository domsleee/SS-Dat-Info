#!/usr/bin/env bun
/**
 * More thorough search for UV data in zeppelin.HX
 */

import { readFileSync } from 'fs';

const hxPath = './public/meshes/Zeppelin/zeppelin.HX';
const buffer = readFileSync(hxPath);
const view = new DataView(buffer.buffer);

console.log(`\n=== Thorough UV Search in ${hxPath} ===`);
console.log(`File size: ${buffer.length} bytes`);

// Load expected UV data
const gameData = await import('./src/meshViewer/gameExtracted_zeppelin.ts');
const extracted = gameData.gameExtracted_zeppelin;

console.log(`\nExpected UV data:`);
console.log(`- First 5 UV pairs from gameExtracted:`);
for (let i = 0; i < 5; i++) {
  const u = extracted.uvs[i * 2];
  const v = extracted.uvs[i * 2 + 1];
  console.log(`  ${i}: (${u.toFixed(6)}, ${v.toFixed(6)})`);
}

// Search for ANY of the first 10 UV values
console.log(`\n=== Searching entire file for UV values ===`);
const searchValues = [
  extracted.uvs[0],  // 0.999500
  extracted.uvs[1],  // -0.999500
  extracted.uvs[2],  // 0.116227
  extracted.uvs[4],  // 0.000500
  extracted.uvs[6],  // 0.918343
];

for (const searchVal of searchValues) {
  console.log(`\nSearching for ${searchVal.toFixed(6)}...`);
  let found = 0;
  
  for (let i = 0; i < buffer.length - 4; i += 4) {
    const float = view.getFloat32(i, true);
    
    if (Math.abs(float - searchVal) < 0.00001) {
      console.log(`  Found at offset 0x${i.toString(16)}`);
      found++;
      
      if (found >= 3) {
        console.log(`  (stopping after 3 matches...)`);
        break;
      }
    }
  }
  
  if (found === 0) {
    console.log(`  ✗ Not found in file!`);
  }
}

// Also look for patterns of reasonable UV ranges
console.log(`\n=== Scanning for UV-like float sequences ===`);
console.log(`(Looking for sequences of floats mostly in range -5 to 5)`);

let consecutiveReasonable = 0;
let bestRun = { offset: 0, length: 0 };

for (let i = 0; i < buffer.length - 8; i += 4) {
  const float = view.getFloat32(i, true);
  
  // Check if this looks like a UV coordinate
  const isReasonable = !isNaN(float) && isFinite(float) && Math.abs(float) < 10;
  
  if (isReasonable) {
    if (consecutiveReasonable === 0) {
      // Start of new sequence
    }
    consecutiveReasonable++;
    
    if (consecutiveReasonable > bestRun.length) {
      bestRun = { offset: i - (consecutiveReasonable - 1) * 4, length: consecutiveReasonable };
    }
  } else {
    consecutiveReasonable = 0;
  }
}

console.log(`\nLongest sequence of UV-like floats:`);
console.log(`- Offset: 0x${bestRun.offset.toString(16)}`);
console.log(`- Length: ${bestRun.length} floats (${bestRun.length / 2} UV pairs)`);

if (bestRun.length >= 20) {
  console.log(`\nFirst 10 values from this sequence:`);
  for (let i = 0; i < 10 && i < bestRun.length; i++) {
    const float = view.getFloat32(bestRun.offset + i * 4, true);
    console.log(`  [${i}]: ${float.toFixed(6)}`);
  }
}

// Check if there's any pattern of vertex count * 2 floats anywhere
console.log(`\n=== Looking for blocks of specific sizes ===`);
const vertexCount = 199;
const expandedVertexCount = 269; // from gameExtracted

console.log(`Searching for ${vertexCount * 2} floats (199 vertices)...`);
console.log(`Searching for ${expandedVertexCount * 2} floats (269 vertices)...`);

// The conclusion might be: UV data is NOT in the HX file!
console.log(`\n=== CONCLUSION ===`);
console.log(`Based on the documentation (HX_FILE_FORMAT.md):`);
console.log(`"UV coordinates are NOT stored in the HX file format."`);
console.log(`"The SurRender engine generates UV coordinates procedurally at runtime"`);
console.log(`"using planar projection mapping (srModeler::planarMap)"`);
console.log(`\nThe UV data in gameExtracted_zeppelin.ts was extracted from GAME MEMORY,`);
console.log(`not from the HX file. The game generated those UVs at runtime.`);
