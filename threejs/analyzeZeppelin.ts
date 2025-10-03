#!/usr/bin/env bun
/**
 * Analyze zeppelin.HX file to search for UV data
 */

import { readFileSync } from 'fs';
import { parseHXFile } from './src/meshViewer/parseHXFile';

const hxPath = './public/meshes/Zeppelin/zeppelin.HX';
const buffer = readFileSync(hxPath);

console.log(`\n=== Analyzing ${hxPath} ===`);
console.log(`File size: ${buffer.length} bytes (0x${buffer.length.toString(16)})`);

// Parse the HX file
const result = parseHXFile(buffer.buffer);
const mesh = result.meshes[0];

console.log(`\nMesh data:`);
console.log(`- Vertices: ${mesh.vertices.length / 3}`);
console.log(`- Triangles: ${mesh.indices.length / 3}`);
console.log(`- Has UVs: ${mesh.uvs ? 'YES' : 'NO'}`);

// Look at the gameExtracted data
console.log(`\n=== Comparing with gameExtracted_zeppelin.ts ===`);

// Load the gameExtracted data
const gameData = await import('./src/meshViewer/gameExtracted_zeppelin.ts');
const extracted = gameData.gameExtracted_zeppelin;

console.log(`Game-extracted data:`);
console.log(`- Vertices: ${extracted.vertexCount}`);
console.log(`- Triangles: ${extracted.triangleCount}`);
console.log(`- Has UVs: ${extracted.uvs ? 'YES' : 'NO'}`);
console.log(`- UV count: ${extracted.uvs.length / 2}`);

// Now let's manually search the HX file for UV-like data
console.log(`\n=== Searching for UV data patterns in HX file ===`);

// UV data should be floats in range roughly -5 to 5 (based on gameExtracted data)
// Let's look for sequences of floats that match the pattern

const view = new DataView(buffer.buffer);
const meshMarker = buffer.indexOf('mesh\0');
console.log(`Mesh marker at offset: ${meshMarker} (0x${meshMarker.toString(16)})`);

// After parsing indices and vertices, where would UVs be?
// From the parser, we know:
// - Indices come right after mesh marker
// - Vertices come after indices
// Let's calculate where that would end

const triangleCount = mesh.indices.length / 3;
const vertexCount = mesh.vertices.length / 3;

const indicesEnd = meshMarker + 5 + (triangleCount * 12); // mesh\0 + triangles * 12 bytes
const verticesEnd = indicesEnd + (vertexCount * 12); // vertices * 12 bytes

console.log(`\nCalculated offsets:`);
console.log(`- Mesh marker: 0x${meshMarker.toString(16)}`);
console.log(`- Indices end: 0x${indicesEnd.toString(16)} (${triangleCount} triangles)`);
console.log(`- Vertices end: 0x${verticesEnd.toString(16)} (${vertexCount} vertices)`);
console.log(`- Remaining bytes: ${buffer.length - verticesEnd}`);

// Check what comes after vertices
console.log(`\n=== Data after vertices (first 200 bytes) ===`);
const afterVertices = verticesEnd;
for (let i = 0; i < 200 && afterVertices + i < buffer.length; i += 4) {
  const float = view.getFloat32(afterVertices + i, true);
  const int = view.getInt32(afterVertices + i, true);
  console.log(`Offset 0x${(afterVertices + i).toString(16)}: float=${float.toFixed(6)}, int=${int}`);
}

// Let's search for the exact UV values from gameExtracted
console.log(`\n=== Searching for UV values from gameExtracted ===`);
const firstUV_U = extracted.uvs[0]; // Should be 0.999500
const firstUV_V = extracted.uvs[1]; // Should be -0.999500

console.log(`Looking for UV pair: (${firstUV_U}, ${firstUV_V})`);

// Search the entire file for this float value
for (let i = 0; i < buffer.length - 8; i += 4) {
  const u = view.getFloat32(i, true);
  const v = view.getFloat32(i + 4, true);
  
  if (Math.abs(u - firstUV_U) < 0.0001 && Math.abs(v - firstUV_V) < 0.0001) {
    console.log(`✓ Found first UV pair at offset 0x${i.toString(16)}: (${u}, ${v})`);
    
    // Check if more UVs follow
    console.log(`Next 10 UV pairs:`);
    for (let j = 0; j < 10; j++) {
      const idx = i + j * 8;
      const u_next = view.getFloat32(idx, true);
      const v_next = view.getFloat32(idx + 4, true);
      const expected_u = extracted.uvs[j * 2];
      const expected_v = extracted.uvs[j * 2 + 1];
      const match = Math.abs(u_next - expected_u) < 0.0001 && Math.abs(v_next - expected_v) < 0.0001;
      console.log(`  ${j}: (${u_next.toFixed(6)}, ${v_next.toFixed(6)}) ${match ? '✓' : '✗'} expected (${expected_u.toFixed(6)}, ${expected_v.toFixed(6)})`);
    }
    
    break;
  }
}
