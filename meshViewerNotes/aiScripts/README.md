# AI Scripts and Investigation Documentation

This folder contains all documentation and scripts for investigating UV mapping in Supreme Snowboarding meshes.

## 📚 Start Here

**New AI Assistant? Read this first:**
1. **[HYPOTHESIS_MESH_TYPES.md](HYPOTHESIS_MESH_TYPES.md)** - Current hypothesis and next steps
2. **[INVESTIGATION_INDEX.md](INVESTIGATION_INDEX.md)** - Complete investigation history
3. **[NEXT_STEPS.md](NEXT_STEPS.md)** - Detailed action plan

## 🎯 Current Status

### ✅ Completed
- **Editor_Cross (cube mesh):** Working perfectly with 3x3 tiling
  - Uses box mapping (per-face projection)
  - scale_u=3.0, scale_v=3.0
  - offset_u=-1.0, offset_v=-3.0
  - UV range: U=[0,3], V=[-3,0] (negative V!)

### 🔄 In Progress  
- **Snowboard (elongated mesh):** Works but not verified
  - Need to extract UV data from game memory
  - Compare with Editor_Cross to find differences
  - Implement board-specific UV generation

## 📂 File Organization

### Investigation Documents (Read in Order)
1. **HYPOTHESIS_MESH_TYPES.md** - Main hypothesis about different mesh types
2. **INVESTIGATION_INDEX.md** - Complete investigation timeline
3. **NEXT_STEPS.md** - Detailed priority list for next tasks

### Phase Documentation
4. **PHASE1_COORDINATE_SYSTEM.md** - Y-up vs Y-down discovery
5. **PHASE2_BOX_MAPPING.md** - Box mapping implementation
6. **PHASE3_UV_SEAMING.md** - UV seaming and vertex duplication

### Memory Extraction Scripts (Cheat Engine Lua)
- **verifyUVData.lua** - Verify UV extraction is correct
- **extractUVsVerified.lua** - Extract complete geometry + UVs
- **dumpUVs.lua** - Quick UV dump only
- **dumpVerticesAndUVs.lua** - Dump vertices and UVs
- **dumpCompleteGeometry.lua** - Full geometry dump
- **findIndices.lua** - Search for triangle indices (failed for Editor_Cross)

### How to Use Lua Scripts
1. Load Supreme Snowboarding
2. Attach Cheat Engine to the game process
3. Set breakpoint at `getModelByName` function
4. Load the mesh you want to extract (Editor_Cross, board, etc.)
5. When breakpoint hits, get `srMeshModel` pointer from EAX register
6. Open Lua script in Cheat Engine
7. Update `modelPtr` variable at top of script with EAX value
8. Execute script
9. Copy output and save to TypeScript file

## 🔬 Investigation Phases

### Phase 1: Coordinate System Discovery
**Problem:** Editor_Cross textures completely wrong  
**Finding:** Game uses Y-down, Three.js uses Y-up. Game UVs are NEGATIVE (V ∈ [-3, 0])  
**Status:** ✅ Solved  
**Doc:** PHASE1_COORDINATE_SYSTEM.md

### Phase 2: Box Mapping Implementation
**Problem:** 2/3 of Editor_Cross faces still wrong  
**Finding:** 3D meshes need box mapping (per-face projection based on normal)  
**Status:** ✅ Solved  
**Doc:** PHASE2_BOX_MAPPING.md

### Phase 3: UV Seaming Discovery
**Problem:** Understanding why game has 96 vertices vs 32 in file  
**Finding:** Vertex duplication for UV seaming - same position, different UVs per face  
**Status:** ✅ Understood (not needed for our procedural approach)  
**Doc:** PHASE3_UV_SEAMING.md

### Phase 4: Board Investigation
**Problem:** Board works but parameters not verified  
**Next Step:** Extract board UVs from game memory  
**Status:** 🔄 In Progress  
**Doc:** HYPOTHESIS_MESH_TYPES.md

## 🎓 Key Learnings

### UV Coordinate System
- Game: LEFT-HANDED Y-DOWN (Y+ = down, Z+ = into screen)
- Three.js: RIGHT-HANDED Y-UP (Y+ = up, Z+ = out of screen)
- Must flip Y axis when calculating UVs

### Negative V Coordinates
- Game uses V ∈ [-3, 0] for 3x3 tiling
- RepeatWrapping in Three.js wraps negative coords
- Critical for proper texture repetition

### Box Mapping Algorithm
- Per-triangle projection selection based on normal
- If |nz| largest → XY projection
- If |ny| largest → XZ projection
- If |nx| largest → YZ projection

### UV Formula (from sr.c line 64585-64587)
```c
U = (vertex[axis_u] - min_u) * scale_u + offset_u
V = (-vertex[axis_v] - min_v) * scale_v + offset_v
```

