-- Dump complete Editor_Cross geometry from game memory
-- This includes the expanded 96-vertex mesh

local modelPtr = 0xF270C40  -- Update from EAX
print("srMeshModel pointer: 0x" .. string.format("%X", modelPtr))

-- Get vertex count at +0x234
local vertexCount = readInteger(modelPtr + 0x234)
print("Vertex count at +0x234: " .. (vertexCount or 0))

-- Triangle count might be at +0x22C or nearby - let's check several offsets
print("\nSearching for triangle count...")
for offset = 0x220, 0x240, 4 do
    local val = readInteger(modelPtr + offset)
    if val and val > 0 and val < 500 then
        print(string.format("  Offset +0x%X: %d", offset, val))
    end
end

-- Get vertex positions at offset 0x1DC
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
print("\nVertex array pointer at +0x1DC: 0x" .. string.format("%X", vertexArrayPtr or 0))

-- Get UV coordinates at offset 0x13C
local uvArrayPtr = readInteger(modelPtr + 0x13C)
print("UV array pointer at +0x13C: 0x" .. string.format("%X", uvArrayPtr or 0))

-- Try to find triangle index array
-- In the TriMesh structure (at this+0x23C), indices might be nearby
print("\nSearching for triangle indices array...")
local triMeshOffset = 0x23C
for offset = triMeshOffset, triMeshOffset + 0x100, 4 do
    local ptr = readInteger(modelPtr + offset)
    if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
        -- Try reading as indices (should be small integers)
        local i0 = readInteger(ptr)
        local i1 = readInteger(ptr + 4)
        local i2 = readInteger(ptr + 8)
        if i0 and i1 and i2 and i0 >= 0 and i0 < 200 and i1 >= 0 and i1 < 200 and i2 >= 0 and i2 < 200 then
            print(string.format("  Possible indices at offset +0x%X: ptr=0x%X, first tri=(%d,%d,%d)", 
                offset, ptr, i0, i1, i2))
        end
    end
end

print("\n=== IMPORTANT ===")
print("If vertex count is 180 (60 triangles * 3), the mesh is already non-indexed!")
print("If vertex count is 96, we need to find the index buffer.")
print("Current vertex count: " .. (vertexCount or 0))
