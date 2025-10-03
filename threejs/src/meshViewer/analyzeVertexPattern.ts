// Analyze the vertex duplication pattern in Editor_Cross
// Compare file's 32 vertices with game's 96 vertices

import { editorCrossUVs } from './editorCrossUVData';

// Game's 96 vertex positions (from Cheat Engine dump)
const gameVertices = [
  [16.750, 16.750, -0.000],   // [0]
  [50.250, 16.750, 0.000],    // [1]
  [50.250, -16.750, 0.000],   // [2]
  [-50.250, 16.750, 0.000],   // [3]
  [-16.750, 16.750, 0.000],   // [4]
  [-16.750, -16.750, 0.000],  // [5]
  [16.750, 16.750, 33.500],   // [6]
  [16.750, -16.750, 33.500],  // [7]
  [50.250, -16.750, 33.500],  // [8]
  [-50.250, 16.750, 33.500],  // [9]
  [-50.250, -16.750, 33.500], // [10]
  [-16.750, -16.750, 33.500], // [11]
  [50.250, 16.750, 33.500],   // [12]
  [50.250, -16.750, 33.500],  // [13]
  [50.250, -16.750, 0.000],   // [14]
  [16.750, 16.750, 33.500],   // [15]
  [50.250, 16.750, 33.500],   // [16]
  [50.250, 16.750, 0.000],    // [17]
  [-50.250, 16.750, 33.500],  // [18]
  [-16.750, 16.750, 33.500],  // [19]
  [-16.750, 16.750, 0.000],   // [20]
  [-50.250, -16.750, 33.500], // [21]
  [-50.250, 16.750, 33.500],  // [22]
  [-50.250, 16.750, 0.000],   // [23]
  [-16.750, -16.750, 33.500], // [24]
  [-50.250, -16.750, 33.500], // [25]
  [-50.250, -16.750, 0.000],  // [26]
  [50.250, -16.750, 33.500],  // [27]
  [16.750, -16.750, 33.500],  // [28]
  [16.750, -16.750, -0.000],  // [29]
  [16.750, -16.750, -0.000],  // [30]
  [16.750, -50.250, 0.000],   // [31]
  // ... need all 96
];

console.log('Analyzing vertex duplication pattern...');
console.log(`File has 32 unique positions`);
console.log(`Game has 96 vertices (with UVs)`);
console.log(`Ratio: 96/32 = 3 vertices per original vertex (average)`);
console.log('');
console.log('This suggests each vertex is duplicated ~3 times for different UV seams');
console.log('This is typical for box mapping where vertices are shared between faces');
