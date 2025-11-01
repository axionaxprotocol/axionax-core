# Axionax Core - Quick Test Script
Write-Host "Axionax Core - Quick Test" -ForegroundColor Cyan
Write-Host "==========================" -ForegroundColor Cyan
Write-Host ""

# 1. Check Binary
Write-Host "Step 1: Check Axionax Core Binary" -ForegroundColor Green
if (Test-Path ".\build\axionax-core.exe") {
    Write-Host "  OK - Binary found" -ForegroundColor Green
    .\build\axionax-core.exe version
} else {
    Write-Host "  ERROR - Binary not found!" -ForegroundColor Red
    Write-Host "  Run: go build -o build\axionax-core.exe .\cmd\axionax" -ForegroundColor Yellow
    exit 1
}

Write-Host ""

# 2. Test Configuration
Write-Host "Step 2: Test Configuration" -ForegroundColor Green
.\build\axionax-core.exe config init
Write-Host ""

# 3. Test Key Generation
Write-Host "Step 3: Test Key Management" -ForegroundColor Green
.\build\axionax-core.exe keys generate --type validator
Write-Host ""

# 4. Test Status Commands
Write-Host "Step 4: Test Status Commands" -ForegroundColor Green
Write-Host "  Validator Status:" -ForegroundColor Yellow
.\build\axionax-core.exe validator status
Write-Host ""

Write-Host "  Worker Status:" -ForegroundColor Yellow
.\build\axionax-core.exe worker status
Write-Host ""

# 5. Check Docker
Write-Host "Step 5: Check Docker" -ForegroundColor Green
$dockerCmd = Get-Command docker -ErrorAction SilentlyContinue
if ($dockerCmd) {
    Write-Host "  OK - Docker installed" -ForegroundColor Green
    $null = docker ps 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  OK - Docker Engine running" -ForegroundColor Green
        Write-Host ""
        Write-Host "  Running Containers:" -ForegroundColor Yellow
        docker ps --format "table {{.Names}}\t{{.Status}}"
    } else {
        Write-Host "  WARN - Docker Engine not running" -ForegroundColor Yellow
        Write-Host "  Please start Docker Desktop to run full testnet" -ForegroundColor Yellow
    }
} else {
    Write-Host "  WARN - Docker not installed" -ForegroundColor Yellow
    Write-Host "  Demo mode available without Docker" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "==========================" -ForegroundColor Cyan
Write-Host "Test Complete!" -ForegroundColor Green
Write-Host "==========================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Useful Commands:" -ForegroundColor Cyan
Write-Host "  .\build\axionax-core.exe start --network testnet" -ForegroundColor White
Write-Host "  .\build\axionax-core.exe validator start" -ForegroundColor White
Write-Host "  .\build\axionax-core.exe worker register" -ForegroundColor White
Write-Host "  .\build\axionax-core.exe --help" -ForegroundColor White
Write-Host ""

Write-Host "Start Testnet:" -ForegroundColor Cyan
Write-Host "  cd Axionax_v1.5_Testnet_in_a_Box" -ForegroundColor White
Write-Host "  powershell -ExecutionPolicy Bypass -File start-testnet.ps1" -ForegroundColor White
Write-Host ""

Write-Host "Documentation:" -ForegroundColor Cyan
Write-Host "  TESTING_GUIDE.md - Full testing guide" -ForegroundColor White
Write-Host "  QUICKSTART.md - Quick start guide" -ForegroundColor White
Write-Host "  docs/API_REFERENCE.md - API documentation" -ForegroundColor White
Write-Host ""
