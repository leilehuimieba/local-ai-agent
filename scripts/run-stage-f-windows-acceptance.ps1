param(
  [int]$MaxMinutes = 10,
  [switch]$RequirePass
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

if ($MaxMinutes -le 0) {
  throw "MaxMinutes 必须大于 0"
}

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-f-windows"
$outFile = Join-Path $outDir "latest.json"
$summaryFile = Join-Path $outDir "latest.md"
$sandboxRoot = Join-Path $outDir "sandbox"
$installRoot = Join-Path $sandboxRoot ("local-agent-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())
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
  param([string]$Url, [int]$Attempts = 60)
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
  $procIds = @(Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess -Unique)
  foreach ($procId in $procIds) {
    if ($procId -gt 0) { Stop-Process -Id $procId -Force -ErrorAction SilentlyContinue }
  }
}

function Wait-RunTerminal {
  param([string]$LogsUrl, [string]$RunId, [int64]$Since, [int]$Attempts = 80)
  for ($i = 0; $i -lt $Attempts; $i++) {
    $payload = Invoke-RestMethod -Uri $LogsUrl -Method Get
    $items = @($payload.items | Where-Object { $_.run_id -eq $RunId -and [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { return [ordered]@{ items = $items; terminal = $terminal[0] } }
    Start-Sleep -Milliseconds 500
  }
  throw "run timeout: $RunId"
}

function Invoke-Install {
  param([string]$Version, [string]$TargetRoot, [int]$GatewayPort, [int]$RuntimePort)
  $json = powershell -ExecutionPolicy Bypass -File $installScript -Mode install -Version $Version -InstallRoot $TargetRoot -GatewayPort $GatewayPort -RuntimePort $RuntimePort
  return $json | ConvertFrom-Json
}

Ensure-Dir $outDir
Remove-Item -Recurse -Force $sandboxRoot -ErrorAction SilentlyContinue
Ensure-Dir $sandboxRoot

$startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$gatewayPort = New-FreePort
$runtimePort = New-FreePort
while ($runtimePort -eq $gatewayPort) { $runtimePort = New-FreePort }
Stop-PortProcess -Port $gatewayPort
Stop-PortProcess -Port $runtimePort

$installVersion = "f05-install-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$installReport = Invoke-Install -Version $installVersion -TargetRoot $installRoot -GatewayPort $gatewayPort -RuntimePort $runtimePort
$currentRoot = [string]$installReport.current_dir

$env:LOCAL_AGENT_REPO_ROOT = $currentRoot
$env:LOCAL_AGENT_NO_BROWSER = "1"
& (Join-Path $currentRoot "gateway\launcher.exe") | Out-Null

$gatewayBase = "http://127.0.0.1:" + $gatewayPort
$runtimeBase = "http://127.0.0.1:" + $runtimePort
$gatewayReady = Wait-HttpReady -Url ($gatewayBase + "/health")
$runtimeReady = Wait-HttpReady -Url ($runtimeBase + "/health")

$cfgPath = Join-Path $currentRoot "config\app.json"
$cfg = Get-Content -Raw $cfgPath | ConvertFrom-Json
$runAccepted = $null
$terminal = $null
$taskCompleted = $false

if ($gatewayReady -and $runtimeReady) {
  $sessionId = "stage-f-windows-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $runStartAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $runAccepted = Invoke-RestMethod -Uri ($gatewayBase + "/api/v1/chat/run") -Method Post -ContentType "application/json; charset=utf-8" -Body (@{
      session_id = $sessionId
      user_input = "cmd: Get-Date"
      mode = $cfg.default_mode
      model = $cfg.default_model
      workspace = $cfg.default_workspace
    } | ConvertTo-Json -Depth 10)
  $logsUrl = $gatewayBase + "/api/v1/logs?session_id=" + [Uri]::EscapeDataString([string]$runAccepted.session_id) + "&run_id=" + [Uri]::EscapeDataString([string]$runAccepted.run_id) + "&limit=200"
  $runResult = Wait-RunTerminal -LogsUrl $logsUrl -RunId $runAccepted.run_id -Since $runStartAt
  $terminal = $runResult.terminal
  $taskCompleted = [string]$terminal.event_type -eq "run_finished" -and [string]$terminal.completion_status -eq "completed"
}

$finishedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$elapsedMs = $finishedAt - $startedAt
$thresholdMs = $MaxMinutes * 60 * 1000
$withinTenMinutes = $elapsedMs -le $thresholdMs
$passed = $gatewayReady -and $runtimeReady -and $taskCompleted -and $withinTenMinutes

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($passed) { "passed" } else { "failed" })
  max_minutes = $MaxMinutes
  elapsed_ms = $elapsedMs
  threshold_ms = $thresholdMs
  ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
  install = [ordered]@{
    version = $installVersion
    current_dir = $currentRoot
  }
  checks = [ordered]@{
    gateway_ready = $gatewayReady
    runtime_ready = $runtimeReady
    first_task_completed = $taskCompleted
    within_time_budget = $withinTenMinutes
  }
  first_task = [ordered]@{
    run_accepted = $runAccepted
    terminal = $terminal
  }
  artifacts = [ordered]@{
    report = $outFile
    summary = $summaryFile
  }
}

$report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8

$summary = @(
  "# F-05 Windows 10-minute verification",
  "",
  "- checked_at: $($report.checked_at)",
  "- status: $($report.status)",
  "- elapsed_ms: $elapsedMs",
  "- threshold_ms: $thresholdMs",
  "- gateway_ready: $gatewayReady",
  "- runtime_ready: $runtimeReady",
  "- first_task_completed: $taskCompleted",
  "- within_time_budget: $withinTenMinutes",
  "- evidence_json: $outFile"
)
$summary | Set-Content -Path $summaryFile -Encoding UTF8

Stop-PortProcess -Port $gatewayPort
Stop-PortProcess -Port $runtimePort

Write-Output $outFile
if ($RequirePass -and -not $passed) {
  throw "stage-f windows acceptance failed: $outFile"
}
