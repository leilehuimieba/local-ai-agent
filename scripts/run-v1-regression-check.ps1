$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\v1-regression-check"
$outFile = Join-Path $outDir "latest.json"

function New-CheckResult {
  param($Name, $Status, $Detail, $Evidence)
  [ordered]@{
    name = $Name
    status = $Status
    detail = $Detail
    evidence = $Evidence
  }
}

function Get-OutputTailText {
  param($Output)
  (($Output | Select-Object -Last 20 | ForEach-Object { [string]$_ }) -join "`n").Trim()
}

function Invoke-CommandCheck {
  param($Name, $Workdir, $FilePath, $Arguments)
  $stdout = New-TemporaryFile
  $stderr = New-TemporaryFile
  try {
    $proc = Start-Process -FilePath $FilePath -ArgumentList $Arguments -WorkingDirectory $Workdir -NoNewWindow -Wait -PassThru -RedirectStandardOutput $stdout.FullName -RedirectStandardError $stderr.FullName
    $output = @()
    if (Test-Path $stdout.FullName) { $output += Get-Content -Path $stdout.FullName }
    if (Test-Path $stderr.FullName) { $output += Get-Content -Path $stderr.FullName }
    $exitCode = $proc.ExitCode
  } catch {
    $output = @($_.Exception.Message)
    $exitCode = 1
  } finally {
    if (Test-Path $stdout.FullName) { Remove-Item -LiteralPath $stdout.FullName -Force }
    if (Test-Path $stderr.FullName) { Remove-Item -LiteralPath $stderr.FullName -Force }
  }
  $status = if ($exitCode -eq 0) { "passed" } else { "failed" }
  $detail = if ($exitCode -eq 0) { "命令执行通过" } else { "命令执行失败" }
  New-CheckResult $Name $status $detail @{
    workdir = $Workdir
    command = "$FilePath $($Arguments -join ' ')".Trim()
    exit_code = $exitCode
    output_tail = Get-OutputTailText $output
  }
}

function Test-JsonSample {
  param($Name, $Path, $PassStatuses, $WarnStatuses)
  if (-not (Test-Path $Path)) {
    return New-CheckResult $Name "failed" "样本文件缺失" @{ path = $Path }
  }
  $data = Get-Content -Path $Path -Raw | ConvertFrom-Json
  $sampleStatus = [string]$data.status
  if ($PassStatuses -contains $sampleStatus) {
    return New-CheckResult $Name "passed" "样本状态正常：$sampleStatus" @{ path = $Path; sample_status = $sampleStatus }
  }
  if ($WarnStatuses -contains $sampleStatus) {
    return New-CheckResult $Name "warn" "样本为非阻断告警：$sampleStatus" @{ path = $Path; sample_status = $sampleStatus; notes = $data.notes }
  }
  New-CheckResult $Name "failed" "样本状态异常：$sampleStatus" @{ path = $Path; sample_status = $sampleStatus; notes = $data.notes }
}

function Test-TextPatterns {
  param($Name, $Path, $Patterns)
  if (-not (Test-Path $Path)) {
    return New-CheckResult $Name "failed" "检查文件缺失" @{ path = $Path }
  }
  $content = Get-Content -Path $Path -Raw
  $missing = @($Patterns | Where-Object { -not $content.Contains($_) })
  $status = if ($missing.Count -eq 0) { "passed" } else { "failed" }
  $detail = if ($missing.Count -eq 0) { "文档口径一致" } else { "存在缺失口径" }
  New-CheckResult $Name $status $detail @{ path = $Path; missing_patterns = $missing }
}

function Wait-RuntimeReady {
  param($BaseUrl)
  for ($i = 0; $i -lt 25; $i++) {
    try { Invoke-RestMethod "$BaseUrl/health" -TimeoutSec 1 | Out-Null; return $true } catch {}
    Start-Sleep -Milliseconds 200
  }
  $false
}

function Test-RuntimeConfirmationSequence {
  param($Root)
  $port = 8899
  $exe = Join-Path $Root "target\debug\runtime-host.exe"
  if (-not (Test-Path $exe)) { return New-CheckResult "runtime_confirmation_sequence" "failed" "runtime-host 可执行文件缺失" @{ path = $exe } }
  $proc = $null
  try {
    $env:LOCAL_AGENT_RUNTIME_PORT = "$port"
    $proc = Start-Process -FilePath $exe -WorkingDirectory $Root -PassThru -WindowStyle Hidden
    if (-not (Wait-RuntimeReady "http://127.0.0.1:$port")) { return New-CheckResult "runtime_confirmation_sequence" "failed" "runtime-host 未在预期时间内就绪" @{ port = $port; exe = $exe } }
    $body = @{ request_id = "reg-confirm-req"; run_id = "reg-confirm-run"; session_id = "reg-confirm-session"; trace_id = "reg-confirm-trace"; user_input = "delete: docs/README.md"; mode = "standard"; model_ref = @{ provider_id = "local"; model_id = "test"; display_name = "test" }; provider_ref = @{ provider_id = "local"; display_name = "local"; base_url = ""; chat_completions_path = ""; models_path = ""; api_key = "" }; workspace_ref = @{ workspace_id = "default"; name = "本地智能体"; root_path = $Root; is_active = $true }; context_hints = @{}; confirmation_decision = $null } | ConvertTo-Json -Depth 8
    $resp = Invoke-RestMethod "http://127.0.0.1:$port/v1/runtime/run" -Method Post -ContentType "application/json" -Body $body
    $events = @($resp.events | Where-Object { $_.event_type -in @("plan_ready", "memory_recalled", "confirmation_required") })
    $actual = @($events | ForEach-Object { [string]$_.event_type })
    $passed = $resp.result.status -eq "awaiting_confirmation" -and $resp.confirmation_request -and (($actual -join " -> ") -eq "plan_ready -> memory_recalled -> confirmation_required")
    New-CheckResult "runtime_confirmation_sequence" $(if ($passed) { "passed" } else { "failed" }) $(if ($passed) { "确认链事件时序正确" } else { "确认链事件时序异常" }) @{ status = $resp.result.status; confirmation_request = [bool]$resp.confirmation_request; event_types = $actual; events = @($events | Select-Object sequence, event_type, stage, summary, @{Name="memory_digest_present";Expression={ [bool]$_.metadata.memory_digest }}, @{Name="memory_action";Expression={ $_.metadata.memory_action }}, @{Name="governance_status";Expression={ $_.metadata.governance_status }}) }
  } catch {
    New-CheckResult "runtime_confirmation_sequence" "failed" "确认链动态验收执行失败" @{ error = $_.Exception.Message; port = $port; exe = $exe }
  } finally {
    if ($proc -and -not $proc.HasExited) { Stop-Process -Id $proc.Id -Force }
    Remove-Item Env:LOCAL_AGENT_RUNTIME_PORT -ErrorAction SilentlyContinue
  }
}

