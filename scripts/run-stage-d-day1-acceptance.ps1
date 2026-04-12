$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-d-day1"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTarget = Join-Path $outDir "cargo-target"
$sandbox = Join-Path $outDir "sandbox"
$outFile = Join-Path $outDir "latest.json"

$runtimeOut = Join-Path $logDir "runtime.stdout.log"
$runtimeErr = Join-Path $logDir "runtime.stderr.log"
$gatewayOut = Join-Path $logDir "gateway.stdout.log"
$gatewayErr = Join-Path $logDir "gateway.stderr.log"
$runtimeBuildOut = Join-Path $logDir "runtime-build.stdout.log"
$runtimeBuildErr = Join-Path $logDir "runtime-build.stderr.log"
$gatewayBuildOut = Join-Path $logDir "gateway-build.stdout.log"
$gatewayBuildErr = Join-Path $logDir "gateway-build.stderr.log"

$workspaceId = "day1_ws"
$token = "D06_DAY1_TOKEN_20260412"
$memorySummary = "D06 Day1 continuity preference"
$memoryContent = "Keep docs writeback and evidence sync in future rounds. token=$token"

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 80)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Wait-RunTerminal {
  param([string]$LogsUrl, [string]$RunId, [int64]$Since)
  for ($i = 0; $i -lt 120; $i++) {
    $all = Invoke-RestMethod -Uri $LogsUrl
    $items = @($all.items | Where-Object { $_.run_id -eq $RunId -and [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { return $items }
    Start-Sleep -Milliseconds 500
  }
  throw ("run timeout: " + $RunId)
}

function Write-Utf8NoBom {
  param([string]$Path, [string]$Content)
  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Invoke-Run {
  param(
    [string]$RunUrl,
    [string]$LogsUrl,
    [string]$SessionId,
    [string]$UserInputText,
    $ModelRef,
    $WorkspaceRef
  )
  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $payload = @{
    session_id = $SessionId
    user_input = $UserInputText
    mode = "standard"
    model = $ModelRef
    workspace = $WorkspaceRef
  } | ConvertTo-Json -Depth 10
  $accepted = Invoke-RestMethod -Uri $RunUrl -Method Post -ContentType "application/json; charset=utf-8" -Body $payload
  $logs = Wait-RunTerminal -LogsUrl $LogsUrl -RunId $accepted.run_id -Since $startedAt
  $terminal = @($logs | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
  return [ordered]@{ accepted = $accepted; logs = $logs; terminal = $(if ($terminal.Count -gt 0) { $terminal[0] } else { $null }) }
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Remove-Item -Recurse -Force $cargoTarget -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cargoTarget | Out-Null
Remove-Item -Recurse -Force $sandbox -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "config") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "data\storage") | Out-Null
Set-Content -Path $runtimeOut -Value "" -Encoding UTF8
Set-Content -Path $runtimeErr -Value "" -Encoding UTF8
Set-Content -Path $gatewayOut -Value "" -Encoding UTF8
Set-Content -Path $gatewayErr -Value "" -Encoding UTF8

$runtimePort = New-FreePort
$gatewayPort = New-FreePort
while ($gatewayPort -eq $runtimePort) { $gatewayPort = New-FreePort }

$config = Get-Content -Path (Join-Path $root "config\app.json") -Raw | ConvertFrom-Json
$config.gateway_port = $gatewayPort
$config.runtime_port = $runtimePort
$workspace = [PSCustomObject]@{
  workspace_id = $workspaceId
  name = "D06-Day1-Acceptance"
  root_path = $root
  is_active = $true
}
$config.default_workspace = $workspace
$config.workspaces = @($workspace)
Write-Utf8NoBom -Path (Join-Path $sandbox "config\app.json") -Content ($config | ConvertTo-Json -Depth 10)
$modelRef = [ordered]@{
  provider_id = [string]$config.default_model.provider_id
  model_id = [string]$config.default_model.model_id
  display_name = [string]$config.default_model.display_name
}

$runtimeExe = Join-Path $cargoTarget "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-d.exe"
$runtimeProc = $null
$gatewayProc = $null

try {
  $cargoBuild = Start-Process -FilePath "cargo" -ArgumentList @("build", "-p", "runtime-host", "--target-dir", $cargoTarget) -WorkingDirectory $root -Wait -PassThru -RedirectStandardOutput $runtimeBuildOut -RedirectStandardError $runtimeBuildErr
  if ($cargoBuild.ExitCode -ne 0) { throw "cargo build failed: $($cargoBuild.ExitCode)" }
  $goBuild = Start-Process -FilePath "go" -ArgumentList @("build", "-o", $gatewayExe, "./cmd/server") -WorkingDirectory (Join-Path $root "gateway") -Wait -PassThru -RedirectStandardOutput $gatewayBuildOut -RedirectStandardError $gatewayBuildErr
  if ($goBuild.ExitCode -ne 0) { throw "go build failed: $($goBuild.ExitCode)" }

  $env:LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort
  $runtimeProc = Start-Process -FilePath $runtimeExe -WorkingDirectory $root -PassThru -RedirectStandardOutput $runtimeOut -RedirectStandardError $runtimeErr
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:$runtimePort/health") -Attempts 120)) { throw "runtime not ready" }

  $env:LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort
  $env:LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort
  $gatewayProc = Start-Process -FilePath $gatewayExe -WorkingDirectory $sandbox -PassThru -RedirectStandardOutput $gatewayOut -RedirectStandardError $gatewayErr
  $gateway = "http://127.0.0.1:$gatewayPort"
  if (-not (Wait-HttpReady -Url ($gateway + "/health") -Attempts 120)) { throw "gateway not ready" }

  $runUrl = $gateway + "/api/v1/chat/run"
  $logsUrl = $gateway + "/api/v1/logs"
  $sessionWrite = "stage-d-day1-write-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $writeInput = "remember: preference|$memorySummary`n$memoryContent"
  $writeRun = Invoke-Run -RunUrl $runUrl -LogsUrl $logsUrl -SessionId $sessionWrite -UserInputText $writeInput -ModelRef $modelRef -WorkspaceRef $workspace
  $writeEventTypes = @($writeRun.logs | Select-Object -ExpandProperty event_type)
  $writeMemoryEvent = @($writeRun.logs | Where-Object { $_.event_type -eq "memory_written" } | Select-Object -Last 1)

  $sessionRecall = "stage-d-day1-recall-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $recallInput = "recall: $token"
  $recallRun = Invoke-Run -RunUrl $runUrl -LogsUrl $logsUrl -SessionId $sessionRecall -UserInputText $recallInput -ModelRef $modelRef -WorkspaceRef $workspace
  $recallEventTypes = @($recallRun.logs | Select-Object -ExpandProperty event_type)
  $memoryRecalled = @($recallRun.logs | Where-Object { $_.event_type -eq "memory_recalled" } | Select-Object -Last 1)
  $memories = Invoke-RestMethod -Uri ($gateway + "/api/v1/memories")
  $tokenHits = @($memories.items | Where-Object { [string]$_.summary -like "*$token*" -or [string]$_.content -like "*$token*" }).Count

  $writeOk = $writeRun.terminal -and $writeRun.terminal.event_type -eq "run_finished" -and $writeMemoryEvent.Count -gt 0
  $recallHit = $memoryRecalled.Count -gt 0 -and ([string]$memoryRecalled[0].detail -like "*$token*" -or $tokenHits -ge 1)
  $recallOk = $recallRun.terminal -and $recallRun.terminal.event_type -eq "run_finished" -and $recallHit
  $crossSession = $sessionWrite -ne $sessionRecall
  $passed = $writeOk -and $recallOk -and $crossSession -and $tokenHits -ge 1

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    gateway = $gateway
    workspace_id = $workspaceId
    token = $token
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    day1_sample = [ordered]@{
      write_session = $sessionWrite
      recall_session = $sessionRecall
      cross_session = $crossSession
      write_run_id = $writeRun.accepted.run_id
      recall_run_id = $recallRun.accepted.run_id
      write_terminal = $(if ($writeRun.terminal) { $writeRun.terminal.event_type } else { "" })
      recall_terminal = $(if ($recallRun.terminal) { $recallRun.terminal.event_type } else { "" })
      write_has_memory_written = ($writeMemoryEvent.Count -gt 0)
      recall_has_memory_recalled = ($memoryRecalled.Count -gt 0)
      recall_hit = $recallHit
      token_hits = $tokenHits
      write_event_types = $writeEventTypes
      recall_event_types = $recallEventTypes
      memory_recalled_event = $(if ($memoryRecalled.Count -gt 0) { $memoryRecalled[0] } else { $null })
    }
    artifacts = [ordered]@{
      report = $outFile
      runtime_stdout = $runtimeOut
      runtime_stderr = $runtimeErr
      gateway_stdout = $gatewayOut
      gateway_stderr = $gatewayErr
      runtime_build_stdout = $runtimeBuildOut
      runtime_build_stderr = $runtimeBuildErr
      gateway_build_stdout = $gatewayBuildOut
      gateway_build_stderr = $gatewayBuildErr
      sandbox_config = (Join-Path $sandbox "config\app.json")
    }
  }

  $report | ConvertTo-Json -Depth 12 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) { throw "stage-d day1 acceptance failed, see $outFile" }
  Write-Output $outFile
}
finally {
  if ($gatewayProc -and -not $gatewayProc.HasExited) {
    Stop-Process -Id $gatewayProc.Id -Force
    $gatewayProc.WaitForExit()
  }
  if ($runtimeProc -and -not $runtimeProc.HasExited) {
    Stop-Process -Id $runtimeProc.Id -Force
    $runtimeProc.WaitForExit()
  }
}
