# 阶段 G 证据保鲜聚合验收（G-01）

param(
  [switch]$RefreshEvidence,
  [switch]$ReleaseWindow,
  [string]$WarningAuditExecutor = "",
  [string]$WarningAuditTrackingId = "",
  [string]$WarningAuditDueAt = "",
  [switch]$RequirePass
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$freshnessScript = Join-Path $PSScriptRoot "run-stage-g-evidence-freshness.ps1"
$freshnessLatest = Join-Path $root "tmp\stage-g-evidence-freshness\latest.json"

$outDir = Join-Path $root "tmp\stage-g-gate"
$outFile = Join-Path $outDir "latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$args = @(
  "-ExecutionPolicy", "Bypass",
  "-File", $freshnessScript
)
if ($RefreshEvidence) {
  $args += "-RefreshEvidence"
}
if ($ReleaseWindow) {
  $args += "-ReleaseWindow"
}
if (-not [string]::IsNullOrWhiteSpace($WarningAuditExecutor)) {
  $args += @("-WarningAuditExecutor", $WarningAuditExecutor)
}
if (-not [string]::IsNullOrWhiteSpace($WarningAuditTrackingId)) {
  $args += @("-WarningAuditTrackingId", $WarningAuditTrackingId)
}
if (-not [string]::IsNullOrWhiteSpace($WarningAuditDueAt)) {
  $args += @("-WarningAuditDueAt", $WarningAuditDueAt)
}

& powershell @args | Out-Null

$freshness = Get-Content -Raw $freshnessLatest | ConvertFrom-Json

$pass = [string]$freshness.status -eq "passed" -and
  [bool]$freshness.checks.backend_reverify_ready -and
  [bool]$freshness.checks.strict_gate_ready -and
  [int]$freshness.checks.failed_checks_count -eq 0

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($pass) { "passed" } else { "failed" })
  gate_g = [ordered]@{
    evidence_freshness_ready = [bool]$freshness.checks.backend_reverify_ready
    strict_gate_ready = [bool]$freshness.checks.strict_gate_ready
    failed_checks_count = [int]$freshness.checks.failed_checks_count
    non_blocking_warning_count = [int]$freshness.checks.non_blocking_warning_count
    ready = $pass
  }
  policy = $freshness.policy
  evidence = [ordered]@{
    freshness_latest = $freshnessLatest
    backend_reverify_report = $freshness.evidence.backend_reverify_report
    warning_audit_record = $freshness.evidence.warning_audit_record
  }
}

$report | ConvertTo-Json -Depth 6 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequirePass -and -not $pass) {
  throw "Gate-G G-01 未达标，见 $outFile"
}
