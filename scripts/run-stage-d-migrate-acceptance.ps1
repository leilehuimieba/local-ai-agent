$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-d-migrate-acceptance"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTarget = Join-Path $outDir "cargo-target"
$sandbox = Join-Path $outDir "sandbox"
$outFile = Join-Path $outDir "latest.json"

$runtimeOut = Join-Path $logDir "runtime.stdout.log"
$runtimeErr = Join-Path $logDir "runtime.stderr.log"
$gatewayOut = Join-Path $logDir "gateway.stdout.log"
$gatewayErr = Join-Path $logDir "gateway.stderr.log"
$runtimeBuildOut = Join-Path $logDir "runtime-build.stdout.log"
$runtimeBuildErr = Join-Path $logDir "runtime-build.stderr.log"
$gatewayBuildOut = Join-Path $logDir "gateway-build.stdout.log"
$gatewayBuildErr = Join-Path $logDir "gateway-build.stderr.log"

$workspaceId = "migrate_ws"
$token = "D02_MIGRATE_TOKEN_20260412"

function New-FreePort {
  $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 0)
  $listener.Start()
  $port = $listener.LocalEndpoint.Port
  $listener.Stop()
  return $port
}

function Wait-HttpReady {
  param([string]$Url, [int]$Attempts = 80)
  for ($i = 0; $i -lt $Attempts; $i++) {
    try {
      $resp = Invoke-WebRequest -Uri $Url -TimeoutSec 1 -UseBasicParsing
      if ($resp.StatusCode -eq 200) { return $true }
    } catch {}
    Start-Sleep -Milliseconds 500
  }
  return $false
}

function Wait-RunTerminal {
  param([string]$LogsUrl, [string]$RunId, [int64]$Since)
  for ($i = 0; $i -lt 120; $i++) {
    $all = Invoke-RestMethod -Uri $LogsUrl
    $items = @($all.items | Where-Object { $_.run_id -eq $RunId -and [int64]$_.timestamp -ge $Since })
    $terminal = @($items | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)
    if ($terminal.Count -gt 0) { return $items }
    Start-Sleep -Milliseconds 500
  }
  throw ("run timeout: " + $RunId)
}

function Sqlite-Count {
  param([string]$DbPath, [string]$Sql)
  if (-not (Test-Path $DbPath)) { return 0 }
  $raw = & sqlite3 $DbPath $Sql 2>$null
  if ($LASTEXITCODE -ne 0) { return 0 }
  if ([string]::IsNullOrWhiteSpace($raw)) { return 0 }
  return [int]$raw.Trim()
}

function Jsonl-Lines {
  param([string]$Path)
  if (-not (Test-Path $Path)) { return 0 }
  return @((Get-Content -Path $Path | Where-Object { $_.Trim() -ne "" })).Count
}

function Write-Utf8NoBom {
  param([string]$Path, [string]$Content)
  $encoding = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null
Remove-Item -Recurse -Force $cargoTarget -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cargoTarget | Out-Null
Remove-Item -Recurse -Force $sandbox -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "config") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "data\memory") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "data\long_term_memory") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "data\knowledge_base") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $sandbox "data\storage") | Out-Null

Set-Content -Path $runtimeOut -Value "" -Encoding UTF8
Set-Content -Path $runtimeErr -Value "" -Encoding UTF8
Set-Content -Path $gatewayOut -Value "" -Encoding UTF8
Set-Content -Path $gatewayErr -Value "" -Encoding UTF8

$runtimePort = New-FreePort
$gatewayPort = New-FreePort
while ($gatewayPort -eq $runtimePort) { $gatewayPort = New-FreePort }

$config = Get-Content -Path (Join-Path $root "config\app.json") -Raw | ConvertFrom-Json
$config.gateway_port = $gatewayPort
$config.runtime_port = $runtimePort
$workspace = [PSCustomObject]@{
  workspace_id = $workspaceId
  name = "D02-Migrate-Acceptance"
  root_path = $root
  is_active = $true
}
$config.default_workspace = $workspace
$config.workspaces = @($workspace)
$configJson = $config | ConvertTo-Json -Depth 10
Write-Utf8NoBom -Path (Join-Path $sandbox "config\app.json") -Content $configJson

