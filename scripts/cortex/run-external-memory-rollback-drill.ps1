param(
  [string]$RepoRoot = "",
  [string]$OutputPath = ""
)

$ErrorActionPreference = "Stop"

$resolvedRepoRoot = if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  (Resolve-Path (Join-Path $PSScriptRoot "..\\..")).Path
} else {
  (Resolve-Path $RepoRoot).Path
}

$setFlagScript = Join-Path $resolvedRepoRoot "scripts\\cortex\\set-external-memory-flag.ps1"
& $setFlagScript -Enabled $true | Out-Null
& $setFlagScript -Enabled $false | Out-Null

$flagPath = Join-Path $resolvedRepoRoot "data\\settings\\external-memory-cortex.json"
$flag = Get-Content -Path $flagPath | ConvertFrom-Json

Push-Location $resolvedRepoRoot
$test1 = "cargo test -p runtime-core knowledge_store::tests::append_knowledge_record_keeps_local_data_when_external_sync_fails -- --nocapture"
$test2 = "cargo test -p runtime-core knowledge::tests::recall_degrades_to_empty_when_external_fails -- --nocapture"
Invoke-Expression $test1 | Out-Null
$test1Exit = $LASTEXITCODE
Invoke-Expression $test2 | Out-Null
$test2Exit = $LASTEXITCODE
Pop-Location

$report = [ordered]@{
  generated_at = (Get-Date).ToString("o")
  flag_path = $flagPath
  rollback_enabled_false = (-not [bool]$flag.enabled)
  test_results = @(
    [ordered]@{ command = $test1; exit_code = $test1Exit; passed = ($test1Exit -eq 0) },
    [ordered]@{ command = $test2; exit_code = $test2Exit; passed = ($test2Exit -eq 0) }
  )
  passed = ((-not [bool]$flag.enabled) -and ($test1Exit -eq 0) -and ($test2Exit -eq 0))
}

$json = $report | ConvertTo-Json -Depth 6
if (-not [string]::IsNullOrWhiteSpace($OutputPath)) {
  $dir = Split-Path -Parent $OutputPath
  if (-not [string]::IsNullOrWhiteSpace($dir)) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
  }
  Set-Content -Path $OutputPath -Value $json -Encoding UTF8
}

$report
