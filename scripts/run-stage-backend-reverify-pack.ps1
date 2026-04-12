param(
  [switch]$RefreshEvidence,
  [switch]$RequirePass,
  [switch]$StrictGate,
  [int]$MaxEvidenceAgeMinutes = 180,
  [switch]$ReleaseWindow,
  [switch]$EmitWarningAuditRecord,
  [string]$WarningAuditOutPath = "",
  [string]$WarningAuditExecutor = "",
  [string]$WarningAuditTrackingId = "",
  [string]$WarningAuditDueAt = "",
  [switch]$RequireWarningAuditReady
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$historyScript = Join-Path $PSScriptRoot "run-stage-e-cli-history-acceptance.ps1"
$cancelScript = Join-Path $PSScriptRoot "run-stage-e-cli-cancel-acceptance.ps1"
$consistencyScript = Join-Path $PSScriptRoot "run-stage-e-consistency-acceptance.ps1"
$gateScript = Join-Path $PSScriptRoot "run-stage-f-gate-acceptance.ps1"
$warningAuditScript = Join-Path $PSScriptRoot "run-stage-f-warning-audit-record.ps1"

$historyLatest = Join-Path $root "tmp\stage-e-cli-history\latest.json"
$cancelLatest = Join-Path $root "tmp\stage-e-cli-cancel\latest.json"
$consistencyLatest = Join-Path $root "tmp\stage-e-consistency\latest.json"
$gateLatest = Join-Path $root "tmp\stage-f-gate\latest.json"

$outDir = Join-Path $root "tmp\stage-backend-reverify"
$outFile = Join-Path $outDir "latest.json"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

function Get-EvidenceAgeMinutes {
  param($Doc, [string]$Label)
  $checkedAtText = [string]$Doc.checked_at
  if ([string]::IsNullOrWhiteSpace($checkedAtText)) {
    throw "$Label.checked_at is missing"
  }
  try {
    $checkedAt = [DateTimeOffset]::Parse($checkedAtText)
  } catch {
    throw "$Label.checked_at is invalid: $checkedAtText"
  }
  $age = ([DateTimeOffset]::Now - $checkedAt).TotalMinutes
  return [Math]::Round($age, 2)
}

function Has-Property {
  param($Doc, [string]$Name)
  return $null -ne $Doc -and $Doc.PSObject.Properties.Name -contains $Name
}

function Add-ReasonIfFailed {
  param([bool]$Condition, [string]$ReasonCode, [ref]$Bucket)
  if (-not $Condition) {
    $Bucket.Value += $ReasonCode
  }
}

function Add-UniqueText {
  param([string]$Item, [ref]$Bucket)
  if ([string]::IsNullOrWhiteSpace($Item)) { return }
  if (-not ($Bucket.Value -contains $Item)) {
    $Bucket.Value += $Item
  }
}

function Build-StrictAgeRefreshCommands {
  param($StrictAges, [int]$MaxAgeMinutes)
  $commands = @()
  if ($StrictAges.e01_history_age_minutes -gt $MaxAgeMinutes) {
    Add-UniqueText -Item 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-history-acceptance.ps1' -Bucket ([ref]$commands)
  }
  if ($StrictAges.e01_cancel_age_minutes -gt $MaxAgeMinutes) {
    Add-UniqueText -Item 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-cancel-acceptance.ps1' -Bucket ([ref]$commands)
  }
  if ($StrictAges.e04_consistency_age_minutes -gt $MaxAgeMinutes) {
    Add-UniqueText -Item 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1' -Bucket ([ref]$commands)
  }
  if ($StrictAges.fg1_gate_age_minutes -gt $MaxAgeMinutes) {
    Add-UniqueText -Item 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RefreshEvidence -RequireGateF' -Bucket ([ref]$commands)
  }
  if (@($commands).Count -eq 0) {
    Add-UniqueText -Item 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -RequirePass' -Bucket ([ref]$commands)
  }
  return @($commands)
}

function Build-FullRefreshCommand {
  param([bool]$UseReleaseWindow)
  $cmd = 'powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate'
  if ($UseReleaseWindow) {
    $cmd += ' -ReleaseWindow'
  }
  return ($cmd + ' -RequirePass')
}

$reasonResolutionMap = @{
  e01_history_not_ready = [ordered]@{
    suggestion = 'History slice not ready.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-history-acceptance.ps1')
  }
  e01_cancel_not_ready = [ordered]@{
    suggestion = 'Cancel slice not ready.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-cli-cancel-acceptance.ps1')
  }
  e04_consistency_not_ready = [ordered]@{
    suggestion = 'Cross-entry consistency not ready.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1')
  }
  fg1_gate_not_ready = [ordered]@{
    suggestion = 'Gate-F evidence not ready.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RefreshEvidence -RequireGateF')
  }
  strict_evidence_age_exceeded = [ordered]@{
    suggestion = 'Evidence age exceeded threshold.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -RefreshEvidence -StrictGate -RequirePass')
  }
  strict_consistency_identity_missing = [ordered]@{
    suggestion = 'Consistency identity fields are incomplete.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1')
  }
  strict_consistency_terminal_missing = [ordered]@{
    suggestion = 'Consistency terminal nodes are missing.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1')
  }
  strict_gate_fields_incomplete = [ordered]@{
    suggestion = 'Gate-F required fields are incomplete.'
    minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-f-gate-acceptance.ps1 -RefreshEvidence -RequireGateF')
  }
}

