# Mesh Type Hypothesis - DISPROVEN ❌

## Executive Summary

**Original Hypothesis:** The game uses different UV mapping strategies based on mesh geometry type.

**ACTUAL FINDING:** ✅ **ALL mesh types use the SAME UV parameters!**

**Evidence:**
- ✅ **Editor_Cross (cube):** scale_v=3.0, offset_v=-3.0, box mapping
- ✅ **Board (elongated):** scale_v=3.0, offset_v=-3.0, box mapping (SAME!)
- ✅ **Memory extraction confirmed:** Both meshes have identical UV parameters

## ❌ HYPOTHESIS DISPROVEN

The extracted board UV data revealed:
```
Board UV ranges:  U=[0.001, 2.999], V=[-2.999, -0.001]
Editor_Cross UVs: U=[0.001, 2.999], V=[-2.999, -0.001]
Result: IDENTICAL!
```

**Estimated UV parameters for board:**
- scale_u = 3.0 (NOT different!)
- scale_v = 3.0 (NOT 0.686!)
- offset_u = -1.0
- offset_v = -3.0 (NOT 0.080!)

## ✅ ACTUAL DISCOVERY: Universal UV Parameters + Mesh Normalization

### The Real Algorithm

**Characteristics:**
- Approximately equal dimensions in X, Y, Z (aspect ratio ~1:1:1)
- Editor_Cross: 100.5 x 100.5 x 100.5

**UV Parameters (VERIFIED WORKING):**
```typescript
mapping_scale_u = 3.0
mapping_scale_v = 3.0
offset_u = -1.0
offset_v = -3.0  // Produces negative V in range [-3, 0]
```

**UV Algorithm:**
- **Box Mapping** - Each face selects projection based on triangle normal
- XY projection: Normal points along Z
- XZ projection: Normal points along Y  
- YZ projection: Normal points along X

**Formula:**
```typescript
U = (vertex[axis_u] - min_u) * scale_factor_u + offset_u
V = (-vertex[axis_v] - min_v) * scale_factor_v + offset_v
```

**UV Ranges:**
- U: [0.001, 2.999] (~3.0 range)
- V: [-2.999, -0.001] (NEGATIVE, ~3.0 range)

**Result:** ✅ 3x3 checkerboard tiling, all faces correct

---

### Type 2: Elongated Meshes (e.g., Snowboard)

**Characteristics:**
- Highly elongated in one dimension (aspect ratio ~4:1 or greater)
- Board: Large Y dimension compared to X and Z

**Current Parameters (MAY BE WRONG):**
```typescript
mapping_scale_u = 3.0
mapping_scale_v = 0.686  // Compressed V scale
offset_u = -1.0
offset_v = ???  // Currently calculated differently
```

**Suspected Issues:**
1. **Scale factors may differ** - Board might not use 3.0 for both U and V
2. **Offset calculation may differ** - Different formula for elongated shapes?
3. **Projection may differ** - Might use single planar projection instead of box mapping?
4. **V compression** - Current code uses `vScaleMultiplier = 0.70` for boards

**Result:** ⚠️ Board works "well" but not verified against game memory data

---

## Investigation Plan

### Phase 1: Extract Board UV Data from Game Memory ✅ NEXT STEP

**Goal:** Get ground truth UV values for board mesh from game

**Action Steps:**
1. Load snowboard mesh in game (any board model)
2. Set breakpoint at `getModelByName` 
3. Extract srMeshModel pointer from EAX
4. Run `verifyUVData.lua` to check UV data quality
5. Run `extractUVsVerified.lua` to dump complete board geometry + UVs
6. Save output to `boardCompleteData.ts`

**Expected Findings:**
- Board UV ranges (U and V)
- Are V coordinates negative like Editor_Cross?
- UV distribution pattern across elongated mesh
- Any special handling for aspect ratio?

### Phase 2: Compare Board Parameters

**Compare:**
1. **UV Ranges**
   - Editor_Cross: U=[0,3], V=[-3,0]
   - Board: U=[?,?], V=[?,?]

