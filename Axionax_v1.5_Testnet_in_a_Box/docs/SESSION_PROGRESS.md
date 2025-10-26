# สรุปงานวันนี้และขั้นตอนถัดไป

## ✅ งานที่เสร็จแล้วในวันนี้

### 1. อัพเกรด Nginx Configuration (reverse-proxy/nginx.conf)
- ✅ แยก certificate paths ต่อโดเมน:
  - `axionax.org` → `/etc/nginx/certs/axionax.org/`
  - `testnet.axionax.org` → `/etc/nginx/certs/testnet.axionax.org/`
  - Fallback (localhost) → `/etc/nginx/certs/`

- ✅ เพิ่ม Rate Limiting:
  - `/rpc/`: 30 requests/second (burst=10)
  - `/faucet/`: 1 request/minute (burst=2)

- ✅ เพิ่ม Security Headers:
  - X-Content-Type-Options: nosniff
  - X-Frame-Options: SAMEORIGIN
  - Referrer-Policy: no-referrer-when-downgrade
  - Strict-Transport-Security: max-age=31536000

- ✅ ตรึง CORS:
  - localhost/127.0.0.1 (สำหรับทดสอบ)
  - https://axionax.org
  - https://testnet.axionax.org

- ✅ ปรับ Webroot สำหรับ Certbot:
  - HTTP-01 challenge path: `/.well-known/acme-challenge/`
  - Root: `/var/www/webroot`

### 2. โครงสร้างใบรับรอง
- ✅ สร้างโฟลเดอร์:
  - `reverse-proxy/certs/axionax.org/`
  - `reverse-proxy/certs/testnet.axionax.org/`

- ✅ สร้าง script สร้าง self-signed certs:
  - `scripts/gen_domain_certs.ps1`

### 3. สำรองไฟล์
- ✅ สำรอง: `reverse-proxy/nginx.conf.bak`

---

## ⏭️ ขั้นตอนถัดไป (ทำต่อเมื่อพร้อม)

### ขั้นตอนที่ 1: เปิด Docker Desktop และคัดลอกใบรับรอง
```powershell
# 1. เปิด Docker Desktop
# 2. คัดลอกใบรับรองไปยังโฟลเดอร์ต่อโดเมน
cd "c:\Users\kong\Desktop\Axionax_v1.5_Testnet_in_a_Box\reverse-proxy\certs"
copy fullchain.pem axionax.org\fullchain.pem
copy privkey.pem axionax.org\privkey.pem
copy fullchain.pem testnet.axionax.org\fullchain.pem
copy privkey.pem testnet.axionax.org\privkey.pem
```

### ขั้นตอนที่ 2: ทดสอบ Nginx Syntax
```powershell
cd "c:\Users\kong\Desktop\Axionax_v1.5_Testnet_in_a_Box"
docker compose up -d
docker exec edge nginx -t
```

### ขั้นตอนที่ 3: รีสตาร์ท Edge
```powershell
docker compose restart edge
docker compose logs -f edge
```

### ขั้นตอนที่ 4: ทดสอบ Endpoints
```powershell
# UI
curl.exe -k -I https://localhost/

# RPC
curl.exe -k -s -H "content-type: application/json" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_chainId\",\"params\":[]}" https://localhost/rpc/

# Faucet (ต้องมี Basic Auth)
curl.exe -k -s -H "Authorization: Basic dXNlcjpwYXNz" https://localhost/faucet/health

# Blockscout API
curl.exe -k -s "https://localhost/blockscout-api/api/v2/blocks?type=canonical&limit=1"

# Proto RPC
curl.exe -k -s https://localhost/proto-rpc/health
curl.exe -k -s -H "content-type: application/json" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"proto_getParams\"}" https://localhost/proto-rpc/
```

### ขั้นตอนที่ 5: สร้าง Certbot Script
สร้าง `scripts/certbot_obtain.ps1` สำหรับขอใบรับรองจริงจาก Let's Encrypt

### ขั้นตอนที่ 6: เอกสารผู้ใช้
สร้าง `docs/JOIN_TESTNET.md` (ภาษาไทย) พร้อมตัวอย่าง:
- เพิ่มเครือข่ายใน MetaMask
- ขอเหรียญจาก Faucet
- ดู Explorer

### ขั้นตอนที่ 7: อัปเดต Protocol Parameters
แก้ `config/protocol_params.json` ให้ตรงกับ Whitepaper v1.5:
- VRF parameters (k, s_min, s_max, β)
- Timing parameters (Δt_*)
- Crypto specs (Ed25519, BLS12-381, ECVRF-P256)

### ขั้นตอนที่ 8: Smoke Test Script
สร้าง `scripts/testnet_smoke_full.ps1` ครอบคลุมทุก endpoint

### ขั้นตอนที่ 9: Runbook
สร้าง `docs/Runbook.md` พร้อมคำสั่งตรวจสุขภาพและ troubleshooting

---

## 📝 หมายเหตุสำคัญ

### CORS Configuration
ปัจจุบัน CORS อนุญาต:
- `localhost` และ `127.0.0.1` (ทุกพอร์ต)
- `https://axionax.org`
- `https://testnet.axionax.org`

### Rate Limiting
- **/rpc/**: 30 req/s, burst 10 (ป้องกัน spam RPC)
- **/faucet/**: 1 req/min, burst 2 (ป้องกันขอเหรียญบ่อยเกินไป)

### TLS Certificates
- Production: ใช้ Certbot + HTTP-01 challenge
- Development/Local: ใช้ self-signed certs ที่มีอยู่

### ตรวจสอบก่อนไปต่อ
1. Docker Desktop เปิดอยู่
2. ใบรับรองคัดลอกครบทุกโฟลเดอร์
3. `nginx -t` ผ่าน
4. Edge container ขึ้นปกติ
5. ทุก endpoint ตอบ 200/301/302 ตามที่คาดไว้

---

## 🔄 Next Session Plan
เมื่อกลับมาทำงานใหม่:
1. เปิด Docker Desktop
2. คัดลอกใบรับรอง
3. รัน `docker compose up -d`
4. ทดสอบทุก endpoint
5. สร้างเอกสารและ scripts ที่เหลือ
