$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot

Push-Location (Join-Path $root "gateway")
try {
  $env:LOCAL_AGENT_REPO_ROOT = $root
  go run ./cmd/launcher
} finally {
  Pop-Location
}
