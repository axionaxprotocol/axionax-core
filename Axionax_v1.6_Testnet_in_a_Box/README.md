# Axionax V1.6 Testnet in a Box

## คำอธิบาย
ระบบนี้ช่วยให้คุณสามารถรัน Testnet ของ Axionax V1.6 ได้ครบทุกส่วนในเครื่องเดียว โดยใช้ Docker Compose — เหมาะสำหรับการพัฒนาและทดสอบ integration ระหว่าง node, faucet, UI และ Blockscout (explorer)

## โครงสร้างของโฟลเดอร์
- `Dockerfile` — สร้างบิลด์สำหรับ node (Rust)
- `docker-compose.yml` — คอนฟิกสำหรับรันบริการทั้งหมด: node, deployer, faucet, UI, blockscout, postgres
- `deployer/` — สคริปต์ Node.js สำหรับคอมไพล์และ deploy contract (`deploy_token.js`) และสคริปต์เช็ก RPC (`deploy.js`)
- `faucet/` — REST API สำหรับแจกเหรียญ native และ ERC-20
- `ui/` — ไฟล์เว็บ UI และ `nginx.conf`

## เตรียมไฟล์ `.env`
1. คัดลอก `.env.example` เป็น `.env` ในโฟลเดอร์ `Axionax_v1.6_Testnet_in_a_Box/`
2. ที่สำคัญ:
    - `RPC_URL` — URL ของ RPC (ค่าเริ่มต้น `http://127.0.0.1:8545`)
    - `DEPLOYER_PRIVATE_KEY` — private key สำหรับ deploy contract (หรือใช้ `FAUCET_PRIVATE_KEY`)
    - `FAUCET_PRIVATE_KEY` — private key ของบัญชีที่ faucet จะใช้แจก
    - `ERC20_TOKEN_ADDRESS` — จะถูกเขียนหลังจาก deploy (หากใช้ `deploy_token.js`)
    - `CHAIN_ID` — ค่าเริ่มต้นคือ `86137`

ตัวอย่าง `.env.example` (มีไฟล์มาให้):

```
RPC_URL=http://127.0.0.1:8545
DEPLOYER_PRIVATE_KEY=... (เติม private key ของคุณ)
FAUCET_PRIVATE_KEY=... (เติม private key ของ faucet)
ERC20_DECIMALS=18
FAUCET_AMOUNT_ETH=1
FAUCET_AMOUNT_ERC20=1000
CHAIN_ID=86137
FAUCET_URL=http://127.0.0.1:8081
```

## รัน Testnet (เบื้องต้น)
1. สร้าง `.env` ตามด้านบน
2. Build และรัน:

```bash
docker-compose build
docker-compose up -d
```

3. ตรวจสอบสถานะของ container:

```bash
docker-compose ps
```

4. ตรวจสอบว่า node ตอบสนอง RPC:

```bash
curl -sS http://127.0.0.1:8545 | jq .
# หรือใช้ deployer script เช็ก
node deployer/deploy.js
```

## การ deploy ERC-20 (AXX)
- โฟลเดอร์ `deployer/` มี `deploy_token.js` ซึ่งคอมไพล์ `contracts/AXX.sol` ด้วย `solc` และ deploy ด้วย `ethers`
- หากต้องการรันด้วยโครงสร้างใน `docker-compose` ตัว `deployer` จะพยายาม `npm ci`/`npm i` และจะรัน `deploy_token.js` (ดู `docker-compose.yml`)

รันด้วยตนเอง (จากโฟลเดอร์ `Axionax_v1.6_Testnet_in_a_Box`):

```bash
# ติดตั้ง dependencies (ถ้ายังไม่มี)
cd deployer
npm ci
node deploy_token.js
```

หลังจากสำเร็จ สคริปต์จะเขียนไฟล์ `ui_config_out/config.json` และ `.env.axx.tmp` (มี `ERC20_TOKEN_ADDRESS`)

## ทดสอบ faucet และ UI
- เรียก faucet (native):

```bash
curl 'http://127.0.0.1:8081/request?address=YOUR_ADDRESS'
```

- เรียก faucet (ERC20):

```bash
curl 'http://127.0.0.1:8081/request-erc20?address=YOUR_ADDRESS'
```

- เปิด UI ที่: http://127.0.0.1:8080
- Blockscout backend จะอยู่ที่ http://127.0.0.1:4000 และ frontend (if used) ที่ http://127.0.0.1:4001

## Troubleshooting (ปัญหาที่พบบ่อย)
- Node ไม่ฟังพอร์ต 8545:
   - ตรวจสอบ `docker-compose ps` และ logs: `docker-compose logs axionax-node`
   - ตรวจสอบว่ volume `axionax-data` ไม่มี permission error
- Deployer ล้มเหลวเมื่อคอมไพล์ solc:
   - ตรวจสอบเวอร์ชัน `solc` ที่ติดตั้งใน `deployer/package.json`
   - ดูข้อความ error ที่ `deploy_token.js` จะพิมพ์ออกมาหาก solc คืนค่า error
- Blockscout ไม่เริ่มหรือ index ช้า:
   - ตรวจสอบ `postgres` ว่า healthy
   - เพิ่ม `RETRIES` หรือรอให้ DB migrate ผ่าน (คำสั่งใน docker-compose จะ run migrate ก่อน start)
- Faucet ไม่แจกเหรียญ:
   - ตรวจสอบว่าคีย์ใน `.env` (FAUCET_PRIVATE_KEY) มีค่าและมี balance
   - ตรวจสอบ logs: `docker-compose logs faucet`

## Tips และข้อแนะนำ
- ถ้าต้องการรีเซ็ต testnet ให้ลบ volumes แล้วรันใหม่:

```bash
docker-compose down -v
docker-compose up -d --build
```

- สำหรับ dev loop รัน node แบบ local (ไม่ต้อง docker) อาจช่วยให้ debug เร็วขึ้น

## ติดต่อ/แจ้งปัญหา
หากมีบั๊กหรือคำถาม เปิด issue ใน GitHub repository พร้อม logs ที่เกี่ยวข้อง

--
เอกสารฉบับนี้ขยายจาก README เดิมเพื่ออธิบายขั้นตอนที่ยังขาด และช่วยให้ผู้ใช้สามารถทำงานกับ v1.6 Testnet in a Box ได้สะดวกขึ้น
