# UV Mapping Investigation - Master Index

**📁 This file is now located in `aiScripts/` folder**  
**🆕 See [README.md](README.md) for quick navigation**

---

This document links all investigation files in logical order to help future AI assistants understand the debugging process.

## ✅ Editor_Cross - SOLVED

**Result:** 3x3 checkerboard tiling working perfectly!

**Solution:**
- Box mapping (per-face projection based on triangle normal)
- scale_u = 3.0, scale_v = 3.0
- offset_u = -1.0, offset_v = -3.0
- UV ranges: U=[0,3], V=[-3,0] (negative V critical!)

## 🔄 Board - IN PROGRESS

**Status:** Works but not verified against game data

**Next Task:** Extract board UV data from game memory using `extractBoardUVs.lua`

**Hypothesis:** Different mesh types may use different UV parameters or algorithms

---

## Problem Statement

Editor_Cross mesh from Supreme Snowboarding displays with incorrect textures:
- About 1/3 of faces show proper checkered pattern
- About 2/3 of faces show solid colors or two colors (but not checkered)
- Goal: Understand how the game generates/stores UVs and replicate it

## Investigation Workflow

### Phase 1: Initial Analysis
**File:** `UV_COORDINATE_SYSTEM_FINDINGS.md`
**Purpose:** Discovered coordinate system differences between game and Three.js
**Key Findings:**
- Game uses Y-DOWN coordinate system
- Three.js uses Y-UP coordinate system
- Game UVs have NEGATIVE V coordinates (range: V ∈ [-3, 0])
- Requires RepeatWrapping in Three.js
- Initial parameters: scale=3.0, offset_u=-1.0, offset_v=-3.0

**Status:** ✅ Coordinate system understood, but not enough to fix Editor_Cross

---

### Phase 2: Box Mapping Implementation
**File:** `BOX_MAPPING_IMPLEMENTATION.md`
**Purpose:** Implemented box mapping (per-face projection selection)
**Key Findings:**
- Editor_Cross is a 3D cross shape, not flat like the board
- Different faces need different projections (XY, XZ, or YZ)
- Each triangle selects projection based on its normal vector
- Implemented algorithm to choose projection per-face

**Status:** ⚠️ Partial improvement, but still 2/3 of faces wrong

---

### Phase 3: UV Seaming Discovery
**File:** `UV_SEAMING_DEBUG.md`
**Purpose:** Discovered why same position needs different UVs
**Key Findings:**
- HX file: 32 unique vertex positions
- Game memory: 96 vertices (3x expansion via vertex duplication)
- **UV SEAMING:** Same 3D position can have multiple UV values depending on face
- Example: pos=(16.75, 16.75, 0.0) → UV=(1.0, -1.0) for front face
- Example: pos=(16.75, 16.75, 0.0) → UV=(1.0, -2.0) for different face orientation
- Game pre-bakes UVs via vertex duplication, doesn't calculate at runtime

**Status:** 🔄 Currently investigating - this may be why our procedural UVs fail

---

### Phase 4: Source Code Analysis
**Key Files:**
- `decompiled/sr.c` - Main rendering engine source
- `decompiled/sr.asm` - Assembly for low-level verification

**Important Functions:**
1. **`srModeler::planarMap`** (sr.c line 64537)
   - UV generation formula:
     ```c
     U = (vertex[axis_u] - min_u) * scale_u + offset_u
     V = (-vertex[axis_v] - min_v) * scale_v + offset_v
     ```
   - ⚠️ **NOT CALLED for file-loaded meshes!** (verified via Cheat Engine)

2. **`srModeler::convert`** (sr.c line 64849)
   - Converts srModeler data to srMeshModel
   - UV copying code (lines 65173-65185):
     ```c
     *(undefined4 *)psVar17 = *(undefined4 *)(iVar5 + iVar27);
     *(undefined4 *)(psVar17 + 4) = *(undefined4 *)(iVar5 + 4 + iVar27);
     ```
   - **COPIES pre-baked UVs from file, doesn't generate them**

