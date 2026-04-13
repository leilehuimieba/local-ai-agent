param(
  [string]$ProjectRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path,
  [string]$OutputDir = "tmp/frontend-wave1-regression",
  [switch]$SkipBuild
)

$targetRoot = Join-Path $ProjectRoot $OutputDir
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$runDir = Join-Path $targetRoot $timestamp
New-Item -ItemType Directory -Path $runDir -Force | Out-Null

$frontendDir = Join-Path $ProjectRoot "frontend"
$buildLog = Join-Path $runDir "frontend-build.log"

$summary = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  script = "scripts/run-frontend-wave1-regression.ps1"
  project_root = $ProjectRoot
  output_dir = $runDir
  build = [ordered]@{
    command = "npm run build"
    status = "skipped"
    log = $buildLog
    error = ""
  }
  manual_checklist = @(
    "task_page_status_loop",
    "investigation_panel_follow_mode",
    "history_filter_and_detail_linkage",
    "settings_diagnostics_group_readability",
    "mobile_layout_check"
  )
}

if (-not $SkipBuild) {
  Push-Location $frontendDir
  try {
    npm run build *>&1 | Tee-Object -FilePath $buildLog | Out-Null
    if ($LASTEXITCODE -eq 0) {
      $summary.build.status = "passed"
    } else {
      $summary.build.status = "failed"
      $summary.build.error = "npm run build exited with code $LASTEXITCODE"
    }
  } catch {
    $summary.build.status = "failed"
    $summary.build.error = $_.Exception.Message
  } finally {
    Pop-Location
  }
}

$summaryPath = Join-Path $runDir "summary.json"
$summary | ConvertTo-Json -Depth 6 | Set-Content -Path $summaryPath -Encoding UTF8

$latestPath = Join-Path $targetRoot "latest.json"
Copy-Item -Path $summaryPath -Destination $latestPath -Force

Write-Output "frontend wave1 regression summary: $summaryPath"
Write-Output "frontend wave1 regression latest: $latestPath"
