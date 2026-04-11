param(
  [int]$Rounds = 5,
  [switch]$RequireGateB
)

$ErrorActionPreference = "Stop"

if ($Rounds -le 0) {
  throw "Rounds 必须大于 0"
}

$root = Split-Path -Parent $PSScriptRoot
$confirmScript = Join-Path $PSScriptRoot "run-stage-b-confirmation-acceptance.ps1"
$retryScript = Join-Path $PSScriptRoot "run-stage-b-retry-acceptance.ps1"
$confirmLatest = Join-Path $root "tmp\stage-b-confirmation-acceptance\latest.json"
$retryLatest = Join-Path $root "tmp\stage-b-retry-acceptance\latest.json"
$outDir = Join-Path $root "tmp\stage-b-acceptance-batch"
$outFile = Join-Path $outDir "latest.json"

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$roundsDetail = @()

for ($i = 1; $i -le $Rounds; $i++) {
  powershell -ExecutionPolicy Bypass -File $confirmScript | Out-Null
  $confirm = Get-Content -Path $confirmLatest -Raw | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $retryScript | Out-Null
  $retry = Get-Content -Path $retryLatest -Raw | ConvertFrom-Json

  $confirmPassed = $confirm.status -eq "passed" -and
    $confirm.after_confirmation.target_resumed_unique -and
    $confirm.after_confirmation.checkpoint_id_matched -and
    $confirm.after_confirmation.boundary_recovered -and
    $confirm.after_confirmation.event_type_matched -and
    $confirm.after_confirmation.reason_matched -and
    $confirm.after_confirmation.stage_matched -and
    $confirm.after_confirmation.verification_empty

  $retryPassed = $retry.status -eq "passed" -and
    $retry.retry_run.target_resumed_unique -and
    $retry.retry_run.checkpoint_id_matched -and
    $retry.retry_run.boundary_recovered -and
    $retry.retry_run.event_type_matched -and
    $retry.retry_run.reason_matched -and
    $retry.retry_run.stage_matched -and
    $retry.retry_run.verification_recovered -and
    $retry.retry_run.artifact_recovered

  $roundsDetail += [ordered]@{
    round = $i
    confirm_checked_at = $confirm.checked_at
    retry_checked_at = $retry.checked_at
    confirm_session_id = $confirm.session_id
    retry_session_id = $retry.session_id
    confirm_run_id = $confirm.after_confirmation.run_id
    retry_run_id = $retry.retry_run.run_id
    confirm_boundary_recovered = $confirm.after_confirmation.boundary_recovered
    retry_boundary_recovered = $retry.retry_run.boundary_recovered
    confirm_event_type = $confirm.after_confirmation.checkpoint_resume_event_type
    retry_event_type = $retry.retry_run.checkpoint_resume_event_type
    confirm_event_type_matched = $confirm.after_confirmation.event_type_matched
    retry_event_type_matched = $retry.retry_run.event_type_matched
    confirm_checkpoint_id_matched = $confirm.after_confirmation.checkpoint_id_matched
    retry_checkpoint_id_matched = $retry.retry_run.checkpoint_id_matched
    confirm_verification_empty = $confirm.after_confirmation.verification_empty
    retry_verification_recovered = $retry.retry_run.verification_recovered
    retry_artifact_recovered = $retry.retry_run.artifact_recovered
    confirm_passed = $confirmPassed
    retry_passed = $retryPassed
    round_passed = ($confirmPassed -and $retryPassed)
  }
}

