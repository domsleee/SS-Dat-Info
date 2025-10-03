# Solution Summary: Why Board Renders Incorrectly

## TL;DR

**The UV generation algorithm is CORRECT.** The problem is we're using the **WRONG PARAMETERS** for board meshes.

- **Editor_Cross:** Uses `scale=3.0, offset=-3.0` → 3x3 texture tiling ✅
- **Board:** Uses `scale=1.0, offset=-1.0` → 1x1 texture tiling ❌ (currently using wrong values)

## What We Discovered

### 1. The Extracted Game Data Tells the Truth

From actual game memory extraction:

| Mesh | UV Range (U) | UV Range (V) | Scale | Offset | Tiling |
|------|-------------|-------------|-------|--------|--------|
| Editor_Cross | [0.001, 2.999] | [-2.999, -0.001] | 3.0 | -3.0 | 3x3 ✅ |
| Board_lod0 | [0.002, 0.999] | [-0.993, -0.017] | 1.0 | -1.0 | 1x1 ❌ |

**The board does NOT use 3x3 tiling!** It uses 1x1 tiling with different parameters!

### 2. The Source Code Confirms It

From `sr.c` (the actual game engine):

```c
// MappingInfo structure (line 61200)
struct MappingInfo {
  e_axis axis1;      // +0x00: Which axis for U (0=X, 1=Y, 2=Z)
  e_axis axis2;      // +0x04: Which axis for V
  float scale_u;     // +0x08: UV scale for U axis
  float scale_v;     // +0x0C: UV scale for V axis
  float offset_u;    // +0x10: UV offset for U axis
  float offset_v;    // +0x14: UV offset for V axis
};

// planarMap algorithm (line 64537)
void planarMap(..., MappingInfo *mappingInfo) {
  // Calculate scale factors from MappingInfo
  scale_u = mappingInfo->scale_u / range_u;
  scale_v = mappingInfo->scale_v / range_v;
  
  // Generate UVs
  U = (vertex_u - min_u) * scale_u + mappingInfo->offset_u;
  V = (-vertex_v - min_v) * scale_v + mappingInfo->offset_v;
}
```

**Key Point:** The `MappingInfo` structure is **PASSED TO** the function. The game doesn't auto-detect - it has explicit parameters per mesh!

### 3. Our Code is Guessing Wrong

Current implementation in `runMeshViewer.ts`:

```typescript
// WRONG: Auto-detect based on aspect ratio
if (elongationRatio > 3.0) {
  // Board-like mesh
  mapping_scale_u = 1.0;  // This is correct
  mapping_scale_v = 1.0;  // This is correct
} else {
  // Cube-like mesh
  mapping_scale_u = 3.0;  // This is correct for Editor_Cross
  mapping_scale_v = 3.0;  // This is correct for Editor_Cross
}

// But we also have this mess:
uvOffsetV = 0.080;        // WRONG for board!
vScaleMultiplier = 0.70;  // What is this even doing?
```

We're trying to be clever with auto-detection, but the game just **looks up** the correct parameters!

## The Real Questions

### Q1: "Is the render logic different between board and Editor_Cross?"

**Answer: NO!** Both use the exact same `planarMap` algorithm. The ONLY difference is the `MappingInfo` parameters passed in.

### Q2: "Why does Editor_Cross work but board doesn't?"

**Answer:** Because we got lucky! Our auto-detection picked `scale=3.0` for cube-like meshes, which happens to match Editor_Cross. But board needs `scale=1.0`, and our elongated mesh detection doesn't set it correctly.

### Q3: "Do we need more game data?"

**Answer: YES!** We need to extract the `MappingInfo` parameters for each mesh type. We can do this by setting a breakpoint at `planarMap` and reading the structure.

### Q4: "Should we use box mapping or planar mapping?"

**Answer:** Both meshes use **planar mapping** (single projection plane). Box mapping is different - it selects projection per-face. The confusion comes from our code trying to do box mapping when the game doesn't.

## What To Do Next

### Step 1: Extract Real MappingInfo (HIGH PRIORITY)

Use the provided `extractMappingInfo.lua` script:

1. Load Supreme Snowboarding
2. Attach Cheat Engine
3. Set breakpoint at `sr.dll+0x43E70` (planarMap function)
4. Load Editor_Cross in game → breakpoint hits
5. Run script → should show: `scale_u=3.0, scale_v=3.0, offset_v=-3.0`
6. Load board in game → breakpoint hits
7. Run script → should show: `scale_u=1.0, scale_v=1.0, offset_v=-1.0`

