param(
  [string]$ApiBaseUrl = "http://127.0.0.1:21100",
  [string]$AuthToken = "",
  [string]$AgentId = "default",
  [switch]$DryRun,
  [string]$OutputPath = "",
  [double]$ScoreThreshold = 0.55,
  [int]$MinLength = 20
)

$ErrorActionPreference = "Stop"

$resolvedToken = if ([string]::IsNullOrWhiteSpace($AuthToken)) {
  if ([string]::IsNullOrWhiteSpace($env:CORTEX_AUTH_TOKEN)) { "local-dev-token-20260413" } else { $env:CORTEX_AUTH_TOKEN }
} else {
  $AuthToken
}

$headers = @{}
if (-not [string]::IsNullOrWhiteSpace($resolvedToken)) {
  $headers.Authorization = "Bearer $resolvedToken"
}

$listUrl = "$ApiBaseUrl/api/v1/memories?agent_id=$AgentId&limit=1000"
$items = @((Invoke-RestMethod -Uri $listUrl -Method Get -Headers $headers).items)

$failedRegex = @("^\s*failed[:\s]", "error while", "\bexception\b", "\btraceback\b", "execution failed", "not passed", "not completed")
$noValueRegex = @("^\s*ok\s*$", "^\s*done\s*$", "\bdone\b", "\bcontinue\b", "pending confirmation", "no update", "^\s*ack\s*$")

$dupMap = @{}
$dupBestMap = @{}
foreach ($item in $items) {
  $text = [string]$item.content
  $key = (($text.ToLowerInvariant() -replace '\s+', ' ').Trim())
  if ([string]::IsNullOrWhiteSpace($key)) { $key = "__empty__" }
  if ($dupMap.ContainsKey($key)) {
    $dupMap[$key] = [int]$dupMap[$key] + 1
  } else {
    $dupMap[$key] = 1
  }
  $conf = if ($null -eq $item.confidence) { 0.5 } else { [double]$item.confidence }
  $updated = [string]$item.updated_at
  if ($dupBestMap.ContainsKey($key)) {
    $best = $dupBestMap[$key]
    if (($conf -gt [double]$best.confidence) -or ($conf -eq [double]$best.confidence -and $updated -gt [string]$best.updated_at)) {
      $dupBestMap[$key] = [pscustomobject]@{ id = [string]$item.id; confidence = $conf; updated_at = $updated }
    }
  } else {
    $dupBestMap[$key] = [pscustomobject]@{ id = [string]$item.id; confidence = $conf; updated_at = $updated }
  }
}

$candidates = @()
$scored = @()
foreach ($item in $items) {
  $content = [string]$item.content
  $lower = $content.ToLowerInvariant()
  $trimmed = $content.Trim()
  $reason = ""
  $source = [string]$item.source
  $sourceLower = $source.ToLowerInvariant()
  $confidenceRaw = if ($null -eq $item.confidence) { 0.5 } else { [double]$item.confidence }
  $confidenceScore = [math]::Min(1.0, [math]::Max(0.0, $confidenceRaw))
  $sourceTrust = 0.6
  if ($sourceLower.Contains("runtime") -or $sourceLower.Contains("manual") -or $sourceLower.Contains("user")) {
    $sourceTrust = 0.85
  } elseif ($sourceLower.Contains("eval") -or $sourceLower.Contains("probe") -or $sourceLower.Contains("test")) {
    $sourceTrust = 0.35
  } elseif ([string]::IsNullOrWhiteSpace($sourceLower)) {
    $sourceTrust = 0.5
  }
  $dupKey = (($lower -replace '\s+', ' ').Trim())
  if ([string]::IsNullOrWhiteSpace($dupKey)) { $dupKey = "__empty__" }
  $dupCount = if ($dupMap.ContainsKey($dupKey)) { [int]$dupMap[$dupKey] } else { 1 }
  $duplicationScore = [math]::Round((1.0 / [double]$dupCount), 3)
  $qualityScore = [math]::Round(($confidenceScore * 0.5) + ($sourceTrust * 0.3) + ($duplicationScore * 0.2), 3)

  if ($trimmed.Length -lt $MinLength) {
    $reason = "short_noise"
  } elseif ($failedRegex | Where-Object { [regex]::IsMatch($lower, [string]$_) } | Select-Object -First 1) {
    $reason = "failed_result"
  } elseif ($noValueRegex | Where-Object { [regex]::IsMatch($lower, [string]$_) } | Select-Object -First 1) {
    $reason = "no_value"
  } elseif ($dupCount -gt 1 -and [string]$item.id -ne [string]$dupBestMap[$dupKey].id) {
    $reason = "duplicate_shadow"
  } elseif ($qualityScore -lt $ScoreThreshold) {
    $reason = "score_low"
  }

  $decision = if ([string]::IsNullOrWhiteSpace($reason)) { "keep" } else { "delete" }
  $scored += [pscustomobject]@{
    id = [string]$item.id
    decision = $decision
    reason = $reason
    quality_score = $qualityScore
    confidence_score = [math]::Round($confidenceScore, 3)
    source_trust = [math]::Round($sourceTrust, 3)
    duplication_score = $duplicationScore
    duplicate_count = $dupCount
    source = $source
    content = $content
  }

  if (-not [string]::IsNullOrWhiteSpace($reason)) {
    $candidates += ($scored | Select-Object -Last 1)
  }
}

$deleted = @()
if (-not $DryRun.IsPresent) {
  foreach ($candidate in $candidates) {
    $deleteUrl = "$ApiBaseUrl/api/v1/memories/$($candidate.id)"
    Invoke-RestMethod -Uri $deleteUrl -Method Delete -Headers $headers | Out-Null
    $deleted += $candidate
  }
}

$reasonStats = @($candidates | Group-Object reason | Sort-Object Name | ForEach-Object {
  [ordered]@{ reason = [string]$_.Name; count = [int]$_.Count }
})

$report = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  agent_id = $AgentId
  dry_run = $DryRun.IsPresent
  total_memories = $items.Count
  candidate_count = $candidates.Count
  kept_count = $items.Count - $candidates.Count
  deleted_count = $deleted.Count
  score_threshold = $ScoreThreshold
  model_weights = [ordered]@{
    confidence = 0.5
    source_trust = 0.3
    duplication = 0.2
  }
  reason_stats = $reasonStats
  scored_samples = @($scored | Select-Object -First 20)
  deleted_samples = @($deleted | Select-Object -First 10)
}

$json = $report | ConvertTo-Json -Depth 6
if (-not [string]::IsNullOrWhiteSpace($OutputPath)) {
  $outDir = Split-Path -Parent $OutputPath
  if (-not [string]::IsNullOrWhiteSpace($outDir)) {
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null
  }
  Set-Content -Path $OutputPath -Value $json -Encoding UTF8
}

$report
