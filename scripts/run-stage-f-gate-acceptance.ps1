param(
  [switch]$RefreshEvidence,
  [switch]$RequireGateF
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$installScript = Join-Path $PSScriptRoot "run-stage-f-install-acceptance.ps1"
$doctorScript = Join-Path $PSScriptRoot "run-stage-f-doctor-acceptance.ps1"
$rcScript = Join-Path $PSScriptRoot "run-stage-f-rc-acceptance.ps1"
$windowsScript = Join-Path $PSScriptRoot "run-stage-f-windows-acceptance.ps1"

$installLatest = Join-Path $root "tmp\stage-f-install\latest.json"
$doctorLatest = Join-Path $root "tmp\stage-f-doctor\latest.json"
$rcLatest = Join-Path $root "tmp\stage-f-rc\latest.json"
$windowsLatest = Join-Path $root "tmp\stage-f-windows\latest.json"

$statusDocs = @(
  (Join-Path $root "docs\11-hermes-rebuild\changes\F-install-upgrade-20260414\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\F-doctor-core-checks-20260414\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\F-release-candidate-regression-20260414\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\F-windows-10min-verification-20260414\status.md")
)

$outDir = Join-Path $root "tmp\stage-f-gate"
$outFile = Join-Path $outDir "latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

if ($RefreshEvidence) {
  powershell -ExecutionPolicy Bypass -File $installScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $doctorScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $rcScript -Rounds 3 -RequirePass | Out-Null
  powershell -ExecutionPolicy Bypass -File $windowsScript -RequirePass | Out-Null
}

$install = Get-Content -Raw $installLatest | ConvertFrom-Json
$doctor = Get-Content -Raw $doctorLatest | ConvertFrom-Json
$rc = Get-Content -Raw $rcLatest | ConvertFrom-Json
$windows = Get-Content -Raw $windowsLatest | ConvertFrom-Json

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

$rcPass = $rc.status -eq "passed" -and
  $rc.release_candidate.regression_ready -and
  $rc.release_candidate.fault_injection_ready -and
  $rc.release_candidate.ready

$windowsPass = $windows.status -eq "passed" -and
  $windows.checks.gateway_ready -and
  $windows.checks.runtime_ready -and
  $windows.checks.first_task_completed -and
  $windows.checks.within_time_budget -and
  [string]$windows.first_task.terminal.event_type -eq "run_finished" -and
  [string]$windows.first_task.terminal.completion_status -eq "completed"

$blockerChecks = @()
foreach ($doc in $statusDocs) {
  $exists = Test-Path $doc
  $content = $(if ($exists) { Get-Content -Raw $doc } else { "" })
  $noBlocker = $exists -and ($content -match "- 阻塞点：\s*\r?\n\s*(?:-|\d+\.)\s*无(?:硬)?阻塞")
  $blockerChecks += [ordered]@{
    path = $doc
    exists = $exists
    no_blocker = $noBlocker
  }
}
$noOpenP0P1 = @($blockerChecks | Where-Object { -not $_.no_blocker }).Count -eq 0

$gateReady = $installPass -and $doctorPass -and $rcPass -and $windowsPass -and $noOpenP0P1
$status = if ($gateReady) { "passed" } else { "failed" }

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $status
  gate_f = [ordered]@{
    install_ready = $installPass
    doctor_ready = $doctorPass
    release_candidate_ready = $rcPass
    windows_10min_ready = $windowsPass
    no_open_p0_p1 = $noOpenP0P1
    ready = $gateReady
  }
  evidence = [ordered]@{
    install_latest = $installLatest
    doctor_latest = $doctorLatest
    release_candidate_latest = $rcLatest
    windows_latest = $windowsLatest
  }
  blocker_checks = $blockerChecks
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequireGateF -and -not $gateReady) {
  throw "Gate-F 判定未达标，见 $outFile"
}

