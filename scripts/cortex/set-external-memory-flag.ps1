param(
  [Parameter(Mandatory = $false)]
  [bool]$Enabled = $false,
  [Parameter(Mandatory = $false)]
  [string]$ServerUrl = "http://127.0.0.1:21100",
  [Parameter(Mandatory = $false)]
  [string]$AgentId = "default"
)

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$settingsDir = Join-Path $repoRoot "data\\settings"
$flagPath = Join-Path $settingsDir "external-memory-cortex.json"

if (!(Test-Path $settingsDir)) {
  New-Item -ItemType Directory -Path $settingsDir | Out-Null
}

$payload = [ordered]@{
  enabled = $Enabled
  provider = "cortex"
  server_url = $ServerUrl
  agent_id = $AgentId
  updated_at = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ssK")
}

$json = $payload | ConvertTo-Json -Depth 5
[System.IO.File]::WriteAllText($flagPath, $json, (New-Object System.Text.UTF8Encoding($false)))
Write-Output ("external-memory flag updated: enabled={0}; path={1}" -f $Enabled, $flagPath)
