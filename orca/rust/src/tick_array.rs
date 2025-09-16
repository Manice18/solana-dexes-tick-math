use borsh::BorshDeserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use std::iter::zip;

use crate::pda::derive_tick_array_pda;
use crate::types::{TickArray, TickArrayFacade, TickFacade};
use crate::utils::constants::TICK_ARRAY_SIZE;

/// Retrieves tick arrays needed for a swap operation
///
/// # Arguments
/// * `rpc` - RPC client for blockchain communication
/// * `whirlpool` - The whirlpool public key
/// * `current_tick_index` - Current tick index of the whirlpool
/// * `tick_spacing` - Spacing between ticks
///
/// # Returns
/// * `Result<[(Pubkey, TickArrayFacade); 5], ProgramError>` - Array of 5 tick arrays with their addresses
pub fn get_tick_arrays_for_swap(
    rpc: &RpcClient,
    whirlpool: &Pubkey,
    current_tick_index: i32,
    tick_spacing: u16,
) -> Result<[(Pubkey, TickArrayFacade); 5], ProgramError> {
    let tick_array_start_index = get_tick_array_start_tick_index(current_tick_index, tick_spacing);
    let tick_spacing_i32 = tick_spacing as i32;
    let offset = tick_spacing_i32 * TICK_ARRAY_SIZE as i32;

    // Calculate the 5 tick array indexes we need
    let tick_array_indexes = [
        tick_array_start_index,
        tick_array_start_index + offset,
        tick_array_start_index + offset * 2,
        tick_array_start_index - offset,
        tick_array_start_index - offset * 2,
    ];

    // Derive PDA addresses for all tick arrays
    let tick_array_addresses: Vec<Pubkey> = tick_array_indexes
        .iter()
        .map(|&x| derive_tick_array_pda(whirlpool, x).map(|y| y.0))
        .collect::<Result<Vec<Pubkey>, _>>()?;

    // Fetch account data for all tick arrays
    let tick_array_infos = rpc.get_multiple_accounts(&tick_array_addresses).unwrap();

    // Deserialize tick arrays, handling cases where accounts might not exist
    let maybe_tick_arrays: Vec<Option<TickArrayFacade>> = tick_array_infos
        .iter()
        .map(|account_opt| {
            account_opt.as_ref().and_then(|account| {
                // Skip the 8-byte discriminator and deserialize the rest
                TickArray::try_from_slice(&account.data[8..])
                    .ok()
                    .map(|tick_array| tick_array.into())
            })
        })
        .collect();

    // Convert to concrete tick arrays, using uninitialized arrays for missing ones
    let tick_arrays: Vec<TickArrayFacade> = maybe_tick_arrays
        .iter()
        .enumerate()
        .map(|(i, x)| x.unwrap_or(uninitialized_tick_array(tick_array_indexes[i])))
        .collect::<Vec<TickArrayFacade>>();

    // Combine addresses with tick arrays and convert to fixed-size array
    let result: [(Pubkey, TickArrayFacade); 5] = zip(tick_array_addresses, tick_arrays)
        .collect::<Vec<(Pubkey, TickArrayFacade)>>()
        .try_into()
        .map_err(|_| "Failed to convert tick arrays to array".to_string())
        .unwrap();

    Ok(result)
}

/// Calculates the start tick index for a tick array containing the given tick index
///
/// # Arguments
/// * `tick_index` - The tick index to find the array for
/// * `tick_spacing` - Spacing between ticks
///
/// # Returns
/// * `i32` - The start tick index of the tick array
pub fn get_tick_array_start_tick_index(tick_index: i32, tick_spacing: u16) -> i32 {
    let tick_spacing_i32 = tick_spacing as i32;
    let tick_array_size_i32 = TICK_ARRAY_SIZE as i32;
    let real_index = tick_index
        .div_euclid(tick_spacing_i32)
        .div_euclid(tick_array_size_i32);
    real_index * tick_spacing_i32 * tick_array_size_i32
}

/// Creates an uninitialized tick array with the given start tick index
///
/// # Arguments
/// * `start_tick_index` - The starting tick index for the array
///
/// # Returns
/// * `TickArrayFacade` - An uninitialized tick array
fn uninitialized_tick_array(start_tick_index: i32) -> TickArrayFacade {
    TickArrayFacade {
        start_tick_index,
        ticks: [TickFacade::default(); TICK_ARRAY_SIZE],
    }
}
