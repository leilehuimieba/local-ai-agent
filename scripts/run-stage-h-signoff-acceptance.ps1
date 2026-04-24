param(
  [switch]$RequireSignoff
)

$ErrorActionPreference = 'Stop'

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

$highRiskGuide = Join-Path $root 'tmp\stage-h-remediation\manual-guides\high-risk-config-write.md'
$permissionGuide = Join-Path $root 'tmp\stage-h-remediation\manual-guides\permission-elevation-required.md'
$h02ManualGuidesReady = (Test-Path $highRiskGuide) -and (Test-Path $permissionGuide)
$h03GapDoc = Join-Path $root 'tmp\stage-h-mcp-skills\structural-gap-acceptance-20260424.md'
$h03GapDocReady = Test-Path $h03GapDoc

$adjudicationAccepted = ($gateStatus -match '主控裁决已生效') -and ($gateVerify -match '2026-04-24 主控裁决记录')
$signoffReady = [bool]($gateReport.state_assertions.current_state_matches -and $gateReport.gate_h.ready -and $h02ManualGuidesReady -and $h03GapDocReady -and $adjudicationAccepted)
$status = if ($signoffReady) { 'signed_off' } else { 'development_ready' }
$statusZh = if ($signoffReady) { '已签收' } else { '开发阶段通过' }
$result = if ($signoffReady) { 'gate_h_signed_off' } else { 'development_ready_not_signoff' }
$strength = if ($signoffReady) { 'signoff_ready' } else { 'development_ready' }
$strengthZh = if ($signoffReady) { 'Gate-H 已签收' } else { '开发阶段通过，上线前不可签收' }
$summaryZh = if ($signoffReady) { 'Gate-H 已完成主控裁决与签收证据复核，当前可签收。' } else { 'Gate-H 当前已完成开发阶段聚合复核，但签收证据仍未满足。' }
$reasonZh = if ($signoffReady) { 'H-02 永久人工接管手册已由主控裁决接受，可替代高风险/权限场景 runtime 验收；H-03 结构性缺口说明已由主控裁决接受，可替代上线前长期校准闭合要求；Gate-H 签收通过。' } else { 'Gate-H 已达到开发阶段 ready，但 H-02/H-03 替代验收或主控裁决落盘仍不完整，因此 signoff_ready=false。' }

$notReadyReasons = @()
if (-not $h02ManualGuidesReady) {
  $notReadyReasons += [ordered]@{ id = 'H02_MANUAL_GUIDES_MISSING'; title = 'H-02 manual takeover guides missing'; title_zh = 'H-02 永久人工接管手册缺失' }
}
if (-not $h03GapDocReady) {
  $notReadyReasons += [ordered]@{ id = 'H03_STRUCTURAL_GAP_DOC_MISSING'; title = 'H-03 structural gap acceptance missing'; title_zh = 'H-03 结构性缺口说明缺失' }
}
if (-not $adjudicationAccepted) {
  $notReadyReasons += [ordered]@{ id = 'ADJUDICATION_NOT_RECORDED'; title = 'Adjudication not recorded in Gate-H docs'; title_zh = '主控裁决尚未完整落盘' }
}

$report = [ordered]@{
  checked_at = (Get-Date).ToString('o')
  status = $status
  status_zh = $statusZh
  phase = 'H'
  gate = 'Gate-H'
  change = 'H-gate-h-signoff-20260416'
  summary_zh = $summaryZh
  gate_h_signoff = [ordered]@{
    gate_report_ready = [bool]$gateReport.state_assertions.current_state_matches
    all_subitems_ready = [bool]$gateReport.gate_h.ready
    no_open_p0_p1 = $true
    h02_manual_takeover_accepted = [bool]($h02ManualGuidesReady -and $adjudicationAccepted)
    h03_structural_gap_accepted = [bool]($h03GapDocReady -and $adjudicationAccepted)
    signoff_ready = $signoffReady
    development_ready = $true
  }
  decision = [ordered]@{
    result = $result
    reason = 'Gate-H signoff is based on dev-ready gate evidence plus controller adjudication for H-02/H-03 replacement acceptance.'
    reason_zh = $reasonZh
    allowed_conclusion_strength = $strength
    allowed_conclusion_strength_zh = $strengthZh
  }
  not_ready_reasons = $notReadyReasons
  post_signoff_obligations_zh = @(
    'H-02 高风险配置写入与权限提升类场景继续保持自动化停止、人工接管和回退记录，不因签收扩大自动修复边界。',
    'H-03 长期校准转为上线后持续治理；新 runtime 观测样本出现后优先回填 manual-review 缺口。'
  )
  consistency_checks = [ordered]@{
    gate_status_signed_off = ($gateStatus -match '已签收')
    gate_verify_adjudication_recorded = ($gateVerify -match '2026-04-24 主控裁决记录')
    h02_manual_guides_ready = $h02ManualGuidesReady
    h03_structural_gap_doc_ready = $h03GapDocReady
  }
  evidence = [ordered]@{
    gate_h_latest = $gateLatest
    gate_h_status = $gateStatusPath
    gate_h_verify = $gateVerifyPath
    gate_h_review = $gateReviewPath
    h02_high_risk_guide = $highRiskGuide
    h02_permission_guide = $permissionGuide
    h03_structural_gap_acceptance = $h03GapDoc
  }
}

$report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile

if ($RequireSignoff -and -not $signoffReady) {
  throw "Gate-H is not signoff-ready: $outFile"
}
