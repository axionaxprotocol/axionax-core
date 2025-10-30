# Axionax DAO Governance

## Overview

Axionax is governed by a decentralized autonomous organization (DAO) where AXX token holders collectively make decisions about the protocol's future.

---

## 🏛️ Governance Structure

### Participants

**AXX Holders**
- Anyone holding AXX tokens
- Voting power proportional to staked AXX
- Can submit and vote on proposals

**Core Contributors**
- Founding team and advisors
- Implement approved proposals
- Provide technical guidance
- Subject to same governance rules

**Delegates**
- Token holders can delegate voting power
- Active community members
- Represent delegators' interests

---

## 🗳️ Voting Mechanism

### Voting Power

```
Voting Power = Staked AXX × Time Multiplier

Time Multiplier:
- < 1 month: 1.0x
- 1-3 months: 1.2x
- 3-6 months: 1.5x
- > 6 months: 2.0x
```

### Quorum Requirements

| Proposal Type | Quorum | Approval Threshold |
|---------------|--------|-------------------|
| Parameter Adjustment | 10% | 66% |
| Protocol Upgrade | 20% | 75% |
| Treasury Allocation | 15% | 66% |
| Emergency Action | 30% | 80% |
| Constitution Change | 40% | 90% |

### Voting Period

- **Discussion Period**: 7 days
- **Voting Period**: 7 days
- **Timelock**: 2-7 days (depending on proposal type)
- **Total**: ~16-21 days

---

## 📋 Proposal Process

### 1. Ideation Phase (Forum Discussion)

**Where**: https://forum.axionax.org

**Requirements**:
- Post in "Governance Ideas" category
- Clear problem statement
- Proposed solution
- Expected impact
- Open for community feedback

**Duration**: Minimum 3 days

### 2. Temperature Check (Off-chain Vote)

