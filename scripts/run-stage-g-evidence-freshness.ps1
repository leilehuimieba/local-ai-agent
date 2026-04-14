param(
  [switch]$RefreshEvidence,
  [switch]$ReleaseWindow,
  [int]$RoutineMaxEvidenceAgeMinutes = 180,
  [string]$WarningAuditExecutor = "",
  [string]$WarningAuditTrackingId = "",
  [string]$WarningAuditDueAt = ""
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$backendScript = Join-Path $PSScriptRoot "run-stage-backend-reverify-pack.ps1"
$backendLatest = Join-Path $root "tmp\stage-backend-reverify\latest.json"

$outDir = Join-Path $root "tmp\stage-g-evidence-freshness"
$outFile = Join-Path $outDir "latest.json"
$warningAuditOut = Join-Path $outDir "warning-audit-latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$effectiveMaxEvidenceAgeMinutes = $(if ($ReleaseWindow) { 30 } else { $RoutineMaxEvidenceAgeMinutes })
$rerunFrequencyMinutes = $(if ($ReleaseWindow) { 30 } else { $RoutineMaxEvidenceAgeMinutes })

$args = @(
  "-ExecutionPolicy", "Bypass",
  "-File", $backendScript,
  "-StrictGate",
  "-MaxEvidenceAgeMinutes", $effectiveMaxEvidenceAgeMinutes,
  "-EmitWarningAuditRecord",
  "-WarningAuditOutPath", $warningAuditOut,
  "-RequirePass"
)

if ($RefreshEvidence) {
  $args += "-RefreshEvidence"
}
if ($ReleaseWindow) {
  $args += "-ReleaseWindow"
  $args += "-RequireWarningAuditReady"
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

$backendOutput = & powershell @args
$backendReportPath = [string]($backendOutput | Select-Object -First 1)
if ([string]::IsNullOrWhiteSpace($backendReportPath)) {
  $backendReportPath = $backendLatest
}

$backend = Get-Content -Raw $backendReportPath | ConvertFrom-Json
$warningAudit = $null
if (Test-Path $warningAuditOut) {
  $warningAudit = Get-Content -Raw $warningAuditOut | ConvertFrom-Json
}

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = [string]$backend.status
  mode = $(if ($ReleaseWindow) { "release_window" } else { "routine" })
  policy = [ordered]@{
    max_evidence_age_minutes = $effectiveMaxEvidenceAgeMinutes
    rerun_frequency_minutes = $rerunFrequencyMinutes
    owner_rule = "warn状态必须登记Executor/TrackingId/DueAt"
    refresh_evidence = [bool]$RefreshEvidence
  }
  checks = [ordered]@{
    backend_reverify_ready = [bool]$backend.summary.backend_reverify_ready
    strict_gate_ready = [bool]$backend.summary.strict_gate_ready
    failed_checks_count = [int]$backend.failed_checks.count
    non_blocking_warning_count = [int]$backend.non_blocking_warnings.count
    warning_audit_ready_for_release = $(if ($null -ne $warningAudit) { [bool]$warningAudit.ready_for_release } else { $null })
  }
  evidence = [ordered]@{
    backend_reverify_report = $backendReportPath
    warning_audit_record = $(if (Test-Path $warningAuditOut) { $warningAuditOut } else { $null })
  }
}

$report | ConvertTo-Json -Depth 6 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
