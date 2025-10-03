# UV Storage Investigation: Runtime vs File-Based

## Question
If `planarMap()` is not called at runtime during mesh loading, where do the UVs come from that we extracted via Cheat Engine?

## Evidence Gathered

### 1. Cheat Engine Extraction Method
Looking at `extractUVsVerified.lua`, the UVs are extracted from:
```lua
local uvArrayPtr = readInteger(modelPtr + 0x13C)  -- UV array pointer in srMeshModel structure
```

This reads from **offset 0x13C** in the `srMeshModel` object in **game memory** (RAM), NOT from disk.

### 2. HX File Contains NO UVs
Confirmed by binary search of `zeppelin.HX`:
- Searched for all UV values from `gameExtracted_zeppelin.ts`
- **Zero matches** - the UV data does not exist in the file

### 3. Decompiled Code Analysis
From `HMG_3DE.c` lines 3200-3450 (`srGFInterface::getModelByName`):
- Reads vertices from HX file ✓
- Reads triangle indices from HX file ✓
- Reads per-triangle material IDs (chunk 0x0A) ✓
- **No code calls `planarMap()` or generates UVs**

### 4. The Missing Link: Where Are UVs Generated?

There are THREE possible explanations:

## Hypothesis 1: Pre-Computed UVs in Level Files
❓ **UVs might be pre-computed offline and stored in level (.LVL) files, not mesh (.HX) files**

Evidence supporting this:
- The game loads entire levels, not individual meshes
- Level files could contain pre-baked UV data for all meshes in that level
- This would explain why `planarMap()` exists in the engine (for content pipeline tools) but isn't called at runtime
- Would explain vertex duplication (199 → 269 vertices for zeppelin) - level file has expanded mesh

Evidence needed to confirm:
- [ ] Examine .LVL file format
- [ ] Search for UV data in level files
- [ ] Check if level files reference mesh names + UV data

## Hypothesis 2: UVs Embedded in Game Executable
❓ **UVs might be compiled into the game binary as static data tables**

Evidence supporting this:
- Small number of meshes (could be practical to hard-code)
- Avoids runtime calculation overhead
- Common pattern in 1999-era games

Evidence needed to confirm:
- [ ] Search game executable (Supreme_Game.dll, sr.dll) for UV value arrays
- [ ] Look for data sections with float arrays matching known UV patterns

## Hypothesis 3: UVs Generated During Asset Build, Cached Somewhere
❓ **Content pipeline might generate UVs offline and cache them in a different file**

Possible locations:
- `.srDB` database files (SurRender database format)
- Separate UV cache files
- Embedded in texture files

Evidence needed:
- [ ] Search for other files in game directory that might contain mesh data
- [ ] Check if HX files are referenced by other files

## Most Likely Answer: Level Files or Executable

Given:
1. `planarMap()` function exists but isn't called during HX loading
2. UV data is definitely in memory (we extracted it)
3. UV data is NOT in HX files
4. The game is from 1999 (pre-computing would be normal)

**The most likely scenario is that UVs are either:**
- **Pre-computed offline** during asset build and stored in level files (.LVL)
- **Hard-coded** in the game executable as data arrays (indexed by mesh name)

## Next Steps to Confirm

1. **Examine a .LVL file** to see if it contains mesh data
2. **Search game binary** (Supreme_Game.dll) for float arrays matching known UV patterns
3. **Look for .srDB files** or other SurRender database files
4. **Check the decompiled code** for level loading to see if it reads UV data

## Implications for This Project

This means:
- ✅ We correctly documented that HX files don't contain UVs
- ✅ The `planarMap()` algorithm is correct (it's in the code)
- ❓ We need to find WHERE the runtime UV data actually comes from
- ✅ For our mesh viewer, we should continue using procedural generation (since we don't have access to the source data)

## Update Needed

The comment in `gameExtracted_zeppelin.ts` should be updated:
```typescript
// ⚠️  IMPORTANT: planarMap() was NOT called for this mesh!
//    UVs are pre-computed or calculated differently.
```

Should become:
```typescript
// ⚠️  UV DATA SOURCE: Unknown (NOT from HX file)
//    Possible sources:
//    1. Level files (.LVL) - pre-computed during asset build
//    2. Game executable - hard-coded data tables
//    3. Separate cache files
//    
//    This data was extracted from GAME MEMORY using Cheat Engine,
//    not from the zeppelin.HX file (which contains no UV data).
```
