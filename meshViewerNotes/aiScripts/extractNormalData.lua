-- Normal Vector Extractor
-- Extracts vertex normals from the game
-- This helps understand if Three.js is calculating normals differently

print("=== NORMAL VECTOR EXTRACTOR ===")
print("")
print("Normals are used for lighting calculations")
print("Incorrect normals can cause rendering issues")
print("")

-- USER: UPDATE THIS VALUE
local meshPtr = 0xBF244E0  -- Replace with mesh pointer from breakpoint

local success, vertexCount = pcall(readInteger, meshPtr + 0x04)
if not success then
  print("❌ Failed to read mesh data!")
  return
end

local vertexArrayPtr = readInteger(meshPtr + 0x08)

print(string.format("Vertices: %d", vertexCount))
print(string.format("Vertex array: 0x%X", vertexArrayPtr))
print("")

-- Normal vectors are stored at offset 0x0C from vertex base
-- Each normal is 3 floats (nx, ny, nz)
print("=== FIRST 20 VERTEX NORMALS ===")
print("Idx | Normal (NX, NY, NZ)           | Length  | Valid?")
print("----|-------------------------------|---------|-------")

local validNormals = 0
local invalidNormals = 0

for i = 0, math.min(19, vertexCount - 1) do
  local vPtr = vertexArrayPtr + (i * 0x368)
  
  -- Position (for reference)
  local x = readFloat(vPtr + 0x30)
  local y = readFloat(vPtr + 0x34)
  local z = readFloat(vPtr + 0x38)
  
  -- Normal vector at offset 0x0C
  local nx = readFloat(vPtr + 0x0C)
  local ny = readFloat(vPtr + 0x10)
  local nz = readFloat(vPtr + 0x14)
  
  -- Calculate length (should be ~1.0 if normalized)
  local length = math.sqrt(nx*nx + ny*ny + nz*nz)
  
  local valid = "✅"
  if math.abs(length - 1.0) > 0.01 then
    valid = "❌"
    invalidNormals = invalidNormals + 1
  else
    validNormals = validNormals + 1
  end
  
  print(string.format("[%02d] | (%7.4f, %7.4f, %7.4f) | %6.4f | %s",
    i, nx, ny, nz, length, valid))
end

print("")
print("=== NORMAL VALIDATION (all vertices) ===")

validNormals = 0
invalidNormals = 0
local zeroNormals = 0

for i = 0, vertexCount - 1 do
  local vPtr = vertexArrayPtr + (i * 0x368)
  
  local nx = readFloat(vPtr + 0x0C)
  local ny = readFloat(vPtr + 0x10)
  local nz = readFloat(vPtr + 0x14)
  
  local length = math.sqrt(nx*nx + ny*ny + nz*nz)
  
  if length < 0.001 then
    zeroNormals = zeroNormals + 1
  elseif math.abs(length - 1.0) > 0.01 then
    invalidNormals = invalidNormals + 1
  else
    validNormals = validNormals + 1
  end
end

print(string.format("Valid normals (length ≈ 1.0): %d (%.1f%%)", 
  validNormals, validNormals / vertexCount * 100))
print(string.format("Invalid normals (length ≠ 1.0): %d (%.1f%%)", 
  invalidNormals, invalidNormals / vertexCount * 100))
print(string.format("Zero normals (length ≈ 0.0): %d (%.1f%%)", 
  zeroNormals, zeroNormals / vertexCount * 100))
print("")

-- Analyze normal distribution
print("=== NORMAL DISTRIBUTION ===")

local upFacing = 0    -- +Y normals
local downFacing = 0  -- -Y normals
local sideFacing = 0  -- mostly X/Z

for i = 0, vertexCount - 1 do
  local vPtr = vertexArrayPtr + (i * 0x368)
  
  local nx = readFloat(vPtr + 0x0C)
  local ny = readFloat(vPtr + 0x10)
  local nz = readFloat(vPtr + 0x14)
  
  -- Determine dominant direction
  local absNx = math.abs(nx)
  local absNy = math.abs(ny)
  local absNz = math.abs(nz)
  
  if absNy > absNx and absNy > absNz then
    if ny > 0 then
      upFacing = upFacing + 1
    else
      downFacing = downFacing + 1
    end
  else
    sideFacing = sideFacing + 1
  end
