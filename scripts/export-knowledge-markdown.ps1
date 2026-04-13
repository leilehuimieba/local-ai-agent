param(
  [string]$WorkspaceId = "main",
  [string]$StartAt = "",
  [string]$EndAt = "",
  [string]$ExportRoot = ""
)

$ErrorActionPreference = "Stop"

function Write-Utf8NoBom {
  param([string]$Path, [string]$Content)
  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Safe-FileSegment {
  param([string]$Value)
  $safe = ($Value -replace "[^a-zA-Z0-9_-]", "_")
  if ([string]::IsNullOrWhiteSpace($safe)) { return "unknown" }
  return $safe
}

function Normalize-Slug {
  param([string]$Text)
  $lower = $Text.ToLowerInvariant().Trim()
  $slug = ($lower -replace "[^a-z0-9\s_-]", "") -replace "\s+", "-"
  $slug = ($slug -replace "-{2,}", "-").Trim("-")
  if ([string]::IsNullOrWhiteSpace($slug)) { return "general" }
  return $slug
}

function Parse-TimeValue {
  param([string]$Text)
  if ([string]::IsNullOrWhiteSpace($Text)) { return $null }
  $num = 0L
  if ([long]::TryParse($Text, [ref]$num)) {
    if ($Text.Length -ge 13) { return [DateTimeOffset]::FromUnixTimeMilliseconds($num) }
    return [DateTimeOffset]::FromUnixTimeSeconds($num)
  }
  $parsed = [DateTimeOffset]::MinValue
  if ([DateTimeOffset]::TryParse($Text, [ref]$parsed)) { return $parsed.ToUniversalTime() }
  return $null
}

function Record-Time {
  param($Record)
  $updated = Parse-TimeValue([string]$Record.updated_at)
  if ($null -ne $updated) { return $updated }
  return Parse-TimeValue([string]$Record.created_at)
}

function In-TimeWindow {
  param($Record, $StartTime, $EndTime)
  $time = Record-Time $Record
  if ($null -eq $time) { return $true }
  if ($null -ne $StartTime -and $time -lt $StartTime) { return $false }
  if ($null -ne $EndTime -and $time -gt $EndTime) { return $false }
  return $true
}

function Ensure-ArrayFromJson {
  param($RawValue)
  if ($null -eq $RawValue) { return @() }
  if ($RawValue -is [array]) { return $RawValue }
  $text = [string]$RawValue
  if ([string]::IsNullOrWhiteSpace($text)) { return @() }
  try {
    $decoded = $text | ConvertFrom-Json
    if ($decoded -is [array]) { return $decoded }
    return @($decoded)
  } catch {
    return @($text)
  }
}

function Normalize-Record {
  param($Record)
  $tags = Ensure-ArrayFromJson $Record.tags
  return [PSCustomObject]@{
    id = [string]$Record.id
    workspace_id = [string]$Record.workspace_id
    knowledge_type = [string]$Record.knowledge_type
    title = [string]$Record.title
    summary = [string]$Record.summary
    content = [string]$Record.content
    tags = @($tags | ForEach-Object { [string]$_ })
    source = [string]$Record.source
    source_type = [string]$Record.source_type
    verified = [bool]$Record.verified
    priority = [int]$Record.priority
    created_at = [string]$Record.created_at
    updated_at = [string]$Record.updated_at
  }
}

function Load-KnowledgeFromSqlite {
  param([string]$RepoRoot, [string]$TargetWorkspaceId)
  $dbPath = Join-Path $RepoRoot "data\storage\main.db"
  if (!(Test-Path $dbPath)) { return @() }
  $workspaceSql = $TargetWorkspaceId.Replace("'", "''")
  $sql = "select id,workspace_id,knowledge_type,title,summary,content,tags,source,source_type,verified,priority,created_at,updated_at from knowledge_base where workspace_id='$workspaceSql' and archived=0 order by updated_at desc;"
  $raw = & sqlite3 $dbPath -json $sql 2>$null
  if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($raw)) { return @() }
  $items = $raw | ConvertFrom-Json
  return @($items | ForEach-Object { Normalize-Record $_ })
}