function Build-ReasonResolution {
  param([string]$ReasonCode, $StrictAges, [int]$MaxAgeMinutes, [bool]$UseReleaseWindow)
  $defaultFullRefresh = Build-FullRefreshCommand -UseReleaseWindow $UseReleaseWindow
  $item = $reasonResolutionMap[$ReasonCode]
  if ($null -eq $item) {
    $item = [ordered]@{
      suggestion = 'Unknown reason code.'
      minimal_commands = @('powershell -ExecutionPolicy Bypass -File scripts/run-stage-backend-reverify-pack.ps1 -StrictGate -RequirePass')
      full_refresh_commands = @($defaultFullRefresh)
    }
  }
  $minimalCommands = @($item.minimal_commands)
  $minimalCommands = @($minimalCommands | Where-Object { -not [string]::IsNullOrWhiteSpace([string]$_) })
  if ($ReasonCode -eq 'strict_evidence_age_exceeded') {
    $minimalCommands = Build-StrictAgeRefreshCommands -StrictAges $StrictAges -MaxAgeMinutes $MaxAgeMinutes
  }
  $fullRefreshCommands = @($item.full_refresh_commands)
  $fullRefreshCommands = @($fullRefreshCommands | Where-Object { -not [string]::IsNullOrWhiteSpace([string]$_) })
  if (@($fullRefreshCommands).Count -eq 0) {
    $fullRefreshCommands = @($defaultFullRefresh)
  }
  return [ordered]@{
    reason_code = $ReasonCode
    suggestion = $item.suggestion
    minimal_commands = @($minimalCommands)
    full_refresh_commands = @($fullRefreshCommands)
  }
}

if ($RefreshEvidence) {
  powershell -ExecutionPolicy Bypass -File $historyScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $cancelScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $gateScript -RefreshEvidence -RequireGateF | Out-Null
}

$history = Get-Content -Raw $historyLatest | ConvertFrom-Json
$cancel = Get-Content -Raw $cancelLatest | ConvertFrom-Json
$consistency = Get-Content -Raw $consistencyLatest | ConvertFrom-Json
$gate = Get-Content -Raw $gateLatest | ConvertFrom-Json
$strictEnabled = [bool]$StrictGate
$effectiveMaxEvidenceAgeMinutes = $(if ($ReleaseWindow) { 30 } else { $MaxEvidenceAgeMinutes })

