# Read-only capture of a completed real-game CONT. Never sends commands or keys.
param(
    [Parameter(Mandatory)][string]$Recording,
    [Parameter(Mandatory)][string]$OutputDirectory,
    [ValidateRange(1,65535)][int]$Splice = 4500
)
$ErrorActionPreference = 'Stop'
$original = [IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $Recording))
$count = [BitConverter]::ToUInt32($original, 0)
if ($original.Length -ne 4 + 13 * $count -or $Splice -gt $count) { throw 'Expected a raw v2 history blob with a complete prefix' }
$mapping = [IO.MemoryMappedFiles.MemoryMappedFile]::OpenExisting('Local\SupremeTAS', [IO.MemoryMappedFiles.MemoryMappedFileRights]::Read)
$view = $mapping.CreateViewAccessor(0, 0, [IO.MemoryMappedFiles.MemoryMappedFileAccess]::Read)
try {
    if ($view.ReadUInt32(0) -ne 49) { throw 'Offsets require shared protocol 49' }
    $mode = $view.ReadUInt32(36)
    $played = $view.ReadUInt32(44)
    $generation = $view.ReadUInt32(1647448)
    if ($view.ReadUInt32(316) -ne $Splice) { throw 'Last recorded segment is not the requested CONT splice; capture refused' }
    if ($mode -eq 2 -or $played -lt $Splice) { throw 'Wait until CONT has spliced and playback has stopped advancing' }
    $bytes = [byte[]]::new(1658240)
    [void]$view.ReadArray(0, $bytes, 0, $bytes.Length)
    if ($view.ReadUInt32(36) -ne $mode -or $view.ReadUInt32(44) -ne $played -or
        $view.ReadUInt32(1647448) -ne $generation) { throw 'Playback changed during capture; retry' }
    $liveGate = [BitConverter]::ToUInt32($bytes, 1647460)
    $recGate = [BitConverter]::ToUInt32($bytes, 1658236)
    if ($liveGate -eq 0 -or $recGate -eq 0 -or $played -ne $Splice + $liveGate - $recGate) {
        throw 'Missing gate alignment or playback is not parked at the requested splice; capture immediately after CONT'
    }
    for ($i = 0; $i -lt $Splice; $i++) {
        if ($bytes[632 + $i] -ne $original[4 + $i]) { throw "Loaded input differs from fixture at tick $i" }
    }
    # playback_pos is the DLL's frozen, gate-aligned splice endpoint.
    $play = [byte[]]::new($Splice * 12)
    [Array]::Copy($bytes, 852600 + ($played - $Splice) * 12, $play, 0, $play.Length)
    New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
    Copy-Item -LiteralPath $Recording -Destination (Join-Path $OutputDirectory 'recording.tasrec')
    [IO.File]::WriteAllBytes((Join-Path $OutputDirectory 'play-prefix.bin'), $play)
    @{splice=$Splice; playback_pos=$played; live_gate=$liveGate; rec_gate=$recGate; arm_generation=$generation} |
        ConvertTo-Json | Set-Content -LiteralPath (Join-Path $OutputDirectory 'capture.json')
    "Captured real CONT: mode=$mode played=$played splice=$Splice offset=$($played-$Splice)"
} finally {
    $view.Dispose()
    $mapping.Dispose()
}