function Load-KnowledgeFromJsonl {
  param([string]$RepoRoot, [string]$TargetWorkspaceId)
  $path = Join-Path $RepoRoot ("data\knowledge_base\" + $TargetWorkspaceId + ".jsonl")
  if (!(Test-Path $path)) { return @() }
  $records = @()
  foreach ($line in Get-Content -Path $path) {
    if ([string]::IsNullOrWhiteSpace($line)) { continue }
    $records += Normalize-Record ($line | ConvertFrom-Json)
  }
  return $records
}

function Resolve-KnowledgeRecords {
  param([string]$RepoRoot, [string]$TargetWorkspaceId)
  $sqliteRecords = Load-KnowledgeFromSqlite -RepoRoot $RepoRoot -TargetWorkspaceId $TargetWorkspaceId
  if ($sqliteRecords.Count -gt 0) { return $sqliteRecords }
  return Load-KnowledgeFromJsonl -RepoRoot $RepoRoot -TargetWorkspaceId $TargetWorkspaceId
}

function Build-Links {
  param($Record)
  $topic = Normalize-Slug($Record.title)
  $workflow = Normalize-Slug($Record.knowledge_type)
  $sourceType = Normalize-Slug($Record.source_type)
  $task = Task-LinkSlug $Record
  $conclusion = Conclusion-LinkSlug $Record
  $links = @(
    "[[topic/$topic]]",
    "[[task/$task]]",
    "[[conclusion/$conclusion]]",
    "[[workflow/$workflow]]",
    "[[source/$sourceType]]"
  )
  return @($links | Select-Object -Unique)
}

function Task-LinkSlug {
  param($Record)
  $seed = [string]$Record.summary
  if ([string]::IsNullOrWhiteSpace($seed)) { $seed = [string]$Record.title }
  $first = ($seed -split "[。！？!?；;，,]")[0]
  $slug = Normalize-Slug($first.Substring(0, [Math]::Min(40, $first.Length)))
  if ($slug.Length -lt 4) { return Normalize-Slug("task-" + [string]$Record.knowledge_type) }
  return $slug
}

function Conclusion-LinkSlug {
  param($Record)
  $seed = [string]$Record.content
  if ([string]::IsNullOrWhiteSpace($seed)) { $seed = [string]$Record.summary }
  $first = ($seed -split "[。！？!?；;，,]")[0]
  $slug = Normalize-Slug($first.Substring(0, [Math]::Min(48, $first.Length)))
  if ($slug.Length -lt 4) { return Normalize-Slug("result-" + [string]$Record.id) }
  return $slug
}

function Build-Tags {
  param($Record)
  $verifiedFlag = if ($Record.verified) { "true" } else { "false" }
  $items = @(
    "kb/" + (Normalize-Slug $Record.knowledge_type);
    "source/" + (Normalize-Slug $Record.source_type);
    "verified/$verifiedFlag";
    "topic/" + (Normalize-Slug $Record.title)
  )
  $unique = New-Object System.Collections.Generic.List[string]
  foreach ($item in $items) {
    if ([string]::IsNullOrWhiteSpace($item)) { continue }
    if (-not $unique.Contains($item)) { $unique.Add($item) }
  }
  return @($unique)
}

function Build-Frontmatter {
  param($Record, [string]$ExportedAt)
  $tags = Build-Tags $Record
  $lines = @(
    "---",
    "id: `"$($Record.id)`"",
    "workspace_id: `"$($Record.workspace_id)`"",
    "knowledge_type: `"$($Record.knowledge_type)`"",
    "source: `"$($Record.source)`"",
    "source_type: `"$($Record.source_type)`"",
    "verified: $($Record.verified.ToString().ToLowerInvariant())",
    "priority: $($Record.priority)",
    "created_at: `"$($Record.created_at)`"",
    "updated_at: `"$($Record.updated_at)`"",
    "exported_at: `"$ExportedAt`"",
    "trace_id: `"`"",
    "tags:"
  )
  foreach ($tag in $tags) { $lines += "  - `"$tag`"" }
  $lines += "---"
  return ($lines -join "`n")
}

function Build-Markdown {
  param($Record, [string]$ExportedAt)
  $frontmatter = Build-Frontmatter -Record $Record -ExportedAt $ExportedAt
  $links = Build-Links $Record
  $linkText = ($links | ForEach-Object { "- $_" }) -join "`n"
  $body = @(
    "# $($Record.title)",
    "",
    "## Summary",
    "$($Record.summary)",
    "",
    "## Content",
    "$($Record.content)",
    "",
    "## Context",
    "- Source: $($Record.source)",
    "- Source Type: $($Record.source_type)",
    "- Verified: $($Record.verified.ToString().ToLowerInvariant())",
    "- Updated At: $($Record.updated_at)",
    "",
    "## Links",
    $linkText
  ) -join "`n"
  return ($frontmatter + "`n`n" + $body + "`n")
}

function Append-JsonLine {
  param([string]$Path, $Object)
  $payload = $Object | ConvertTo-Json -Compress -Depth 8
  Add-Content -Path $Path -Value $payload -Encoding UTF8
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$resolvedExportRoot = if ([string]::IsNullOrWhiteSpace($ExportRoot)) {
  Join-Path $repoRoot "data\exports\knowledge-markdown"
} else {
  $ExportRoot
}

$startTime = Parse-TimeValue $StartAt
$endTime = Parse-TimeValue $EndAt
$records = Resolve-KnowledgeRecords -RepoRoot $repoRoot -TargetWorkspaceId $WorkspaceId
$selected = @($records | Where-Object { In-TimeWindow $_ $startTime $endTime })

$batchId = Get-Date -Format "yyyyMMdd-HHmmss"
$batchDir = Join-Path $resolvedExportRoot $batchId
New-Item -ItemType Directory -Path $batchDir -Force | Out-Null
$indexPath = Join-Path $batchDir "index.jsonl"
if (Test-Path $indexPath) { Remove-Item -LiteralPath $indexPath -Force }

$exportedAt = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
$exportedCount = 0
foreach ($record in $selected) {
  $name = "{0}__{1}.md" -f (Safe-FileSegment $record.knowledge_type), (Safe-FileSegment $record.id)
  $target = Join-Path $batchDir $name
  Write-Utf8NoBom -Path $target -Content (Build-Markdown -Record $record -ExportedAt $exportedAt)
  Append-JsonLine -Path $indexPath -Object ([ordered]@{
    id = $record.id
    workspace_id = $record.workspace_id
    knowledge_type = $record.knowledge_type
    source = $record.source
    source_type = $record.source_type
    updated_at = $record.updated_at
    export_path = $target
  })
  $exportedCount++
}

$summary = [ordered]@{
  workspace_id = $WorkspaceId
  start_at = $StartAt
  end_at = $EndAt
  batch_id = $batchId
  batch_dir = $batchDir
  index_path = $indexPath
  total_records = $records.Count
  exported_records = $exportedCount
  data_source = $(if ($records.Count -gt 0 -and (Test-Path (Join-Path $repoRoot "data\storage\main.db"))) { "sqlite_or_jsonl" } else { "jsonl_or_empty" })
}
$summary | ConvertTo-Json -Depth 6
