$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-e-cli-history"
$outFile = Join-Path $outDir "latest.json"
$logFile = Join-Path $outDir "go-test.log"

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
Set-Content -Path $logFile -Value "" -Encoding UTF8

Push-Location (Join-Path $root "gateway")
& go test ./internal/api ./internal/session -run "TestLogsHandlerRunsViewReturnsDistinctRuns|TestParseLogsViewDefaultAndReject|TestRecentRunsKeepsLatestPerRun" -count=1 *>&1 | Tee-Object -FilePath $logFile
$code = $LASTEXITCODE
Pop-Location

$passed = $code -eq 0
$report = [ordered]@{
  checked_at = (Get-Date).ToString("o")
  status = $(if ($passed) { "passed" } else { "failed" })
  checks = [ordered]@{
    logs_runs_view = $passed
    cli_history_slice_ready = $passed
  }
  artifacts = [ordered]@{
    report = $outFile
    go_test_log = $logFile
  }
}

$report | ConvertTo-Json -Depth 6 | Set-Content -Path $outFile -Encoding UTF8
if (-not $passed) {
  throw "stage-e cli history acceptance failed: $outFile"
}
Write-Output $outFile
