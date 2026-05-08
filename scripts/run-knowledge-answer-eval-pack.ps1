param(
  [string]$RepoRoot = "",
  [string]$FixturePath = "",
  [string]$OutputDir = "",
  [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"

$root = if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
} else {
  (Resolve-Path $RepoRoot).Path
}
$config = Get-Content -Raw -Encoding UTF8 (Join-Path $root "config\app.json") | ConvertFrom-Json
$fixtureFile = if ([string]::IsNullOrWhiteSpace($FixturePath)) {
  Join-Path $root "docs\11-hermes-rebuild\changes\AP-knowledge-answer-eval-pack-20260506\fixtures\knowledge-answer-cases.json"
} else {
  (Resolve-Path $FixturePath).Path
}
$outDir = if ([string]::IsNullOrWhiteSpace($OutputDir)) {
  Join-Path $root "tmp\knowledge-answer-evals"
} else {
  $OutputDir
}
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-knowledge-answer-eval.exe"
$resultFile = Join-Path $outDir "latest.json"
$runId = "knowledge-answer-eval-" + (Get-Date -Format "yyyyMMdd-HHmmss")
$historyFile = Join-Path $outDir "$runId.json"
$runtimeOut = Join-Path $logDir "runtime.stdout.log"
$runtimeErr = Join-Path $logDir "runtime.stderr.log"
$gatewayOut = Join-Path $logDir "gateway.stdout.log"
$gatewayErr = Join-Path $logDir "gateway.stderr.log"
$runtimeBuildOut = Join-Path $logDir "runtime-build.stdout.log"
$runtimeBuildErr = Join-Path $logDir "runtime-build.stderr.log"
$gatewayBuildOut = Join-Path $logDir "gateway-build.stdout.log"
$gatewayBuildErr = Join-Path $logDir "gateway-build.stderr.log"
$gatewayTokenFile = Join-Path $root "data\.gateway_token"

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
  param([string]$FilePath, [string[]]$Arguments, [string]$WorkDir, [string]$OutPath, [string]$ErrPath)
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
  if ($proc.ExitCode -ne 0) { throw "$FilePath failed with exit code: $($proc.ExitCode)" }
}

function Start-LoggedServer {
  param([string]$FilePath, [string]$WorkDir, [string]$OutPath, [string]$ErrPath, [hashtable]$Env)
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkDir
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($key in $Env.Keys) { $psi.Environment[$key] = [string]$Env[$key] }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  return [pscustomobject]@{ Process = $proc; StdoutTask = $proc.StandardOutput.ReadToEndAsync(); StderrTask = $proc.StandardError.ReadToEndAsync(); OutPath = $OutPath; ErrPath = $ErrPath }
}

function Stop-LoggedServer {
  param($Server)
  if ($null -eq $Server) { return }
  if ($Server.Process -and -not $Server.Process.HasExited) {
    Stop-Process -Id $Server.Process.Id -Force
    $Server.Process.WaitForExit()
  }
  Set-Content -Path $Server.OutPath -Value $Server.StdoutTask.Result -Encoding UTF8
  Set-Content -Path $Server.ErrPath -Value $Server.StderrTask.Result -Encoding UTF8
}

function Read-CaseFile {
  param([string]$Path)
  return @(Get-Content -Raw -Encoding UTF8 $Path | ConvertFrom-Json)
}