function Get-OverallStatus {
  param($Results)
  if ($Results.status -contains "failed") {
    return "failed"
  }
  if ($Results.status -contains "warn") {
    return "passed_with_warnings"
  }
  "passed"
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$results = @()
$results += Invoke-CommandCheck "cargo_build" $root "cargo" @("build")
$results += Invoke-CommandCheck "go_build" (Join-Path $root "gateway") "go" @("build", "./...")
$results += Invoke-CommandCheck "frontend_build" (Join-Path $root "frontend") "cmd.exe" @("/c", "npm", "run", "build")

$results += Test-JsonSample "sample_launch_local_program" (Join-Path $root "tmp\v1-capability-acceptance\launch-local-program-sample.json") @("passed") @()
$results += Test-JsonSample "sample_controlled_install" (Join-Path $root "tmp\v1-capability-acceptance\controlled-install-verify-sample.json") @("passed") @()
$results += Test-JsonSample "sample_winget_install" (Join-Path $root "tmp\v1-capability-acceptance\winget-system-install-verify-sample.json") @("passed") @()
$results += Test-JsonSample "sample_choco_install" (Join-Path $root "tmp\v1-capability-acceptance\choco-system-install-verify-sample.json") @("passed") @("failed")

$results += Test-TextPatterns "doc_freeze_scope" (Join-Path $root "docs\00-charter\需求冻结稿_V1.md") @(
  "5. 启动本地程序",
  "可控安装与验证能力"
)
$results += Test-TextPatterns "doc_completion_status" (Join-Path $root "docs\archive\completed-v1-20260410\需求文档对照完成度清单_V1.md") @(
  "第一版产品落地门槛：",
  '`V1` 冻结需求实现：',
  "6. 启动本地程序：",
  "7. 可控安装与验证能力："
)
$results += Test-TextPatterns "doc_product_acceptance" (Join-Path $root "docs\archive\completed-v1-20260410\产品级总体验收文档_V1.md") @(
  "### 3.2 当前正式结论",
  "### 4.13 启动本地程序",
  "### 4.14 可控安装与验证能力",
  "tmp/v1-capability-acceptance/winget-system-install-verify-sample.json",
  "sample-choco-system-install-verify.json"
)
$results += Test-TextPatterns "runtime_memory_recalled_formalized" (Join-Path $root "crates\runtime-core\src\events.rs") @(
  "with_runtime_memory_recall_event",
  "memory_recalled",
  "memory_action",
  "governance_status",
  "source_event_type",
  "source_artifact_path"
)
$results += Test-RuntimeConfirmationSequence $root
$results += Test-TextPatterns "memory_governance_contract_go" (Join-Path $root "gateway\internal\contracts\contracts.go") @(
  "memory_kind",
  "governance_status",
  "memory_action",
  "source_event_type",
  "source_artifact_path"
)
$results += Test-TextPatterns "memory_governance_contract_ts" (Join-Path $root "frontend\src\shared\contracts.ts") @(
  "memory_kind",
  "governance_status",
  "memory_action",
  "source_event_type",
  "source_artifact_path"
)
$results += Test-TextPatterns "runtime_contract_resume_fields_rust" (Join-Path $root "crates\runtime-core\src\contracts.rs") @(
  "resume_from_checkpoint_id",
  "resume_strategy",
  "checkpoint_id",
  "resumable"
)
$results += Test-TextPatterns "runtime_contract_resume_fields_go" (Join-Path $root "gateway\internal\contracts\contracts.go") @(
  "resume_from_checkpoint_id",
  "resume_strategy",
  "checkpoint_id",
  "resumable"
)
$results += Test-TextPatterns "runtime_contract_resume_fields_ts" (Join-Path $root "frontend\src\shared\contracts.ts") @(
  "resume_from_checkpoint_id",
  "resume_strategy",
  "checkpoint_id",
  "resumable"
)

$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  overall_status = Get-OverallStatus $results
  result_count = $results.Count
  results = $results
}

$report | ConvertTo-Json -Depth 8 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
