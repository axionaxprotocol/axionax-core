# รีดีพลอย AXX, อัปเดต .env + ui/config.json แล้วรีสตาร์ท faucet/ui
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

docker compose run --rm deployer sh -lc 'node deploy_token.js'

if (Test-Path .\.env.axx.tmp) {
  Add-Content .\.env (Get-Content .\.env.axx.tmp -Raw)
}

if (Test-Path .\deployer\ui_config_out\config.json) {
  Copy-Item .\deployer\ui_config_out\config.json .\ui\config.json -Force
}

docker compose up -d --force-recreate faucet ui

Write-Host "Done. Open http://127.0.0.1:8080 and try ERC-20 faucet again." -ForegroundColor Green
