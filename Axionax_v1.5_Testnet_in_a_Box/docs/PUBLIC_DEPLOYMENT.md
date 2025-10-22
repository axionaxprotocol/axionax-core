# การเปิดใช้งานสาธารณะ (Public Deployment)

ไฟล์นี้อธิบายขั้นตอนแนะนำในการเปิดบริการ Axionax (UI, RPC, Faucet, Blockscout) สู่สาธารณะบนโดเมนจริง โดยตัวอย่างใช้โดเมน:
- ผลิตภัณฑ์หลัก (mainnet/public site): axionax.io
- เครือข่ายทดสอบ (testnet): testnet.axionax.io

สถาปัตยกรรม
- ใช้ Nginx (edge) เป็น reverse proxy หน้าอินเทอร์เน็ต (พอร์ต 80/443)
- บริการภายในทั้งหมด bind ที่ 127.0.0.1 เท่านั้น เพื่อบังคับวิ่งผ่าน Nginx
- เปิด CORS เฉพาะ origin ที่กำหนด (localhost สำหรับ dev และ (testnet.)axionax.io สำหรับสาธารณะ)

สิ่งที่ต้องพร้อม
- Docker Desktop/Compose บนเครื่องเซิร์ฟเวอร์ Windows
- เปิดพอร์ตอินเทอร์เน็ต: TCP 80, 443 เข้ามายังเครื่องเซิร์ฟเวอร์
- DNS ชี้ A record ดังนี้
  - axionax.io → Public IP ของเครื่องเซิร์ฟเวอร์
  - testnet.axionax.io → Public IP ของเครื่องเซิร์ฟเวอร์ (หรือ IP เดียวกัน)

ขั้นตอนเสนอแนะ

1) ตรวจ Nginx edge และ config
- เราได้เตรียม `reverse-proxy/nginx.conf` รองรับ 3 กรณี:
  - vhost: axionax.io (ใบรับรองที่ `reverse-proxy/certs/axionax.io/…`)
  - vhost: testnet.axionax.io (ใบรับรองที่ `reverse-proxy/certs/testnet.axionax.io/…`)
  - fallback: สำหรับ localhost/self-signed (ที่ `reverse-proxy/certs/…`)
- เส้นทางสำคัญ:
  - `/` → UI
  - `/rpc` → RPC (JSON-RPC)
  - `/faucet` → Faucet
  - `/blockscout-api` → Blockscout Backend API
  - `/explorer` → Blockscout Frontend UI

2) ออกใบรับรอง TLS (แนะนำ Let’s Encrypt)
- ใช้วิธี webroot ผ่านคอนเทนเนอร์ Certbot โดยให้ไฟล์ challenge ไปเก็บที่ `reverse-proxy/webroot` (ซึ่ง Nginx map ให้เสิร์ฟได้ที่ `/.well-known/acme-challenge/`)
- ตัวอย่างคำสั่ง (Windows PowerShell / cmd):

```bat
REM ออกใบ cert สำหรับ axionax.io
docker run --rm \
  -v "%cd%\reverse-proxy\webroot:/webroot" \
  -v "%cd%\reverse-proxy\letsencrypt:/etc/letsencrypt" \
  certbot/certbot certonly --webroot -w /webroot \
  -d axionax.io \
  -m you@example.com --agree-tos --no-eff-email

REM ออกใบ cert สำหรับ testnet.axionax.io
docker run --rm \
  -v "%cd%\reverse-proxy\webroot:/webroot" \
  -v "%cd%\reverse-proxy\letsencrypt:/etc/letsencrypt" \
  certbot/certbot certonly --webroot -w /webroot \
  -d testnet.axionax.io \
  -m you@example.com --agree-tos --no-eff-email
```

- หลังออกสำเร็จ ไฟล์จะอยู่ที่ `reverse-proxy/letsencrypt/live/<DOMAIN>/` ให้คัดลอกไปไว้ที่โฟลเดอร์ตามที่ Nginx ใช้:

