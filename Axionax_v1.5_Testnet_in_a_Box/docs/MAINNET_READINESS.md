# Axionax Mainnet Readiness Checklist

## D−30 ~ D−7
- [ ] Freeze genesis/params, audit PoPC timers/fees/slash
- [ ] Key ceremony (validator, DA, signer), publish bootstraps & faucet policy
- [ ] Dry-run cutover (canary), data-wipe rehearsal, SIEM rules on P0 metrics

## D−6 ~ D−1
- [ ] Freeze build artifacts (axionaxd, predeploys, precompiles)
- [ ] Publish genesis.json / params.toml / peers.txt / wallet_add_network.json
- [ ] Spin up explorers, indexers, mirrors, status page

## D-Day
- [ ] Launch genesis validators, open RPC (read), enable tx after N blocks
- [ ] Start marketplace sealed-bid, DA mirrors, slashing live

## Go-Live Gates
- [ ] Finality < 180s (p95)
- [ ] da_miss_rate < 0.5%
- [ ] vrf_verify_fail ~ 0
- [ ] Orphan ≤ baseline+3σ
- [ ] SIEM alerts active, Slash path E2E tested
- [ ] Wallet add-network OK, explorer sync 100%, faucet rate-limit

## Files
- ops/configs/genesis.config.yaml
- ops/configs/params.toml
- ops/configs/peers.txt
- ops/configs/wallet_add_network.json
- ops/manifests/docker-compose.mainnet.yml
- ops/manifests/nginx.mainnet.conf
- ops/manifests/systemd/axionaxd.service
