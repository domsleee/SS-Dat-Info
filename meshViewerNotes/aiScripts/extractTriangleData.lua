-- Triangle and Geometry Structure Extractor
-- Extracts actual triangle data to understand how geometry is built
-- This will show us if Three.js is building triangles differently than the game

print("=== TRIANGLE GEOMETRY EXTRACTOR ===")
print("")

-- USER: UPDATE THIS VALUE
local meshPtr = 0xBF244E0  -- Replace with mesh pointer from breakpoint

print("Analyzing mesh at: " .. string.format("0x%X", meshPtr))
print("")

local vertexCount = readInteger(meshPtr + 0x04)
local vertexArrayPtr = readInteger(meshPtr + 0x08)

print("Vertex count: " .. vertexCount)
print("Vertex array: " .. string.format("0x%X", vertexArrayPtr))
print("")

-- The game stores vertices in a non-indexed format (triangle list)
-- So 3 consecutive vertices = 1 triangle
local triangleCount = vertexCount / 3

print("=== TRIANGLE STRUCTURE ===")
print(string.format("Triangles: %d (from %d vertices)", triangleCount, vertexCount))
print("")
print("First 10 triangles:")
print("Tri | V0 Position (X, Y, Z)       | V0 UV    | V1 Position (X, Y, Z)       | V1 UV    | V2 Position (X, Y, Z)       | V2 UV")
print("----|------------------------------|----------|------------------------------|----------|------------------------------|----------")

for tri = 0, math.min(9, triangleCount - 1) do
  local v0Idx = tri * 3
  local v1Idx = tri * 3 + 1
  local v2Idx = tri * 3 + 2
  
  local v0Ptr = vertexArrayPtr + (v0Idx * 0x368)
  local v1Ptr = vertexArrayPtr + (v1Idx * 0x368)
  local v2Ptr = vertexArrayPtr + (v2Idx * 0x368)
  
  local v0x, v0y, v0z = readFloat(v0Ptr + 0x30), readFloat(v0Ptr + 0x34), readFloat(v0Ptr + 0x38)
  local v0u, v0v = readFloat(v0Ptr + 0xF0), readFloat(v0Ptr + 0xF4)
  
  local v1x, v1y, v1z = readFloat(v1Ptr + 0x30), readFloat(v1Ptr + 0x34), readFloat(v1Ptr + 0x38)
  local v1u, v1v = readFloat(v1Ptr + 0xF0), readFloat(v1Ptr + 0xF4)
  
  local v2x, v2y, v2z = readFloat(v2Ptr + 0x30), readFloat(v2Ptr + 0x34), readFloat(v2Ptr + 0x38)
  local v2u, v2v = readFloat(v2Ptr + 0xF0), readFloat(v2Ptr + 0xF4)
  
  print(string.format("[%2d] | (%6.1f,%6.1f,%6.1f) | (%5.2f,%5.2f) | (%6.1f,%6.1f,%6.1f) | (%5.2f,%5.2f) | (%6.1f,%6.1f,%6.1f) | (%5.2f,%5.2f)",
    tri,
    v0x, v0y, v0z, v0u, v0v,
    v1x, v1y, v1z, v1u, v1v,
    v2x, v2y, v2z, v2u, v2v))
end

print("")
print("=== CHECKING FOR DEGENERATE TRIANGLES ===")

local degenerateCount = 0
for tri = 0, triangleCount - 1 do
  local v0Idx = tri * 3
  local v1Idx = tri * 3 + 1
  local v2Idx = tri * 3 + 2
  
  local v0Ptr = vertexArrayPtr + (v0Idx * 0x368)
  local v1Ptr = vertexArrayPtr + (v1Idx * 0x368)
  local v2Ptr = vertexArrayPtr + (v2Idx * 0x368)
  
  local v0x, v0y, v0z = readFloat(v0Ptr + 0x30), readFloat(v0Ptr + 0x34), readFloat(v0Ptr + 0x38)
  local v1x, v1y, v1z = readFloat(v1Ptr + 0x30), readFloat(v1Ptr + 0x34), readFloat(v1Ptr + 0x38)
  local v2x, v2y, v2z = readFloat(v2Ptr + 0x30), readFloat(v2Ptr + 0x34), readFloat(v2Ptr + 0x38)
  
  -- Check if all three vertices are the same (degenerate triangle)
  local tolerance = 0.001
  local d01 = math.sqrt((v1x-v0x)^2 + (v1y-v0y)^2 + (v1z-v0z)^2)
  local d12 = math.sqrt((v2x-v1x)^2 + (v2y-v1y)^2 + (v2z-v1z)^2)
  local d20 = math.sqrt((v0x-v2x)^2 + (v0y-v2y)^2 + (v0z-v2z)^2)
  
  if d01 < tolerance or d12 < tolerance or d20 < tolerance then
    degenerateCount = degenerateCount + 1
    if degenerateCount <= 5 then
      print(string.format("  Triangle %d is degenerate (edges: %.3f, %.3f, %.3f)", tri, d01, d12, d20))
    end
  end
end

if degenerateCount > 0 then
  print(string.format("⚠️  Found %d degenerate triangles!", degenerateCount))
else
  print("✅ No degenerate triangles found")
end
print("")

-- Export first 20 triangles for Three.js comparison
print("=== EXPORT FOR THREE.JS COMPARISON ===")
print("")
print("Copy this data to compare with Three.js rendering:")
print("")
print("const gameTriangles = [")

for tri = 0, math.min(19, triangleCount - 1) do
  local v0Idx = tri * 3
  local v1Idx = tri * 3 + 1
  local v2Idx = tri * 3 + 2
  
  local v0Ptr = vertexArrayPtr + (v0Idx * 0x368)
  local v1Ptr = vertexArrayPtr + (v1Idx * 0x368)
  local v2Ptr = vertexArrayPtr + (v2Idx * 0x368)
  
  local v0x, v0y, v0z = readFloat(v0Ptr + 0x30), readFloat(v0Ptr + 0x34), readFloat(v0Ptr + 0x38)
  local v0u, v0v = readFloat(v0Ptr + 0xF0), readFloat(v0Ptr + 0xF4)
  
  local v1x, v1y, v1z = readFloat(v1Ptr + 0x30), readFloat(v1Ptr + 0x34), readFloat(v1Ptr + 0x38)
  local v1u, v1v = readFloat(v1Ptr + 0xF0), readFloat(v1Ptr + 0xF4)
  
  local v2x, v2y, v2z = readFloat(v2Ptr + 0x30), readFloat(v2Ptr + 0x34), readFloat(v2Ptr + 0x38)
  local v2u, v2v = readFloat(v2Ptr + 0xF0), readFloat(v2Ptr + 0xF4)
  
  print(string.format("  { // Triangle %d", tri))
  print(string.format("    v0: { pos: [%f, %f, %f], uv: [%f, %f] },", v0x, v0y, v0z, v0u, v0v))
  print(string.format("    v1: { pos: [%f, %f, %f], uv: [%f, %f] },", v1x, v1y, v1z, v1u, v1v))
  print(string.format("    v2: { pos: [%f, %f, %f], uv: [%f, %f] }", v2x, v2y, v2z, v2u, v2v))
  
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
print("Use this data to:")
print("1. Verify Three.js is building the same triangles")
print("2. Check if vertex order matches")
print("3. Confirm UV coordinates are assigned correctly")
print("4. Look for any missing or extra triangles")
