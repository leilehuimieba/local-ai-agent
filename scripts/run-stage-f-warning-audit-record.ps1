param(
  [string]$ReportPath = "",
  [string]$OutPath = "",
  [string]$Executor = "",
  [string]$TrackingId = "",
  [string]$DueAt = "",
  [switch]$RequireReady
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($ReportPath)) {
  $ReportPath = Join-Path $root "tmp\stage-backend-reverify\latest.json"
}
if ([string]::IsNullOrWhiteSpace($OutPath)) {
  $OutPath = Join-Path $root "tmp\stage-backend-reverify\warning-audit-record.json"
}
if (-not (Test-Path $ReportPath)) {
  throw "report missing: $ReportPath"
}

$report = Get-Content -Raw $ReportPath | ConvertFrom-Json
$failedChecksCount = [int]$report.failed_checks.count
$warningCount = [int]$report.non_blocking_warnings.count
$decision = "pass"
$releaseAllowed = $true
$mustRecordWarning = $false
if ($failedChecksCount -gt 0) {
  $decision = "blocked"
  $releaseAllowed = $false
} elseif ($warningCount -gt 0) {
  $decision = "warn"
  $mustRecordWarning = $true
}

$warnings = @()
foreach ($detail in @($report.non_blocking_warnings.details)) {
  $warnings += [ordered]@{
    warning_code = [string]$detail.warning_code
    title = [string]$detail.title
    description = [string]$detail.description
    priority = [string]$detail.priority
    ui_hint = [string]$detail.ui_hint
    action_label = [string]$detail.action_label
    action_command = [string]$detail.action_command
  }
}

$validationErrors = @()
if ($mustRecordWarning) {
  if ([string]::IsNullOrWhiteSpace($Executor)) { $validationErrors += "warn状态缺少Executor" }
  if ([string]::IsNullOrWhiteSpace($TrackingId)) { $validationErrors += "warn状态缺少TrackingId" }
  if ([string]::IsNullOrWhiteSpace($DueAt)) { $validationErrors += "warn状态缺少DueAt" }
}
$readyForRelease = $releaseAllowed -and (@($validationErrors).Count -eq 0)

$record = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  source_report = $ReportPath
  decision = $decision
  ready_for_release = $readyForRelease
  must_record_warning = $mustRecordWarning
  review = [ordered]@{
    checked_at = [string]$report.checked_at
    status = [string]$report.status
    identity_diff_severity = [string]$report.summary.identity_diff_severity
    failed_checks_count = $failedChecksCount
    non_blocking_warning_count = $warningCount
  }
  warning_codes = @($report.non_blocking_warnings.warning_codes)
  warning_details = @($warnings)
  audit_owner = $Executor
  tracking_id = $TrackingId
  due_at = $DueAt
  validation_errors = @($validationErrors)
}

$outDir = Split-Path -Parent $OutPath
if (-not [string]::IsNullOrWhiteSpace($outDir)) {
  New-Item -ItemType Directory -Force -Path $outDir | Out-Null
}
$record | ConvertTo-Json -Depth 8 | Set-Content -Path $OutPath -Encoding UTF8
Write-Output $OutPath

if ($RequireReady -and -not $readyForRelease) {
  throw "warning audit record not ready for release: $OutPath"
}
