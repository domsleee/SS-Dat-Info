# HX File Format Documentation

> **For AI Assistants**: See [`decompiled/README_FOR_AI.md`](decompiled/README_FOR_AI.md) for guidance on using the decompiled source files to verify and extend this documentation.

## Overview

The HX file format is a proprietary 3D model format used by Housemarque's 3D Engine. It stores mesh geometry data including vertices, faces, normals, texture coordinates, and material information in a hierarchical chunk-based structure.

## File Structure

The HX file uses a hierarchical chunk-based format where data is organized into typed chunks. Each chunk has:
- A **chunk type ID** (integer)
- A **data buffer** containing the chunk's payload
- Optional **child chunks** forming a tree structure

### Chunk Navigation

Chunks are organized as a tree with:
- `offset + 0x00`: Chunk data
- `offset + 0x04`: Chunk type ID
- `offset + 0x08`: Pointer to data buffer
- `offset + 0x18`: Pointer to next sibling chunk
- `offset + 0x1C`: Pointer to first child chunk

The code uses `FUN_10003c50(object, chunkTypeId)` to search for chunks by type ID.

## Chunk Types

Based on analysis of `srGFInterface::getModelByName` and related functions, the following chunk types are used:

### Chunk Type 0x00 - Root/Object Node
- Contains the root object or model reference
- Used to access the hierarchical structure

### Chunk Type 0x01 - Face/Triangle Indices
- **Size**: Variable (multiple of 12 bytes)
- **Format**: Array of triangle definitions
- **Data per triangle**: 3 indices × 4 bytes = 12 bytes per triangle
- Each index points to a vertex in the vertex array (Chunk 0x02)
- Count = buffer_size / 12

### Chunk Type 0x02 - Vertex Positions
- **Size**: Variable (multiple of 12 bytes)
- **Format**: Array of 3D vertex positions
- **Data per vertex**: 3 floats (x, y, z) × 4 bytes = 12 bytes per vertex
- Stores the geometric positions of all vertices in the mesh
- Count = buffer_size / 12

### Chunk Type 0x05 - Extended Vertex Data (Optional)
- **Size**: Variable (multiple of 24 bytes, 0x18)
- **Format**: Array of extended vertex attributes
- **Data per entry**: 24 bytes (6 floats per entry)
- Count = buffer_size / 24
- Retrieved with `FUN_10002e60(object, &count, &buffer)` (HMG_3DE.c)
- **Note**: Optional chunk - not all models contain it
- **Note**: NOT used in basic triangle construction (HMG_3DE.c lines 3280-3340)
- **Possible contents**: Normals (3 floats) + additional data (3 floats), or tangent/binormal data
- **Status**: Exact format and usage unclear from decompiled code

### Chunk Type 0x06 - Names/Strings
- **Size**: Variable
- **Format**: String data or name information
- Retrieved with `FUN_10002ed0`
- Contains model or material names

### Chunk Type 0x08 - Unknown Data Type A
- Referenced in code but specific purpose unclear
- Part of hierarchical model structure

### Chunk Type 0x09 - Unknown Data Type B
- Referenced in code but specific purpose unclear
- Part of hierarchical model structure

### Chunk Type 0x0A (10) - Per-Triangle Material Indices
- **Size**: Variable (multiple of 4 bytes)
- **Format**: Array of float values, **ONE per triangle** (not per vertex)
- **Data per entry**: 4 bytes (float)
- Count = buffer_size / 4 = number of triangles
- **Usage**: `pfStack_400[triangle_index]` - indexed by triangle counter in mesh construction loop
- Used to assign material IDs or texture scales to individual triangles
- The code checks if all values are identical to determine single-material vs multi-material mesh
- **NOT UV coordinates** - UVs are generated procedurally at runtime (see UV Generation section)
- **Source**: HMG_3DE.c mesh construction loop, retrieved via `FUN_10002f10`

### Chunk Type 0x0C (12) - Extended Model Data A
- Part of extended model information
- Specific format varies

### Chunk Type 0x0E (14) - Extended Model Data B
- Part of extended model information
- Specific format varies

### Chunk Type 0x0F (15) - Extended Model Data C
- Part of extended model information
- Specific format varies

### Chunk Type 0x11 (17) - Model Object Reference
- **Size**: Varies
- **Format**: Model/object reference node
- Used with name lookup: `FUN_10004480(object, 0x11, modelName)`
- Contains the named model object that can be retrieved by name

