use super::{Felt, Word};

use air::{
    trace::{AUX_TRACE_RAND_ELEMENTS, TRACE_WIDTH},
    AuxRandElements, FieldElement, GkrRandElements,
};
use alloc::vec::Vec;
pub use rand_utils::*;
use winter_prover::LagrangeKernelRandElements;

/// Generates a valid [`AuxRandElements`] to build the auxiliary segment of the given trace  length.
pub fn aux_rand_elements_for_trace<E: FieldElement>(trace_len: usize) -> AuxRandElements<E> {
    let gkr_rand_elements = {
        let lagrange_kernel_rand_elements: LagrangeKernelRandElements<E> =
            LagrangeKernelRandElements::new(rand_vector(trace_len.ilog2() as usize));
        let openings_combining_randomness: Vec<E> = rand_vector(TRACE_WIDTH);

        GkrRandElements::new(lagrange_kernel_rand_elements, openings_combining_randomness)
    };

    AuxRandElements::new_with_gkr(rand_vector(AUX_TRACE_RAND_ELEMENTS), gkr_rand_elements)
}

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
/// Increments the argument.
fn generate_bytes_seed(seed: &mut u64) -> [u8; 32] {
    // increment the seed
    *seed = seed.wrapping_add(1);

    // generate a bytes seed
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&seed.to_le_bytes());
    bytes
}
