# Generates a self-signed TLS certificate for the edge proxy (testing only)
param(
  [string]$CommonName = "localhost",
  [int]$Days = 365
)

$certDir = Join-Path $PSScriptRoot 'certs'
New-Item -ItemType Directory -Force -Path $certDir | Out-Null

$container = 'alpine:3'

docker pull $container | Out-Null

$subject = "/C=US/ST=Local/L=Local/O=Axionax/OU=Dev/CN=$CommonName"
$san = "subjectAltName=DNS:$CommonName,IP:127.0.0.1"

# Build Linux shell command (quoted for sh -lc). Install openssl first.
$linuxCmd = "apk add --no-cache openssl >/dev/null && openssl req -x509 -newkey rsa:2048 -sha256 -days $Days -nodes -keyout /work/privkey.pem -out /work/fullchain.pem -subj '$subject' -addext '$san'"

# Use ${} to avoid ':' parsing in PowerShell and ensure proper volume mapping
docker run --rm -v "${certDir}:/work" $container sh -lc $linuxCmd

Write-Host "Generated certs at $certDir"
