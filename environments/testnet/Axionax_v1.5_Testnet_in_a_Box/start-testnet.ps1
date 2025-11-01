# Axionax Testnet Startup Script
# สคริปต์สำหรับเริ่มต้น Testnet-in-a-Box

Write-Host "🚀 Starting Axionax Testnet v1.5" -ForegroundColor Cyan
Write-Host "================================`n" -ForegroundColor Cyan

$ErrorActionPreference = "Continue"

# ตรวจสอบ Docker
Write-Host "Step 1: ตรวจสอบ Docker" -ForegroundColor Green
try {
    docker --version
    Write-Host "✅ Docker installed" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker not found! กรุณาติดตั้ง Docker Desktop" -ForegroundColor Red
    Write-Host "   Download: https://www.docker.com/products/docker-desktop" -ForegroundColor Yellow
    exit 1
}

# ตรวจสอบ Docker Engine
Write-Host "`nStep 2: ตรวจสอบ Docker Engine" -ForegroundColor Green
$dockerStatus = docker ps 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Docker Engine ไม่ทำงาน!" -ForegroundColor Red
    Write-Host "   กรุณาเปิด Docker Desktop แล้วรอจนกว่า icon จะเป็นสีเขียว" -ForegroundColor Yellow
    Write-Host "   กด Enter เมื่อพร้อม..." -ForegroundColor Yellow
    Read-Host
    
    # ตรวจสอบอีกครั้ง
    $dockerStatus = docker ps 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Docker Engine ยังไม่ทำงาน - ยกเลิก" -ForegroundColor Red
        exit 1
    }
}
Write-Host "✅ Docker Engine running" -ForegroundColor Green

# หยุด containers เก่า (ถ้ามี)
Write-Host "`nStep 3: ทำความสะอาด containers เก่า" -ForegroundColor Green
docker compose down 2>&1 | Out-Null
Write-Host "✅ Cleaned up old containers" -ForegroundColor Green

# เริ่มต้น Testnet
Write-Host "`nStep 4: เริ่มต้น Testnet Services" -ForegroundColor Green
Write-Host "   (อาจใช้เวลาสักครู่สำหรับการดาวน์โหลด images ครั้งแรก)" -ForegroundColor Yellow
docker compose up -d

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Testnet started!" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to start testnet" -ForegroundColor Red
    exit 1
}

# รอให้ services พร้อม
Write-Host "`nStep 5: รอให้ services พร้อมใช้งาน..." -ForegroundColor Green
Start-Sleep -Seconds 10

# แสดง status
Write-Host "`nStep 6: ตรวจสอบ Service Status" -ForegroundColor Green
docker compose ps

# ทดสอบ RPC endpoint
Write-Host "`nStep 7: ทดสอบ RPC Endpoint" -ForegroundColor Green
$maxRetries = 10
$retryCount = 0
$rpcReady = $false

while ($retryCount -lt $maxRetries -and -not $rpcReady) {
    try {
        $response = curl.exe -s -X POST http://localhost:8545 `
            -H "Content-Type: application/json" `
            -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' 2>&1
        
        if ($response -match "0x7a69") {
            Write-Host "✅ RPC Endpoint ready! (Chain ID: 31337)" -ForegroundColor Green
            $rpcReady = $true
        } else {
            $retryCount++
            Write-Host "   ⏳ Waiting for RPC... ($retryCount/$maxRetries)" -ForegroundColor Yellow
            Start-Sleep -Seconds 3
        }
    } catch {
        $retryCount++
        Write-Host "   ⏳ Waiting for RPC... ($retryCount/$maxRetries)" -ForegroundColor Yellow
        Start-Sleep -Seconds 3
    }
}

if (-not $rpcReady) {
    Write-Host "⚠️  RPC Endpoint ไม่ตอบสนอง - ลองตรวจสอบ logs: docker compose logs anvil" -ForegroundColor Yellow
}

# แสดงข้อมูล endpoints
Write-Host "`n================================" -ForegroundColor Cyan
Write-Host "🎉 Testnet กำลังทำงาน!" -ForegroundColor Green
Write-Host "================================`n" -ForegroundColor Cyan

Write-Host "📡 Endpoints:" -ForegroundColor Cyan
Write-Host "   • RPC:      http://localhost:8545" -ForegroundColor White
Write-Host "   • Explorer: http://localhost:4001" -ForegroundColor White
Write-Host "   • Faucet:   http://localhost:8080" -ForegroundColor White
Write-Host ""

Write-Host "🔑 Default Accounts (Anvil):" -ForegroundColor Cyan
Write-Host "   Address: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" -ForegroundColor White
Write-Host "   PKey:    0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" -ForegroundColor White
Write-Host "   Balance: 10000 ETH" -ForegroundColor Green
Write-Host ""

Write-Host "💰 ขอ Test Tokens:" -ForegroundColor Cyan
Write-Host '   curl -X POST http://localhost:8080/faucet -H "Content-Type: application/json" -d "{\"address\":\"YOUR_ADDRESS\"}"' -ForegroundColor White
Write-Host ""

Write-Host "🔍 ดู Logs:" -ForegroundColor Cyan
Write-Host "   docker compose logs -f anvil      # RPC node" -ForegroundColor White
Write-Host "   docker compose logs -f blockscout # Explorer" -ForegroundColor White
Write-Host "   docker compose logs -f faucet     # Faucet" -ForegroundColor White
Write-Host ""

Write-Host "🛑 หยุด Testnet:" -ForegroundColor Cyan
Write-Host "   docker compose down" -ForegroundColor White
Write-Host ""

Write-Host "📖 Next Steps:" -ForegroundColor Cyan
Write-Host "   1. เปิด Explorer: http://localhost:4001" -ForegroundColor White
Write-Host "   2. ทดสอบ RPC: curl -X POST http://localhost:8545 -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}'" -ForegroundColor White
Write-Host "   3. เริ่ม Axionax Core: cd .. && .\build\axionax-core.exe start --network testnet" -ForegroundColor White
Write-Host ""

Write-Host "================================`n" -ForegroundColor Cyan