```bat
REM สร้างโฟลเดอร์ปลายทาง
mkdir reverse-proxy\certs\axionax.io
mkdir reverse-proxy\certs\testnet.axionax.io

REM คัดลอกใบรับรองจริงไปยังที่ที่ Nginx ใช้
copy /Y reverse-proxy\letsencrypt\live\axionax.io\fullchain.pem reverse-proxy\certs\axionax.io\fullchain.pem
copy /Y reverse-proxy\letsencrypt\live\axionax.io\privkey.pem   reverse-proxy\certs\axionax.io\privkey.pem
copy /Y reverse-proxy\letsencrypt\live\testnet.axionax.io\fullchain.pem reverse-proxy\certs\testnet.axionax.io\fullchain.pem
copy /Y reverse-proxy\letsencrypt\live\testnet.axionax.io\privkey.pem   reverse-proxy\certs\testnet.axionax.io\privkey.pem

REM รีสตาร์ท edge เพื่อโหลดใบรับรองใหม่
docker compose restart edge
```

- การต่ออายุ (renew): รันคำสั่ง certbot เดิมซ้ำ (หรือใช้ `certbot renew`) แล้วคัดลอกไฟล์มาแทนของเดิม จากนั้น `docker compose restart edge`

หมายเหตุ: สำหรับทดสอบชั่วคราว มีสคริปต์ออก self-signed ให้ที่:
```bat
powershell -ExecutionPolicy Bypass -File reverse-proxy\generate-domain-certs.ps1 -Domain axionax.io -Days 90
powershell -ExecutionPolicy Bypass -File reverse-proxy\generate-domain-certs.ps1 -Domain testnet.axionax.io -Days 90
```

3) CORS และความปลอดภัย
- ใน `reverse-proxy/nginx.conf` ได้จำกัด CORS ไว้สำหรับ origin: localhost/127.0.0.1 และ (testnet.)axionax.io อยู่แล้ว
- ควรเปลี่ยนรหัสผ่าน BASIC_AUTH ของ Faucet และคีย์กระเป๋า faucet ใน `.env` ก่อนเปิดสาธารณะ
- แนะนำเพิ่ม rate limit ให้กับ `/rpc` และ `/faucet` (ปรับ Nginx เพิ่มได้) และพิจารณา WAF/CDN (เช่น Cloudflare)

4) ทดสอบหลังตั้งค่าโดเมนจริง
- ตรวจสอบ HTTP→HTTPS redirect: `http://axionax.io` ควรเด้งไป `https://axionax.io/`
- ตรวจสอบ UI: `https://axionax.io/` และ `https://testnet.axionax.io/`
- ตรวจสอบ RPC (JSON-RPC):
```bat
curl.exe -s -k -H "content-type: application/json" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_chainId\",\"params\":[]}" https://testnet.axionax.io/rpc/
```
- ตรวจสอบ Faucet health (Basic Auth):
```bat
curl.exe -k -H "Authorization: Basic <base64(user:pass)>" https://testnet.axionax.io/faucet/health
```
- ตรวจสอบ Blockscout API:
```bat
curl.exe -k "https://testnet.axionax.io/blockscout-api/api/v2/blocks?type=canonical&limit=1"
```
- ตรวจสอบ Explorer UI: `https://testnet.axionax.io/explorer/`

5) การตั้งค่าแยก mainnet/testnet (ตัวเลือก)
- ปัจจุบัน UI ชี้เป็น relative path อยู่แล้ว จึงใช้งานได้ทั้งสอง vhost โดยไม่ต้องแก้ค่า
- หากต้องการชื่อเครือข่ายต่างกันบน Blockscout Frontend ให้แก้ env ของบริการ `blockscout-frontend` ใน `docker-compose.yml` (เช่น เปลี่ยน `NEXT_PUBLIC_NETWORK_NAME`) แล้ว recreate เฉพาะคอนเทนเนอร์นั้น

ปัญหาพบบ่อย
- PowerShell ติดสัญลักษณ์ `&` เวลาเรียก URL ที่มี query string → ให้ครอบ URL ด้วย ""
- ถ้าเรียก Blockscout API แล้วขึ้น Page not found ให้ตรวจ path `/api/v2/...` และตรวจดูว่า Blockscout backend ทำงานปกติ (`docker compose logs blockscout`)
- ถ้า edge แจ้งว่าโหลดใบรับรองไม่ได้ ให้ตรวจว่ามีไฟล์ `fullchain.pem` และ `privkey.pem` ในโฟลเดอร์โดเมน และสิทธิ์การอ่านไฟล์ถูกต้อง
