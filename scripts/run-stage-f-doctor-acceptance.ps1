$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$root = Split-Path -Parent $PSScriptRoot
$outDir = Join-Path $root "tmp\stage-f-doctor"
$outFile = Join-Path $outDir "latest.json"
$logDir = Join-Path $outDir "logs"
$binDir = Join-Path $outDir "bin"
$cargoTargetDir = Join-Path $outDir "cargo-target"
$runtimeExe = Join-Path $cargoTargetDir "debug\runtime-host.exe"
$gatewayExe = Join-Path $binDir "gateway-stage-f-doctor.exe"
$runtimeLog = Join-Path $logDir "runtime.log"
$gatewayLog = Join-Path $logDir "gateway.log"
$doctorScript = Join-Path $PSScriptRoot "doctor.ps1"
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
  if (-not (Wait-HttpReady -Url ("http://127.0.0.1:" + $gatewayPort + "/health"))) { throw "gateway not ready" }

  $doctorJson = powershell -ExecutionPolicy Bypass -File $doctorScript -RepoRoot $root -GatewayPort $gatewayPort -RuntimePort $runtimePort -OutFile $outFile -RequirePass
  $doctor = $doctorJson | ConvertFrom-Json
  $checks = $doctor.checks
  $passed = $doctor.status -eq "passed" -and
    [bool]$checks.go_available -and
    [bool]$checks.rust_available -and
    [bool]$checks.node_available -and
    [bool]$checks.npm_available -and
    [bool]$checks.config_exists -and
    [bool]$checks.ports_valid -and
    [bool]$checks.frontend_dist_exists -and
    [bool]$checks.runtime_health_ok -and
    [bool]$checks.gateway_health_ok -and
    [bool]$checks.logs_writable

  $report = [ordered]@{
    checked_at = (Get-Date).ToString("o")
    status = $(if ($passed) { "passed" } else { "failed" })
    ports = [ordered]@{ gateway = $gatewayPort; runtime = $runtimePort }
    doctor = $doctor
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
  Write-Output $outFile
  if (-not $passed) { throw "stage-f doctor acceptance failed: $outFile" }
} finally {
  Stop-LoggedProcess -Process $gatewayProc
  Stop-LoggedProcess -Process $runtimeProc
  Cleanup-ProcessEvents
}
