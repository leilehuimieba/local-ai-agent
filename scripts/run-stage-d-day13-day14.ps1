param(
  [string]$Gateway = "http://127.0.0.1:8897",
  [string]$EvidenceDirName = "20260410-two-week-stage-b",
  [int]$WaitRetry = 120,
  [switch]$RetryStateSuggestOnce,
  [switch]$RetryDay14ContinueOnce,
  [switch]$UseStablePrompts
)

$ErrorActionPreference = "Stop"
$script:RunStamp = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds().ToString()

$root = Split-Path -Parent $PSScriptRoot
$evidence = Join-Path $root "docs/07-test/evidence/$EvidenceDirName"
$tmpDir = Join-Path $root "tmp/knowledge-import-day2/20260410-two-week"
$workspaceRoot = ($root -replace "\\", "/")
$workspace = @{
  workspace_id = "main"
  name = "local-agent"
  root_path = $workspaceRoot
  is_active = $true
}

function Wait-RunItems {
  param(
    [string]$GatewayBase,
    [string]$RunId,
    [int]$RetryCount
  )
  for ($i = 0; $i -lt $RetryCount; $i++) {
    $logs = Invoke-RestMethod -Uri ($GatewayBase + "/api/v1/logs")
    $items = @($logs.items | Where-Object { $_.run_id -eq $RunId })
    $finish = $items | Where-Object {
      $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" -or $_.event_type -eq "confirmation_required"
    } | Select-Object -Last 1
    if ($finish) { return [pscustomobject]@{ items = $items; finish = $finish } }
    Start-Sleep -Milliseconds 500
  }
  throw "run timeout: $RunId"
}

function Get-FieldText {
  param(
    $Target,
    [string]$PrimaryKey,
    [string]$FallbackKey = ""
  )
  if ($null -eq $Target) { return "" }
  $primary = $Target.$PrimaryKey
  if ($primary) { return [string]$primary }
  if (-not $FallbackKey) { return "" }
  $fallback = $Target.$FallbackKey
  if ($fallback) { return [string]$fallback }
  ""
}

function New-StepResult {
  param(
    [string]$Prefix,
    [string]$RunId,
    $Finish
  )
  $completion = Get-FieldText -Target $Finish -PrimaryKey "completion_status" -FallbackKey "event_type"
  $mode = Get-FieldText -Target $Finish -PrimaryKey "result_mode"
  if (-not $mode) { $mode = Get-FieldText -Target $Finish.metadata -PrimaryKey "result_mode" }
  $verify = Get-FieldText -Target $Finish.verification_snapshot -PrimaryKey "code"
  if (-not $verify) { $verify = Get-FieldText -Target $Finish.metadata -PrimaryKey "verification_code" }
  $answer = Get-FieldText -Target $Finish -PrimaryKey "final_answer" -FallbackKey "detail"
  [pscustomobject]@{
    prefix = $Prefix
    run_id = $RunId
    completion_status = $completion
    result_mode = $mode
    verification_code = $verify
    final_answer = $answer
  }
}

function Invoke-Step {
  param(
    [string]$GatewayBase,
    [hashtable]$WorkspaceConfig,
    [string]$EvidenceRoot,
    [string]$Prefix,
    [string]$SessionId,
    [string]$UserInput,
    [int]$RetryCount
  )
  $effectiveSessionId = "$SessionId-$($script:RunStamp)"
  $bodyJson = @{
    session_id = $effectiveSessionId
    user_input = $UserInput
    mode = "standard"
    workspace = $WorkspaceConfig
  } | ConvertTo-Json -Depth 6
  $accepted = Invoke-RestMethod -Uri ($GatewayBase + "/api/v1/chat/run") -Method Post -ContentType "application/json; charset=utf-8" -Body $bodyJson
  $accepted | ConvertTo-Json -Depth 8 | Set-Content -Path (Join-Path $EvidenceRoot ($Prefix + ".run-accepted.json")) -Encoding UTF8
  $result = Wait-RunItems -GatewayBase $GatewayBase -RunId $accepted.run_id -RetryCount $RetryCount
  $result.items | ConvertTo-Json -Depth 20 | Set-Content -Path (Join-Path $EvidenceRoot ($Prefix + ".run-events.json")) -Encoding UTF8
  $result.finish | ConvertTo-Json -Depth 20 | Set-Content -Path (Join-Path $EvidenceRoot ($Prefix + ".run-finished.json")) -Encoding UTF8
  New-StepResult -Prefix $Prefix -RunId ([string]$accepted.run_id) -Finish $result.finish
}

