$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$config = Get-Content -Raw -Encoding UTF8 (Join-Path $root "config\app.json") | ConvertFrom-Json
$outDir = Join-Path $root "tmp\project-status-answer-regression"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-project-status-answer-regression.exe"
$resultFile = Join-Path $outDir "latest.json"
$runtimeOut = Join-Path $logDir "runtime.stdout.log"
$runtimeErr = Join-Path $logDir "runtime.stderr.log"
$gatewayOut = Join-Path $logDir "gateway.stdout.log"
$gatewayErr = Join-Path $logDir "gateway.stderr.log"
$runtimeBuildOut = Join-Path $logDir "runtime-build.stdout.log"
$runtimeBuildErr = Join-Path $logDir "runtime-build.stderr.log"
$gatewayBuildOut = Join-Path $logDir "gateway-build.stdout.log"
$gatewayBuildErr = Join-Path $logDir "gateway-build.stderr.log"
$question = "我现在接手这个项目，请直接告诉我：当前停在什么状态、为什么不能继续默认推进、以及以后满足什么条件才值得重启。"

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 50)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 400
  }
  return $false
}

function Invoke-LoggedProcess {
  param(
    [string]$FilePath,
    [string[]]$Arguments,
    [string]$WorkDir,
    [string]$OutPath,
    [string]$ErrPath
  )
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkDir
  $psi.Arguments = [string]::Join(" ", $Arguments)
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $stdout = $proc.StandardOutput.ReadToEndAsync()
  $stderr = $proc.StandardError.ReadToEndAsync()
  $proc.WaitForExit()
  Set-Content -Path $OutPath -Value $stdout.Result -Encoding UTF8
  Set-Content -Path $ErrPath -Value $stderr.Result -Encoding UTF8
  if ($proc.ExitCode -ne 0) {
    throw "$FilePath failed with exit code: $($proc.ExitCode)"
  }
}

function Start-LoggedServer {
  param(
    [string]$FilePath,
    [string]$WorkDir,
    [string]$OutPath,
    [string]$ErrPath,
    [hashtable]$Env
  )
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkDir
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($key in $Env.Keys) {
    $psi.Environment[$key] = [string]$Env[$key]
  }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  return [pscustomobject]@{
    Process = $proc
    StdoutTask = $proc.StandardOutput.ReadToEndAsync()
    StderrTask = $proc.StandardError.ReadToEndAsync()
    OutPath = $OutPath
    ErrPath = $ErrPath
  }
}

function Stop-LoggedServer {
  param($Server)
  if ($null -eq $Server) { return }
  $proc = $Server.Process
  if ($proc -and -not $proc.HasExited) {
    Stop-Process -Id $proc.Id -Force
    $proc.WaitForExit()
  }
  Set-Content -Path $Server.OutPath -Value $Server.StdoutTask.Result -Encoding UTF8
  Set-Content -Path $Server.ErrPath -Value $Server.StderrTask.Result -Encoding UTF8
}

New-Item -ItemType Directory -Force -Path $outDir, $logDir, $binDir, $cargoTargetDir | Out-Null
Set-Content -Path $runtimeOut -Value "" -Encoding UTF8
Set-Content -Path $runtimeErr -Value "" -Encoding UTF8
Set-Content -Path $gatewayOut -Value "" -Encoding UTF8
Set-Content -Path $gatewayErr -Value "" -Encoding UTF8

$runtimePort = New-FreePort
$gatewayPort = New-FreePort
while ($gatewayPort -eq $runtimePort) {
  $gatewayPort = New-FreePort
}

$runtimeServer = $null
$gatewayServer = $null
$accepted = $null
$items = @()
$terminal = $null
$planReady = $null
$runError = $null

