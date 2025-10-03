-- Find triangle indices for Editor_Cross

local modelPtr = 0xF270C40
print("srMeshModel pointer: 0x" .. string.format("%X", modelPtr))

local vertexCount = readInteger(modelPtr + 0x234)
local triangleCount = readInteger(modelPtr + 0x230)

print("Vertex count: " .. vertexCount)
print("Triangle count: " .. triangleCount)
print("Expected indices: " .. (triangleCount * 3))

-- The TriMesh structure starts at this+0x23C
-- Let's scan it more thoroughly for pointers to index data
print("\nScanning TriMesh structure for index array...")

local triMeshStart = modelPtr + 0x23C

-- Scan first 512 bytes of TriMesh for valid pointers
for offset = 0, 0x1FF, 4 do
    local ptr = readInteger(triMeshStart + offset)
    if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
        -- Check if this looks like an index array (180 integers for 60 triangles)
        local allValid = true
        local maxIdx = 0
        
        -- Check first 9 values (3 triangles)
        for i = 0, 8 do
            local idx = readInteger(ptr + i * 4)
            if not idx or idx < 0 or idx >= 200 then
                allValid = false
                break
            end
            if idx > maxIdx then
                maxIdx = idx
            end
        end
        
        if allValid and maxIdx < vertexCount then
            print(string.format("\n*** FOUND at TriMesh+0x%X (srMeshModel+0x%X) ***", offset, offset + 0x23C))
            print(string.format("Pointer: 0x%X", ptr))
            print("First 30 indices:")
            for i = 0, 29 do
                local idx = readInteger(ptr + i * 4)
                if i % 3 == 0 then
                    io.write(string.format("  Tri%d: ", i/3))
                end
                io.write(string.format("%d ", idx))
                if i % 3 == 2 then
                    print("")
                end
            end
        end
    end
end

print("\n=== Also check if indices are stored as shorts (uint16) instead of ints ===")
for offset = 0, 0x1FF, 4 do
    local ptr = readInteger(triMeshStart + offset)
    if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
        local allValid = true
        local maxIdx = 0
        
        -- Check as 16-bit indices
        for i = 0, 8 do
            local idx = readSmallInteger(ptr + i * 2)
            if not idx or idx < 0 or idx >= 200 then
                allValid = false
                break
            end
            if idx > maxIdx then
                maxIdx = idx
            end
        end
        
        if allValid and maxIdx < vertexCount then
            print(string.format("\n*** FOUND (uint16) at TriMesh+0x%X ***", offset))
            print(string.format("Pointer: 0x%X", ptr))
            print("First 30 indices:")
            for i = 0, 29 do
                local idx = readSmallInteger(ptr + i * 2)
                if i % 3 == 0 then
                    io.write(string.format("  Tri%d: ", i/3))
                end
                io.write(string.format("%d ", idx))
                if i % 3 == 2 then
                    print("")
                end
            end
        end
    end
end
