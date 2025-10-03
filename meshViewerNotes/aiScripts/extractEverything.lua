-- ============================================================================
-- EXTRACT EVERYTHING - Complete Mesh Data Extractor
-- ============================================================================
-- This script extracts ALL data you need in ONE run:
--   1. Complete geometry (vertices, UVs, triangles) in TypeScript format
--   2. MappingInfo parameters (UV generation settings) - if available
--   3. Normal vectors
--   4. Mesh analysis (dimensions, UV ranges, duplicates)
--   5. Triangle validation (degenerate triangles)
--
-- SETUP:
--   1. Set breakpoint at getModelByName function (when it returns)
--   2. Load mesh in game (Editor_Cross or board)
--   3. When breakpoint hits, mesh pointer is in EAX
--   4. Run this script
--   5. Copy the TypeScript output
--
-- ============================================================================

print("╔════════════════════════════════════════════════════════════════╗")
print("║          EXTRACT EVERYTHING - Complete Data Extractor         ║")
print("╚════════════════════════════════════════════════════════════════╝")
print("")

-- ============================================================================
-- CONFIGURATION
-- ============================================================================

-- USER: Update this with the mesh name
local meshName = "board"  -- Change to "Editor_Cross", "board", etc.

print("🎯 Extracting data for: " .. meshName)
print("")

-- ============================================================================
-- STEP 1: Get mesh pointer from getModelByName return value (EAX)
-- ============================================================================

print("┌────────────────────────────────────────────────────────────────┐")
print("│ STEP 1: Getting Mesh Pointer from getModelByName              │")
print("└────────────────────────────────────────────────────────────────┘")
print("")

local meshPtr = 0
local esp = 0

