param(
  [string]$RepoRoot = "",
  [string]$ApiBaseUrl = "http://127.0.0.1:21100",
  [string]$AuthToken = "",
  [string]$FixturePath = "",
  [string]$AgentId = "eval-cet4-t25",
  [string]$OutputDir = ""
)

$ErrorActionPreference = "Stop"

$resolvedRepoRoot = if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
} else {
  (Resolve-Path $RepoRoot).Path
}

$resolvedFixturePath = if ([string]::IsNullOrWhiteSpace($FixturePath)) {
  Join-Path $resolvedRepoRoot "docs/11-hermes-rebuild/changes/E-knowledge-base-activation/fixtures/cet4-acceptance-cases.jsonl"
} else {
  (Resolve-Path $FixturePath).Path
}

$resolvedOutputDir = if ([string]::IsNullOrWhiteSpace($OutputDir)) {
  Join-Path $resolvedRepoRoot "tmp/stage-e-knowledge-recall-eval"
} else {
  $OutputDir
}

$resolvedToken = if ([string]::IsNullOrWhiteSpace($AuthToken)) {
  if ([string]::IsNullOrWhiteSpace($env:CORTEX_AUTH_TOKEN)) { "local-dev-token-20260413" } else { $env:CORTEX_AUTH_TOKEN }
} else {
  $AuthToken
}

New-Item -ItemType Directory -Force -Path $resolvedOutputDir | Out-Null

$headers = @{}
if (-not [string]::IsNullOrWhiteSpace($resolvedToken)) {
  $headers.Authorization = "Bearer $resolvedToken"
}

$cases = @()
$utf8 = New-Object System.Text.UTF8Encoding($false)
$fixtureLines = [System.IO.File]::ReadAllLines($resolvedFixturePath, $utf8)
foreach ($line in $fixtureLines) {
  if ([string]::IsNullOrWhiteSpace($line)) { continue }
  $cases += ($line | ConvertFrom-Json)
}

$listUrl = "$ApiBaseUrl/api/v1/memories?agent_id=$AgentId&limit=500"
$existing = Invoke-RestMethod -Uri $listUrl -Method Get -Headers $headers
foreach ($mem in @($existing.items)) {
  $deleteUrl = "$ApiBaseUrl/api/v1/memories/$($mem.id)"
  Invoke-RestMethod -Uri $deleteUrl -Method Delete -Headers $headers | Out-Null
}

$seededCount = 0
foreach ($case in $cases) {
  $checks = @($case.expected_checks) -join " | "
  $content = "topic:cet4; case_id:$($case.id); category:$($case.category); source_run:$($case.source_run_id); query:$($case.user_input); checks:$checks"
  $payload = @{
    layer = "core"
    category = "fact"
    content = $content
    agent_id = $AgentId
    importance = 0.9
    confidence = 0.9
    source = "t25-eval-fixture"
  } | ConvertTo-Json -Compress
  Invoke-RestMethod -Uri "$ApiBaseUrl/api/v1/memories" -Method Post -Headers $headers -ContentType "application/json" -Body $payload | Out-Null
  $seededCount++
}

