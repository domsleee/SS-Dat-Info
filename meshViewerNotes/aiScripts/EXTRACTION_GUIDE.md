# Complete Data Extraction Guide

This guide shows you how to extract **ALL** the data from the game to understand why board rendering fails.

## 📊 Available Extraction Scripts

### 1. **extractCompleteGeometry.lua** ⭐ MOST IMPORTANT
**What it does:** Extracts complete mesh data (all vertices, UVs, triangles) in TypeScript format

**Why use it:** Gives you the EXACT geometry data to compare with Three.js

**Output:** TypeScript file you can directly import and compare

**Usage:**
```
1. Set breakpoint at getModelByName
2. Load mesh (Editor_Cross or board)
3. Note mesh pointer from EAX
4. Update meshPtr in script
5. Run script
6. Copy TypeScript output to: threejs/src/meshViewer/gameExtracted_board.ts
```

### 2. **extractComparisonData.lua**
**What it does:** Comprehensive mesh analysis and comparison

**Why use it:** Shows UV ranges, mesh dimensions, aspect ratios, vertex duplication

**Output:** Side-by-side comparison data

**Usage:**
```
1. Run for Editor_Cross mesh
2. Run for board mesh
3. Compare outputs to find differences
```

### 3. **extractTriangleData.lua**
**What it does:** Extracts first 20 triangles with full vertex/UV data

**Why use it:** Helps verify triangle structure and find degenerate triangles

**Output:** Triangle-by-triangle breakdown

### 4. **extractNormalData.lua**
**What it does:** Extracts vertex normal vectors

**Why use it:** Checks if lighting/normal calculation differs from Three.js

**Output:** Normal vectors + validation

### 5. **extractMappingInfo.lua** ⭐ CRITICAL
**What it does:** Extracts the actual MappingInfo structure used by planarMap()

**Why use it:** Shows the REAL UV generation parameters the game uses

**Output:** axis1, axis2, scale_u, scale_v, offset_u, offset_v

**Usage:**
```
1. Find sr.dll base address
2. Set breakpoint at sr.dll+0x43E70 (planarMap function)
3. Load mesh in game
4. When breakpoint hits, run script
5. Script auto-reads MappingInfo from stack
```

## 🎯 Recommended Extraction Order

### Phase 1: Get Complete Geometry
```
1. Run extractCompleteGeometry.lua for Editor_Cross
   → Save output as gameExtracted_Editor_Cross.ts

2. Run extractCompleteGeometry.lua for board
   → Save output as gameExtracted_board.ts
```

### Phase 2: Get UV Parameters
```
3. Run extractMappingInfo.lua when loading Editor_Cross
   → Note the parameters

4. Run extractMappingInfo.lua when loading board
   → Note the parameters
```

### Phase 3: Comparison Analysis
```
5. Run extractComparisonData.lua for both meshes
   → Compare UV ranges and patterns

6. Use compareGeometry.ts to compare game vs Three.js geometry
```

## 📝 What Data You Can Extract

| Data Type | Where It Is | How To Extract |
|-----------|-------------|----------------|
| **Vertex Positions** | Mesh+0x08 array, offset 0x30-0x38 | extractCompleteGeometry.lua |
| **UV Coordinates** | Vertex offset 0xF0-0xF4 | extractCompleteGeometry.lua |
| **Normal Vectors** | Vertex offset 0x0C-0x14 | extractNormalData.lua |
| **Triangle Indices** | Implicit (3 consecutive vertices = 1 tri) | extractTriangleData.lua |
| **UV Parameters** | MappingInfo structure at planarMap call | extractMappingInfo.lua |
| **Mesh Dimensions** | Calculated from min/max vertex positions | extractComparisonData.lua |
| **UV Ranges** | Calculated from min/max UVs | extractComparisonData.lua |
| **Vertex Duplication** | Positions match, UVs differ | extractComparisonData.lua |

## 🔍 Additional Data You Can Extract

### Texture Information
- **Texture pointer:** Mesh structure may contain texture reference
- **Texture dimensions:** Important for UV coordinate interpretation
- **Texture format:** Check if it's using mipmaps, compression, etc.

### Material Properties
- **Diffuse color:** May affect how texture appears
- **Specular properties:** Affects lighting
- **Transparency/Alpha:** Check if mesh uses alpha blending

