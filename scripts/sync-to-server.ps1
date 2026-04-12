param(
  [string]$ServerIp = "175.178.90.193",
  [string]$ServerUser = "root",
  [string]$KeyPath = "$HOME\.ssh\labsafe_new.pem",
  [string]$RemoteRepoPath = "/opt/hermes",
  [int]$KeepRemoteBackups = 3
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
$archive = Join-Path $env:TEMP "hermes-push-$stamp.tar.gz"
$remoteArchive = "/tmp/hermes-push-$stamp.tar.gz"
$remoteBackup = "$RemoteRepoPath.bak.$stamp"
$pruneStart = $KeepRemoteBackups + 1

if (-not (Test-Path $KeyPath)) {
  throw "SSH key not found: $KeyPath"
}

try {
  Write-Host "Packing local repo: $localRepoPath"
  tar -C $localRepoPath -czf $archive .

  Write-Host "Uploading archive to server: $ServerIp"
  scp -i $KeyPath -o StrictHostKeyChecking=accept-new $archive "${ServerUser}@${ServerIp}:$remoteArchive" | Out-Null

  $remoteCmd = @"
set -e
if [ -d \"$RemoteRepoPath\" ]; then
  mv \"$RemoteRepoPath\" \"$remoteBackup\"
fi
mkdir -p \"$RemoteRepoPath\"
tar -C \"$RemoteRepoPath\" -xzf \"$remoteArchive\"
rm -f \"$remoteArchive\"
if [ ! -d \"$RemoteRepoPath/.git\" ]; then
  echo \"REMOTE_SYNC_FAIL: .git not found under $RemoteRepoPath\" >&2
  exit 3
fi
if [ $KeepRemoteBackups -ge 0 ]; then
  ls -dt \"$RemoteRepoPath\".bak.* 2>/dev/null | tail -n +$pruneStart | xargs -r rm -rf
fi
echo \"REMOTE_SYNC_OK path=$RemoteRepoPath backup=$remoteBackup\"
"@

  Write-Host "Extracting on server: $RemoteRepoPath"
  ssh -i $KeyPath -o StrictHostKeyChecking=accept-new "$ServerUser@$ServerIp" "bash -lc '$remoteCmd'"
} finally {
  if (Test-Path $archive) {
    Remove-Item $archive -Force
  }
}

Write-Host "Sync to server completed."
