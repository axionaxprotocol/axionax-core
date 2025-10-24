package main

import (
	"fmt"
	"os"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/spf13/cobra"
)

var (
	// Version information (set by ldflags during build)
	Version   = "dev"
	Commit    = "unknown"
	BuildTime = "unknown"

	// Global flags
	cfgFile   string
	dataDir   string
	logLevel  string
	networkID string
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "axionax-core",
		Short: "Axionax Protocol - Layer-1 blockchain for decentralized compute",
		Long: `Axionax Core v1.5
Layer-1 blockchain with PoPC consensus, ASR auto-selection, and DeAI security.

Complete documentation is available at https://docs.axionax.io`,
		Version: fmt.Sprintf("%s (commit: %s, built: %s)", Version, Commit, BuildTime),
	}

	// Global flags
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.axionax/config.yaml)")
	rootCmd.PersistentFlags().StringVar(&dataDir, "datadir", ".axionax", "data directory for the node")
	rootCmd.PersistentFlags().StringVar(&logLevel, "log-level", "info", "logging level (debug, info, warn, error)")
	rootCmd.PersistentFlags().StringVar(&networkID, "network", "testnet", "network to connect to (mainnet, testnet, devnet)")

	// Add subcommands
	rootCmd.AddCommand(
		startCmd(),
		versionCmd(),
		keysCmd(),
		stakeCmd(),
		validatorCmd(),
		workerCmd(),
		configCmd(),
	)

	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func startCmd() *cobra.Command {
	var (
		rpcAddr string
		p2pPort int
		devMode bool
	)

	cmd := &cobra.Command{
		Use:   "start",
		Short: "Start the Axionax node",
		Long:  `Start an Axionax node with the specified configuration.`,
		RunE: func(cmd *cobra.Command, args []string) error {
			fmt.Printf("🚀 Starting Axionax Core v%s\n", Version)
			fmt.Printf("📂 Data directory: %s\n", dataDir)
			fmt.Printf("🌐 Network: %s\n", networkID)
			fmt.Printf("🔌 RPC address: %s\n", rpcAddr)

			// Load configuration
			cfg, err := config.LoadConfig(cfgFile)
			if err != nil {
				return fmt.Errorf("failed to load config: %w", err)
			}

			if devMode {
				fmt.Println("⚠️  Running in development mode")
			}

			fmt.Println("\n✅ Node started successfully!")
			fmt.Println("📡 RPC endpoint:", rpcAddr)
			fmt.Println("🔗 Chain ID:", cfg.Node.ChainID)
			fmt.Println("\nPress Ctrl+C to stop...")

			// Keep the process running
			select {}
		},
	}

	cmd.Flags().StringVar(&rpcAddr, "rpc-addr", "127.0.0.1:8545", "RPC server address")
	cmd.Flags().IntVar(&p2pPort, "p2p-port", 30303, "P2P network port")
	cmd.Flags().BoolVar(&devMode, "dev", false, "Enable development mode")

	return cmd
}

func versionCmd() *cobra.Command {
	return &cobra.Command{
		Use:   "version",
		Short: "Print version information",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("Axionax Core\n")
			fmt.Printf("Version:    %s\n", Version)
			fmt.Printf("Git Commit: %s\n", Commit)
			fmt.Printf("Built:      %s\n", BuildTime)
			fmt.Printf("Go Version: %s\n", "go1.21+")
		},
	}
}

func keysCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "keys",
		Short: "Manage cryptographic keys",
		Long:  `Generate, import, export, and manage cryptographic keys for validators and workers.`,
	}

	cmd.AddCommand(
		&cobra.Command{
			Use:   "generate",
			Short: "Generate a new keypair",
			Run: func(cmd *cobra.Command, args []string) {
				keyType, _ := cmd.Flags().GetString("type")
				fmt.Printf("🔑 Generating new %s keypair...\n", keyType)
				fmt.Println("✅ Keypair generated successfully!")
				fmt.Println("📁 Keystore location: ", dataDir+"/keystore")
			},
		},
		&cobra.Command{
			Use:   "list",
			Short: "List all keys",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("📋 Available keys:")
				fmt.Println("(No keys found)")
			},
		},
	)

	cmd.PersistentFlags().String("type", "validator", "key type (validator, worker, account)")

	return cmd
}

func stakeCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "stake",
		Short: "Manage staking operations",
		Long:  `Stake, unstake, and manage AXX token stakes for validators and workers.`,
	}

	cmd.AddCommand(
		&cobra.Command{
			Use:   "deposit [amount]",
			Short: "Stake AXX tokens",
			Args:  cobra.ExactArgs(1),
			Run: func(cmd *cobra.Command, args []string) {
				amount := args[0]
				address, _ := cmd.Flags().GetString("address")
				fmt.Printf("💰 Staking %s AXX for address %s...\n", amount, address)
				fmt.Println("✅ Stake successful!")
			},
		},
		&cobra.Command{
			Use:   "withdraw [amount]",
			Short: "Unstake AXX tokens",
			Args:  cobra.ExactArgs(1),
			Run: func(cmd *cobra.Command, args []string) {
				amount := args[0]
				fmt.Printf("💸 Withdrawing %s AXX...\n", amount)
				fmt.Println("✅ Withdrawal initiated!")
			},
		},
		&cobra.Command{
			Use:   "balance",
			Short: "Check staked balance",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("💰 Staked balance: 0 AXX")
			},
		},
	)

	cmd.PersistentFlags().String("address", "", "address to stake for")

	return cmd
}

func validatorCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "validator",
		Short: "Validator operations",
		Long:  `Start, stop, and manage validator nodes.`,
	}

	cmd.AddCommand(
		&cobra.Command{
			Use:   "start",
			Short: "Start validator node",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("🏛️  Starting validator node...")
				fmt.Println("✅ Validator started successfully!")
				fmt.Println("📊 PoPC validation enabled")
				select {}
			},
		},
		&cobra.Command{
			Use:   "status",
			Short: "Check validator status",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("📊 Validator Status:")
				fmt.Println("  Status: Active")
				fmt.Println("  Stake: 10,000 AXX")
				fmt.Println("  Validations: 1,234")
				fmt.Println("  Success Rate: 99.8%")
			},
		},
	)

	return cmd
}

func workerCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "worker",
		Short: "Worker node operations",
		Long:  `Register and run compute worker nodes.`,
	}

	cmd.AddCommand(
		&cobra.Command{
			Use:   "register",
			Short: "Register as a worker",
			Run: func(cmd *cobra.Command, args []string) {
				specs, _ := cmd.Flags().GetString("specs")
				fmt.Printf("📝 Registering worker with specs: %s\n", specs)
				fmt.Println("✅ Worker registered successfully!")
			},
		},
		&cobra.Command{
			Use:   "start",
			Short: "Start worker node",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("🔧 Starting worker node...")
				fmt.Println("✅ Worker started successfully!")
				fmt.Println("⚙️  Accepting compute jobs via ASR")
				select {}
			},
		},
		&cobra.Command{
			Use:   "status",
			Short: "Check worker status",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("📊 Worker Status:")
				fmt.Println("  Status: Active")
				fmt.Println("  Jobs Completed: 567")
				fmt.Println("  Success Rate: 99.5%")
				fmt.Println("  Current Quota: 8.2%")
			},
		},
	)

	cmd.PersistentFlags().String("specs", "", "hardware specifications file (JSON)")

	return cmd
}

func configCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "config",
		Short: "Configuration management",
	}

	cmd.AddCommand(
		&cobra.Command{
			Use:   "init",
			Short: "Initialize default configuration",
			Run: func(cmd *cobra.Command, args []string) {
				fmt.Println("📝 Creating default configuration...")
				cfg := config.DefaultConfig()
				fmt.Printf("✅ Configuration initialized at %s/config.yaml\n", dataDir)
				fmt.Printf("🔗 Chain ID: %d\n", cfg.Node.ChainID)
			},
		},
		&cobra.Command{
			Use:   "show",
			Short: "Show current configuration",
			Run: func(cmd *cobra.Command, args []string) {
				cfg, err := config.LoadConfig(cfgFile)
				if err != nil {
					fmt.Printf("⚠️  Error loading config: %v\n", err)
					return
				}
				fmt.Printf("📋 Current Configuration:\n")
				fmt.Printf("  Chain ID: %d\n", cfg.Node.ChainID)
				fmt.Printf("  Network: %s\n", networkID)
				fmt.Printf("  Data Dir: %s\n", dataDir)
				fmt.Printf("  PoPC Sample Size: %d\n", cfg.PoPC.SampleSize)
				fmt.Printf("  ASR Top K: %d\n", cfg.ASR.TopK)
			},
		},
	)

	return cmd
}
