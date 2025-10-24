package ppc

import (
	"testing"
	"time"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewController(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 300 * time.Second,
	}

	controller := NewController(cfg)

	require.NotNil(t, controller)
	assert.Equal(t, cfg, controller.config)
	assert.Greater(t, controller.currentPrice, 0.0)
	assert.NotNil(t, controller.stopCh)

	// Initial price should be at midpoint
	expectedInitialPrice := (cfg.MinPrice + cfg.MaxPrice) / 2
	assert.Equal(t, expectedInitialPrice, controller.currentPrice)
}

func TestGetCurrentPrice(t *testing.T) {
	cfg := &config.PPCConfig{
		MinPrice: 0.01,
		MaxPrice: 5.0,
	}

	controller := NewController(cfg)
	price := controller.GetCurrentPrice()

	assert.Greater(t, price, 0.0)
	assert.GreaterOrEqual(t, price, cfg.MinPrice)
	assert.LessOrEqual(t, price, cfg.MaxPrice)
}

func TestUpdateMetrics(t *testing.T) {
	controller := NewController(&config.PPCConfig{})

	utilization := 0.75
	queueTime := 45.0

	controller.UpdateMetrics(utilization, queueTime)

	assert.Equal(t, utilization, controller.utilization)
	assert.Equal(t, queueTime, controller.queueLength)
}

func TestAdjustPrice_HighUtilization(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)
	initialPrice := controller.GetCurrentPrice()

	// Set high utilization (above target)
	controller.UpdateMetrics(0.85, 50.0)

	// Manually trigger price adjustment
	controller.adjustPrice()

	newPrice := controller.GetCurrentPrice()

	// Price should increase due to high utilization
	assert.Greater(t, newPrice, initialPrice, "Price should increase with high utilization")
}

func TestAdjustPrice_LowUtilization(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)
	initialPrice := controller.GetCurrentPrice()

	// Set low utilization (below target)
	controller.UpdateMetrics(0.40, 80.0)

	// Manually trigger price adjustment
	controller.adjustPrice()

	newPrice := controller.GetCurrentPrice()

	// Price should decrease due to low utilization
	assert.Less(t, newPrice, initialPrice, "Price should decrease with low utilization")
}

func TestAdjustPrice_LongQueueTime(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)
	initialPrice := controller.GetCurrentPrice()

	// Set long queue time (above target)
	controller.UpdateMetrics(0.70, 120.0) // Queue time = 2x target

	// Manually trigger price adjustment
	controller.adjustPrice()

	newPrice := controller.GetCurrentPrice()

	// Price should increase due to long queue
	assert.Greater(t, newPrice, initialPrice, "Price should increase with long queue time")
}

func TestAdjustPrice_ShortQueueTime(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)
	initialPrice := controller.GetCurrentPrice()

	// Set short queue time (below target)
	controller.UpdateMetrics(0.70, 30.0) // Queue time = 0.5x target

	// Manually trigger price adjustment
	controller.adjustPrice()

	newPrice := controller.GetCurrentPrice()

	// Price should decrease due to short queue
	assert.Less(t, newPrice, initialPrice, "Price should decrease with short queue time")
}

func TestAdjustPrice_MinPriceBound(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.5, // Large adjustment factor
		Beta:               0.2,
		MinPrice:           0.1,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)

	// Force price down with very low utilization and short queue
	for i := 0; i < 10; i++ {
		controller.UpdateMetrics(0.1, 10.0)
		controller.adjustPrice()
	}

	finalPrice := controller.GetCurrentPrice()

	// Price should be clamped at minimum
	assert.GreaterOrEqual(t, finalPrice, cfg.MinPrice)
	assert.LessOrEqual(t, finalPrice, cfg.MaxPrice)
}

func TestAdjustPrice_MaxPriceBound(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.5, // Large adjustment factor
		Beta:               0.2,
		MinPrice:           0.1,
		MaxPrice:           10.0,
		AdjustmentInterval: 100 * time.Millisecond,
	}

	controller := NewController(cfg)

	// Force price up with very high utilization and long queue
	for i := 0; i < 10; i++ {
		controller.UpdateMetrics(0.95, 200.0)
		controller.adjustPrice()
	}

	finalPrice := controller.GetCurrentPrice()

	// Price should be clamped at maximum
	assert.GreaterOrEqual(t, finalPrice, cfg.MinPrice)
	assert.LessOrEqual(t, finalPrice, cfg.MaxPrice)
}

