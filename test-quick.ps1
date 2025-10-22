# Axionax Core - Quick Test Script
# สคริปต์สำหรับทดสอบ Axionax Core อย่างรวดเร็ว

Write-Host "🚀 Axionax Core - Quick Test" -ForegroundColor Cyan
Write-Host "================================`n" -ForegroundColor Cyan

$ErrorActionPreference = "Continue"

# ฟังก์ชันช่วย
function Test-Command {
    param($Name, $Command)
    Write-Host "🔍 Testing: $Name" -ForegroundColor Yellow
    Invoke-Expression $Command
    Write-Host "`n"
}

# 1. ตรวจสอบ Binary
Write-Host "Step 1: ตรวจสอบ Axionax Core Binary" -ForegroundColor Green
if (Test-Path ".\build\axionax-core.exe") {
    Write-Host "✅ Binary found!" -ForegroundColor Green
    Test-Command "Version" ".\build\axionax-core.exe version"
} else {
    Write-Host "❌ Binary not found! กรุณา build ก่อน: go build -o build\axionax-core.exe .\cmd\axionax" -ForegroundColor Red
    exit 1
}

# 2. ทดสอบ Configuration
Write-Host "Step 2: ทดสอบ Configuration" -ForegroundColor Green
Test-Command "Config Init" ".\build\axionax-core.exe config init"
Test-Command "Config Show" ".\build\axionax-core.exe config show"

# 3. ทดสอบ Key Generation
Write-Host "Step 3: ทดสอบ Key Management" -ForegroundColor Green
Test-Command "Generate Validator Key" ".\build\axionax-core.exe keys generate --type validator"
Test-Command "Generate Worker Key" ".\build\axionax-core.exe keys generate --type worker"
Test-Command "List Keys" ".\build\axionax-core.exe keys list"

# 4. ทดสอบ Status Commands
Write-Host "Step 4: ทดสอบ Status Commands" -ForegroundColor Green
Test-Command "Validator Status" ".\build\axionax-core.exe validator status"
Test-Command "Worker Status" ".\build\axionax-core.exe worker status"
Test-Command "Stake Balance" ".\build\axionax-core.exe stake balance"

# 5. สร้าง Worker Specs
Write-Host "Step 5: สร้าง Worker Specifications" -ForegroundColor Green
$workerJson = @'
{
  "gpus": [{
    "model": "NVIDIA RTX 4090",
    "vram": 24,
    "count": 1
  }],
  "cpu_cores": 16,
  "ram": 64,
  "storage": 1000,
  "bandwidth": 1000,
  "region": "us-west"
}
'@
Set-Content -Path "worker-specs.json" -Value $workerJson -Encoding UTF8
Write-Host "Created worker-specs.json" -ForegroundColor Green

# 6. ตรวจสอบ Docker (ถ้ามี)
Write-Host "`nStep 6: ตรวจสอบ Docker และ Testnet" -ForegroundColor Green
$dockerInstalled = Get-Command docker -ErrorAction SilentlyContinue
if ($dockerInstalled) {
    Write-Host "Docker พบแล้ว" -ForegroundColor Green
    
    $dockerStatus = docker ps 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Docker Engine กำลังทำงาน" -ForegroundColor Green
        
        Write-Host "`nDocker Containers:" -ForegroundColor Cyan
        docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
        
        Write-Host "`nTestnet Services:" -ForegroundColor Cyan
        Write-Host "  RPC: http://localhost:8545" -ForegroundColor White
        Write-Host "  Explorer: http://localhost:4001" -ForegroundColor White
        Write-Host "  Faucet: http://localhost:8080" -ForegroundColor White
        
    } else {
        Write-Host "Docker Engine ไม่ทำงาน - กรุณาเปิด Docker Desktop" -ForegroundColor Yellow
    }
} else {
    Write-Host "Docker ไม่ได้ติดตั้ง - ใช้ Demo Mode ได้" -ForegroundColor Yellow
}

# 7. แสดงคำสั่งที่เป็นประโยชน์
Write-Host "`n" -ForegroundColor White
Write-Host "================================" -ForegroundColor Cyan
Write-Host "คำสั่งที่เป็นประโยชน์:" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Start Node:" -ForegroundColor Green
Write-Host "   .\build\axionax-core.exe start --network testnet" -ForegroundColor White
Write-Host ""
Write-Host "Start Validator:" -ForegroundColor Green
Write-Host "   .\build\axionax-core.exe validator start" -ForegroundColor White
Write-Host ""
Write-Host "Register Worker:" -ForegroundColor Green
Write-Host "   .\build\axionax-core.exe worker register --specs worker-specs.json" -ForegroundColor White
Write-Host ""
Write-Host "Deposit Stake:" -ForegroundColor Green
Write-Host "   .\build\axionax-core.exe stake deposit 10000 --address 0x..." -ForegroundColor White
Write-Host ""
Write-Host "Help:" -ForegroundColor Green
Write-Host "   .\build\axionax-core.exe --help" -ForegroundColor White
Write-Host ""

# 8. ทดสอบ RPC endpoint (ถ้า Docker ทำงาน)
if ($dockerInstalled -and ($LASTEXITCODE -eq 0)) {
    Write-Host "ทดสอบ Testnet Endpoints..." -ForegroundColor Green
    
    $curlInstalled = Get-Command curl.exe -ErrorAction SilentlyContinue
    if ($curlInstalled) {
        $rpcTest = curl.exe -s -X POST http://localhost:8545 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' 2>&1
        
        if ($rpcTest -match "0x7a69") {
            Write-Host "RPC Endpoint ทำงานปกติ (Chain ID: 31337)" -ForegroundColor Green
        } else {
            Write-Host "RPC Endpoint ไม่ตอบสนอง" -ForegroundColor Yellow
        }
    }
}

Write-Host "`n================================" -ForegroundColor Cyan
Write-Host "การทดสอบเสร็จสมบูรณ์!" -ForegroundColor Green
Write-Host "================================`n" -ForegroundColor Cyan

Write-Host "อ่านเพิ่มเติมได้ที่:" -ForegroundColor Cyan
Write-Host "   TESTING_GUIDE.md - คู่มือการทดสอบแบบเต็ม" -ForegroundColor White
Write-Host "   QUICKSTART.md - Quick start guide" -ForegroundColor White
Write-Host "   docs/API_REFERENCE.md - API documentation" -ForegroundColor White
Write-Host ""
