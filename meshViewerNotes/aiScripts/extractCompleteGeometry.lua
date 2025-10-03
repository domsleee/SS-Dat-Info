-- Complete Geometry Extractor - Exports ALL mesh data in TypeScript format
-- This creates a file you can directly copy-paste into TypeScript for comparison
-- Run this for BOTH Editor_Cross and board meshes

print("=== COMPLETE GEOMETRY EXTRACTOR ===")
print("")
print("This will extract ALL vertices, triangles, and UVs in TypeScript format")
print("")

-- USER: UPDATE THESE VALUES
local meshPtr = 0xBF244E0  -- Replace with mesh pointer from breakpoint
local meshName = "board"   -- Change to "Editor_Cross" or "board"

print("Extracting mesh: " .. meshName)
print("Mesh pointer: " .. string.format("0x%X", meshPtr))
print("")

local success, vertexCount = pcall(readInteger, meshPtr + 0x04)
if not success then
  print("❌ Failed to read mesh data!")
  return
end

local vertexArrayPtr = readInteger(meshPtr + 0x08)
local triangleCount = vertexCount / 3

print(string.format("Vertices: %d", vertexCount))
print(string.format("Triangles: %d", triangleCount))
print("")
print("=== EXTRACTING DATA ===")

-- Read all vertex data
local vertices = {}
local uvs = {}

for i = 0, vertexCount - 1 do
  local vPtr = vertexArrayPtr + (i * 0x368)
  
  local x = readFloat(vPtr + 0x30)
  local y = readFloat(vPtr + 0x34)
  local z = readFloat(vPtr + 0x38)
  
  local u = readFloat(vPtr + 0xF0)
  local v = readFloat(vPtr + 0xF4)
  
  vertices[i * 3 + 1] = x
  vertices[i * 3 + 2] = y
  vertices[i * 3 + 3] = z
  
  uvs[i * 2 + 1] = u
  uvs[i * 2 + 2] = v
end

print("✅ Extracted all vertex data")
print("")
print("=== TYPESCRIPT OUTPUT ===")
print("")
print("Copy the following into a new .ts file:")
print("")
print("// Game-extracted geometry for: " .. meshName)
print("// Extracted on: " .. os.date("%Y-%m-%d %H:%M:%S"))
print("")
print("export const gameExtracted_" .. meshName .. " = {")
print("  vertexCount: " .. vertexCount .. ",")
print("  triangleCount: " .. triangleCount .. ",")
print("")
print("  // Position data (x, y, z for each vertex)")
print("  positions: new Float32Array([")

-- Print positions in chunks of 30 values (10 vertices per line)
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

-- Print UVs in chunks of 20 values (10 UV pairs per line)
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

-- Calculate metadata
local minX, maxX = math.huge, -math.huge
local minY, maxY = math.huge, -math.huge
local minZ, maxZ = math.huge, -math.huge
local minU, maxU = math.huge, -math.huge
local minV, maxV = math.huge, -math.huge

for i = 1, #vertices, 3 do
  minX = math.min(minX, vertices[i])
  maxX = math.max(maxX, vertices[i])
  minY = math.min(minY, vertices[i + 1])
  maxY = math.max(maxY, vertices[i + 1])
  minZ = math.min(minZ, vertices[i + 2])
  maxZ = math.max(maxZ, vertices[i + 2])
end

for i = 1, #uvs, 2 do
  minU = math.min(minU, uvs[i])
  maxU = math.max(maxU, uvs[i])
  minV = math.min(minV, uvs[i + 1])
  maxV = math.max(maxV, uvs[i + 1])
end

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
print("    }")
print("  }")
print("};")
print("")

-- Export individual triangles for detailed analysis
print("")
print("// Individual triangles for debugging")
print("export const triangles_" .. meshName .. " = [")

for tri = 0, math.min(19, triangleCount - 1) do
  local v0Idx = tri * 3
  local v1Idx = tri * 3 + 1
  local v2Idx = tri * 3 + 2
  
  print(string.format("  { // Triangle %d", tri))
  print(string.format("    v0: { pos: [%.6f, %.6f, %.6f], uv: [%.6f, %.6f] },",
    vertices[v0Idx * 3 + 1], vertices[v0Idx * 3 + 2], vertices[v0Idx * 3 + 3],
    uvs[v0Idx * 2 + 1], uvs[v0Idx * 2 + 2]))
  print(string.format("    v1: { pos: [%.6f, %.6f, %.6f], uv: [%.6f, %.6f] },",
    vertices[v1Idx * 3 + 1], vertices[v1Idx * 3 + 2], vertices[v1Idx * 3 + 3],
    uvs[v1Idx * 2 + 1], uvs[v1Idx * 2 + 2]))
  print(string.format("    v2: { pos: [%.6f, %.6f, %.6f], uv: [%.6f, %.6f] }",
    vertices[v2Idx * 3 + 1], vertices[v2Idx * 3 + 2], vertices[v2Idx * 3 + 3],
    uvs[v2Idx * 2 + 1], uvs[v2Idx * 2 + 2]))
  
  if tri < math.min(19, triangleCount - 1) then
    print("  },")
  else
    print("  }")
  end
end

print("];")
print("")
print("=== DONE ===")
print("")
print("Next steps:")
print("1. Copy the TypeScript output above")
print("2. Save to: threejs/src/meshViewer/gameExtracted_" .. meshName .. ".ts")
print("3. Run the comparison script to analyze differences")
