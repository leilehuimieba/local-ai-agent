$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$config = Get-Content -Raw (Join-Path $root "config\app.json") | ConvertFrom-Json
$outDir = Join-Path $root "tmp\stage-e-entry1"
$outFile = Join-Path $outDir "latest.json"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-e-entry1.exe"
$runtimeLog = Join-Path $logDir "runtime.log"
$gatewayLog = Join-Path $logDir "gateway.log"
$sessionId = "stage-e-entry1-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$runtimeProc = $null
$gatewayProc = $null

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 40)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Join-ProcessArguments {
  param([string[]]$Arguments)
  if ($null -eq $Arguments -or $Arguments.Count -eq 0) { return "" }
  $quoted = foreach ($arg in $Arguments) {
    if ($arg -match '[\s"]') { '"' + ($arg -replace '"', '\"') + '"' } else { $arg }
  }
  return ($quoted -join " ")
}

function Invoke-LoggedProcess {
  param([string]$FilePath, [string[]]$Arguments, [string]$WorkDir, [string]$OutPath, [string]$ErrPath)
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.Arguments = Join-ProcessArguments $Arguments
  $psi.WorkingDirectory = $WorkDir
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $stdout = $proc.StandardOutput.ReadToEndAsync()
  $stderr = $proc.StandardError.ReadToEndAsync()
  $proc.WaitForExit()
  Set-Content -Path $OutPath -Value $stdout.Result -Encoding UTF8
  Set-Content -Path $ErrPath -Value $stderr.Result -Encoding UTF8
  if ($proc.ExitCode -ne 0) { throw "$FilePath failed with exit code: $($proc.ExitCode)" }
}

function Build-IsolatedBinaries {
  param([string]$Root, [string]$CargoTargetDir, [string]$RuntimeExe, [string]$GatewayExe, [string]$LogDir)
  Invoke-LoggedProcess "cargo" @("build", "-p", "runtime-host", "--target-dir", $CargoTargetDir) $Root (Join-Path $LogDir "runtime-build.stdout.log") (Join-Path $LogDir "runtime-build.stderr.log")
  if (-not (Test-Path $RuntimeExe)) { throw "runtime-host binary missing: $RuntimeExe" }
  Invoke-LoggedProcess "go" @("build", "-o", $GatewayExe, "./cmd/server") (Join-Path $Root "gateway") (Join-Path $LogDir "gateway-build.stdout.log") (Join-Path $LogDir "gateway-build.stderr.log")
  if (-not (Test-Path $GatewayExe)) { throw "gateway binary missing: $GatewayExe" }
}

function Start-IsolatedProcess {
  param([string]$FilePath, [string]$WorkDir, [string]$OutPath, [string]$ErrPath, [hashtable]$Env)
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkDir
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($key in $Env.Keys) { $psi.Environment[$key] = [string]$Env[$key] }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $proc.BeginOutputReadLine()
  $proc.BeginErrorReadLine()
  Register-ObjectEvent -InputObject $proc -EventName OutputDataReceived -Action { if ($EventArgs.Data) { Add-Content -Path $using:OutPath -Value $EventArgs.Data } } | Out-Null
  Register-ObjectEvent -InputObject $proc -EventName ErrorDataReceived -Action { if ($EventArgs.Data) { Add-Content -Path $using:ErrPath -Value $EventArgs.Data } } | Out-Null
  return $proc
}

function Stop-IsolatedProcess {
  param($Process)
  if ($null -eq $Process) { return }
  if (-not $Process.HasExited) {
    Stop-Process -Id $Process.Id -Force
    $Process.WaitForExit()
  }
}

function Cleanup-ProcessEvents {
  Get-EventSubscriber | Where-Object { $_.SourceObject -is [System.Diagnostics.Process] } | Unregister-Event
}

function Invoke-JsonPost {
  param([string]$Url, $Body)
  return Invoke-RestMethod -Uri $Url -Method Post -ContentType "application/json; charset=utf-8" -Body ($Body | ConvertTo-Json -Depth 8)
}

function Invoke-JsonGet {
  param([string]$Url)
  return Invoke-RestMethod -Uri $Url -Method Get
}

