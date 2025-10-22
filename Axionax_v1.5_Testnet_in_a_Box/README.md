# Axionax v1.5 — Testnet-in-a-Box
อัปเดต: 2025-10-18

สแต็กทดสอบบนเครื่อง (local testnet) ด้วย Docker Compose ที่ประกอบด้วย:
- โหนดเชน: Foundry Anvil (RPC: http://localhost:8545, Chain ID: 31337)
- Blockscout Explorer Backend API (http://localhost:4000) + Postgres
- Blockscout Frontend UI (http://localhost:4001)
- Faucet API/Server (http://localhost:8081) + UI หน้าเว็บ (http://localhost:8080)
- Deployer (ดีพลอย AXX และสัญญาที่เกี่ยวข้องตามสคริปต์)

ค่าเริ่มต้นสำคัญที่ตั้งไว้แล้ว
- Chain ID: 31337 (สอดคล้องกับ Anvil)
- สกุลเงิน: AXX
- Faucet เปิด Basic Auth (ดีฟอลต์: admin:password)

## เริ่มต้นใช้งานอย่างรวดเร็ว (Windows)
1) ติดตั้ง Docker Desktop และเปิดใช้งาน Docker Compose
2) เปิด PowerShell/Command Prompt ที่โฟลเดอร์โปรเจกต์นี้ แล้วรัน:

	(ตัวอย่างคำสั่ง, รันได้ใน PowerShell หรือ cmd)

	docker compose up -d

3) ตรวจสอบคอนเทนเนอร์ขึ้นครบ

	docker compose ps

บริการและพอร์ต
- RPC (Anvil):       http://localhost:8545
- Blockscout Backend: http://localhost:4000 (API)
- Blockscout Frontend: http://localhost:4001 (UI)
- Faucet API:        http://localhost:8081
- Faucet Web (UI):   http://localhost:8080
	- มี proxy ไปยัง Blockscout API ผ่านพาธเดียวกัน เพื่อเลี่ยง CORS: http://localhost:8080/blockscout-api/

## การตรวจสอบ (Smoke test)
- ตรวจสอบ Blockscout API v2 (ตัวอย่างดึงบล็อกล่าสุด)
  - ตรงไปที่ backend:

	  curl.exe -s "http://localhost:4000/api/v2/blocks?type=canonical&limit=1"

  - หรือผ่าน proxy ของ UI (เลี่ยง CORS):

	  curl.exe -s "http://localhost:8080/blockscout-api/api/v2/blocks?type=canonical&limit=1"

- ตรวจสอบ Faucet health (มี Basic Auth)

	curl.exe -s -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" "http://localhost:8081/health"

ผลที่คาดหวัง: ควรได้ JSON ที่มี ok=true และ chainId=31337

## ขอเหรียญจาก Faucet
Faucet รองรับการโอน Native (AXX) และหากตั้งค่าไว้จะโอน ERC-20 AXX ด้วย

- ตัวอย่างขอ Native ไปยังแอดเดรสจาก Anvil (ใช้ Basic Auth):

  1) ขอรายการบัญชีตัวอย่างจาก Anvil:

	  docker compose exec hardhat cast rpc eth_accounts

  2) ใช้ที่อยู่หนึ่งอันไปขอจาก faucet:

	  curl.exe -s -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" "http://localhost:8081/request?address=0x70997970c51812dc3a010c7d01b50e0d17dc79c8"

จะได้ผลลัพธ์ JSON ที่มี hash ธุรกรรมและ blockNumber ซึ่งสามารถเปิดดูใน Blockscout ได้

หมายเหตุเรื่อง PowerShell vs curl
- บน Windows PowerShell คำว่า `curl` เป็น alias ของ Invoke-WebRequest แนะนำเรียก `curl.exe` เพื่อตัดปัญหา alias/การครอบสตริง
- หากใช้ `Invoke-RestMethod` ให้ระวังการต่อสตริงด้วย `+` และใช้ `-Uri` อย่างชัดเจน

## ตั้งค่า MetaMask
เพิ่มเครือข่ายแบบกำหนดเอง:
- Network Name: Axionax Local (Anvil)
- RPC URL: http://127.0.0.1:8545
- Chain ID: 31337
- Currency Symbol: AXX
- Block Explorer: http://127.0.0.1:4000
	(หน้า UI ของ Blockscout อยู่ที่ http://127.0.0.1:4001; ในหน้าเว็บของโปรเจกต์นี้มีปุ่มเปิด/คัดลอกลิงก์สำเร็จรูป)

## การปรับแต่งค่า
- UI config: `ui/config.json` (เช่น chainId, explorer, faucet)
- Faucet ค่าเริ่มต้น: แก้ไฟล์ `.env` ที่รูทโปรเจกต์
  - RPC_URL, CHAIN_ID, PORT, BASIC_AUTH, FAUCET_PRIVATE_KEY, ERC20_TOKEN_ADDRESS, ฯลฯ
  - เปลี่ยนค่าแล้วให้รีสตาร์ทคอนเทนเนอร์ faucet: `docker compose restart faucet`
- Docker Compose: `docker-compose.yml`
  - โหนด Anvil ถูกตั้งให้ `--chain-id 31337` และ listen ที่ 0.0.0.0
  - Blockscout ใช้ ETHEREUM_JSONRPC_VARIANT=geth, ชี้ RPC ไปที่ anvil, และ CHAIN_ID=31337

## Known issues & Troubleshooting
- PowerShell แสดง error เรื่อง `&` หรือการต่อสตริงด้วย `+` ให้ใช้ `curl.exe` หรือ `Invoke-RestMethod` พร้อมครอบสตริงอย่างถูกต้อง
- Blockscout API รุ่นใหม่ใช้เส้นทาง `/api/v2/...` ถ้าเรียก `/api?module=...` อาจเจอ "Unknown module"
- หาก Blockscout ชี้ไปยังเชน/ฐานข้อมูลผิดพลาดแล้วเกิดข้อมูลค้างใน DB ให้หยุด Blockscout แล้วรีเซ็ตสคีมาของ Postgres (ขั้นสูง, ทำเฉพาะกรณีจำเป็น):
  - หยุด Blockscout แล้วรันในคอนเทนเนอร์ postgres: `DROP SCHEMA public CASCADE; CREATE SCHEMA public;` จากนั้นสตาร์ท Blockscout ใหม่เพื่อรัน migrations

## โครงสร้างสำคัญ
- `docker-compose.yml` – orchestration ของทุกบริการ
- `ui/config.json` – ตั้งค่า chainId, rpc, faucet, explorer สำหรับหน้าเว็บ
- `.env` – คอนฟิก faucet (Basic Auth, RPC URL, Chain ID, คีย์กระเป๋า faucet)
- `shared/addresses.json` – ที่อยู่สัญญาที่ deployer บันทึกไว้ (ถ้ามี)
- `chain/`, `deployer/`, `faucet/` – โค้ดและสคริปต์ของแต่ละบริการ

## ความปลอดภัย
- อย่าใช้คีย์และรหัสผ่านดีฟอลต์สำหรับการเปิดใช้งานสาธารณะ
- หากเปิดพอร์ตออกอินเทอร์เน็ต แนะนำให้ตั้ง reverse proxy, rate limit และเปลี่ยนค่า BASIC_AUTH/FAUCET_PRIVATE_KEY ทันที