Note the **negation** of vertex[axis_v]!

### Scale and Offset
- scale = tiling factor (3.0 = 3x3 tiling)
- offset shifts UV range to desired domain
- For 3x3: offset_u=-1.0, offset_v=-3.0

## 🚀 Quick Reference

### Extract UV Data from Game
```lua
-- 1. Set breakpoint at getModelByName
-- 2. Load mesh in game
-- 3. Get modelPtr from EAX
-- 4. Run this in Cheat Engine:

local modelPtr = 0xXXXXXXXX  -- Replace with EAX value
local uvArrayPtr = readInteger(modelPtr + 0x13C)
local vertexCount = readInteger(modelPtr + 0x234)

-- Read UVs
for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    print(string.format("  %.6f, %.6f,  // [%d]", u, v, i))
end
```

### srMeshModel Structure Offsets
```
+0x13C: UV array pointer (float pairs)
+0x1DC: Vertex position array pointer (float triples)
+0x230: Triangle count (int)
+0x234: Vertex count (int)
```

### Mesh Type Detection
```typescript
const aspectRatio = sizeY / Math.max(sizeX, sizeZ);
const isCube = aspectRatio >= 0.8 && aspectRatio <= 1.2;

if (isCube) {
  // Box mapping, scale=3.0
} else if (aspectRatio > 3.0) {
  // Elongated (board), scale=?
}
```

## 📊 Comparison Table

| Aspect | Editor_Cross | Board | Notes |
|--------|-------------|-------|-------|
| **Dimensions** | 100.5 x 100.5 x 100.5 | Large Y, small X/Z | Aspect ratio differs |
| **Type** | Cubic/Box | Elongated | Detected automatically |
| **Projection** | Box mapping | Unknown | Need to verify |
| **scale_u** | 3.0 | 3.0 | Assumed same |
| **scale_v** | 3.0 | 0.686? | Need to verify |
| **offset_u** | -1.0 | -1.0? | Need to verify |
| **offset_v** | -3.0 | Unknown | Need to verify |
| **UV Range U** | [0, 3] | Unknown | Need extraction |
| **UV Range V** | [-3, 0] | Unknown | Need extraction |
| **Status** | ✅ Working | ⚠️ Not verified | |

## 🎯 Next Actions

1. **Extract Board UV Data**
   - Use `extractUVsVerified.lua`
   - Save to `boardCompleteData.ts`

2. **Analyze Board UVs**
   - Compare ranges with Editor_Cross
   - Check if V is negative
   - Identify scale/offset values

3. **Implement Board UV Generation**
   - Adjust parameters based on findings
   - Test with multiple board models
   - Verify against game appearance

4. **Document Findings**
   - Update HYPOTHESIS_MESH_TYPES.md
   - Add board-specific parameters
   - Create comparison table

## 📖 Related Files

### Outside This Folder
- `../HX_FILE_FORMAT.md` - HX file format specification (kept separate)
- `../../threejs/src/meshViewer/runMeshViewer.ts` - Main UV generation code
- `../../threejs/src/meshViewer/parseHXFile.ts` - HX file parser
- `../../threejs/src/meshViewer/editorCrossCompleteData.ts` - Extracted Editor_Cross data

### Source Code References
- `../decompiled/sr.c` - SurRender engine source (120K lines)
- Line 64537: `srModeler::planarMap` function
- Line 64849: `srModeler::convert` function
- Line 65173-65185: UV copying code

## 💡 Tips for Next AI

1. **Always verify with game data** - Don't guess UV parameters
2. **Test with multiple meshes** - One working doesn't mean all work
3. **Check UV ranges** - Negative V is critical for RepeatWrapping
4. **Use Lua scripts** - Fastest way to get ground truth
5. **Document everything** - Future AI will thank you!

## 🆘 Troubleshooting

### UVs Look Wrong
- Check if V is positive (should be negative!)
- Verify scale and offset values
- Check projection mode (XY/XZ/YZ)
- Compare with extracted game UVs

### Texture Stretching
- Wrong scale factor (should be 3.0 for 3x3)
- Wrong projection axis selection
- Aspect ratio not handled correctly

### Some Faces Wrong
- Box mapping not selecting correct projection
- Check triangle normal calculation
- Verify per-face UV generation

### Cheat Engine Issues
- Make sure game is running
- Verify breakpoint address is correct
- Check srMeshModel pointer from EAX
- Verify structure offsets (+0x13C, +0x1DC, etc.)

---

**Last Updated:** After Editor_Cross fix (3x3 tiling working)  
**Next Task:** Extract and analyze board UV data  
**Maintainer:** AI Assistant Team 🤖
