param(
  [switch]$RefreshEvidence,
  [int]$Rounds = 3,
  [switch]$RequirePass,
  [string]$OutPath = ""
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$historyScript = Join-Path $PSScriptRoot "run-stage-e-cli-history-acceptance.ps1"
$cancelScript = Join-Path $PSScriptRoot "run-stage-e-cli-cancel-acceptance.ps1"
$consistencyScript = Join-Path $PSScriptRoot "run-stage-e-consistency-acceptance.ps1"
$gateFScript = Join-Path $PSScriptRoot "run-stage-f-gate-acceptance.ps1"
$gateG01Script = Join-Path $PSScriptRoot "run-stage-g-gate-acceptance.ps1"
$g02Script = Join-Path $PSScriptRoot "run-stage-g-warning-governance.ps1"

$outDir = Join-Path $root "tmp\stage-g-regression"
if ([string]::IsNullOrWhiteSpace($OutPath)) {
  $OutPath = Join-Path $outDir "latest.json"
}
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$roundsSafe = [Math]::Max(1, $Rounds)
$trackingSeed = (Get-Date).ToString("yyyyMMddHHmmss")
$dueAt = (Get-Date).AddDays(1).ToString("yyyy-MM-ddTHH:mm:sszzz")
$refreshLog = @()

if ($RefreshEvidence) {
  & powershell -ExecutionPolicy Bypass -File $historyScript | Out-Null
  $refreshLog += "E-01 history refreshed"
  & powershell -ExecutionPolicy Bypass -File $cancelScript | Out-Null
  $refreshLog += "E-01 cancel refreshed"
  & powershell -ExecutionPolicy Bypass -File $consistencyScript | Out-Null
  $refreshLog += "E-04 consistency refreshed"
}

$results = @()
$failedSamples = @()
$passCount = 0
$totalRuns = 0

for ($i = 1; $i -le $roundsSafe; $i++) {
  $startAt = Get-Date

  & powershell -ExecutionPolicy Bypass -File $gateFScript -RequireGateF | Out-Null
  & powershell -ExecutionPolicy Bypass -File $gateG01Script $(if ($RefreshEvidence -and $i -eq 1) { "-RefreshEvidence" }) -WarningAuditExecutor "g-regression" -WarningAuditTrackingId ("G03-G01-{0}-{1:D2}" -f $trackingSeed, $i) -WarningAuditDueAt $dueAt -RequirePass | Out-Null
  & powershell -ExecutionPolicy Bypass -File $g02Script $(if ($RefreshEvidence -and $i -eq 1) { "-RefreshEvidence" }) -WarningAuditExecutor "g-regression" -WarningAuditTrackingId ("G03-G02-{0}-{1:D2}" -f $trackingSeed, $i) -WarningAuditDueAt $dueAt -RequirePass | Out-Null

  if ($RefreshEvidence -and $i -eq 1) {
    $refreshLog += "G-01 gate refreshed"
    $refreshLog += "G-02 governance refreshed"
  }

  $history = Get-Content -Raw (Join-Path $root "tmp\stage-e-cli-history\latest.json") | ConvertFrom-Json
  $cancel = Get-Content -Raw (Join-Path $root "tmp\stage-e-cli-cancel\latest.json") | ConvertFrom-Json
  $consistency = Get-Content -Raw (Join-Path $root "tmp\stage-e-consistency\latest.json") | ConvertFrom-Json
  $fGate = Get-Content -Raw (Join-Path $root "tmp\stage-f-gate\latest.json") | ConvertFrom-Json
  $gGate = Get-Content -Raw (Join-Path $root "tmp\stage-g-gate\latest.json") | ConvertFrom-Json
  $gFreshness = Get-Content -Raw (Join-Path $root "tmp\stage-g-evidence-freshness\latest.json") | ConvertFrom-Json
  $gOps = Get-Content -Raw (Join-Path $root "tmp\stage-g-ops\latest.json") | ConvertFrom-Json

  $eReady = [string]$history.status -eq "passed" -and
    [string]$cancel.status -eq "passed" -and
    [string]$consistency.status -eq "passed"
  $fReady = [string]$fGate.status -eq "passed" -and [bool]$fGate.gate_f.ready
  $gReady = [string]$gGate.status -eq "passed" -and
    [bool]$gGate.gate_g.ready -and
    [string]$gFreshness.status -eq "passed" -and
    [string]$gOps.status -eq "passed" -and
    [bool]$gOps.checks.governance_ready
  $samplePass = $eReady -and $fReady -and $gReady

  $route = @()
  if (-not $eReady) { $route += "E" }
  if (-not $fReady) { $route += "F" }
  if (-not $gReady) { $route += "G" }
  if (@($route).Count -eq 0) { $route = @("none") }

  $totalRuns += 1
  if ($samplePass) {
    $passCount += 1
  } else {
    $failedSamples += [ordered]@{
      round = $i
      route = @($route)
      history_status = [string]$history.status
      cancel_status = [string]$cancel.status
      consistency_status = [string]$consistency.status
      gate_f_status = [string]$fGate.status
      gate_g01_status = [string]$gGate.status
      freshness_status = [string]$gFreshness.status
      g02_status = [string]$gOps.status
      gate_f_ready = [bool]$fGate.gate_f.ready
      gate_g_ready = [bool]$gGate.gate_g.ready
      governance_ready = [bool]$gOps.checks.governance_ready
    }
  }

  $results += [ordered]@{
    round = $i
    started_at = $startAt.ToString("o")
    ended_at = (Get-Date).ToString("o")
    passed = $samplePass
    route = @($route)
    evidence = [ordered]@{
      e01_history = Join-Path $root "tmp\stage-e-cli-history\latest.json"
      e01_cancel = Join-Path $root "tmp\stage-e-cli-cancel\latest.json"
      e04_consistency = Join-Path $root "tmp\stage-e-consistency\latest.json"
      f_gate = Join-Path $root "tmp\stage-f-gate\latest.json"
      g_gate = Join-Path $root "tmp\stage-g-gate\latest.json"
      g_freshness = Join-Path $root "tmp\stage-g-evidence-freshness\latest.json"
      g_ops = Join-Path $root "tmp\stage-g-ops\latest.json"
    }
  }
}

$passRate = if ($totalRuns -eq 0) { 0 } else { [Math]::Round(($passCount * 100.0) / $totalRuns, 2) }
$ready = $passRate -ge 95 -and @($failedSamples).Count -eq 0

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($ready) { "passed" } else { "failed" })
  mode = $(if ($RefreshEvidence) { "refresh_then_verify" } else { "snapshot_verify" })
  summary = [ordered]@{
    rounds = $roundsSafe
    total_runs = $totalRuns
    pass_count = $passCount
    pass_rate = $passRate
    threshold = 95
    ready = $ready
  }
  refresh_log = @($refreshLog)
  failed_samples = @($failedSamples)
  runs = @($results)
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $OutPath -Encoding UTF8
Write-Output $OutPath

if ($RequirePass -and -not $ready) {
  throw "G-03 regression baseline failed: $OutPath"
}
