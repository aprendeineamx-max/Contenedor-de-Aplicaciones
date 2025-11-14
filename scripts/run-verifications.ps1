param(
    [string]$MemLogPrefix = 'artifacts/mem-log-ci'
)

$workspace = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $workspace

Write-Host ':: Profiling scoped_tokens_limit_permissions'
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$memLog = "$MemLogPrefix-$timestamp.csv"
$profile = Join-Path $workspace 'scripts/profile-tests.ps1'
$command = "& `"$env:USERPROFILE\.cargo\bin\cargo.exe`" test -p agent scoped_tokens_limit_permissions -- --test-threads=1 --nocapture"
powershell -NoProfile -ExecutionPolicy Bypass -File $profile -Command $command -LogPath $memLog
if ($LASTEXITCODE -ne 0) { throw 'Perfil falló' }

Write-Host ':: npm run smoke'
if (-not $env:ORBIT_BASE_URL) { $env:ORBIT_BASE_URL = 'http://127.0.0.1:7443' }
if (-not $env:ORBIT_ADMIN_TOKEN) { $env:ORBIT_ADMIN_TOKEN = 'dev-token' }
npm run smoke
if ($LASTEXITCODE -ne 0) { throw 'Smoke falló' }

Write-Host ':: npm run panel-flow'
npm run panel-flow
if ($LASTEXITCODE -ne 0) { throw 'Panel flow falló' }

Write-Host "Verificaciones completadas. Log de memoria: $memLog"