end

print(string.format("Up-facing (+Y): %d (%.1f%%)", upFacing, upFacing / vertexCount * 100))
print(string.format("Down-facing (-Y): %d (%.1f%%)", downFacing, downFacing / vertexCount * 100))
print(string.format("Side-facing (±X/±Z): %d (%.1f%%)", sideFacing, sideFacing / vertexCount * 100))
print("")

-- Check if normals are consistent with triangle orientation
print("=== NORMAL CONSISTENCY CHECK ===")
print("Checking if normals match computed face normals...")
print("")

local triangleCount = vertexCount / 3
local consistentNormals = 0
local inconsistentNormals = 0

for tri = 0, math.min(9, triangleCount - 1) do
  local v0Idx = tri * 3
  local v1Idx = tri * 3 + 1
  local v2Idx = tri * 3 + 2
  
  local v0Ptr = vertexArrayPtr + (v0Idx * 0x368)
  local v1Ptr = vertexArrayPtr + (v1Idx * 0x368)
  local v2Ptr = vertexArrayPtr + (v2Idx * 0x368)
  
  -- Positions
  local v0x = readFloat(v0Ptr + 0x30)
  local v0y = readFloat(v0Ptr + 0x34)
  local v0z = readFloat(v0Ptr + 0x38)
  
  local v1x = readFloat(v1Ptr + 0x30)
  local v1y = readFloat(v1Ptr + 0x34)
  local v1z = readFloat(v1Ptr + 0x38)
  
  local v2x = readFloat(v2Ptr + 0x30)
  local v2y = readFloat(v2Ptr + 0x34)
  local v2z = readFloat(v2Ptr + 0x38)
  
  -- Compute face normal using cross product
  local e1x = v1x - v0x
  local e1y = v1y - v0y
  local e1z = v1z - v0z
  
  local e2x = v2x - v0x
  local e2y = v2y - v0y
  local e2z = v2z - v0z
  
  -- Cross product: e1 × e2
  local fnx = e1y * e2z - e1z * e2y
  local fny = e1z * e2x - e1x * e2z
  local fnz = e1x * e2y - e1y * e2x
  
  -- Normalize
  local fnLen = math.sqrt(fnx*fnx + fny*fny + fnz*fnz)
  if fnLen > 0.001 then
    fnx = fnx / fnLen
    fny = fny / fnLen
    fnz = fnz / fnLen
  end
  
  -- Get vertex normal
  local vnx = readFloat(v0Ptr + 0x0C)
  local vny = readFloat(v0Ptr + 0x10)
  local vnz = readFloat(v0Ptr + 0x14)
  
  -- Dot product to check alignment
  local dot = vnx * fnx + vny * fny + vnz * fnz
  
  local status = "✅"
  if dot > 0.9 then
    consistentNormals = consistentNormals + 1
  else
    inconsistentNormals = inconsistentNormals + 1
    status = "❌"
  end
  
  print(string.format("Triangle %02d: Face normal=(%.2f,%.2f,%.2f) Vertex normal=(%.2f,%.2f,%.2f) Dot=%.2f %s",
    tri, fnx, fny, fnz, vnx, vny, vnz, dot, status))
end

print("")
print("=== SUMMARY ===")
print("")

if validNormals == vertexCount then
  print("✅ All normals are properly normalized")
else
  print("⚠️  Some normals are not normalized - this may cause lighting issues")
end

if zeroNormals > 0 then
  print("⚠️  Found zero-length normals - these will cause rendering problems!")
end

print("")
print("RECOMMENDATION:")
print("  - If Three.js computes normals differently, rendering WILL differ")
print("  - Three.js uses computeVertexNormals() which averages face normals")
print("  - Game may use pre-computed normals or different calculation")
print("  - Compare with Three.js normal calculation to find differences")
print("")
print("=== DONE ===")
