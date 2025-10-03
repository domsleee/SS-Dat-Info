-- ============================================================================
-- Extract Board Triangle Indices
-- ============================================================================
-- Based on findIndices.lua - this finds the actual index buffer for the board mesh
-- 
-- USAGE:
-- 1. Set breakpoint at getModelByName when it returns
-- 2. Load the board mesh in game
-- 3. When breakpoint hits, run this script
-- 4. Copy the TypeScript output
-- ============================================================================

print("╔════════════════════════════════════════════════════════════════╗")
print("║         EXTRACT BOARD INDICES - Find Index Buffer             ║")
print("╚════════════════════════════════════════════════════════════════╝")
print("")

-- Mesh pointer from getModelByName return value (EAX)
local meshPtr = 0xF270C40  -- UPDATE THIS if needed

print("🎯 srMeshModel pointer: 0x" .. string.format("%X", meshPtr))
print("")

-- Read counts
local vertexCount = readInteger(meshPtr + 0x234)
local triangleCount = readInteger(meshPtr + 0x230)

print("📊 Mesh Info:")
print(string.format("   Vertices: %d", vertexCount))
print(string.format("   Triangles: %d", triangleCount))
print(string.format("   Expected indices: %d", triangleCount * 3))
print("")

-- Scan entire srMeshModel structure (not just TriMesh)
-- We'll search from offset 0 to 0x500 (1280 bytes)
print("🔍 Scanning ENTIRE srMeshModel structure for index buffer...")
print("")

local foundBuffers = {}

-- METHOD 1: Search for pointers in the entire structure
for offset = 0, 0x4FF, 4 do
    local ptr = readInteger(meshPtr + offset)
    if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
        -- Check if this looks like a 32-bit index array
        local allValid = true
        local maxIdx = 0

        -- Check first 30 indices (10 triangles) to be more thorough
        for i = 0, 29 do
            local idx = readInteger(ptr + i * 4)
            if not idx or idx < 0 or idx >= 1000 then
                allValid = false
                break
            end
            if idx > maxIdx then
                maxIdx = idx
            end
        end

        if allValid and maxIdx < vertexCount and maxIdx > 0 then
            table.insert(foundBuffers, {
                offset = offset,
                ptr = ptr,
                type = "uint32"
            })
            print(string.format("   ✅ Found uint32 index buffer at srMeshModel+0x%X", offset))
            print(string.format("      Pointer: 0x%X, Max index: %d", ptr, maxIdx))
        end
    end
end

-- METHOD 2: Also check for 16-bit indices
for offset = 0, 0x4FF, 4 do
    local ptr = readInteger(meshPtr + offset)
    if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
        local allValid = true
        local maxIdx = 0

        -- Check as 16-bit indices
        for i = 0, 29 do
            local idx = readSmallInteger(ptr + i * 2)
            if not idx or idx < 0 or idx >= 1000 then
                allValid = false
                break
            end
            if idx > maxIdx then
                maxIdx = idx
            end
        end

        if allValid and maxIdx < vertexCount and maxIdx > 0 then
            -- Check if we already found this as uint32
            local isDuplicate = false
            for _, buf in ipairs(foundBuffers) do
                if buf.ptr == ptr then
                    isDuplicate = true
                    break
                end
            end

            if not isDuplicate then
                table.insert(foundBuffers, {
                    offset = offset,
                    ptr = ptr,
                    type = "uint16"
                })
                print(string.format("   ✅ Found uint16 index buffer at srMeshModel+0x%X", offset))
                print(string.format("      Pointer: 0x%X, Max index: %d", ptr, maxIdx))
            end
        end
    end
end

print("")

if #foundBuffers == 0 then
    print("❌ ERROR: No index buffer found in entire srMeshModel structure!")
    print("")
    print("   This might mean:")
    print("   1. The mesh uses triangle strips/fans instead of indexed triangles")
    print("   2. The indices are stored inline (not via pointer)")
    print("   3. Different data structure than expected")
    print("")
    print("   Let's dump some pointers to investigate:")
    print("")
    
    -- Dump all valid-looking pointers
    print("   Valid pointers found in srMeshModel:")
    for offset = 0, 0x2FF, 4 do
        local ptr = readInteger(meshPtr + offset)
        if ptr and ptr > 0x1000000 and ptr < 0x20000000 then
            print(string.format("      Offset 0x%03X: 0x%08X", offset, ptr))
        end
    end
    return
end

-- Try extracting from ALL found buffers to see which one is correct
print("🔍 Testing all found buffers to find the correct one...")
print("")

for bufferIdx, testBuffer in ipairs(foundBuffers) do
    print(string.format("═══ Testing Buffer #%d (offset 0x%X, type: %s) ═══", bufferIdx, testBuffer.offset, testBuffer.type))
    
    -- Extract first 30 indices to check
    local testIndices = {}
    local maxIdx = 0
    local allValid = true
    
    for i = 0, math.min(29, triangleCount * 3 - 1) do
        local idx
        if testBuffer.type == "uint16" then
            idx = readSmallInteger(testBuffer.ptr + i * 2)
        else
            idx = readInteger(testBuffer.ptr + i * 4)
        end
        
        if not idx or idx < 0 or idx >= vertexCount then
            allValid = false
            print(string.format("   ❌ Invalid index at position %d: %s", i, tostring(idx)))
            break
        end
        
        if idx > maxIdx then
            maxIdx = idx
        end
        table.insert(testIndices, idx)
    end
    
    if allValid then
        print(string.format("   ✅ Valid! Max index in first 30: %d (vertex count: %d)", maxIdx, vertexCount))
        print(string.format("   First 30 indices: %s", table.concat(testIndices, ", ")))
    end
    print("")
end

-- Let user choose which buffer, or default to first one (it usually has the proper mesh topology)
local bufferChoice = 1  -- Changed to use buffer #1 which shows proper triangle patterns
print(string.format("📦 Using buffer #%d (offset: 0x%X, type: %s)", bufferChoice, foundBuffers[bufferChoice].offset, foundBuffers[bufferChoice].type))
print("")

local indexBuffer = foundBuffers[bufferChoice]

-- Extract all indices
print("╔════════════════════════════════════════════════════════════════╗")
print("║                    TYPESCRIPT OUTPUT                           ║")
print("╚════════════════════════════════════════════════════════════════╝")
print("")
print("-- Copy the indices array below into gameExtracted_board.ts --")
print("")
print("  // Index data (triangle list - ACTUAL GAME INDICES)")
print("  indices: new Uint16Array([")

local indicesPerLine = 30
for i = 0, triangleCount * 3 - 1, indicesPerLine do
    local line = "    "
    for j = i, math.min(i + indicesPerLine - 1, triangleCount * 3 - 1) do
        local idx
        if indexBuffer.type == "uint16" then
            idx = readSmallInteger(indexBuffer.ptr + j * 2)
        else
            idx = readInteger(indexBuffer.ptr + j * 4)
        end
        line = line .. string.format("%d", idx)
        if j < triangleCount * 3 - 1 then
            line = line .. ", "
        end
    end
    print(line)
end

print("  ]),")
print("")
print("════════════════════════════════════════════════════════════════")
print("")
print("✅ Done! Now:")
print("   1. Copy the indices array above")
print("   2. Replace the GENERATED INDICES in gameExtracted_board.ts")
print("   3. The mesh should now render correctly!")
print("")
