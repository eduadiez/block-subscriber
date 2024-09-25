# Ethereum Block Listener

This Rust program connects to an Ethereum node via WebSocket, subscribes to new blocks, and logs information about them, including the time difference between the block timestamp and the current time. It can optionally display the current gas price if the corresponding feature is enabled.

## Features

- **Basic Block Information**: Logs the current block number, timestamp, and the difference in seconds between the block timestamp and the current time.
- **Gas Price Logging**: If the `gasPrice` feature is enabled, the program will also log the current gas price.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.56 or later)
- An Ethereum node running with WebSocket support (e.g., Geth, OpenEthereum, etc.)

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/eduadiez/ethereum-block-listener.git
   cd ethereum-block-listener
   ```

## Configuration

The program uses command line arguments to configure the RPC URL:

- `--rpc-url`: The WebSocket URL of your Ethereum node. Defaults to `ws://localhost:8546`.

### Example Usage

To run the program with the default RPC URL:

```bash
cargo run
```

To specify a custom RPC URL:

```bash
cargo run -- --rpc-url ws://your-node-url:port
```

### Enabling the `gasPrice` Feature

By default, the program does not log the gas price. To enable this feature, use the `--features` flag:

```bash
cargo run --features gasPrice -- --rpc-url ws://your-node-url:port
```

### Enabling Trace Logging

If you want more detailed trace logging, compile the program with the `trace` feature:

```bash
cargo run --features trace -- --rpc-url ws://your-node-url:port
```

## How It Works

1. **Connects to the Ethereum Node**: The program connects to the specified WebSocket RPC URL using the `ethers` crate.
2. **Subscribes to New Blocks**: Once connected, it subscribes to new blocks using the `subscribe_blocks()` method.
3. **Logs Block Information**: For each new block, it logs the current time, block time, time difference, and optionally the gas price.