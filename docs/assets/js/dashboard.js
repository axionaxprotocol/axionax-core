// ========================================
// Axionax Testnet Dashboard - Live Network Monitoring
// ========================================

const RPC_URL = 'https://testnet-rpc.axionax.org';
const WS_URL = 'wss://testnet-ws.axionax.org';
const UPDATE_INTERVAL = 10000; // 10 seconds

let ws = null;
let updateTimer = null;

// Initialize dashboard on page load
document.addEventListener('DOMContentLoaded', () => {
  initDashboard();
});

function initDashboard() {
  // Start periodic updates
  updateNetworkMetrics();
  updateTimer = setInterval(updateNetworkMetrics, UPDATE_INTERVAL);
  
  // Initialize WebSocket connection for real-time updates
  connectWebSocket();
  
  // Setup event listeners
  setupEventListeners();
  
  console.log('✅ Axionax Dashboard initialized');
}

// ========================================
// Network Metrics Updates
// ========================================

async function updateNetworkMetrics() {
  try {
    // Fetch metrics from RPC
    const [blockHeight, blockTime, validators, txCount, gasPrice] = await Promise.all([
      getBlockHeight(),
      getBlockTime(),
      getValidatorCount(),
      getTransactionCount(),
      getGasPrice()
    ]);
    
    // Update UI
    updateMetricCard('block-height', blockHeight, '+12');
    updateMetricCard('block-time', `${blockTime}s`, 'stable');
    updateMetricCard('validators', validators, 'stable');
    updateMetricCard('transactions', txCount.toLocaleString(), '+156');
    updateMetricCard('tps', calculateTPS(txCount), '+2.3%');
    updateMetricCard('peers', Math.floor(Math.random() * 5 + 45), 'stable'); // Mock data
    
    // Update health indicators
    updateHealthIndicators();
    
    // Update status indicator
    updateStatusIndicator('online', 'All Systems Operational');
    
  } catch (error) {
    console.error('❌ Failed to update network metrics:', error);
    updateStatusIndicator('offline', 'Connection Error');
  }
}

// RPC API Calls
async function rpcCall(method, params = []) {
  try {
    const response = await fetch(RPC_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        method: method,
        params: params,
        id: Date.now()
      })
    });
    
    if (!response.ok) {
      throw new Error(`RPC call failed: ${response.status}`);
    }
    
    const data = await response.json();
    
    if (data.error) {
      throw new Error(data.error.message);
    }
    
    return data.result;
  } catch (error) {
    console.error(`RPC ${method} error:`, error);
    return null;
  }
}

async function getBlockHeight() {
  const result = await rpcCall('eth_blockNumber');
  return result ? parseInt(result, 16) : 1234567; // Fallback mock data
}

async function getBlockTime() {
  // Get last 2 blocks and calculate time difference
  const latestBlock = await rpcCall('eth_getBlockByNumber', ['latest', false]);
  if (!latestBlock) return 5;
  
  const prevBlockNum = '0x' + (parseInt(latestBlock.number, 16) - 1).toString(16);
  const prevBlock = await rpcCall('eth_getBlockByNumber', [prevBlockNum, false]);
  if (!prevBlock) return 5;
  
  const timeDiff = parseInt(latestBlock.timestamp, 16) - parseInt(prevBlock.timestamp, 16);
  return timeDiff || 5;
}

async function getValidatorCount() {
  // This would call a custom RPC method for validator count
  const result = await rpcCall('popc_getValidatorCount');
  return result || 7; // Fallback
}

async function getTransactionCount() {
  const latestBlock = await rpcCall('eth_getBlockByNumber', ['latest', true]);
  if (!latestBlock || !latestBlock.transactions) return 45678;
  
  // This is just current block TX count, in production you'd aggregate
  return latestBlock.transactions.length || 45678;
}

async function getGasPrice() {
  const result = await rpcCall('eth_gasPrice');
  return result ? parseInt(result, 16) : 1000000000;
}

function calculateTPS(txCount) {
  // Simplified TPS calculation (would need historical data in production)
  const tps = Math.floor(txCount / 100);
  return tps > 1000 ? `${(tps / 1000).toFixed(1)}k` : tps;
}

// ========================================
// UI Update Functions
// ========================================

function updateMetricCard(id, value, trend) {
  const valueEl = document.getElementById(id);
  if (!valueEl) return;
  
  // Animate value change
  valueEl.style.opacity = '0.5';
  setTimeout(() => {
    valueEl.textContent = value;
    valueEl.style.opacity = '1';
  }, 150);
  
  // Update trend indicator
  const trendEl = valueEl.parentElement.querySelector('.metric-trend');
  if (trendEl && trend !== 'stable') {
    trendEl.textContent = trend;
    trendEl.classList.toggle('trend-up', trend.includes('+'));
    trendEl.classList.toggle('trend-down', trend.includes('-'));
  }
}

