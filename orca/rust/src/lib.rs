//! Orca Whirlpool Tick Math Library
//!
//! This library provides utilities for working with Orca Whirlpool tick arrays,
//! including tick array fetching, PDA derivation, and whirlpool data parsing.

pub mod pda;
pub mod tick_array;
pub mod types;
pub mod utils;
pub mod whirlpool;

// Re-export commonly used types and functions
pub use pda::derive_tick_array_pda;
pub use tick_array::{get_tick_array_start_tick_index, get_tick_arrays_for_swap};
pub use types::{Tick, TickArray, TickArrayFacade, TickFacade};
pub use whirlpool::{WhirlpoolCore, parse_whirlpool};
