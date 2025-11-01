# Axionax Protocol — Architecture Overview v1.5 (Breakdown)

เอกสารนี้แยกสถาปัตยกรรม Axionax ออกเป็นส่วนย่อยตามสรุปเวอร์ชัน 1.5 เพื่อให้เห็นภาพรวมและรายละเอียดของแต่ละองค์ประกอบได้ชัดเจนขึ้น

- วงจร L1 แบบบูรณาการ: Execute → Validate PoPC → Data Availability → Settlement
- ระบบตลาดที่ขับเคลื่อนโดยโปรโตคอล: ASR และ Posted Price Controller
- ความปลอดภัยและความโปร่งใส: Delayed VRF, DA Pre-commit, Stratified + Adaptive Sampling, Replica Diversity, Fraud-Proof Window
- ระบบ DeAI และการกำกับดูแลด้วย DAO
- พารามิเตอร์หลักและเวิร์กโฟลว์ v1.5

---

## 0) High-Level Overview

```mermaid
flowchart LR
    %% Access
    Clients["Users and DApps and Wallets"] --> RPC["RPC Nodes"]

    %% Core
    subgraph L1["Axionax L1"]
        direction LR
        subgraph Market["Assignment and Pricing"]
            ASR["Auto Selection Router<br/>Top K weighted VRF"]
            PPC["Posted Price Controller<br/>Util and Queue control"]
        end
        subgraph Core["Integrated Core Loop"]
            Exec["Execution Engine<br/>Deterministic"]
            PoPC["Validation<br/>PoPC Sampling"]
            DA["Data Availability<br/>EC and Storage"]
            Settle["Settlement and Finality"]
        end
        Validators["Validator Set"]
        Auditors["DA Live Auditors"]
    end

    %% Flows
    RPC --> Exec
    Exec --> PoPC --> DA --> Settle
    Settle -.-> Exec

    %% Market control links
    PPC -.-> ASR
    ASR --> Exec

    %% Security and ops
    VRF["Delayed VRF"] -.-> PoPC
    Auditors -.-> DA
    Telemetry["Monitoring and Telemetry"] -.-> Exec
    Telemetry -.-> PoPC
    Telemetry -.-> DA
    Telemetry -.-> Settle
    Attest["Public Attestations"] -.-> Telemetry

    %% Governance and DeAI
    DAO["Axionax DAO"] -.-> PPC
    DAO -.-> ASR
    DeAI["DeAI Sentinel and Assist"] -.-> Telemetry
    Telemetry -.-> DeAI
    DeAI --> DAO
```

หมายเหตุ
- เส้นทึบ: เส้นทางข้อมูลหลัก
- เส้นประ: การควบคุม, การตรวจสอบ, และสัญญาณกำกับดูแล

---

## 1) Core Workflow v1.5 (ไม่มีประมูล)

```mermaid
sequenceDiagram
    autonumber
    actor Client
    participant RPC as "RPC Nodes"
    participant ASR as "ASR Router"
    participant W as "Worker"
    participant DA as "DA Service"
    participant V as "Validators"
    participant VRF as "Delayed VRF"
    participant AUD as "DA Auditors"
    participant CH as "Chain / Settlement"

    Client->>RPC: Post Job and SLA
    RPC->>ASR: Enqueue job
    ASR-->>RPC: Assign worker (Top K weighted VRF)
    RPC->>W: Job details and SLA

    W->>W: Deterministic execution
    W->>DA: DA Pre commit (erasure coded chunks)
    W->>CH: Commit o_root and stake

    Note over VRF,CH: Wait k blocks (delayed VRF)
    VRF-->>V: Challenge set S

    W->>V: Prove samples with Merkle paths
    V->>V: Verify samples and tally votes
    V-->>CH: Seal block if pass

    AUD-->>DA: Live availability checks
    AUD-->>CH: Report DA withhold if any

    CH-->>Client: Receipt and status
    CH-->>W: Reward or Slash (Worker)
    CH-->>V: Slash False PASS (if fraud)

    par Fraud Window
        Client-->>CH: Fraud claim and evidence
        CH-->>W: Retroactive penalty if proven
        CH-->>V: False PASS slashing
    end

    CH-->>CH: Finalize and Settlement
```