### Chunk Type 0x15 (21) - Color/RGBA Data
- **Size**: Variable (multiple of 4 bytes)
- **Format**: Array of integer/color values
- **Data per entry**: 4 bytes
- Count = buffer_size / 4
- Possibly vertex colors or material properties

## Data Reading Functions

The following functions are used to extract data from chunks:

```c
// Get face/triangle indices (chunk type 1)
FUN_10002dc0(object, &triangleCount, &triangleIndexBuffer);
// triangleCount = buffer_size / 12
// triangleIndexBuffer = pointer to array of int[3] per triangle

// Get vertex positions (chunk type 2)
FUN_10002e10(object, &vertexCount, &vertexBuffer);
// vertexCount = buffer_size / 12
// vertexBuffer = pointer to array of float[3] per vertex (x,y,z)

// Get normals (chunk type 5)
FUN_10002e60(object, &normalCount, &normalBuffer);
// normalCount = buffer_size / 24
// normalBuffer = pointer to normal data

// Get name/string data (chunk type 6)
FUN_10002ed0(object, &nameBuffer);

// Get material/UV data (chunk type 10)
FUN_10002f10(object, &uvCount, &uvBuffer);
// uvCount = buffer_size / 4
// uvBuffer = pointer to float array

// Get color/RGBA data (chunk type 21)
FUN_10002ff0(object, &colorCount, &colorBuffer);
// colorCount = buffer_size / 4
// colorBuffer = pointer to int/color array
```

## Mesh Construction Process

From `srGFInterface::getModelByName` (HMG_3DE.c lines 3200-3400), the mesh construction follows this process:

1. **Find Model Node**: Search for chunk type 0x11 by name using `FUN_10004480(object, 0x11, modelName)`
2. **Get Root Chunk**: Call `FUN_10003c50(model, 0)` to get the root chunk
3. **Read Geometry Data**:
   - Read vertex positions (chunk 0x02) via `FUN_10002e10` → `iStack_3d8` buffer, `uStack_40c` count
   - Read face indices (chunk 0x01) via `FUN_10002dc0` → `iStack_3c4` buffer, `uStack_3f8` count
   - Optionally read chunk 0x05 data via `FUN_10002e60` (but not directly used in triangle construction)
   - Read per-triangle material IDs (chunk 0x0A) via `FUN_10002f10` → `pfStack_400[triangle_index]`
   - Read color data (chunk 0x15) via `FUN_10002ff0` if present
4. **Process Triangles** (loop with `uStack_430` as triangle counter):
   - For each triangle index set at `iStack_3c4[uStack_404...uStack_404+8]`:
     - Read 3 vertex indices from index buffer
     - Fetch 3 vertex positions from `iStack_3d8` using those indices (12 bytes each: x,y,z floats)
     - Read material index: `fStack_3ac = pfStack_400[uStack_430]` (one float per triangle)
     - Build `srModeler::Triangle` object
     - Add to mesh model
5. **Build Mesh**:
   - Create `Emissive_ARGB_Model` mesh object
   - Set model name
   - Validate vertex counts (max 782,330 vertices)
6. **Material Assignment**:
   - If chunk 0x0A exists and contains varying values (multi-material), look up material by index
   - Otherwise use default material
   - Materials are matched by name and contain texture references

**Key Detail**: The triangle loop reads vertex data directly from the vertex buffer using indices. There is no evidence of UV coordinates being read from the file - they must be generated procedurally.

## Coordinate System

**CRITICAL**: The SurRender engine uses a **LEFT-HANDED Y-DOWN** coordinate system:
- **X+** = Right
- **Y+** = DOWN (not up!)
- **Z+** = Into screen (away from camera)

This differs from most modern 3D engines (Three.js, Unity, Blender) which use Y-up systems. When implementing parsers/viewers:
- You MUST account for this when transforming to Y-up coordinate systems
- Projection plane selection is affected (XY in Y-down ≠ XY in Y-up)
- UV generation may need coordinate transformation
- See sr.c for axis handling in projection functions

**Source**: Coordinate system confirmed through UV projection algorithm analysis (sr.c line 64537)

## UV Coordinates - NOT STORED IN FILE

