# Axionax Core API Reference

Version: 1.5.0-testnet

## Overview

Axionax Core provides JSON-RPC APIs compatible with Ethereum clients, plus custom extensions for PoPC, ASR, and protocol-specific features.

## Endpoints

### Standard JSON-RPC (Port 8545)

Standard Ethereum JSON-RPC methods are supported.

### WebSocket (Port 8546)

Real-time event subscriptions.

### Metrics (Port 9090)

Prometheus-compatible metrics endpoint.

---

## Standard Ethereum Methods

### eth_chainId

Returns the current chain ID.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "eth_chainId",
  "params": [],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x7a69",
  "id": 1
}
```

### eth_getBalance

Get AXX balance of an address.

**Parameters:**
1. `address` - Address to check
2. `block` - Block number or "latest"

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "eth_getBalance",
  "params": ["0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", "latest"],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x1b1ae4d6e2ef500000",
  "id": 1
}
```

---

## Axionax Custom Methods

### axn_submitJob

Submit a compute job to the network.

**Parameters:**
1. `job` - Job specification object

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_submitJob",
  "params": [{
    "specs": {
      "gpu": "NVIDIA RTX 4090",
      "vram": 24,
      "framework": "PyTorch",
      "region": "us-west"
    },
    "sla": {
      "max_latency": "30s",
      "max_retries": 3,
      "timeout": "300s",
      "required_uptime": 0.99
    },
    "data": "0x..."
  }],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "job_id": "job_abc123",
    "assigned_worker": "0x...",
    "price": "0.1",
    "status": "assigned"
  },
  "id": 1
}
```

### axn_getJobStatus

Get status of a submitted job.

**Parameters:**
1. `job_id` - Job identifier

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_getJobStatus",
  "params": ["job_abc123"],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "job_id": "job_abc123",
    "status": "completed",
    "worker": "0x...",
    "output_root": "0x...",
    "popc_passed": true,
    "completed_at": "2025-10-22T10:30:00Z"
  },
  "id": 1
}
```

### axn_registerWorker

Register as a compute worker.

**Parameters:**
1. `specs` - Worker hardware specifications

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_registerWorker",
  "params": [{
    "gpus": [{
      "model": "NVIDIA RTX 4090",
      "vram": 24,
      "count": 1
    }],
    "cpu_cores": 16,
    "ram": 64,
    "storage": 1000,
    "bandwidth": 1000,
    "region": "us-west",
    "asn": "AS15169",
    "organization": "example-org"
  }],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "address": "0x...",
    "status": "active",
    "registered_at": "2025-10-22T10:00:00Z"
  },
  "id": 1
}
```

### axn_getWorkerStatus

Get worker status and statistics.

**Parameters:**
1. `address` - Worker address

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_getWorkerStatus",
  "params": ["0x..."],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "address": "0x...",
    "status": "active",
    "performance": {
      "total_jobs": 567,
      "successful_jobs": 564,
      "failed_jobs": 3,
      "popc_pass_rate": 0.995,
      "da_reliability": 0.998,
      "avg_latency": 12.5,
      "uptime": 0.997
    },
    "quota_used": 0.082,
    "reputation": 0.96
  },
  "id": 1
}
```

### axn_getPricingInfo

Get current pricing information from PPC.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_getPricingInfo",
  "params": [],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "current_price": 0.15,
    "utilization": 0.68,
    "queue_time": 55.2,
    "target_utilization": 0.7,
    "target_queue_time": 60.0,
    "min_price": 0.001,
    "max_price": 10.0
  },
  "id": 1
}
```

### axn_getValidatorInfo

Get validator information.

**Parameters:**
1. `address` - Validator address

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_getValidatorInfo",
  "params": ["0x..."],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "address": "0x...",
    "stake": "10000",
    "status": "active",
    "total_votes": 1234,
    "correct_votes": 1230,
    "false_pass": 4,
    "commission": 0.05
  },
  "id": 1
}
```

### axn_getPoPCChallenge

Get PoPC challenge for a job (validators only).

**Parameters:**
1. `job_id` - Job identifier

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_getPoPCChallenge",
  "params": ["job_abc123"],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "job_id": "job_abc123",
    "samples": [12, 45, 78, 123, ...],
    "vrf_seed": "0x...",
    "block_delay": 2
  },
  "id": 1
}
```

### axn_submitPoPCProof

Submit PoPC proof (workers only).

**Parameters:**
1. `proof` - PoPC proof object

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "axn_submitPoPCProof",
  "params": [{
    "job_id": "job_abc123",
    "samples": {
      "12": "0x...",
      "45": "0x...",
      ...
    },
    "merkle_paths": {
      "12": ["0x...", "0x...", ...],
      "45": ["0x...", "0x...", ...],
      ...
    },
    "output_root": "0x..."
  }],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "job_id": "job_abc123",
    "passed": true,
    "samples_verified": 1000,
    "samples_total": 1000,
    "confidence": 0.9995
  },
  "id": 1
}
```

---

## WebSocket Subscriptions

### Subscribe to New Jobs

```json
{
  "jsonrpc": "2.0",
  "method": "axn_subscribe",
  "params": ["newJobs"],
  "id": 1
}
```

### Subscribe to Job Updates

```json
{
  "jsonrpc": "2.0",
  "method": "axn_subscribe",
  "params": ["jobUpdates", {"job_id": "job_abc123"}],
  "id": 1
}
```

### Subscribe to Price Updates

```json
{
  "jsonrpc": "2.0",
  "method": "axn_subscribe",
  "params": ["priceUpdates"],
  "id": 1
}
```

---

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Request is not valid JSON-RPC |
| -32601 | Method not found | Method does not exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal JSON-RPC error |
| -32000 | Job not found | Job ID does not exist |
| -32001 | Worker not found | Worker address not registered |
| -32002 | Insufficient stake | Not enough staked AXX |
| -32003 | Invalid specs | Hardware specs don't meet requirements |
| -32004 | Quota exceeded | Worker has exceeded epoch quota |
| -32005 | Validation failed | PoPC validation failed |

---

## Rate Limits

- Standard RPC: 100 requests/second per IP
- WebSocket: 50 subscriptions per connection
- Burst: 200 requests in 10 seconds

---

## Authentication

Most endpoints are public. Sensitive operations require signed transactions:

```json
{
  "from": "0x...",
  "signature": "0x...",
  "nonce": 123,
  ...
}
```

---

## Examples

### Using curl

```bash
# Get chain ID
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Get pricing info
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getPricingInfo","params":[],"id":1}'
```

### Using Go

```go
import "github.com/ethereum/go-ethereum/rpc"

client, _ := rpc.Dial("http://localhost:8545")

var result string
client.Call(&result, "eth_chainId")
```

### Using JavaScript

```javascript
const Web3 = require('web3');
const web3 = new Web3('http://localhost:8545');

// Standard methods
const chainId = await web3.eth.getChainId();

// Custom methods
const pricing = await web3.currentProvider.send('axn_getPricingInfo', []);
```

---

## Additional Resources

- [Testnet Integration Guide](./TESTNET_INTEGRATION.md)
- [Architecture Overview](../ARCHITECTURE.md)
- [GitHub Repository](https://github.com/axionaxprotocol/axionax-core)