$historyReady = $history.status -eq "passed" -and $history.checks.logs_runs_view -and $history.checks.cli_history_slice_ready
$cancelReady = $cancel.status -eq "passed" -and $cancel.checks.cancel_endpoint_ready -and $cancel.checks.cancel_terminal_status
$consistencyReady = $consistency.status -eq "passed" `
  -and $consistency.checks.accepted_id_matched `
  -and $consistency.checks.runtime_result_matched `
  -and $consistency.checks.all_gateway_run_matched `
  -and $consistency.checks.all_gateway_session_matched `
  -and $consistency.checks.terminal_type_matched `
  -and $consistency.checks.terminal_tool_matched `
  -and $consistency.checks.completion_status_matched `
  -and $consistency.checks.gateway_trace_matched
$gateReady = $gate.status -eq "passed" -and $gate.gate_f.ready

$strictChecks = [ordered]@{
  enabled = $strictEnabled
  evidence_age_within_limit = $true
  consistency_identity_present = $true
  consistency_terminal_present = $true
  gate_fields_complete = $true
}
$strictAges = [ordered]@{}
if ($strictEnabled) {
  $strictAges.e01_history_age_minutes = Get-EvidenceAgeMinutes -Doc $history -Label "e01_history"
  $strictAges.e01_cancel_age_minutes = Get-EvidenceAgeMinutes -Doc $cancel -Label "e01_cancel"
  $strictAges.e04_consistency_age_minutes = Get-EvidenceAgeMinutes -Doc $consistency -Label "e04_consistency"
  $strictAges.fg1_gate_age_minutes = Get-EvidenceAgeMinutes -Doc $gate -Label "fg1_gate"

  $strictChecks.evidence_age_within_limit = $strictAges.e01_history_age_minutes -le $effectiveMaxEvidenceAgeMinutes `
    -and $strictAges.e01_cancel_age_minutes -le $effectiveMaxEvidenceAgeMinutes `
    -and $strictAges.e04_consistency_age_minutes -le $effectiveMaxEvidenceAgeMinutes `
    -and $strictAges.fg1_gate_age_minutes -le $effectiveMaxEvidenceAgeMinutes
  $strictChecks.consistency_identity_present = $null -ne $consistency.run_identity `
    -and -not [string]::IsNullOrWhiteSpace([string]$consistency.run_identity.request_id) `
    -and -not [string]::IsNullOrWhiteSpace([string]$consistency.run_identity.run_id) `
    -and -not [string]::IsNullOrWhiteSpace([string]$consistency.run_identity.session_id) `
    -and -not [string]::IsNullOrWhiteSpace([string]$consistency.run_identity.trace_id)
  $strictChecks.consistency_terminal_present = $null -ne $consistency.runtime.terminal -and $null -ne $consistency.gateway_run.terminal
  $strictChecks.gate_fields_complete = (Has-Property -Doc $gate.gate_f -Name "install_ready") `
    -and (Has-Property -Doc $gate.gate_f -Name "doctor_ready") `
    -and (Has-Property -Doc $gate.gate_f -Name "release_candidate_ready") `
    -and (Has-Property -Doc $gate.gate_f -Name "windows_10min_ready") `
    -and (Has-Property -Doc $gate.gate_f -Name "no_open_p0_p1") `
    -and (Has-Property -Doc $gate.gate_f -Name "ready")
}

