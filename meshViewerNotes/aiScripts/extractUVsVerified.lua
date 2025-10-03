-- Extract UVs with position matching for Editor_Cross
-- This helps verify we're reading the right data by matching positions

local modelPtr = 0xF270C40  -- UPDATE THIS from EAX!

print("=== Editor_Cross UV Extraction with Position Verification ===\n")

-- Get counts
local vertexCount = readInteger(modelPtr + 0x234)
local triangleCount = readInteger(modelPtr + 0x230)
print(string.format("Vertices: %d, Triangles: %d\n", vertexCount or 0, triangleCount or 0))

-- Get pointers
local vertexArrayPtr = readInteger(modelPtr + 0x1DC)
local uvArrayPtr = readInteger(modelPtr + 0x13C)

if not vertexArrayPtr or not uvArrayPtr or vertexArrayPtr == 0 or uvArrayPtr == 0 then
    print("ERROR: Invalid pointers!")
    return
end

print(string.format("Vertex array: 0x%X", vertexArrayPtr))
print(string.format("UV array: 0x%X\n", uvArrayPtr))

-- Known positions from HX file (first 6 unique vertices)
local knownPositions = {
    {16.75, 16.75, 0.0, "V0: Front face, top-right"},
    {50.25, 16.75, 0.0, "V1: Front face, far-right"},
    {50.25, -16.75, 0.0, "V2: Front face, bottom-right"},
    {-50.25, 16.75, 0.0, "V3: Front face, far-left"},
    {-16.75, 16.75, 0.0, "V4: Front face, top-left"},
    {-16.75, -16.75, 0.0, "V5: Front face, bottom-left"}
}

-- Find these positions and show their UVs
print("Looking for known positions:\n")

for idx, known in ipairs(knownPositions) do
    local kx, ky, kz, desc = known[1], known[2], known[3], known[4]
    
    -- Search through all vertices
    for i = 0, vertexCount - 1 do
        local x = readFloat(vertexArrayPtr + i * 12)
        local y = readFloat(vertexArrayPtr + i * 12 + 4)
        local z = readFloat(vertexArrayPtr + i * 12 + 8)
        
        -- Check if position matches (with small tolerance for floating point)
        if x and y and z then
            local dx = math.abs(x - kx)
            local dy = math.abs(y - ky)
            local dz = math.abs(z - kz)
            
            if dx < 0.01 and dy < 0.01 and dz < 0.01 then
                -- Found match - get UV
                local u = readFloat(uvArrayPtr + i * 8)
                local v = readFloat(uvArrayPtr + i * 8 + 4)
                
                print(string.format("%s", desc))
                print(string.format("  Index: %d", i))
                print(string.format("  Position: (%.2f, %.2f, %.2f)", x, y, z))
                print(string.format("  UV: (%.6f, %.6f)", u or 0, v or 0))
                print()
            end
        end
    end
end

-- Also dump all data to file for analysis
print("=== Complete Data Dump ===\n")
print("export const editorCrossCompleteData = {")
print("  vertices: new Float32Array([")

for i = 0, vertexCount - 1 do
    local x = readFloat(vertexArrayPtr + i * 12)
    local y = readFloat(vertexArrayPtr + i * 12 + 4)
    local z = readFloat(vertexArrayPtr + i * 12 + 8)
    local suffix = (i < vertexCount - 1) and "," or ""
    print(string.format("    %.6f, %.6f, %.6f%s  // [%d]", x, y, z, suffix, i))
end

print("  ]),")
print("  uvs: new Float32Array([")

for i = 0, vertexCount - 1 do
    local u = readFloat(uvArrayPtr + i * 8)
    local v = readFloat(uvArrayPtr + i * 8 + 4)
    local suffix = (i < vertexCount - 1) and "," or ""
    print(string.format("    %.6f, %.6f%s  // [%d]", u, v, suffix, i))
end

print("  ]),")
print("  vertexCount: " .. vertexCount .. ",")
print("  triangleCount: " .. triangleCount)
print("};")

print("\n=== IMPORTANT ===")
print("Compare the UVs for vertices at the SAME position.")
print("If they have DIFFERENT UVs, that confirms UV seaming.")
print("Example: Look for vertices with position (16.75, 16.75, X) at different Z values.")
