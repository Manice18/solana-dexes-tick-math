use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use orca_rust_tick_maths::{get_tick_arrays_for_swap, parse_whirlpool};

#[test]
fn test_whirlpool_tick_array_fetching() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize RPC client
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    // Test whirlpool ID - SOL/USDC pool
    let whirlpool_id = Pubkey::from_str("8erNF5u3CHrqZJXtkfY8CjSxFYF1yqHmN8uDbAhk6tWM")?; // Replace with pool id for which you want fetch tick arrays for

    // Fetch whirlpool account data
    let account = rpc.get_account(&whirlpool_id)?;

    // Parse whirlpool data
    let whirlpool = parse_whirlpool(&account.data);

    println!("✅ Successfully parsed whirlpool:");
    println!("   Tick spacing: {}", whirlpool.tick_spacing);
    println!("   Current tick index: {}", whirlpool.tick_current_index);

    // Verify parsed data is reasonable
    assert!(
        whirlpool.tick_spacing > 0,
        "Tick spacing should be positive"
    );
    assert!(
        whirlpool.tick_current_index.abs() < 1_000_000,
        "Current tick index should be within reasonable bounds"
    );

    // Fetch tick arrays for swap
    let tick_arrays = get_tick_arrays_for_swap(
        &rpc,
        &whirlpool_id,
        whirlpool.tick_current_index,
        whirlpool.tick_spacing,
    )?;

    println!("✅ Successfully fetched tick arrays:");
    for (i, (pubkey, tick_array)) in tick_arrays.iter().enumerate() {
        println!(
            "   Tick array {}: {} (start_tick: {})",
            i, pubkey, tick_array.start_tick_index
        );
    }

    // Verify we got 5 tick arrays
    assert_eq!(tick_arrays.len(), 5, "Should fetch exactly 5 tick arrays");

    // Verify each tick array has a valid pubkey
    for (pubkey, _) in &tick_arrays {
        assert_ne!(
            *pubkey,
            Pubkey::default(),
            "Tick array pubkey should not be default"
        );
    }

    println!("✅ All tests passed!");
    Ok(())
}
