// Package ppc implements the Posted Price Controller for dynamic pricing
package ppc

import (
	"math"
	"sync"
	"time"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
)

// Controller manages dynamic pricing based on utilization and queue length
type Controller struct {
	config       *config.PPCConfig
	currentPrice float64
	utilization  float64
	queueLength  float64
	mu           sync.RWMutex
	ticker       *time.Ticker
	stopCh       chan struct{}
}

// NewController creates a new PPC controller
func NewController(cfg *config.PPCConfig) *Controller {
	return &Controller{
		config:       cfg,
		currentPrice: (cfg.MinPrice + cfg.MaxPrice) / 2, // Start at midpoint
		stopCh:       make(chan struct{}),
	}
}

// Start begins the price adjustment loop
func (c *Controller) Start() {
	c.ticker = time.NewTicker(c.config.AdjustmentInterval)

	go func() {
		for {
			select {
			case <-c.ticker.C:
				c.adjustPrice()
			case <-c.stopCh:
				return
			}
		}
	}()
}

// Stop stops the price adjustment loop
func (c *Controller) Stop() {
	if c.ticker != nil {
		c.ticker.Stop()
	}
	close(c.stopCh)
}

// GetCurrentPrice returns the current price per job class
func (c *Controller) GetCurrentPrice() float64 {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.currentPrice
}

// UpdateMetrics updates utilization and queue metrics
func (c *Controller) UpdateMetrics(utilization, queueTime float64) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.utilization = utilization
	c.queueLength = queueTime
}

// adjustPrice adjusts price based on current metrics
func (c *Controller) adjustPrice() {
	c.mu.Lock()
	defer c.mu.Unlock()

	// Calculate utilization error
	utilError := c.utilization - c.config.TargetUtilization

	// Calculate queue error (normalize queue time)
	queueError := (c.queueLength - c.config.TargetQueueTime) / c.config.TargetQueueTime

	// Price adjustment with exponential response
	// Δp = α * util_error + β * queue_error
	adjustment := c.config.Alpha*utilError + c.config.Beta*queueError

	// Apply adjustment with exponential scaling
	newPrice := c.currentPrice * (1.0 + adjustment)

	// Clamp to bounds
	newPrice = math.Max(c.config.MinPrice, math.Min(c.config.MaxPrice, newPrice))

	c.currentPrice = newPrice
}

// CalculateJobPrice calculates price for a specific job based on specs
func (c *Controller) CalculateJobPrice(jobClass string, complexity float64) float64 {
	basePrice := c.GetCurrentPrice()

	// Apply multipliers based on job class and complexity
	multiplier := 1.0

	switch jobClass {
	case "standard":
		multiplier = 1.0
	case "premium":
		multiplier = 1.5
	case "enterprise":
		multiplier = 2.0
	default:
		multiplier = 1.0
	}

	// Complexity factor (1.0 = baseline)
	complexityFactor := math.Max(0.5, math.Min(3.0, complexity))

	return basePrice * multiplier * complexityFactor
}

// GetPricingStats returns current pricing statistics
func (c *Controller) GetPricingStats() PricingStats {
	c.mu.RLock()
	defer c.mu.RUnlock()

	return PricingStats{
		CurrentPrice:      c.currentPrice,
		Utilization:       c.utilization,
		QueueTime:         c.queueLength,
		TargetUtilization: c.config.TargetUtilization,
		TargetQueueTime:   c.config.TargetQueueTime,
		MinPrice:          c.config.MinPrice,
		MaxPrice:          c.config.MaxPrice,
	}
}

// PricingStats contains current pricing statistics
type PricingStats struct {
	CurrentPrice      float64 `json:"current_price"`
	Utilization       float64 `json:"utilization"`
	QueueTime         float64 `json:"queue_time"`
	TargetUtilization float64 `json:"target_utilization"`
	TargetQueueTime   float64 `json:"target_queue_time"`
	MinPrice          float64 `json:"min_price"`
	MaxPrice          float64 `json:"max_price"`
}
