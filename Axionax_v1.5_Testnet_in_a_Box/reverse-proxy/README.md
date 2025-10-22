Edge reverse proxy (Nginx) for public access

Overview
- Terminates HTTPS and routes to internal services
- Adds CORS for RPC, Faucet, and Blockscout API
- Optional path-based access to Blockscout frontend at /explorer

Ports
- 80: HTTP (redirects to HTTPS)
- 443: HTTPS

TLS
- This folder expects certs at reverse-proxy/certs:
  - fullchain.pem
  - privkey.pem
- For testing, you can generate a self-signed pair with the provided script.
- For production, use Let's Encrypt (Certbot) or your CA and place files here.

Paths
- /           -> ui:80 (static UI)
- /rpc/       -> hardhat:8545 (Anvil RPC) with CORS
- /faucet/    -> faucet:8081 (Faucet) with CORS
- /blockscout-api/ -> blockscout:4000 (Explorer API) with CORS
- /explorer/  -> blockscout-frontend:80 (Explorer UI)

Security tips
- Consider Basic Auth or IP allowlists on /faucet/ in production.
- Add rate limiting for /rpc/ and /faucet/ if exposing to the internet.
- Put explorer behind Cloudflare/NGINX WAF if possible.
