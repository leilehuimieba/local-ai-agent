param(
  [int]$Rounds = 3,
  [switch]$RequirePass
)

$ErrorActionPreference = "Stop"

if ($Rounds -le 0) {
  throw "Rounds 必须大于 0"
}

$root = Split-Path -Parent $PSScriptRoot
$installScript = Join-Path $PSScriptRoot "run-stage-f-install-acceptance.ps1"
$doctorScript = Join-Path $PSScriptRoot "run-stage-f-doctor-acceptance.ps1"
$entryScript = Join-Path $PSScriptRoot "run-stage-e-entry1-acceptance.ps1"
$consistencyScript = Join-Path $PSScriptRoot "run-stage-e-consistency-acceptance.ps1"
$failureScript = Join-Path $PSScriptRoot "run-stage-e-entry-failure-acceptance.ps1"

$installLatest = Join-Path $root "tmp\stage-f-install\latest.json"
$doctorLatest = Join-Path $root "tmp\stage-f-doctor\latest.json"
$entryLatest = Join-Path $root "tmp\stage-e-entry1\latest.json"
$consistencyLatest = Join-Path $root "tmp\stage-e-consistency\latest.json"
$failureLatest = Join-Path $root "tmp\stage-e-entry-failure\latest.json"

$outDir = Join-Path $root "tmp\stage-f-rc"
$outFile = Join-Path $outDir "latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$roundsDetail = @()
for ($i = 1; $i -le $Rounds; $i++) {
  powershell -ExecutionPolicy Bypass -File $installScript | Out-Null
  $install = Get-Content -Raw $installLatest | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $doctorScript | Out-Null
  $doctor = Get-Content -Raw $doctorLatest | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $entryScript | Out-Null
  $entry = Get-Content -Raw $entryLatest | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  $consistency = Get-Content -Raw $consistencyLatest | ConvertFrom-Json
  powershell -ExecutionPolicy Bypass -File $failureScript | Out-Null
  $failure = Get-Content -Raw $failureLatest | ConvertFrom-Json

  $installPass = $install.status -eq "passed" -and
    $install.install.artifact_ok -and
    $install.install.boot.gateway_ready -and
    $install.install.boot.runtime_ready -and
    $install.upgrade.backup_ok -and
    $install.upgrade.version_file_matched -and
    $install.upgrade.boot.gateway_ready -and
    $install.upgrade.boot.runtime_ready

  $doctorChecks = $doctor.doctor.checks
  $doctorPass = $doctor.status -eq "passed" -and
    $doctorChecks.go_available -and
    $doctorChecks.rust_available -and
    $doctorChecks.node_available -and
    $doctorChecks.npm_available -and
    $doctorChecks.config_exists -and
    $doctorChecks.ports_valid -and
    $doctorChecks.frontend_dist_exists -and
    $doctorChecks.runtime_health_ok -and
    $doctorChecks.gateway_health_ok -and
    $doctorChecks.logs_writable

  $entryChecks = $entry.checks
  $entryPass = $entry.status -eq "passed" -and
    $entryChecks.protocol_fields_ok -and
    $entryChecks.all_run_matched -and
    $entryChecks.all_session_matched -and
    $entryChecks.has_run_started -and
    $entryChecks.terminal_completed

  $consChecks = $consistency.checks
  $consistencyPass = $consistency.status -eq "passed" -and
    $consChecks.accepted_id_matched -and
    $consChecks.runtime_result_matched -and
    $consChecks.all_gateway_run_matched -and
    $consChecks.all_gateway_session_matched -and
    $consChecks.terminal_type_matched -and
    $consChecks.terminal_tool_matched -and
    $consChecks.completion_status_matched -and
    $consChecks.gateway_trace_matched

  $failureChecks = $failure.checks
  $faultInjectionPass = $failure.status -eq "passed" -and
    $failureChecks.all_run_matched -and
    $failureChecks.all_session_matched -and
    $failureChecks.has_run_failed -and
    $failureChecks.has_run_finished -and
    $failureChecks.terminal_is_run_finished -and
    $failureChecks.error_code_runtime_unavailable

  $regressionPass = $installPass -and $doctorPass -and $entryPass -and $consistencyPass
  $roundPass = $regressionPass -and $faultInjectionPass
  $roundsDetail += [ordered]@{
    round = $i
    install_checked_at = $install.checked_at
    doctor_checked_at = $doctor.checked_at
    entry_checked_at = $entry.checked_at
    consistency_checked_at = $consistency.checked_at
    failure_checked_at = $failure.checked_at
    install_passed = $installPass
    doctor_passed = $doctorPass
    entry_passed = $entryPass
    consistency_passed = $consistencyPass
    fault_injection_passed = $faultInjectionPass
    regression_passed = $regressionPass
    round_passed = $roundPass
  }
}

$regressionCount = @($roundsDetail | Where-Object { $_.regression_passed }).Count
$faultCount = @($roundsDetail | Where-Object { $_.fault_injection_passed }).Count
$roundPassCount = @($roundsDetail | Where-Object { $_.round_passed }).Count
$status = if ($roundPassCount -eq $Rounds) { "passed" } else { "failed" }

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  rounds = $Rounds
  status = $status
  summary = [ordered]@{
    regression_count = $regressionCount
    fault_injection_count = $faultCount
    round_pass_count = $roundPassCount
    regression_rate = [Math]::Round($regressionCount / $Rounds, 4)
    fault_injection_rate = [Math]::Round($faultCount / $Rounds, 4)
    round_pass_rate = [Math]::Round($roundPassCount / $Rounds, 4)
  }
  release_candidate = [ordered]@{
    regression_ready = ($regressionCount -eq $Rounds)
    fault_injection_ready = ($faultCount -eq $Rounds)
    ready = ($roundPassCount -eq $Rounds)
  }
  rounds_detail = $roundsDetail
  artifacts = [ordered]@{
    report = $outFile
    install_latest = $installLatest
    doctor_latest = $doctorLatest
    entry_latest = $entryLatest
    consistency_latest = $consistencyLatest
    failure_latest = $failureLatest
  }
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
if ($RequirePass -and $status -ne "passed") {
  throw "stage-f release candidate acceptance failed: $outFile"
}
