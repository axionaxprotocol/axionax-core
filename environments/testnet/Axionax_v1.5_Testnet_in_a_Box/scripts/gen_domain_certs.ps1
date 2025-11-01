# Generate self-signed certificates for each domain
# Usage: .\gen_domain_certs.ps1

$domains = @("axionax.org", "testnet.axionax.org")
$certsRoot = "$PSScriptRoot\..\reverse-proxy\certs"

foreach ($domain in $domains) {
    $domainPath = Join-Path $certsRoot $domain
    
    Write-Host "Generating self-signed cert for $domain ..." -ForegroundColor Cyan
    
    # Run openssl in Alpine container to generate cert
    docker run --rm -v "${domainPath}:/certs" alpine:latest sh -c @"
apk add --no-cache openssl && \
openssl req -x509 -nodes -newkey rsa:2048 \
  -keyout /certs/privkey.pem \
  -out /certs/fullchain.pem \
  -days 365 \
  -subj '/CN=$domain/O=Axionax Testnet/C=TH' \
  -addext 'subjectAltName=DNS:$domain,DNS:*.$domain'
"@
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Generated cert for $domain" -ForegroundColor Green
    } else {
        Write-Host "✗ Failed to generate cert for $domain" -ForegroundColor Red
    }
}

Write-Host "`nDone! Certificates generated in:" -ForegroundColor Yellow
foreach ($domain in $domains) {
    Write-Host "  - reverse-proxy/certs/$domain/"
}
