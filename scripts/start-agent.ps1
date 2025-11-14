Param(
    [string]$AdminToken = "dev-admin-token"
)

$ErrorActionPreference = "Stop"

Set-Location -Path "$PSScriptRoot\.."

$env:ORBIT_AUTH_ENABLED = "1"
$env:ORBIT_ADMIN_TOKEN = $AdminToken

$cargo = Join-Path $env:USERPROFILE ".cargo\bin\cargo.exe"
if (-not (Test-Path $cargo)) {
    throw "cargo executable not found at $cargo. Please ensure Rust is installed."
}

$logPath = Join-Path (Get-Location) "agent-ui-out.log"

Write-Host "Starting agent with auth enabled. Logging to $logPath"

& $cargo run -p agent 2>&1 | ForEach-Object {
    $_
    $_ | Out-File -FilePath $logPath -Encoding utf8 -Append
}
