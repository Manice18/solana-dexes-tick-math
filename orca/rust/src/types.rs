use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

use crate::utils::constants::{NUM_REWARDS, TICK_ARRAY_SIZE};

/// Represents a single tick in the tick array
/// Total size: 113 bytes
#[derive(Default, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct Tick {
    pub initialized: bool,     // 1 byte
    pub liquidity_net: i128,   // 16 bytes
    pub liquidity_gross: u128, // 16 bytes

    // Q64.64 fixed point numbers
    pub fee_growth_outside_a: u128, // 16 bytes
    pub fee_growth_outside_b: u128, // 16 bytes

    // Array of Q64.64 fixed point numbers
    pub reward_growths_outside: [u128; NUM_REWARDS], // 48 bytes = 16 * 3
}

/// Represents an array of ticks with metadata
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TickArray {
    pub start_tick_index: i32,
    pub ticks: [Tick; TICK_ARRAY_SIZE],
    pub whirlpool: Pubkey,
}

/// A more lightweight version of Tick for computations
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TickFacade {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}

/// A more lightweight version of TickArray for computations
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TickArrayFacade {
    pub start_tick_index: i32,
    pub ticks: [TickFacade; TICK_ARRAY_SIZE],
}

impl From<TickArray> for TickArrayFacade {
    fn from(tick_array: TickArray) -> Self {
        TickArrayFacade {
            start_tick_index: tick_array.start_tick_index,
            ticks: tick_array.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<Tick> for TickFacade {
    fn from(tick: Tick) -> Self {
        TickFacade {
            initialized: tick.initialized,
            liquidity_net: tick.liquidity_net,
            liquidity_gross: tick.liquidity_gross,
            fee_growth_outside_a: tick.fee_growth_outside_a,
            fee_growth_outside_b: tick.fee_growth_outside_b,
            reward_growths_outside: tick.reward_growths_outside,
        }
    }
}
