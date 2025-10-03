# CRITICAL ANALYSIS: Board vs Editor_Cross Rendering

## Executive Summary

**The Problem:** Editor_Cross renders correctly with 3x3 tiling, Board does not render correctly.

**Root Cause Hypothesis:** The game uses **DIFFERENT MappingInfo parameters** for different mesh types. Board uses 1x1 tiling, Editor_Cross uses 3x3 tiling.

**Status:** We have extracted actual game data showing this difference, but the code still tries to auto-detect instead of using the correct per-mesh parameters.

---

## Evidence Analysis

### 1. Extracted Game Data Comparison

#### Editor_Cross (WORKING):
```
Dimensions: ~100.5 x 100.5 x 100.5 (cube)
UV Ranges:  U=[0.001, 2.999], V=[-2.999, -0.001]
Parameters: scale_u=3.0, scale_v=3.0, offset_v=-3.0
Result:     3x3 texture tiling ✅
```

#### Board_lod0 (NOT WORKING):
```
Dimensions: 148.5 x 36.7 x 5.9 (elongated 4:1 ratio)
UV Ranges:  U=[0.002, 0.999], V=[-0.993, -0.017]
Parameters: scale_u=1.0, scale_v=1.0, offset_v=-1.0
Result:     1x1 texture tiling (NOT 3x3!) ❌
```

**CRITICAL FINDING:** The board uses **completely different UV parameters** than Editor_Cross!

### 2. The MappingInfo Structure

From `sr.c` line 61200, the MappingInfo constructor:
```c
MappingInfo(e_axis axis1, e_axis axis2, float scale_u, float scale_v, float offset_u, float offset_v)
{
  *(e_axis *)this = axis1;         // offset +0x00
  *(e_axis *)(this + 4) = axis2;   // offset +0x04
  *(float *)(this + 8) = scale_u;  // offset +0x08
  *(float *)(this + 0xc) = scale_v;  // offset +0x0C
  *(float *)(this + 0x10) = offset_u; // offset +0x10
  *(float *)(this + 0x14) = offset_v; // offset +0x14
}
```

This structure is **PASSED TO** `planarMap()` and determines the UV generation for each mesh!

### 3. The planarMap Algorithm

From `sr.c` line 64537:
```c
void srModeler::planarMap(long param_1, long param_2, MappingInfo *param_3)
{
  // Get bounding box along the two axes from MappingInfo
  getAxialBounds(this, *(e_axis *)param_3, &local_10, &local_8);
  getAxialBounds(this, *(e_axis *)(param_3 + 4), &param_3, &local_4);
  
  // Calculate scale factors
  if (local_8 - local_10 == 0.0) {
    local_c = 0.0;
  } else {
    local_c = *(float *)(param_3 + 8) / (local_8 - local_10);  // scale_u / range_u
  }
  
  if (local_4 - (float)param_3 == 0.0) {
    local_14 = 0.0;
  } else {
    local_14 = *(float *)(param_3 + 0xc) / (local_4 - (float)param_3);  // scale_v / range_v
  }
  
  // For each vertex:
  *pfVar5 = (vertex_u - local_10) * local_c + *(float *)(param_3 + 0x10);  // offset_u
  pfVar5[1] = (-vertex_v - (float)param_3) * local_14 + *(float *)(param_3 + 0x14);  // offset_v
}
```

**KEY INSIGHT:** The algorithm is **NOT** auto-detecting parameters! It receives a `MappingInfo` structure that **already contains** the scale and offset values!

---

## The Real Problem

### Current Implementation (WRONG):
```typescript
// Auto-detect based on aspect ratio
if (aspectRatio > 3.0) {
  mapping_scale_u = 1.0;
  mapping_scale_v = 1.0;
} else {
  mapping_scale_u = 3.0;
  mapping_scale_v = 3.0;
}
```

This tries to **guess** the parameters based on mesh shape.

### What the Game Actually Does:
```c
// Each mesh has its OWN MappingInfo stored with the material/mesh data
MappingInfo info = getMappingInfoForMesh(meshName);
planarMap(layer, stage, &info);
```

The game **looks up** the correct parameters for each specific mesh!

---

## Where Are These Parameters Stored?

### Hypothesis 1: In the Material Definition
- Each material (texture) has associated MappingInfo
- Board uses "Burton Custom" material → 1x1 tiling
- Editor_Cross uses "checker" material → 3x3 tiling

