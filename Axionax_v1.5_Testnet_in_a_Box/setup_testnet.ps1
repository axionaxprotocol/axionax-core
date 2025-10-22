# Axionax v1.5 - Testnet_in_a_Box Auto-Setup (ASCII-only)
$ErrorActionPreference = 'Stop'

function T($m){ Write-Host "`n=== $m ===" -ForegroundColor Cyan }
function Have($c){ $null -ne (Get-Command $c -ErrorAction SilentlyContinue) }
function NeedAdmin {
  $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).
    IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
  if(-not $isAdmin){ throw "Please run PowerShell as Administrator." }
}

NeedAdmin
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $Root
T "Axionax v1.5 Auto Setup (Windows)"

# --- System checks ---
T "System checks"
try {
  $cpu = Get-CimInstance Win32_Processor
  if($cpu -and ($cpu.VirtualizationFirmwareEnabled -eq $false)){
    Write-Warning "CPU virtualization may be disabled in BIOS/UEFI (enable VT-x/AMD-V if Docker fails)."
  }
} catch {}

# --- Winget ---
function HaveWinget { Have 'winget.exe' }
if(-not (HaveWinget)){
  Write-Warning "winget not found. If Docker is missing, install Docker Desktop manually from Microsoft Store."
}

# --- WSL2 ---
T "WSL2"
if(-not (Have 'wsl.exe')){
  Write-Host "Installing WSL (Ubuntu)..."
  wsl --install -d Ubuntu
  Write-Warning "Windows may require a reboot. After reboot, run this script again."
  exit 0
}else{
  try { wsl --set-default-version 2 | Out-Null } catch {}
  try { & wsl.exe --status | Out-Null } catch {}
}

# --- Docker Desktop ---
T "Docker Desktop"
if(-not (Have 'docker')){
  if(HaveWinget){
    winget install --id Docker.DockerDesktop -e --accept-source-agreements --accept-package-agreements
  } else {
    throw "Docker not found and winget unavailable. Install Docker Desktop manually, then re-run."
  }
}else{
  Write-Host "Docker found."
}

# Enable WSL engine in Docker settings (best effort)
$settings = Join-Path $env:APPDATA "Docker\settings.json"
try {
  if(Test-Path $settings){
    $json = Get-Content $settings | ConvertFrom-Json
    if($null -eq $json.wslEngineEnabled -or $json.wslEngineEnabled -ne $true){
      $json.wslEngineEnabled = $true
      ($json | ConvertTo-Json -Depth 12) | Set-Content $settings -Encoding UTF8
      Write-Host "Enabled Docker WSL engine."
    }
  }
} catch { Write-Warning "Could not update Docker settings (safe to ignore)." }

# Start Docker Desktop and wait until ready
Write-Host "Starting Docker Desktop (if not running)..."
$dockerExe = Join-Path $env:ProgramFiles "Docker\Docker\Docker Desktop.exe"
if(Test-Path $dockerExe){ Start-Process -FilePath $dockerExe -ErrorAction SilentlyContinue | Out-Null }

$retries = 90
while($retries -gt 0){
  Start-Sleep -Seconds 2
  if(Have 'docker'){
    try { docker version | Out-Null; break } catch {}
  }
  $retries--
}
if($retries -eq 0){ throw "Docker is not ready. Open Docker Desktop manually and run this script again." }
Write-Host "Docker is ready."

# --- Git (optional) ---
T "Git"
if(-not (Have 'git')){
  if(HaveWinget){
    winget install --id Git.Git -e --accept-source-agreements --accept-package-agreements
  } else {
    Write-Warning "Git not found (not required for basic run)."
  }
}else{ Write-Host "Git found." }

# --- Project .env ---
T "Project .env"
$envFile   = Join-Path $Root ".env"
$envSample = Join-Path $Root ".env.sample"
if(-not (Test-Path $envFile)){
  if(Test-Path $envSample){ Copy-Item $envSample $envFile }
  else { Set-Content $envFile "CHAIN_ID=8615`nDEPLOYER_PRIVATE_KEY=`nFAUCET_PRIVATE_KEY=`n" -Encoding ASCII }
}

Add-Type -AssemblyName System.Security
function NewTestKey {
  $b = New-Object byte[] 32
  [System.Security.Cryptography.RandomNumberGenerator]::Fill($b)
  '0x' + ($b | ForEach-Object { $_.ToString('x2') }) -join ''
}
$c = Get-Content $envFile
if(-not ($c -match '^CHAIN_ID=')){ $c += "`nCHAIN_ID=8615" }
if(-not ($c -match '^DEPLOYER_PRIVATE_KEY=')){ $c += "`nDEPLOYER_PRIVATE_KEY=$(NewTestKey)" }
if(-not ($c -match '^FAUCET_PRIVATE_KEY=')){ $c += "`nFAUCET_PRIVATE_KEY=$(NewTestKey)" }
Set-Content $envFile ($c -join "`n") -Encoding ASCII
Write-Host ".env prepared (TEST keys generated)."

# --- Docker Compose Up ---
T "Compose Up"
$useV2 = $true
try { docker compose version | Out-Null } catch { $useV2 = $false }
if($useV2){
  docker compose pull
  docker compose up -d
}elseif(Have 'docker-compose'){
  docker-compose pull
  docker-compose up -d
}else{
  throw "docker compose not found."
}

# --- Health checks ---
T "Health checks"
function JsonRpc($url, $method){
  $payload = @{ jsonrpc="2.0"; id=1; method=$method; params=@() } | ConvertTo-Json -Compress
  try { Invoke-RestMethod -Uri $url -Method POST -Body $payload -ContentType "application/json" -TimeoutSec 5 } catch { return $null }
}
$ip = "127.0.0.1"

# wait for services
for($i=0; $i -lt 20; $i++){
  $rpc = JsonRpc "http://$ip:8545" "eth_blockNumber"
  if($rpc){ break } else { Start-Sleep 3 }
}
try { $faucet = Invoke-RestMethod -Uri "http://$ip:8081/health" -TimeoutSec 5 } catch { $faucet = $null }

if($rpc){ Write-Host ("RPC OK: " + $rpc.result) -ForegroundColor Green } else { Write-Warning "RPC not responding yet." }
if($faucet){ Write-Host ("Faucet OK: " + $faucet) -ForegroundColor Green } else { Write-Warning "Faucet not responding yet." }

T "Ready"
Write-Host ('RPC:      http://{0}:8545' -f $ip)
Write-Host ('Faucet:   http://{0}:8081    (GET /health, /request?address=0x...)' -f $ip)
Write-Host ('Explorer: http://{0}:3000    (if included in compose)' -f $ip)
Write-Host 'Add the RPC to MetaMask (ChainID=8615, symbol=AXX) and request test tokens.'