$legacyMemoryEntries = @(
  [ordered]@{
    id = "legacy-memory-a1"; memory_type = "project_rule"; title = "D02 migrate memory A"; summary = "$token memory summary"
    content = "$token memory content"; workspace_id = $workspaceId; session_id = "legacy-session"; source_run_id = "legacy-run"
    source = "docs/README.md"; source_type = "seed"; verified = $true; priority = 80; archived = $false
    created_at = "1775964100000"; updated_at = "1775964100000"; scope = "D02-Migrate"; timestamp = "1775964100000"
  }
  [ordered]@{
    id = "legacy-memory-a2"; memory_type = "project_rule"; title = "D02 migrate memory A"; summary = "$token memory summary"
    content = "$token memory content duplicate"; workspace_id = $workspaceId; session_id = "legacy-session"; source_run_id = "legacy-run"
    source = "docs/README.md"; source_type = "seed"; verified = $true; priority = 80; archived = $false
    created_at = "1775964100001"; updated_at = "1775964100001"; scope = "D02-Migrate"; timestamp = "1775964100001"
  }
  [ordered]@{
    id = "legacy-memory-noise"; memory_type = "project_knowledge"; title = "project explanation"
    summary = "generated project explanation noise"; content = "noise item should be dropped by migration rule"
    workspace_id = $workspaceId; session_id = "legacy-session"; source_run_id = "legacy-run"; source = "run:legacy-run"
    source_type = "runtime"; verified = $false; priority = 0; archived = $false
    created_at = "1775964100002"; updated_at = "1775964100002"; scope = "D02-Migrate"; timestamp = "1775964100002"
  }
)

$legacyLongTermEntries = @(
  [ordered]@{
    id = "legacy-memory-b1"; memory_type = "workflow_pattern"; title = "D02 migrate memory B"; summary = "$token long term summary"
    content = "$token long term content"; workspace_id = $workspaceId; session_id = "legacy-session"; source_run_id = "legacy-run"
    source = "docs/11-hermes-rebuild/stage-plans/阶段计划总表.md"; source_type = "runtime"; verified = $true; priority = 30
    archived = $false; created_at = "1775964100003"; updated_at = "1775964100003"; scope = "D02-Migrate"; timestamp = "1775964100003"
  }
)

$legacyKnowledgeEntries = @(
  [ordered]@{
    id = "legacy-knowledge-a1"; knowledge_type = "project_status"; title = "D02 migrate knowledge"
    summary = "$token knowledge summary"; content = "$token knowledge content"; tags = @("d02", "migration")
    source = "docs/README.md"; source_type = "runtime"; verified = $true; workspace_id = $workspaceId
    priority = 10; archived = $false; created_at = "1775964100004"; updated_at = "1775964100004"
  }
)

$memoryPath = Join-Path $sandbox "data\memory\entries.jsonl"
$longTermPath = Join-Path $sandbox "data\long_term_memory\$workspaceId.jsonl"
$knowledgePath = Join-Path $sandbox "data\knowledge_base\$workspaceId.jsonl"
$memoryJsonl = (($legacyMemoryEntries | ForEach-Object { ($_ | ConvertTo-Json -Compress) }) -join "`n") + "`n"
$longTermJsonl = (($legacyLongTermEntries | ForEach-Object { ($_ | ConvertTo-Json -Compress) }) -join "`n") + "`n"
$knowledgeJsonl = (($legacyKnowledgeEntries | ForEach-Object { ($_ | ConvertTo-Json -Compress) }) -join "`n") + "`n"
Write-Utf8NoBom -Path $memoryPath -Content $memoryJsonl
Write-Utf8NoBom -Path $longTermPath -Content $longTermJsonl
Write-Utf8NoBom -Path $knowledgePath -Content $knowledgeJsonl

$runtimeExe = Join-Path $cargoTarget "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-d.exe"
$runtimeProc = $null
$gatewayProc = $null

