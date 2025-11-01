# Generate per-domain self-signed TLS certs for quick bring-up (testing only)
param(
  [Parameter(Mandatory=$true)][string]$Domain,
  [int]$Days = 90
)

$base = Join-Path $PSScriptRoot 'certs'
$certDir = Join-Path $base $Domain
New-Item -ItemType Directory -Force -Path $certDir | Out-Null

$container = 'alpine:3'

docker pull $container | Out-Null

$subject = "/C=US/ST=Local/L=Local/O=Axionax/OU=Dev/CN=$Domain"
$san = "subjectAltName=DNS:$Domain,IP:127.0.0.1"

$linuxCmd = "apk add --no-cache openssl >/dev/null && openssl req -x509 -newkey rsa:2048 -sha256 -days $Days -nodes -keyout /work/privkey.pem -out /work/fullchain.pem -subj '$subject' -addext '$san'"

docker run --rm -v "${certDir}:/work" $container sh -lc $linuxCmd

Write-Host "Generated certs at $certDir"