This will **confirm** our hypothesis and give us exact values.

### Step 2: Build Parameter Lookup Table

Create a config mapping mesh names to their MappingInfo:

```typescript
const MESH_UV_PARAMS: Record<string, MappingInfo> = {
  'Editor_Cross': {
    axis1: 0,  // X
    axis2: 1,  // Y
    scale_u: 3.0,
    scale_v: 3.0,
    offset_u: 0.0,
    offset_v: -3.0
  },
  'board': {
    axis1: 0,  // X
    axis2: 1,  // Y
    scale_u: 1.0,
    scale_v: 1.0,
    offset_u: 0.0,
    offset_v: -1.0
  },
  'board_lod0': {
    // Same as board
    axis1: 0,
    axis2: 1,
    scale_u: 1.0,
    scale_v: 1.0,
    offset_u: 0.0,
    offset_v: -1.0
  }
  // Add more as we discover them
};
```

### Step 3: Simplify the UV Generation

Remove all the auto-detection complexity and just use the correct parameters:

```typescript
function loadMesh(vertices: Float32Array, indices: Uint32Array, meshName: string) {
  // Look up parameters for this mesh
  const params = MESH_UV_PARAMS[meshName] || MESH_UV_PARAMS['default'];
  
  // Generate UVs with correct parameters
  geometry = generatePlanarUVs(
    geometry,
    params.scale_u,
    params.scale_v,
    params.offset_v
  );
}

function generatePlanarUVs(geometry, scale_u, scale_v, offset_v) {
  // Simple planar projection - SINGLE mode, no box mapping!
  // Always use XY projection (axis1=X, axis2=Y)
  
  const bbox = geometry.boundingBox;
  const range_x = bbox.max.x - bbox.min.x;
  const range_y = bbox.max.y - bbox.min.y;
  
  const factor_u = scale_u / range_x;
  const factor_v = scale_v / range_y;
  
  for each vertex:
    U = (vertex.x - bbox.min.x) * factor_u + 0.0;  // offset_u always 0
    V = (-vertex.y - bbox.min.y) * factor_v + offset_v;
}
```

### Step 4: Test and Verify

1. Load Editor_Cross → should show 3x3 checker pattern ✅
2. Load board → should show 1x1 texture (full board covered by single texture instance) ✅
3. Load other meshes → may need to extract their parameters

## Why This Makes Sense

### Different Meshes Need Different Tiling

- **Editor_Cross:** A debug/editor mesh that WANTS a repeating checker pattern to show orientation → 3x3 tiling
- **Board:** A game asset with a custom texture (board graphics) that should NOT repeat → 1x1 tiling
- **Characters, obstacles, etc.:** Probably also 1x1 (each has unique texture)

The game designers **intentionally** set different UV parameters for different mesh types!

## Common Misconceptions (Cleared Up)

### ❌ "Board uses different projection algorithm"
**✅ Reality:** Same algorithm, different scale/offset parameters

### ❌ "We need mesh normalization before UV generation"
**✅ Reality:** Normalization is built into the scale calculation (`scale / range`)

### ❌ "Box mapping vs planar mapping"
**✅ Reality:** Both use planar mapping, always XY projection

### ❌ "V-axis inversion is the problem"
**✅ Reality:** V-axis inversion (`-vertex.y`) is correct and used for both meshes

### ❌ "We need to auto-detect mesh type"
**✅ Reality:** We need to look up the correct parameters per mesh name

## Success Criteria

✅ Extract MappingInfo for Editor_Cross → verify scale=3.0  
✅ Extract MappingInfo for board → verify scale=1.0  
✅ Update code with parameter lookup table  
✅ Remove auto-detection and complexity  
✅ Both meshes render correctly  
✅ Document parameters for other mesh types  

## Files to Reference

- **Analysis:** `meshViewerNotes/CRITICAL_ANALYSIS.md`
- **Source Code:** `meshViewerNotes/decompiled/sr.c` (lines 61200, 64537)
- **Extracted Data:** `threejs/src/meshViewer/boardLod0CompleteData.ts`
- **Extracted Data:** `threejs/src/meshViewer/editorCrossCompleteData.ts`
- **Extraction Script:** `meshViewerNotes/aiScripts/extractMappingInfo.lua`
- **Current Implementation:** `threejs/src/meshViewer/runMeshViewer.ts`

## Bottom Line

**Stop trying to fix the algorithm. It's already correct.**

**Start using the correct input parameters for each mesh.**

The game has a lookup table (or material system) that provides the right `MappingInfo` for each mesh. We need to replicate that, not try to guess based on mesh shape.
