param(
  [switch]$RequireGateH
)

$ErrorActionPreference = 'Stop'

$decodeZh = {
  param([string]$value)
  if ([string]::IsNullOrWhiteSpace($value)) { return '' }
  $bytes = [Convert]::FromBase64String($value)
  [Text.Encoding]::UTF8.GetString($bytes)
}

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root 'tmp\stage-h-gate'
$outFile = Join-Path $outDir 'latest.json'
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$currentStatePath = Join-Path $root 'docs\11-hermes-rebuild\current-state.md'
$gateStatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\status.md'
$gateVerifyPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\verify.md'
$gateReviewPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\review.md'

$currentState = Get-Content -Raw $currentStatePath
$gateStatus = Get-Content -Raw $gateStatusPath
$gateVerify = Get-Content -Raw $gateVerifyPath

$h01Latest = Join-Path $root 'tmp\stage-h-visibility\latest.json'
$h02Latest = Join-Path $root 'tmp\stage-h-remediation\latest.json'
$h03Latest = Join-Path $root 'tmp\stage-h-mcp-skills\latest.json'
$h04Latest = Join-Path $root 'tmp\stage-h-learning\latest.json'
$h05Latest = Join-Path $root 'tmp\stage-h-memory-routing\latest.json'

$h01StatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\archive\2026-04-15\H-visibility-runtime-20260415\status.md'
$h02StatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-remediation-playbook-20260415\status.md'
$h03StatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-mcp-skills-quality-20260415\status.md'
$h04StatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-learning-mode-browser-20260415\status.md'
$h05StatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-memory-routing-kb-20260415\status.md'

$h01 = Get-Content -Raw $h01Latest | ConvertFrom-Json
$h04 = Get-Content -Raw $h04Latest | ConvertFrom-Json
$h05 = Get-Content -Raw $h05Latest | ConvertFrom-Json

$h01Ready = [bool]$h01.h01_signed_off
$h04Ready = [bool]$h04.h04.ready
$h05Ready = [bool]$h05.h05.ready
$h02Ready = $false
$h03Ready = $false
$signedOffCount = @($h01Ready, $h04Ready, $h05Ready) | Where-Object { $_ } | Measure-Object | Select-Object -ExpandProperty Count
$warningCount = 2
$blockedCount = 0
$gateReady = $false

$warningZh = & $decodeZh '6aKE6K2m'
$signedOffZh = & $decodeZh '5bey562+5pS2'
$gateSummaryZh = & $decodeZh 'R2F0ZS1IIOW9k+WJjeS4uuiBmuWQiOWkjeaguOS4re+8jEgtMDHjgIFILTA044CBSC0wNSDlt7Lnrb7mlLbvvIxILTAy44CBSC0wMyDku43kuLogd2FybmluZ++8jOWboOatpOW9k+WJjeS4jeWPr+mAmui/h+OAgg=='
$gateHSummaryZh = & $decodeZh '5b2T5YmNIEdhdGUtSCDku4XlrozmiJDogZrlkIjlpI3moLjliKTmlq3vvIxyZWFkeT1mYWxzZeOAgg=='
$h02BlockerZh = & $decodeZh '5b2T5YmN5peg5paw55qE5ZCI5qC85Y+X6ZmQ5qC35pys77yM5LuN5L+d5oyB5Ya757uT6KeC5a+f44CC'
$h03BlockerZh = & $decodeZh 'SDAzLTM5IOW3suWujOaIkO+8jOS9huW9k+WJjeacgOW8uue7k+iuuuS7jeWPquaYr+W7uuiuruS4u+aOp+ivhOS8sOaYr+WQpuWIh+S4u+aOqOi/m+OAgg=='
$gateReason1Zh = & $decodeZh 'SC0wMiDlvZPliY3ku43kuLogd2FybmluZ++8jOS4lOayoeacieaWsOeahOWQiOagvOWPl+mZkOagt+acrOOAgg=='
$gateReason2Zh = & $decodeZh 'SC0wMyDlvZPliY3ku43kuLogd2FybmluZ++8jEgwMy0zOSDlrozmiJDkuI3nrYnkuo4gcmVhZHnjgII='
$gateNextStepZh = & $decodeZh '5aaC6ZyA5YaN5qyh5aSN5qC477yM5YWI5pu05pawIEgtMDIg5oiWIEgtMDMg55qE5p2D5aiB54q25oCB77yM5YaN6YeN5paw5omn6KGMIEdhdGUtSCDogZrlkIjohJrmnKzjgII='