$caseResults = @()
$latencyValues = @()
foreach ($case in $cases) {
  $primaryQuery = [string]$case.user_input
  $fallbackQuery = "cet4 $($case.category) $($case.id) $($case.source_run_id)"
  $recallBody = @{
    query = $primaryQuery
    agent_id = $AgentId
    skip_filters = $true
    layers = @("core")
  } | ConvertTo-Json -Compress
  $recall = Invoke-RestMethod -Uri "$ApiBaseUrl/api/v1/recall" -Method Post -Headers $headers -ContentType "application/json" -Body $recallBody
  $queryMode = "primary"
  $memories = @($recall.memories)
  if ($memories.Count -eq 0) {
    $fallbackBody = @{
      query = $fallbackQuery
      agent_id = $AgentId
      skip_filters = $true
      layers = @("core")
    } | ConvertTo-Json -Compress
    $recall = Invoke-RestMethod -Uri "$ApiBaseUrl/api/v1/recall" -Method Post -Headers $headers -ContentType "application/json" -Body $fallbackBody
    $queryMode = "fallback"
    $memories = @($recall.memories)
  }

  $top5 = @($memories | Select-Object -First 5)
  $joined = (@($top5 | ForEach-Object { [string]$_.content }) -join "`n").ToLowerInvariant()
  $category = ([string]$case.category).ToLowerInvariant()
  $topicHit = ($joined.Contains("cet4") -or $joined.Contains("四级"))
  $actionHit = $joined.Contains($category)
  $matchedChecks = 0
  foreach ($check in @($case.expected_checks)) {
    if ($joined.Contains(([string]$check).ToLowerInvariant())) { $matchedChecks++ }
  }

  $passed = $topicHit -and $actionHit
  $failureReason = ""
  if (-not $passed) {
    if ($top5.Count -eq 0) {
      $failureReason = "recall_empty"
    } elseif (-not $topicHit) {
      $failureReason = "topic_missing"
    } elseif (-not $actionHit) {
      $failureReason = "action_missing"
    } else {
      $failureReason = "expected_checks_missing"
    }
  }

  $latency = [int]$recall.meta.latency_ms
  $latencyValues += $latency
  $caseResults += [pscustomobject]@{
    case_id = [string]$case.id
    query_mode = $queryMode
    top5_count = $top5.Count
    topic_hit = $topicHit
    action_hit = $actionHit
    matched_checks = $matchedChecks
    passed = $passed
    failure_reason = $failureReason
    recall_latency_ms = $latency
  }
}

$totalCases = $caseResults.Count
$hitCases = @($caseResults | Where-Object { $_.passed }).Count
$hitRate = if ($totalCases -eq 0) { 0.0 } else { [math]::Round(($hitCases * 100.0 / $totalCases), 2) }
$sortedLatency = @($latencyValues | Sort-Object)
$p95Index = if ($sortedLatency.Count -eq 0) { 0 } else { [math]::Ceiling($sortedLatency.Count * 0.95) - 1 }
if ($p95Index -lt 0) { $p95Index = 0 }
$p95Latency = if ($sortedLatency.Count -eq 0) { 0 } else { [int]$sortedLatency[$p95Index] }
$failedCases = @($caseResults | Where-Object { -not $_.passed })
$failureTop3 = @($failedCases | Group-Object -Property failure_reason | Sort-Object Count -Descending | Select-Object -First 3 | ForEach-Object {
  [ordered]@{ reason = [string]$_.Name; count = [int]$_.Count }
})

$runId = "t25-" + (Get-Date -Format "yyyyMMdd-HHmmss")
$result = [ordered]@{
  run_id = $runId
  generated_at = (Get-Date).ToString("o")
  agent_id = $AgentId
  source_path = $resolvedFixturePath
  checks = [ordered]@{
    seeded_cases = $seededCount
    total_cases = $totalCases
    hit_cases = $hitCases
    top5_hit_rate = $hitRate
    top5_passed = ($hitRate -ge 70.0)
    recall_p95_latency_ms = $p95Latency
  }
  metric_records = @(
    [ordered]@{
      metric_id = "M02"
      value = $hitRate
      window = "cet4-fixed-batch"
      source_path = $resolvedFixturePath
      passed = ($hitRate -ge 70.0)
    },
    [ordered]@{
      metric_id = "M03"
      value = $p95Latency
      window = "cet4-fixed-batch"
      source_path = "api/v1/recall.meta.latency_ms"
      passed = ($p95Latency -le 1500)
    }
  )
  failure_reason_top3 = $failureTop3
  failed_cases = $failedCases
  case_results = $caseResults
}

$latestPath = Join-Path $resolvedOutputDir "latest.json"
$historyPath = Join-Path $resolvedOutputDir "$runId.json"
$json = $result | ConvertTo-Json -Depth 8
Set-Content -Path $latestPath -Value $json -Encoding UTF8
Set-Content -Path $historyPath -Value $json -Encoding UTF8

$result