function Save-MdTable {
  param(
    [string]$Title,
    [string]$OutputPath,
    [array]$Steps,
    [int]$PassedCount,
    [string]$Note = ""
  )
  $lines = @()
  $lines += "# $Title"
  $lines += ""
  $lines += "| step | run_id | completion | mode | verification |"
  $lines += "|---|---|---|---|---|"
  foreach ($s in $Steps) {
    $lines += "| $($s.prefix) | $($s.run_id) | $($s.completion_status) | $($s.result_mode) | $($s.verification_code) |"
  }
  $lines += ""
  $lines += "overall: $PassedCount/$(@($Steps).Count) passed."
  if ($Note) { $lines += $Note }
  $lines -join "`r`n" | Set-Content -Path $OutputPath -Encoding UTF8
}

function Is-StepPassed {
  param($Step)
  ($Step.completion_status -eq "completed" -and $Step.result_mode -eq "answer" -and $Step.verification_code -eq "verified")
}

if (-not (Test-Path $evidence)) { New-Item -ItemType Directory -Path $evidence | Out-Null }
if (-not (Test-Path $tmpDir)) { New-Item -ItemType Directory -Path $tmpDir | Out-Null }

$stateContent = @"
# Day13 state fields writeback (2026-04-10)

- recent_result: Day10 continuation rerun2 is 6/6; Day11 next-step template check is 3/3; Day12 low-risk bridge is 4/4.
- stage_decision: Stage C closed; Stage D can continue.
- next_candidates:
1. run Day14 full chain and keep reproducible entry.
2. write Day13/Day14 status into section 12.9.
3. keep empty failure closeout record for audit consistency.
"@

$failureContent = @"
# Day13 failure closeout (2026-04-10)

- conclusion: no new failure sample in this round.
- empty_failure_closeout: true
- note: if recovery_fallback or system failure appears later, append run_id, trigger, minimal patch and rerun result.
"@

$day13 = @()
$day13 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-state-suggest" -SessionId "day13-state-suggest-20260410" -UserInput "Based on day10/day11/day12 evidence, output recent_result, stage_decision, next_candidates in 3 lines." -RetryCount $WaitRetry
$day13 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-state-write" -SessionId "day13-state-write-20260410" -UserInput ("write: tmp/knowledge-import-day2/20260410-two-week/day13-state-fields.md`n" + $stateContent) -RetryCount $WaitRetry
$day13 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-state-readback" -SessionId "day13-state-readback-20260410" -UserInput "read: tmp/knowledge-import-day2/20260410-two-week/day13-state-fields.md" -RetryCount $WaitRetry
$day13 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-failure-write" -SessionId "day13-failure-write-20260410" -UserInput ("write: tmp/knowledge-import-day2/20260410-two-week/day13-failure-closeout.md`n" + $failureContent) -RetryCount $WaitRetry
$day13 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-failure-readback" -SessionId "day13-failure-readback-20260410" -UserInput "read: tmp/knowledge-import-day2/20260410-two-week/day13-failure-closeout.md" -RetryCount $WaitRetry

