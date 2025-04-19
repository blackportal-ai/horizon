<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/blackportal-ai/resources/refs/heads/main/horizon/logo/horizon_logo.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/blackportal-ai/resources/refs/heads/main/horizon/logo/horizon_logo.svg">
    <img alt="Horizon - A Decentralized GPU Marketplace for Machine Learning Workloads."
         src="https://raw.githubusercontent.com/blackportal-ai/resources/refs/heads/main/horizon/logo/horizon_logo.svg"
         width="55%">
  </picture>

<br/>
<br/>

![build](https://img.shields.io/github/actions/workflow/status/blackportal-ai/horizon/core.yml?branch=master)
[![discord badge]](https://discord.gg/g5HtkAzRNG)
[![x handle]][x badge]

[//]: # (![crates.io]&#40;https://img.shields.io/crates/v/deltaml.svg&#41;)
[//]: # ([![documentation]&#40;https://img.shields.io/badge/docs-deltaml-blue?logo=rust&#41;]&#40;https://docs.rs/deltaml/latest/&#41;)

</div>

[x badge]: https://twitter.com/intent/follow?screen_name=BlackPortal_AI
[x handle]: https://img.shields.io/twitter/follow/BlackPortal_AI.svg?style=social&label=Follow
[discord badge]: https://img.shields.io/discord/1320514043424931861

# Horizon
A Decentralized GPU Marketplace for Machine Learning Workloads.

## Horizon CLI

The Horizon CLI provides a set of commands to interact with the Horizon decentralized GPU marketplace. These commands will facilitate tasks like interacting with the GPU market, submitting jobs, managing nodes, configuring the system, and more.

```text
horizon init # Initializes the Horizon environment locally

horizon list-gpus # Lists available GPUs in the Horizon marketplace
horizon rent <gpu_id> --duration <hours> [--price <price>] # Rent a GPU for a specified duration
horizon submit-job <gpu_id> --job <job_file> # Submit a job for execution on a specified GPU
horizon bid <gpu_id> --price <price> # Place a bid for a GPU rental
horizon auction start <gpu_id> --duration <duration> # Start an auction for a GPU
horizon auction end <gpu_id> # End an auction for a GPU
horizon status <job_id> # Retrieve the status of a job submitted to the marketplace
horizon cancel-job <job_id> # Cancel a job in progress

horizon config --set <key> <value> # Set a configuration option (e.g., price limits, security keys)
horizon config --get <key> # Retrieve a configuration value

horizon network status # Check the status of the Horizon network and nodes
horizon network connect <node_id> # Connect to a specific node in the network
horizon network disconnect <node_id> # Disconnect from a node in the network

horizon marketplace sync # Synchronize local data with the decentralized GPU marketplace
horizon marketplace update # Update local GPU listings from the marketplace

horizon register-gpu --gpu-id <gpu_id> --gpu-type <type> --price <price> # Register a GPU for rental
horizon deregister-gpu <gpu_id> # Deregister a GPU from the marketplace

horizon explore <gpu_id> [--filter <json_path>] # Interactively explore the details of a GPU

horizon logs # Display logs for Horizon operations and node status
horizon logs clear # Clear the local logs

horizon version # Displays the Horizon CLI version
horizon <command> --help # Shows help for a specific command
```

### Example Usage

```text
horizon list-gpus # List all available GPUs in the Horizon marketplace
horizon rent gpu_123 --duration 5 --price 0.25 # Rent GPU with ID 'gpu_123' for 5 hours at $0.25 per hour
horizon bid gpu_456 --price 0.3 # Place a bid for GPU with ID 'gpu_456' at $0.30 per hour
horizon submit-job gpu_123 --job training_model_001.job # Submit a job on GPU 'gpu_123'
horizon auction start gpu_789 --duration 12 # Start an auction for GPU 'gpu_789' for 12 hours
horizon status job_987 # Check the status of job 'job_987'
horizon cancel-job job_123 # Cancel job 'job_123'
horizon register-gpu --gpu-id gpu_001 --gpu-type RTX 3090 --price 0.5 # Register a new GPU for rental
horizon explore gpu_123 --filter .performance # Explore the performance data of GPU 'gpu_123'
horizon marketplace sync # Synchronize with the decentralized GPU marketplace
horizon config --set auction_timeout 60 # Set auction timeout to 60 minutes
horizon network status # Check the status of the network and nodes
horizon logs # View Horizon operation logs
horizon version # Display the CLI version
```

## Planned Structure

### horizon_cli

This will handle all interactions through the CLI. Users can interact with the marketplace, register GPUs, and submit jobs through simple command-line commands.

```text
horizon_cli/
├── src/
│   ├── main.rs          # Entry point for the CLI
│   ├── commands.rs      # Command definitions (register GPU, rent GPU, submit job)
│   ├── gpu_cmds.rs      # GPU-specific commands
│   └── job_cmds.rs      # Job-related commands (e.g., submit, status check)
└── Cargo.toml           # CLI-specific dependencies
```

### horizon_common

This will contain the shared logic used across different parts of the project, such as utility functions, error handling, and configuration management.

```text
horizon_common/
├── src/
│   ├── lib.rs           # Entry point for the common library
│   ├── protocol.rs      # Protocol definitions (communication formats, messages)
│   ├── network.rs       # Network communication (decentralized node management)
│   ├── auction.rs       # Auction and bidding system
│   ├── registry.rs      # GPU marketplace registry and resource management
│   ├── security.rs      # Cryptography and security-related logic
│   ├── config.rs        # Configuration parsing and management
│   ├── logging.rs       # Logging utilities
│   └── error.rs         # Error handling and custom error types
└── Cargo.toml           # Dependencies and package metadata
```

### horizon_registry

This component handles the marketplace’s core functionality, like GPU resource registration, listings, and availability management.

```text
horizon_registry/
├── src/
│   ├── main.rs           # Entry point for the registry service (starts gRPC server)
│   ├── lib.rs            # Core logic for the registry
│   ├── gpu_manager.rs    # Manages the registration and management of GPUs
│   ├── job_scheduler.rs  # Logic for managing job scheduling on available GPUs
│   ├── marketplace.rs    # Logic for managing the marketplace (bidding, renting)
│   ├── resource.rs       # Resource management (tracking GPU status, availability)
│   ├── registry.rs       # gRPC service logic for handling marketplace operations
└── Cargo.toml            # Registry-specific dependencies
```

There might be changes afterwards if something does not align with the project's requirements. But this is a rough idea of the project structure.

## License

The BSD 3-Clause License.