หัวใจ: Post → Assign → Execute → Commit and DA Pre-commit → Wait k → Challenge → Prove → Verify and Seal → Fraud Window → Finalize

---

## 2) ASR — Auto-Selection Router

```mermaid
flowchart LR
    subgraph Inputs["Inputs"]
        HW["Hardware and Software<br/>GPU and VRAM and Framework and Region"]
        Hist["Historical Performance<br/>PoPC pass and DA reliability and Latency and Uptime"]
        Quota["Quota State<br/>Per org and ASN and Region"]
        New["Newcomer Status"]
    end

    subgraph Scoring["Scoring and Eligibility"]
        Elig["Eligibility Filter"]
        Score["Score = Suitability x Performance x FairnessBoost"]
        TopK["Select Top K"]
        VRF["VRF Weighted Draw"]
    end

    subgraph Output["Output"]
        Assign["Assigned Worker"]
    end

    HW --> Elig
    Hist --> Score
    Quota --> Score
    New --> Score
    Elig --> Score --> TopK --> VRF --> Assign
```

รายละเอียด
- Suitability: ความเข้ากันได้กับข้อกำหนดงาน
- Performance: ค่าความน่าเชื่อถือเชิงสถิติ (เช่น EWMA 7–30 วัน)
- FairnessBoost: จำกัดโควต้า, newcomer boost แบบ ε-greedy, และ anti-collusion ตาม org/ASN/ภูมิภาค
- พารามิเตอร์หลัก: K, q_max, ε (กำหนดโดย DAO)

---

## 3) Posted Price Controller (PPC)

```mermaid
flowchart LR
    subgraph Metrics["Live Metrics"]
        Util["Utilization (util)"]
        Queue["Queue Length (q)"]
    end
    Controller["Price Controller<br/>exp response and clamps"]
    Prices["Per class price p_c"]
    Market["Market Execution Rate"]
    Target["Targets<br/>util* and q*"]

    Util --> Controller
    Queue --> Controller
    Target -.-> Controller
    Controller --> Prices --> Market --> Util
    Market --> Queue
```

สูตรปรับราคา (แนวคิด)
- ปรับราคาต่อรอบเพื่อลดความแออัดและรักษาเสถียรภาพ
- พารามิเตอร์โดย DAO: α, β, ขอบเขต p_min ถึง p_max

---

## 4) PoPC — Proof of Probabilistic Checking

```mermaid
flowchart LR
    Client["Client"] --> W["Worker"]
    W -->|Compute deterministic| Out["Outputs"]
    Out -->|Merkle tree| Root["o_root"]
    W -->|Commit o_root and stake| Chain["Chain"]

    subgraph Challenge["Challenge via VRF"]
        VRF["Delayed VRF (k blocks)"]
        S["Sample set S size s"]
    end

    Chain -.-> VRF --> S
    W -->|Provide samples + Merkle paths| V["Validators"]
    V -->|Verify| Chain

    PDetect["Detect probability<br/>P_detect = 1 - (1 - f)^s"]
    V -.-> PDetect
```

แนวคิดสำคัญ
- ลดต้นทุนตรวจสอบเหลือ O(s)
- ปรับ s เพื่อเพิ่มความมั่นใจตามระดับความเสี่ยง

---

## 5) Data Availability (DA) และการตรวจสอบ

```mermaid
flowchart LR
    subgraph DAFlow["DA Flow"]
        Pre["DA Pre commit<br/>Erasure coded chunks"]
        Store["Storage and Retrieval"]
        Audit["Live Audit"]
    end

    W["Worker"] --> Pre --> Store
    Audit -.-> Store
    Audit -.-> Chain["Chain"]
    Chain -.-> Pen["DA withhold slashing"]
```

หลักการ
- ต้องพร้อมให้ดึงข้อมูลส่วนที่ท้าทายได้เสมอภายในหน้าต่างเวลา Δt_DA
- หากขาดความพร้อม มีบทลงโทษทันที

---

## 6) Security and Anti-Fraud Layer