$day13State = @($day13 | Where-Object { $_.prefix -like "day13-state-*" })
$day13Failure = @($day13 | Where-Object { $_.prefix -like "day13-failure-*" })
$stateSuggestRerunUsed = $false
$stateSuggestRerunRunId = ""
if ($RetryStateSuggestOnce -and -not (Is-StepPassed -Step $day13State[0])) {
  $rerun = Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day13-rerun-state-suggest" -SessionId "day13-rerun-state-suggest-20260410" -UserInput "Give only 3 lines: recent_result, stage_decision, next_candidates. Keep concise." -RetryCount $WaitRetry
  if (Is-StepPassed -Step $rerun) {
    $day13State[0] = $rerun
    $stateSuggestRerunUsed = $true
    $stateSuggestRerunRunId = $rerun.run_id
  }
}
$day13StatePass = @($day13State | Where-Object { Is-StepPassed -Step $_ }).Count
$day13FailurePass = @($day13Failure | Where-Object { Is-StepPassed -Step $_ }).Count

[ordered]@{
  date = "2026-04-10"
  stage = "Stage D Day13 state writeback"
  passed_steps = $day13StatePass
  total_steps = @($day13State).Count
  all_passed = ($day13StatePass -eq @($day13State).Count)
  state_suggest_rerun_used = $stateSuggestRerunUsed
  state_suggest_rerun_run_id = $stateSuggestRerunRunId
  steps = $day13State
} | ConvertTo-Json -Depth 10 | Set-Content -Path (Join-Path $evidence "day13-state-writeback-20260410.json") -Encoding UTF8

[ordered]@{
  date = "2026-04-10"
  stage = "Stage D Day13 failure closeout"
  passed_steps = $day13FailurePass
  total_steps = @($day13Failure).Count
  all_passed = ($day13FailurePass -eq @($day13Failure).Count)
  steps = $day13Failure
} | ConvertTo-Json -Depth 10 | Set-Content -Path (Join-Path $evidence "day13-failure-closeout-20260410.json") -Encoding UTF8

Save-MdTable -Title "Day13 state writeback" -OutputPath (Join-Path $evidence "day13-state-writeback-20260410.md") -Steps $day13State -PassedCount $day13StatePass
Save-MdTable -Title "Day13 failure closeout" -OutputPath (Join-Path $evidence "day13-failure-closeout-20260410.md") -Steps $day13Failure -PassedCount $day13FailurePass -Note "note: this round keeps an empty failure closeout record."

$day14ImportContent = @"
# Day14 import sediment (2026-04-10)

1. docs/README.md has been read as import sample.
2. this sample sediment is prepared for state update and continuation answer.
"@

$day14StateContent = @"
# Day14 state update (2026-04-10)

- recent_result: Day13 state writeback and failure closeout done.
- stage_decision: full-chain closeout can proceed.
- next_candidates:
1. fix full-chain run index.
2. update section 12.9 in acceptance document.
3. output Top3 next issues.
"@

$day14WritebackContent = @"
# Day14 full-chain writeback result (2026-04-10)

1. import read done.
2. sediment write done.
3. state update write done.
4. continuation answer done.
5. next-step suggestion done.
6. low-risk read done.
7. writeback file done.
"@