**Where**: Snapshot (https://snapshot.org/#/axionax.eth)

**Requirements**:
- Summary of forum discussion
- Refined proposal
- Poll options

**Success Criteria**: >50% approval, >5% quorum

**Duration**: 3 days

### 3. Formal Proposal (On-chain)

**Requirements**:
- Passed temperature check
- Minimum stake: 100,000 AXX
- Complete proposal template
- Implementation plan

**Submission**: Via governance dashboard or CLI
```bash
axionax-cli governance propose \
  --title "Adjust ASR parameter K to 128" \
  --description "..." \
  --type parameter-adjustment \
  --stake 100000
```

### 4. Voting Period

**Actions**:
- Token holders vote (For/Against/Abstain)
- Live results visible
- Vote can be changed during period

```bash
axionax-core governance vote \
  --proposal-id 42 \
  --choice for
```

### 5. Timelock

**Purpose**:
- Allow time for security review
- Enable emergency intervention if needed
- Prepare for execution

**Duration**:
- Parameter changes: 2 days
- Smart contract upgrades: 7 days
- Emergency actions: 0 days (but require higher threshold)

### 6. Execution

**Automatic Execution**:
- Parameter changes
- Treasury transfers

**Manual Execution** (requires multi-sig):
- Protocol upgrades
- Smart contract deployments

---

## 📐 Proposal Types

### 1. Parameter Adjustments

**Examples**:
- Change ASR parameter K (Top-K size)
- Adjust PoPC sample size s
- Modify PPC alpha/beta sensitivity
- Update slashing rates

**Template**:
```markdown
### Parameter Adjustment Proposal

**Parameter**: ASR Top-K size (K)
**Current Value**: 64
**Proposed Value**: 128
**Rationale**: Increase worker selection pool to improve fairness
**Impact Analysis**: 
- Increased compute overhead: ~2%
- Improved newcomer access: ~15%
- Network effect: Positive
**Implementation**: Immediate (on-chain parameter update)
```

### 2. Protocol Upgrades

**Examples**:
- Consensus mechanism improvements
- New feature additions
- Security patches
- Performance optimizations

**Template**:
```markdown
### Protocol Upgrade Proposal

**Title**: Implement Adaptive Sample Escalation v2
**Description**: Enhanced adaptive sampling with ML-based anomaly detection
**Specification**: [Link to technical spec]
**Security Audit**: [Link to audit report]
**Breaking Changes**: None
**Migration Plan**: Automatic, no user action required
**Timeline**: 
- Implementation: 4 weeks
- Testnet deployment: Week 5
- Mainnet deployment: Week 8
```

### 3. Treasury Allocation

**Examples**:
- Grant programs
- Partnership funding
- Marketing initiatives
- Development bounties

**Template**:
```markdown
### Treasury Allocation Proposal

**Recipient**: [Name/Team]
**Amount**: 10,000,000 AXX ($100,000 at current price)
**Purpose**: Build AI training marketplace on Axionax
**Milestones**:
1. MVP (25%): 2,500,000 AXX
2. Beta launch (50%): 5,000,000 AXX
3. Production + 1000 users (25%): 2,500,000 AXX
**Reporting**: Monthly progress reports to DAO
**KPIs**: 
- Launch date: Q2 2026
- User growth: 1000+ in 6 months
- Compute volume: 10k jobs/month
```

### 4. Emergency Actions

**Examples**:
- Pause protocol due to exploit
- Emergency parameter changes
- Critical bug fixes

**Requirements**:
- Multi-sig execution (5-of-9)
- Higher quorum (30%)
- Faster voting (48 hours)
- Immediate execution (no timelock)

---

## 🛡️ Multi-Sig Council

### Composition

**9 Members**:
- 3 Core team members
- 3 Community members (elected)
- 3 Strategic partners

**Term**: 12 months, staggered

**Responsibilities**:
- Execute approved proposals
- Emergency response
- Contract upgrades
- Treasury management (large amounts)

### Powers

**Can Do**:
- Execute DAO-approved proposals
- Emergency pause (24-hour limit)
- Bug bounty payouts (<$50k)

**Cannot Do**:
- Change parameters without DAO approval
- Allocate treasury funds without DAO approval
- Upgrade protocol without DAO approval

---

## 📊 Governed Parameters

### PoPC Consensus

| Parameter | Current | Range | Description |
|-----------|---------|-------|-------------|
| Sample size (s) | 1000 | 600-1500 | Challenge sample count |
| Redundancy (β) | 3% | 2-5% | Replica job percentage |
| VRF delay (k) | 2 blocks | 2-10 | Challenge delay |
| Fraud window (Δt) | 3600s | 1800-7200s | Fraud claim period |
| Slash rate (fraud) | 100% | 100% | Worker fraud penalty |
| Slash rate (false-pass) | 5% | 3-10% | Validator penalty |

### ASR Router

| Parameter | Current | Range | Description |
|-----------|---------|-------|-------------|
| Top-K size (K) | 64 | 32-256 | Selection pool size |
| Quota max (q_max) | 15% | 10-25% | Per-worker max jobs |
| Epsilon (ε) | 5% | 3-10% | Newcomer exploration |

### PPC Pricing

| Parameter | Current | Range | Description |
|-----------|---------|-------|-------------|
| Alpha (α) | 0.5 | 0.1-2.0 | Util sensitivity |
| Beta (β) | 0.3 | 0.1-1.0 | Queue sensitivity |
| Target util (util*) | 0.7 | 0.5-0.85 | Target utilization |
| Target queue (q*) | 60s | 30-300s | Target queue time |

### Economic

| Parameter | Current | Range | Description |
|-----------|---------|-------|-------------|
| Validator min stake | 100,000 AXX | 50k-500k | Min validator stake |
| Worker stake ratio | 15% | 10-30% | Stake % of job value |
| Protocol fee | 5% | 3-10% | Job fee percentage |
| Emission rate | 2.25% APY | 1-5% | Staking reward rate |

---

## 🔔 Staying Informed

### Notifications

- **Forum**: https://forum.axionax.org
- **Discord #governance**: https://discord.gg/axionax
- **Twitter**: @axionaxprotocol
- **Governance Dashboard**: https://gov.axionax.org
- **Snapshot**: https://snapshot.org/#/axionax.eth

### Participation

**Low Effort**:
- Follow forum discussions
- Vote on proposals
- Delegate to active members

**Medium Effort**:
- Provide feedback on proposals
- Participate in temperature checks
- Join community calls

**High Effort**:
- Submit proposals
- Become delegate
- Join working groups
- Run for multi-sig council

---

## 🎓 Governance Resources

### Documentation
- [Governance Portal](https://gov.axionax.org)
- [How to Vote Guide](https://docs.axionax.org/governance/voting)
- [Parameter Descriptions](https://docs.axionax.org/governance/parameters)
- [Proposal Templates](https://github.com/axionaxprotocol/governance-proposals)

### Tools
- [Governance Dashboard](https://gov.axionax.org)
- [Snapshot Voting](https://snapshot.org/#/axionax.eth)
- [Forum](https://forum.axionax.org)
- [Delegation Platform](https://delegate.axionax.org)

---

## 📜 Constitution

### Core Principles

1. **Decentralization**: Progressively decentralize control
2. **Transparency**: All decisions public and auditable
3. **Security**: Safety and correctness above all
4. **Fairness**: Equal opportunity for all participants
5. **Sustainability**: Long-term viability over short-term gains

### Immutable Rules

These require 90% approval to change:

1. **Fixed Supply**: 1 Trillion AXX, no inflation
2. **Consensus Type**: PoPC-based validation
3. **Open Source**: Code remains open source (MIT)
4. **Censorship Resistance**: No privileged access
5. **Community Governance**: DAO remains in control

---

## 🤝 Get Involved

**Ready to participate?**

1. Acquire AXX tokens
2. Stake tokens for voting power
3. Join Discord #governance
4. Follow forum discussions
5. Vote on your first proposal!

**Questions?** Ask in Discord or forum.

---

Last Updated: 2025-01-21 | v1.5.0
