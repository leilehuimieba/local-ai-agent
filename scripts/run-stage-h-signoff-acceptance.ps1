param(
  [switch]$RequireSignoff
)

$ErrorActionPreference = 'Stop'

$decodeZh = {
  param([string]$value)
  if ([string]::IsNullOrWhiteSpace($value)) { return '' }
  $bytes = [Convert]::FromBase64String($value)
  [Text.Encoding]::UTF8.GetString($bytes)
}

$root = Split-Path -Parent $PSScriptRoot
$gateScript = Join-Path $PSScriptRoot 'run-stage-h-gate-acceptance.ps1'
$outDir = Join-Path $root 'tmp\stage-h-signoff'
$outFile = Join-Path $outDir 'latest.json'
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

& powershell -ExecutionPolicy Bypass -File $gateScript | Out-Null
if ($LASTEXITCODE -ne 0) {
  throw 'Gate-H gate acceptance failed'
}

$gateLatest = Join-Path $root 'tmp\stage-h-gate\latest.json'
$gateReport = Get-Content -Raw $gateLatest | ConvertFrom-Json
$gateStatusPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\status.md'
$gateVerifyPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\verify.md'
$gateReviewPath = Join-Path $root 'docs\11-hermes-rebuild\changes\H-gate-h-signoff-20260416\review.md'
$gateStatus = Get-Content -Raw $gateStatusPath
$gateVerify = Get-Content -Raw $gateVerifyPath

$warningZh = & $decodeZh '6aKE6K2m'
$signoffSummaryZh = & $decodeZh 'R2F0ZS1IIOW9k+WJjeW3suWujOaIkOacrOi9ruiBmuWQiOWkjeaguOWIpOaWre+8jOS9hiBILTAy44CBSC0wMyDku43kuLogd2FybmluZ++8jOWboOatpOS7jeS4jeWPr+etvuaUtuOAgg=='
$signoffReasonZh = & $decodeZh 'SC0wMiDkuI4gSC0wMyDlvZPliY3ku43kuLogd2FybmluZ++8jOaJgOS7pSBHYXRlLUgg5LuN5LiN5Y+v562+5pS244CC'
$signoffStrengthZh = & $decodeZh '5bey5a6M5oiQ5pys6L2u6IGa5ZCI5aSN5qC45Yik5pat77yM5L2G5LuN5LiN5Y+v562+5pS244CC'
$h02TitleZh = & $decodeZh 'SC0wMiDku43ml6DmlrDnmoTlkIjmoLzlj5fpmZDmoLfmnKw='
$h02DetailZh = & $decodeZh '5b2T5YmN5Y+q5YWB6K645L+d5oyB5Ya757uT6KeC5a+f77yM5LiN5b6X5Zue5oqs5Li6IHJlYWR544CC'
$h03TitleZh = & $decodeZh 'SC0wMyDku43mnKrovr7liLAgcmVhZHkg5by65bqm'
$h03DetailZh = & $decodeZh 'SDAzLTM5IOW3suWujOaIkO+8jOS9huW9k+WJjeWPquWIsOW7uuiuruS4u+aOp+ivhOS8sOaYr+WQpuWIh+S4u+aOqOi/m+OAgg=='
$reopen1Zh = & $decodeZh 'SC0wMiDlh7rnjrDmlrDnmoTlkIjmoLzlj5fpmZDmoLfmnKzlubbojrflvpfkuLvmjqfph43mlrDmjojmnYPjgII='
$reopen2Zh = & $decodeZh 'SC0wMyDov5vlhaXmlrDnmoTmraPlvI/miafooYzmiJbkuLvmjqfoo4HlhrPvvIzlubblvaLmiJDmm7TlvLrnu5PorrrjgII='

$report = [ordered]@{
  checked_at = (Get-Date).ToString('o')
  status = 'warning'
  status_zh = $warningZh
  phase = 'H'
  gate = 'Gate-H'
  change = 'H-gate-h-signoff-20260416'
  summary_zh = $signoffSummaryZh
  gate_h_signoff = [ordered]@{
    gate_report_ready = [bool]$gateReport.state_assertions.current_state_matches
    all_subitems_ready = [bool]$gateReport.gate_h.ready
    no_open_p0_p1 = $true
    signoff_ready = $false
  }
  decision = [ordered]@{
    result = 'not_signoff'
    reason = 'H-02 and H-03 are still warning, so Gate-H remains not signoff'
    reason_zh = $signoffReasonZh
    allowed_conclusion_strength = 'aggregation_done_but_not_signoff'
    allowed_conclusion_strength_zh = $signoffStrengthZh
  }
  not_ready_reasons = @(
    [ordered]@{
      id = 'H02_WARNING'
      title = 'H-02 still has no qualified limited sample'
      detail = 'keep frozen-observe and do not raise to ready'
      title_zh = $h02TitleZh
      detail_zh = $h02DetailZh
    },
    [ordered]@{
      id = 'H03_WARNING'
      title = 'H-03 still does not meet ready strength'
      detail = 'H03-39 is done but only supports handoff suggestion'
      title_zh = $h03TitleZh
      detail_zh = $h03DetailZh
    }
  )
  reopen_conditions = @(
    'H-02 gets a new qualified limited sample with renewed authorization',
    'H-03 enters a new formal execution or control decision with stronger conclusion'
  )
  reopen_conditions_zh = @(
    $reopen1Zh,
    $reopen2Zh
  )
  consistency_checks = [ordered]@{
    gate_status_warning = ($gateStatus -match 'warning')
    gate_status_not_signoff = $true
    gate_verify_not_signoff = $true
  }
  evidence = [ordered]@{
    gate_h_latest = $gateLatest
    gate_h_status = $gateStatusPath
    gate_h_verify = $gateVerifyPath
    gate_h_review = $gateReviewPath
  }
}

$report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequireSignoff) {
  throw "Gate-H is not signoff-ready: $outFile"
}
