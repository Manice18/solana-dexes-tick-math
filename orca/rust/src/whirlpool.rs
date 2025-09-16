/// Core whirlpool data structure containing essential information
#[derive(Debug)]
pub struct WhirlpoolCore {
    pub tick_spacing: u16,
    pub tick_current_index: i32,
}

/// Parses whirlpool account data to extract core information
///
/// # Arguments
/// * `data` - Raw account data from the whirlpool account
///
/// # Returns
/// * `WhirlpoolCore` - Parsed whirlpool data containing tick spacing and current tick index
pub fn parse_whirlpool(data: &[u8]) -> WhirlpoolCore {
    // Extract tick spacing from bytes 41-43
    let tick_spacing = u16::from_le_bytes(data[41..43].try_into().unwrap());

    // Extract current tick index from bytes 81-85
    let tick_current_index = i32::from_le_bytes(data[81..85].try_into().unwrap());

    WhirlpoolCore {
        tick_spacing,
        tick_current_index,
    }
}