$strictReady = $strictChecks.evidence_age_within_limit -and $strictChecks.consistency_identity_present -and $strictChecks.consistency_terminal_present -and $strictChecks.gate_fields_complete
$passed = $historyReady -and $cancelReady -and $consistencyReady -and $gateReady -and ((-not $strictEnabled) -or $strictReady)
$reasonCodes = @()
Add-ReasonIfFailed -Condition $historyReady -ReasonCode "e01_history_not_ready" -Bucket ([ref]$reasonCodes)
Add-ReasonIfFailed -Condition $cancelReady -ReasonCode "e01_cancel_not_ready" -Bucket ([ref]$reasonCodes)
Add-ReasonIfFailed -Condition $consistencyReady -ReasonCode "e04_consistency_not_ready" -Bucket ([ref]$reasonCodes)
Add-ReasonIfFailed -Condition $gateReady -ReasonCode "fg1_gate_not_ready" -Bucket ([ref]$reasonCodes)
if ($strictEnabled) {
  Add-ReasonIfFailed -Condition $strictChecks.evidence_age_within_limit -ReasonCode "strict_evidence_age_exceeded" -Bucket ([ref]$reasonCodes)
  Add-ReasonIfFailed -Condition $strictChecks.consistency_identity_present -ReasonCode "strict_consistency_identity_missing" -Bucket ([ref]$reasonCodes)
  Add-ReasonIfFailed -Condition $strictChecks.consistency_terminal_present -ReasonCode "strict_consistency_terminal_missing" -Bucket ([ref]$reasonCodes)
  Add-ReasonIfFailed -Condition $strictChecks.gate_fields_complete -ReasonCode "strict_gate_fields_incomplete" -Bucket ([ref]$reasonCodes)
}
$reasonResolutions = @()
$recommendedCommandsMinimal = @()
$recommendedCommandsFullRefresh = @()
foreach ($code in @($reasonCodes)) {
  $resolution = Build-ReasonResolution -ReasonCode $code -StrictAges $strictAges -MaxAgeMinutes $effectiveMaxEvidenceAgeMinutes -UseReleaseWindow ([bool]$ReleaseWindow)
  $reasonResolutions += $resolution
  foreach ($cmd in @($resolution.minimal_commands)) {
    Add-UniqueText -Item $cmd -Bucket ([ref]$recommendedCommandsMinimal)
  }
  foreach ($cmd in @($resolution.full_refresh_commands)) {
    Add-UniqueText -Item $cmd -Bucket ([ref]$recommendedCommandsFullRefresh)
  }
}
$warningCodes = @()
$warningSuggestions = @()
$warningDetails = @()
$identityDiffSummary = $consistency.identity_diff_summary
$identityDiffSeverity = [string]$identityDiffSummary.severity
$warningPresetMap = @{
  e04_identity_diff_warn = [ordered]@{
    suggestion = "Identity diff has non-blocking missing fields."
    title = "E-04 identity diff warning"
    description = "Identity diff has missing fields but no blocking mismatch."
    priority = "medium"
    ui_hint = "warning_card"
    action_label = "Refresh E-04 evidence"
    action_command = "powershell -ExecutionPolicy Bypass -File scripts/run-stage-e-consistency-acceptance.ps1"
  }
}
if ([string]::IsNullOrWhiteSpace($identityDiffSeverity)) {
  $identityDiffSeverity = "ok"
}
if ($identityDiffSeverity -eq "warn") {
  $warningCode = "e04_identity_diff_warn"
  $preset = $warningPresetMap[$warningCode]
  Add-UniqueText -Item $warningCode -Bucket ([ref]$warningCodes)
  Add-UniqueText -Item ([string]$preset.suggestion) -Bucket ([ref]$warningSuggestions)
  $warningDetails += [ordered]@{
    warning_code = $warningCode
    title = [string]$preset.title
    description = [string]$preset.description
    priority = [string]$preset.priority
    ui_hint = [string]$preset.ui_hint
    action_label = [string]$preset.action_label
    action_command = [string]$preset.action_command
    source = "e04_consistency"
    severity = $identityDiffSeverity
    mismatch_count = [int]$identityDiffSummary.mismatch_count
    missing_count = [int]$identityDiffSummary.missing_count
    missing_dimensions = @($identityDiffSummary.missing_dimensions)
  }
}

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($passed) { "passed" } else { "failed" })
  summary = [ordered]@{
    e01_history_ready = $historyReady
    e01_cancel_ready = $cancelReady
    e04_consistency_ready = $consistencyReady
    fg1_gate_ready = $gateReady
    strict_gate_ready = $(if ($strictEnabled) { $strictReady } else { $null })
    identity_diff_severity = $identityDiffSeverity
    non_blocking_warning_count = @($warningCodes).Count
    backend_reverify_ready = $passed
  }
  failed_checks = [ordered]@{
    count = @($reasonCodes).Count
    reason_codes = @($reasonCodes)
    suggestions = @($reasonResolutions | ForEach-Object { $_.suggestion })
    resolutions = @($reasonResolutions)
    recommended_commands_minimal = @($recommendedCommandsMinimal)
    recommended_commands_full_refresh = @($recommendedCommandsFullRefresh)
    recommended_commands = @($recommendedCommandsMinimal)
  }
  non_blocking_warnings = [ordered]@{
    count = @($warningCodes).Count
    warning_codes = @($warningCodes)
    suggestions = @($warningSuggestions)
    details = @($warningDetails)
  }
  strict_gate = [ordered]@{
    enabled = $strictEnabled
    release_window = [bool]$ReleaseWindow
    max_evidence_age_minutes = $effectiveMaxEvidenceAgeMinutes
    checks = $strictChecks
    evidence_age_minutes = $strictAges
  }
  interface_evidence = [ordered]@{
    run_identity = $consistency.run_identity
    consistency_checks = $consistency.checks
    identity_diff_summary = $consistency.identity_diff_summary
    identity_diff_groups = $consistency.identity_diff_groups
    runtime_terminal = $consistency.runtime.terminal
    gateway_terminal = $consistency.gateway_run.terminal
  }
  evidence = [ordered]@{
    e01_history_latest = $historyLatest
    e01_cancel_latest = $cancelLatest
    e04_consistency_latest = $consistencyLatest
    fg1_gate_latest = $gateLatest
    report = $outFile
  }
}

