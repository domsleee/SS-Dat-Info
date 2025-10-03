-- Extract UV data for the SPECIFIC board.hx mesh (40 vertices, 76 triangles)
-- This should match the board.hx file in /meshes/board/board.hx
--
-- INSTRUCTIONS:
-- 1. Load Supreme Snowboarding
-- 2. Attach Cheat Engine
-- 3. Set breakpoint at getModelByName
-- 4. Load the Burton Custom '10 board in game
-- 5. When breakpoint hits, get srMeshModel pointer from EAX register
-- 6. Update modelPtr below with EAX value
-- 7. Execute this script

local modelPtr =  EAX   -- ⚠️ UPDATE THIS from EAX register!

print("=" .. string.rep("=", 70))
print("  BOARD.HX UV EXTRACTION (40 vertices, 76 triangles)")
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
print(string.format("  Vertices: %d (expecting 40 or more with seaming)", vertexCount))
print(string.format("  Triangles: %d (expecting 76)", triangleCount))
print()

if triangleCount ~= 76 then
    print("⚠️  WARNING: Triangle count doesn't match board.hx!")
    print("   board.hx has 76 triangles")
    print("   Make sure you loaded the Burton Custom '10 board")
end
print()

-- Get array pointers
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
local uvArrayPtr = readInteger(modelPtr + 0x13C)

if not vertexArrayPtr or not uvArrayPtr or vertexArrayPtr == 0 or uvArrayPtr == 0 then
    print("❌ ERROR: Invalid array pointers!")
    return
end

print(string.format("Memory Pointers:"))
print(string.format("  srMeshModel: 0x%X", modelPtr))
print(string.format("  Vertex array: 0x%X", vertexArrayPtr))
print(string.format("  UV array: 0x%X", uvArrayPtr))
print()

-- Show first 20 vertices and UVs
print("First 20 vertices with UVs:")
print()
print("Index | Position (X, Y, Z)              | UV (U, V)")
print("------|--------------------------------|------------------")

for i = 0, math.min(19, vertexCount - 1) do
    local x = readFloat(vertexArrayPtr + i * 12)
    local y = readFloat(vertexArrayPtr + i * 12 + 4)
    local z = readFloat(vertexArrayPtr + i * 12 + 8)
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    
    print(string.format("[%3d] | (%8.2f, %8.2f, %8.2f) | (%8.4f, %8.4f)",
        i, x or 0, y or 0, z or 0, u or 0, v or 0))
end
print()

-- Analyze UV ranges
local minU, maxU = 99999, -99999
local minV, maxV = 99999, -99999

for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    
    if u then minU = math.min(minU, u); maxU = math.max(maxU, u) end
    if v then minV = math.min(minV, v); maxV = math.max(maxV, v) end
end

print(string.format("UV Ranges:"))
print(string.format("  U: %.6f to %.6f", minU, maxU))
print(string.format("  V: %.6f to %.6f %s", minV, maxV, (minV < 0) and "✅ NEGATIVE" or ""))
print()

print("=" .. string.rep("=", 70))
print("TYPESCRIPT DATA (copy this to boardLod0CompleteData.ts)")
print("=" .. string.rep("=", 70))
print()

print("// Board_lod0 mesh extracted from Supreme Snowboarding game memory")
print("// 76 triangles, 60 vertices (with UV seaming)")
print()
print("export const boardLod0MeshData = {")
print("  metadata: {")
print(string.format("    vertexCount: %d,", vertexCount))
print(string.format("    triangleCount: %d,", triangleCount))
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