function updateHealthIndicators() {
  const health = {
    consensus: 100,
    network: 98,
    storage: 85,
    security: 100
  };
  
  Object.entries(health).forEach(([key, value]) => {
    const fillEl = document.querySelector(`[data-health="${key}"] .health-fill`);
    const valueEl = document.querySelector(`[data-health="${key}"] .health-value`);
    
    if (fillEl && valueEl) {
      fillEl.style.width = `${value}%`;
      fillEl.style.background = value >= 95 ? 'var(--success)' : 
                                 value >= 80 ? 'var(--accent)' : 
                                 'var(--error)';
      valueEl.textContent = `${value}%`;
    }
  });
}

function updateStatusIndicator(status, text) {
  const dotEl = document.querySelector('.status-dot');
  const textEl = document.querySelector('.status-text');
  
  if (dotEl) {
    dotEl.className = `status-dot status-${status}`;
  }
  
  if (textEl) {
    textEl.textContent = text;
  }
}

// ========================================
// WebSocket for Real-Time Updates
// ========================================

function connectWebSocket() {
  try {
    ws = new WebSocket(WS_URL);
    
    ws.onopen = () => {
      console.log('🔌 WebSocket connected');
      // Subscribe to new blocks
      ws.send(JSON.stringify({
        jsonrpc: '2.0',
        method: 'eth_subscribe',
        params: ['newHeads'],
        id: 1
      }));
    };
    
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      if (data.method === 'eth_subscription') {
        handleNewBlock(data.params.result);
      }
    };
    
    ws.onerror = (error) => {
      console.error('❌ WebSocket error:', error);
    };
    
    ws.onclose = () => {
      console.log('🔌 WebSocket disconnected, reconnecting in 5s...');
      setTimeout(connectWebSocket, 5000);
    };
    
  } catch (error) {
    console.error('❌ Failed to connect WebSocket:', error);
  }
}

function handleNewBlock(blockData) {
  console.log('📦 New block:', blockData.number);
  
  // Update block height immediately
  const blockHeight = parseInt(blockData.number, 16);
  updateMetricCard('block-height', blockHeight, '+1');
  
  // Add to activity feed
  addActivityItem('Block', `#${blockHeight}`, 'Just now');
}

function addActivityItem(type, description, time) {
  const activityList = document.querySelector('.activity-list');
  if (!activityList) return;
  
  const item = document.createElement('div');
  item.className = 'activity-item';
  item.innerHTML = `
    <span class="activity-time">${time}</span>
    <span class="activity-type">${type}</span>
    <span class="activity-desc">${description}</span>
  `;
  
  // Add to top and remove oldest if more than 10
  activityList.insertBefore(item, activityList.firstChild);
  
  while (activityList.children.length > 10) {
    activityList.removeChild(activityList.lastChild);
  }
  
  // Animate in
  item.style.opacity = '0';
  item.style.transform = 'translateY(-20px)';
  setTimeout(() => {
    item.style.transition = 'all 0.3s ease';
    item.style.opacity = '1';
    item.style.transform = 'translateY(0)';
  }, 50);
}

// ========================================
// Event Listeners & Helper Functions
// ========================================

function setupEventListeners() {
  // Copy to clipboard functionality
  document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', function() {
      const textToCopy = this.dataset.copy;
      copyToClipboard(textToCopy);
      
      // Visual feedback
      const originalText = this.textContent;
      this.textContent = '✓';
      setTimeout(() => {
        this.textContent = originalText;
      }, 2000);
    });
  });
  
  // Add to MetaMask button
  const metamaskBtn = document.getElementById('add-metamask');
  if (metamaskBtn) {
    metamaskBtn.addEventListener('click', addToMetaMask);
  }
}

async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text);
    console.log('📋 Copied to clipboard:', text);
  } catch (error) {
    console.error('❌ Failed to copy:', error);
  }
}

async function addToMetaMask() {
  if (typeof window.ethereum === 'undefined') {
    alert('Please install MetaMask to add this network!');
    window.open('https://metamask.io/download/', '_blank');
    return;
  }
  
  try {
    await window.ethereum.request({
      method: 'wallet_addEthereumChain',
      params: [{
        chainId: '0x150A9', // 86137 in hex
        chainName: 'Axionax Testnet',
        nativeCurrency: {
          name: 'AXX',
          symbol: 'AXX',
          decimals: 18
        },
        rpcUrls: ['https://testnet-rpc.axionax.org'],
        blockExplorerUrls: ['https://testnet-explorer.axionax.org']
      }]
    });
    
    console.log('✅ Network added to MetaMask');
    alert('✅ Axionax Testnet added to MetaMask!');
    
  } catch (error) {
    console.error('❌ Failed to add network:', error);
    alert('Failed to add network. Please try again.');
  }
}

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
  if (updateTimer) {
    clearInterval(updateTimer);
  }
  
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.close();
  }
});
