# UV Seaming and Box Mapping Debug Session

**📚 [Master Investigation Index](INVESTIGATION_INDEX.md)** - See all related documents  
**📂 [aiScripts README](README.md)** - Quick navigation and reference

**Investigation Phase:** 3 - UV Seaming Discovery  
**Previous Phase:** [Box Mapping Implementation](PHASE2_BOX_MAPPING.md)  
**Status:** ✅ Editor_Cross solved! Next: [Board Investigation](HYPOTHESIS_MESH_TYPES.md)

---

## Current Status

After implementing box mapping, still seeing ~1/3 of faces with incorrect texturing (two colors instead of checkered pattern).

## Key Discovery

The game uses **UV SEAMING** via vertex duplication:
- HX file: 32 unique vertex positions
- Game memory: 96 vertices (3x expansion)
- **Same 3D position can have DIFFERENT UV coordinates** depending on which face the vertex belongs to

### Example from Memory Data

```
Vertex 0:  position=(16.75, 16.75, 0.00)    UV=(1.000, -1.000)
Vertex 15: position=(16.75, 16.75, 33.50)   UV=(1.000, -2.000)
```

These are at the SAME X,Y but:
- Different Z coordinate (front vs back of cross)
- Different V UV value (-1.000 vs -2.000)

This is intentional - vertices shared by multiple faces need different UVs per face!

## Our Implementation

We already handle this correctly by:
1. Using `toNonIndexed()` - creates 180 vertices (60 triangles × 3)
2. Calculating UVs **per-triangle** based on each triangle's normal
3. Box mapping selects projection axis based on normal

So vertices at the same position BUT in different triangles get different UVs. ✅

## Why Still Broken?

Possible issues to investigate:

### 1. Projection Selection Logic

Are we selecting the correct projection axes? Check console output:
- Should see mix of XY, XZ, and YZ projections
- Editor_Cross has 6 "arms" pointing in different directions
- Each arm should use appropriate projection

### 2. Scale/Offset Values

Current values:
- `mapping_scale = 3.0` (for 3x3 tiling)
- `offset_u = -1.0`
- `offset_v = -3.0`

These work for **one** projection type but might need adjustment for others!

**CRITICAL INSIGHT**: Different projection axes might need different offsets!
- XY projection: offset_u based on X range
- XZ projection: offset_u based on X range, offset_v based on Z range
- YZ projection: offset_u based on Y range, offset_v based on Z range

### 3. Bbox Range Selection

When we select a projection, are we using the correct bbox min/max values?

Current code:
```typescript
case 'XY':
  min_u = bbox.min.x;
  min_v = isFlipped ? -bbox.max.y : bbox.min.y;
case 'XZ':
  min_u = bbox.min.x;
  min_v = bbox.min.z;
case 'YZ':
  min_u = isFlipped ? -bbox.max.y : bbox.min.y;
  min_v = bbox.min.z;
```

### 4. Fixed Offsets Don't Work for All Axes

The problem might be here:
```typescript
const offset_u = -1.0;   // Fixed value
const offset_v = -scaleV; // = -3.0
```

But these offsets were calculated for ONE specific projection (XY with X and Y ranges)!

For XZ projection (X and Z), the ranges are DIFFERENT:
- X range: 100.5 (same)
- **Z range: 100.5** (different from Y range!)

For YZ projection:
- Y range: 100.5
- Z range: 100.5

**All ranges happen to be the same for Editor_Cross** (it's a cube!), so offsets should work...

### 5. Y-Flipping Issue

Are we handling Y-flipping consistently across all projections?

```typescript
const posY = isFlipped ? -pos.getY(i + j) : pos.getY(i + j);
```

This flips Y for use in projections. But:
- XY: uses posY for V → correct
- XZ: uses posZ for V → doesn't need flip
- YZ: uses posY for U → needs flip

Is the flip being applied correctly in all cases?

## Debug Steps

1. **Run mesh viewer** and check console output
2. **Look for pattern** in which triangles have wrong UVs
3. **Check projection distribution**:
   - How many XY, XZ, YZ?
   - Do faces facing certain directions all fail?
4. **Compare calculated UVs** with expected values from memory dump
5. **Verify offset calculation** per projection type

## Test Hypothesis

**Hypothesis**: The offset calculation needs to be different for each projection axis combination!

For XY projection:
```typescript
offset_u = -1.0  // Shifts X-based U range
offset_v = -3.0  // Shifts Y-based V range
```

For XZ projection:
```typescript
offset_u = -1.0  // Shifts X-based U range (same X)
offset_v = ???   // Should shift Z-based V range - needs different value?
```

But wait - all ranges are 100.5 for Editor_Cross, so offsets should be the same!

Unless... the game uses DIFFERENT scale values for different axes?

## Next Investigation

Check if the game uses:
- Same scale (3.0) for all axes? OR
- Different scales per axis (scale_x, scale_y, scale_z)?

From our earlier analysis:
```typescript
const scale_x = scaleU / range_x;  // 3.0 / 100.5
const scale_y = scaleU / range_y;  // 3.0 / 100.5
const scale_z = scaleV / range_z;  // 3.0 / 100.5
```

We use `scaleU` for X and Y, but `scaleV` for Z. Should all use same value?

## Action Items

1. Add more console debugging to show:
   - Triangle index
   - Normal vector
   - Selected projection
   - Calculated UV values
   - Vertex positions

2. Run and analyze output pattern

3. Compare with game's memory-extracted UVs for same triangles

4. Test if issue is:
   - Projection selection (wrong axis chosen)
   - Scale values (wrong multiplier)
   - Offset values (wrong shift)
   - Bbox range (wrong min/max used)