function Wait-RunTerminal {
  param([string]$LogsUrl, [string]$RunId, [int64]$Since, [int]$Attempts = 80)
  for ($i = 0; $i -lt $Attempts; $i++) {
    $payload = Invoke-JsonGet -Url $LogsUrl
    $items = @($payload.items | Where-Object { $_.run_id -eq $RunId -and [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { return $items }
    Start-Sleep -Milliseconds 500
  }
  throw "run timeout: $RunId"
}

function Has-ProtocolFields {
  param($Accepted)
  return [bool](
    $Accepted.accepted -and $Accepted.session_id -and $Accepted.run_id -and
    $Accepted.request_id -and $Accepted.trace_id -and $Accepted.entry_id -and
    $Accepted.protocol_version -and $Accepted.stream_endpoint -and
    $Accepted.logs_endpoint -and $Accepted.confirm_endpoint -and $Accepted.retry_endpoint
  )
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Remove-Item -Recurse -Force $cargoTargetDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cargoTargetDir | Out-Null
Set-Content -Path $runtimeLog -Value "" -Encoding UTF8
Set-Content -Path $gatewayLog -Value "" -Encoding UTF8

try {
  $runtimePort = New-FreePort
  $gatewayPort = New-FreePort
  while ($gatewayPort -eq $runtimePort) { $gatewayPort = New-FreePort }
  $gateway = "http://127.0.0.1:" + $gatewayPort
  $runUrl = $gateway + "/api/v1/chat/run"
  $healthUrl = $gateway + "/health"

  Build-IsolatedBinaries -Root $root -CargoTargetDir $cargoTargetDir -RuntimeExe $runtimeExe -GatewayExe $gatewayExe -LogDir $logDir

  $runtimeProc = Start-IsolatedProcess -FilePath $runtimeExe -WorkDir $root -OutPath $runtimeLog -ErrPath $runtimeLog -Env @{ LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:" + $runtimePort + "/health"))) { throw "runtime not ready" }

  $gatewayProc = Start-IsolatedProcess -FilePath $gatewayExe -WorkDir $root -OutPath $gatewayLog -ErrPath $gatewayLog -Env @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url $healthUrl)) { throw "gateway not ready" }

  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $accepted = Invoke-JsonPost -Url $runUrl -Body @{
    session_id = $sessionId
    user_input = "cmd: Get-ChildItem -Name | Select-Object -First 1"
    mode = $config.default_mode
    model = $config.default_model
    workspace = $config.default_workspace
  }
  $logsUrl = $gateway + "/api/v1/logs?session_id=" + [Uri]::EscapeDataString([string]$accepted.session_id) + "&run_id=" + [Uri]::EscapeDataString([string]$accepted.run_id) + "&limit=200"
  $items = Wait-RunTerminal -LogsUrl $logsUrl -RunId $accepted.run_id -Since $startedAt
  $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)

  $protocolFieldsOK = Has-ProtocolFields -Accepted $accepted
  $allRunMatched = @($items | Where-Object { $_.run_id -ne $accepted.run_id }).Count -eq 0
  $allSessionMatched = @($items | Where-Object { $_.session_id -ne $accepted.session_id }).Count -eq 0
  $hasRunStarted = @($items | Where-Object { $_.event_type -eq "run_started" }).Count -gt 0
  $hasTerminal = $terminal.Count -gt 0
  $terminalCompleted = $hasTerminal -and ([string]$terminal[0].completion_status -eq "completed")
  $passed = $protocolFieldsOK -and $allRunMatched -and $allSessionMatched -and $hasRunStarted -and $terminalCompleted

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    gateway = $gateway
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    run_accepted = $accepted
    checks = [ordered]@{
      protocol_fields_ok = $protocolFieldsOK
      all_run_matched = $allRunMatched
      all_session_matched = $allSessionMatched
      has_run_started = $hasRunStarted
      has_terminal = $hasTerminal
      terminal_completed = $terminalCompleted
    }
    run = [ordered]@{
      run_id = $accepted.run_id
      session_id = $accepted.session_id
      event_types = @($items | Select-Object -ExpandProperty event_type)
      terminal_event = $(if ($hasTerminal) { $terminal[0] } else { $null })
    }
    artifacts = [ordered]@{
      report = $outFile
      runtime_log = $runtimeLog
      gateway_log = $gatewayLog
      runtime_build_stdout = (Join-Path $logDir "runtime-build.stdout.log")
      runtime_build_stderr = (Join-Path $logDir "runtime-build.stderr.log")
      gateway_build_stdout = (Join-Path $logDir "gateway-build.stdout.log")
      gateway_build_stderr = (Join-Path $logDir "gateway-build.stderr.log")
    }
  }

  $report | ConvertTo-Json -Depth 9 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) { throw "stage-e entry1 acceptance failed: $outFile" }
  Write-Output $outFile
} finally {
  Stop-IsolatedProcess -Process $gatewayProc
  Stop-IsolatedProcess -Process $runtimeProc
  Cleanup-ProcessEvents
}