try {
  $cargoBuild = Start-Process -FilePath "cargo" -ArgumentList @("build", "-p", "runtime-host", "--target-dir", $cargoTarget) -WorkingDirectory $root -Wait -PassThru -RedirectStandardOutput $runtimeBuildOut -RedirectStandardError $runtimeBuildErr
  if ($cargoBuild.ExitCode -ne 0) { throw "cargo build failed: $($cargoBuild.ExitCode)" }

  $goBuild = Start-Process -FilePath "go" -ArgumentList @("build", "-o", $gatewayExe, "./cmd/server") -WorkingDirectory (Join-Path $root "gateway") -Wait -PassThru -RedirectStandardOutput $gatewayBuildOut -RedirectStandardError $gatewayBuildErr
  if ($goBuild.ExitCode -ne 0) { throw "go build failed: $($goBuild.ExitCode)" }

  if (-not (Test-Path $runtimeExe)) { throw "runtime-host binary not found" }
  if (-not (Test-Path $gatewayExe)) { throw "gateway binary not found" }

  $env:LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort
  $runtimeProc = Start-Process -FilePath $runtimeExe -WorkingDirectory $root -PassThru -RedirectStandardOutput $runtimeOut -RedirectStandardError $runtimeErr
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:$runtimePort/health") -Attempts 120)) { throw "runtime not ready" }

  $env:LOCAL_AGENT_GATEWAY_PORT = [string]$gatewayPort
  $env:LOCAL_AGENT_RUNTIME_PORT = [string]$runtimePort
  $gatewayProc = Start-Process -FilePath $gatewayExe -WorkingDirectory $sandbox -PassThru -RedirectStandardOutput $gatewayOut -RedirectStandardError $gatewayErr
  $gateway = "http://127.0.0.1:$gatewayPort"
  if (-not (Wait-HttpReady -Url ($gateway + "/health") -Attempts 120)) { throw "gateway not ready" }

  $dbPath = Join-Path $sandbox "data\storage\main.db"
  $memoryCountBefore = 0
  $sqliteMemoryBefore = Sqlite-Count -DbPath $dbPath -Sql "select count(1) from long_term_memory where workspace_id='$workspaceId';"
  $sqliteKnowledgeBefore = Sqlite-Count -DbPath $dbPath -Sql "select count(1) from knowledge_base where workspace_id='$workspaceId';"
  $memoryLinesBefore = Jsonl-Lines -Path $memoryPath

  $runUrl = $gateway + "/api/v1/chat/run"
  $logsUrl = $gateway + "/api/v1/logs"
  $sessionId = "stage-d-migrate-" + [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $startedAt = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
  $payload = @{
    session_id = $sessionId
    user_input = "recall: $token"
    mode = "standard"
    model = $config.default_model
    workspace = $workspace
  } | ConvertTo-Json -Depth 10

  $runAccepted = Invoke-RestMethod -Uri $runUrl -Method Post -ContentType "application/json; charset=utf-8" -Body $payload
  $logs = Wait-RunTerminal -LogsUrl $logsUrl -RunId $runAccepted.run_id -Since $startedAt
  $memoryRecalled = @($logs | Where-Object { $_.event_type -eq "memory_recalled" } | Select-Object -Last 1)
  $terminal = @($logs | Where-Object { $_.event_type -eq "run_finished" -or $_.event_type -eq "run_failed" } | Select-Object -Last 1)

  $memoryApiAfter = Invoke-RestMethod -Uri ($gateway + "/api/v1/memories")
  $memoryCountAfter = @($memoryApiAfter.items).Count
  $sqliteMemoryAfter = Sqlite-Count -DbPath $dbPath -Sql "select count(1) from long_term_memory where workspace_id='$workspaceId';"
  $sqliteKnowledgeAfter = Sqlite-Count -DbPath $dbPath -Sql "select count(1) from knowledge_base where workspace_id='$workspaceId';"
  $sqliteTokenCount = Sqlite-Count -DbPath $dbPath -Sql "select count(1) from long_term_memory where workspace_id='$workspaceId' and (summary like '%$token%' or content like '%$token%');"
  $memoryLinesAfter = Jsonl-Lines -Path $memoryPath
  $longTermLinesAfter = Jsonl-Lines -Path $longTermPath
  $knowledgeLinesAfter = Jsonl-Lines -Path $knowledgePath
  $memoryJsonlRaw = Get-Content -Path $memoryPath -Raw
  $legacyDuplicateRemoved = $memoryJsonlRaw -notlike "*legacy-memory-a2*"

  $terminalOk = $terminal.Count -gt 0 -and $terminal[0].event_type -eq "run_finished"
  $memoryApiHasToken = @($memoryApiAfter.items | Where-Object { [string]$_.summary -like "*$token*" -or [string]$_.content -like "*$token*" }).Count -ge 1
  $memoryRecallHit = $memoryApiHasToken -and $sqliteTokenCount -ge 1
  $memoryImported = $memoryCountBefore -eq 0 -and $memoryCountAfter -ge 1 -and $sqliteMemoryBefore -eq 0 -and $sqliteMemoryAfter -ge 1
  $knowledgeImported = $sqliteKnowledgeBefore -eq 0 -and $sqliteKnowledgeAfter -ge 1
  $legacyCompacted = $legacyDuplicateRemoved -and $longTermLinesAfter -ge 1 -and $knowledgeLinesAfter -ge 1
  $passed = $terminalOk -and $memoryRecallHit -and $memoryImported -and $knowledgeImported -and $legacyCompacted

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    workspace_id = $workspaceId
    token = $token
    gateway = $gateway
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    run = [ordered]@{
      session_id = $sessionId
      run_id = $runAccepted.run_id
      terminal_event = $(if ($terminal.Count -gt 0) { $terminal[0].event_type } else { "" })
      event_types = @($logs | Select-Object -ExpandProperty event_type)
      memory_recalled_event = $(if ($memoryRecalled.Count -gt 0) { $memoryRecalled[0] } else { $null })
      memory_recall_hit = $memoryRecallHit
    }
    migration = [ordered]@{
      memory_api_before = $memoryCountBefore
      memory_api_after = $memoryCountAfter
      sqlite_memory_before = $sqliteMemoryBefore
      sqlite_memory_after = $sqliteMemoryAfter
      sqlite_knowledge_before = $sqliteKnowledgeBefore
      sqlite_knowledge_after = $sqliteKnowledgeAfter
      sqlite_token_count = $sqliteTokenCount
      memory_jsonl_lines_before = $memoryLinesBefore
      memory_jsonl_lines_after = $memoryLinesAfter
      long_term_jsonl_lines_after = $longTermLinesAfter
      knowledge_jsonl_lines_after = $knowledgeLinesAfter
      memory_api_has_token = $memoryApiHasToken
      legacy_duplicate_removed = $legacyDuplicateRemoved
      memory_imported = $memoryImported
      knowledge_imported = $knowledgeImported
      legacy_compacted = $legacyCompacted
    }
    artifacts = [ordered]@{
      report = $outFile
      db_path = $dbPath
      sandbox_config = (Join-Path $sandbox "config\app.json")
      memory_jsonl = $memoryPath
      long_term_jsonl = $longTermPath
      knowledge_jsonl = $knowledgePath
      runtime_stdout = $runtimeOut
      runtime_stderr = $runtimeErr
      gateway_stdout = $gatewayOut
      gateway_stderr = $gatewayErr
      runtime_build_stdout = $runtimeBuildOut
      runtime_build_stderr = $runtimeBuildErr
      gateway_build_stdout = $gatewayBuildOut
      gateway_build_stderr = $gatewayBuildErr
    }
  }

  $report | ConvertTo-Json -Depth 12 | Set-Content -Path $outFile -Encoding UTF8
  if (-not $passed) { throw "stage-d migrate acceptance failed, see $outFile" }
  Write-Output $outFile
}
finally {
  if ($gatewayProc -and -not $gatewayProc.HasExited) {
    Stop-Process -Id $gatewayProc.Id -Force
    $gatewayProc.WaitForExit()
  }
  if ($runtimeProc -and -not $runtimeProc.HasExited) {
    Stop-Process -Id $runtimeProc.Id -Force
    $runtimeProc.WaitForExit()
  }
}
