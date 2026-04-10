$ErrorActionPreference = "Continue"

$root = Split-Path -Parent $PSScriptRoot
$runtime = Join-Path $root "target\debug\runtime-host.exe"
$outFile = Join-Path $root "tmp\runtime-host-lock-check.json"
$source = Join-Path $root "crates\runtime-host\src\main.rs"
$stdoutFile = Join-Path $root "tmp\runtime-host-lock-check.stdout.log"
$stderrFile = Join-Path $root "tmp\runtime-host-lock-check.stderr.log"

if (-not (Test-Path $runtime)) {
  throw "runtime-host.exe missing; run cargo build -p runtime-host first"
}

$process = Get-Process runtime-host -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $process) {
  Start-Process -FilePath $runtime -WorkingDirectory $root | Out-Null
  Start-Sleep -Seconds 1
  $process = Get-Process runtime-host -ErrorAction SilentlyContinue | Select-Object -First 1
}

if (-not $process) {
  throw "runtime-host failed to start"
}

$originalContent = [System.IO.File]::ReadAllText($source)
[System.IO.File]::WriteAllText($source, "$originalContent`r`n")
try {
  if (Test-Path $stdoutFile) {
    Remove-Item -LiteralPath $stdoutFile -Force
  }
  if (Test-Path $stderrFile) {
    Remove-Item -LiteralPath $stderrFile -Force
  }
  $build = Start-Process -FilePath "cargo" `
    -ArgumentList @("build", "-p", "runtime-host") `
    -WorkingDirectory $root `
    -NoNewWindow `
    -Wait `
    -PassThru `
    -RedirectStandardOutput $stdoutFile `
    -RedirectStandardError $stderrFile
  $exitCode = $build.ExitCode
} finally {
  [System.IO.File]::WriteAllText($source, $originalContent)
}
$stdout = if (Test-Path $stdoutFile) { Get-Content -Path $stdoutFile -Raw } else { "" }
$stderr = if (Test-Path $stderrFile) { Get-Content -Path $stderrFile -Raw } else { "" }
$text = "$stdout`r`n$stderr".Trim()
$failed = $text.Contains("failed to remove file") -and $text.Contains("runtime-host.exe")
$waited = $text.Contains("Blocking waiting for file lock")
$mode = "none"

if ($failed) {
  $mode = "remove_failed"
} elseif ($waited) {
  $mode = "file_lock_wait"
}

$result = [ordered]@{
  process_id = $process.Id
  process_name = $process.ProcessName
  command = "cargo build -p runtime-host"
  exit_code = $exitCode
  lock_detected = ($failed -or $waited)
  lock_mode = $mode
  output = $text
}

$result | ConvertTo-Json -Depth 4 | Set-Content -Path $outFile -Encoding UTF8
Write-Output $outFile