### Hypothesis 2: In the Mesh Metadata
- HX file MIGHT contain MappingInfo (we haven't found it yet)
- Or it's in a separate material/config file

### Hypothesis 3: Hardcoded Per-Mesh-Name
- Game has a lookup table: `meshName → MappingInfo`
- "board_lod0" → {scale: 1.0, offset: -1.0}
- "Editor_Cross" → {scale: 3.0, offset: -3.0}

---

## What We Need To Find

### Option A: Extract MappingInfo from Game Memory

**Where to look:**
1. Set breakpoint at `planarMap` (sr.c line 64537)
2. Inspect `param_3` (the MappingInfo* pointer)
3. Read the 6 values at offsets: 0x00, 0x04, 0x08, 0x0C, 0x10, 0x14
4. Do this for BOTH Editor_Cross AND board_lod0

**Expected output:**
```
Editor_Cross MappingInfo:
  axis1 = 0 (X)
  axis2 = 1 (Y)
  scale_u = 3.0
  scale_v = 3.0
  offset_u = 0.0
  offset_v = -3.0

Board_lod0 MappingInfo:
  axis1 = 0 (X)
  axis2 = 1 (Y)
  scale_u = 1.0
  scale_v = 1.0
  offset_u = 0.0
  offset_v = -1.0
```

### Option B: Search for MappingInfo in HX File

**Strategy:**
1. Look for float sequences in HX file
2. Search for pattern: `[axis1, axis2, scale_u, scale_v, offset_u, offset_v]`
3. For board.hx, search for: `[0, 1, 1.0, 1.0, 0.0, -1.0]` or similar
4. For Editor_Cross.HX, search for: `[0, 1, 3.0, 3.0, 0.0, -3.0]`

### Option C: Find Material Files

**Look for:**
- `.mat` files
- `.cfg` files
- Database/table files with texture mappings
- Could be in game data folder or packed archives

---

## Immediate Action Plan

### Step 1: Extract MappingInfo from Game Memory (HIGHEST PRIORITY)

**Script needed:**
```lua
-- extractMappingInfo.lua
-- Run when breakpoint hits at planarMap

local mappingInfoPtr = getRegisters().ECX  -- or appropriate register
local axis1 = readInteger(mappingInfoPtr + 0x00)
local axis2 = readInteger(mappingInfoPtr + 0x04)
local scale_u = readFloat(mappingInfoPtr + 0x08)
local scale_v = readFloat(mappingInfoPtr + 0x0C)
local offset_u = readFloat(mappingInfoPtr + 0x10)
local offset_v = readFloat(mappingInfoPtr + 0x14)

print("MappingInfo:")
print("  axis1 = " .. axis1)
print("  axis2 = " .. axis2)
print("  scale_u = " .. scale_u)
print("  scale_v = " .. scale_v)
print("  offset_u = " .. offset_u)
print("  offset_v = " .. offset_v)
```

**Do this for:**
1. Editor_Cross (should match our working values)
2. Board_lod0 (should match extracted UV ranges)
3. Other meshes (keith, obstacles, etc.)

### Step 2: Compare Render Logic

**Key Question:** Does the game call `planarMap` with the SAME algorithm for both meshes?

**How to verify:**
1. Set breakpoint at `planarMap` entry
2. Step through algorithm for both meshes
3. Confirm it's the exact same code path
4. Only difference should be the MappingInfo values

### Step 3: Update Code with Correct Parameters

Once we have the real MappingInfo values, update the code:

```typescript
// Per-mesh parameter lookup table
const MESH_MAPPING_INFO: Record<string, { scale_u: number, scale_v: number, offset_v: number }> = {
  'board': { scale_u: 1.0, scale_v: 1.0, offset_v: -1.0 },
  'board_lod0': { scale_u: 1.0, scale_v: 1.0, offset_v: -1.0 },
  'Editor_Cross': { scale_u: 3.0, scale_v: 3.0, offset_v: -3.0 },
  // Add more as we discover them
};

// Use mesh name or detect from file
const params = MESH_MAPPING_INFO[meshName] || { scale_u: 1.0, scale_v: 1.0, offset_v: -1.0 };
```

---

## Debugging Questions to Answer

### Q1: Are BOTH meshes using planar projection (not box mapping)?
**Answer:** Based on `boardLod0CompleteData.ts` notes, board uses 1x1 tiling which suggests **single planar projection**, not box mapping.

### Q2: Do both use the same projection axes (XY)?
**Answer:** Need to extract axis1/axis2 from MappingInfo to confirm.

### Q3: Is the V-negation applied to both?
**Answer:** Yes, both have negative V coordinates in extracted data. Algorithm uses `-vertex_v` for both.

### Q4: Are the scale factors per-axis or uniform?
**Answer:** MappingInfo has separate scale_u and scale_v, but evidence suggests both are equal for each mesh (3.0/3.0 or 1.0/1.0).

### Q5: Is offset calculated or fixed?
**Answer:** Fixed in MappingInfo structure. offset_u=0.0, offset_v varies (-3.0 for Editor_Cross, -1.0 for board).

---

## Why Our Current Code Fails

### Problem 1: Auto-Detection is Wrong
We try to detect elongated meshes, but that's not how the game works. The game has EXPLICIT parameters per mesh/material.

### Problem 2: We Mix Box Mapping with Planar
Editor_Cross might use box mapping (different projection per face), but board uses single planar projection. We need to handle both!

### Problem 3: Missing Parameter Source
Without the real MappingInfo, we're guessing. Need actual values from game.

---

## Success Criteria

✅ **Phase 1:** Extract MappingInfo for both Editor_Cross and board_lod0
✅ **Phase 2:** Confirm render logic is identical (only MappingInfo differs)
✅ **Phase 3:** Implement per-mesh parameter lookup
✅ **Phase 4:** Both meshes render correctly with proper texturing

---

## Recommendation

**STOP trying to fix the UV generation algorithm.** It's probably already correct!

**START extracting the actual MappingInfo parameters from the game.**

The algorithm is working fine - we just need to feed it the correct input parameters that the game uses for each specific mesh.

**Next Step:** Create Cheat Engine script to extract MappingInfo at `planarMap` breakpoint, run it for both meshes, compare results.
