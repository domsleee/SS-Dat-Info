# CRITICAL DISCOVERY: Mesh Normalization Before UV Mapping

## The Problem

When comparing board.hx UV generation with extracted game UVs:

**board.hx dimensions** (from HX file):
- 148.5 x 36.7 x 5.9 units

**Extracted game mesh dimensions** (from memory):
- 1.00 x 1.01 x 1.01 units

**UV Ranges:**
- Game: U=[0.001, 2.999], V=[-2.999, -0.001]
- Generated (no normalization): U=[-1.000, 2.000], V=[-5.760, 0.026]

## The Discovery

The game **NORMALIZES the mesh to ~1.0 unit size** before calculating UVs!

This explains why:
1. All extracted meshes have dimensions around 1.0 units
2. The scale factors in memory are consistent (scale_u=3.0, scale_v=3.0)
3. Our generated UVs don't match (we're using 148.5 unit dimensions!)

## The Solution

Before generating UVs, we need to:

1. **Find the maximum dimension** of the mesh
2. **Scale all vertices** so the largest dimension becomes 1.0
3. **Then apply UV generation** with scale=3.0

###

 Algorithm

```typescript
// 1. Calculate bounding box
const bbox = geometry.computeBoundingBox();
const sizeX = bbox.max.x - bbox.min.x;
const sizeY = bbox.max.y - bbox.min.y;
const sizeZ = bbox.max.z - bbox.min.z;

// 2. Find maximum dimension
const maxDim = Math.max(sizeX, sizeY, sizeZ);

// 3. Normalize scale factors
const normalized_scale_x = scaleU / (sizeX / maxDim);
const normalized_scale_y = scaleU / (sizeY / maxDim);
const normalized_scale_z = scaleV / (sizeZ / maxDim);

// Equivalent to:
const normalized_scale_x = (scaleU * maxDim) / sizeX;
const normalized_scale_y = (scaleU * maxDim) / sizeY;
const normalized_scale_z = (scaleV * maxDim) / sizeZ;
```

## Evidence

**Editor_Cross (working correctly):**
- Dimensions: All axes ~1.0 (already normalized in HX file)
- Result: Perfect 3x3 tiling

**Board.hx (not working):**
- Dimensions: 148.5 x 36.7 x 5.9 (NOT normalized)
- Max dimension: 148.5
- Needs normalization factor: 148.5

**After normalization:**
- Effective dimensions: 1.0 x 0.247 x 0.040
- This should produce correct UV ranges!

## Implementation

Change the scale factor calculation to account for normalization:

```typescript
const range_x = bbox.max.x - bbox.min.x;
const range_y = bbox.max.y - bbox.min.y;
const range_z = bbox.max.z - bbox.min.z;
const maxDim = Math.max(range_x, range_y, range_z);

// Normalize scale factors relative to largest dimension
const scale_x = range_x !== 0 ? (scaleU * maxDim) / range_x : 0.0;
const scale_y = range_y !== 0 ? (scaleU * maxDim) / range_y : 0.0;
const scale_z = range_z !== 0 ? (scaleV * maxDim) / range_z : 0.0;
```

## Testing

After implementing normalization:
1. Load board.hx in viewer
2. Check UV ranges should be: U=[0,3], V=[-3,0] (approximately)
3. Check visual appearance matches game
4. Extract UVs from same board in game memory and compare

## Next Steps

1. Update `runMeshViewer.ts` with normalized scale factors
2. Test with board.hx
3. Verify UV ranges match game extraction
4. Test with other meshes (keith, etc.)