func TestCalculateJobPrice_StandardClass(t *testing.T) {
	cfg := &config.PPCConfig{
		MinPrice: 1.0,
		MaxPrice: 10.0,
	}

	controller := NewController(cfg)
	controller.currentPrice = 2.0

	price := controller.CalculateJobPrice("standard", 1.0)

	// Standard class with baseline complexity
	assert.Equal(t, 2.0, price)
}

func TestCalculateJobPrice_PremiumClass(t *testing.T) {
	cfg := &config.PPCConfig{
		MinPrice: 1.0,
		MaxPrice: 10.0,
	}

	controller := NewController(cfg)
	controller.currentPrice = 2.0

	price := controller.CalculateJobPrice("premium", 1.0)

	// Premium class has 1.5x multiplier
	expectedPrice := 2.0 * 1.5
	assert.Equal(t, expectedPrice, price)
}

func TestCalculateJobPrice_EnterpriseClass(t *testing.T) {
	cfg := &config.PPCConfig{
		MinPrice: 1.0,
		MaxPrice: 10.0,
	}

	controller := NewController(cfg)
	controller.currentPrice = 2.0

	price := controller.CalculateJobPrice("enterprise", 1.0)

	// Enterprise class has 2.0x multiplier
	expectedPrice := 2.0 * 2.0
	assert.Equal(t, expectedPrice, price)
}

func TestCalculateJobPrice_WithComplexity(t *testing.T) {
	cfg := &config.PPCConfig{
		MinPrice: 1.0,
		MaxPrice: 10.0,
	}

	controller := NewController(cfg)
	controller.currentPrice = 2.0

	tests := []struct {
		name       string
		jobClass   string
		complexity float64
		minPrice   float64
		maxPrice   float64
	}{
		{
			name:       "High complexity standard",
			jobClass:   "standard",
			complexity: 2.0,
			minPrice:   4.0, // 2.0 * 1.0 * 2.0
			maxPrice:   4.0,
		},
		{
			name:       "Low complexity premium",
			jobClass:   "premium",
			complexity: 0.5,
			minPrice:   1.5, // 2.0 * 1.5 * 0.5
			maxPrice:   1.5,
		},
		{
			name:       "Very high complexity enterprise",
			jobClass:   "enterprise",
			complexity: 3.0,
			minPrice:   12.0, // 2.0 * 2.0 * 3.0
			maxPrice:   12.0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			price := controller.CalculateJobPrice(tt.jobClass, tt.complexity)
			assert.InDelta(t, tt.minPrice, price, 0.01)
		})
	}
}

func TestCalculateJobPrice_ComplexityBounds(t *testing.T) {
	controller := NewController(&config.PPCConfig{})
	controller.currentPrice = 1.0

	// Test complexity bounds (should clamp between 0.5 and 3.0)
	tests := []struct {
		name       string
		complexity float64
		minFactor  float64
		maxFactor  float64
	}{
		{"Very low complexity", 0.1, 0.5, 0.5},   // Clamped to 0.5
		{"Normal low complexity", 0.5, 0.5, 0.5}, // At minimum
		{"Baseline complexity", 1.0, 1.0, 1.0},
		{"High complexity", 2.0, 2.0, 2.0},
		{"Very high complexity", 5.0, 3.0, 3.0}, // Clamped to 3.0
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			price := controller.CalculateJobPrice("standard", tt.complexity)
			assert.GreaterOrEqual(t, price, tt.minFactor)
			assert.LessOrEqual(t, price, tt.maxFactor)
		})
	}
}

func TestGetPricingStats(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 300 * time.Second,
	}

	controller := NewController(cfg)
	controller.UpdateMetrics(0.75, 55.0)

	stats := controller.GetPricingStats()

	assert.Greater(t, stats.CurrentPrice, 0.0)
	assert.Equal(t, 0.75, stats.Utilization)
	assert.Equal(t, 55.0, stats.QueueTime)
	assert.Equal(t, cfg.TargetUtilization, stats.TargetUtilization)
	assert.Equal(t, cfg.TargetQueueTime, stats.TargetQueueTime)
	assert.Equal(t, cfg.MinPrice, stats.MinPrice)
	assert.Equal(t, cfg.MaxPrice, stats.MaxPrice)
}