try {
  Invoke-LoggedProcess "cargo" @("build", "-p", "runtime-host", "--target-dir", $cargoTargetDir) $root $runtimeBuildOut $runtimeBuildErr
  if (-not (Test-Path $runtimeExe)) { throw "runtime-host binary missing: $runtimeExe" }
  Invoke-LoggedProcess "go" @("build", "-o", $gatewayExe, "./cmd/server") (Join-Path $root "gateway") $gatewayBuildOut $gatewayBuildErr
  if (-not (Test-Path $gatewayExe)) { throw "gateway binary missing: $gatewayExe" }

  $runtimeServer = Start-LoggedServer $runtimeExe $root $runtimeOut $runtimeErr @{ LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady ("http://127.0.0.1:{0}/health" -f $runtimePort))) { throw "runtime not ready" }

  $gatewayServer = Start-LoggedServer $gatewayExe $root $gatewayOut $gatewayErr @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady ("http://127.0.0.1:{0}/health" -f $gatewayPort))) { throw "gateway not ready" }

  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $accepted = Invoke-RestMethod -Uri ("http://127.0.0.1:{0}/api/v1/chat/run" -f $gatewayPort) -Method Post -ContentType "application/json; charset=utf-8" -Body (@{
    session_id = "project-status-answer-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
    user_input = $question
    mode = $config.default_mode
    model = $config.default_model
    workspace = $config.default_workspace
  } | ConvertTo-Json -Depth 8)

  $sessionIdEsc = [Uri]::EscapeDataString([string]$accepted.session_id)
  $runIdEsc = [Uri]::EscapeDataString([string]$accepted.run_id)
  $logsUrl = "http://127.0.0.1:{0}/api/v1/logs?session_id={1}&run_id={2}&limit=200" -f $gatewayPort, $sessionIdEsc, $runIdEsc
  for ($i = 0; $i -lt 120; $i++) {
    $payload = Invoke-RestMethod -Uri $logsUrl -Method Get
    $items = @($payload.items | Where-Object { $_.run_id -eq $accepted.run_id -and [int64]$_.timestamp -ge $startedAt })
    $terminalHit = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminalHit.Count -gt 0) {
      $terminal = $terminalHit[0]
      break
    }
    Start-Sleep -Milliseconds 500
  }
  if ($null -eq $terminal) { throw "run timeout" }
  $planReadyHit = @($items | Where-Object { $_.event_type -eq "plan_ready" } | Select-Object -Last 1)
  if ($planReadyHit.Count -gt 0) { $planReady = $planReadyHit[0] }
} catch {
  $runError = $_.Exception.Message
} finally {
  Stop-LoggedServer $gatewayServer
  Stop-LoggedServer $runtimeServer
}

$finalAnswer = if ($terminal) { [string]$terminal.final_answer } else { "" }
$resultSummary = if ($terminal) { [string]$terminal.result_summary } else { "" }
$verificationSummary = if ($terminal -and $terminal.verification_snapshot) { [string]$terminal.verification_snapshot.summary } else { "" }
$knowledgeDigest = if ($planReady) { [string]$planReady.metadata.knowledge_digest } else { "" }
$checks = [ordered]@{
  runtime_ready = [bool](Test-Path $runtimeExe)
  gateway_ready = [bool](Test-Path $gatewayExe)
  run_finished = [bool]$terminal
  answer_hits_stage_h = $finalAnswer.Contains("stage H") -or $finalAnswer.Contains("阶段 H")
  answer_hits_gate_h = $finalAnswer.Contains("Gate-H")
  answer_hits_not_signoff = $finalAnswer.Contains("未签收")
  knowledge_hits_current_state = $knowledgeDigest.Contains("current-state.md")
  summary_hits_current_state = $resultSummary.Contains("current-state.md")
  verification_hits_current_state = $verificationSummary.Contains("current-state.md")
}
$failedChecks = @($checks.GetEnumerator() | Where-Object { -not $_.Value })
$passed = ($failedChecks.Count -eq 0) -and [string]::IsNullOrEmpty($runError)

$result = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  question = $question
  status = $(if ($passed) { "passed" } else { "failed" })
  output_dir = $outDir
  runtime_port = $runtimePort
  gateway_port = $gatewayPort
  run_response = $accepted
  run_error = $runError
  checks = $checks
  final_answer = $finalAnswer
  result_summary = $resultSummary
  verification_summary = $verificationSummary
  plan_knowledge_digest = $knowledgeDigest
  terminal_event_type = $(if ($terminal) { [string]$terminal.event_type } else { "" })
  log_count = @($items).Count
  artifact_paths = [ordered]@{
    result = $resultFile
    runtime_stdout = $runtimeOut
    runtime_stderr = $runtimeErr
    gateway_stdout = $gatewayOut
    gateway_stderr = $gatewayErr
    runtime_build_stdout = $runtimeBuildOut
    runtime_build_stderr = $runtimeBuildErr
    gateway_build_stdout = $gatewayBuildOut
    gateway_build_stderr = $gatewayBuildErr
  }
}

$result | ConvertTo-Json -Depth 10 | Set-Content -Path $resultFile -Encoding UTF8
if (-not $passed) { throw "project status answer regression failed: $resultFile" }
Write-Output $resultFile
