# UV Coordinate System Investigation - Critical Findings

**📚 [Master Investigation Index](INVESTIGATION_INDEX.md)** - See all related documents  
**📂 [aiScripts README](README.md)** - Quick navigation and reference

**Investigation Phase:** 1 - Initial Analysis  
**Next Phase:** [Box Mapping Implementation](PHASE2_BOX_MAPPING.md)

---

## Problem Statement
Editor_Cross mesh displays incorrectly with textures - no visible tiling pattern even though the game shows 3x3 tiling.

## Root Cause Analysis

### Coordinate System Differences

**Supreme Snowboarding (SurRender Engine)**:
- **LEFT-HANDED Y-DOWN** coordinate system
- X+ = Right
- Y+ = DOWN (not up!)
- Z+ = Into screen (away from camera)

**Three.js**:
- **RIGHT-HANDED Y-UP** coordinate system  
- X+ = Right
- Y+ = UP
- Z+ = Out of screen (toward camera)

### UV Generation Formula

From decompiled source (sr.c line 64585-64587):
```c
U = (vertex[axis_u] - bbox_min_u) * scale_factor_u + offset_u
V = (-vertex[axis_v] - bbox_min_v) * scale_factor_v + offset_v
```

**Critical Detail**: The V formula uses `-vertex[axis_v]` (negation!)

### Why Game UVs are NEGATIVE

In the game's Y-down coordinate system:
1. Upper parts of objects have LARGE positive Y values (Y+ = down)
2. Lower parts have SMALL or negative Y values (going up)
3. The negation `-vertex.y` flips this:
   - Upper parts: `-large_positive = large_negative` → NEGATIVE V coordinates
   - Lower parts: `-small_negative = small_positive` → Less negative V coordinates

### Memory-Extracted Data

From Editor_Cross in game memory (96 vertices, 60 triangles):
- **UV range**: U = [0.001, 2.999], V = **[-2.999, -0.001]**
- **Negative V coordinates** are the KEY!

Example vertices (Z=0 face):
```
V0: pos=(16.8, 16.8, 0.0) → UV=(1.000, -1.000)
V1: pos=(50.3, 16.8, 0.0) → UV=(1.000, -0.001)
V2: pos=(50.3, -16.8, 0.0) → UV=(2.000, -0.001)
```

### Texture Wrapping Behavior

**Three.js `RepeatWrapping`**:
- Positive UVs (0-1): No repetition needed, just clamp/wrap within [0,1]
- Positive UVs (1-3): Wraps to equivalent [0-1] range, tiles 3 times
- **NEGATIVE UVs (-3 to 0)**: Wraps by taking modulo, creates tiling!
  - UV = -1.5 → wraps to 0.5 (second tile)
  - UV = -2.5 → wraps to 0.5 (third tile)

This is why the game uses NEGATIVE V coordinates - to enable texture repetition via `RepeatWrapping`!

## Solution

### Parameters for 3x3 Tiling

**Mapping scale**: 3.0 (not 1.0!)
- `mapping_scale_u = 3.0` → creates U range [0, 3]
- `mapping_scale_v = 3.0` → creates V range for 3x vertical tiling

**Offsets**:
- `offset_u = -1.0` → shifts U range to center at [0, 3]
- `offset_v = -3.0` → shifts V range to **[-3, 0]** (NEGATIVE!)

### Complete Formula

For Three.js (Y-up) to match game (Y-down):

```typescript
// 1. Flip Y coordinate to Y-down system
const vertex_ydown = { x: vertex.x, y: -vertex.y, z: vertex.z };

// 2. Calculate scale factors
const range_u = bbox.max.x - bbox.min.x;
const range_v = bbox.max.y - bbox.min.y;
const scale_factor_u = mapping_scale_u / range_u;  // 3.0 / 100.5
const scale_factor_v = mapping_scale_v / range_v;  // 3.0 / 100.5

// 3. Apply formula with game's Y-down coordinates
const u = (vertex_ydown.x - bbox.min.x) * scale_factor_u + offset_u;  // offset_u = -1.0
const v = (-vertex_ydown.y - (-bbox.max.y)) * scale_factor_v + offset_v;  // offset_v = -3.0

// Result: U in [0, 3], V in [-3, 0]
```

### Verification

Test case (V0 from Editor_Cross):
- Three.js position: (16.75, 16.75, 0)
- Y-down position: (16.75, -16.75, 0)
- Calculated UV: (1.000, -1.000)
- Game UV: (1.000, -1.000)
- ✓ **MATCH!**

## Implementation Changes

1. **Change mapping_scale from 1.0 to 3.0**:
   ```typescript
   const mapping_scale_u = 3.0;  // Was 1.0
   const mapping_scale_v = 3.0;  // Was 1.0
   ```

2. **Update offsets to create negative V**:
   ```typescript
   const offset_u = -1.0;  // Was 0.0
   const offset_v = -3.0;  // Was (max_v + min_v) * scale_factor_v
   ```

3. **Ensure RepeatWrapping is enabled**:
   ```typescript
   texture.wrapS = THREE.RepeatWrapping;
   texture.wrapT = THREE.RepeatWrapping;
   texture.repeat.set(1, 1);  // No additional repeat - UVs handle it!
   ```

## Key Insights

1. **Coordinate system matters**: Y-down vs Y-up affects UV generation
2. **Negative V enables tiling**: RepeatWrapping in Three.js needs negative coords
3. **Scale = tiling factor**: scale=3.0 creates 3x3 tiling pattern
4. **Offsets shift range**: offset_v=-3.0 makes V negative
5. **Don't use texture.repeat**: UVs already encode the repetition!

## References

- HX_FILE_FORMAT.md - Coordinate System section (line 174)
- sr.c line 64585-64587 - UV generation formula
- editorCrossCompleteData.ts - Memory-extracted UVs showing negative V coordinates
- Three.js RepeatWrapping documentation

## Status

✅ Root cause identified: Incorrect scale (1.0 vs 3.0) and positive V coords (should be negative)
🔄 Fix in progress: Updating runMeshViewer.ts with correct parameters
⏳ Testing pending: Verify 3x3 tiling displays correctly
