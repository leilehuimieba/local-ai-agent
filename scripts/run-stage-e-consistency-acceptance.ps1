$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$root = Split-Path -Parent $PSScriptRoot
$config = Get-Content -Raw (Join-Path $root "config\app.json") | ConvertFrom-Json
$outDir = Join-Path $root "tmp\stage-e-consistency"
$outFile = Join-Path $outDir "latest.json"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-e-consistency.exe"
$runtimeLog = Join-Path $logDir "runtime.log"
$gatewayLog = Join-Path $logDir "gateway.log"
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

function Wait-GatewayTerminal {
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

function Read-ProviderRef {
  param($Config)
  $provider = @($Config.providers | Where-Object { $_.provider_id -eq $Config.default_model.provider_id } | Select-Object -First 1)
  if ($provider.Count -eq 0) { throw "default provider not found in config.providers" }
  return @{
    provider_id = $provider[0].provider_id
    display_name = $provider[0].display_name
    base_url = $provider[0].base_url
    chat_completions_path = $provider[0].chat_completions_path
    models_path = $provider[0].models_path
    api_key = $provider[0].api_key
  }
}

function Cleanup-ProcessEvents {
  Get-EventSubscriber | Where-Object { $_.SourceObject -is [System.Diagnostics.Process] } | Unregister-Event
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

  $runtimeBuildOut = Join-Path $logDir "runtime-build.stdout.log"
  $runtimeBuildErr = Join-Path $logDir "runtime-build.stderr.log"
  $gatewayBuildOut = Join-Path $logDir "gateway-build.stdout.log"
  $gatewayBuildErr = Join-Path $logDir "gateway-build.stderr.log"

  $previousErrorAction = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  & cargo build -p runtime-host --target-dir $cargoTargetDir 1> $runtimeBuildOut 2> $runtimeBuildErr
  $ErrorActionPreference = $previousErrorAction
  if ($LASTEXITCODE -ne 0) { throw "runtime build failed" }
  if (-not (Test-Path $runtimeExe)) { throw "runtime binary missing: $runtimeExe" }
  $previousErrorAction = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  Push-Location (Join-Path $root "gateway")
  & go build -o $gatewayExe ./cmd/server 1> $gatewayBuildOut 2> $gatewayBuildErr
  Pop-Location
  $ErrorActionPreference = $previousErrorAction
  if ($LASTEXITCODE -ne 0) { throw "gateway build failed" }
  if (-not (Test-Path $gatewayExe)) { throw "gateway binary missing: $gatewayExe" }

  $runtimeProc = Start-LoggedProcess -FilePath $runtimeExe -WorkDir $root -OutPath $runtimeLog -ErrPath $runtimeLog -Env @{ LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:" + $runtimePort + "/health"))) { throw "runtime not ready" }

  $gatewayProc = Start-LoggedProcess -FilePath $gatewayExe -WorkDir $root -OutPath $gatewayLog -ErrPath $gatewayLog -Env @{ LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort; LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort }
  $gatewayBase = "http://127.0.0.1:" + $gatewayPort
  if (-not (Wait-HttpReady -Url ($gatewayBase + "/health"))) { throw "gateway not ready" }

  $seed = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $requestId = "request-e4-" + $seed
  $runId = "run-e4-" + $seed
  $sessionId = "session-e4-" + $seed
  $traceId = "trace-e4-" + $seed
  $userInput = "cmd: Get-ChildItem -Name | Select-Object -First 1"
  $providerRef = Read-ProviderRef -Config $config

  $runtimeResponse = Invoke-JsonPost -Url ("http://127.0.0.1:" + $runtimePort + "/v1/runtime/run") -Body @{
    request_id = $requestId
    run_id = $runId
    session_id = $sessionId
    trace_id = $traceId
    user_input = $userInput
    mode = $config.default_mode
    model_ref = $config.default_model
    provider_ref = $providerRef
    workspace_ref = $config.default_workspace
    context_hints = @{ entry_origin = "cli"; consistency_probe = "true" }
  }

  $startAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $accepted = Invoke-JsonPost -Url ($gatewayBase + "/api/v1/chat/run") -Body @{
    request_id = $requestId
    run_id = $runId
    session_id = $sessionId
    trace_id = $traceId
    user_input = $userInput
    mode = $config.default_mode
    model = $config.default_model
    workspace = $config.default_workspace
    context_hints = @{ entry_origin = "gateway"; consistency_probe = "true" }
  }
  $logsUrl = $gatewayBase + "/api/v1/logs?session_id=" + [Uri]::EscapeDataString($sessionId) + "&run_id=" + [Uri]::EscapeDataString($runId) + "&limit=300"
  $gatewayLogs = Wait-GatewayTerminal -LogsUrl $logsUrl -RunId $runId -Since $startAt

  $runtimeEvents = @($runtimeResponse.events)
  $gatewayEvents = @($gatewayLogs | Sort-Object {[int]$_.sequence})
  $runtimeTypes = @($runtimeEvents | Select-Object -ExpandProperty event_type)
  $gatewayTypes = @($gatewayEvents | Select-Object -ExpandProperty event_type)
  $runtimeTerminal = @($runtimeEvents | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
  $gatewayTerminal = @($gatewayEvents | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)

  $idMatched = $accepted.request_id -eq $requestId -and $accepted.run_id -eq $runId -and $accepted.trace_id -eq $traceId
  $runEchoMatched = $runtimeResponse.result.run_id -eq $runId -and $runtimeResponse.result.session_id -eq $sessionId
  $allGatewayRunMatched = @($gatewayEvents | Where-Object { $_.run_id -ne $runId }).Count -eq 0
  $allGatewaySessionMatched = @($gatewayEvents | Where-Object { $_.session_id -ne $sessionId }).Count -eq 0
  $hasTerminals = $runtimeTerminal.Count -gt 0 -and $gatewayTerminal.Count -gt 0
  $terminalTypeMatched = $hasTerminals -and $runtimeTerminal[0].event_type -eq $gatewayTerminal[0].event_type
  $terminalToolMatched = $hasTerminals -and ([string]$runtimeTerminal[0].tool_name) -eq ([string]$gatewayTerminal[0].tool_name)
  $completionMatched = $hasTerminals -and $runtimeResponse.result.status -eq $gatewayTerminal[0].completion_status
  $gatewayTraceMatched = $accepted.trace_id -eq $traceId
  $identityExpected = [ordered]@{
    run_id = $runId
    session_id = $sessionId
    trace_id = $traceId
  }
  $identityDiffGroups = [ordered]@{}
  $identityMismatchDimensions = @()
  $identityMissingDimensions = @()
  $identityMismatchCount = 0
  $identityMissingCount = 0
  foreach ($field in @("run_id", "session_id", "trace_id")) {
    $expectedValue = [string]$identityExpected[$field]
    $acceptedValue = [string]$accepted.$field
    $acceptedMatched = $acceptedValue -eq $expectedValue
    $runtimeObserved = @($runtimeEvents | Where-Object { -not [string]::IsNullOrWhiteSpace([string]$_.$field) })
    $gatewayObserved = @($gatewayEvents | Where-Object { -not [string]::IsNullOrWhiteSpace([string]$_.$field) })
    $runtimeMissing = @($runtimeEvents | Where-Object { [string]::IsNullOrWhiteSpace([string]$_.$field) })
    $gatewayMissing = @($gatewayEvents | Where-Object { [string]::IsNullOrWhiteSpace([string]$_.$field) })
    $runtimeMismatches = @($runtimeObserved | Where-Object { [string]$_.$field -ne $expectedValue })
    $gatewayMismatches = @($gatewayObserved | Where-Object { [string]$_.$field -ne $expectedValue })
    $dimensionMismatchCount = $(if ($acceptedMatched) { 0 } else { 1 }) + @($runtimeMismatches).Count + @($gatewayMismatches).Count
    $dimensionMissingCount = @($runtimeMissing).Count + @($gatewayMissing).Count
    if ($dimensionMismatchCount -gt 0) {
      $identityMismatchDimensions += $field
    }
    if ($dimensionMissingCount -gt 0) {
      $identityMissingDimensions += $field
    }
    $identityMismatchCount += $dimensionMismatchCount
    $identityMissingCount += $dimensionMissingCount
    $identityDiffGroups[$field] = [ordered]@{
      expected = $expectedValue
      accepted = [ordered]@{ value = $acceptedValue; matched = $acceptedMatched }
      runtime = [ordered]@{ observed_count = @($runtimeObserved).Count; missing_count = @($runtimeMissing).Count; mismatch_count = @($runtimeMismatches).Count; mismatched_values = @($runtimeMismatches | ForEach-Object { [string]$_.$field } | Sort-Object -Unique); sample_event_ids = @($runtimeMismatches | Select-Object -First 3 | ForEach-Object { [string]$_.event_id }); sample_missing_event_ids = @($runtimeMissing | Select-Object -First 3 | ForEach-Object { [string]$_.event_id }) }
      gateway = [ordered]@{ observed_count = @($gatewayObserved).Count; missing_count = @($gatewayMissing).Count; mismatch_count = @($gatewayMismatches).Count; mismatched_values = @($gatewayMismatches | ForEach-Object { [string]$_.$field } | Sort-Object -Unique); sample_event_ids = @($gatewayMismatches | Select-Object -First 3 | ForEach-Object { [string]$_.event_id }); sample_missing_event_ids = @($gatewayMissing | Select-Object -First 3 | ForEach-Object { [string]$_.event_id }) }
      matched = $dimensionMismatchCount -eq 0
      mismatch_count = $dimensionMismatchCount
      missing_count = $dimensionMissingCount
    }
  }
  $identityDiffSummary = [ordered]@{
    all_matched = @($identityMismatchDimensions).Count -eq 0
    mismatch_dimensions = @($identityMismatchDimensions)
    mismatch_count = $identityMismatchCount
    missing_dimensions = @($identityMissingDimensions)
    missing_count = $identityMissingCount
  }
  $identityDiffSeverity = "ok"
  if ($identityMismatchCount -gt 0) {
    $identityDiffSeverity = "error"
  } elseif ($identityMissingCount -gt 0) {
    $identityDiffSeverity = "warn"
  }
  $identityDiffSummary.severity = $identityDiffSeverity
  $passed = $idMatched -and $runEchoMatched -and $allGatewayRunMatched -and $allGatewaySessionMatched -and $terminalTypeMatched -and $terminalToolMatched -and $completionMatched -and $gatewayTraceMatched

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    gateway = $gatewayBase
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    run_identity = [ordered]@{ request_id = $requestId; run_id = $runId; session_id = $sessionId; trace_id = $traceId }
    checks = [ordered]@{
      accepted_id_matched = $idMatched
      runtime_result_matched = $runEchoMatched
      all_gateway_run_matched = $allGatewayRunMatched
      all_gateway_session_matched = $allGatewaySessionMatched
      terminal_type_matched = $terminalTypeMatched
      terminal_tool_matched = $terminalToolMatched
      completion_status_matched = $completionMatched
      gateway_trace_matched = $gatewayTraceMatched
      identity_diff_all_matched = $identityDiffSummary.all_matched
      identity_diff_severity = $identityDiffSummary.severity
    }
    identity_diff_summary = $identityDiffSummary
    identity_diff_groups = $identityDiffGroups
    runtime = [ordered]@{ event_types = $runtimeTypes; terminal = $(if ($runtimeTerminal.Count -gt 0) { $runtimeTerminal[0] } else { $null }) }
    gateway_run = [ordered]@{ accepted = $accepted; event_types = $gatewayTypes; terminal = $(if ($gatewayTerminal.Count -gt 0) { $gatewayTerminal[0] } else { $null }) }
    artifacts = [ordered]@{
      report = $outFile
      runtime_log = $runtimeLog
      gateway_log = $gatewayLog
      runtime_build_stdout = $runtimeBuildOut
      runtime_build_stderr = $runtimeBuildErr
      gateway_build_stdout = $gatewayBuildOut
      gateway_build_stderr = $gatewayBuildErr
    }
  }

  $report | ConvertTo-Json -Depth 10 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) { throw "stage-e consistency acceptance failed: $outFile" }
  Write-Output $outFile
} finally {
  Stop-LoggedProcess -Process $gatewayProc
  Stop-LoggedProcess -Process $runtimeProc
  Cleanup-ProcessEvents
}