2. **Scale Factors**
   - Editor_Cross: scale_u=3.0, scale_v=3.0
   - Board: scale_u=?, scale_v=?

3. **Offsets**
   - Editor_Cross: offset_u=-1.0, offset_v=-3.0
   - Board: offset_u=?, offset_v=?

4. **Projection Type**
   - Editor_Cross: Box mapping (XY/XZ/YZ based on normal)
   - Board: Single projection? Box mapping? Which axes?

### Phase 3: Implement Board-Specific UV Generation

Based on extracted data, implement correct UV algorithm for boards.

**Possible Outcomes:**

**Scenario A: Same Algorithm, Different Parameters**
- Both use box mapping
- Only scale/offset values differ
- Simple fix: Adjust parameters based on mesh type

**Scenario B: Different Projection Strategy**
- Board uses single planar projection (e.g., only XY)
- Editor_Cross uses box mapping
- Need mesh-type detection to choose algorithm

**Scenario C: Different Formula Entirely**
- Board uses different UV generation formula
- May need aspect-ratio-based calculations
- More complex implementation needed

---

## Mesh Type Detection

**Current Code:**
```typescript
const aspectRatio = sizeY / Math.max(sizeX, sizeZ);
const isCube = aspectRatio >= 0.8 && aspectRatio <= 1.2;

if (isCube) {
  // Editor_Cross path
  mapping_scale_v = 3.0;
} else if (aspectRatio > 3.0) {
  // Board path  
  mapping_scale_v = 0.686;
} else {
  // Other meshes
}
```

**Hypothesis:**
- This detection works for identifying mesh type
- But the UV parameters for boards may still be wrong
- Need to verify against actual game data

---

## Success Criteria

### Editor_Cross (ACHIEVED ✅)
- ✅ 3x3 checkerboard tiling visible
- ✅ All faces show correct pattern
- ✅ Matches game's visual appearance
- ✅ UV values match memory extraction

### Snowboard (TO BE VERIFIED)
- ❓ Texture tiling matches game
- ❓ No stretching or distortion
- ❓ UV values match memory extraction
- ❓ Works for multiple board models

---

## Key Learnings from Editor_Cross

1. **UV Verification is Critical**
   - Always extract and verify game's actual UV data first
   - Don't assume formula without ground truth

2. **Negative V Coordinates Matter**
   - Game uses V ∈ [-3, 0] for RepeatWrapping
   - Positive V values produce different tiling results

3. **Scale Values Drive Tiling**
   - scale=3.0 produces 3x3 tiling
   - Offset shifts the UV range to desired domain

4. **Box Mapping for 3D Objects**
   - Per-face projection selection based on normal
   - Different faces use different projection axes
   - Essential for complex 3D shapes like Editor_Cross

5. **Coordinate System Conversion**
   - Game: Y-down coordinate system
   - Three.js: Y-up coordinate system
   - Must flip Y when calculating UVs

---

## Next AI Assistant: Start Here

**Current Status:**
- ✅ Editor_Cross working perfectly (3x3 tiling, box mapping)
- ⚠️ Board works but not verified against game data
- 📋 Need to extract board UV data from game memory

**Your Task:**
1. Run board UV extraction (see Phase 1 above)
2. Compare board UVs with Editor_Cross UVs
3. Identify differences in algorithm/parameters
4. Implement board-specific UV generation
5. Verify board displays correctly

**Files You'll Need:**
- `verifyUVData.lua` - Check UV data quality
- `extractUVsVerified.lua` - Extract complete geometry
- `runMeshViewer.ts` - Main UV generation code
- This document - Hypothesis and investigation plan

**Quick Start:**
```
1. Load game, attach Cheat Engine
2. Load ANY board mesh/model
3. Breakpoint at getModelByName
4. Update modelPtr in Lua scripts
5. Run extractUVsVerified.lua
6. Save output to boardCompleteData.ts
7. Analyze and compare with Editor_Cross data
```

Good luck! 🚀
