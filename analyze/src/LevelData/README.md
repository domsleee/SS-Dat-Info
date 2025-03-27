
# How to collect .raw.ts files

1) Run the script in cheat engine
2) Open the track in the game

```lua
print("export const rawPoints = [")
local breakpoint = "Supreme_Game.dll+1a59b"
function debugger_onBreakpoint()
  if EIP == getAddress(breakpoint) then
    local name = readString(readPointer(EBX+0x1c))
    if string.match(name, "^xxx_Point_") or string.match(name, "^Player_Start_Location_") then
       print(string.format('  ["%s", [%f,%f,%f]],', readString(readPointer(EBX+0x1c)), readFloat(EBX+0x34), readFloat(EBX+0x38), readFloat(EBX+0x3c)))
    end
    debug_continueFromBreakpoint(co_run)
    return 1
  end

  return 0
end

debug_removeBreakpoint(breakpoint)
debug_setBreakpoint(breakpoint)
```
