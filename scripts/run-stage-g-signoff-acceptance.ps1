param(
  [switch]$RefreshEvidence,
  [switch]$RequireGateG,
  [string]$WarningAuditExecutor = "g-signoff",
  [string]$WarningAuditTrackingId = "",
  [string]$WarningAuditDueAt = ""
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$gateScript = Join-Path $PSScriptRoot "run-stage-g-gate-acceptance.ps1"
$opsScript = Join-Path $PSScriptRoot "run-stage-g-warning-governance.ps1"
$regressionScript = Join-Path $PSScriptRoot "run-stage-g-regression-baseline.ps1"
$outDir = Join-Path $root "tmp\stage-g-signoff"
$outFile = Join-Path $outDir "latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

if ([string]::IsNullOrWhiteSpace($WarningAuditTrackingId)) {
  $WarningAuditTrackingId = "GG1-" + (Get-Date).ToString("yyyyMMddHHmmss")
}
if ([string]::IsNullOrWhiteSpace($WarningAuditDueAt)) {
  $WarningAuditDueAt = (Get-Date).AddDays(1).ToString("yyyy-MM-ddTHH:mm:sszzz")
}

$gateArgs = @("-ExecutionPolicy", "Bypass", "-File", $gateScript, "-WarningAuditExecutor", $WarningAuditExecutor, "-WarningAuditTrackingId", $WarningAuditTrackingId, "-WarningAuditDueAt", $WarningAuditDueAt, "-RequirePass")
if ($RefreshEvidence) { $gateArgs += "-RefreshEvidence" }
& powershell @gateArgs | Out-Null
if ($LASTEXITCODE -ne 0) {
  throw "G-G1 gate acceptance failed"
}

$opsArgs = @("-ExecutionPolicy", "Bypass", "-File", $opsScript, "-WarningAuditExecutor", $WarningAuditExecutor, "-WarningAuditTrackingId", $WarningAuditTrackingId, "-WarningAuditDueAt", $WarningAuditDueAt, "-RequirePass")
if ($RefreshEvidence) { $opsArgs += "-RefreshEvidence" }
& powershell @opsArgs | Out-Null
if ($LASTEXITCODE -ne 0) {
  throw "G-G1 warning governance failed"
}

$regressionArgs = @("-ExecutionPolicy", "Bypass", "-File", $regressionScript, "-Rounds", "3", "-RequirePass")
if ($RefreshEvidence) { $regressionArgs += "-RefreshEvidence" }
& powershell @regressionArgs | Out-Null
if ($LASTEXITCODE -ne 0) {
  throw "G-G1 regression baseline failed"
}

$gateReport = Get-Content -Raw (Join-Path $root "tmp\stage-g-gate\latest.json") | ConvertFrom-Json
$opsReport = Get-Content -Raw (Join-Path $root "tmp\stage-g-ops\latest.json") | ConvertFrom-Json
$regReport = Get-Content -Raw (Join-Path $root "tmp\stage-g-regression\latest.json") | ConvertFrom-Json

$statusDocs = @(
  (Join-Path $root "docs\11-hermes-rebuild\changes\G-evidence-freshness-policy-20260414\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\G-warning-governance-closure-20260415\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\G-regression-baseline-20260415\status.md"),
  (Join-Path $root "docs\11-hermes-rebuild\changes\G-runbook-duty-closure-20260415\status.md")
)
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

$auditFieldsReady = -not [string]::IsNullOrWhiteSpace([string]$opsReport.audit.owner) -and
  -not [string]::IsNullOrWhiteSpace([string]$opsReport.audit.tracking_id) -and
  -not [string]::IsNullOrWhiteSpace([string]$opsReport.audit.due_at)
$regReady = [string]$regReport.status -eq "passed" -and [double]$regReport.summary.pass_rate -ge 95 -and [bool]$regReport.summary.ready

$ready = [string]$gateReport.status -eq "passed" -and
  [bool]$gateReport.gate_g.ready -and
  [string]$opsReport.status -eq "passed" -and
  [bool]$opsReport.checks.governance_ready -and
  $auditFieldsReady -and
  $regReady -and
  $noOpenP0P1

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($ready) { "passed" } else { "failed" })
  gate_g_signoff = [ordered]@{
    g01_ready = [bool]$gateReport.gate_g.ready
    g02_ready = [bool]$opsReport.checks.governance_ready
    g03_ready = $regReady
    warning_audit_fields_ready = $auditFieldsReady
    no_open_p0_p1 = $noOpenP0P1
    ready = $ready
  }
  evidence = [ordered]@{
    g_gate = (Join-Path $root "tmp\stage-g-gate\latest.json")
    g_ops = (Join-Path $root "tmp\stage-g-ops\latest.json")
    g_regression = (Join-Path $root "tmp\stage-g-regression\latest.json")
  }
  blocker_checks = @($blockerChecks)
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequireGateG -and -not $ready) {
  throw "Gate-G signoff failed: $outFile"
}
