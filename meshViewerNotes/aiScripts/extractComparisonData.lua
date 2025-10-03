-- Comprehensive Comparison Data Extractor
-- This script extracts EVERYTHING we need to understand rendering differences
-- Run this for BOTH Editor_Cross and board meshes

print("=== COMPREHENSIVE MESH DATA EXTRACTOR ===")
print("")
print("INSTRUCTIONS:")
print("1. Set breakpoint at getModelByName function")
print("2. Load mesh in game (Editor_Cross or board)")
print("3. When breakpoint hits, note the EAX value (mesh pointer)")
print("4. Update meshPtr below with that value")
print("5. Run this script")
print("")

-- USER: UPDATE THIS VALUE WHEN BREAKPOINT HITS
local meshPtr = 0xBF244E0  -- Replace with EAX value from breakpoint

print("Analyzing mesh at pointer: " .. string.format("0x%X", meshPtr))
print("")

-- Read mesh structure
-- From HMG_3DE.c analysis:
-- offset +0x00: vtable pointer
-- offset +0x04: vertex count
-- offset +0x08: vertex array pointer
-- offset +0x0C: triangle count
-- offset +0x10: triangle array pointer (maybe?)

local success, vertexCount = pcall(readInteger, meshPtr + 0x04)
if not success then
  print("❌ Failed to read mesh data - invalid pointer!")
  print("   Make sure you updated meshPtr with the value from EAX")
  return
end

local vertexArrayPtr = readInteger(meshPtr + 0x08)
local triangleCount = readInteger(meshPtr + 0x0C)

print("=== MESH STRUCTURE ===")
print(string.format("Mesh pointer:     0x%X", meshPtr))
print(string.format("Vertex count:     %d", vertexCount))
print(string.format("Vertex array ptr: 0x%X", vertexArrayPtr))
print(string.format("Triangle count:   %d", triangleCount))
print("")

-- Calculate mesh dimensions and aspect ratio
local minX, maxX = math.huge, -math.huge
local minY, maxY = math.huge, -math.huge
local minZ, maxZ = math.huge, -math.huge

for i = 0, vertexCount - 1 do
  local vPtr = vertexArrayPtr + (i * 0x368)  -- Each vertex is 0x368 bytes
  local x = readFloat(vPtr + 0x30)  -- Position offset
  local y = readFloat(vPtr + 0x34)
  local z = readFloat(vPtr + 0x38)
  
  minX = math.min(minX, x)
  maxX = math.max(maxX, x)
  minY = math.min(minY, y)
  maxY = math.max(maxY, y)
  minZ = math.min(minZ, z)
  maxZ = math.max(maxZ, z)
end

local sizeX = maxX - minX
local sizeY = maxY - minY
local sizeZ = maxZ - minZ
local maxDim = math.max(sizeX, sizeY, sizeZ)
local minDim = math.min(sizeX, sizeY, sizeZ)
local midDim = sizeX + sizeY + sizeZ - maxDim - minDim
local aspectRatio = maxDim / midDim

print("=== MESH DIMENSIONS ===")
print(string.format("X: %.2f to %.2f (size: %.2f)", minX, maxX, sizeX))
print(string.format("Y: %.2f to %.2f (size: %.2f)", minY, maxY, sizeY))
print(string.format("Z: %.2f to %.2f (size: %.2f)", minZ, maxZ, sizeZ))
print(string.format("Max dimension: %.2f", maxDim))
print(string.format("Aspect ratio: %.2f:1", aspectRatio))
print("")

-- Determine mesh type
local meshType = "UNKNOWN"
if aspectRatio > 3.0 then
  meshType = "ELONGATED (board-like)"
else
  meshType = "CUBIC (box-like)"
end
print("Mesh type: " .. meshType)
print("")

-- Extract UV data for first 20 vertices
-- UV coordinates are stored at offset: (stage + 0x1e + layer * 2) * 8
-- For layer 0, stage 0: offset = 0x1e * 8 = 240 (0xF0)
print("=== UV COORDINATES (first 20 vertices) ===")
print("Idx | Position (X, Y, Z)                  | UV (U, V)")
print("----|-------------------------------------|--------------------")

