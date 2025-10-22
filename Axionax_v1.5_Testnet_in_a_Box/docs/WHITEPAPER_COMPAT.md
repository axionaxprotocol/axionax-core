# Axionax Whitepaper v1.5 Compatibility Map (Testnet Mock)

Status legend: DONE (real), MOCK (simulated), PLANNED (not implemented)

- Protocol parameters (Appendix): DONE (config/protocol_params.json)
- Proto JSON-RPC surface:
  - proto_getParams: DONE (MOCK) → returns protocol_params.json
  - popc_submitCommit / popc_submitProve: DONE (MOCK)
  - da_precommit: DONE (MOCK)
  - asr_getAssignment: DONE (MOCK)
  - price_getCurrent: DONE (MOCK)
  - fraud_submitClaim: DONE (MOCK)
- Edge proxy /proto-rpc with CORS: DONE
- UI demo triggers: DONE (ui/index.html)
- Integration with on-chain contracts: PLANNED (future phases)
- Security hardening (auth/rate-limit for proto): PLANNED for public deployments

Notes
- The proto service is a lightweight Node.js mock for demo/testing on the local testnet.
- Replace mocks with real microservices as subsystems mature.

How to use
- Bring up the stack, open the UI, and use the "Protocol RPC (Mock)" buttons. Responses are logged in the panel.
- Programmatic calls: POST JSON-RPC 2.0 to /proto-rpc.
