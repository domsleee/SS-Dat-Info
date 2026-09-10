# Shared physical-device discovery for deploy.ps1 and tas_test pico/live preflight.
function Get-PicoParent([string]$InstanceId) {
    for ($depth=0; $depth -lt 8; $depth++) {
        if ($InstanceId -match '^USB\\VID_2E8A&PID_000B\\[A-Za-z0-9]+$') { return $InstanceId }
        if (!$InstanceId) { return $null }
        try { $InstanceId = (Get-PnpDeviceProperty -InstanceId $InstanceId -KeyName DEVPKEY_Device_Parent -ErrorAction Stop).Data }
        catch { return $null }
    }
    return $null
}

function Select-PicoDevice($Candidates, [string]$Serial, [string]$Drive) {
    $matches = @($Candidates | Where-Object {
        (!$Serial -or $_.serial -eq $Serial) -and (!$Drive -or $_.drive.TrimEnd('\') -eq $Drive.TrimEnd('\'))
    })
    if ($matches.Count -ne 1) { throw "Expected exactly one Pico, found $($matches.Count). Specify -Serial or -Drive; never guess a drive or COM port." }
    $board = $matches[0]
    if (!$board.data_port -or !$board.console_port -or !$board.drive) { throw 'Pico storage and both CDC interfaces must resolve to the same USB parent' }
    return $board
}

function Find-PicoDevice([string]$Serial, [string]$Drive) {
    $ports = @(Get-CimInstance Win32_SerialPort | Where-Object PNPDeviceID -match '^USB\\VID_2E8A&PID_000B&MI_(00|02)\\')
    $candidates = @(foreach ($disk in Get-CimInstance Win32_DiskDrive | Where-Object { $_.PNPDeviceID -like 'USBSTOR*' -and $_.Size -gt 0 }) {
        if ((Get-Disk -Number $disk.Index).Size -eq 0) { continue } # Empty secondary LUN.
        $parent = Get-PicoParent $disk.PNPDeviceID
        if (!$parent) { continue }
        $data = @(); $console = @()
        foreach ($port in $ports) {
            if ((Get-PicoParent $port.PNPDeviceID) -ne $parent) { continue }
            if ($port.PNPDeviceID -match '&MI_02\\') { $data += $port.DeviceID } else { $console += $port.DeviceID }
        }
        $letters = @(Get-Partition -DiskNumber $disk.Index | Where-Object DriveLetter | ForEach-Object { "$($_.DriveLetter):" })
        if ($letters.Count -ne 1 -or $data.Count -ne 1 -or $console.Count -ne 1) { throw "Ambiguous interfaces on $parent" }
        [pscustomobject]@{ serial=$parent.Split('\')[-1]; instance_id=$parent; drive=$letters[0]; data_port=$data[0]; console_port=$console[0] }
    })
    Select-PicoDevice $candidates $Serial $Drive
}

function Open-PicoPort([string]$Name) {
    $port = [System.IO.Ports.SerialPort]::new($Name,115200)
    $port.ReadTimeout=1000; $port.WriteTimeout=1000; $port.DtrEnable=$true
    try { $port.Open(); return $port } catch { $port.Dispose(); throw }
}

function Close-PicoPort($Port) {
    # A board reset can invalidate the handle before Windows finishes Close.
    try { $Port.Close() } catch { }
    try { $Port.Dispose() } catch { }
}

function Test-PicoAck($Board) {
    $port = Open-PicoPort $Board.data_port
    try {
        $port.DiscardInBuffer()
        $port.Write([byte[]]@(255),0,1)
        $reply = [byte[]]::new(3); $count=0
        while ($count -lt 3) { $count += $port.Read($reply,$count,3-$count) }
        if ([BitConverter]::ToString($reply) -ne '5A-01-01') { throw "Invalid Pico acknowledgement: $([BitConverter]::ToString($reply))" }
    } finally { Close-PicoPort $port }
}

function Test-PicoFiles($Board, [string]$Source) {
    foreach ($name in 'boot.py','code.py') {
        if ((Get-FileHash -LiteralPath (Join-Path $Source $name)).Hash -ne (Get-FileHash -LiteralPath (Join-Path $Board.drive $name)).Hash) {
            throw "$name on Pico $($Board.serial) differs from the repository"
        }
    }
}