3. **`srMeshModel` structure:**
   - +0x13C: UV array pointer
   - +0x1DC: Vertex position array pointer
   - +0x230: Triangle count
   - +0x234: Vertex count

**Status:** ✅ Confirmed UVs are pre-baked, not calculated via planarMap

---

### Phase 5: Memory Extraction
**Files:**
- `dumpUVs.lua` - Extract UV coordinates from game memory
- `dumpVerticesAndUVs.lua` - Extract both positions and UVs
- `dumpCompleteGeometry.lua` - Full geometry dump
- `findIndices.lua` - Search for triangle index buffer (failed)
- **NEW:** `verifyUVData.lua` - Verify UV extraction is correct
- **NEW:** `extractUVsVerified.lua` - Extract with position matching

**Extracted Data:**
- `editorCrossCompleteData.ts` - 96 vertices with pre-baked UVs from game memory

**Problem:** 
- ✅ Have 96 vertices with correct positions
- ✅ Have 96 UV coordinates from game
- ❌ **NO triangle indices** - can't render without knowing which vertices form triangles!

**Current Concern:**
- Need to VERIFY the extracted UV data is actually correct
- UV values might be wrong if:
  - Reading from wrong memory offset
  - Wrong data structure interpretation
  - Wrong vertex/UV array pairing

**Status:** 🔄 Need to re-verify UV extraction using new Lua scripts

---

### Phase 6: Current Problem Analysis

**What We Know:**
1. Game doesn't use `planarMap` for Editor_Cross (confirmed via debugging)
2. UVs are pre-baked in the file data
3. Game expands 32 vertices → 96 vertices via duplication for UV seaming
4. We have 32 vertices + 60 triangles from HX file
5. We extracted 96 vertices + 96 UVs from game memory
6. We DON'T have triangle indices for the 96-vertex mesh

**What We're Doing:**
- Using HX file's 32 vertices + 60 triangle indices
- Converting to non-indexed mesh (180 vertices via `toNonIndexed()`)
- Generating UVs procedurally via box mapping
- Coordinate system: Flipping Y-axis for Y-down → Y-up conversion

**Current Output vs Expected:**
```
Our output:     V0: pos=(16.8, 16.8, -0.0) → UV=(1.000, 1.000)
Game memory:    V0: pos=(16.8, 16.8,  0.0) → UV=(1.000, -1.000)
                                                      ^^^^^^^^^ POSITIVE instead of NEGATIVE!
```

**Why This Matters:**
- Positive V vs Negative V changes which part of texture is sampled
- With RepeatWrapping:
  - UV=1.0 wraps to 0.0 (top of texture)
  - UV=-1.0 wraps to 0.0 differently (may sample different tile)

**Possible Issues:**
1. **UV extraction is wrong** - Reading wrong data from memory
2. **Offset calculation is wrong** - `offset_v` should make V negative
3. **Box mapping formula is wrong** - Not matching game's actual algorithm
4. **Coordinate flip is wrong** - Y-axis flip not applied correctly
5. **Scale factors are wrong** - Different axes need different scales?

---

## Next Steps for Investigation

### Step 1: Verify UV Data is Correct ⚠️ **DO THIS FIRST!**
**Action:** Run the new Lua scripts in Cheat Engine
1. Load Editor_Cross in game
2. Set breakpoint at `getModelByName`
3. Get `srMeshModel` pointer from EAX register
4. Update `modelPtr` in `verifyUVData.lua` and `extractUVsVerified.lua`
5. Run scripts to verify UV data

**What to Check:**
- Do UV values look reasonable? (Should be in range [-3, 3])
- Do positions match the HX file data?
- For vertices at the SAME position, do they have DIFFERENT UVs?

If UV data looks wrong, the entire `editorCrossCompleteData.ts` file is suspect!

### Step 2: Compare Procedural vs Memory UVs
**Action:** Add debugging to show both calculated and expected UVs side-by-side

**Example:**
```typescript
console.log(`V0: pos=(${x},${y},${z})`);
console.log(`  Calculated: UV=(${u_calc}, ${v_calc})`);
console.log(`  Expected:   UV=(${u_game}, ${v_game})`);
console.log(`  Difference: ΔU=${Math.abs(u_calc - u_game)}, ΔV=${Math.abs(v_calc - v_game)}`);
```

