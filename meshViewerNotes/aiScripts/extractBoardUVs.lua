-- Extract UV data for BOARD mesh from Supreme Snowboarding game memory
-- Compare with Editor_Cross to identify differences in UV processing
--
-- INSTRUCTIONS:
-- 1. Load Supreme Snowboarding
-- 2. Attach Cheat Engine
-- 3. Set breakpoint at getModelByName (address from previous sessions)
-- 4. Load ANY board mesh/model in game (character selection, track editor, etc.)
-- 5. When breakpoint hits, get srMeshModel pointer from EAX register
-- 6. Update modelPtr below with EAX value
-- 7. Execute this script
-- 8. Copy output and save to: threejs/src/meshViewer/boardCompleteData.ts

local modelPtr = 0xF270C40  -- ⚠️ UPDATE THIS from EAX register!

print("=" .. string.rep("=", 70))
print("  BOARD MESH UV EXTRACTION")
print("=" .. string.rep("=", 70))
print()

-- Get mesh metadata
local vertexCount = readInteger(modelPtr + 0x234)
local triangleCount = readInteger(modelPtr + 0x230)

if not vertexCount or not triangleCount then
    print("❌ ERROR: Could not read vertex/triangle counts!")
    print("   Make sure modelPtr is correct")
    return
end

print(string.format("Mesh Info:"))
print(string.format("  Vertices: %d", vertexCount))
print(string.format("  Triangles: %d", triangleCount))
print()

-- Get array pointers
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
local uvArrayPtr = readInteger(modelPtr + 0x13C)

if not vertexArrayPtr or not uvArrayPtr or vertexArrayPtr == 0 or uvArrayPtr == 0 then
    print("❌ ERROR: Invalid array pointers!")
    print(string.format("   Vertex array: 0x%X", vertexArrayPtr or 0))
    print(string.format("   UV array: 0x%X", uvArrayPtr or 0))
    return
end

print(string.format("Memory Pointers:"))
print(string.format("  srMeshModel: 0x%X", modelPtr))
print(string.format("  Vertex array: 0x%X", vertexArrayPtr))
print(string.format("  UV array: 0x%X", uvArrayPtr))
print()

-- Calculate bounding box and dimensions
print("Calculating bounding box...")
local minX, maxX = 99999, -99999
local minY, maxY = 99999, -99999
local minZ, maxZ = 99999, -99999

for i = 0, vertexCount - 1 do
    local x = readFloat(vertexArrayPtr + i * 12)
    local y = readFloat(vertexArrayPtr + i * 12 + 4)
    local z = readFloat(vertexArrayPtr + i * 12 + 8)
    
    if x then minX = math.min(minX, x); maxX = math.max(maxX, x) end
    if y then minY = math.min(minY, y); maxY = math.max(maxY, y) end
    if z then minZ = math.min(minZ, z); maxZ = math.max(maxZ, z) end
end

local sizeX = maxX - minX
local sizeY = maxY - minY
local sizeZ = maxZ - minZ
local aspectRatio = sizeY / math.max(sizeX, sizeZ)

print(string.format("  X: %.2f to %.2f (size: %.2f)", minX, maxX, sizeX))
print(string.format("  Y: %.2f to %.2f (size: %.2f)", minY, maxY, sizeY))
print(string.format("  Z: %.2f to %.2f (size: %.2f)", minZ, maxZ, sizeZ))
print(string.format("  Aspect ratio: %.2f:1", aspectRatio))
print()

-- Check if this is actually a board (elongated Y)
if aspectRatio > 3.0 then
    print("✅ Confirmed: This is an ELONGATED mesh (likely a board)")
else
    print("⚠️  WARNING: This doesn't look like a board (aspect ratio < 3:1)")
    print("   Board should have large Y dimension compared to X/Z")
end
print()

-- Analyze UV ranges
print("Analyzing UV data...")
local minU, maxU = 99999, -99999
local minV, maxV = 99999, -99999

for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    
    if u then minU = math.min(minU, u); maxU = math.max(maxU, u) end
    if v then minV = math.min(minV, v); maxV = math.max(maxV, v) end
end

local rangeU = maxU - minU
local rangeV = maxV - minV

print(string.format("UV Ranges:"))
print(string.format("  U: %.6f to %.6f (range: %.6f)", minU, maxU, rangeU))
print(string.format("  V: %.6f to %.6f (range: %.6f)", minV, maxV, rangeV))
print()

