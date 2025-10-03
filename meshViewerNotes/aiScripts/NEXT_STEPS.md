# NEXT STEPS - UV Investigation Priority List

## ⚠️ CRITICAL FIRST STEP: Verify UV Data Extraction

**Problem:** We're unsure if the UV data in `editorCrossCompleteData.ts` is actually correct!

### Why This Matters
- All our comparisons assume the extracted UVs are correct
- If extraction is wrong, we're chasing the wrong problem
- Current UVs show `V0: UV=(1.000, -1.000)` but our code generates `UV=(1.000, 1.000)`
- Need to confirm the game really has NEGATIVE V coordinates

### Action Required

1. **Load Game with Cheat Engine**
   - Launch Supreme Snowboarding
   - Attach Cheat Engine
   - Set breakpoint at `getModelByName` (address from previous session)

2. **Load Editor_Cross**
   - In game, load the Editor_Cross mesh/model
   - Breakpoint should hit
   - Get `srMeshModel` pointer from EAX register

3. **Run Verification Script**
   - Open `verifyUVData.lua` in Cheat Engine
   - Update `modelPtr` variable at top with EAX value
   - Execute script
   - Check output:
     - Are UV values reasonable? (Should be in range [-3, 3])
     - Are there any obvious errors? (NaN, huge numbers, etc.)
     - Do first 10 UVs match what's in `editorCrossCompleteData.ts`?

4. **Run Extraction Script**
   - Open `extractUVsVerified.lua`
   - Update `modelPtr` variable
   - Execute script
   - This will:
     - Find known vertex positions
     - Show their UVs
     - Confirm UV seaming (same position, different UVs)
     - Dump complete data for verification

### Expected Output

If data is **CORRECT**, you should see:
```
V0: Front face, top-right
  Index: 0
  Position: (16.75, 16.75, 0.00)
  UV: (1.000500, -1.000499)  ← NEGATIVE V!

V1: Front face, far-right  
  Index: 1
  Position: (50.25, 16.75, 0.00)
  UV: (1.000500, -0.001499)  ← NEGATIVE V, closer to 0!
```

If data is **WRONG**, you might see:
- All UVs = (0.0, 0.0)
- Huge values like UV=(99999.0, 12345.6)
- Positive V coordinates when expecting negative
- Values don't match positions logically

### After Verification

**If UVs are CORRECT:**
- Problem is in our procedural generation algorithm
- Need to fix offset_v, scale factors, or coordinate flipping
- Proceed to Step 2

**If UVs are WRONG:**
- Need to find correct memory offset
- Re-extract all UV data
- Update `editorCrossCompleteData.ts` with correct values

---

## Step 2: Compare Procedural vs Memory UVs

**Only do this after Step 1 confirms UV data is correct!**

Add debugging to compare our calculated UVs with game's extracted UVs:

```typescript
// In runMeshViewer.ts, after calculating UVs
import { editorCrossUVs } from './editorCrossCompleteData';

// For first 10 vertices
if ((i + j) < 10) {
  const gameU = editorCrossUVs[(i + j) * 2];
  const gameV = editorCrossUVs[(i + j) * 2 + 1];
  
  console.log(`V${i+j}:`);
  console.log(`  Position: (${px}, ${py}, ${pz})`);
  console.log(`  Calculated: UV=(${u.toFixed(3)}, ${v.toFixed(3)})`);
  console.log(`  Game data:  UV=(${gameU.toFixed(3)}, ${gameV.toFixed(3)})`);
  console.log(`  Difference: ΔU=${Math.abs(u - gameU).toFixed(3)}, ΔV=${Math.abs(v - gameV).toFixed(3)}`);
}
```

This will show EXACTLY where our algorithm diverges from game data.

---

## Step 3: Analyze the Pattern

Look at the differences between calculated and expected:

### Pattern A: Consistent Offset
```
V0: ΔU=0.000, ΔV=2.000  (always 2.0 difference)
V1: ΔU=0.000, ΔV=2.000
V2: ΔU=0.000, ΔV=2.000
```
**Diagnosis:** `offset_v` is wrong, need to adjust by -2.0

### Pattern B: Scaling Issue
```
V0: ΔU=0.000, ΔV=0.500
V1: ΔU=0.000, ΔV=1.000
V2: ΔU=0.000, ΔV=1.500
```
**Diagnosis:** `scale_v` is wrong, need to multiply by different factor

### Pattern C: Wrong Projection
```
V0: ΔU=0.000, ΔV=2.000
V1: ΔU=5.000, ΔV=0.000  (completely different)
V2: ΔU=0.000, ΔV=3.000
```
**Diagnosis:** Box mapping selecting wrong projection axis for some faces

### Pattern D: Sign Flip
```
V0: ΔU=0.000, ΔV=2.000  (calculated=1.0, expected=-1.0)
V1: ΔU=0.000, ΔV=2.000  (calculated=1.0, expected=-1.0)
```
**Diagnosis:** Not negating V properly, or double-negating

---

## Step 4: Fix Based on Pattern

### For Offset Issues
Adjust `offset_u` or `offset_v` in code:
```typescript
const offset_v = -3.0;  // Try different values: -1.0, -2.0, -3.0, -4.0
```

### For Scaling Issues
Adjust `mapping_scale_v` or scale calculation:
```typescript
const mapping_scale_v = 3.0;  // Try: 1.0, 2.0, 3.0, 4.0
```

### For Projection Issues
Check box mapping logic - maybe we're selecting axis incorrectly:
```typescript
// Currently: If |nz| largest → XY
// Maybe should be: If |nz| largest → XZ?
```

### For Sign Issues
Check Y-flip and V-negation:
```typescript
// Formula: v = (-vertex_v - min_v) * scale_v + offset_v
// Maybe should be: v = (vertex_v - min_v) * scale_v + offset_v ?
// Or: const posY = -pos.getY(i + j);  // Remove isFlipped check?
```

---

## Step 5: Alternative Approaches

If all else fails, try these:

### Approach A: Use Game's 96 Vertices Directly
- Extract triangle indices by position matching
- Map each of 60 triangles to vertices in the 96-vertex array
- Use game's pre-baked UVs directly
- **Advantage:** No calculation needed, should be 100% correct
- **Disadvantage:** Complex mapping logic needed

### Approach B: Simpler Projection
- Forget box mapping temporarily
- Use single XY projection for entire mesh
- See if at least SOME faces work correctly
- This isolates whether problem is box mapping or formula itself

### Approach C: Visual Debugging
- Set UVs to rainbow gradient based on position
- Use different texture (gradient instead of checkerboard)
- Visually see which faces map where
- Easier to spot patterns than with checkerboard

---

## Current Status Summary

**What Works:**
- ✅ Board mesh displays perfectly (flat, single projection)
- ✅ Can load HX files (32 vertices, 60 triangles)
- ✅ Can extract UVs from game memory (96 values)
- ✅ Box mapping implemented (per-face projection)
- ✅ Coordinate system conversion (Y-up ↔ Y-down)

**What Doesn't Work:**
- ❌ Editor_Cross shows 2/3 faces wrong
- ❌ UVs are positive when should be negative
- ❌ No triangle indices for 96-vertex mesh
- ❌ Can't directly use game's pre-baked UVs

**What We Don't Know:**
- ❓ Is extracted UV data actually correct?
- ❓ Why are we generating positive V instead of negative?
- ❓ Is box mapping selecting correct projections?
- ❓ Are scale/offset values right for all axes?

**Priority:** Verify UV data (Step 1) before anything else!
