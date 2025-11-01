# Axionax Core - Simple Test Script
Write-Host "Axionax Core - Quick Test" -ForegroundColor Cyan
Write-Host "================================`n" -ForegroundColor Cyan

# 1. ตรวจสอบ Binary
Write-Host "Step 1: ตรวจสอบ Axionax Core Binary" -ForegroundColor Green
if (Test-Path ".\build\axionax-core.exe") {
    Write-Host "OK - Binary found" -ForegroundColor Green
    .\build\axionax-core.exe version
} else {
    Write-Host "ERROR - Binary not found!" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 2. ทดสอบ Configuration
Write-Host "Step 2: ทดสอบ Configuration" -ForegroundColor Green
.\build\axionax-core.exe config init
Write-Host ""

# 3. ทดสอบ Key Generation
Write-Host "Step 3: ทดสอบ Key Management" -ForegroundColor Green
.\build\axionax-core.exe keys generate --type validator
Write-Host ""

# 4. ทดสอบ Status Commands
Write-Host "Step 4: ทดสอบ Status Commands" -ForegroundColor Green
.\build\axionax-core.exe validator status
Write-Host ""
.\build\axionax-core.exe worker status
Write-Host ""

# 5. ตรวจสอบ Docker
Write-Host "Step 5: ตรวจสอบ Docker" -ForegroundColor Green
$dockerCmd = Get-Command docker -ErrorAction SilentlyContinue
if ($dockerCmd) {
    Write-Host "OK - Docker installed" -ForegroundColor Green
    $dockerStatus = docker ps 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "OK - Docker Engine running" -ForegroundColor Green
        docker ps --format "table {{.Names}}\t{{.Status}}"
    } else {
        Write-Host "WARN - Docker Engine not running" -ForegroundColor Yellow
    }
} else {
    Write-Host "WARN - Docker not installed" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "================================" -ForegroundColor Cyan
Write-Host "การทดสอบเสร็จสมบูรณ์!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "คำสั่งที่เป็นประโยชน์:" -ForegroundColor Cyan
Write-Host "  .\build\axionax-core.exe start --network testnet" -ForegroundColor White
Write-Host "  .\build\axionax-core.exe validator start" -ForegroundColor White
Write-Host "  .\build\axionax-core.exe --help" -ForegroundColor White
Write-Host ""
Write-Host "เริ่มต้น Testnet:" -ForegroundColor Cyan
Write-Host "  cd Axionax_v1.5_Testnet_in_a_Box" -ForegroundColor White
Write-Host "  powershell -ExecutionPolicy Bypass -File start-testnet.ps1" -ForegroundColor White
Write-Host ""
