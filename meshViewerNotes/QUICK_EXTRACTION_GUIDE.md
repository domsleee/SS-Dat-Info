# Quick Reference: MappingInfo Extraction Guide

## What You Need to Do

### 1. Setup Cheat Engine Breakpoint

**Find sr.dll base:**
- Cheat Engine → View → Enumerate DLLs and Symbols
- Find `sr.dll` 
- Note the base address (e.g., `0x10000000`)

**Set breakpoint:**
- Memory View → Tools → Debugger
- Set breakpoint at: `sr.dll + 0x43E70`
  - This is the `planarMap` function
  - From sr.c line 64537

### 2. Load Mesh in Game

**For Editor_Cross:**
- Open track editor
- Place an object (Editor_Cross should load)
- Breakpoint should hit

**For Board:**
- Go to character selection
- Select rider with snowboard visible
- Breakpoint should hit

### 3. Run Extraction Script

When breakpoint hits:
1. Open Cheat Engine → Lua Engine
2. Load: `meshViewerNotes/aiScripts/extractMappingInfo.lua`
3. Execute Script
4. Copy the output

### 4. Expected Results

**Editor_Cross:**
```
axis1     = 0 (X)
axis2     = 1 (Y)
scale_u   = 3.0
scale_v   = 3.0
offset_u  = 0.0
offset_v  = -3.0

Texture Tiling: 3x3
Projection Mode: XY (X for U, Y for V)
```

**Board:**
```
axis1     = 0 (X)
axis2     = 1 (Y)
scale_u   = 1.0
scale_v   = 1.0
offset_u  = 0.0
offset_v  = -1.0

Texture Tiling: 1x1
Projection Mode: XY (X for U, Y for V)
```

## What If the Script Fails?

### "Pointer looks invalid"

The script tries multiple stack locations. If all fail:

1. Check the function signature in IDA/Ghidra
2. `planarMap` parameters:
   - `this` (ECX register)
   - `param_1` (long) - layer index
   - `param_2` (long) - stage index  
   - `param_3` (MappingInfo*) - the one we want!

3. In debugger, check where `param_3` is:
   - Could be `[ESP+0x08]`
   - Could be `[ESP+0x0C]`
   - Could be `[ESP+0x10]`

4. Update the script to read from the correct offset

### "Values look wrong"

If you get weird values like `scale_u = 0.00000001` or huge numbers:

1. Check if pointer is actually pointing to MappingInfo
2. Try reading as different offsets
3. Verify breakpoint is hitting the RIGHT `planarMap` call
   - Game might call it multiple times
   - Step through to the call for the mesh you care about

## Alternative: Manual Inspection

If script doesn't work, inspect manually:

1. When breakpoint hits, note `param_3` address
2. Memory View → Go to address
3. Read the structure manually:
   ```
   +0x00: [4 bytes] axis1 (int)
   +0x04: [4 bytes] axis2 (int)
   +0x08: [4 bytes] scale_u (float)
   +0x0C: [4 bytes] scale_v (float)
   +0x10: [4 bytes] offset_u (float)
   +0x14: [4 bytes] offset_v (float)
   ```

## What to Do With the Data

### Document it:

Create a table in `CRITICAL_ANALYSIS.md`:

| Mesh Name | axis1 | axis2 | scale_u | scale_v | offset_u | offset_v | Notes |
|-----------|-------|-------|---------|---------|----------|----------|-------|
| Editor_Cross | 0 (X) | 1 (Y) | 3.0 | 3.0 | 0.0 | -3.0 | 3x3 checker |
| board_lod0 | 0 (X) | 1 (Y) | 1.0 | 1.0 | 0.0 | -1.0 | 1x1 tiling |
| ... | ... | ... | ... | ... | ... | ... | ... |

### Update the code:

In `runMeshViewer.ts`, replace auto-detection with lookup table:

```typescript
const MESH_PARAMS = {
  'Editor_Cross': { scale_u: 3.0, scale_v: 3.0, offset_v: -3.0 },
  'board': { scale_u: 1.0, scale_v: 1.0, offset_v: -1.0 },
  'board_lod0': { scale_u: 1.0, scale_v: 1.0, offset_v: -1.0 },
};
```

## Troubleshooting Checklist

- [ ] sr.dll loaded in game process?
- [ ] Correct breakpoint address (base + 0x43E70)?
- [ ] Breakpoint actually hits when mesh loads?
- [ ] Script reads valid pointer (0x10000000 - 0x7FFFFFFF range)?
- [ ] Values are reasonable (scale: 0.1-10.0, offset: -10.0 to 10.0)?
- [ ] Documented which mesh triggered the breakpoint?

## Next Meshes to Extract

After Editor_Cross and board, try:
- Character models (Keith, Akiko, etc.)
- Obstacles (trees, rocks, etc.)
- Track elements (ramps, rails, etc.)
- UI elements (if any use 3D models)

Build a complete mapping table!

## Time Estimate

- First extraction: ~20 minutes (setup + learning)
- Subsequent extractions: ~5 minutes each
- Total for 5-10 meshes: ~1 hour

## Success = Both Meshes Render Correctly

Once you have the correct parameters:
1. Editor_Cross shows 3x3 checker pattern ✅
2. Board shows 1x1 texture coverage ✅
3. No more auto-detection hacks needed ✅
