$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$gateway = "http://127.0.0.1:8897"
$sessionBase = "session-mainline-acceptance"
$workspace = @{
  workspace_id = "main"
  name = "本地智能体"
  root_path = "D:/newwork/本地智能体"
  is_active = $true
}
$model = @{
  provider_id = "scnet"
  model_id = "MiniMax-M2.5"
  display_name = "MiniMax M2.5"
}
$samples = @(
  @{ id = "A"; title = "项目说明问答"; user_input = "这个项目现在做到什么程度了" }
  @{ id = "B"; title = "会话延续问答"; user_input = "继续说刚才主链路还差什么" }
  @{ id = "C"; title = "文件读取任务"; user_input = "read: docs/README.md" }
  @{ id = "D"; title = "命令执行任务"; user_input = "cmd: cargo build" }
  @{ id = "E"; title = "风险确认任务"; user_input = "delete: logs" }
  @{ id = "F"; title = "记忆召回可见性"; user_input = "继续总结我们当前项目主口径" }
  @{ id = "G"; title = "知识召回优先级"; user_input = "knowledge: 项目主口径 README 06-development" }
)

function Invoke-ChatRun($sample) {
  $sessionId = "$sessionBase-$($sample.id.ToLower())"
  $body = @{
    session_id = $sessionId
    user_input = $sample.user_input
    mode = "standard"
    model = $model
    workspace = $workspace
  } | ConvertTo-Json -Depth 5
  Invoke-RestMethod -Uri "$gateway/api/v1/chat/run" -Method Post -ContentType "application/json; charset=utf-8" -Body $body
}

function Wait-RunResult($runId) {
  for ($i = 0; $i -lt 60; $i++) {
    $logs = Invoke-RestMethod -Uri "$gateway/api/v1/logs"
    $items = @($logs.items | Where-Object { $_.run_id -eq $runId })
    $finished = $items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" -or $_.event_type -eq "confirmation_required" } | Select-Object -Last 1
    if ($finished) {
      return $items
    }
    Start-Sleep -Milliseconds 500
  }
  throw "run timeout: $runId"
}

function Format-SampleResult($sample, $runId, $items) {
  $chain = @($items | Select-Object -ExpandProperty event_type) -join " -> "
  $finish = $items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" -or $_.event_type -eq "confirmation_required" } | Select-Object -Last 1
  $finalAnswer = if ($finish.final_answer) { $finish.final_answer } else { $finish.detail }
  $resultSummary = if ($finish.result_summary) { $finish.result_summary } else { $finish.summary }
  $completionStatus = if ($finish.completion_status) { $finish.completion_status } else { $finish.event_type }
  [ordered]@{
    sample_id = $sample.id
    title = $sample.title
    request = $sample.user_input
    run_id = $runId
    event_chain = $chain
    final_answer = $finalAnswer
    result_summary = $resultSummary
    memory_digest = $finish.context_snapshot.memory_digest
    knowledge_digest = $finish.context_snapshot.knowledge_digest
    completion_status = $completionStatus
    error_code = $finish.metadata.error_code
  }
}

$results = @()
foreach ($sample in $samples) {
  $accepted = Invoke-ChatRun $sample
  $items = Wait-RunResult $accepted.run_id
  $results += Format-SampleResult $sample $accepted.run_id $items
}

$output = Join-Path $root "tmp\mainline-acceptance-results.json"
$results | ConvertTo-Json -Depth 6 | Set-Content -Path $output -Encoding UTF8
Write-Output $output