function Invoke-CaseRun {
  param($Case, [string]$BaseUrl, $Config, [hashtable]$Headers)
  $sessionId = "knowledge-answer-" + $Case.id + "-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $body = @{
    session_id = $sessionId
    user_input = [string]$Case.question
    mode = $Config.default_mode
    model = $Config.default_model
    workspace = $Config.default_workspace
  } | ConvertTo-Json -Depth 8
  $accepted = Invoke-RestMethod -Uri "$BaseUrl/api/v1/chat/run" -Method Post -Headers $Headers -ContentType "application/json; charset=utf-8" -Body $body
  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $logsUrl = "$BaseUrl/api/v1/logs?session_id=$([Uri]::EscapeDataString($accepted.session_id))&run_id=$([Uri]::EscapeDataString($accepted.run_id))&limit=300"
  $items = @()
  $terminal = $null
  $planReady = $null
  for ($i = 0; $i -lt 120; $i++) {
    $payload = Invoke-RestMethod -Uri $logsUrl -Method Get -Headers $Headers
    $items = @($payload.items | Where-Object { $_.run_id -eq $accepted.run_id -and [int64]$_.timestamp -ge $startedAt })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { break }
    Start-Sleep -Milliseconds 500
  }
  $planReady = @($items | Where-Object { $_.event_type -eq "plan_ready" } | Select-Object -Last 1)
  return [pscustomobject]@{ Accepted = $accepted; Items = $items; Terminal = $(if ($terminal.Count -gt 0) { $terminal[0] } else { $null }); PlanReady = $(if ($planReady.Count -gt 0) { $planReady[0] } else { $null }) }
}

function Get-CaseSignals {
  param($Run)
  $terminal = $Run.Terminal
  $metadata = if ($terminal -and $terminal.metadata) { $terminal.metadata } else { @{} }
  $verification = if ($terminal -and $terminal.verification_snapshot) { $terminal.verification_snapshot } else { $null }
  $combined = @([string]$terminal.final_answer, [string]$terminal.result_summary, [string]$terminal.verification_summary, [string]$verification.summary) -join " || "
  $citations = @([string]$metadata.knowledge_pack_citations, [string]$metadata.verification_has_citation) -join " || "
  $knowledgeDigest = ""
  if ($Run.PlanReady) { $knowledgeDigest = [string]$Run.PlanReady.metadata.knowledge_digest }
  return [pscustomobject]@{
    terminal = $terminal
    metadata = $metadata
    combined = $combined
    has_citation = $citations.Contains("true") -or $citations.Contains("docs/") -or $citations.Contains(".md")
    has_boundary = $combined.Contains("事实") -and ($combined.Contains("推断") -or $combined.Contains("建议"))
    meta_visible = ([string]$metadata.verification_task_type -eq "knowledge_answer") -and ([string]$metadata.capability_risk_checked -ne "") -and ([string]$metadata.permission_boundary_respected -ne "")
    replan_hit = @($Run.Items | Where-Object { $_.event_type -eq "replan_requested" }).Count -gt 0
    handoff_hit = $combined.Contains("handoff") -or [string]$terminal.failure_route -like "*handoff*" -or [string]$terminal.next_action_hint -like "*handoff*"
    low_evidence_hit = $combined.Contains("证据不足") -or $combined.Contains("无法确认") -or $combined.Contains("需要补充")
    knowledge_digest = $knowledgeDigest
  }
}

function Test-CaseOutcome {
  param($Case, $Run)
  $signals = Get-CaseSignals $Run
  $expected = [string]$Case.expected_outcome
  $eventType = [string]$signals.terminal.event_type
  $passed = ($eventType -eq "run_finished") -and $signals.has_citation -and $signals.has_boundary -and $signals.meta_visible
  if ($expected -eq "replan_or_handoff") {
    $passed = $signals.replan_hit -or $signals.handoff_hit -or $signals.low_evidence_hit -or ($eventType -eq "run_failed")
  }
  $result = [ordered]@{}
  $result.case_id = [string]$Case.id
  $result.kind = [string]$Case.kind
  $result.expected_outcome = $expected
  $result.passed = $passed
  $result.terminal_event_type = $eventType
  $result.verification_task_type = [string]$signals.metadata.verification_task_type
  $result.verification_has_citation = [string]$signals.metadata.verification_has_citation
  $result.capability_risk_checked = [string]$signals.metadata.capability_risk_checked
  $result.permission_boundary_respected = [string]$signals.metadata.permission_boundary_respected
  $result.has_boundary = $signals.has_boundary
  $result.replan_requested = $signals.replan_hit
  $result.handoff_visible = $signals.handoff_hit
  $result.low_evidence_visible = $signals.low_evidence_hit
  $result.final_answer = [string]$signals.terminal.final_answer
  $result.result_summary = [string]$signals.terminal.result_summary
  $result.verification_summary = [string]$signals.terminal.verification_summary
  $result.knowledge_digest = $signals.knowledge_digest
  $result.knowledge_pack_citations = [string]$signals.metadata.knowledge_pack_citations
  return [pscustomobject]$result
}

