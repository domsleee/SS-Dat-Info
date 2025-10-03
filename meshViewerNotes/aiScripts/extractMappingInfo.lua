-- Extract MappingInfo parameters from planarMap function call
-- This reveals the ACTUAL UV parameters the game uses for each mesh
--
-- SETUP:
-- 1. Find sr.dll base address (View -> Enumerate DLLs)
-- 2. Set breakpoint at sr.dll+0x43E70 (planarMap function)
-- 3. Load a mesh in the game (Editor_Cross or board)
-- 4. When breakpoint hits, run this script
--
-- The script will extract the MappingInfo structure passed to planarMap

print("=== MAPPINGINFO EXTRACTOR ===")
print("")

-- Get registers at breakpoint
local regs = getRegisters()

-- planarMap signature: void __thiscall planarMap(srModeler *this, long param_1, long param_2, MappingInfo *param_3)
-- This pointer is in ECX
-- param_3 (MappingInfo*) should be on the stack or in a register

-- Check the stack for MappingInfo pointer
-- In __thiscall, parameters are pushed right-to-left
-- param_3 is at [ESP+0x0C] (after return address + 2 params)
-- Or might be in EAX/EDX depending on calling convention

local esp = regs.ESP
print("ESP = " .. string.format("0x%X", esp))
print("ECX = " .. string.format("0x%X", regs.ECX))
print("")

-- Try reading param_3 from stack
local param3Ptr = readInteger(esp + 0x0C)
print("Attempting to read MappingInfo from stack [ESP+0x0C]:")
print("  param_3 pointer = " .. string.format("0x%X", param3Ptr))
print("")

-- Verify pointer looks valid (should be in game memory range)
if param3Ptr < 0x10000 or param3Ptr > 0x7FFFFFFF then
  print("⚠️  Pointer looks invalid! Trying alternative locations...")
  print("")
  
  -- Try ESP+0x08
  param3Ptr = readInteger(esp + 0x08)
  print("Trying [ESP+0x08]: " .. string.format("0x%X", param3Ptr))
  
  if param3Ptr < 0x10000 or param3Ptr > 0x7FFFFFFF then
    -- Try ESP+0x10
    param3Ptr = readInteger(esp + 0x10)
    print("Trying [ESP+0x10]: " .. string.format("0x%X", param3Ptr))
  end
  print("")
end

-- MappingInfo structure (from sr.c line 61200):
-- offset +0x00: axis1 (int/enum e_axis)
-- offset +0x04: axis2 (int/enum e_axis)
-- offset +0x08: scale_u (float)
-- offset +0x0C: scale_v (float)
-- offset +0x10: offset_u (float)
-- offset +0x14: offset_v (float)

print("=== MAPPINGINFO STRUCTURE ===")
print("Reading from address: " .. string.format("0x%X", param3Ptr))
print("")

local success, axis1 = pcall(readInteger, param3Ptr + 0x00)
if not success then
  print("❌ Failed to read MappingInfo - pointer invalid!")
  print("   Try setting breakpoint and running again")
  return
end

local axis2 = readInteger(param3Ptr + 0x04)
local scale_u = readFloat(param3Ptr + 0x08)
local scale_v = readFloat(param3Ptr + 0x0C)
local offset_u = readFloat(param3Ptr + 0x10)
local offset_v = readFloat(param3Ptr + 0x14)

-- e_axis enum values (0=X, 1=Y, 2=Z)
local axisNames = { [0]="X", [1]="Y", [2]="Z" }

print("✅ MappingInfo extracted successfully!")
print("")
print("  axis1     = " .. axis1 .. " (" .. (axisNames[axis1] or "UNKNOWN") .. ")")
print("  axis2     = " .. axis2 .. " (" .. (axisNames[axis2] or "UNKNOWN") .. ")")
print("  scale_u   = " .. scale_u)
print("  scale_v   = " .. scale_v)
print("  offset_u  = " .. offset_u)
print("  offset_v  = " .. offset_v)
print("")

-- Show projection mode
local projectionMode = "UNKNOWN"
if axis1 == 0 and axis2 == 1 then
  projectionMode = "XY (X for U, Y for V)"
elseif axis1 == 0 and axis2 == 2 then
  projectionMode = "XZ (X for U, Z for V)"
elseif axis1 == 1 and axis2 == 2 then
  projectionMode = "YZ (Y for U, Z for V)"
end

print("📐 Projection Mode: " .. projectionMode)
print("")

-- Interpret tiling
local tilingU = math.floor(scale_u + 0.5)
local tilingV = math.floor(scale_v + 0.5)
print("🎨 Texture Tiling: " .. tilingU .. "x" .. tilingV)
print("")

-- Compare with known values
print("=== COMPARISON WITH KNOWN MESHES ===")
print("")
print("Editor_Cross (expected):")
print("  axis1=0 (X), axis2=1 (Y)")
print("  scale_u=3.0, scale_v=3.0")
print("  offset_u=0.0, offset_v=-3.0")
print("  → 3x3 tiling, XY projection")
print("")
print("Board_lod0 (expected):")
print("  axis1=0 (X), axis2=1 (Y)")
print("  scale_u=1.0, scale_v=1.0")
print("  offset_u=0.0, offset_v=-1.0")
print("  → 1x1 tiling, XY projection")
print("")

-- Determine which mesh this matches
local matchType = "UNKNOWN"
if math.abs(scale_u - 3.0) < 0.01 and math.abs(scale_v - 3.0) < 0.01 then
  matchType = "✅ Matches Editor_Cross pattern (3x3 tiling)"
elseif math.abs(scale_u - 1.0) < 0.01 and math.abs(scale_v - 1.0) < 0.01 then
  matchType = "✅ Matches Board pattern (1x1 tiling)"
else
  matchType = "⚠️  NEW pattern! Document this!"
end

print("Match: " .. matchType)
print("")

-- Generate TypeScript config
print("=== TYPESCRIPT CONFIG ===")
print("")
print("Add to MESH_MAPPING_INFO:")
print("{")
print("  'meshName': {")
print("    axis1: " .. axis1 .. ",  // " .. (axisNames[axis1] or "UNKNOWN"))
print("    axis2: " .. axis2 .. ",  // " .. (axisNames[axis2] or "UNKNOWN"))
print("    scale_u: " .. scale_u .. ",")
print("    scale_v: " .. scale_v .. ",")
print("    offset_u: " .. offset_u .. ",")
print("    offset_v: " .. offset_v)
print("  }")
print("}")
print("")

print("=== INSTRUCTIONS ===")
print("1. Note which mesh triggered this (Editor_Cross, board, etc.)")
print("2. Copy the TypeScript config above")
print("3. Run script again for different meshes")
print("4. Build complete mapping table")
print("5. Update runMeshViewer.ts with correct parameters")
print("")
print("=== DONE ===")