```mermaid
flowchart TB
    VRF["Delayed VRF and Anti grinding"]
    Strat["Stratified Sampling"]
    Adapt["Adaptive Escalation"]
    Replica["Replica Diversity and Jury"]
    FraudW["Fraud Proof Window"]
    SlashW["Worker Slashing"]
    SlashV["Validator False PASS Slashing"]

    VRF -.-> Challenge["Challenge Set S"]
    Strat -.-> Challenge
    Adapt -.-> Challenge
    Replica -.-> Verify["Cross check by replicas"]
    FraudW -.-> SlashW
    FraudW -.-> SlashV
```

หมายเหตุ
- สุ่มท้าทายหน่วงเวลา (k บล็อก) ลดโอกาส grinding
- ตรวจแบบแบ่งชั้นและเพิ่มตัวอย่างอัตโนมัติเมื่อพบสัญญาณเสี่ยง
- ทำซ้ำบางส่วนกับความหลากหลายของแหล่งที่มาเพื่อลดการฮั้ว
- มีหน้าต่างพิสูจน์การโกงเพื่อความรับผิดชอบย้อนหลัง

---

## 7) DeAI และ Governance

```mermaid
flowchart LR
    Telemetry["Telemetry and Metrics"] --> Sentinel["DeAI Sentinel<br/>Anomaly detection"]
    Attest["Public Attestations"] --> Sentinel
    Sentinel --> DAO["Axionax DAO"]
    Assist["Assistive DeAI<br/>Guidance and Explanations"] -.-> Clients["Users and Devs"]
    DAO -.-> Params["Protocol Parameters<br/>PoPC and ASR and PPC and VRF and Fraud Window"]
```

บทบาท
- DeAI Sentinel: ตรวจจับความผิดปกติ เช่น การเลี่ยงโควต้า, capacity spoof, การฮั้ว
- Assistive DeAI: แนะนำค่าพารามิเตอร์ที่ปลอดภัย, อธิบายการตัดสินใจ, ตรวจจับ determinism drift
- DAO: ปรับพารามิเตอร์สำคัญทั้งหมดของโปรโตคอล

---

## 8) พารามิเตอร์ที่แนะนำ (v1.5)

| พารามิเตอร์ | ค่าที่แนะนำ | คำอธิบาย |
|---|---:|---|
| s (samples) | 600–1500 | จำนวนจุดสุ่มตรวจใน PoPC |
| β (redundancy) | 2–3% | สัดส่วนงานที่ถูกทำซ้ำเพื่อ cross-check |
| K (Top K) | 64 | จำนวนผู้สมัครสูงสุดใน ASR ก่อนสุ่มด้วย VRF |
| q_max (quota) | 10–15% / epoch | โควต้าสูงสุดต่อผู้ให้บริการ |
| ε (epsilon) | 5% | สัดส่วน exploration สำหรับผู้เล่นใหม่ |
| util* | 0.7 | เป้าหมาย utilization ของ PPC |
| q* | 60 วินาที | เป้าหมายเวลารอคิวของ PPC |
| k (delay blocks) | ≥ 2 บล็อก | หน่วงเวลา seed ของ VRF |
| Δt_fraud | ~3600 วินาที | ระยะเวลาของ Fraud-Proof Window |
| False PASS (V) | ≥ 500 bp | อัตราโทษกับ Validator ที่โหวตผ่านงานโกง |

---

## 9) อ้างอิงเวิร์กโฟลว์ v1.5 (ย่อ)

1. โพสต์งาน → 2. ASR Assign → 3. Execute → 4. Commit และ DA Pre-commit → 5. Wait k → 6. Challenge (VRF) → 7. Prove → 8. Verify และ Seal → 9. Fraud Window → 10. Finalize และ Settlement → 11. DeAI Monitor

---

เคล็ดลับการเรนเดอร์ Mermaid บน GitHub
- ใช้ `<br/>` สำหรับขึ้นบรรทัดใหม่ในป้ายชื่อ
- หากมีวงเล็บในป้ายชื่อ ให้ครอบด้วยเครื่องหมายคำพูด เช่น `Node["Text (extra)"]`
- หลีกเลี่ยงลิงก์สองหัวแบบเส้นประในบรรทัดเดียว ให้ใช้สองบรรทัด `A -.-> B` และ `B -.-> A` แทน