-- Compare with Editor_Cross reference values
print("Comparison with Editor_Cross:")
print("  Editor_Cross UV ranges:")
print("    U: [0.001, 2.999] (range: ~3.0)")
print("    V: [-2.999, -0.001] (range: ~3.0, NEGATIVE)")
print()
print("  Board UV ranges:")
print(string.format("    U: [%.3f, %.3f] (range: %.3f)", minU, maxU, rangeU))
print(string.format("    V: [%.3f, %.3f] (range: %.3f)%s", minV, maxV, rangeV, 
    (minV < 0) and " ✅ NEGATIVE" or " ⚠️  POSITIVE"))
print()

-- Estimate scale and offset values
print("Estimated UV Parameters:")

-- Calculate what scale_u would be to produce this U range
local estimated_scale_u = rangeU  -- Approximation
print(string.format("  scale_u ≈ %.3f (based on U range)", estimated_scale_u))

-- Calculate what scale_v would be
local estimated_scale_v = rangeV
print(string.format("  scale_v ≈ %.3f (based on V range)", estimated_scale_v))

-- Estimate offsets (if V is negative)
if minV < 0 then
    print(string.format("  offset_v ≈ %.3f (to shift V range to negative)", minV))
else
    print("  offset_v: Unknown (V is positive, unexpected!)")
end
print()

-- Show first 10 UVs for verification
print("First 10 UV coordinates:")
print()
print("Index | U          | V          | Position (X, Y, Z)")
print("------|------------|------------|--------------------------------")

for i = 0, 9 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    local x = readFloat(vertexArrayPtr + i * 12)
    local y = readFloat(vertexArrayPtr + i * 12 + 4)
    local z = readFloat(vertexArrayPtr + i * 12 + 8)
    
    print(string.format("[%2d]  | %10.6f | %10.6f | (%7.2f, %7.2f, %7.2f)",
        i, u or 0, v or 0, x or 0, y or 0, z or 0))
end
print()

-- Full data dump for TypeScript
print("=" .. string.rep("=", 70))
print("TYPESCRIPT DATA (copy this to boardCompleteData.ts)")
print("=" .. string.rep("=", 70))
print()

print("// Board mesh extracted from Supreme Snowboarding game memory")
print("// This data is used to verify UV generation algorithm")
print()
print("export const boardMeshData = {")
print("  metadata: {")
print(string.format("    vertexCount: %d,", vertexCount))
print(string.format("    triangleCount: %d,", triangleCount))
print(string.format("    dimensions: { x: %.2f, y: %.2f, z: %.2f },", sizeX, sizeY, sizeZ))
print(string.format("    aspectRatio: %.2f,", aspectRatio))
print(string.format("    uvRanges: { u: [%.6f, %.6f], v: [%.6f, %.6f] }", minU, maxU, minV, maxV))
print("  },")
print()
print("  vertices: new Float32Array([")

for i = 0, vertexCount - 1 do
    local x = readFloat(vertexArrayPtr + i * 12)
    local y = readFloat(vertexArrayPtr + i * 12 + 4)
    local z = readFloat(vertexArrayPtr + i * 12 + 8)
    local suffix = (i < vertexCount - 1) and "," or ""
    print(string.format("    %.6f, %.6f, %.6f%s  // [%d]", x, y, z, suffix, i))
end

print("  ]),")
print()
print("  uvs: new Float32Array([")

for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    local suffix = (i < vertexCount - 1) and "," or ""
    print(string.format("    %.6f, %.6f%s  // [%d]", u, v, suffix, i))
end

print("  ])")
print("};")
print()

print("=" .. string.rep("=", 70))
print("NEXT STEPS")
print("=" .. string.rep("=", 70))
print()
print("1. Save the TypeScript data above to:")
print("   threejs/src/meshViewer/boardCompleteData.ts")
print()
print("2. Compare board UVs with Editor_Cross:")
print("   - Are V coordinates negative? (Editor_Cross: YES)")
print("   - What are the UV ranges? (Editor_Cross: [0,3] and [-3,0])")
print("   - Are scale values similar? (Editor_Cross: scale=3.0)")
print()
print("3. Update runMeshViewer.ts with board-specific parameters")
print()
print("4. Test with multiple board models to verify consistency")
print()
