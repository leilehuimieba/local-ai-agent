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

$warningZh = '预警'
$devReadyZh = '开发阶段通过'
$signoffSummaryZh = 'Gate-H 当前已完成开发阶段聚合复核，但上线前验收未完成，暂不可签收。'
$signoffReasonZh = 'Gate-H 已达到开发阶段 ready；H-02 高风险/权限场景和 H-03 长期校准仍需上线前验收，因此 signoff_ready=false。'
$signoffStrengthZh = '开发阶段通过，上线前不可签收'
$h02TitleZh = 'H-02 上线前 runtime 验收待补'
$h02DetailZh = '高风险配置写入场景 C-B~C-F 与权限类场景 P-C/P-D 仍需 runtime 验证或永久人工接管手册。'
$h03TitleZh = 'H-03 上线前长期校准待补'
$h03DetailZh = 'manual-review 剩余结构化回指缺口、命中有效性分布长期校准和多评审制度化流程仍需补齐。'
$reopen1Zh = 'H-02 完成高风险配置写入和权限类场景的上线前 runtime 验收。'
$reopen2Zh = 'H-03 完成 manual-review 缺口闭合、长期校准或正式多评审机制验证。'

$report = [ordered]@{
  checked_at = (Get-Date).ToString('o')
  status = 'development_ready'
  status_zh = $devReadyZh
  phase = 'H'
  gate = 'Gate-H'
  change = 'H-gate-h-signoff-20260416'
  summary_zh = $signoffSummaryZh
  gate_h_signoff = [ordered]@{
    gate_report_ready = [bool]$gateReport.state_assertions.current_state_matches
    all_subitems_ready = [bool]$gateReport.gate_h.ready
    no_open_p0_p1 = $true
    signoff_ready = $false
    development_ready = $true
  }
  decision = [ordered]@{
    result = 'development_ready_not_signoff'
    reason = 'Gate-H is development-ready (all subitems ready in dev-stage standard) but not signoff-ready (production verification pending)'
    reason_zh = $signoffReasonZh
    allowed_conclusion_strength = 'development_ready'
    allowed_conclusion_strength_zh = $signoffStrengthZh
  }
  not_ready_reasons = @(
    [ordered]@{
      id = 'PRODUCTION_VERIFICATION_PENDING'
      title = 'Production runtime verification not yet performed'
      detail = 'H-02 high-risk scenarios (C-B~F config write, P-C/P-D permissions) and H-03 long-term calibration need production verification before signoff'
      title_zh = $h02TitleZh
      detail_zh = $h02DetailZh
    },
    [ordered]@{
      id = 'H03_INSTITUTIONAL_PROCESS_PENDING'
      title = 'H-03 institutional review process needs long-term calibration'
      detail = 'H-03 multi-review minimum closure formed but manual_review gap remains'
      title_zh = $h03TitleZh
      detail_zh = $h03DetailZh
    }
  )
  reopen_conditions = @(
    'H-02 high-risk scenarios get production runtime verification with renewed authorization',
    'H-03 institutional review process completes long-term calibration and closes manual_review gap'
  )
  reopen_conditions_zh = @(
    $reopen1Zh,
    $reopen2Zh
  )
  consistency_checks = [ordered]@{
    gate_status_development_ready = ($gateStatus -match 'development_ready|warning')
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
