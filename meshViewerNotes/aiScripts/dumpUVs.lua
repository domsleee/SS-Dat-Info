-- Cheat Engine Lua script to dump srMeshModel UV coordinates
-- Based on srMeshModel::getVertexTexCoords at sr.c line 70716
-- UV array pointer is at: this + (textureIndex + polyIndex * 2) * 8 + 0x13C

local modelPtr = 0xF270C40  -- Update this from the EAX register value
print(string.format("srMeshModel pointer: 0x%X", modelPtr))

-- Read vertex count from offset 0x234
local vertexCount = readInteger(modelPtr + 0x234)
print(string.format("Vertex count at +0x234: %d", vertexCount or 0))

-- For poly 0, texture 0: offset = (0 + 0 * 2) * 8 + 0x13C = 0x13C
local uvOffset = 0x13C
local uvArrayPtrAddr = modelPtr + uvOffset
local uvArrayPtr = readInteger(uvArrayPtrAddr)

print(string.format("\n=== UV Array Info ==="))
print(string.format("UV array pointer address: 0x%X", uvArrayPtrAddr))
print(string.format("UV array pointer value: 0x%X", uvArrayPtr or 0))

if uvArrayPtr and uvArrayPtr > 0x1000000 and uvArrayPtr < 0x20000000 then
    print(string.format("\n=== UV Coordinates (first 32) ==="))
    for i = 0, 31 do
        local u = readFloat(uvArrayPtr + i * 8)
        local v = readFloat(uvArrayPtr + i * 8 + 4)
        if u and v then
            print(string.format("UV[%2d]: (%.6f, %.6f)", i, u, v))
        else
            print(string.format("UV[%2d]: ERROR reading", i))
            break
        end
    end
else
    print("UV array pointer is NULL or invalid")
    
    -- Check if there's size info
    local uvArraySize = readInteger(uvArrayPtrAddr + 4)
    print(string.format("UV array size at +0x%X: %d", uvOffset + 4, uvArraySize or 0))
end


