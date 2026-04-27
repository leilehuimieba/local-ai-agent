param(
  [ValidateSet("debug", "release")]
  [string]$Profile = "release",
  [string]$OutputDir = "./build"
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$repoRoot = Split-Path -Parent $PSScriptRoot
$cargoTarget = Join-Path $OutputDir "cargo-target"
$frontendDir = Join-Path $repoRoot "frontend"

function Assert-CommandAvailable {
  param([string]$Name)
  if (-not (Get-Command -Name $Name -ErrorAction SilentlyContinue)) {
    throw "missing required command: $Name"
  }
}

function Invoke-LoggedCommand {
  param([string]$WorkDir, [string]$OutPath, [string]$ErrPath, [string]$File, [string[]]$Arguments)
  $prev = $ErrorActionPreference
  $ErrorActionPreference = "Continue"
  Push-Location $WorkDir
  try {
    & $File @Arguments 1> $OutPath 2> $ErrPath
  } finally {
    Pop-Location
  }
  $ErrorActionPreference = $prev
  if ($LASTEXITCODE -ne 0) { throw "$File failed: $($Arguments -join ' ')" }
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $OutputDir "gateway") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $OutputDir "frontend\dist") | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $OutputDir "config") | Out-Null

# Frontend
Assert-CommandAvailable -Name "npm"
Invoke-LoggedCommand -WorkDir $frontendDir -OutPath (Join-Path $OutputDir "frontend-build.stdout.log") -ErrPath (Join-Path $OutputDir "frontend-build.stderr.log") -File "npm" -Arguments @("install")
Invoke-LoggedCommand -WorkDir $frontendDir -OutPath (Join-Path $OutputDir "frontend-build.stdout.log") -ErrPath (Join-Path $OutputDir "frontend-build.stderr.log") -File "npm" -Arguments @("run", "build")
Copy-Item -Recurse -Force -Path (Join-Path $frontendDir "dist\*") -Destination (Join-Path $OutputDir "frontend\dist")

# Rust runtime
Assert-CommandAvailable -Name "cargo"
$cargoProfile = if ($Profile -eq "release") { @("--release") } else { @() }
Invoke-LoggedCommand -WorkDir $repoRoot -OutPath (Join-Path $OutputDir "runtime-build.stdout.log") -ErrPath (Join-Path $OutputDir "runtime-build.stderr.log") -File "cargo" -Arguments (@("build", "-p", "runtime-host", "--target-dir", $cargoTarget) + $cargoProfile)

# Go gateway
Assert-CommandAvailable -Name "go"
Invoke-LoggedCommand -WorkDir (Join-Path $repoRoot "gateway") -OutPath (Join-Path $OutputDir "gateway-server-build.stdout.log") -ErrPath (Join-Path $OutputDir "gateway-server-build.stderr.log") -File "go" -Arguments @("build", "-o", (Join-Path $OutputDir "gateway\server.exe"), "./cmd/server")
Invoke-LoggedCommand -WorkDir (Join-Path $repoRoot "gateway") -OutPath (Join-Path $OutputDir "gateway-launcher-build.stdout.log") -ErrPath (Join-Path $OutputDir "gateway-launcher-build.stderr.log") -File "go" -Arguments @("build", "-o", (Join-Path $OutputDir "gateway\launcher.exe"), "./cmd/launcher")

# Config
Copy-Item -Force -Path (Join-Path $repoRoot "config\app.json") -Destination (Join-Path $OutputDir "config\app.json")
Copy-Item -Force -Path (Join-Path $repoRoot "gateway\go.mod") -Destination (Join-Path $OutputDir "gateway\go.mod")

# BUILD_INFO
$buildInfo = [ordered]@{
  built_at = (Get-Date).ToString("o")
  profile = $Profile
  cargo_target = $cargoTarget
  output_dir = (Resolve-Path $OutputDir).Path
}
Set-Content -Path (Join-Path $OutputDir "BUILD_INFO.txt") -Value ($buildInfo | ConvertTo-Json -Depth 3) -Encoding UTF8

Write-Host "Build complete: $OutputDir"