### Transformation Matrices
- **Model matrix:** Check if mesh has pre-applied transformations
- **World position:** Where mesh is placed in game world
- **Rotation/Scale:** May affect rendering

### Render State
- **Culling mode:** Front-face vs back-face culling
- **Winding order:** Clockwise vs counter-clockwise
- **Depth testing:** Z-buffer settings

### LOD (Level of Detail)
- **LOD levels:** Board has "board_lod0" suggesting multiple LODs
- **LOD switching distances:** When game switches between LODs
- **Differences between LODs:** May explain missing detail

## 🧪 Using the Comparison Tool

After extracting game geometry, use `compareGeometry.ts`:

```typescript
import { compareGeometry, analyzeUVPattern, findDuplicateVertices } from './compareGeometry';
import { gameExtracted_board } from './gameExtracted_board';

// In your mesh viewer code:
const threeJsGeometry = mesh.geometry; // Your Three.js mesh

// Compare geometries
const result = compareGeometry(gameExtracted_board, threeJsGeometry, {
  positionTolerance: 0.001,
  uvTolerance: 0.001,
  verbose: true
});

// Analyze UV patterns
analyzeUVPattern(threeJsGeometry);

// Find UV seams
const duplicates = findDuplicateVertices(threeJsGeometry);
console.log(`Found ${duplicates.length} UV seams`);
```

## 🎯 Key Questions to Answer

1. **Do vertex positions match?**
   - If NO: Coordinate system transformation issue
   - If YES: Problem is elsewhere

2. **Do UV coordinates match?**
   - If NO: UV generation algorithm is wrong
   - If YES: Problem may be in texture sampling or rendering

3. **Do normal vectors match?**
   - If NO: Lighting calculation differs
   - If YES: Normals are correct

4. **Is vertex count the same?**
   - If NO: UV seaming expansion differs
   - If YES: Same vertex structure

5. **Are triangles in same order?**
   - If NO: Triangle winding or indexing issue
   - If YES: Geometry structure matches

6. **Are MappingInfo parameters correct?**
   - If NO: **THIS IS YOUR BUG** - fix UV generation
   - If YES: Bug is in how parameters are applied

## 💡 Expected Findings

### Editor_Cross (Working)
- MappingInfo: axis1=0 (X), axis2=1 (Y), scale_u=3.0, scale_v=3.0, offset_v=-3.0
- UV Range: U=[0, 3], V=[-3, 0]
- Vertices: 32 HX → 96 game (3x expansion)
- Aspect Ratio: ~1:1 (cubic)

### Board (Broken)
- MappingInfo: axis1=0 (X), axis2=1 (Y), scale_u=1.0, scale_v=1.0, offset_v=-1.0
- UV Range: U=[0, 1], V=[-1, 0]
- Vertices: 25 HX → 60 game (2.4x expansion)
- Aspect Ratio: ~4:1 (elongated)

## 🚨 What to Look For

1. **UV coordinate mismatch** → UV generation formula is wrong
2. **Missing triangles** → Triangle indices not built correctly
3. **Flipped/mirrored rendering** → Coordinate system transformation issue
4. **Wrong texture tiling** → scale_u/scale_v parameters incorrect
5. **Seam artifacts** → Vertex duplication for UV seaming not working

## ✅ Success Criteria

You've extracted enough data when:

1. ✅ You have complete vertex positions for both meshes
2. ✅ You have complete UV coordinates for both meshes
3. ✅ You have actual MappingInfo parameters from game
4. ✅ You can compare game vs Three.js side-by-side
5. ✅ You can identify the SPECIFIC difference causing the bug

## 📚 Related Files

- `sr.c` - planarMap function (line 64537)
- `HMG_3DE.c` - Mesh loading from HX files
- `runMeshViewer.ts` - Three.js rendering code
- `boardLod0CompleteData.ts` - Previously extracted board data
- `editorCrossCompleteData.ts` - Previously extracted Editor_Cross data

## 🔧 Next Steps After Extraction

1. **Compare extracted data** with Three.js rendering
2. **Identify specific differences** (UV mismatch, triangle order, etc.)
3. **Fix the specific issue** in runMeshViewer.ts
4. **Verify fix** doesn't break Editor_Cross
5. **Test with other meshes** to ensure general solution
