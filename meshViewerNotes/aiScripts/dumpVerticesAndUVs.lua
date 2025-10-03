-- Dump both vertices and UVs from srMeshModel
-- Based on sr.c source code analysis

local modelPtr = 0xF270C40  -- Update from EAX
print(string.format("srMeshModel pointer: 0x%X\n", modelPtr))

-- Get vertex count
local vertexCount = readInteger(modelPtr + 0x234)
print(string.format("Vertex count: %d\n", vertexCount))

-- Get vertex positions at offset 0x1DC (from sr.c line 69991)
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
print(string.format("Vertex array pointer: 0x%X", vertexArrayPtr or 0))

-- UV coordinates at offset 0x13C (from sr.c line 70737)
local uvArrayPtr = readInteger(modelPtr + 0x13C)
print(string.format("UV array pointer: 0x%X\n", uvArrayPtr or 0))

if vertexArrayPtr and uvArrayPtr and vertexArrayPtr > 0 and uvArrayPtr > 0 then
    print("=== TypeScript Float32Array format ===")
    print("export const editorCrossUVs = new Float32Array([")
    for i = 0, vertexCount - 1 do
        local u = readFloat(uvArrayPtr + i * 8)
        local v = readFloat(uvArrayPtr + i * 8 + 4)
        local comment = string.format("  // [%d]", i)
        if i < vertexCount - 1 then
            print(string.format("  %.6f, %.6f,%s", u, v, comment))
        else
            print(string.format("  %.6f, %.6f %s", u, v, comment))
        end
    end
    print("]);")
else
    print("ERROR: Could not read vertex or UV arrays")
end


