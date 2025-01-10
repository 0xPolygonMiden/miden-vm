pub use rand_utils::*;

use super::{Felt, Word};

// SEEDED GENERATORS
// ================================================================================================

/// Mutates a seed and generates a word deterministically
pub fn seeded_word(seed: &mut u64) -> Word {
    let seed = generate_bytes_seed(seed);
    prng_array(seed)
}

/// Mutates a seed and generates an element deterministically
pub fn seeded_element(seed: &mut u64) -> Felt {
    let seed = generate_bytes_seed(seed);
    let num = prng_array::<u64, 1>(seed)[0];
    Felt::new(num)
}

// HELPERS
// ================================================================================================

/// Generate a bytes seed that can be used as input for rand_utils.
///
/// Increments the argument and generates a 32-byte seed by spreading
/// the seed value across all bytes using a simple mixing function.
fn generate_bytes_seed(seed: &mut u64) -> [u8; 32] {
    // increment the seed
    *seed = seed.wrapping_add(1);
    let seed_value = *seed;

    let mut bytes = [0u8; 32];

    // Fill first 8 bytes with the original seed
    bytes[..8].copy_from_slice(&seed_value.to_le_bytes());

    // Fill remaining bytes using a simple mixing function
    for i in 1..4 {
        let next_value = seed_value.wrapping_mul(0x517cc1b727220a95).wrapping_add(i as u64);
        let start = i * 8;
        bytes[start..start + 8].copy_from_slice(&next_value.to_le_bytes());
    }

    bytes
}
