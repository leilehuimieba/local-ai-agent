param(
  [ValidateSet("install", "upgrade")]
  [string]$Mode = "install",
  [string]$InstallRoot = "$env:LOCALAPPDATA\LocalAgent",
  [string]$Version = "",
  [int]$GatewayPort = 0,
  [int]$RuntimePort = 0
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

if ([string]::IsNullOrWhiteSpace($Version)) {
  $Version = "v" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$releaseRoot = Join-Path $InstallRoot "releases"
$backupRoot = Join-Path $InstallRoot "backups"
$currentDir = Join-Path $InstallRoot "current"
$releaseDir = Join-Path $releaseRoot $Version
$buildDir = Join-Path $InstallRoot ".build-cache"
$logDir = Join-Path $InstallRoot "logs"

function Ensure-Dir {
  param([string]$Path)
  New-Item -ItemType Directory -Force -Path $Path | Out-Null
}

function Write-Utf8NoBom {
  param([string]$Path, [string]$Content)
  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Invoke-LoggedCommand {
  param([string]$WorkDir, [string]$OutPath, [string]$ErrPath, [string]$File, [string[]]$Arguments)
  $prev = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  Push-Location $WorkDir
  try {
    & $File @Arguments 1> $OutPath 2> $ErrPath
  } finally {
    Pop-Location
  }
  $ErrorActionPreference = $prev
  if ($LASTEXITCODE -ne 0) { throw "$File failed: $($Arguments -join ' ')" }
}

function Assert-CommandAvailable {
  param([string]$Name)
  if (-not (Get-Command -Name $Name -ErrorAction SilentlyContinue)) {
    throw "missing required command: $Name"
  }
}

function Prepare-FrontendDist {
  param([string]$Root, [string]$LogRoot)
  $distIndex = Join-Path $Root "frontend\dist\index.html"
  if (Test-Path $distIndex) { return }
  Assert-CommandAvailable -Name "npm"
  $frontend = Join-Path $Root "frontend"
  Invoke-LoggedCommand -WorkDir $frontend -OutPath (Join-Path $LogRoot "frontend-install.stdout.log") -ErrPath (Join-Path $LogRoot "frontend-install.stderr.log") -File "npm" -Arguments @("install")
  Invoke-LoggedCommand -WorkDir $frontend -OutPath (Join-Path $LogRoot "frontend-build.stdout.log") -ErrPath (Join-Path $LogRoot "frontend-build.stderr.log") -File "npm" -Arguments @("run", "build")
}

function Build-ReleaseBinaries {
  param([string]$Root, [string]$BuildRoot, [string]$StageRoot, [string]$LogRoot)
  Assert-CommandAvailable -Name "cargo"
  Assert-CommandAvailable -Name "go"
  $runtimeTarget = Join-Path $BuildRoot "cargo-target"
  $runtimeExe = Join-Path $runtimeTarget "release\runtime-host.exe"
  Invoke-LoggedCommand -WorkDir $Root -OutPath (Join-Path $LogRoot "runtime-build.stdout.log") -ErrPath (Join-Path $LogRoot "runtime-build.stderr.log") -File "cargo" -Arguments @("build", "--release", "-p", "runtime-host", "--target-dir", $runtimeTarget)
  Invoke-LoggedCommand -WorkDir (Join-Path $Root "gateway") -OutPath (Join-Path $LogRoot "gateway-server-build.stdout.log") -ErrPath (Join-Path $LogRoot "gateway-server-build.stderr.log") -File "go" -Arguments @("build", "-o", (Join-Path $StageRoot "gateway\server.exe"), "./cmd/server")
  Invoke-LoggedCommand -WorkDir (Join-Path $Root "gateway") -OutPath (Join-Path $LogRoot "gateway-launcher-build.stdout.log") -ErrPath (Join-Path $LogRoot "gateway-launcher-build.stderr.log") -File "go" -Arguments @("build", "-o", (Join-Path $StageRoot "gateway\launcher.exe"), "./cmd/launcher")
  if (-not (Test-Path $runtimeExe)) { throw "runtime-host.exe missing" }
  Copy-Item -LiteralPath $runtimeExe -Destination (Join-Path $StageRoot "target\release\runtime-host.exe") -Force
}

function Write-StartScript {
  param([string]$StageRoot)
  $content = @(
    '$ErrorActionPreference = "Stop"',
    '$root = Split-Path -Parent $MyInvocation.MyCommand.Path',
    '$env:LOCAL_AGENT_REPO_ROOT = $root',
    '$env:LOCAL_AGENT_NO_BROWSER = "1"',
    '& (Join-Path $root "gateway\launcher.exe")'
  )
  Set-Content -Path (Join-Path $StageRoot "start-agent.ps1") -Value $content -Encoding UTF8
}

function Write-RunReadme {
  param([string]$StageRoot, [int]$GatewayPort)
  $lines = @(
    '# Run Guide',
    '',
    '1. Run `start-agent.ps1` from PowerShell.',
    "2. Visit `http://127.0.0.1:$GatewayPort` after startup.",
    '3. If upgrade fails, switch to the previous folder under `backups`.'
  )
  Set-Content -Path (Join-Path $StageRoot "README-run.md") -Value $lines -Encoding UTF8
}

function Update-AppConfig {
  param([string]$ConfigPath, [string]$Root, [int]$Gateway, [int]$Runtime)
  $cfg = Get-Content -Raw $ConfigPath | ConvertFrom-Json
  if ($Gateway -gt 0) { $cfg.gateway_port = $Gateway }
  if ($Runtime -gt 0) { $cfg.runtime_port = $Runtime }
  $workspaceRoot = Join-Path $Root "workspace"
  $cfg.default_workspace.root_path = $workspaceRoot
  if ($cfg.workspaces.Count -gt 0) { $cfg.workspaces[0].root_path = $workspaceRoot }
  Write-Utf8NoBom -Path $ConfigPath -Content ($cfg | ConvertTo-Json -Depth 10)
}

Ensure-Dir $InstallRoot
Ensure-Dir $releaseRoot
Ensure-Dir $backupRoot
Ensure-Dir $buildDir
Ensure-Dir $logDir

if ($Mode -eq "install" -and (Test-Path $currentDir)) {
  throw "current already exists, please use upgrade mode"
}
if ($Mode -eq "upgrade" -and -not (Test-Path $currentDir)) {
  throw "current not found, please run install mode first"
}
if (Test-Path $releaseDir) { throw "release version exists: $Version" }

Ensure-Dir $releaseDir
Ensure-Dir (Join-Path $releaseDir "gateway")
Ensure-Dir (Join-Path $releaseDir "target\debug")
Ensure-Dir (Join-Path $releaseDir "config")
Ensure-Dir (Join-Path $releaseDir "frontend\dist")
Ensure-Dir (Join-Path $releaseDir "workspace")

Prepare-FrontendDist -Root $repoRoot -LogRoot $logDir
Build-ReleaseBinaries -Root $repoRoot -BuildRoot $buildDir -StageRoot $releaseDir -LogRoot $logDir
Copy-Item -Recurse -Force -Path (Join-Path $repoRoot "frontend\dist\*") -Destination (Join-Path $releaseDir "frontend\dist")
Copy-Item -Force -Path (Join-Path $repoRoot "config\app.json") -Destination (Join-Path $releaseDir "config\app.json")
Copy-Item -Force -Path (Join-Path $repoRoot "gateway\go.mod") -Destination (Join-Path $releaseDir "gateway\go.mod")
Write-StartScript -StageRoot $releaseDir
Write-RunReadme -StageRoot $releaseDir -GatewayPort $GatewayPort
Update-AppConfig -ConfigPath (Join-Path $releaseDir "config\app.json") -Root $releaseDir -Gateway $GatewayPort -Runtime $RuntimePort

$backupDir = ""
if (Test-Path $currentDir) {
  $backupDir = Join-Path $backupRoot ("backup-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())
  Copy-Item -Recurse -Force -Path $currentDir -Destination $backupDir
  Remove-Item -Recurse -Force -Path $currentDir
}
Copy-Item -Recurse -Force -Path $releaseDir -Destination $currentDir
Set-Content -Path (Join-Path $InstallRoot "current-version.txt") -Value $Version -Encoding UTF8

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  mode = $Mode
  version = $Version
  install_root = $InstallRoot
  release_dir = $releaseDir
  current_dir = $currentDir
  backup_dir = $backupDir
  artifacts = [ordered]@{
    launcher = (Join-Path $currentDir "gateway\launcher.exe")
    server = (Join-Path $currentDir "gateway\server.exe")
    runtime = (Join-Path $currentDir "target\release\runtime-host.exe")
    frontend_index = (Join-Path $currentDir "frontend\dist\index.html")
    config = (Join-Path $currentDir "config\app.json")
    start_script = (Join-Path $currentDir "start-agent.ps1")
    readme = (Join-Path $currentDir "README-run.md")
  }
}

$report | ConvertTo-Json -Depth 6