-- Try to get registers (if we're at a breakpoint)
local success, regs = pcall(getRegisters)
if success and regs then
  esp = regs.ESP
  meshPtr = regs.EAX  -- getModelByName returns mesh pointer in EAX
  
  print("✅ Breakpoint detected!")
  print(string.format("   EAX (mesh pointer) = 0x%X", meshPtr))
  print(string.format("   ESP = 0x%X", esp))
  print("")
  
  -- Validate mesh pointer
  if meshPtr < 0x10000 or meshPtr > 0x7FFFFFFF then
    print("   ⚠️  EAX doesn't look like a valid pointer!")
    print("   Trying other registers...")
    print("")
    
    -- Try ECX
    if regs.ECX >= 0x10000 and regs.ECX <= 0x7FFFFFFF then
      meshPtr = regs.ECX
      print(string.format("   Trying ECX = 0x%X", meshPtr))
    end
  end
else
  print("⚠️  Not at a breakpoint - will try auto-detection")
  print("")
end

-- NOTE: planarMap is NOT called for these meshes!
-- UVs are pre-computed in the HX file or calculated elsewhere
-- We'll reverse-engineer the parameters from the actual UV data
local mappingInfo = nil  -- Will be calculated later from UV data

-- Mesh pointer should now be set from ECX
if meshPtr == 0 then
  print("   ⚠️  Failed to get mesh pointer from registers")
  print("")
  print("   USER: Update meshPtr in script and run again!")
  print("")
  meshPtr = 0xF270C40  -- USER: UPDATE THIS with ECX value
end

-- ============================================================================
-- STEP 3: Extract complete geometry
-- ============================================================================

print("┌────────────────────────────────────────────────────────────────┐")
print("│ STEP 3: Extracting Complete Geometry                          │")
print("└────────────────────────────────────────────────────────────────┘")
print("")

-- Correct offsets based on extractUVsVerified.lua
local ok, vertexCount = pcall(readInteger, meshPtr + 0x234)
if not ok or not vertexCount or vertexCount <= 0 or vertexCount > 100000 then
  print("❌ FATAL: Cannot read mesh data at " .. string.format("0x%X", meshPtr))
  print("   vertexCount offset 0x234 returned: " .. tostring(vertexCount))
  print("   Please update meshPtr in the script")
  return
end

local triangleCount = readInteger(meshPtr + 0x230)
local vertexArrayPtr = readInteger(meshPtr + 0x1DC)  -- Position array
local uvArrayPtr = readInteger(meshPtr + 0x13C)       -- UV array (separate!)

-- Check for index buffer (might be at different offsets)
local indexBufferPtr = nil
local indexCount = 0

-- Try common index buffer offset patterns
local possibleIndexOffsets = {0x1E0, 0x1E4, 0x200, 0x22C, 0x238}
for _, offset in ipairs(possibleIndexOffsets) do
  local ok, ptr = pcall(readInteger, meshPtr + offset)
  if ok and ptr and ptr >= 0x10000 and ptr <= 0x7FFFFFFF then
    -- Verify it looks like an index buffer by checking if values are reasonable
    local ok2, testVal = pcall(readSmallInteger, ptr)
    if ok2 and testVal and testVal >= 0 and testVal < vertexCount * 3 then
      indexBufferPtr = ptr
      print(string.format("   🔍 Found potential index buffer at offset 0x%X -> 0x%X", offset, ptr))
      break
    end
  end
end

if not vertexArrayPtr or not uvArrayPtr or vertexArrayPtr == 0 or uvArrayPtr == 0 then
  print("❌ FATAL: Invalid array pointers!")
  print(string.format("   vertexArrayPtr: 0x%X", vertexArrayPtr or 0))
  print(string.format("   uvArrayPtr: 0x%X", uvArrayPtr or 0))
  return
end

print(string.format("✅ Mesh found at: 0x%X", meshPtr))
print(string.format("   Vertices: %d", vertexCount))
print(string.format("   Triangles: %d", triangleCount))
print(string.format("   Vertex array: 0x%X", vertexArrayPtr))
print(string.format("   UV array: 0x%X", uvArrayPtr))
if indexBufferPtr then
  print(string.format("   Index buffer: 0x%X ✅", indexBufferPtr))
else
  print("   Index buffer: Not found (using triangle list)")
end
print("")

-- Extract all vertex data
print("   Extracting vertex positions and UVs...")
local vertices = {}
local uvs = {}

local minX, maxX = math.huge, -math.huge
local minY, maxY = math.huge, -math.huge
local minZ, maxZ = math.huge, -math.huge
local minU, maxU = math.huge, -math.huge
local minV, maxV = math.huge, -math.huge

for i = 0, vertexCount - 1 do
  -- Positions are stored as 3 floats (12 bytes per vertex)
  local x = readFloat(vertexArrayPtr + i * 12)
  local y = readFloat(vertexArrayPtr + i * 12 + 4)
  local z = readFloat(vertexArrayPtr + i * 12 + 8)
  
  -- UVs are stored separately as 2 floats (8 bytes per vertex)
  local u = readFloat(uvArrayPtr + i * 8)
  local v = readFloat(uvArrayPtr + i * 8 + 4)
  
  vertices[i * 3 + 1] = x
  vertices[i * 3 + 2] = y
  vertices[i * 3 + 3] = z
  
  uvs[i * 2 + 1] = u
  uvs[i * 2 + 2] = v
  
  -- Track bounds
  minX = math.min(minX, x)
  maxX = math.max(maxX, x)
  minY = math.min(minY, y)
  maxY = math.max(maxY, y)
  minZ = math.min(minZ, z)
  maxZ = math.max(maxZ, z)
  minU = math.min(minU, u)
  maxU = math.max(maxU, u)
  minV = math.min(minV, v)
  maxV = math.max(maxV, v)
end

-- Note: We don't have normals in this structure
local normals = nil

print("   ✅ Extracted all data")
print("")

-- ============================================================================
-- STEP 4: Reverse-engineer MappingInfo from UV data
-- ============================================================================

print("┌────────────────────────────────────────────────────────────────┐")
print("│ STEP 4a: Reverse-Engineering UV Parameters                    │")
print("└────────────────────────────────────────────────────────────────┘")
print("")

-- Since planarMap is NOT called, we need to figure out the UV parameters
-- from the actual UV data in the mesh

-- Calculate UV ranges first
local sizeX = maxX - minX
local sizeY = maxY - minY
local sizeZ = maxZ - minZ
local rangeU = maxU - minU
local rangeV = maxV - minV

-- Try to determine which axes map to U and V
-- Compare mesh dimensions with UV ranges
print("🔍 Analyzing UV mapping...")
print("")
print("   Mesh dimensions:")
print(string.format("      X size: %.2f", sizeX))
print(string.format("      Y size: %.2f", sizeY))
print(string.format("      Z size: %.2f", sizeZ))
print("")
print("   UV ranges:")
print(string.format("      U range: %.4f (%.4f to %.4f)", rangeU, minU, maxU))
print(string.format("      V range: %.4f (%.4f to %.4f)", rangeV, minV, maxV))
print("")

-- Estimate UV parameters
local estimatedScaleU = math.floor(rangeU + 0.5)
local estimatedScaleV = math.floor(math.abs(rangeV) + 0.5)

-- Common patterns from previous analysis:
-- - Editor_Cross: axis1=0 (X), axis2=1 (Y), scale_u=3, scale_v=3
-- - Board: axis1=0 (X), axis2=1 (Y), scale_u=1, scale_v=1
-- Both use offset_u=0, offset_v=negative scale_v

mappingInfo = {
  axis1 = 0,  -- Assuming X (most common)
  axis2 = 1,  -- Assuming Y (most common)
  scale_u = estimatedScaleU,
  scale_v = estimatedScaleV,
  offset_u = 0.0,
  offset_v = maxV  -- V ranges from offset_v to 0
}

local axisNames = { [0]="X", [1]="Y", [2]="Z" }

print("   ⚠️  ESTIMATED UV Parameters (planarMap was not called!):")
print(string.format("      axis1     = %d (%s) - ASSUMED", mappingInfo.axis1, axisNames[mappingInfo.axis1]))
print(string.format("      axis2     = %d (%s) - ASSUMED", mappingInfo.axis2, axisNames[mappingInfo.axis2]))
print(string.format("      scale_u   = %.6f (from UV range)", mappingInfo.scale_u))
print(string.format("      scale_v   = %.6f (from UV range)", mappingInfo.scale_v))
print(string.format("      offset_u  = %.6f (assumed 0)", mappingInfo.offset_u))
print(string.format("      offset_v  = %.6f (from max V)", mappingInfo.offset_v))
print("")
print("   🎨 Texture Tiling: " .. estimatedScaleU .. "x" .. estimatedScaleV)
print("")

print("┌────────────────────────────────────────────────────────────────┐")
print("│ STEP 5: Mesh Analysis                                         │")
print("└────────────────────────────────────────────────────────────────┘")
print("")

print("📏 Mesh Dimensions:")
print(string.format("   X: %.2f to %.2f (size: %.2f)", minX, maxX, sizeX))
print(string.format("   Y: %.2f to %.2f (size: %.2f)", minY, maxY, sizeY))
print(string.format("   Z: %.2f to %.2f (size: %.2f)", minZ, maxZ, sizeZ))
print("")

local maxDim = math.max(sizeX, sizeY, sizeZ)
local minDim = math.min(sizeX, sizeY, sizeZ)
local midDim = sizeX + sizeY + sizeZ - maxDim - minDim
local aspectRatio = maxDim / midDim

print(string.format("📐 Aspect Ratio: %.2f:1", aspectRatio))
if aspectRatio > 3.0 then
  print("   Type: ELONGATED (board-like)")
else
  print("   Type: CUBIC (box-like)")
end
print("")

print("🎨 UV Ranges:")
print(string.format("   U: %.4f to %.4f (range: %.4f)", minU, maxU, rangeU))
print(string.format("   V: %.4f to %.4f (range: %.4f)", minV, maxV, rangeV))
print("")

-- Check for duplicate vertices (UV seaming)
print("🔍 Checking for UV seaming (duplicate vertices)...")
local duplicates = 0
local tolerance = 0.001

for i = 0, math.min(99, vertexCount - 1) do
  local x1 = vertices[i * 3 + 1]
  local y1 = vertices[i * 3 + 2]
  local z1 = vertices[i * 3 + 3]
  local u1 = uvs[i * 2 + 1]
  local v1 = uvs[i * 2 + 2]
  
  for j = i + 1, math.min(99, vertexCount - 1) do
    local x2 = vertices[j * 3 + 1]
    local y2 = vertices[j * 3 + 2]
    local z2 = vertices[j * 3 + 3]
    local u2 = uvs[j * 2 + 1]
    local v2 = uvs[j * 2 + 2]
    
    local posDist = math.sqrt((x2-x1)^2 + (y2-y1)^2 + (z2-z1)^2)
    local uvDist = math.sqrt((u2-u1)^2 + (v2-v1)^2)
    
    if posDist < tolerance and uvDist > tolerance then
      duplicates = duplicates + 1
    end
  end
end

print(string.format("   Found %d duplicate positions with different UVs", duplicates))
if duplicates > 0 then
  print("   ✅ UV seaming detected")
else
  print("   ⚠️  No UV seaming detected (checked first 100 vertices)")
end
print("")

-- Note: Skip degenerate triangle check - mesh uses indexed geometry
-- We don't have the index buffer in this structure
local degenerateCount = 0

print("")

-- ============================================================================
-- STEP 6: Generate TypeScript output
-- ============================================================================

print("╔════════════════════════════════════════════════════════════════╗")
print("║                    TYPESCRIPT OUTPUT                           ║")
print("╚════════════════════════════════════════════════════════════════╝")
print("")
print("Copy everything below into:")
print("  threejs/src/meshViewer/gameExtracted_" .. meshName .. ".ts")
print("")
print("════════════════════════════════════════════════════════════════")
print("")

-- Generate TypeScript file
print("// Game-extracted complete mesh data for: " .. meshName)
print("// Extracted on: " .. os.date("%Y-%m-%d %H:%M:%S"))
print("// Source: extractEverything.lua")
print("// ")
print("// ⚠️  IMPORTANT: planarMap() was NOT called for this mesh!")
print("//    UVs are pre-computed or calculated differently.")
print("//    The MappingInfo below is REVERSE-ENGINEERED from the actual UV data.")
print("")

if mappingInfo then
  local axisNames = { [0]="X", [1]="Y", [2]="Z" }
  print("// MappingInfo (ESTIMATED from UV data - NOT from planarMap!)")
  print("export const mappingInfo_" .. meshName .. " = {")
  print(string.format("  axis1: %d,     // %s - ASSUMED (verify this!)", mappingInfo.axis1, axisNames[mappingInfo.axis1] or "?"))
  print(string.format("  axis2: %d,     // %s - ASSUMED (verify this!)", mappingInfo.axis2, axisNames[mappingInfo.axis2] or "?"))
  print(string.format("  scale_u: %.6f,  // Calculated from UV range", mappingInfo.scale_u))
  print(string.format("  scale_v: %.6f,  // Calculated from UV range", mappingInfo.scale_v))
  print(string.format("  offset_u: %.6f, // Assumed", mappingInfo.offset_u))
  print(string.format("  offset_v: %.6f  // From max V coordinate", mappingInfo.offset_v))
  print("};")
  print("")
end

print("export const gameExtracted_" .. meshName .. " = {")
print("  vertexCount: " .. vertexCount .. ",")
print("  triangleCount: " .. triangleCount .. ",")
print("")
print("  // Position data (x, y, z for each vertex)")
print("  positions: new Float32Array([")

-- Print positions in chunks
for i = 1, #vertices, 30 do
  local line = "    "
  for j = i, math.min(i + 29, #vertices) do
    line = line .. string.format("%.6f", vertices[j])
    if j < #vertices then
      line = line .. ", "
    end
  end
  print(line)
end

print("  ]),")
print("")
print("  // UV data (u, v for each vertex)")
print("  uvs: new Float32Array([")

-- Print UVs in chunks
for i = 1, #uvs, 20 do
  local line = "    "
  for j = i, math.min(i + 19, #uvs) do
    line = line .. string.format("%.6f", uvs[j])
    if j < #uvs then
      line = line .. ", "
    end
  end
  print(line)
end

print("  ]),")
print("")

-- Extract index buffer if it exists, otherwise generate sequential indices
print("  // Index data (triangle list - each set of 3 indices forms a triangle)")
print("  indices: new Uint16Array([")

if indexBufferPtr then
  -- Extract actual indices from game
  print("    // ACTUAL GAME INDICES (from index buffer)")
  local indicesPerLine = 30
  for i = 0, triangleCount * 3 - 1, indicesPerLine do
    local line = "    "
    for j = i, math.min(i + indicesPerLine - 1, triangleCount * 3 - 1) do
      local idx = readSmallInteger(indexBufferPtr + j * 2)  -- Assuming 16-bit indices
      line = line .. string.format("%d", idx)
      if j < triangleCount * 3 - 1 then
        line = line .. ", "
      end
    end
    print(line)
  end
else
  -- Generate sequential indices for non-indexed geometry
  print("    // GENERATED INDICES (no index buffer found - triangle list assumed)")
  local indicesPerLine = 30
  for i = 0, triangleCount * 3 - 1, indicesPerLine do
    local line = "    "
    for j = i, math.min(i + indicesPerLine - 1, triangleCount * 3 - 1) do
      line = line .. string.format("%d", j)
      if j < triangleCount * 3 - 1 then
        line = line .. ", "
      end
    end
    print(line)
  end
end

print("  ]),")
print("")
print("  // Metadata")
print("  metadata: {")
print("    bounds: {")
print(string.format("      x: { min: %.6f, max: %.6f },", minX, maxX))
print(string.format("      y: { min: %.6f, max: %.6f },", minY, maxY))
print(string.format("      z: { min: %.6f, max: %.6f }", minZ, maxZ))
print("    },")
print("    uvBounds: {")
print(string.format("      u: { min: %.6f, max: %.6f },", minU, maxU))
print(string.format("      v: { min: %.6f, max: %.6f }", minV, maxV))
print("    },")
print("    analysis: {")
print(string.format("      aspectRatio: %.2f,", aspectRatio))
print(string.format("      estimatedTiling: { u: %d, v: %d },", estimatedScaleU, estimatedScaleV))
print(string.format("      duplicateVertices: %d,", duplicates))
print(string.format("      degenerateTriangles: %d,", degenerateCount))
print(string.format("      hasIndexBuffer: %s", indexBufferPtr and "true" or "false"))
print("    }")
print("  }")
print("};")
print("")
print("════════════════════════════════════════════════════════════════")
print("")
print("╔════════════════════════════════════════════════════════════════╗")
print("║                      EXTRACTION COMPLETE                       ║")
print("╚════════════════════════════════════════════════════════════════╝")
print("")
print("✅ Extracted:")
print("   • " .. vertexCount .. " vertices (positions + UVs + normals)")
print("   • " .. triangleCount .. " triangles")
print("   • Mesh analysis (bounds, aspect ratio, etc.)")
print("")
print("⚠️  CRITICAL FINDING:")
print("   planarMap() was NOT called! This means:")
print("   • UVs are pre-computed in the HX file, OR")
print("   • UVs are generated by a DIFFERENT function, OR")
print("   • UVs are loaded from a separate data source")
print("")
print("   The extracted UV data above is the ACTUAL game data.")
print("   Compare this directly with Three.js to find the difference!")
print("")
print("📋 Next steps:")
print("   1. Copy the TypeScript output above")
print("   2. Save to: threejs/src/meshViewer/gameExtracted_" .. meshName .. ".ts")
print("   3. Use compareGeometry.ts to compare with Three.js rendering")
print("   4. The bug is probably: Three.js generating UVs differently than game")
print("   5. Solution: Use the extracted UV data DIRECTLY instead of generating")
print("")
print("═══════════════════════════════════════════════════════════════")
