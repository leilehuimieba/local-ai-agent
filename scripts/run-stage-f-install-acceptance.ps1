$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-f-install"
$logDir = Join-Path $outDir "logs"
$sandboxRoot = Join-Path $outDir "sandbox"
$outFile = Join-Path $outDir "latest.json"
$installScript = Join-Path $PSScriptRoot "install-local-agent.ps1"

function Ensure-Dir {
  param([string]$Path)
  New-Item -ItemType Directory -Force -Path $Path | Out-Null
}

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 40)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Stop-PortProcess {
  param([int]$Port)
  $pids = @(Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique)
  foreach ($procId in $pids) {
    if ($procId -gt 0) { Stop-Process -Id $procId -Force -ErrorAction SilentlyContinue }
  }
}

function Read-InstallReport {
  param([string]$JsonText)
  return $JsonText | ConvertFrom-Json
}

function Invoke-Install {
  param([string]$Mode, [string]$Version, [string]$InstallRootPath, [int]$GatewayPort, [int]$RuntimePort)
  $json = powershell -ExecutionPolicy Bypass -File $installScript -Mode $Mode -Version $Version -InstallRoot $InstallRootPath -GatewayPort $GatewayPort -RuntimePort $RuntimePort
  return Read-InstallReport -JsonText $json
}

function Start-AndCheckSystem {
  param([string]$CurrentRoot, [int]$GatewayPort, [int]$RuntimePort)
  $env:LOCAL_AGENT_REPO_ROOT = $CurrentRoot
  $env:LOCAL_AGENT_NO_BROWSER = "1"
  & (Join-Path $CurrentRoot "gateway\launcher.exe") | Out-Null
  $gatewayReady = Wait-HttpReady -Url ("http://127.0.0.1:" + $GatewayPort + "/health")
  $runtimeReady = Wait-HttpReady -Url ("http://127.0.0.1:" + $RuntimePort + "/health")
  $sys = $null
  if ($gatewayReady) { $sys = Invoke-RestMethod -Uri ("http://127.0.0.1:" + $GatewayPort + "/api/v1/system/info") -Method Get }
  return [ordered]@{
    gateway_ready = $gatewayReady
    runtime_ready = $runtimeReady
    system_info_ok = ($null -ne $sys -and [string]$sys.repo_root -eq $CurrentRoot)
    formal_entry = $(if ($null -ne $sys) { [string]$sys.formal_entry } else { "" })
  }
}

Ensure-Dir $outDir
Ensure-Dir $logDir
Remove-Item -Recurse -Force $sandboxRoot -ErrorAction SilentlyContinue
Ensure-Dir $sandboxRoot
$installRoot = Join-Path $sandboxRoot ("local-agent-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())

$gatewayPort = New-FreePort
$runtimePort = New-FreePort
while ($runtimePort -eq $gatewayPort) { $runtimePort = New-FreePort }
Stop-PortProcess -Port $gatewayPort
Stop-PortProcess -Port $runtimePort

$version1 = "f01-install-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$installReport = Invoke-Install -Mode "install" -Version $version1 -InstallRootPath $installRoot -GatewayPort $gatewayPort -RuntimePort $runtimePort
$installCheck = Start-AndCheckSystem -CurrentRoot $installReport.current_dir -GatewayPort $gatewayPort -RuntimePort $runtimePort
Stop-PortProcess -Port $gatewayPort
Stop-PortProcess -Port $runtimePort

$version2 = "f01-upgrade-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$upgradeReport = Invoke-Install -Mode "upgrade" -Version $version2 -InstallRootPath $installRoot -GatewayPort $gatewayPort -RuntimePort $runtimePort
$upgradeCheck = Start-AndCheckSystem -CurrentRoot $upgradeReport.current_dir -GatewayPort $gatewayPort -RuntimePort $runtimePort
Stop-PortProcess -Port $gatewayPort
Stop-PortProcess -Port $runtimePort

$artifactOk = (Test-Path $installReport.artifacts.launcher) -and
  (Test-Path $installReport.artifacts.server) -and
  (Test-Path $installReport.artifacts.runtime) -and
  (Test-Path $installReport.artifacts.frontend_index) -and
  (Test-Path $installReport.artifacts.config) -and
  (Test-Path $installReport.artifacts.start_script) -and
  (Test-Path $installReport.artifacts.readme)
$upgradeBackupOk = -not [string]::IsNullOrWhiteSpace([string]$upgradeReport.backup_dir) -and (Test-Path $upgradeReport.backup_dir)
$versionFile = Join-Path $installRoot "current-version.txt"
$versionMatched = (Test-Path $versionFile) -and ((Get-Content -Raw $versionFile).Trim() -eq $version2)
$installBootOk = $installCheck.gateway_ready -and $installCheck.runtime_ready -and $installCheck.system_info_ok
$upgradeBootOk = $upgradeCheck.gateway_ready -and $upgradeCheck.runtime_ready -and $upgradeCheck.system_info_ok
$passed = $artifactOk -and $installBootOk -and $upgradeBackupOk -and $upgradeBootOk -and $versionMatched

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($passed) { "passed" } else { "failed" })
  ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
  install = [ordered]@{
    version = $version1
    install_root = $installRoot
    artifact_ok = $artifactOk
    boot = $installCheck
  }
  upgrade = [ordered]@{
    version = $version2
    backup_ok = $upgradeBackupOk
    version_file_matched = $versionMatched
    boot = $upgradeCheck
  }
  artifacts = [ordered]@{
    report = $outFile
    install_report = $installReport
    upgrade_report = $upgradeReport
    current_version_file = $versionFile
  }
}

$report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
if (-not $passed) { throw "stage-f install acceptance failed: $outFile" }