$confirmPassCount = @($roundsDetail | Where-Object { $_.confirm_passed }).Count
$retryPassCount = @($roundsDetail | Where-Object { $_.retry_passed }).Count
$roundPassCount = @($roundsDetail | Where-Object { $_.round_passed }).Count
$confirmBoundaryCount = @($roundsDetail | Where-Object { $_.confirm_boundary_recovered }).Count
$retryBoundaryCount = @($roundsDetail | Where-Object { $_.retry_boundary_recovered }).Count
$confirmEventTypeCount = @($roundsDetail | Where-Object { $_.confirm_event_type_matched }).Count
$retryEventTypeCount = @($roundsDetail | Where-Object { $_.retry_event_type_matched }).Count
$confirmCheckpointIdCount = @($roundsDetail | Where-Object { $_.confirm_checkpoint_id_matched }).Count
$retryCheckpointIdCount = @($roundsDetail | Where-Object { $_.retry_checkpoint_id_matched }).Count
$confirmVerificationEmptyCount = @($roundsDetail | Where-Object { $_.confirm_verification_empty }).Count
$retryVerificationRecoveredCount = @($roundsDetail | Where-Object { $_.retry_verification_recovered }).Count
$retryArtifactRecoveredCount = @($roundsDetail | Where-Object { $_.retry_artifact_recovered }).Count
$summary = [ordered]@{
  confirm_pass_count = $confirmPassCount
  retry_pass_count = $retryPassCount
  round_pass_count = $roundPassCount
  confirm_pass_rate = [Math]::Round($confirmPassCount / $Rounds, 4)
  retry_pass_rate = [Math]::Round($retryPassCount / $Rounds, 4)
  round_pass_rate = [Math]::Round($roundPassCount / $Rounds, 4)
  confirm_boundary_count = $confirmBoundaryCount
  retry_boundary_count = $retryBoundaryCount
  confirm_boundary_rate = [Math]::Round($confirmBoundaryCount / $Rounds, 4)
  retry_boundary_rate = [Math]::Round($retryBoundaryCount / $Rounds, 4)
  confirm_event_type_count = $confirmEventTypeCount
  retry_event_type_count = $retryEventTypeCount
  confirm_event_type_rate = [Math]::Round($confirmEventTypeCount / $Rounds, 4)
  retry_event_type_rate = [Math]::Round($retryEventTypeCount / $Rounds, 4)
  confirm_checkpoint_id_count = $confirmCheckpointIdCount
  retry_checkpoint_id_count = $retryCheckpointIdCount
  confirm_checkpoint_id_rate = [Math]::Round($confirmCheckpointIdCount / $Rounds, 4)
  retry_checkpoint_id_rate = [Math]::Round($retryCheckpointIdCount / $Rounds, 4)
  confirm_verification_empty_count = $confirmVerificationEmptyCount
  retry_verification_recovered_count = $retryVerificationRecoveredCount
  retry_artifact_recovered_count = $retryArtifactRecoveredCount
  confirm_verification_empty_rate = [Math]::Round($confirmVerificationEmptyCount / $Rounds, 4)
  retry_verification_recovered_rate = [Math]::Round($retryVerificationRecoveredCount / $Rounds, 4)
  retry_artifact_recovered_rate = [Math]::Round($retryArtifactRecoveredCount / $Rounds, 4)
}

$gateRequiredRounds = 50
$gateRequiredRate = 0.95
$gateRoundsOk = $Rounds -ge $gateRequiredRounds
$gateConfirmRateOk = $summary.confirm_pass_rate -ge $gateRequiredRate
$gateRetryRateOk = $summary.retry_pass_rate -ge $gateRequiredRate
$gateRoundRateOk = $summary.round_pass_rate -ge $gateRequiredRate
$gateReady = $gateRoundsOk -and $gateConfirmRateOk -and $gateRetryRateOk -and $gateRoundRateOk

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  rounds = $Rounds
  status = $(if ($roundPassCount -eq $Rounds) { "passed" } else { "failed" })
  summary = $summary
  gate_b = [ordered]@{
    required_rounds = $gateRequiredRounds
    required_rate = $gateRequiredRate
    rounds_ok = $gateRoundsOk
    confirm_rate_ok = $gateConfirmRateOk
    retry_rate_ok = $gateRetryRateOk
    round_rate_ok = $gateRoundRateOk
    ready = $gateReady
  }
  rounds_detail = $roundsDetail
  artifacts = [ordered]@{
    report = $outFile
    confirm_latest = $confirmLatest
    retry_latest = $retryLatest
  }
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
if ($roundPassCount -ne $Rounds) {
  throw "批量验收存在失败轮次，见 $outFile"
}
if ($RequireGateB -and -not $gateReady) {
  throw "Gate-B 判定未达标，见 $outFile"
}
