-- Verify UV data extraction from srMeshModel
-- Check if UVs are actually stored as we think

local modelPtr = 0xF270C40  -- UPDATE THIS from EAX register!

print("=== UV Data Verification ===\n")

-- Get vertex count
local vertexCount = readInteger(modelPtr + 0x234)
print(string.format("Vertex count at +0x234: %d", vertexCount or 0))

-- Get triangle count
local triangleCount = readInteger(modelPtr + 0x230)
print(string.format("Triangle count at +0x230: %d\n", triangleCount or 0))

-- Get UV array pointer
local uvArrayPtr = readInteger(modelPtr + 0x13C)
print(string.format("UV array pointer at +0x13C: 0x%X\n", uvArrayPtr or 0))

if not uvArrayPtr or uvArrayPtr == 0 then
    print("ERROR: UV array pointer is NULL!")
    return
end

-- Read first 10 UV pairs and show in multiple formats
print("First 10 UV coordinates:\n")
print("Index | As Floats (U, V)          | As Hex (U, V)     | As Integers")
print("------|---------------------------|-------------------|-------------")

for i = 0, 9 do
    local addr = uvArrayPtr + i * 8
    
    -- Read as floats
    local u_float = readFloat(addr)
    local v_float = readFloat(addr + 4)
    
    -- Read as raw bytes/hex
    local u_bytes = readInteger(addr)
    local v_bytes = readInteger(addr + 4)
    
    print(string.format("[%2d]  | (%9.6f, %9.6f) | (0x%08X, 0x%08X) | (%d, %d)",
        i, u_float or 0, v_float or 0, u_bytes or 0, v_bytes or 0, u_bytes or 0, v_bytes or 0))
end

-- Check if UVs look reasonable (should be roughly in range [-3, 3] based on our analysis)
print("\n=== Sanity Checks ===")
local allUVsOk = true
local minU, maxU, minV, maxV = 9999, -9999, 9999, -9999

for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    
    if u then minU = math.min(minU, u); maxU = math.max(maxU, u) end
    if v then minV = math.min(minV, v); maxV = math.max(maxV, v) end
    
    -- Check for obviously wrong values
    if not u or not v or math.abs(u) > 100 or math.abs(v) > 100 then
        print(string.format("WARNING: Vertex %d has suspicious UVs: (%.6f, %.6f)", i, u or 0, v or 0))
        allUVsOk = false
    end
end

print(string.format("\nUV Ranges across all %d vertices:", vertexCount))
print(string.format("  U: %.6f to %.6f (range: %.6f)", minU, maxU, maxU - minU))
print(string.format("  V: %.6f to %.6f (range: %.6f)", minV, maxV, maxV - minV))

if allUVsOk then
    print("\n✓ All UVs look reasonable!")
else
    print("\n✗ Some UVs look suspicious - check the pointer!")
end

-- Also check vertex positions for reference
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
print(string.format("\nVertex array pointer at +0x1DC: 0x%X", vertexArrayPtr or 0))

if vertexArrayPtr and vertexArrayPtr > 0 then
    print("\nFirst 3 vertex positions:")
    for i = 0, 2 do
        local x = readFloat(vertexArrayPtr + i * 12)
        local y = readFloat(vertexArrayPtr + i * 12 + 4)
        local z = readFloat(vertexArrayPtr + i * 12 + 8)
        print(string.format("  V%d: (%.6f, %.6f, %.6f)", i, x or 0, y or 0, z or 0))
    end
end

print("\n=== Instructions ===")
print("1. Set breakpoint at getModelByName")
print("2. Load Editor_Cross in game")
print("3. Get srMeshModel pointer from EAX")
print("4. Update modelPtr at top of this script")
print("5. Run this script to verify UV data looks correct")