**Important Discovery**: UV texture coordinates are **NOT** stored in the HX file format. 

Evidence from source code analysis (HMG_3DE.c):
- The mesh construction loop (lines 3280-3340) reads only: vertex positions, triangle indices, and per-triangle material IDs
- No UV coordinate data is read from any chunk
- Chunk 0x0A contains per-triangle material indices (1 float per triangle), not per-vertex UV pairs
- Chunk 0x05 (24 bytes per entry) is not used in the triangle construction

### UV Generation Algorithm

The SurRender engine generates UV coordinates procedurally at runtime using **planar projection mapping** (`srModeler::planarMap` - sr.c line 64537).

**Algorithm** (verified from decompiled source code):

1. **Calculate mesh bounding box** along two projection axes (e.g., X and Y for Z-plane projection)
2. **Compute scale factors**:
   ```
   scale_u = mapping.scale_u / (bbox_max_u - bbox_min_u)
   scale_v = mapping.scale_v / (bbox_max_v - bbox_min_v)
   ```
   (with division-by-zero protection)

3. **For each vertex, generate UV coordinates**:
   ```
   U = ((vertex[axis_u] - bbox_min_u) * scale_u) + offset_u
   V = ((-vertex[axis_v] - bbox_min_v) * scale_v) + offset_v
   ```
   
   **Source**: sr.c lines 64585-64587
   
   Note: The V formula uses negation of the vertex coordinate before subtracting min. 
   Alternative form: `V = ((-vertex[axis_v] - bbox_min_v)) * scale_v + offset_v`

**MappingInfo Structure** (sr.c line 64537):
```c
offset +0x00: axis1 (int)        // First projection axis (0=X, 1=Y, 2=Z)
offset +0x04: axis2 (int)        // Second projection axis
offset +0x08: scale_u (float)    // U scale factor
offset +0x0C: scale_v (float)    // V scale factor  
offset +0x10: offset_u (float)   // U offset
offset +0x14: offset_v (float)   // V offset
```

**Key Details**:
- **V-axis negation**: The V coordinate uses `-vertex[axis_v]`, inverting the V texture coordinate
- **Normalization**: Coordinates are normalized to the mesh's bounding box, then scaled
- **Multi-layer support**: Up to 4 texture layers × 2 stages = 8 UV coordinate sets per vertex
- **Mapping configuration**: MappingInfo structure provides axis selection, scale factors, and offset values
- **Alternative mode**: `planarMapAbsolute` (sr.c line 64605) skips bounding box normalization and applies scale directly to world-space coordinates
- **Vertex structure**: Each vertex in the renderer is 872 bytes (0x368), with UV coordinates stored at offset `(stage + 0x1e + layer * 2) * 8` from the vertex base
- **UV coordinate offset calculation**: For layer 0, stage 0: offset = 0x1e * 8 = 240 bytes into vertex structure

**Implementation for Modern Parsers**:

For basic texture mapping, implement the planar projection following the engine algorithm:
```python
# Calculate bounding box along projection axes (example: X and Y for XY plane)
min_u, max_u = min(v.x for v in vertices), max(v.x for v in vertices)
min_v, max_v = min(v.y for v in vertices), max(v.y for v in vertices)

# Calculate scale factors (with user-specified tiling scale)
scale_factor_u = scale_u / (max_u - min_u) if max_u != min_u else 0.0
scale_factor_v = scale_v / (max_v - min_v) if max_v != min_v else 0.0

# Calculate offset to ensure V starts at 0
# From sr.c: offset_v makes the V coordinate range start at 0
offset_u = 0.0
offset_v = (max_v + min_v) * scale_factor_v

# Generate UVs (XY plane projection example)
for vertex in vertices:
    u = (vertex.x - min_u) * scale_factor_u + offset_u
    # V-axis negation is CRITICAL - matches sr.c line 64587
    v = (-vertex.y - min_v) * scale_factor_v + offset_v
    vertex.uv = (u, v)
```

