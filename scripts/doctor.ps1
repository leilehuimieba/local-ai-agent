param(
  [string]$RepoRoot = "",
  [int]$GatewayPort = 0,
  [int]$RuntimePort = 0,
  [string]$OutFile = "",
  [switch]$RequirePass
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  $RepoRoot = Split-Path -Parent $PSScriptRoot
}
$RepoRoot = [System.IO.Path]::GetFullPath($RepoRoot)
$configPath = Join-Path $RepoRoot "config\app.json"
$configExists = Test-Path $configPath
$config = $null
if ($configExists) {
  $config = Get-Content -Raw $configPath | ConvertFrom-Json
}
if ($GatewayPort -le 0 -and $null -ne $config) { $GatewayPort = [int]$config.gateway_port }
if ($RuntimePort -le 0 -and $null -ne $config) { $RuntimePort = [int]$config.runtime_port }

function Get-CommandVersion {
  param([string]$Name, [string[]]$Arguments)
  $prev = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  $output = & $Name @Arguments 2>&1
  $code = $LASTEXITCODE
  $ErrorActionPreference = $prev
  if ($code -ne 0) { return [ordered]@{ ok = $false; output = "" } }
  return [ordered]@{ ok = $true; output = [string]($output | Select-Object -First 1) }
}

function Test-HttpOK {
  param([string]$Url)
  try {
    $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 2 -UseBasicParsing
    return $resp.StatusCode -eq 200
  } catch {
    return $false
  }
}

function Test-LogsWritable {
  param([string]$Root)
  $logsDir = Join-Path $Root "logs"
  try {
    New-Item -ItemType Directory -Force -Path $logsDir | Out-Null
    $probe = Join-Path $logsDir ("doctor-write-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds() + ".tmp")
    Set-Content -Path $probe -Value "ok" -Encoding UTF8
    Remove-Item -Force $probe
    return $true
  } catch {
    return $false
  }
}

$go = Get-CommandVersion -Name "go" -Arguments @("version")
$cargo = Get-CommandVersion -Name "cargo" -Arguments @("--version")
$node = Get-CommandVersion -Name "node" -Arguments @("--version")
$npm = Get-CommandVersion -Name "npm" -Arguments @("--version")
$frontendIndex = Join-Path $RepoRoot "frontend\dist\index.html"
$portsValid = $GatewayPort -gt 0 -and $GatewayPort -le 65535 -and $RuntimePort -gt 0 -and $RuntimePort -le 65535
$gatewayOK = $portsValid -and (Test-HttpOK -Url ("http://127.0.0.1:" + $GatewayPort + "/health"))
$runtimeOK = $portsValid -and (Test-HttpOK -Url ("http://127.0.0.1:" + $RuntimePort + "/health"))
$logsWritable = Test-LogsWritable -Root $RepoRoot

$checks = [ordered]@{
  go_available = [bool]$go.ok
  rust_available = [bool]$cargo.ok
  node_available = [bool]$node.ok
  npm_available = [bool]$npm.ok
  config_exists = [bool]$configExists
  ports_valid = [bool]$portsValid
  frontend_dist_exists = [bool](Test-Path $frontendIndex)
  runtime_health_ok = [bool]$runtimeOK
  gateway_health_ok = [bool]$gatewayOK
  logs_writable = [bool]$logsWritable
}

$allPassed = @($checks.Values | Where-Object { -not $_ }).Count -eq 0
$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($allPassed) { "passed" } else { "failed" })
  repo_root = $RepoRoot
  ports = [ordered]@{ gateway = $GatewayPort; runtime = $RuntimePort }
  checks = $checks
  versions = [ordered]@{
    go = $go.output
    cargo = $cargo.output
    node = $node.output
    npm = $npm.output
  }
  artifacts = [ordered]@{
    config = $configPath
    frontend_index = $frontendIndex
    logs_dir = (Join-Path $RepoRoot "logs")
  }
}

$json = $report | ConvertTo-Json -Depth 6
if (-not [string]::IsNullOrWhiteSpace($OutFile)) {
  $dir = Split-Path -Parent $OutFile
  if (-not [string]::IsNullOrWhiteSpace($dir)) { New-Item -ItemType Directory -Force -Path $dir | Out-Null }
  Set-Content -Path $OutFile -Value $json -Encoding UTF8
}
Write-Output $json
if ($RequirePass -and -not $allPassed) {
  throw "doctor checks failed"
}