local minU, maxU = math.huge, -math.huge
local minV, maxV = math.huge, -math.huge

for i = 0, math.min(19, vertexCount - 1) do
  local vPtr = vertexArrayPtr + (i * 0x368)
  local x = readFloat(vPtr + 0x30)
  local y = readFloat(vPtr + 0x34)
  local z = readFloat(vPtr + 0x38)
  
  -- UV coordinates at offset 0xF0 for layer 0, stage 0
  local u = readFloat(vPtr + 0xF0)
  local v = readFloat(vPtr + 0xF4)
  
  minU = math.min(minU, u)
  maxU = math.max(maxU, u)
  minV = math.min(minV, v)
  maxV = math.max(maxV, v)
  
  print(string.format("[%02d] | (%8.2f, %8.2f, %8.2f) | (%8.4f, %8.4f)", 
    i, x, y, z, u, v))
end

print("")
print("=== UV RANGES (all vertices) ===")

-- Scan all vertices for UV ranges
minU, maxU = math.huge, -math.huge
minV, maxV = math.huge, -math.huge

for i = 0, vertexCount - 1 do
  local vPtr = vertexArrayPtr + (i * 0x368)
  local u = readFloat(vPtr + 0xF0)
  local v = readFloat(vPtr + 0xF4)
  
  minU = math.min(minU, u)
  maxU = math.max(maxU, u)
  minV = math.min(minV, v)
  maxV = math.max(maxV, v)
end

local rangeU = maxU - minU
local rangeV = maxV - minV

print(string.format("U: %.6f to %.6f (range: %.6f)", minU, maxU, rangeU))
print(string.format("V: %.6f to %.6f (range: %.6f)", minV, maxV, rangeV))
print("")

-- Estimate UV parameters
local estimatedScaleU = math.floor(rangeU + 0.5)
local estimatedScaleV = math.floor(math.abs(rangeV) + 0.5)
local estimatedOffsetV = maxV  -- Assumes V goes from offset_v to 0

print("=== ESTIMATED UV PARAMETERS ===")
print(string.format("scale_u ≈ %.1f (UV range: %.1f)", estimatedScaleU, rangeU))
print(string.format("scale_v ≈ %.1f (UV range: %.1f)", estimatedScaleV, rangeV))
print(string.format("offset_v ≈ %.3f", estimatedOffsetV))
print("")

-- Compare with known patterns
print("=== COMPARISON WITH KNOWN PATTERNS ===")
print("")
print("Editor_Cross (expected):")
print("  - Aspect ratio: ~1:1 (cubic)")
print("  - UV range: U=[0, 3], V=[-3, 0]")
print("  - scale_u=3.0, scale_v=3.0, offset_v=-3.0")
print("")
print("Board (expected):")
print("  - Aspect ratio: ~4:1 (elongated)")
print("  - UV range: U=[0, 1], V=[-1, 0]")
print("  - scale_u=1.0, scale_v=1.0, offset_v=-1.0")
print("")

-- Determine which pattern this matches
local pattern = "UNKNOWN"
if math.abs(estimatedScaleU - 3.0) < 0.1 and math.abs(estimatedScaleV - 3.0) < 0.1 then
  pattern = "✅ MATCHES Editor_Cross (3x3 tiling)"
elseif math.abs(estimatedScaleU - 1.0) < 0.1 and math.abs(estimatedScaleV - 1.0) < 0.1 then
  pattern = "✅ MATCHES Board (1x1 tiling)"
else
  pattern = "⚠️  NEW PATTERN - Document this!"
end

print("Pattern match: " .. pattern)
print("")

