# Orca Whirlpool Tick Math Implementation 🌊

This directory contains implementations for working with Orca Whirlpool tick arrays and tick math. The implementations provide utilities for fetching, parsing, and working with Orca's concentrated liquidity pools.

## Available Languages

- **🦀 Rust** - Complete implementation with integration tests
- **TypeScript** - Coming soon
- **Go** - Coming soon

## Core Features

All implementations provide the following key functionality:

### 📦 Tick Array Management

- **Fetch tick arrays**: Retrieve the 5 tick arrays needed for swap operations
- **PDA derivation**: Calculate Program Derived Addresses for tick arrays
- **Uninitialized handling**: Gracefully handle missing/uninitialized tick arrays

### 🔍 Whirlpool Data Parsing

- **Core data extraction**: Parse tick spacing and current tick index from whirlpool accounts
- **Efficient parsing**: Direct byte-level parsing for optimal performance

### 🏗️ Type System

- **Full types**: Complete serializable structures for ticks and tick arrays
- **Optimized types**: Lightweight versions for computations
- **Conversions**: Seamless conversion between full and optimized types

---

## 🦀 Rust Implementation

### Usage Example

```rust
use orca_rust_tick_maths::{get_tick_arrays_for_swap, parse_whirlpool};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Initialize RPC client
let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

// Example: SOL/USDC whirlpool
let whirlpool_id = Pubkey::from_str("8erNF5u3CHrqZJXtkfY8CjSxFYF1yqHmN8uDbAhk6tWM")?;

// Fetch and parse whirlpool data
let account = rpc.get_account(&whirlpool_id)?;
let whirlpool = parse_whirlpool(&account.data);

println!("Tick spacing: {}", whirlpool.tick_spacing);
println!("Current tick: {}", whirlpool.tick_current_index);

// Fetch tick arrays for swap
let tick_arrays = get_tick_arrays_for_swap(
    &rpc,
    &whirlpool_id,
    whirlpool.tick_current_index,
    whirlpool.tick_spacing,
)?;

// Use tick arrays for swap calculations
for (i, (pubkey, tick_array)) in tick_arrays.iter().enumerate() {
    println!("Tick array {}: {} (start: {})",
        i, pubkey, tick_array.start_tick_index);
}
```

### API Reference

#### Core Functions

**`get_tick_arrays_for_swap()`**

- Fetches the 5 tick arrays needed for swap operations
- Returns both forward and backward tick arrays relative to current position

**`parse_whirlpool()`**

- Extracts essential data from whirlpool account bytes
- Returns `WhirlpoolCore` with tick spacing and current tick index

**`get_tick_array_start_tick_index()`**

- Calculates the starting tick index for a tick array containing a given tick
- Uses proper tick spacing and array size calculations

**`derive_tick_array_pda()`**

- Derives Program Derived Address for tick arrays
- Required for fetching tick array accounts from the blockchain

#### Data Structures

**`TickArrayFacade`**

- Lightweight tick array structure for computations
- Contains 88 ticks with start tick index
- Optimized for performance-critical operations

**`TickFacade`**

- Individual tick representation
- Contains liquidity data, fee growth, and reward information
- Used within tick arrays for price range calculations

### Running Tests

```bash
cd orca/rust
cargo test --package orca-rust-tick-maths --test integration_test -- test_whirlpool_tick_array_fetching --exact --show-output
```

The integration test fetches live data from Solana mainnet and validates:

- Whirlpool data parsing
- Tick array fetching
- PDA derivation
- Data structure integrity

### Implementation Details

The Rust library implements Orca's tick math by:

1. **Calculating tick array positions**: Uses tick spacing and array size (88 ticks) to determine which tick arrays contain relevant price ranges
2. **Fetching multiple arrays**: Retrieves 5 tick arrays (current + 2 forward + 2 backward) to handle price movements during swaps
3. **Handling edge cases**: Gracefully handles uninitialized tick arrays by creating empty placeholders
4. **Optimizing performance**: Uses facade types to minimize memory usage during calculations

This implementation provides the essential tick math components needed for Orca Whirlpool integrations.

### Getting Started

1. **Clone the repository**
2. **Navigate to the Rust directory**: `cd orca/rust`
3. **Build the project**: `cargo build`
4. **Run tests**: `cargo test`
5. **Use in your project**: Add to your `Cargo.toml` dependencies

---

## Use Cases

- **Tick Array Fetching**: Get the correct tick arrays needed for swap operations
- **Price Range Calculations**: Determine which tick arrays contain specific price ranges
- **Whirlpool Data Parsing**: Extract tick spacing and current tick information
- **PDA Derivation**: Calculate tick array addresses for on-chain data fetching
- **Liquidity Analysis**: Access tick-level liquidity data for analysis tools