This will show WHERE our algorithm diverges from game data.

### Step 3: Analyze the Divergence Pattern
Look for patterns in which UVs are wrong:
- All wrong by same amount? → Offset issue
- Wrong by varying amounts? → Scale factor issue  
- Wrong projection selection? → Box mapping logic issue
- V wrong but U correct? → Y-axis flip issue

### Step 4: Try Alternative Approaches
If procedural generation keeps failing:

**Option A:** Match game's 96-vertex structure
- Problem: We don't have triangle indices for 96 vertices
- Solution: Try to reconstruct indices by matching positions
- Use position matching to map 96 game vertices → 60 triangles from HX file

**Option B:** Use simpler projection
- Try single planar XY projection for entire mesh
- See if that works for at least SOME faces
- Compare which faces work vs fail

**Option C:** Texture coordinate debugging
- Set UVs to vertex index colors to visualize mapping
- Use gradient texture instead of checkerboard
- See which faces map to which texture regions

---

## Files Reference

### Documentation
- `UV_COORDINATE_SYSTEM_FINDINGS.md` - Coordinate system analysis
- `BOX_MAPPING_IMPLEMENTATION.md` - Box mapping algorithm
- `UV_SEAMING_DEBUG.md` - UV seaming discovery
- `UV_MAPPING_INVESTIGATION_INDEX.md` - This file

### Source Code
- `decompiled/sr.c` - Game engine source (120K lines)
- `decompiled/HMG_3DE.c` - 3D editor source
- `decompiled/Supreme_Game.c` - Main game logic

### Memory Extraction Scripts
- `verifyUVData.lua` - **NEW** - Verify UV extraction is correct
- `extractUVsVerified.lua` - **NEW** - Extract with position matching
- `dumpUVs.lua` - Extract UVs only
- `dumpVerticesAndUVs.lua` - Extract positions + UVs
- `dumpCompleteGeometry.lua` - Full geometry dump
- `findIndices.lua` - Search for triangle indices (failed)

### Data Files
- `editorCrossCompleteData.ts` - 96 vertices + UVs from memory (VERIFY THIS!)
- `../threejs/public/meshes/Editor_Cross/*.HX` - Original HX mesh file (32 verts, 60 tris)

### Implementation
- `../threejs/src/meshViewer/runMeshViewer.ts` - Three.js viewer with box mapping
- `../threejs/src/meshViewer/parseHXFile.ts` - HX file parser

---

## Key Constraints

1. **planarMap is NOT used** - Verified via Cheat Engine breakpoint debugging
2. **Coordinate systems differ** - Game is Y-down, Three.js is Y-up
3. **UVs are pre-baked** - Copied from file via `srModeler::convert`, not generated at runtime
4. **Vertex duplication for UV seaming** - 32 positions → 96 vertices in game
5. **No index buffer found** - Can't use 96-vertex mesh directly without indices

---

## Critical Questions

1. **Is the extracted UV data actually correct?** 
   - Run `verifyUVData.lua` to check
   - Compare first few UVs with expected values
   - Verify pointer offsets are right

2. **Why are our UVs positive when they should be negative?**
   - Is `offset_v` applied correctly?
   - Is the negation `-vertex_v` in the formula working?
   - Is Y-flip causing double-negation?

3. **Can we match 96 game vertices to 60 HX triangles?**
   - Try position-based matching
   - See if we can reconstruct the index buffer

4. **Is box mapping even the right approach?**
   - Maybe game uses something simpler?
   - Try single projection to test

---

## Success Criteria

✅ **Complete Success:**
- All faces show proper checkered 3x3 tiling
- Matches game's visual appearance exactly

⚠️ **Partial Success:**
- Most faces correct, few faces wrong
- Identifies which faces fail and why

❌ **Current State:**
- 1/3 faces correct (checkered)
- 2/3 faces wrong (solid or two colors)
- Don't know which algorithm/data is wrong