**Coordinate System Considerations**:
- Game uses Y-down (Y+ = down), most modern engines use Y-up (Y+ = up)
- When implementing for Y-up systems (Three.js, Unity), you may need to:
  - Transform vertex coordinates: `vertex.y = -vertex.y`
  - Or adjust projection plane (game's XY ≈ modern XZ)
  - Test different projections: XY, XZ, YZ

Note: Without access to the original material definitions, the exact projection axes and scale factors used by each material are unknown. Experiment with different axis combinations (XY, XZ, YZ) to achieve proper texturing.

## Material System

Materials are referenced through a material lookup table:
- Materials are matched by name using case-insensitive comparison (`_stricmp`)
- Each material can have:
  - Up to 2 textures (layers 0 and 1)
  - Shader references
  - Pass count (1-4 rendering passes)
  - Material properties at offsets 0x18, 0x1C, 0x20, 0x24, 0x28, 0x2C, 0x30

## Validation and Limits

The loader performs several validation checks:
- Maximum vertices: 782,330 (0xBEFFA)
- Minimum/maximum pass count: 1-4
- Material pass count consistency check
- Vertex position deduplication during import

## Error Handling

The code includes extensive error checking:
- Missing chunk validation
- Vertex count limits
- Material pass count validation
- Vertex deduplication verification

## Notes

- The format supports hierarchical object structures
- Multi-material meshes are supported via per-triangle material indices (chunk 0x0A)
- The engine uses a software renderer (SR - Software Renderer / SurRender)
- File format appears designed for efficient streaming and partial loading
- Some chunk types (0x08, 0x09, 0x0C, 0x0E, 0x0F) are not fully understood from the decompiled code alone
- **UV coordinates are NOT stored** - they must be generated procedurally
- The actual file structure in simple meshes (e.g., editor_cross.HX) uses sequential data blocks after markers like "mesh\0" rather than a strict chunk-based hierarchy
- Function names in decompiled code (HMG_3DE.c) are auto-generated (FUN_XXXXXXXX) and do not reflect original names

## Implementation Tips

When implementing a parser:
1. Search for the "mesh\0" marker in the file (signals start of geometry data)
2. Read triangle indices immediately after the marker (int32 triplets, 12 bytes per triangle)
3. Read vertex positions after indices (float32 triplets, 12 bytes per vertex)
4. Optional: Read per-triangle material indices (chunk 0x0A) if present
5. **Generate UV coordinates using planar projection** - they are not stored in the file
   - Calculate mesh bounding box along two axes (e.g., X and Y)
   - Normalize vertex positions to [0,1] range within bounding box
   - Apply `V = -V` inversion to match engine behavior
   - Scale UVs for texture tiling as needed
   - See "UV Coordinates" section above for detailed algorithm
6. Build triangle mesh from indices and vertices
7. Apply materials and textures

**Note**: The file format does not use a clear chunk-based structure in the simple mesh files examined. The data appears to be stored sequentially after markers like "mesh\0".

## Example Chunk Hierarchy

```
Root (type 0x00)
├── Model "ModelName" (type 0x11)
│   ├── Vertex Positions (type 0x02)
│   ├── Face Indices (type 0x01)
│   ├── Normals (type 0x05)
│   ├── Material Indices (type 0x0A)
│   ├── Names (type 0x06)
│   └── Colors (type 0x15)
└── [Other models...]
```

---

## Source Code Reference

*This documentation is based on reverse-engineering analysis of Housemarque's 3D Engine executable and examination of actual HX files from Supreme Snowboarding (1999).*

**Decompiled source files** (located in [`decompiled/`](decompiled/) directory):
- [`HMG_3DE.c`](decompiled/HMG_3DE.c) - 3D Engine & HX file loading (chunk readers)
- [`sr.c`](decompiled/sr.c) - SurRender engine core (UV generation algorithm at line 64537)
- [`HMG_3DE.asm`](decompiled/HMG_3DE.asm) - Assembly reference for exact byte offsets
- [`sr.asm`](decompiled/sr.asm) - Assembly reference for low-level details

**See also**: [`decompiled/README_FOR_AI.md`](decompiled/README_FOR_AI.md) - Guide for using these files to verify documentation

**Key findings confirmed through source code analysis:**
- *Triangle indices and vertex positions are stored sequentially*
- *Chunk 0x0A contains per-triangle material indices (1 float per triangle)*
- *UV coordinates are NOT stored in the file - they are generated procedurally at runtime using `srModeler::planarMap()`*
- *The UV generation uses bounding-box-normalized planar projection with V-axis inversion*
- *The mesh construction code reads only positions, indices, and material IDs*

*Some details about complex hierarchical features and optional chunks may be incomplete and require further validation.*
