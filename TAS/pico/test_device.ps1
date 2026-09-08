$ErrorActionPreference='Stop'
. "$PSScriptRoot/device.ps1"
$a = [pscustomobject]@{ serial='AAA'; drive='Q:'; data_port='COM42'; console_port='COM43' }
$b = [pscustomobject]@{ serial='BBB'; drive='R:'; data_port='COM44'; console_port='COM45' }
function Must-Fail([scriptblock]$Action) {
    $failed=$false
    try { & $Action | Out-Null } catch { $failed=$true }
    if (!$failed) { throw 'Expected discovery rejection' }
}
Must-Fail { Select-PicoDevice @() }
Must-Fail { Select-PicoDevice @($a,$b) }
Must-Fail { Select-PicoDevice @($a,$b) -Serial 'UNKNOWN' }
Must-Fail { Select-PicoDevice @($a,$b) -Serial 'AAA' -Drive 'R:' }
if ((Select-PicoDevice @($a,$b) -Serial 'BBB').data_port -ne 'COM44') { throw 'Wrong serial selected' }
if ((Select-PicoDevice @($a,$b) -Drive 'Q:\').serial -ne 'AAA') { throw 'Wrong drive selected' }
$incomplete = [pscustomobject]@{ serial='AAA'; drive='Q:'; data_port=''; console_port='COM43' }
Must-Fail { Select-PicoDevice @($incomplete) }
# Mock the OS parent chain, including an unrelated same-label storage device.
function Get-PnpDeviceProperty($InstanceId, $KeyName, $ErrorAction) {
    $parents = @{ 'disk'='storage'; 'storage'='USB\VID_2E8A&PID_000B\AAA'; 'console'='USB\VID_2E8A&PID_000B\BBB'; 'other'='root' }
    [pscustomobject]@{ Data=$parents[$InstanceId] }
}
if ((Get-PicoParent 'disk') -ne 'USB\VID_2E8A&PID_000B\AAA') { throw 'Disk ancestry not resolved' }
if ((Get-PicoParent 'console') -eq (Get-PicoParent 'disk')) { throw 'Cross-board interfaces matched' }
if (Get-PicoParent 'other') { throw 'Unrelated storage accepted' }
Write-Host 'PASS: 10 Pico identity selection/ancestry checks (no hardware access)'