$report | ConvertTo-Json -Depth 6 | Set-Content -Path $outFile -Encoding UTF8
$auditRecordPath = ""
if ($EmitWarningAuditRecord) {
  if ([string]::IsNullOrWhiteSpace($WarningAuditOutPath)) {
    $WarningAuditOutPath = Join-Path $outDir "warning-audit-from-pack.json"
  }
  $WarningAuditOutPath = [System.IO.Path]::GetFullPath($WarningAuditOutPath)
  $auditArgs = @(
    "-ExecutionPolicy", "Bypass",
    "-File", $warningAuditScript,
    "-ReportPath", $outFile,
    "-OutPath", $WarningAuditOutPath
  )
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditExecutor)) {
    $auditArgs += @("-Executor", $WarningAuditExecutor)
  }
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditTrackingId)) {
    $auditArgs += @("-TrackingId", $WarningAuditTrackingId)
  }
  if (-not [string]::IsNullOrWhiteSpace($WarningAuditDueAt)) {
    $auditArgs += @("-DueAt", $WarningAuditDueAt)
  }
  if ($RequireWarningAuditReady) {
    $auditArgs += "-RequireReady"
  }
  $auditRecordPath = [string](& powershell @auditArgs)
  if (-not [string]::IsNullOrWhiteSpace($auditRecordPath)) {
    $normalizedAuditPath = [System.IO.Path]::GetFullPath($auditRecordPath.Trim())
    $report.evidence.warning_audit_record = $normalizedAuditPath
    $report | ConvertTo-Json -Depth 6 | Set-Content -Path $outFile -Encoding UTF8
    $auditRecordPath = $normalizedAuditPath
  }
}
Write-Output $outFile
if (-not [string]::IsNullOrWhiteSpace($auditRecordPath)) {
  Write-Output $auditRecordPath.Trim()
}

if ($RequirePass -and -not $passed) {
  throw "backend reverify pack failed: $outFile"
}
