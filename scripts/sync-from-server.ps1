param(
  [string]$ServerIp = "175.178.90.193",
  [string]$ServerUser = "root",
  [string]$KeyPath = "$HOME\.ssh\labsafe_new.pem",
  [string]$RemoteRepoPath = "/opt/hermes",
  [switch]$KeepLocalBackup
)

$ErrorActionPreference = "Stop"

$requiredCommands = @("tar", "ssh", "scp")
foreach ($cmd in $requiredCommands) {
  if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
    throw "Required command not found: $cmd"
  }
}

$localRepoPath = Split-Path -Parent $PSScriptRoot
$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$archive = Join-Path $env:TEMP "hermes-pull-$stamp.tar.gz"
$remoteArchive = "/tmp/hermes-pull-$stamp.tar.gz"
$localBackup = "$localRepoPath.bak.$stamp"

if (-not (Test-Path $KeyPath)) {
  throw "SSH key not found: $KeyPath"
}

try {
  $remoteCmd = @"
set -e
if [ ! -d \"$RemoteRepoPath\" ]; then
  echo \"Remote path not found: $RemoteRepoPath\" >&2
  exit 2
fi
tar -C \"$RemoteRepoPath\" -czf \"$remoteArchive\" .
echo \"REMOTE_ARCHIVE_OK file=$remoteArchive\"
"@

  Write-Host "Packing server repo: $RemoteRepoPath"
  ssh -i $KeyPath -o StrictHostKeyChecking=accept-new "$ServerUser@$ServerIp" "bash -lc '$remoteCmd'"

  Write-Host "Downloading archive: $archive"
  scp -i $KeyPath -o StrictHostKeyChecking=accept-new "${ServerUser}@${ServerIp}:$remoteArchive" $archive | Out-Null

  Write-Host "Cleaning server temp archive"
  ssh -i $KeyPath -o StrictHostKeyChecking=accept-new "$ServerUser@$ServerIp" "rm -f $remoteArchive"

  if (Test-Path $localRepoPath) {
    Rename-Item -Path $localRepoPath -NewName (Split-Path $localBackup -Leaf)
  }
  New-Item -ItemType Directory -Path $localRepoPath | Out-Null

  Write-Host "Extracting into local repo: $localRepoPath"
  tar -C $localRepoPath -xzf $archive

  if (-not (Test-Path (Join-Path $localRepoPath ".git"))) {
    throw "Local restore failed: .git not found in $localRepoPath"
  }
} finally {
  if (Test-Path $archive) {
    Remove-Item $archive -Force
  }
}

if (-not $KeepLocalBackup -and (Test-Path $localBackup)) {
  Remove-Item $localBackup -Recurse -Force
  Write-Host "Local backup removed: $localBackup"
} else {
  Write-Host "Local backup kept: $localBackup"
}

Write-Host "Sync from server completed."
