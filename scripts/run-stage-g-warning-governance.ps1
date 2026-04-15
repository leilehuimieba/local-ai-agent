param(
  [switch]$RefreshEvidence,
  [string]$ReportPath = "",
  [string]$WarningAuditPath = "",
  [string]$TrackerPath = "",
  [string]$OutPath = "",
  [int]$EscalateThreshold = 2,
  [string]$WarningAuditExecutor = "",
  [string]$WarningAuditTrackingId = "",
  [string]$WarningAuditDueAt = "",
  [switch]$RequirePass
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$freshnessScript = Join-Path $PSScriptRoot "run-stage-g-evidence-freshness.ps1"
if ([string]::IsNullOrWhiteSpace($ReportPath)) {
  $ReportPath = Join-Path $root "tmp\stage-g-evidence-freshness\latest.json"
}
if ([string]::IsNullOrWhiteSpace($WarningAuditPath)) {
  $WarningAuditPath = Join-Path $root "tmp\stage-g-evidence-freshness\warning-audit-latest.json"
}
$outDir = Join-Path $root "tmp\stage-g-ops"
if ([string]::IsNullOrWhiteSpace($TrackerPath)) {
  $TrackerPath = Join-Path $outDir "warning-tracker.json"
}
if ([string]::IsNullOrWhiteSpace($OutPath)) {
  $OutPath = Join-Path $outDir "latest.json"
}
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

if ($RefreshEvidence) {
  $refreshArgs = @(
    "-ExecutionPolicy", "Bypass",
    "-File", $freshnessScript,
    "-RefreshEvidence"
  )
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditExecutor)) {
    $refreshArgs += @("-WarningAuditExecutor", $WarningAuditExecutor)
  }
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditTrackingId)) {
    $refreshArgs += @("-WarningAuditTrackingId", $WarningAuditTrackingId)
  }
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditDueAt)) {
    $refreshArgs += @("-WarningAuditDueAt", $WarningAuditDueAt)
  }
  & powershell @refreshArgs | Out-Null
}

if (-not (Test-Path $ReportPath)) {
  throw "freshness report missing: $ReportPath"
}
$freshness = Get-Content -Raw $ReportPath | ConvertFrom-Json
$backendReportPath = [string]$freshness.evidence.backend_reverify_report
if ([string]::IsNullOrWhiteSpace($backendReportPath) -or -not (Test-Path $backendReportPath)) {
  throw "backend reverify report missing: $backendReportPath"
}
$backend = Get-Content -Raw $backendReportPath | ConvertFrom-Json

$warningAudit = $null
if (Test-Path $WarningAuditPath) {
  $warningAudit = Get-Content -Raw $WarningAuditPath | ConvertFrom-Json
}

$warningCodes = @($backend.non_blocking_warnings.warning_codes)
$warningCount = [int]$backend.non_blocking_warnings.count
$auditOwner = ""
$auditTrackingId = ""
$auditDueAt = ""
if ($null -ne $warningAudit) {
  $auditOwner = [string]$warningAudit.audit_owner
  $auditTrackingId = [string]$warningAudit.tracking_id
  $auditDueAt = [string]$warningAudit.due_at
}
$auditFieldsReady = $true
if ($warningCount -gt 0) {
  $auditFieldsReady = -not [string]::IsNullOrWhiteSpace($auditOwner) -and
    -not [string]::IsNullOrWhiteSpace($auditTrackingId) -and
    -not [string]::IsNullOrWhiteSpace($auditDueAt)
}

$existingItems = @{}
$history = @()
if (Test-Path $TrackerPath) {
  $trackerObj = Get-Content -Raw $TrackerPath | ConvertFrom-Json
  if ($null -ne $trackerObj.items) {
    foreach ($prop in $trackerObj.items.PSObject.Properties) {
      $existingItems[[string]$prop.Name] = [int]$prop.Value
    }
  }
  if ($null -ne $trackerObj.history) {
    foreach ($entry in @($trackerObj.history)) {
      $history += $entry
    }
  }
}

$escalatedCodes = @()
foreach ($code in $warningCodes) {
  $name = [string]$code
  if ([string]::IsNullOrWhiteSpace($name)) { continue }
  if ($existingItems.ContainsKey($name)) {
    $existingItems[$name] = [int]$existingItems[$name] + 1
  } else {
    $existingItems[$name] = 1
  }
  if ([int]$existingItems[$name] -ge $EscalateThreshold) {
    $escalatedCodes += $name
  }
}

$history += [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = [string]$freshness.status
  warning_count = $warningCount
  warning_codes = @($warningCodes)
  escalated_codes = @($escalatedCodes)
}
if (@($history).Count -gt 30) {
  $history = @($history | Select-Object -Last 30)
}

$itemsOrdered = [ordered]@{}
foreach ($key in @($existingItems.Keys | Sort-Object)) {
  $itemsOrdered[$key] = [int]$existingItems[$key]
}
$tracker = [ordered]@{
  updated_at = (Get-Date).ToString("o")
  escalate_threshold = $EscalateThreshold
  items = $itemsOrdered
  last_warning_codes = @($warningCodes)
  history = @($history)
}
$tracker | ConvertTo-Json -Depth 8 | Set-Content -Path $TrackerPath -Encoding UTF8

$governanceReady = [string]$freshness.status -eq "passed" -and $auditFieldsReady
$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($governanceReady) { "passed" } else { "failed" })
  checks = [ordered]@{
    freshness_ready = [string]$freshness.status -eq "passed"
    warning_count = $warningCount
    audit_fields_ready = $auditFieldsReady
    tracker_updated = $true
    governance_ready = $governanceReady
  }
  warnings = [ordered]@{
    warning_codes = @($warningCodes)
    escalated_codes = @($escalatedCodes)
    escalation_threshold = $EscalateThreshold
  }
  audit = [ordered]@{
    owner = $auditOwner
    tracking_id = $auditTrackingId
    due_at = $auditDueAt
  }
  evidence = [ordered]@{
    freshness_report = $ReportPath
    backend_reverify_report = $backendReportPath
    warning_audit_record = $(if (Test-Path $WarningAuditPath) { $WarningAuditPath } else { $null })
    tracker = $TrackerPath
  }
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $OutPath -Encoding UTF8
Write-Output $OutPath

if ($RequirePass -and -not $governanceReady) {
  throw "G-02 warning governance check failed: $OutPath"
}
