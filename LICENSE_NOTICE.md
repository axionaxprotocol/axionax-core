# ⚠️ IMPORTANT LEGAL NOTICE

## License Summary

Axionax Core is licensed under **GNU Affero General Public License v3.0 (AGPLv3)**
with additional terms to protect the Axionax Network and community.

📄 **Full license:** [LICENSE](./LICENSE)

---

## 🔒 Key Restrictions

### 1. Trademark Protection

**Protected marks:**
- "Axionax" and "Axionax Protocol"
- "AXX" and "AXX Token"
- Axionax logo and branding materials

**Unauthorized uses:**
- ❌ Operating networks claiming to be "official Axionax"
- ❌ Creating tokens branded as "AXX" without authorization
- ❌ Using Axionax branding to misrepresent derivative works

**Authorized uses:**
- ✅ Factual references in documentation
- ✅ Academic research and education
- ✅ Contributing to the official Axionax project

**Contact for authorization:** legal@axionax.org

---

### 2. Mainnet Launch Restriction

This codebase may be used for:
- ✅ **Development and testing** (local networks, testnets)
- ✅ **Private networks** (internal use, research)
- ✅ **Open source contributions** to the official project

**PROHIBITED without explicit written authorization:**
- ❌ Launching **public mainnets** with economic tokens
- ❌ Operating networks that could **confuse users** with official Axionax
- ❌ Creating **token systems** claiming to be "AXX"

**Why this restriction?**
- Protects users from fake/scam networks
- Ensures quality and security standards
- Maintains ecosystem integrity

**Request authorization:** mainnet-auth@axionax.org

---

### 3. Chain Identity Requirements

All forks and derivatives MUST:
- 🆔 Use a **different chain ID** than official networks
- 🔗 Generate a **different genesis hash**
- 🏷️ Use a **clearly different name** in all interfaces
- ⚠️ Display **prominent warnings** that it's NOT official Axionax

**Official chain IDs (RESERVED):**
```yaml
Testnet:  86137  # AXI testnet (active)
Mainnet:  86150  # AXI mainnet (not launched yet)
```

**Example of compliant fork naming:**
```
✅ "MyChain Network (based on Axionax technology)"
✅ "Axionax Fork for Research Purposes"
❌ "Axionax Mainnet" (PROHIBITED)
❌ "AXX Chain" (PROHIBITED)
```

---

## 🛡️ Why These Restrictions?

### User Protection
Prevents confusion between official networks and unauthorized clones that may:
- Lack security audits
- Have malicious modifications
- Misrepresent token economics
- Scam users with fake "AXX" tokens

### Ecosystem Integrity
Ensures:
- Quality control for production deployments
- Consistent user experience
- Trustworthy network operations
- Clear accountability

### Legal Compliance
Protects against:
- Trademark infringement
- Consumer fraud
- Securities law violations (if unauthorized tokens are marketed)

---

## ✅ What You CAN Do

### Development & Testing
```bash
# Run local development network
cargo run -- --chain-id 1337 --dev

# Join the official testnet
cargo run -- --chain-id 86137 --testnet
```

### Research & Education
- Fork for academic research
- Modify for learning purposes
- Experiment with consensus mechanisms
- Test security assumptions

### Contributing
- Submit pull requests
- Report security issues
- Improve documentation
- Build developer tools

### Private Networks
- Internal corporate networks
- Consortium blockchains (with different branding)
- Research networks (clearly marked as derivatives)

---

## 🚫 What You CANNOT Do

### Without Authorization

❌ **Clone and launch as "Axionax Mainnet 2.0"**
```bash
# This violates trademark and mainnet restrictions
git clone axionax-core
sed -i 's/86137/99999/' config.yaml
./launch-mainnet.sh --name "Axionax Mainnet 2.0"
```

❌ **Create "AXX Token" on your fork**
```solidity
// This violates trademark protection
contract AXXToken {
    string public name = "Axionax Token";  // PROHIBITED
    string public symbol = "AXX";          // PROHIBITED
}
```

❌ **Market unauthorized network as "official"**
```markdown
# This is fraudulent
## Welcome to Axionax Official Mainnet
Buy AXX tokens now! Only $0.01 each!
```

---

## 📞 Contact & Reporting

### Authorization Requests
- **Mainnet launch:** mainnet-auth@axionax.org
- **Trademark usage:** legal@axionax.org

### Security Issues
- **Vulnerabilities:** security@axionax.org
- **Impersonation/clones:** security@axionax.org
- **PGP key:** https://axionax.org/pgp-key.asc

### Community
- **Discord:** https://discord.gg/axionax
- **Forum:** https://forum.axionax.org
- **Governance:** https://gov.axionax.org

---

## 🔍 Verify Official Networks

**Before connecting to any "Axionax" network:**

1. **Check the official registry:**
   https://axionax.org/networks

2. **Verify genesis hash:**
   ```bash
   axionax-cli verify-genesis --chain-id 86137
   ```

3. **Confirm on official channels:**
   - Website: https://axionax.org
   - Twitter: @AxionaxProtocol
   - Discord: https://discord.gg/axionax

**🚨 WARNING:** Any network NOT listed on https://axionax.org/networks is **UNOFFICIAL** and potentially fraudulent.

---

## ⚖️ Enforcement

Violations of these terms may result in:

1. **Cease and desist** notices
2. **DMCA takedown** requests
3. **Trademark infringement** lawsuits
4. **Consumer fraud** reports to authorities
5. **Community warnings** and blacklisting

We take these restrictions seriously to protect our community.

---

## 📚 Additional Resources

- **Full License:** [LICENSE](./LICENSE)
- **Security Policy:** [SECURITY.md](./SECURITY.md)
- **Contribution Guide:** [CONTRIBUTING.md](./CONTRIBUTING.md)
- **Code of Conduct:** [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
- **Governance:** [GOVERNANCE.md](./GOVERNANCE.md)

---

## 🤝 We Support Open Source

These restrictions do NOT prevent:
- Learning from the code
- Contributing improvements
- Forking for research
- Building compatible tools
- Running private networks

We believe in open source AND protecting our community from fraud.

**Questions?** legal@axionax.org

---

**Last updated:** October 24, 2025  
**License version:** AGPLv3 with Axionax Network Protection Clause v1.0