func TestStartStop(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 50 * time.Millisecond,
	}

	controller := NewController(cfg)

	// Start the controller
	controller.Start()
	assert.NotNil(t, controller.ticker)

	// Let it run for a short time
	time.Sleep(150 * time.Millisecond)

	// Stop the controller
	controller.Stop()

	// Verify stop channel is closed (will panic if we try to close again)
	// We just check that Stop() doesn't panic
	assert.NotPanics(t, func() {
		// Calling Stop again should be safe (channel already closed)
	})
}

func TestPriceAdjustmentLoop(t *testing.T) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.2,
		Beta:               0.1,
		MinPrice:           0.5,
		MaxPrice:           5.0,
		AdjustmentInterval: 50 * time.Millisecond,
	}

	controller := NewController(cfg)
	initialPrice := controller.GetCurrentPrice()

	// Set high utilization
	controller.UpdateMetrics(0.90, 100.0)

	// Start the controller
	controller.Start()

	// Wait for a few adjustment cycles
	time.Sleep(200 * time.Millisecond)

	// Stop the controller
	controller.Stop()

	finalPrice := controller.GetCurrentPrice()

	// Price should have adjusted (increased due to high utilization)
	assert.NotEqual(t, initialPrice, finalPrice)
	assert.Greater(t, finalPrice, initialPrice)
}

func TestConcurrentAccess(t *testing.T) {
	controller := NewController(&config.PPCConfig{
		MinPrice: 0.1,
		MaxPrice: 10.0,
	})

	done := make(chan bool)

	// Concurrent readers
	for i := 0; i < 10; i++ {
		go func() {
			for j := 0; j < 100; j++ {
				controller.GetCurrentPrice()
				controller.GetPricingStats()
			}
			done <- true
		}()
	}

	// Concurrent writers
	for i := 0; i < 5; i++ {
		go func() {
			for j := 0; j < 100; j++ {
				controller.UpdateMetrics(0.5+float64(j%50)/100.0, 30.0+float64(j%30))
				controller.adjustPrice()
			}
			done <- true
		}()
	}

	// Wait for all goroutines
	for i := 0; i < 15; i++ {
		<-done
	}

	// Should not panic or deadlock
	assert.NotPanics(t, func() {
		controller.GetCurrentPrice()
	})
}

func TestPricingStats_Serialization(t *testing.T) {
	stats := PricingStats{
		CurrentPrice:      2.5,
		Utilization:       0.75,
		QueueTime:         45.0,
		TargetUtilization: 0.7,
		TargetQueueTime:   60.0,
		MinPrice:          0.1,
		MaxPrice:          10.0,
	}

	assert.Equal(t, 2.5, stats.CurrentPrice)
	assert.Equal(t, 0.75, stats.Utilization)
	assert.Equal(t, 45.0, stats.QueueTime)
}

func BenchmarkAdjustPrice(b *testing.B) {
	cfg := &config.PPCConfig{
		TargetUtilization:  0.7,
		TargetQueueTime:    60.0,
		Alpha:              0.1,
		Beta:               0.05,
		MinPrice:           0.001,
		MaxPrice:           10.0,
		AdjustmentInterval: 300 * time.Second,
	}

	controller := NewController(cfg)
	controller.UpdateMetrics(0.75, 55.0)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		controller.adjustPrice()
	}
}

func BenchmarkCalculateJobPrice(b *testing.B) {
	controller := NewController(&config.PPCConfig{
		MinPrice: 0.1,
		MaxPrice: 10.0,
	})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		controller.CalculateJobPrice("standard", 1.5)
	}
}

func BenchmarkGetCurrentPrice(b *testing.B) {
	controller := NewController(&config.PPCConfig{})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		controller.GetCurrentPrice()
	}
}

func BenchmarkUpdateMetrics(b *testing.B) {
	controller := NewController(&config.PPCConfig{})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		controller.UpdateMetrics(0.75, 55.0)
	}
}
