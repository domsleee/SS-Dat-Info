# Fix: Properly Handle Extracted Game Data (Non-Indexed Geometry)

## The Problem

After implementing Option 2 to use extracted game UVs, the triangles were rendering **completely wrong**. 

### Root Cause

The extracted game data has a **fundamentally different structure** than the HX file data:

| Source | Format | Vertex Count (Board) | Vertex Count (Editor_Cross) |
|--------|--------|---------------------|---------------------------|
| **HX File** | Indexed geometry | ~25 unique vertices | 32 unique vertices |
| **Game Memory** | Non-indexed (triangle list) | 60 vertices (duplicated) | 96 vertices (duplicated) |

**The issue:** We were trying to use the extracted game data (non-indexed) with the HX file loading path (which expects indexed geometry).

### What Was Happening

```typescript
// WRONG - Mixed HX file flow with extracted data
loadHXData(hxData);  // Parses HX file → gets indices + vertices
loadMesh(vertices, indices, uvs);  // Expects indexed geometry
// Then tried to use extracted UVs with HX vertices
geometry.setAttribute("position", new THREE.BufferAttribute(boardLod0MeshData.vertices, 3));
geometry.setAttribute("uv", new THREE.BufferAttribute(boardLod0MeshData.uvs, 2));
// Result: Mismatched vertex counts, wrong triangles!
```

## The Solution

**Completely bypass HX file parsing** for meshes with extracted data:

### 1. Separate Loading Paths

```typescript
// For board - use extracted data directly
async function loadBoard() {
  currentMeshType = 'board';
  loadExtractedGameMesh();  // ← NEW: Direct to extracted data
  loadTGAData(/* texture */);
}

// For Editor_Cross - use extracted data directly
async function loadEditorCross() {
  currentMeshType = 'editorCross';
  loadExtractedGameMesh();  // ← NEW: Direct to extracted data
  loadTGAData(/* texture */);
}
```

### 2. New Mesh Loading Function

```typescript
function loadMeshDirect(vertices: Float32Array, uvs: Float32Array) {
  // Extracted game data is ALREADY:
  // - Non-indexed (triangle list format)
  // - Vertices duplicated for UV seaming
  // - UVs matched to vertices
  
  geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3));
  geometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
  
  // No indices needed!
  // No conversion to non-indexed needed!
  // Just use the data as-is
}
```

### 3. Keep Original Function for HX Files

The original `loadMesh(vertices, indices, uvs)` function remains for:
- Loading from HX files
- UV generation for unknown meshes
- Fallback behavior

## Why the Extracted Data Is Non-Indexed

### UV Seaming Requires Vertex Duplication

In the game, vertices shared by multiple faces need **different UVs per face**. For example:

```
HX File:
  Vertex[10] at position (16.75, 16.75, 0.0)
  
Game Memory (after expansion):
  Vertex[0]  at (16.75, 16.75, 0.0)  UV=(1.000, -1.000)  ← Front face
  Vertex[15] at (16.75, 16.75, 0.0)  UV=(1.000, -2.000)  ← Side face
  Vertex[46] at (16.75, 16.75, 0.0)  UV=(1.999, -1.000)  ← Top face
```

**Same 3D position, but 3 different UV coordinates!**

This is why the game expands:
- **Board:** 25 unique vertices → 60 vertices (2.4x expansion)
- **Editor_Cross:** 32 unique vertices → 96 vertices (3x expansion)

## Structure Comparison

### HX File (board.hx)
```
Vertices: 25 unique positions (Float32Array, 75 floats)
Indices: 76 triangles (Uint32Array, 228 ints = 76 * 3)
UVs: None (generated procedurally)

Format: Indexed geometry
Usage: geometry.setIndex(indices)
```

### Extracted Game Data (boardLod0MeshData)
```
Vertices: 60 expanded positions (Float32Array, 180 floats)
Indices: None (vertices are in triangle-list order)
UVs: 60 matched UVs (Float32Array, 120 floats)

Format: Non-indexed triangle list
Usage: Direct attribute assignment, no index
```

## The Fix in Detail

### Before (WRONG)
```typescript
async function loadBoard() {
  const hxData = await fetch('/board.hx');
  loadHXData(hxData);  // Parse HX → vertices[25], indices[228]
  // Then somewhere later...
  geometry.setAttribute("position", boardLod0MeshData.vertices);  // 60 vertices!
  geometry.setAttribute("uv", boardLod0MeshData.uvs);  // 60 UVs!
  // MISMATCH: Using 60-vertex UVs with 25-vertex geometry + indices
}
```

### After (CORRECT)
```typescript
async function loadBoard() {
  currentMeshType = 'board';
  loadExtractedGameMesh();  // Direct to extracted data
}

function loadExtractedGameMesh() {
  loadMeshDirect(boardLod0MeshData.vertices, boardLod0MeshData.uvs);
}

function loadMeshDirect(vertices: Float32Array, uvs: Float32Array) {
  // vertices[60] and uvs[60] - perfectly matched!
  geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3));
  geometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
  // No indices - vertices are in triangle-list order
}
```

## Testing

After this fix:

1. **Load Board** (`#board` or default)
   - ✅ Should show correct snowboard shape
   - ✅ Texture should map properly (1x1)
   - ✅ No weird triangles or distortion

2. **Load Editor_Cross** (`#editorCross`)
   - ✅ Should show correct cross shape
   - ✅ 3x3 checker pattern should be visible
   - ✅ No weird triangles or distortion

3. **Check Console**
   - ✅ "Using extracted game data (Option 2)"
   - ✅ "Vertices: 60 (non-indexed, expanded for UV seaming)" for board
   - ✅ "Vertices: 96 (non-indexed, expanded for UV seaming)" for Editor_Cross

## Key Learnings

1. **HX files use indexed geometry** (vertex reuse with indices)
2. **Game runtime uses non-indexed geometry** (vertices duplicated for UV seaming)
3. **These are incompatible formats** - can't mix data from both
4. **Option 2 requires complete bypass of HX parsing** for extracted meshes
5. **Extracted data is ready-to-use** - no transformation needed

## Files Modified

- `threejs/src/meshViewer/runMeshViewer.ts`
  - `loadBoard()` - now calls `loadExtractedGameMesh()`
  - `loadEditorCross()` - now calls `loadExtractedGameMesh()`
  - Added `loadExtractedGameMesh()` - dispatcher function
  - Added `loadMeshDirect()` - handles non-indexed geometry
  - Kept `loadMesh()` - for HX file data (indexed geometry)

## Success Criteria

✅ Both meshes render with correct geometry  
✅ Textures map properly (1x1 for board, 3x3 for Editor_Cross)  
✅ No triangle distortion or weird artifacts  
✅ Console shows extracted data is being used  
✅ UV ranges match extracted game data  

This fix properly accounts for the difference between the HX file format (indexed) and the game's runtime format (non-indexed with UV seaming).
