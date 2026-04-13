param(
  [string]$ApiBaseUrl = "http://127.0.0.1:21100",
  [string]$AuthToken = "",
  [string]$AgentId = "default",
  [switch]$DryRun,
  [string]$OutputPath = ""
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

$failedRegex = @("\bfailed\b", "\berror\b", "\bexception\b", "\btraceback\b", "not passed", "not completed")
$noValueRegex = @("^\s*ok\s*$", "^\s*done\s*$", "\bdone\b", "\bcontinue\b", "pending confirmation", "no update", "^\s*ack\s*$")

$candidates = @()
foreach ($item in $items) {
  $content = [string]$item.content
  $lower = $content.ToLowerInvariant()
  $trimmed = $content.Trim()
  $reason = ""

  if ($trimmed.Length -lt 20) {
    $reason = "short_noise"
  } elseif ($failedRegex | Where-Object { [regex]::IsMatch($lower, [string]$_) } | Select-Object -First 1) {
    $reason = "failed_result"
  } elseif ($noValueRegex | Where-Object { [regex]::IsMatch($lower, [string]$_) } | Select-Object -First 1) {
    $reason = "no_value"
  }

  if (-not [string]::IsNullOrWhiteSpace($reason)) {
    $candidates += [pscustomobject]@{
      id = [string]$item.id
      reason = $reason
      content = $content
      created_at = [string]$item.created_at
    }
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
  deleted_count = $deleted.Count
  reason_stats = $reasonStats
  deleted_samples = @($deleted | Select-Object -First 5)
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
