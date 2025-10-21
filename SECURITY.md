# Security Policy

## Reporting a Vulnerability

The Axionax team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please DO NOT file a public issue for security vulnerabilities.**

Instead, please report security issues to:
- **Email**: security@axionax.io
- **PGP Key**: [Available on our website](https://axionax.io/security.asc)

### What to Include

When reporting a vulnerability, please include:
1. **Description**: Clear description of the vulnerability
2. **Impact**: Potential impact and attack scenarios
3. **Reproduction Steps**: Detailed steps to reproduce the issue
4. **Proof of Concept**: Code or logs demonstrating the vulnerability (if available)
5. **Suggested Fix**: Your recommendations for fixing the issue (optional)
6. **Contact Information**: How we can reach you for follow-up

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution Timeline**: Varies based on severity
  - Critical: 7-14 days
  - High: 14-30 days
  - Medium: 30-60 days
  - Low: 60-90 days

### Disclosure Policy

- We request that you give us reasonable time to address the issue before public disclosure
- We will credit security researchers in our release notes (unless you prefer to remain anonymous)
- We may offer bug bounties for significant vulnerabilities (details TBA)

---

## Threat Model

### Trust Assumptions

**Trusted Components:**
- DAO governance process (assumes honest majority)
- Validator set (assumes >66% honest stake)
- VRF implementation (cryptographic security)

**Untrusted Components:**
- Individual workers (can be malicious)
- Individual validators (can vote incorrectly)
- Network participants (can attempt Sybil attacks)
- External data sources

### Attack Vectors & Mitigations

#### 1. Worker Fraud (Submitting Incorrect Outputs)

**Attack**: Worker submits fraudulent compute results to earn rewards without doing work.

**Mitigations**:
- ✅ PoPC statistical sampling (P_detect = 1 - (1-f)^s)
- ✅ Stratified sampling ensures coverage
- ✅ Adaptive escalation for suspicious workers
- ✅ 100% stake slashing on proven fraud
- ✅ Fraud-proof window allows retroactive challenges
- ✅ Replica diversity (β% redundant execution)

**Residual Risk**: Low (if s ≥ 600 and β ≥ 2%)

---

#### 2. Validator Collusion (False-PASS Voting)

**Attack**: Validators collude to pass fraudulent work from co-conspirator workers.

**Mitigations**:
- ✅ Economic penalties: False-PASS slashing (≥500bp)
- ✅ Fraud-proof window allows challenge after sealing
- ✅ Reputation tracking and adaptive escalation
- ✅ Geographic and organizational diversity requirements
- ✅ Public attestations and transparency

**Residual Risk**: Medium (requires ongoing monitoring)

---

#### 3. VRF Grinding (Predicting Challenge Sets)

**Attack**: Worker attempts to manipulate VRF seed to get favorable challenge samples.

**Mitigations**:
- ✅ Delayed VRF (k-block delay, k ≥ 2)
- ✅ Seed derived from future block hashes (unpredictable at commit time)
- ✅ Cryptographic VRF guarantees

**Residual Risk**: Very Low (cryptographically secure)

---

#### 4. DA Withholding (Hiding Outputs)

**Attack**: Worker commits o_root but refuses to provide data for verification.

**Mitigations**:
- ✅ DA pre-commit requirement (erasure coded)
- ✅ Live DA audits by independent auditors
- ✅ Immediate slashing for DA unavailability (50% stake)
- ✅ Time-bound DA windows (Δt_DA)

**Residual Risk**: Low (immediate penalties)

---

#### 5. Sybil Attacks (Fake Identities)

**Attack**: Attacker creates many fake workers/validators to gain influence.

**Mitigations**:
- ✅ Stake requirements (economic barrier)
- ✅ Quota limits per organization/ASN/region
- ✅ Reputation building required for high-value jobs
- ✅ Geographic diversity tracking

**Residual Risk**: Medium (requires ongoing monitoring)

---

#### 6. ASR Gaming (Quota Bypass)

**Attack**: Worker attempts to game ASR scoring to receive disproportionate jobs.

**Mitigations**:
- ✅ Hard quota enforcement (q_max per epoch)
- ✅ FairnessBoost penalty for exceeding quota
- ✅ Organization/ASN aggregation
- ✅ DeAI Sentinel anomaly detection
- ✅ VRF-weighted selection (not just highest score)

**Residual Risk**: Low (multiple enforcement layers)

---

#### 7. Price Manipulation

**Attack**: Colluding parties manipulate PPC to artificially inflate/deflate prices.

**Mitigations**:
- ✅ Protocol-controlled pricing (no bidding)
- ✅ Automatic adjustment based on real metrics (util, queue)
- ✅ Governance-set price bounds (p_min, p_max)
- ✅ Transparent price update logs
- ✅ DeAI monitoring for anomalies

**Residual Risk**: Low (limited manipulation vectors)

---

#### 8. Long-Range Attacks

**Attack**: Attacker rewrites history using old validator keys.

**Mitigations**:
- ✅ Checkpointing mechanism (finality gadget)
- ✅ Weak subjectivity requirements
- ✅ Social consensus on canonical chain

**Residual Risk**: Low (standard PoS mitigations)

---

#### 9. Denial of Service (Network Level)

**Attack**: Flood network with spam transactions or jobs.

**Mitigations**:
- ✅ Gas fees (economic spam deterrent)
- ✅ Rate limiting per address
- ✅ Priority fee market
- ✅ DDoS protection at infrastructure level

**Residual Risk**: Medium (always a concern for public networks)

---

#### 10. Smart Contract Vulnerabilities

**Attack**: Exploit bugs in governance, staking, or settlement contracts.

**Mitigations**:
- ✅ External security audits (planned Q1-Q2 2026)
- ✅ Formal verification (critical components)
- ✅ Bug bounty program (TBA)
- ✅ Timelock on governance changes
- ✅ Emergency pause mechanisms

**Residual Risk**: Medium (requires continuous auditing)

---

## Security Best Practices

### For Validators

1. **Key Management**
   - Use hardware security modules (HSM) for validator keys
   - Never share private keys
   - Implement multi-sig for high-value operations

2. **Infrastructure Security**
   - Keep nodes updated with latest patches
   - Use firewall rules to restrict access
   - Monitor for unusual activity
   - Implement DDoS protection

3. **Operational Security**
   - Diversify validator infrastructure across regions
   - Use monitoring and alerting systems
   - Have incident response plan
   - Participate in fraud detection

### For Workers

1. **Determinism Assurance**
   - Test jobs for deterministic behavior
   - Use pinned dependencies and versions
   - Document execution environment requirements

2. **Data Availability**
   - Ensure reliable storage backend
   - Implement redundancy
   - Monitor DA health metrics
   - Respond quickly to DA challenges

3. **Stake Management**
   - Only stake what you can afford to lose
   - Understand slashing conditions
   - Monitor reputation scores
   - Maintain high uptime

### For Clients (Job Submitters)

1. **Job Specification**
   - Clearly define determinism requirements
   - Set appropriate SLA parameters
   - Use reasonable timeout values

2. **Result Verification**
   - Implement application-level checks
   - Use fraud-proof mechanism if suspicious
   - Monitor worker reputation

3. **Economic Security**
   - Understand pricing mechanism
   - Set appropriate budgets
   - Monitor for price anomalies

---

## Audits and Reviews

### Completed Audits
- None yet (testnet phase)

### Planned Audits

| Component | Auditor | Timeline | Status |
|-----------|---------|----------|--------|
| Core Consensus (PoPC) | TBD | Q1 2026 | Planned |
| ASR & PPC | TBD | Q1 2026 | Planned |
| Smart Contracts | TBD | Q2 2026 | Planned |
| Cryptography (VRF) | TBD | Q2 2026 | Planned |
| DA Layer | TBD | Q2 2026 | Planned |

### Bug Bounty Program

**Status**: Coming Soon

We plan to launch a bug bounty program before mainnet with rewards for:
- Critical vulnerabilities: Up to $100,000
- High severity: Up to $50,000
- Medium severity: Up to $10,000
- Low severity: Up to $1,000

Details will be published at: https://axionax.io/bounty

---

## Security Updates

### Notification Channels
- **Security Advisory**: https://github.com/axionaxprotocol/axionax-core/security/advisories
- **Discord #security**: https://discord.gg/axionax
- **Twitter**: @axionaxprotocol
- **Email Newsletter**: Subscribe at https://axionax.io

### Update Policy
- Critical patches: Released immediately
- Security patches: Released within 7 days
- Non-security updates: Regular release cycle

---

## Contact

- **General Security**: security@axionax.io
- **PGP Key**: https://axionax.io/security.asc
- **Security Page**: https://axionax.io/security

---

Last Updated: 2025-10-21 | v1.5.0
