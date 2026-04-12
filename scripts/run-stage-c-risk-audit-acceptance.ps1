$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$configPath = Join-Path $root "config\app.json"
$config = Get-Content -Path $configPath -Raw | ConvertFrom-Json
$gatewayPort = 0
$runtimePort = 0
$gateway = "http://127.0.0.1:" + $gatewayPort
$runUrl = $gateway + "/api/v1/chat/run"
$confirmUrl = $gateway + "/api/v1/chat/confirm"
$healthUrl = $gateway + "/health"
$logsUrl = $gateway + "/api/v1/logs"
$outDir = Join-Path $root "tmp\stage-c-risk-audit-acceptance"
$outFile = Join-Path $outDir "latest.json"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$sessionId = "stage-c-risk-audit-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$command = "cmd: Remove-Item AGENTS.md -WhatIf"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-c-risk.exe"
$runtimeLog = Join-Path $logDir "runtime.log"
$gatewayLog = Join-Path $logDir "gateway.log"
$runtimeProc = $null
$gatewayProc = $null

function Wait-HttpReady {
  param(
    [string]$Url,
    [int]$Attempts
  )
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $response = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($response.StatusCode -eq 200) {
        return $true
      }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Invoke-JsonPost {
  param(
    [string]$Url,
    $Body
  )
  $json = $Body | ConvertTo-Json -Depth 8
  return Invoke-RestMethod -Uri $Url -Method Post -ContentType 'application/json; charset=utf-8' -Body $json
}

function Get-RunLogs {
  param(
    [string]$Url,
    [string]$RunId
  )
  $logs = Invoke-RestMethod -Uri $Url
  return @($logs.items | Where-Object { $_.run_id -eq $RunId })
}

function Wait-ForEvent {
  param(
    [string]$Url,
    [string]$RunId,
    [int64]$Since,
    [string]$EventType,
    [int]$Attempts = 80
  )
  for ($i = 0; $i -lt $Attempts; $i++) {
    $items = @(Get-RunLogs -Url $Url -RunId $RunId | Where-Object { [int64]$_.timestamp -ge $Since })
    $hit = @($items | Where-Object { $_.event_type -eq $EventType } | Select-Object -Last 1)
    if ($hit.Count -gt 0) {
      return [PSCustomObject]@{
        Items = $items
        Event = $hit[0]
      }
    }
    Start-Sleep -Milliseconds 500
  }
  throw ("event timeout: " + $EventType + " for " + $RunId)
}

function Wait-RunLogs {
  param(
    [string]$Url,
    [string]$RunId,
    [int64]$Since
  )
  for ($i = 0; $i -lt 80; $i++) {
    $items = @(Get-RunLogs -Url $Url -RunId $RunId | Where-Object { [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) {
      return $items
    }
    Start-Sleep -Milliseconds 500
  }
  throw ("run timeout: " + $RunId)
}

function Last-Event {
  param(
    $Items,
    [string]$EventType
  )
  return ,@($Items | Where-Object { $_.event_type -eq $EventType } | Select-Object -Last 1)
}

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Join-ProcessArguments {
  param([string[]]$Arguments)
  if ($null -eq $Arguments -or $Arguments.Count -eq 0) {
    return ""
  }
  $quoted = foreach ($arg in $Arguments) {
    if ($arg -match '[\s"]') {
      '"' + ($arg -replace '"', '\"') + '"'
    } else {
      $arg
    }
  }
  return ($quoted -join " ")
}

function Invoke-LoggedProcess {
  param(
    [string]$FilePath,
    [string[]]$Arguments,
    [string]$WorkingDirectory,
    [string]$StdoutPath,
    [string]$StderrPath,
    [hashtable]$Environment
  )
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.Arguments = Join-ProcessArguments $Arguments
  $psi.WorkingDirectory = $WorkingDirectory
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($key in $Environment.Keys) {
    $psi.Environment[$key] = [string]$Environment[$key]
  }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $stdout = $proc.StandardOutput.ReadToEndAsync()
  $stderr = $proc.StandardError.ReadToEndAsync()
  $proc.WaitForExit()
  Set-Content -Path $StdoutPath -Value $stdout.Result -Encoding UTF8
  Set-Content -Path $StderrPath -Value $stderr.Result -Encoding UTF8
  if ($proc.ExitCode -ne 0) {
    throw "$FilePath 执行失败，退出码: $($proc.ExitCode)"
  }
}

function Build-IsolatedBinaries {
  param(
    [string]$Root,
    [string]$CargoTargetDir,
    [string]$RuntimeExe,
    [string]$GatewayExe,
    [string]$LogDir
  )
  $runtimeBuildOut = Join-Path $LogDir "runtime-build.stdout.log"
  $runtimeBuildErr = Join-Path $LogDir "runtime-build.stderr.log"
  $gatewayBuildOut = Join-Path $LogDir "gateway-build.stdout.log"
  $gatewayBuildErr = Join-Path $LogDir "gateway-build.stderr.log"
  Invoke-LoggedProcess -FilePath "cargo" -Arguments @("build", "-p", "runtime-host", "--target-dir", $CargoTargetDir) -WorkingDirectory $Root -StdoutPath $runtimeBuildOut -StderrPath $runtimeBuildErr -Environment @{}
  if (-not (Test-Path $RuntimeExe)) {
    throw "runtime-host 构建产物缺失：$RuntimeExe"
  }
  Invoke-LoggedProcess -FilePath "go" -Arguments @("build", "-o", $GatewayExe, "./cmd/server") -WorkingDirectory (Join-Path $Root "gateway") -StdoutPath $gatewayBuildOut -StderrPath $gatewayBuildErr -Environment @{}
  if (-not (Test-Path $GatewayExe)) {
    throw "gateway 构建产物缺失：$GatewayExe"
  }
}

function Start-IsolatedProcess {
  param(
    [string]$FilePath,
    [string]$WorkingDirectory,
    [string]$StdoutPath,
    [string]$StderrPath,
    [hashtable]$Environment
  )
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkingDirectory
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($key in $Environment.Keys) {
    $psi.Environment[$key] = [string]$Environment[$key]
  }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $proc.BeginOutputReadLine()
  $proc.BeginErrorReadLine()
  Register-ObjectEvent -InputObject $proc -EventName OutputDataReceived -Action {
    if ($EventArgs.Data) { Add-Content -Path $using:StdoutPath -Value $EventArgs.Data }
  } | Out-Null
  Register-ObjectEvent -InputObject $proc -EventName ErrorDataReceived -Action {
    if ($EventArgs.Data) { Add-Content -Path $using:StderrPath -Value $EventArgs.Data }
  } | Out-Null
  return $proc
}

function Stop-IsolatedProcess {
  param($Process)
  if ($null -eq $Process) {
    return
  }
  if (-not $Process.HasExited) {
    Stop-Process -Id $Process.Id -Force
    $Process.WaitForExit()
  }
}

function Cleanup-ProcessEvents {
  Get-EventSubscriber | Where-Object { $_.SourceObject -is [System.Diagnostics.Process] } | Unregister-Event
}

function Approve-Confirmation {
  param(
    [string]$Url,
    [string]$ConfirmationId,
    [string]$RunId,
    [string]$Note
  )
  return Invoke-JsonPost -Url $Url -Body @{
    confirmation_id = $ConfirmationId
    run_id = $RunId
    decision = "approve"
    note = $Note
    remember = $false
  }
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Remove-Item -Recurse -Force $cargoTargetDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cargoTargetDir | Out-Null
Set-Content -Path $runtimeLog -Value "" -Encoding UTF8
Set-Content -Path $gatewayLog -Value "" -Encoding UTF8

try {
  $runtimePort = New-FreePort
  $gatewayPort = New-FreePort
  while ($gatewayPort -eq $runtimePort) {
    $gatewayPort = New-FreePort
  }
  $gateway = "http://127.0.0.1:" + $gatewayPort
  $runUrl = $gateway + "/api/v1/chat/run"
  $confirmUrl = $gateway + "/api/v1/chat/confirm"
  $healthUrl = $gateway + "/health"
  $logsUrl = $gateway + "/api/v1/logs"

  Build-IsolatedBinaries -Root $root -CargoTargetDir $cargoTargetDir -RuntimeExe $runtimeExe -GatewayExe $gatewayExe -LogDir $logDir

  $runtimeProc = Start-IsolatedProcess -FilePath $runtimeExe -WorkingDirectory $root -StdoutPath $runtimeLog -StderrPath $runtimeLog -Environment @{ LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:" + $runtimePort + "/health") -Attempts 40)) {
    throw "runtime 未在隔离端口就绪"
  }

  $gatewayProc = Start-IsolatedProcess -FilePath $gatewayExe -WorkingDirectory $root -StdoutPath $gatewayLog -StderrPath $gatewayLog -Environment @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url $healthUrl -Attempts 40)) {
    throw "gateway 未在隔离端口就绪"
  }

  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $runAccepted = Invoke-JsonPost -Url $runUrl -Body @{
    session_id = $sessionId
    user_input = $command
    mode = $config.default_mode
    model = $config.default_model
    workspace = $config.default_workspace
  }

  $preApprovals = @()
  $confirmationCursor = $startedAt
  $highRiskConfirmation = $null
  for ($round = 0; $round -lt 3; $round++) {
    $confirmResult = Wait-ForEvent -Url $logsUrl -RunId $runAccepted.run_id -Since $confirmationCursor -EventType "confirmation_required"
    $candidate = $confirmResult.Event
    $kind = [string]$candidate.metadata.kind
    if ($kind -eq "high_risk_action") {
      $highRiskConfirmation = $candidate
      break
    }
    $confirmStart = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
    $approved = Approve-Confirmation -Url $confirmUrl -ConfirmationId $candidate.confirmation_id -RunId $runAccepted.run_id -Note "stage-c risk audit pre-approve"
    $preApprovals += [ordered]@{
      confirmation_id = $candidate.confirmation_id
      kind = $kind
      response = $approved
    }
    $confirmationCursor = $confirmStart
  }

  if ($null -eq $highRiskConfirmation) {
    throw "未命中 high_risk_action 确认事件"
  }

  $approveStartedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $approveResponse = Approve-Confirmation -Url $confirmUrl -ConfirmationId $highRiskConfirmation.confirmation_id -RunId $runAccepted.run_id -Note "stage-c risk audit approval"

  $terminalWindowLogs = Wait-RunLogs -Url $logsUrl -RunId $runAccepted.run_id -Since $approveStartedAt
  $allLogs = @(Get-RunLogs -Url $logsUrl -RunId $runAccepted.run_id | Where-Object { [int64]$_.timestamp -ge $startedAt })
  $terminal = @($terminalWindowLogs | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)

  $resumeCandidates = @($allLogs | Where-Object {
      $_.event_type -eq "checkpoint_resumed" -and
      $_.metadata.checkpoint_resume_reason -eq "confirmation_required" -and
      [string]$_.metadata.confirmation_id -eq [string]$highRiskConfirmation.confirmation_id
    })
  $resumeEvent = @($resumeCandidates | Select-Object -Last 1)

  $resumeFound = $resumeEvent.Count -gt 0
  $resumeDecisionMatched = $resumeFound -and [string]$resumeEvent[0].metadata.confirmation_decision -eq "approve"
  $resumeStrategyMatched = $resumeFound -and [string]$resumeEvent[0].metadata.confirmation_resume_strategy -eq "after_confirmation"
  $resumeStepMatched = $resumeFound -and [string]$resumeEvent[0].metadata.confirmation_chain_step -eq "resumed"
  $resumeSourceMatched = $resumeFound -and [string]$resumeEvent[0].metadata.confirmation_decision_source -eq "user_confirm_api"
  $terminalOk = $terminal.Count -gt 0 -and [string]$terminal[0].event_type -eq "run_finished"

  $passed = $resumeFound -and
    $resumeDecisionMatched -and
    $resumeStrategyMatched -and
    $resumeStepMatched -and
    $resumeSourceMatched -and
    $terminalOk

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    gateway = $gateway
    ports = [ordered]@{
      gateway = $gatewayPort
      runtime = $runtimePort
    }
    session_id = $sessionId
    command = $command
    pre_approvals = $preApprovals
    run = [ordered]@{
      run_id = $runAccepted.run_id
      high_risk_confirmation_id = $highRiskConfirmation.confirmation_id
      high_risk_kind = [string]$highRiskConfirmation.metadata.kind
      event_types = @($allLogs | Select-Object -ExpandProperty event_type)
      resume_found = $resumeFound
      resume_decision_matched = $resumeDecisionMatched
      resume_strategy_matched = $resumeStrategyMatched
      resume_step_matched = $resumeStepMatched
      resume_source_matched = $resumeSourceMatched
      terminal_ok = $terminalOk
      high_risk_confirmation_event = $highRiskConfirmation
      checkpoint_resumed_event = $(if ($resumeFound) { $resumeEvent[0] } else { $null })
      terminal_event = $(if ($terminal.Count -gt 0) { $terminal[0] } else { $null })
    }
    approve_response = $approveResponse
    artifacts = [ordered]@{
      report = $outFile
      runtime_log = $runtimeLog
      gateway_log = $gatewayLog
      runtime_build_stdout = (Join-Path $logDir "runtime-build.stdout.log")
      runtime_build_stderr = (Join-Path $logDir "runtime-build.stderr.log")
      gateway_build_stdout = (Join-Path $logDir "gateway-build.stdout.log")
      gateway_build_stderr = (Join-Path $logDir "gateway-build.stderr.log")
    }
  }

  $report | ConvertTo-Json -Depth 9 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) {
    throw "stage-c risk audit 验收失败，见 $outFile"
  }
  Write-Output $outFile
} finally {
  Stop-IsolatedProcess -Process $gatewayProc
  Stop-IsolatedProcess -Process $runtimeProc
  Cleanup-ProcessEvents
}
