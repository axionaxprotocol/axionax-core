import express from 'express';
import fs from 'fs';
import path from 'path';

const app = express();
const PORT = process.env.PORT || 8082;

app.use(express.json({ limit: '1mb' }));

const paramsPath = path.resolve('/workspace/config/protocol_params.json');
let params = null;
try {
  const altPath = path.resolve('/app/config/protocol_params.json'); // in case of different mount
  const p = fs.existsSync(paramsPath) ? paramsPath : (fs.existsSync(altPath) ? altPath : null);
  if (p) {
    params = JSON.parse(fs.readFileSync(p, 'utf8'));
  }
} catch (e) {
  // ignore
}

app.get('/health', (_req, res) => {
  res.json({ ok: true, service: 'proto-mock', version: '0.1.0' });
});

app.post('/', (req, res) => {
  const body = req.body || {};
  const id = body.id ?? null;
  const method = body.method;
  const result = handle(method, body.params);
  if (result && result.error) {
    return res.json({ jsonrpc: '2.0', id, error: result.error });
  }
  return res.json({ jsonrpc: '2.0', id, result });
});

function handle(method, paramsIn) {
  switch (method) {
    case 'proto_getParams':
      return params || { version: 'unknown' };
    case 'popc_submitCommit':
      return mockTx('popcCommit');
    case 'popc_submitProve':
      return mockTx('popcProve');
    case 'da_precommit':
      return { ok: true, ticket: rnd('da') };
    case 'asr_getAssignment':
      return { role: 'validator', slot: Date.now(), committee: [1, 2, 3].map(i => rnd('validator'+i)) };
    case 'price_getCurrent':
      return { symbol: 'AXX/USD', price: 1.0, ts: Date.now() };
    case 'fraud_submitClaim':
      return mockTx('fraudClaim');
    default:
      return { error: { code: -32601, message: 'Method not found' } };
  }
}

function mockTx(prefix) {
  return {
    ok: true,
    txHash: '0x' + rnd(prefix) + rnd(prefix),
    blockNumber: Math.floor(Date.now() / 1000)
  };
}

function rnd(seed = '') {
  const s = (Math.random().toString(16).slice(2) + Buffer.from(seed).toString('hex')).slice(0, 16);
  return s.padEnd(16, '0');
}

app.listen(PORT, () => {
  console.log(`[proto] listening on ${PORT}`);
});
