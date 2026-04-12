param(
  [int]$Rounds = 5,
  [double]$MinEntryPassRate = 0.95,
  [double]$MinConsistencyRate = 0.95,
  [double]$MinFailureClosureRate = 0.95,
  [switch]$RequireGateE
)

$ErrorActionPreference = "Stop"

if ($Rounds -le 0) {
  throw "Rounds 必须大于 0"
}

$root = Split-Path -Parent $PSScriptRoot
$entryScript = Join-Path $PSScriptRoot "run-stage-e-entry1-acceptance.ps1"
$consistencyScript = Join-Path $PSScriptRoot "run-stage-e-consistency-acceptance.ps1"
$failureScript = Join-Path $PSScriptRoot "run-stage-e-entry-failure-acceptance.ps1"
$entryLatest = Join-Path $root "tmp\stage-e-entry1\latest.json"
$consistencyLatest = Join-Path $root "tmp\stage-e-consistency\latest.json"
$failureLatest = Join-Path $root "tmp\stage-e-entry-failure\latest.json"
$outDir = Join-Path $root "tmp\stage-e-batch"
$outFile = Join-Path $outDir "latest.json"

function Read-Json {
  param([string]$Path)
  return Get-Content -Path $Path -Raw | ConvertFrom-Json
}

function Read-BoolCheck {
  param($Checks, [string]$Name)
  if ($null -eq $Checks) { return $false }
  $value = $Checks.$Name
  return [bool]$value
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$roundsDetail = @()

for ($i = 1; $i -le $Rounds; $i++) {
  powershell -ExecutionPolicy Bypass -File $entryScript | Out-Null
  $entry = Read-Json -Path $entryLatest
  powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  $consistency = Read-Json -Path $consistencyLatest
  powershell -ExecutionPolicy Bypass -File $failureScript | Out-Null
  $failure = Read-Json -Path $failureLatest

  $entryPass = $entry.status -eq "passed" -and
    (Read-BoolCheck $entry.checks "protocol_fields_ok") -and
    (Read-BoolCheck $entry.checks "all_run_matched") -and
    (Read-BoolCheck $entry.checks "all_session_matched") -and
    (Read-BoolCheck $entry.checks "terminal_completed")

  $consistencyPass = $consistency.status -eq "passed" -and
    (Read-BoolCheck $consistency.checks "accepted_id_matched") -and
    (Read-BoolCheck $consistency.checks "runtime_result_matched") -and
    (Read-BoolCheck $consistency.checks "all_gateway_run_matched") -and
    (Read-BoolCheck $consistency.checks "all_gateway_session_matched") -and
    (Read-BoolCheck $consistency.checks "terminal_type_matched") -and
    (Read-BoolCheck $consistency.checks "terminal_tool_matched") -and
    (Read-BoolCheck $consistency.checks "completion_status_matched") -and
    (Read-BoolCheck $consistency.checks "gateway_trace_matched")

  $failurePass = $failure.status -eq "passed" -and
    (Read-BoolCheck $failure.checks "all_run_matched") -and
    (Read-BoolCheck $failure.checks "all_session_matched") -and
    (Read-BoolCheck $failure.checks "has_run_failed") -and
    (Read-BoolCheck $failure.checks "has_run_finished") -and
    (Read-BoolCheck $failure.checks "terminal_is_run_finished") -and
    (Read-BoolCheck $failure.checks "error_code_runtime_unavailable")

  $roundsDetail += [ordered]@{
    round = $i
    entry_checked_at = $entry.checked_at
    consistency_checked_at = $consistency.checked_at
    failure_checked_at = $failure.checked_at
    entry_run_id = $entry.run.run_id
    consistency_run_id = $consistency.run_identity.run_id
    failure_run_id = $failure.run.run_id
    entry_passed = $entryPass
    consistency_passed = $consistencyPass
    failure_closure_passed = $failurePass
    round_passed = ($entryPass -and $consistencyPass -and $failurePass)
  }
}

$entryCount = @($roundsDetail | Where-Object { $_.entry_passed }).Count
$consistencyCount = @($roundsDetail | Where-Object { $_.consistency_passed }).Count
$failureCount = @($roundsDetail | Where-Object { $_.failure_closure_passed }).Count
$roundPassCount = @($roundsDetail | Where-Object { $_.round_passed }).Count

$summary = [ordered]@{
  entry_count = $entryCount
  consistency_count = $consistencyCount
  failure_closure_count = $failureCount
  round_pass_count = $roundPassCount
  entry_rate = [Math]::Round($entryCount / $Rounds, 4)
  consistency_rate = [Math]::Round($consistencyCount / $Rounds, 4)
  failure_closure_rate = [Math]::Round($failureCount / $Rounds, 4)
  round_pass_rate = [Math]::Round($roundPassCount / $Rounds, 4)
}

$requiredRounds = 5
$roundsOk = $Rounds -ge $requiredRounds
$entryOk = $summary.entry_rate -ge $MinEntryPassRate
$consistencyOk = $summary.consistency_rate -ge $MinConsistencyRate
$failureOk = $summary.failure_closure_rate -ge $MinFailureClosureRate
$gateReady = $roundsOk -and $entryOk -and $consistencyOk -and $failureOk

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  rounds = $Rounds
  status = $(if ($roundPassCount -eq $Rounds) { "passed" } else { "failed" })
  summary = $summary
  gate_e = [ordered]@{
    required_rounds = $requiredRounds
    required_entry_rate = $MinEntryPassRate
    required_consistency_rate = $MinConsistencyRate
    required_failure_closure_rate = $MinFailureClosureRate
    rounds_ok = $roundsOk
    entry_ok = $entryOk
    consistency_ok = $consistencyOk
    failure_closure_ok = $failureOk
    ready = $gateReady
  }
  rounds_detail = $roundsDetail
  artifacts = [ordered]@{
    report = $outFile
    entry_latest = $entryLatest
    consistency_latest = $consistencyLatest
    failure_latest = $failureLatest
  }
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
if ($report.status -ne "passed") {
  throw "Gate-E 批量验收存在失败轮次，见 $outFile"
}
if ($RequireGateE -and -not $gateReady) {
  throw "Gate-E 判定未达标，见 $outFile"
}