New-Item -ItemType Directory -Force -Path $outDir, $logDir, $binDir, $cargoTargetDir | Out-Null
Set-Content -Path $runtimeOut -Value "" -Encoding UTF8
Set-Content -Path $runtimeErr -Value "" -Encoding UTF8
Set-Content -Path $gatewayOut -Value "" -Encoding UTF8
Set-Content -Path $gatewayErr -Value "" -Encoding UTF8
$runtimePort = New-FreePort
$gatewayPort = New-FreePort
while ($gatewayPort -eq $runtimePort) { $gatewayPort = New-FreePort }
$runtimeServer = $null
$gatewayServer = $null
$runError = $null
$caseResults = @()
$cases = Read-CaseFile $fixtureFile
$headers = @{}

try {
  if (-not $SkipBuild) {
    Invoke-LoggedProcess "cargo" @("build", "-p", "runtime-host", "--target-dir", $cargoTargetDir) $root $runtimeBuildOut $runtimeBuildErr
    Invoke-LoggedProcess "go" @("build", "-o", $gatewayExe, "./cmd/server") (Join-Path $root "gateway") $gatewayBuildOut $gatewayBuildErr
  }
  if (-not (Test-Path $runtimeExe)) { throw "runtime-host binary missing: $runtimeExe" }
  if (-not (Test-Path $gatewayExe)) { throw "gateway binary missing: $gatewayExe" }
  $runtimeServer = Start-LoggedServer $runtimeExe $root $runtimeOut $runtimeErr @{ LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady ("http://127.0.0.1:{0}/health" -f $runtimePort))) { throw "runtime not ready" }
  $gatewayServer = Start-LoggedServer $gatewayExe $root $gatewayOut $gatewayErr @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady ("http://127.0.0.1:{0}/health" -f $gatewayPort))) { throw "gateway not ready" }
  if (-not (Test-Path $gatewayTokenFile)) { throw "gateway token missing: $gatewayTokenFile" }
  $headers["X-Local-Agent-Token"] = [string](Get-Content -Raw -Encoding UTF8 $gatewayTokenFile).Trim()
  $baseUrl = "http://127.0.0.1:{0}" -f $gatewayPort
  foreach ($case in $cases) {
    $run = Invoke-CaseRun $case $baseUrl $config $headers
    $caseResults += Test-CaseOutcome $case $run
  }
} catch {
  $runError = $_.Exception.Message
} finally {
  Stop-LoggedServer $gatewayServer
  Stop-LoggedServer $runtimeServer
}

$passedCases = @($caseResults | Where-Object { $_.passed }).Count
$failedCases = @($caseResults | Where-Object { -not $_.passed })
$status = if ([string]::IsNullOrEmpty($runError) -and $failedCases.Count -eq 0 -and $caseResults.Count -gt 0) { "passed" } else { "failed" }
$result = [ordered]@{
  run_id = $runId
  checked_at = (Get-Date).ToString("o")
  status = $status
  fixture_path = $fixtureFile
  output_dir = $outDir
  runtime_port = $runtimePort
  gateway_port = $gatewayPort
  run_error = $runError
  checks = [ordered]@{
    total_cases = @($caseResults).Count
    passed_cases = $passedCases
    failed_cases = $failedCases.Count
    all_cases_passed = ($failedCases.Count -eq 0 -and $caseResults.Count -gt 0)
  }
  case_results = $caseResults
  failed_case_ids = @($failedCases | ForEach-Object { $_.case_id })
  artifact_paths = [ordered]@{
    result = $resultFile
    history = $historyFile
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

$json = $result | ConvertTo-Json -Depth 10
Set-Content -Path $resultFile -Value $json -Encoding UTF8
Set-Content -Path $historyFile -Value $json -Encoding UTF8
if ($status -ne "passed") { throw "knowledge answer eval pack failed: $resultFile" }
Write-Output $resultFile
