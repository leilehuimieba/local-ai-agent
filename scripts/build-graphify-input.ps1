param(
  [string]$BatchDir = "",
  [string]$ExportRoot = ""
)

$ErrorActionPreference = "Stop"

function Write-Utf8NoBom {
  param([string]$Path, [string]$Content)
  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Resolve-BatchDirectory {
  param([string]$RootPath, [string]$SelectedBatch)
  if (-not [string]::IsNullOrWhiteSpace($SelectedBatch)) { return $SelectedBatch }
  if (-not (Test-Path $RootPath)) { throw "导出根目录不存在：$RootPath" }
  $latest = Get-ChildItem $RootPath -Directory | Sort-Object Name -Descending | Select-Object -First 1
  if ($null -eq $latest) { throw "导出根目录下无可用批次：$RootPath" }
  return $latest.FullName
}

function Extract-WikiLinks {
  param([string]$Text)
  $matches = [regex]::Matches($Text, "\[\[([^\]]+)\]\]")
  $links = @()
  foreach ($m in $matches) { $links += $m.Groups[1].Value.Trim() }
  return @($links | Select-Object -Unique)
}

function Link-ToNode {
  param([string]$Link)
  $parts = $Link.Split("/", 2)
  if ($parts.Count -lt 2) {
    return [PSCustomObject]@{ id = "misc/$Link"; kind = "misc"; label = $Link; relation = "related_to" }
  }
  return [PSCustomObject]@{
    id = "$($parts[0])/$($parts[1])"
    kind = $parts[0]
    label = $parts[1]
    relation = "$($parts[0])_link"
  }
}

$repoRoot = Split-Path -Parent $PSScriptRoot
$resolvedExportRoot = if ([string]::IsNullOrWhiteSpace($ExportRoot)) {
  Join-Path $repoRoot "data\exports\knowledge-markdown"
} else {
  $ExportRoot
}
$resolvedBatch = Resolve-BatchDirectory -RootPath $resolvedExportRoot -SelectedBatch $BatchDir

$graphDir = Join-Path $resolvedBatch "graphify"
New-Item -ItemType Directory -Force -Path $graphDir | Out-Null
$graphPath = Join-Path $graphDir "graphify-input.json"
$nodesPath = Join-Path $graphDir "graphify-nodes.jsonl"
$edgesPath = Join-Path $graphDir "graphify-edges.jsonl"
if (Test-Path $nodesPath) { Remove-Item -LiteralPath $nodesPath -Force }
if (Test-Path $edgesPath) { Remove-Item -LiteralPath $edgesPath -Force }

$nodes = @{}
$edges = New-Object System.Collections.Generic.List[object]
$mdFiles = Get-ChildItem -Path $resolvedBatch -Filter "*.md" -File

foreach ($file in $mdFiles) {
  $docId = "doc/" + $file.BaseName
  if (-not $nodes.ContainsKey($docId)) {
    $nodes[$docId] = [ordered]@{ id = $docId; kind = "document"; label = $file.BaseName; path = $file.FullName }
  }
  $content = Get-Content -Path $file.FullName -Raw
  foreach ($link in Extract-WikiLinks $content) {
    $target = Link-ToNode $link
    if (-not $nodes.ContainsKey($target.id)) {
      $nodes[$target.id] = [ordered]@{ id = $target.id; kind = $target.kind; label = $target.label; path = "" }
    }
    $edges.Add([ordered]@{
      id = "$docId->$($target.id)"
      source = $docId
      target = $target.id
      relation = $target.relation
    })
  }
}

$nodeItems = New-Object System.Collections.Generic.List[object]
foreach ($node in $nodes.Values) { $nodeItems.Add($node) }
$graph = @{
  batch_dir = $resolvedBatch
  generated_at = (Get-Date).ToString("o")
  nodes = $nodeItems.ToArray()
  edges = $edges.ToArray()
}
Write-Utf8NoBom -Path $graphPath -Content ($graph | ConvertTo-Json -Depth 8)
foreach ($node in $nodes.Values) { Add-Content -Path $nodesPath -Value ($node | ConvertTo-Json -Compress) -Encoding UTF8 }
foreach ($edge in $edges) { Add-Content -Path $edgesPath -Value ($edge | ConvertTo-Json -Compress) -Encoding UTF8 }

[ordered]@{
  batch_dir = $resolvedBatch
  markdown_files = $mdFiles.Count
  nodes = $nodes.Count
  edges = $edges.Count
  graphify_input = $graphPath
  graphify_nodes = $nodesPath
  graphify_edges = $edgesPath
} | ConvertTo-Json -Depth 6