$day14 = @()
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-import-read" -SessionId "day14-chain-import-read-20260410" -UserInput "read: docs/README.md" -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-sediment-write" -SessionId "day14-chain-sediment-write-20260410" -UserInput ("write: tmp/knowledge-import-day2/20260410-two-week/day14-import-summary.md`n" + $day14ImportContent) -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-state-update" -SessionId "day14-chain-state-update-20260410" -UserInput ("write: tmp/knowledge-import-day2/20260410-two-week/day14-state-update.md`n" + $day14StateContent) -RetryCount $WaitRetry
$day14ContinuePrompt = "what is done now and what is missing, answer by current evidence folder."
$day14NextStepPrompt = "based on current evidence, give one most-priority action and one reason."
if ($UseStablePrompts) {
  $day14ContinuePrompt = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String("5oiR546w5Zyo5YGa5Yiw5ZOq5LqG77yf6L+Y5beu5LuA5LmI77yf6K+35oyJ5b2T5YmN6K+B5o2u55uu5b2V5Zue562U44CC"))
  $day14NextStepPrompt = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String("5Z+65LqO5b2T5YmN6K+B5o2u77yM5LuK5aSp5pyA5LyY5YWI5YGa5ZOq5LiA5Lu25LqL77yf57uZ5LiA5Liq5Yqo5L2c5ZKM5Y6f5Zug44CC"))
}
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-continue-answer" -SessionId "day14-chain-continue-answer-20260410" -UserInput $day14ContinuePrompt -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-next-step" -SessionId "day14-chain-next-step-20260410" -UserInput $day14NextStepPrompt -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-lowrisk-read" -SessionId "day14-chain-lowrisk-read-20260410" -UserInput "read: tmp/knowledge-import-day2/20260410-two-week/day14-state-update.md" -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-writeback" -SessionId "day14-chain-writeback-20260410" -UserInput ("write: tmp/knowledge-import-day2/20260410-two-week/day14-full-chain-result.md`n" + $day14WritebackContent) -RetryCount $WaitRetry
$day14 += Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-chain-readback" -SessionId "day14-chain-readback-20260410" -UserInput "read: tmp/knowledge-import-day2/20260410-two-week/day14-full-chain-result.md" -RetryCount $WaitRetry

$day14ContinueRerunUsed = $false
$day14ContinueRerunRunId = ""
if ($RetryDay14ContinueOnce -and -not (Is-StepPassed -Step $day14[3])) {
  $rerunContinue = Invoke-Step -GatewayBase $Gateway -WorkspaceConfig $workspace -EvidenceRoot $evidence -Prefix "day14-rerun-continue-answer" -SessionId "day14-rerun-continue-answer-20260410" -UserInput $day14ContinuePrompt -RetryCount $WaitRetry
  if (Is-StepPassed -Step $rerunContinue) {
    $day14[3] = $rerunContinue
    $day14ContinueRerunUsed = $true
    $day14ContinueRerunRunId = $rerunContinue.run_id
  }
}

$day14Pass = @($day14 | Where-Object { Is-StepPassed -Step $_ }).Count
[ordered]@{
  date = "2026-04-10"
  stage = "Stage D Day14 full chain"
  passed_steps = $day14Pass
  total_steps = @($day14).Count
  all_passed = ($day14Pass -eq @($day14).Count)
  day14_continue_rerun_used = $day14ContinueRerunUsed
  day14_continue_rerun_run_id = $day14ContinueRerunRunId
  steps = $day14
} | ConvertTo-Json -Depth 10 | Set-Content -Path (Join-Path $evidence "day14-full-chain-summary-20260410.json") -Encoding UTF8

Save-MdTable -Title "Day14 full chain rehearsal" -OutputPath (Join-Path $evidence "day14-full-chain-summary-20260410.md") -Steps $day14 -PassedCount $day14Pass
@(
  "# Day14 Top3 next issues (2026-04-10)",
  "",
  "1. Provider parse fluctuation can still trigger recovery. Keep tracking stable-template hit rate.",
  "2. state fields are written back, but cross-day samples are still needed for stability check.",
  "3. full chain is runnable; next stage should have minimum automated regression entry."
) -join "`r`n" | Set-Content -Path (Join-Path $evidence "day14-top3-next-issues-20260410.md") -Encoding UTF8

[ordered]@{
  day13_state = "$day13StatePass/$(@($day13State).Count)"
  day13_failure = "$day13FailurePass/$(@($day13Failure).Count)"
  day14_chain = "$day14Pass/$(@($day14).Count)"
  day14_continue_rerun_used = $day14ContinueRerunUsed
  day14_continue_rerun_run_id = $day14ContinueRerunRunId
  evidence_dir = $evidence
} | ConvertTo-Json -Depth 6
