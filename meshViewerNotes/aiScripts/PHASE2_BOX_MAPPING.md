# Box Mapping Implementation - Editor_Cross Fix

**📚 [Master Investigation Index](INVESTIGATION_INDEX.md)** - See all related documents  
**📂 [aiScripts README](README.md)** - Quick navigation and reference

**Investigation Phase:** 2 - Box Mapping Implementation  
**Previous Phase:** [UV Coordinate System Findings](PHASE1_COORDINATE_SYSTEM.md)  
**Next Phase:** [UV Seaming Debug](PHASE3_UV_SEAMING.md)

---

## Problem Statement

After implementing 3x3 tiling with negative V coordinates, Editor_Cross still displayed incorrectly:
- ~1/3 of faces: checkered pattern (correct)
- ~1/3 of faces: solid color (wrong)
- ~1/3 of faces: two colors but not checkered (wrong)

## Root Cause

We were applying a **SINGLE planar projection** to ALL faces, but Editor_Cross is a 3D cross shape that needs **BOX MAPPING**.

### What is Box Mapping?

Box mapping is a UV projection technique where:
1. Each triangle's normal is calculated
2. The **dominant axis** of the normal determines which projection plane to use:
   - Normal mostly along Z → use XY projection
   - Normal mostly along Y → use XZ projection  
   - Normal mostly along X → use YZ projection

This ensures each face gets the appropriate projection based on its orientation.

## Implementation

### 1. Calculate Triangle Normal (Per-Face)

```typescript
// For each triangle
const edge1_x = v1_x - v0_x;
const edge1_y = v1_y - v0_y;
const edge1_z = v1_z - v0_z;
const edge2_x = v2_x - v0_x;
const edge2_y = v2_y - v0_y;
const edge2_z = v2_z - v0_z;

// Cross product
const normal_x = edge1_y * edge2_z - edge1_z * edge2_y;
const normal_y = edge1_z * edge2_x - edge1_x * edge2_z;
const normal_z = edge1_x * edge2_y - edge1_y * edge2_x;
```

### 2. Select Projection Based on Dominant Normal Component

```typescript
const abs_nx = Math.abs(normal_x);
const abs_ny = Math.abs(normal_y);
const abs_nz = Math.abs(normal_z);

let faceProjection: 'XY' | 'XZ' | 'YZ';
if (abs_nz >= abs_nx && abs_nz >= abs_ny) {
  faceProjection = 'XY';  // Normal along Z → project onto XY plane
} else if (abs_ny >= abs_nx && abs_ny >= abs_nz) {
  faceProjection = 'XZ';  // Normal along Y → project onto XZ plane
} else {
  faceProjection = 'YZ';  // Normal along X → project onto YZ plane
}
```

### 3. Use Per-Face Projection for UV Calculation

Each triangle now uses its own projection:

```typescript
switch (faceProjection) {
  case 'XY':
    vertex_u = pos.getX(i + j);
    vertex_v = posY;  // Y-flipped for Y-down conversion
    scale_factor_u = scale_x;
    scale_factor_v = scale_y;
    min_u = bbox.min.x;
    min_v = isFlipped ? -bbox.max.y : bbox.min.y;
    break;
  case 'XZ':
    vertex_u = pos.getX(i + j);
    vertex_v = pos.getZ(i + j);
    scale_factor_u = scale_x;
    scale_factor_v = scale_z;
    min_u = bbox.min.x;
    min_v = bbox.min.z;
    break;
  case 'YZ':
    vertex_u = posY;
    vertex_v = pos.getZ(i + j);
    scale_factor_u = scale_y;
    scale_factor_v = scale_z;
    min_u = isFlipped ? -bbox.max.y : bbox.min.y;
    min_v = bbox.min.z;
    break;
}
```

### 4. Pre-Calculate Scale Factors for All Axes

Instead of calculating for one projection mode, we now calculate for ALL axes:

```typescript
const range_x = bbox.max.x - bbox.min.x;
const range_y = bbox.max.y - bbox.min.y;
const range_z = bbox.max.z - bbox.min.z;

const scale_x = range_x !== 0 ? scaleU / range_x : 0.0;
const scale_y = range_y !== 0 ? scaleU / range_y : 0.0;
const scale_z = range_z !== 0 ? scaleV / range_z : 0.0;
```

## Key Differences from Single Projection

### Before (Single Projection)
- All faces use same projection (e.g., XY only)
- Faces perpendicular to projection plane get stretched/distorted
- Only ~1/3 of Editor_Cross faces looked correct

### After (Box Mapping)
- Each face uses best projection based on its orientation
- All faces get appropriate UV mapping
- Full 3x3 tiling pattern visible on all faces

## Parameters

### Coordinate System
- Game: **Y-DOWN** (Y+ = down)
- Three.js: **Y-UP** (Y+ = up)
- Solution: Use `isFlipped = true` to flip Y when calculating UVs

### Scale Values
- `scaleU = 3.0` (for 3x horizontal tiling)
- `scaleV = 3.0` (for 3x vertical tiling)

### Offsets  
- `offset_u = -1.0` (shifts U range to [0, 3])
- `offset_v = -3.0` (shifts V range to [-3, 0])

### UV Ranges
- **U: [0, 3]** (positive, 3 tiles)
- **V: [-3, 0]** (NEGATIVE, 3 tiles)

The negative V is critical for `THREE.RepeatWrapping` to tile correctly!

## Verification

For Editor_Cross front face (Z=0) with XY projection:
- Vertex: (16.75, 16.75, 0)
- Game UV: (1.000, -1.000)
- Our calculation: ✓ Matches!

For Editor_Cross side faces (perpendicular to X/Y), box mapping automatically selects YZ or XZ projection, ensuring correct texturing.

## Results

With box mapping implemented:
- ✅ All faces of Editor_Cross get appropriate projection
- ✅ 3x3 checkered pattern visible on all faces
- ✅ No solid colors or distorted faces
- ✅ Matches game rendering

## Code Changes Summary

1. **Removed single-axis projection**: No longer use `baseMode` switch
2. **Added per-face normal calculation**: Calculate for each triangle
3. **Implemented projection selection**: Based on dominant normal axis
4. **Pre-calculated all axis scales**: `scale_x`, `scale_y`, `scale_z`
5. **Updated UV generation**: Use face-specific projection and scales
6. **Simplified controls**: Removed manual adjustment keyboard controls

## References

- `UV_COORDINATE_SYSTEM_FINDINGS.md` - Negative V coordinate discovery
- `HX_FILE_FORMAT.md` - Coordinate system documentation
- `runMeshViewer.ts` lines 240-310 - Box mapping implementation
- Editor_Cross mesh: 32 vertices, 60 triangles, needs box mapping for all 6 "arms" of the cross

## Future Improvements

- Could optimize by caching normal/projection per face instead of recalculating
- Could add seam reduction techniques (vertex duplication) like the game does
- Could implement the poly-group system (4 groups) that the game uses
