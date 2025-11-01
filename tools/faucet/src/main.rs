use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};

// Configuration
const FAUCET_AMOUNT: u64 = 100_000_000_000_000_000_000; // 100 AXX (in wei)
const COOLDOWN_HOURS: u64 = 24;
const MAX_REQUESTS_PER_IP: usize = 3;

/// Faucet state
#[derive(Clone)]
struct FaucetState {
    /// Map of address -> last request time
    address_requests: Arc<RwLock<HashMap<String, SystemTime>>>,
    /// Map of IP -> request count
    ip_requests: Arc<RwLock<HashMap<String, Vec<SystemTime>>>>,
    /// Faucet wallet private key
    private_key: String,
    /// RPC endpoint
    rpc_url: String,
    /// Chain ID
    chain_id: u64,
}

/// Request model
#[derive(Debug, Deserialize)]
struct FaucetRequest {
    address: String,
    #[serde(default)]
    captcha_token: Option<String>,
}

/// Response model
#[derive(Debug, Serialize)]
struct FaucetResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Stats response
#[derive(Debug, Serialize)]
struct StatsResponse {
    total_requests: usize,
    total_distributed: String,
    faucet_balance: String,
    cooldown_hours: u64,
    amount_per_request: String,
}

/// Error type
#[derive(Debug)]
enum FaucetError {
    InvalidAddress,
    TooSoon(Duration),
    RateLimited,
    InsufficientFunds,
    RpcError(String),
    InternalError(String),
}

impl IntoResponse for FaucetError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            FaucetError::InvalidAddress => {
                (StatusCode::BAD_REQUEST, "Invalid Ethereum address".to_string())
            }
            FaucetError::TooSoon(remaining) => {
                let hours = remaining.as_secs() / 3600;
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    format!("Please wait {} hours before requesting again", hours),
                )
            }
            FaucetError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many requests from this IP. Try again later.".to_string(),
            ),
            FaucetError::InsufficientFunds => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Faucet is currently out of funds. Please try again later.".to_string(),
            ),
            FaucetError::RpcError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("RPC error: {}", err),
            ),
            FaucetError::InternalError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", err),
            ),
        };

        let body = FaucetResponse {
            success: false,
            tx_hash: None,
            amount: None,
            message: None,
            error: Some(message),
        };

        (status, Json(body)).into_response()
    }
}

/// Health check endpoint
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "axionax-faucet",
        "version": "1.0.0"
    }))
}

/// Get faucet info
async fn info(State(state): State<FaucetState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "chain_id": state.chain_id,
        "amount": format!("{} AXX", FAUCET_AMOUNT / 1_000_000_000_000_000_000),
        "cooldown_hours": COOLDOWN_HOURS,
        "network": "Axionax Testnet"
    }))
}

