# Quick Start: Board UV Investigation

**Goal:** Extract and verify UV parameters for snowboard meshes

## Prerequisites

- Supreme Snowboarding game running
- Cheat Engine attached to game process
- Breakpoint set at `getModelByName` function

## Step-by-Step Instructions

### 1. Load a Board Mesh

In the game:
- Go to character selection screen, OR
- Load track editor, OR
- Any screen where a snowboard model is visible

The board model should load and trigger your breakpoint.

### 2. Get the Memory Pointer

When breakpoint hits:
1. Check EAX register value (this is the `srMeshModel*` pointer)
2. Copy the hex value (e.g., `0xF270C40`)

### 3. Update the Lua Script

1. Open `extractBoardUVs.lua` in Cheat Engine
2. Find line 13: `local modelPtr = 0xF270C40`
3. Replace `0xF270C40` with your EAX value
4. Save (or just edit in Cheat Engine's Lua window)

### 4. Run the Script

1. Click "Execute Script" in Cheat Engine
2. Wait for output to appear
3. Script will show:
   - Mesh dimensions and aspect ratio
   - UV ranges (U and V)
   - Comparison with Editor_Cross
   - Estimated parameters
   - Complete TypeScript data dump

### 5. Save the Output

1. Find the section marked "TYPESCRIPT DATA"
2. Copy everything from `export const boardMeshData = {` to the final `};`
3. Create new file: `threejs/src/meshViewer/boardCompleteData.ts`
4. Paste the copied data
5. Save the file

### 6. Analyze the Results

Look for these key values in the script output:

```
UV Ranges:
  U: ?.??? to ?.??? (range: ?.???)
  V: ?.??? to ?.??? (range: ?.???)
       ^^^^^^^^ Is this NEGATIVE like Editor_Cross?

Estimated UV Parameters:
  scale_u ≈ ?.???
  scale_v ≈ ?.???
  offset_v ≈ ?.???
```

Compare with Editor_Cross:
```
Editor_Cross:
  U: [0.001, 2.999] (range: ~3.0)
  V: [-2.999, -0.001] (range: ~3.0, NEGATIVE)
  scale_u = 3.0
  scale_v = 3.0
  offset_v = -3.0
```

### 7. Update the Code

Based on your findings, update `threejs/src/meshViewer/runMeshViewer.ts`:

Find this section (around line 391):
```typescript
} else if (aspectRatio > 3.0) {
  // Elongated meshes (like snowboard ~4:1 ratio)
  mapping_scale_v = 0.686;  // ← UPDATE THIS
  uvOffsetV = 0.080;        // ← UPDATE THIS
  vScaleMultiplier = 0.70;  // ← UPDATE THIS
```

Replace with values from extraction:
```typescript
} else if (aspectRatio > 3.0) {
  // Elongated meshes (like snowboard ~4:1 ratio)
  mapping_scale_v = ???;  // From extraction
  uvOffsetV = ???;        // From extraction
  vScaleMultiplier = 1.0; // Probably don't need this anymore
```

### 8. Test

1. Reload the mesh viewer in browser
2. Load a board HX file
3. Check if textures look correct
4. Compare with how it looks in the game

### 9. Document Findings

Update `HYPOTHESIS_MESH_TYPES.md` with:
- Actual board UV ranges
- Confirmed scale/offset values
- Whether hypothesis was correct
- Any differences from Editor_Cross

## Expected Output Examples

### Case 1: Same as Editor_Cross
```
UV Ranges:
  U: 0.001 to 2.999 (range: 3.0)
  V: -2.999 to -0.001 (range: 3.0) ✅ NEGATIVE

Estimated UV Parameters:
  scale_u ≈ 3.0
  scale_v ≈ 3.0
  offset_v ≈ -3.0
```

**Interpretation:** Board uses same parameters as Editor_Cross!  
**Action:** Change board code to use `mapping_scale_v = 3.0`

### Case 2: Different Scale
```
UV Ranges:
  U: 0.001 to 2.999 (range: 3.0)
  V: -0.999 to -0.001 (range: 1.0) ✅ NEGATIVE

Estimated UV Parameters:
  scale_u ≈ 3.0
  scale_v ≈ 1.0
  offset_v ≈ -1.0
```

**Interpretation:** Board uses 3x1 tiling (not 3x3)  
**Action:** Set `mapping_scale_v = 1.0`, `offset_v = -1.0`

### Case 3: Compressed Range
```
UV Ranges:
  U: 0.001 to 2.999 (range: 3.0)
  V: -2.058 to -0.001 (range: 2.057) ✅ NEGATIVE

Estimated UV Parameters:
  scale_u ≈ 3.0
  scale_v ≈ 2.057
  offset_v ≈ -2.057
```

**Interpretation:** Board uses compressed V (like current 0.686 might be doing)  
**Action:** Set `mapping_scale_v = 2.057`, `offset_v = -2.057`

## Troubleshooting

### "Could not read vertex/triangle counts"
- Double-check the modelPtr value
- Make sure you copied the full hex address from EAX
- Verify breakpoint hit on the correct mesh load

### "This doesn't look like a board (aspect ratio < 3:1)"
- You may have loaded wrong mesh (character, obstacle, etc.)
- Try loading a different board model
- Check dimensions - board should have large Y value

### "UV array pointer is NULL"
- Mesh might not have UVs loaded yet
- Try stepping through a few more instructions
- Or reload the mesh

### UVs look strange (all zeros, huge values)
- Pointer offset might be wrong for this game version
- Try nearby offsets: +0x138, +0x140, +0x144
- Verify with `verifyUVData.lua` first

## Success Checklist

- [ ] Game running with board mesh loaded
- [ ] Breakpoint hit, EAX captured
- [ ] Lua script executed successfully
- [ ] Aspect ratio > 3.0 (confirmed board)
- [ ] UV ranges look reasonable (not all zeros)
- [ ] V coordinates are negative (like Editor_Cross)
- [ ] TypeScript data saved to file
- [ ] Parameters documented
- [ ] Code updated
- [ ] Tested in mesh viewer
- [ ] Findings documented

## Time Estimate

- First time: ~15-20 minutes
- Subsequent extractions: ~5 minutes
- Analysis and code update: ~10 minutes
- Testing and verification: ~5 minutes

**Total:** ~30-40 minutes for complete board investigation

---

**Next Steps After Completion:**
1. Update HYPOTHESIS_MESH_TYPES.md with findings
2. Test with multiple board models (different brands/designs)
3. Verify other elongated meshes use same parameters
4. Mark board investigation as ✅ COMPLETE

Good luck! 🎿
