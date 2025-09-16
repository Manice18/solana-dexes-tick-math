use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;

use crate::utils::constants::{TICK_ARRAY_SEED, WHIRLPOOL_PROGRAM_ID};

/// Derives the Program Derived Address (PDA) for a tick array
///
/// # Arguments
/// * `whirlpool` - The whirlpool public key
/// * `start_tick_index` - The starting tick index for the tick array
///
/// # Returns
/// * `Result<(Pubkey, u8), ProgramError>` - The derived PDA and bump seed, or an error
pub fn derive_tick_array_pda(
    whirlpool: &Pubkey,
    start_tick_index: i32,
) -> Result<(Pubkey, u8), ProgramError> {
    let start_tick_index_str = start_tick_index.to_string();
    let seeds = &[
        TICK_ARRAY_SEED,
        whirlpool.as_ref(),
        start_tick_index_str.as_bytes(),
    ];

    Pubkey::try_find_program_address(seeds, &WHIRLPOOL_PROGRAM_ID).ok_or(ProgramError::InvalidSeeds)
}