-- Extract HX file comparison data
print("=== HX FILE COMPARISON DATA ===")
print("If you have the HX file loaded, compare:")
print(string.format("  HX vertices: ?? (check file) vs Game vertices: %d", vertexCount))
print(string.format("  Expansion ratio: %d / ?? = ??x", vertexCount))
print("")
print("NOTE: Game expands HX vertices for UV seaming")
print("  - Editor_Cross: 32 HX vertices → 96 game vertices (3x expansion)")
print("  - Board: 25 HX vertices → 60 game vertices (2.4x expansion)")
print("")

-- Check for vertex duplication (UV seaming)
print("=== VERTEX DUPLICATION ANALYSIS ===")
print("Checking for vertices at same position with different UVs...")

local duplicates = 0
local tolerance = 0.001

for i = 0, math.min(vertexCount - 1, 99) do
  local vPtr1 = vertexArrayPtr + (i * 0x368)
  local x1 = readFloat(vPtr1 + 0x30)
  local y1 = readFloat(vPtr1 + 0x34)
  local z1 = readFloat(vPtr1 + 0x38)
  local u1 = readFloat(vPtr1 + 0xF0)
  local v1 = readFloat(vPtr1 + 0xF4)
  
  for j = i + 1, math.min(vertexCount - 1, 99) do
    local vPtr2 = vertexArrayPtr + (j * 0x368)
    local x2 = readFloat(vPtr2 + 0x30)
    local y2 = readFloat(vPtr2 + 0x34)
    local z2 = readFloat(vPtr2 + 0x38)
    local u2 = readFloat(vPtr2 + 0xF0)
    local v2 = readFloat(vPtr2 + 0xF4)
    
    -- Check if positions match but UVs differ
    local posDist = math.sqrt((x2-x1)^2 + (y2-y1)^2 + (z2-z1)^2)
    local uvDist = math.sqrt((u2-u1)^2 + (v2-v1)^2)
    
    if posDist < tolerance and uvDist > tolerance then
      duplicates = duplicates + 1
      if duplicates <= 5 then
        print(string.format("  [%d] & [%d]: pos=(%.2f,%.2f,%.2f) UV1=(%.3f,%.3f) UV2=(%.3f,%.3f)", 
          i, j, x1, y1, z1, u1, v1, u2, v2))
      end
    end
  end
end

print(string.format("Found %d duplicate positions with different UVs (checked first 100 vertices)", duplicates))
print("This confirms UV seaming is used!")
print("")

-- Summary and recommendations
print("=== SUMMARY & RECOMMENDATIONS ===")
print("")
print("Mesh Analysis:")
print(string.format("  - Type: %s", meshType))
print(string.format("  - Dimensions: %.1f x %.1f x %.1f", sizeX, sizeY, sizeZ))
print(string.format("  - Aspect: %.2f:1", aspectRatio))
print("")
print("UV Analysis:")
print(string.format("  - Range: U=[%.3f, %.3f], V=[%.3f, %.3f]", minU, maxU, minV, maxV))
print(string.format("  - Tiling: %dx%d", estimatedScaleU, estimatedScaleV))
print(string.format("  - Pattern: %s", pattern))
print("")
print("Key Findings:")
if estimatedScaleU ~= estimatedScaleV then
  print("  ⚠️  WARNING: U and V scales differ! Not square tiling!")
end
if minV >= 0 then
  print("  ⚠️  WARNING: V coordinates are positive (expected negative)!")
end
if duplicates > 0 then
  print("  ✅ UV seaming detected (vertex duplication)")
else
  print("  ⚠️  No UV seaming detected in first 100 vertices")
end
print("")

print("=== NEXT STEPS ===")
print("")
print("1. Run this script for BOTH Editor_Cross and board")
print("2. Compare the outputs side-by-side")
print("3. Look for differences in:")
print("   - UV ranges (should be U=[0,3],V=[-3,0] vs U=[0,1],V=[-1,0])")
print("   - Tiling (3x3 vs 1x1)")
print("   - Aspect ratio (1:1 vs 4:1)")
print("4. If UV ranges match expectations, the problem is in Three.js rendering")
print("5. If UV ranges DON'T match, need to investigate UV generation in game")
print("")
print("=== DONE ===")
