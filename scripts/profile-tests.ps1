param(
    [string]$Command,
    [string]$LogPath = "artifacts/test-mem-log.csv",
    [int]$IntervalMs = 500
)

$ErrorActionPreference = "Stop"

$workspace = Split-Path -Parent $PSScriptRoot
Set-Location $workspace

if (-not $Command) {
    $cargoExe = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
    if (-not (Test-Path $cargoExe)) {
        $cargoExe = "cargo"
    }
    $Command = "& `"$cargoExe`" test -p agent -- --test-threads=1"
}

$logFullPath = Join-Path $workspace $LogPath
$logDir = Split-Path $logFullPath
if (-not (Test-Path $logDir)) {
    New-Item -ItemType Directory -Force -Path $logDir | Out-Null
}
"timestamp,process_name,pid,working_set_mb,virtual_mb" | Out-File -FilePath $logFullPath -Encoding utf8

$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = "powershell.exe"
$psi.Arguments = "-NoProfile -Command $Command"
$psi.UseShellExecute = $false
$psi.RedirectStandardOutput = $false
$psi.RedirectStandardError = $false

$process = [System.Diagnostics.Process]::Start($psi)

while (-not $process.HasExited) {
    foreach ($proc in Get-Process cargo,rustc,link -ErrorAction SilentlyContinue) {
        $line = "{0},{1},{2},{3},{4}" -f (
            (Get-Date).ToString("o"),
            $proc.ProcessName,
            $proc.Id,
            [math]::Round($proc.WorkingSet64 / 1MB, 2),
            [math]::Round($proc.VirtualMemorySize64 / 1MB, 2)
        )
        Add-Content -Path $logFullPath -Value $line
    }
    Start-Sleep -Milliseconds $IntervalMs
}

exit $process.ExitCode
