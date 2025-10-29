// core/src/ppc/mod.rs
// Posted Price Controller (PPC) migrated from Go to Rust

use std::sync::{Arc, RwLock};
use tokio::time::{self, Duration, MissedTickBehavior};
use tokio::task::JoinHandle;

// --- Configuration ---

pub struct PPCConfig {
    pub min_price: f64,
    pub max_price: f64,
    pub target_utilization: f64,
    pub target_queue_time: f64, // in seconds
    pub adjustment_interval: Duration,
    pub alpha: f64, // weight for utilization error
    pub beta: f64,  // weight for queue error
}

impl Default for PPCConfig {
    fn default() -> Self {
        Self {
            min_price: 0.1,
            max_price: 10.0,
            target_utilization: 0.85,
            target_queue_time: 30.0,
            adjustment_interval: Duration::from_secs(60),
            alpha: 0.1,
            beta: 0.05,
        }
    }
}

// --- Controller State ---

#[derive(Debug, Clone)]
pub struct PricingStats {
    pub current_price: f64,
    pub utilization: f64,
    pub queue_time: f64,
}

struct ControllerState {
    stats: PricingStats,
    config: PPCConfig,
}

// --- Controller Logic ---

#[derive(Clone)]
pub struct Controller {
    state: Arc<RwLock<ControllerState>>,
}

impl Controller {
    pub fn new(config: PPCConfig) -> Self {
        let initial_price = (config.min_price + config.max_price) / 2.0;
        let state = ControllerState {
            stats: PricingStats {
                current_price: initial_price,
                utilization: 0.0,
                queue_time: 0.0,
            },
            config,
        };
        Self {
            state: Arc::new(RwLock::new(state)),
        }
    }

    /// Starts the background task for periodic price adjustments.
    pub fn start(self) -> JoinHandle<()> {
        let controller = self.clone();
        tokio::spawn(async move {
            let interval_duration = controller.state.read().unwrap().config.adjustment_interval;
            let mut interval = time::interval(interval_duration);
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                interval.tick().await;
                controller.adjust_price();
            }
        })
    }

    /// Updates the metrics used for price adjustments.
    pub fn update_metrics(&self, utilization: f64, queue_time: f64) {
        let mut state = self.state.write().unwrap();
        state.stats.utilization = utilization;
        state.stats.queue_time = queue_time;
    }

    /// Returns the current pricing stats.
    pub fn get_pricing_stats(&self) -> PricingStats {
        self.state.read().unwrap().stats.clone()
    }

    /// Calculates the final price for a job based on its class and complexity.
    pub fn calculate_job_price(&self, job_class: &str, complexity: f64) -> f64 {
        let stats = self.get_pricing_stats();
        let base_price = stats.current_price;

        let multiplier = match job_class {
            "premium" => 1.5,
            "enterprise" => 2.0,
            _ => 1.0, // "standard" or default
        };

        // Clamp complexity factor to prevent abuse
        let complexity_factor = complexity.max(0.5).min(3.0);

        base_price * multiplier * complexity_factor
    }

    /// Adjusts the price based on current metrics.
    fn adjust_price(&self) {
        let mut state = self.state.write().unwrap();

        // Calculate utilization error
        let util_error = state.stats.utilization - state.config.target_utilization;

        // Calculate normalized queue error
        let queue_error = (state.stats.queue_time - state.config.target_queue_time)
            / state.config.target_queue_time;

        // Δp = α * util_error + β * queue_error
        let adjustment = state.config.alpha * util_error + state.config.beta * queue_error;

        // Apply adjustment with exponential scaling
        let new_price = state.stats.current_price * (1.0 + adjustment);

        // Clamp to min/max bounds
        let new_price = new_price.max(state.config.min_price).min(state.config.max_price);

        state.stats.current_price = new_price;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_price_adjustment_increase() {
        let config = PPCConfig {
            adjustment_interval: Duration::from_millis(10),
            ..Default::default()
        };
        let controller = Controller::new(config);
        let initial_price = controller.get_pricing_stats().current_price;

        // Simulate high utilization and long queue times
        controller.update_metrics(0.95, 60.0); // 95% util, 60s queue

        controller.adjust_price();

        let new_price = controller.get_pricing_stats().current_price;
        assert!(new_price > initial_price, "Price should increase under high load");
    }

    #[tokio::test]
    async fn test_price_adjustment_decrease() {
        let config = PPCConfig {
            adjustment_interval: Duration::from_millis(10),
            ..Default::default()
        };
        let controller = Controller::new(config);
        let initial_price = controller.get_pricing_stats().current_price;

        // Simulate low utilization and short queue times
        controller.update_metrics(0.50, 10.0); // 50% util, 10s queue

        controller.adjust_price();

        let new_price = controller.get_pricing_stats().current_price;
        assert!(new_price < initial_price, "Price should decrease under low load");
    }

    #[test]
    fn test_calculate_job_price() {
        let controller = Controller::new(PPCConfig::default());
        let base_price = controller.get_pricing_stats().current_price;

        let standard_price = controller.calculate_job_price("standard", 1.0);
        assert_eq!(standard_price, base_price);

        let premium_price = controller.calculate_job_price("premium", 1.2);
        assert_eq!(premium_price, base_price * 1.5 * 1.2);
    }
}
