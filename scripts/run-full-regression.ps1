param(
    [switch]$SkipE2E,
    [string]$OutFile = ""
)

$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$report = @{
    checked_at = (Get-Date -Format "o")
    overall_status = "passed"
    steps = @()
}

function Add-Step($name, $status, $exitCode, $summary) {
    $report.steps += @{
        name = $name
        status = $status
        exit_code = $exitCode
        summary = $summary
    }
    if ($status -ne "passed" -and $status -ne "skipped") {
        $report.overall_status = "failed"
    }
}

# Step 1: Rust check
Write-Host "[1/6] Rust cargo check --workspace ..." -ForegroundColor Cyan
$output = & cargo check --workspace 2>&1
$ec = $LASTEXITCODE
$rustCheckStatus = if ($ec -eq 0) { "passed" } else { "failed" }
$rustCheckSummary = if ($ec -eq 0) { "clean" } else { "failed" }
Add-Step "rust_check" $rustCheckStatus $ec $rustCheckSummary

# Step 2: Rust test
Write-Host "[2/6] Rust cargo test --workspace ..." -ForegroundColor Cyan
$output = & cargo test --workspace 2>&1
$ec = $LASTEXITCODE
$summary = ($output | Select-String "test result: ok\. \d+ passed" | Select-Object -Last 1).ToString().Trim()
if ($ec -ne 0) {
    Write-Host "  First attempt failed, retrying once ..." -ForegroundColor Yellow
    $output = & cargo test --workspace 2>&1
    $ec = $LASTEXITCODE
    $summary = ($output | Select-String "test result:" | Select-Object -Last 1).ToString().Trim()
}
$rustTestStatus = if ($ec -eq 0) { "passed" } else { "failed" }
Add-Step "rust_test" $rustTestStatus $ec $summary

# Step 3: Go build
Write-Host "[3/6] Go build ..." -ForegroundColor Cyan
Set-Location "$repoRoot\gateway"
$output = & go build ./... 2>&1
$ec = $LASTEXITCODE
Set-Location $repoRoot
$goBuildStatus = if ($ec -eq 0) { "passed" } else { "failed" }
$goBuildSummary = if ($ec -eq 0) { "clean" } else { "failed" }
Add-Step "go_build" $goBuildStatus $ec $goBuildSummary

# Step 4: Go test (service only, skip slow api tests)
Write-Host "[4/6] Go test ./internal/service/ ..." -ForegroundColor Cyan
Set-Location "$repoRoot\gateway"
$output = & go test ./internal/service/ 2>&1
$ec = $LASTEXITCODE
$summary = ($output | Select-String "^(ok|FAIL)" | Select-Object -Last 1).ToString().Trim()
$goTestStatus = if ($ec -eq 0) { "passed" } else { "failed" }
Add-Step "go_test" $goTestStatus $ec $summary
Set-Location $repoRoot

# Step 5: Frontend build
Write-Host "[5/6] Frontend npm run build ..." -ForegroundColor Cyan
Set-Location "$repoRoot\frontend"
$output = & npm run build 2>&1
$ec = $LASTEXITCODE
$frontendStatus = if ($ec -eq 0) { "passed" } else { "failed" }
$frontendSummary = if ($ec -eq 0) { "clean" } else { "failed" }
Add-Step "frontend_build" $frontendStatus $ec $frontendSummary
Set-Location $repoRoot

# Step 6: E2E acceptance
if (-not $SkipE2E) {
    Write-Host "[6/6] E2E acceptance ..." -ForegroundColor Cyan
    & "$repoRoot\scripts\run-stage-e-entry1-acceptance.ps1" -AllowAcceptedFallback:$false | Out-Null
    $ec = $LASTEXITCODE
    $e2eReport = Get-Content "$repoRoot\tmp\stage-e-entry1\latest.json" -Raw | ConvertFrom-Json
    $e2eStatus = if ($e2eReport.status -eq "passed") { "passed" } else { "failed" }
    Add-Step "e2e_acceptance" $e2eStatus $ec "mode=$($e2eReport.acceptance_mode); status=$($e2eReport.status)"
} else {
    Add-Step "e2e_acceptance" "skipped" 0 "skipped by -SkipE2E"
}

# Output
$json = $report | ConvertTo-Json -Depth 5
if ($OutFile) {
    $json | Out-File -FilePath $OutFile -Encoding utf8
    Write-Host "Report saved to $OutFile" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "===== Regression Report =====" -ForegroundColor Green
    Write-Host $json
}

if ($report.overall_status -eq "passed") {
    Write-Host "All checks passed." -ForegroundColor Green
    exit 0
} else {
    Write-Host "Some checks failed." -ForegroundColor Red
    exit 1
}
