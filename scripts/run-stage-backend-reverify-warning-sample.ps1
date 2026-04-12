$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$historyScript = Join-Path $PSScriptRoot "run-stage-e-cli-history-acceptance.ps1"
$cancelScript = Join-Path $PSScriptRoot "run-stage-e-cli-cancel-acceptance.ps1"
$consistencyScript = Join-Path $PSScriptRoot "run-stage-e-consistency-acceptance.ps1"
$gateScript = Join-Path $PSScriptRoot "run-stage-f-gate-acceptance.ps1"
$reverifyScript = Join-Path $PSScriptRoot "run-stage-backend-reverify-pack.ps1"

$consistencyLatest = Join-Path $root "tmp\stage-e-consistency\latest.json"
$reverifyLatest = Join-Path $root "tmp\stage-backend-reverify\latest.json"
$warningSample = Join-Path $root "tmp\stage-backend-reverify\warning-sample.json"

$originalConsistencyRaw = ""
$hasBackup = $false

try {
  powershell -ExecutionPolicy Bypass -File $historyScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $cancelScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $gateScript -RequireGateF | Out-Null

  if (-not (Test-Path $consistencyLatest)) {
    throw "consistency evidence missing: $consistencyLatest"
  }
  $originalConsistencyRaw = Get-Content -Raw $consistencyLatest
  $hasBackup = $true
  $consistency = $originalConsistencyRaw | ConvertFrom-Json
  $consistency.identity_diff_summary.severity = "warn"
  if ([int]$consistency.identity_diff_summary.missing_count -eq 0) {
    $consistency.identity_diff_summary.missing_count = 1
    $consistency.identity_diff_summary.missing_dimensions = @("trace_id")
  }
  if ($null -ne $consistency.identity_diff_groups.trace_id) {
    $consistency.identity_diff_groups.trace_id.missing_count = [Math]::Max(1, [int]$consistency.identity_diff_groups.trace_id.missing_count)
    $consistency.identity_diff_groups.trace_id.gateway.missing_count = [Math]::Max(1, [int]$consistency.identity_diff_groups.trace_id.gateway.missing_count)
  }
  $consistency.checks.identity_diff_severity = "warn"
  $consistency | ConvertTo-Json -Depth 10 | Set-Content -Path $consistencyLatest -Encoding UTF8

  powershell -ExecutionPolicy Bypass -File $reverifyScript -StrictGate -ReleaseWindow -RequirePass | Out-Null
  if (-not (Test-Path $reverifyLatest)) {
    throw "reverify latest missing: $reverifyLatest"
  }
  Copy-Item -LiteralPath $reverifyLatest -Destination $warningSample -Force
  Write-Output $warningSample
} finally {
  if ($hasBackup) {
    Set-Content -Path $consistencyLatest -Value $originalConsistencyRaw -Encoding UTF8
  }
  powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  powershell -ExecutionPolicy Bypass -File $reverifyScript -StrictGate -ReleaseWindow -RequirePass | Out-Null
}
