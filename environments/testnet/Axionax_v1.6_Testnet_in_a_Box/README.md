# Axionax V1.6 Testnet in a Box

## คำอธิบาย
ระบบนี้ช่วยให้คุณสามารถรัน Testnet ของ Axionax V1.6 ได้ครบทุกส่วนในเครื่องเดียว โดยใช้ Docker Compose

## โครงสร้าง
- `Dockerfile` สร้าง node จาก Rust
- `docker-compose.yml` รันทุกบริการ (node, deployer, faucet, UI, blockscout, postgres)
- `deployer/` สคริปต์สำหรับ deploy contract และตรวจสอบ RPC
- `faucet/` API แจกเหรียญ native/erc20
- `ui/` เว็บ UI และ nginx config

## วิธีใช้งาน
1. สร้างไฟล์ `.env` โดยดูตัวอย่างจาก `.env.example`
2. สั่ง build และรัน
   ```bash
   docker-compose build
   docker-compose up -d
   ```
3. ตรวจสอบสถานะ
   ```bash
   docker-compose ps
   ```
4. ทดสอบ faucet
   ```bash
   curl 'http://127.0.0.1:8081/request?address=YOUR_ADDRESS'
   curl 'http://127.0.0.1:8081/request-erc20?address=YOUR_ADDRESS'
   ```
5. เข้าหน้าเว็บ UI ที่ http://127.0.0.1:8080

## หมายเหตุ
- หากต้องการ deploy contract ใหม่ ให้ใช้ `deployer/deploy_token.js`
- สามารถปรับแต่ง config ได้ตามต้องการ

## ติดต่อ/แจ้งปัญหา
โปรดแจ้งปัญหาผ่าน GitHub repository
