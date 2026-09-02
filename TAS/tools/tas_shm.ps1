# Minimal shared-memory poke/peek for Local\SupremeTAS (test/dev only).
# -Cmd N writes the command field (offset 4): 1=ARM_REC 2=ARM_PLAY 3=STOP 5=RESTART
# Always prints version/command/mode/game_in_game.
param([int]$Cmd = -1)
$mmf = [System.IO.MemoryMappedFiles.MemoryMappedFile]::OpenExisting("Local\SupremeTAS")
$acc = $mmf.CreateViewAccessor(0, 1663520)
if ($Cmd -ge 0) { $acc.Write(4, [uint32]$Cmd) }
$ver = $acc.ReadUInt32(0)
$cmd2 = $acc.ReadUInt32(4)
$mode = $acc.ReadUInt32(36)
$cw = $acc.ReadUInt32(1663504); $rid = $acc.ReadUInt32(1663508)
"version=$ver command=$cmd2 mode=$mode renderer_id=$rid fpu_cw=0x{0:X4}" -f $cw
$acc.Dispose(); $mmf.Dispose()
