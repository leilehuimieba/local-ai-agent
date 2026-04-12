param(
  [int]$Rounds = 5,
  [switch]$RequireGateC
)

$ErrorActionPreference = "Stop"

if ($Rounds -le 0) {
  throw "Rounds 必须大于 0"
}

$root = Split-Path -Parent $PSScriptRoot
$riskScript = Join-Path $PSScriptRoot "run-stage-c-risk-audit-acceptance.ps1"
$retryScript = Join-Path $PSScriptRoot "run-stage-b-retry-acceptance.ps1"
$elapsedScript = Join-Path $PSScriptRoot "run-stage-c-tool-elapsed-acceptance.ps1"
$riskLatest = Join-Path $root "tmp\stage-c-risk-audit-acceptance\latest.json"
$retryLatest = Join-Path $root "tmp\stage-b-retry-acceptance\latest.json"
$elapsedLatest = Join-Path $root "tmp\stage-c-tool-elapsed-acceptance\latest.json"
$outDir = Join-Path $root "tmp\stage-c-gate-c-batch"
$outFile = Join-Path $outDir "latest.json"

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$roundsDetail = @()

for ($i = 1; $i -le $Rounds; $i++) {
  powershell -ExecutionPolicy Bypass -File $riskScript | Out-Null
  $risk = Get-Content -Path $riskLatest -Raw | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $retryScript | Out-Null
  $retry = Get-Content -Path $retryLatest -Raw | ConvertFrom-Json

  $riskPass = $risk.status -eq "passed" -and
    $risk.run.high_risk_kind -eq "high_risk_action" -and
    $risk.run.resume_found -and
    $risk.run.resume_decision_matched -and
    $risk.run.resume_strategy_matched -and
    $risk.run.resume_step_matched -and
    $risk.run.resume_source_matched -and
    $risk.run.terminal_ok

  $failureLocatePass = $retry.status -eq "passed" -and
    $retry.retry_run.reason_matched -and
    $retry.retry_run.stage_matched -and
    $retry.retry_run.event_type_matched -and
    $retry.retry_run.boundary_recovered -and
    $retry.retry_run.verification_recovered -and
    $retry.retry_run.artifact_recovered -and
    $retry.retry_run.target_resumed_unique -and
    $retry.retry_run.checkpoint_id_matched

  $resumeMeta = $risk.run.checkpoint_resumed_event.metadata
  $auditPass = -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.confirmation_id) -and
    -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.confirmation_decision) -and
    -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.confirmation_chain_step) -and
    -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.confirmation_resume_strategy) -and
    -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.confirmation_decision_source) -and
    -not [string]::IsNullOrWhiteSpace([string]$resumeMeta.checkpoint_id)

  $roundsDetail += [ordered]@{
    round = $i
    risk_checked_at = $risk.checked_at
    retry_checked_at = $retry.checked_at
    risk_run_id = $risk.run.run_id
    retry_run_id = $retry.retry_run.run_id
    high_risk_intercepted = $riskPass
    failure_locatable = $failureLocatePass
    audit_fields_complete = $auditPass
    round_passed = ($riskPass -and $failureLocatePass -and $auditPass)
  }
}

powershell -ExecutionPolicy Bypass -File $elapsedScript | Out-Null
$elapsed = Get-Content -Path $elapsedLatest -Raw | ConvertFrom-Json
$elapsedPass = $elapsed.status -eq "passed" -and
  $elapsed.run.verification_elapsed_ok -and
  $elapsed.run.run_finished_elapsed_ok

$interceptCount = @($roundsDetail | Where-Object { $_.high_risk_intercepted }).Count
$locateCount = @($roundsDetail | Where-Object { $_.failure_locatable }).Count
$auditCount = @($roundsDetail | Where-Object { $_.audit_fields_complete }).Count
$roundPassCount = @($roundsDetail | Where-Object { $_.round_passed }).Count

$summary = [ordered]@{
  intercept_count = $interceptCount
  locate_count = $locateCount
  audit_count = $auditCount
  round_pass_count = $roundPassCount
  intercept_rate = [Math]::Round($interceptCount / $Rounds, 4)
  locate_rate = [Math]::Round($locateCount / $Rounds, 4)
  audit_rate = [Math]::Round($auditCount / $Rounds, 4)
  round_pass_rate = [Math]::Round($roundPassCount / $Rounds, 4)
  tool_elapsed_passed = $elapsedPass
}

$requiredRounds = 5
$requiredInterceptRate = 0.99
$requiredLocateRate = 0.95
$requiredAuditRate = 1.0
$roundsOk = $Rounds -ge $requiredRounds
$interceptOk = $summary.intercept_rate -ge $requiredInterceptRate
$locateOk = $summary.locate_rate -ge $requiredLocateRate
$auditOk = $summary.audit_rate -ge $requiredAuditRate
$gateReady = $roundsOk -and $interceptOk -and $locateOk -and $auditOk -and $elapsedPass

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  rounds = $Rounds
  status = $(if (($roundPassCount -eq $Rounds) -and $elapsedPass) { "passed" } else { "failed" })
  summary = $summary
  gate_c = [ordered]@{
    required_rounds = $requiredRounds
    required_intercept_rate = $requiredInterceptRate
    required_locate_rate = $requiredLocateRate
    required_audit_rate = $requiredAuditRate
    rounds_ok = $roundsOk
    intercept_ok = $interceptOk
    locate_ok = $locateOk
    audit_ok = $auditOk
    tool_elapsed_ok = $elapsedPass
    ready = $gateReady
  }
  rounds_detail = $roundsDetail
  tool_elapsed_sample = [ordered]@{
    run_id = $elapsed.run.run_id
    verification_elapsed_ok = $elapsed.run.verification_elapsed_ok
    run_finished_elapsed_ok = $elapsed.run.run_finished_elapsed_ok
  }
  artifacts = [ordered]@{
    report = $outFile
    risk_latest = $riskLatest
    retry_latest = $retryLatest
    elapsed_latest = $elapsedLatest
  }
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
if ($report.status -ne "passed") {
  throw "Gate-C 批量验收存在失败轮次，见 $outFile"
}
if ($RequireGateC -and -not $gateReady) {
  throw "Gate-C 判定未达标，见 $outFile"
}
