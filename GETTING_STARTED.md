# Quick Start - การทดสอบ Axionax Core

## สถานการณ์ปัจจุบัน ✅

- ✅ Axionax Core binary พร้อมใช้งาน (build/axionax-core.exe)
- ✅ ทุก CLI commands ทำงานได้ปกติ
- ⚠️ Docker Engine ยังไม่ได้เปิด (จำเป็นสำหรับ Full Testnet)

## วิธีทดสอบ

### 1. ทดสอบ CLI (ไม่ต้องใช้ Docker)

รันสคริปต์ทดสอบอัตโนมัติ:
```powershell
powershell -ExecutionPolicy Bypass -File quick-test.ps1
```

### 2. ทดสอบ Full Testnet (ต้องใช้ Docker)

**ขั้นตอนที่ 1: เปิด Docker Desktop**
- เปิดโปรแกรม Docker Desktop
- รอจนกว่า icon จะเป็นสีเขียว (Engine running)

**ขั้นตอนที่ 2: เริ่มต้น Testnet**
```powershell
cd Axionax_v1.5_Testnet_in_a_Box
powershell -ExecutionPolicy Bypass -File start-testnet.ps1
```

**ขั้นตอนที่ 3: เริ่มต้น Axionax Node**
```powershell
cd ..
.\build\axionax-core.exe start --network testnet
```

## Testnet Endpoints

เมื่อ Testnet ทำงานแล้ว คุณจะสามารถเข้าถึง:

- **RPC Node**: http://localhost:8545
- **Block Explorer**: http://localhost:4001
- **Faucet**: http://localhost:8080

## คำสั่งที่ใช้บ่อย

```powershell
# แสดง version
.\build\axionax-core.exe version

# สร้าง keypair
.\build\axionax-core.exe keys generate --type validator

# เริ่ม validator
.\build\axionax-core.exe validator start

# ดู status
.\build\axionax-core.exe validator status
.\build\axionax-core.exe worker status

# Configuration
.\build\axionax-core.exe config show

# Help
.\build\axionax-core.exe --help
```

## เอกสารเพิ่มเติม

- `TESTING_GUIDE.md` - คู่มือทดสอบแบบละเอียด
- `QUICKSTART.md` - Quick start guide  
- `docs/API_REFERENCE.md` - API documentation
- `docs/BUILD.md` - Build instructions

## Troubleshooting

### Docker Engine ไม่ทำงาน
```
Error: error during connect: this error may indicate that the docker daemon is not running
```
**แก้ไข**: เปิด Docker Desktop และรอให้ Engine เริ่มทำงาน

### Port ถูกใช้งานแล้ว
```
Error: port 8545 is already in use
```
**แก้ไข**: หยุด process ที่ใช้ port นั้นอยู่ หรือเปลี่ยน port ใน config

### Container ไม่เริ่มทำงาน
```powershell
# ดู logs
docker compose logs anvil
docker compose logs blockscout
docker compose logs faucet

# Restart services
docker compose down
docker compose up -d
```

## ขั้นตอนต่อไป

1. ✅ ทดสอบ CLI ด้วย `quick-test.ps1` (เสร็จแล้ว)
2. ⏳ เปิด Docker Desktop
3. ⏳ รัน `start-testnet.ps1` ใน Axionax_v1.5_Testnet_in_a_Box/
4. ⏳ ทดสอบการเชื่อมต่อกับ Testnet
5. ⏳ ส่ง transaction ทดสอบ
6. ⏳ ทดสอบ PoPC validation

---

**หมายเหตุ**: ปัจจุบันอยู่ในโหมด Demo (mock data) เพราะ Docker Engine ยังไม่ได้เปิด  
เมื่อเปิด Docker แล้ว คุณจะสามารถทดสอบกับ blockchain จริงได้