/// Request tokens
async fn request_tokens(
    State(state): State<FaucetState>,
    client_ip: Option<String>,
    Json(payload): Json<FaucetRequest>,
) -> Result<impl IntoResponse, FaucetError> {
    let address = payload.address.trim().to_lowercase();
    
    // Validate address format
    if !is_valid_address(&address) {
        warn!("Invalid address format: {}", address);
        return Err(FaucetError::InvalidAddress);
    }

    // Check IP rate limiting
    if let Some(ip) = client_ip.as_ref() {
        let mut ip_requests = state.ip_requests.write().await;
        let now = SystemTime::now();
        
        // Clean old requests (older than 24 hours)
        let cutoff = now - Duration::from_secs(COOLDOWN_HOURS * 3600);
        
        let requests = ip_requests.entry(ip.clone()).or_insert_with(Vec::new);
        requests.retain(|&time| time > cutoff);
        
        if requests.len() >= MAX_REQUESTS_PER_IP {
            warn!("Rate limited IP: {}", ip);
            return Err(FaucetError::RateLimited);
        }
    }

    // Check address cooldown
    let mut address_requests = state.address_requests.write().await;
    let now = SystemTime::now();
    
    if let Some(&last_request) = address_requests.get(&address) {
        let elapsed = now.duration_since(last_request).unwrap_or(Duration::ZERO);
        let cooldown = Duration::from_secs(COOLDOWN_HOURS * 3600);
        
        if elapsed < cooldown {
            let remaining = cooldown - elapsed;
            warn!("Address {} requested too soon", address);
            return Err(FaucetError::TooSoon(remaining));
        }
    }

    // Send transaction
    info!("Sending {} AXX to {}", FAUCET_AMOUNT / 1_000_000_000_000_000_000, address);
    
    match send_transaction(&state, &address, FAUCET_AMOUNT).await {
        Ok(tx_hash) => {
            // Update request tracking
            address_requests.insert(address.clone(), now);
            
            if let Some(ip) = client_ip {
                let mut ip_requests = state.ip_requests.write().await;
                ip_requests.entry(ip).or_insert_with(Vec::new).push(now);
            }

            info!("✓ Sent {} AXX to {} (tx: {})", 
                  FAUCET_AMOUNT / 1_000_000_000_000_000_000, 
                  address, 
                  tx_hash);

            Ok(Json(FaucetResponse {
                success: true,
                tx_hash: Some(tx_hash),
                amount: Some(format!("{} AXX", FAUCET_AMOUNT / 1_000_000_000_000_000_000)),
                message: Some("Tokens sent successfully!".to_string()),
                error: None,
            }))
        }
        Err(e) => {
            error!("Failed to send transaction: {:?}", e);
            Err(e)
        }
    }
}

/// Get stats
async fn stats(State(state): State<FaucetState>) -> impl IntoResponse {
    let address_requests = state.address_requests.read().await;
    let total_requests = address_requests.len();
    let total_distributed = total_requests as u64 * FAUCET_AMOUNT;

    // Get faucet balance (mock for now)
    let faucet_balance = "1000 AXX"; // Would call RPC in production

    Json(StatsResponse {
        total_requests,
        total_distributed: format!("{} AXX", total_distributed / 1_000_000_000_000_000_000),
        faucet_balance: faucet_balance.to_string(),
        cooldown_hours: COOLDOWN_HOURS,
        amount_per_request: format!("{} AXX", FAUCET_AMOUNT / 1_000_000_000_000_000_000),
    })
}

/// Validate Ethereum address format
fn is_valid_address(address: &str) -> bool {
    address.starts_with("0x") && address.len() == 42 && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Send transaction via RPC
async fn send_transaction(
    state: &FaucetState,
    to_address: &str,
    amount: u64,
) -> Result<String, FaucetError> {
    // This is a simplified version
    // In production, you would:
    // 1. Get nonce from RPC
    // 2. Create and sign transaction
    // 3. Send via eth_sendRawTransaction
    
    // For now, return mock transaction hash
    let mock_tx_hash = format!(
        "0x{:x}",
        std::collections::hash_map::DefaultHasher::new()
    );
    
    // TODO: Implement actual transaction signing and sending
    // See: https://docs.rs/ethers/latest/ethers/
    
    Ok(mock_tx_hash)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration from environment
    let private_key = std::env::var("FAUCET_PRIVATE_KEY")
        .expect("FAUCET_PRIVATE_KEY must be set");
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://localhost:8545".to_string());
    let chain_id = std::env::var("CHAIN_ID")
        .unwrap_or_else(|_| "86137".to_string())
        .parse()
        .expect("Invalid CHAIN_ID");

    // Create state
    let state = FaucetState {
        address_requests: Arc::new(RwLock::new(HashMap::new())),
        ip_requests: Arc::new(RwLock::new(HashMap::new())),
        private_key,
        rpc_url: rpc_url.clone(),
        chain_id,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
        .route("/request", post(request_tokens))
        .route("/stats", get(stats))
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚰 Faucet server starting on {}", addr);
    info!("   Chain ID: {}", chain_id);
    info!("   RPC: {}", rpc_url);
    info!("   Amount: {} AXX", FAUCET_AMOUNT / 1_000_000_000_000_000_000);
    info!("   Cooldown: {} hours", COOLDOWN_HOURS);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