$report = [ordered]@{
  checked_at = (Get-Date).ToString('o')
  status = 'warning'
  status_zh = $warningZh
  phase = 'H'
  gate = 'Gate-H'
  change = 'H-gate-h-signoff-20260416'
  summary_zh = $gateSummaryZh
  gate_h = [ordered]@{
    h01_ready = $h01Ready
    h02_ready = $h02Ready
    h03_ready = $h03Ready
    h04_ready = $h04Ready
    h05_ready = $h05Ready
    warning_count = $warningCount
    blocked_count = $blockedCount
    signed_off_count = $signedOffCount
    ready = $gateReady
    summary_zh = $gateHSummaryZh
  }
  state_assertions = [ordered]@{
    current_state_matches = ($currentState -match 'Gate-H') -and ($currentState -match 'H-gate-h-signoff-20260416')
    gate_status_warning = ($gateStatus -match 'warning')
    gate_not_signoff = $true
    gate_verify_warning = ($gateVerify -match 'warning')
  }
  subitems = [ordered]@{
    h01 = [ordered]@{
      status = 'signed_off'
      status_zh = $signedOffZh
      ready = $h01Ready
      status_doc = $h01StatusPath
      evidence_ref = $h01Latest
      blocker_reason = ''
      blocker_reason_zh = ''
    }
    h02 = [ordered]@{
      status = 'warning'
      status_zh = $warningZh
      ready = $h02Ready
      status_doc = $h02StatusPath
      evidence_ref = $h02Latest
      blocker_reason = 'no_new_qualified_limited_sample'
      blocker_reason_zh = $h02BlockerZh
    }
    h03 = [ordered]@{
      status = 'warning'
      status_zh = $warningZh
      ready = $h03Ready
      status_doc = $h03StatusPath
      evidence_ref = $h03Latest
      blocker_reason = 'h03_39_done_but_not_ready'
      blocker_reason_zh = $h03BlockerZh
    }
    h04 = [ordered]@{
      status = 'signed_off'
      status_zh = $signedOffZh
      ready = $h04Ready
      status_doc = $h04StatusPath
      evidence_ref = $h04Latest
      blocker_reason = ''
      blocker_reason_zh = ''
    }
    h05 = [ordered]@{
      status = 'signed_off'
      status_zh = $signedOffZh
      ready = $h05Ready
      status_doc = $h05StatusPath
      evidence_ref = $h05Latest
      blocker_reason = ''
      blocker_reason_zh = ''
    }
  }
  blocking_reasons = @(
    'H-02 remains warning without new qualified limited sample',
    'H-03 remains warning and H03-39 is not equal to ready'
  )
  blocking_reasons_zh = @(
    $gateReason1Zh,
    $gateReason2Zh
  )
  next_step_zh = $gateNextStepZh
  evidence = [ordered]@{
    current_state = $currentStatePath
    gate_status = $gateStatusPath
    gate_verify = $gateVerifyPath
    gate_review = $gateReviewPath
    h01_latest = $h01Latest
    h02_latest = $h02Latest
    h03_latest = $h03Latest
    h04_latest = $h04Latest
    h05_latest = $h05Latest
  }
}

$report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequireGateH -and -not $gateReady) {
  throw "Gate-H is not ready: $outFile"
}
