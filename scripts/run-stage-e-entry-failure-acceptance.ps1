$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$root = Split-Path -Parent $PSScriptRoot
$config = Get-Content -Raw (Join-Path $root "config\app.json") | ConvertFrom-Json
$outDir = Join-Path $root "tmp\stage-e-entry-failure"
$outFile = Join-Path $outDir "latest.json"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$gatewayExe = Join-Path $binDir "gateway-stage-e-entry-failure.exe"
$gatewayLog = Join-Path $logDir "gateway.log"
$gatewayProc = $null

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 30)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Invoke-JsonPost {
  param([string]$Url, $Body)
  $payload = $Body | ConvertTo-Json -Depth 10
  return Invoke-RestMethod -Uri $Url -Method Post -ContentType "application/json; charset=utf-8" -Body $payload
}

function Invoke-JsonGet {
  param([string]$Url)
  return Invoke-RestMethod -Uri $Url -Method Get
}

function Start-LoggedProcess {
  param([string]$FilePath, [string]$WorkDir, [string]$OutPath, [string]$ErrPath, [hashtable]$Env)
  $psi = New-Object System.Diagnostics.ProcessStartInfo
  $psi.FileName = $FilePath
  $psi.WorkingDirectory = $WorkDir
  $psi.UseShellExecute = $false
  $psi.CreateNoWindow = $true
  $psi.RedirectStandardOutput = $true
  $psi.RedirectStandardError = $true
  foreach ($k in $Env.Keys) { $psi.Environment[$k] = [string]$Env[$k] }
  $proc = New-Object System.Diagnostics.Process
  $proc.StartInfo = $psi
  $null = $proc.Start()
  $proc.BeginOutputReadLine()
  $proc.BeginErrorReadLine()
  Register-ObjectEvent -InputObject $proc -EventName OutputDataReceived -Action { if ($EventArgs.Data) { Add-Content -Path $using:OutPath -Value $EventArgs.Data } } | Out-Null
  Register-ObjectEvent -InputObject $proc -EventName ErrorDataReceived -Action { if ($EventArgs.Data) { Add-Content -Path $using:ErrPath -Value $EventArgs.Data } } | Out-Null
  return $proc
}

function Stop-LoggedProcess {
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

function Wait-GatewayTerminal {
  param([string]$LogsUrl, [string]$RunId, [int64]$Since, [int]$Attempts = 60)
  for ($i = 0; $i -lt $Attempts; $i++) {
    $payload = Invoke-JsonGet -Url $LogsUrl
    $items = @($payload.items | Where-Object { $_.run_id -eq $RunId -and [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { return $items }
    Start-Sleep -Milliseconds 500
  }
  throw "run timeout: $RunId"
}

function Read-ErrorCode {
  param($Event)
  if ($null -eq $Event) { return "" }
  if ($Event.error_code) { return [string]$Event.error_code }
  if ($Event.metadata -and $Event.metadata.error_code) { return [string]$Event.metadata.error_code }
  return ""
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Set-Content -Path $gatewayLog -Value "" -Encoding UTF8

try {
  $runtimePort = New-FreePort
  $gatewayPort = New-FreePort
  while ($gatewayPort -eq $runtimePort) { $gatewayPort = New-FreePort }
  $gatewayBase = "http://127.0.0.1:" + $gatewayPort

  $buildStdout = Join-Path $logDir "gateway-build.stdout.log"
  $buildStderr = Join-Path $logDir "gateway-build.stderr.log"
  $prevErr = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  Push-Location (Join-Path $root "gateway")
  & go build -o $gatewayExe ./cmd/server 1> $buildStdout 2> $buildStderr
  Pop-Location
  $ErrorActionPreference = $prevErr
  if ($LASTEXITCODE -ne 0) { throw "gateway build failed" }
  if (-not (Test-Path $gatewayExe)) { throw "gateway binary missing: $gatewayExe" }

  $gatewayProc = Start-LoggedProcess -FilePath $gatewayExe -WorkDir $root -OutPath $gatewayLog -ErrPath $gatewayLog -Env @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url ($gatewayBase + "/health"))) { throw "gateway not ready" }

  $seed = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $sessionId = "session-e5-fail-" + $seed
  $startAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $accepted = Invoke-JsonPost -Url ($gatewayBase + "/api/v1/chat/run") -Body @{
    session_id = $sessionId
    user_input = "cmd: Get-Date"
    mode = $config.default_mode
    model = $config.default_model
    workspace = $config.default_workspace
  }
  $logsUrl = $gatewayBase + "/api/v1/logs?session_id=" + [Uri]::EscapeDataString([string]$accepted.session_id) + "&run_id=" + [Uri]::EscapeDataString([string]$accepted.run_id) + "&limit=200"
  $events = Wait-GatewayTerminal -LogsUrl $logsUrl -RunId $accepted.run_id -Since $startAt
  $runFailed = @($events | Where-Object { $_.event_type -eq "run_failed" } | Select-Object -Last 1)
  $runFinished = @($events | Where-Object { $_.event_type -eq "run_finished" } | Select-Object -Last 1)

  $allRunMatched = @($events | Where-Object { $_.run_id -ne $accepted.run_id }).Count -eq 0
  $allSessionMatched = @($events | Where-Object { $_.session_id -ne $accepted.session_id }).Count -eq 0
  $hasRunFailed = $runFailed.Count -gt 0
  $hasRunFinished = $runFinished.Count -gt 0
  $terminalIsFinished = $hasRunFinished -and ([string]$runFinished[0].event_type -eq "run_finished")
  $errorCodeMatched = $hasRunFailed -and (Read-ErrorCode -Event $runFailed[0]) -eq "runtime_unavailable"
  $passed = [bool]$accepted.accepted -and $allRunMatched -and $allSessionMatched -and $hasRunFailed -and $hasRunFinished -and $terminalIsFinished -and $errorCodeMatched

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    gateway = $gatewayBase
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    run_accepted = $accepted
    checks = [ordered]@{
      all_run_matched = $allRunMatched
      all_session_matched = $allSessionMatched
      has_run_failed = $hasRunFailed
      has_run_finished = $hasRunFinished
      terminal_is_run_finished = $terminalIsFinished
      error_code_runtime_unavailable = $errorCodeMatched
    }
    run = [ordered]@{
      run_id = $accepted.run_id
      session_id = $accepted.session_id
      event_types = @($events | Select-Object -ExpandProperty event_type)
      failed_event = $(if ($hasRunFailed) { $runFailed[0] } else { $null })
      terminal_event = $(if ($hasRunFinished) { $runFinished[0] } else { $null })
    }
    artifacts = [ordered]@{
      report = $outFile
      gateway_log = $gatewayLog
      gateway_build_stdout = $buildStdout
      gateway_build_stderr = $buildStderr
    }
  }

  $report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) { throw "stage-e failure acceptance failed: $outFile" }
  Write-Output $outFile
} finally {
  Stop-LoggedProcess -Process $gatewayProc
  Cleanup-ProcessEvents
}
