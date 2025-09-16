use solana_sdk::pubkey::Pubkey;

pub const TICK_ARRAY_SIZE: usize = 88;
pub const NUM_REWARDS: usize = 3;

/// The Whirlpool program ID on Solana
pub const WHIRLPOOL_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub const TICK_ARRAY_SEED: &[u8] = b"tick_array";
