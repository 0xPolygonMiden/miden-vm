//! TODO: add docs

use std::ops::Range;

use super::{Felt, FieldElement};
use crypto::{ElementHasher, Hasher as HashFn};

pub use crypto::hashers::Rp64_256 as Hasher;

// TYPES ALIASES
// ================================================================================================

/// Output type of Rescue Prime hash function.
///
/// The digest consists of 4 field elements or 32 bytes.
pub type Digest = <Hasher as HashFn>::Digest;

pub type Selectors = [Felt; NUM_SELECTORS];

// CONSTANTS
// ================================================================================================

/// Number of field element needed to represent the sponge state for the hash function.
///
/// This value is set to 12: 8 elements are reserved for rate and the remaining 4 elements are
/// reserved for capacity. This configuration enables computation of 2-to-1 hash in a single
/// permutation.
pub const STATE_WIDTH: usize = Hasher::STATE_WIDTH;

/// The length of the capacity portion of the hash state.
pub const CAPACITY_LEN: usize = 4;

// The length of the output portion of the hash state.
pub const DIGEST_LEN: usize = 4;

/// The output portion of the hash state, located in state elements 4, 5, 6, and 7.
pub const DIGEST_RANGE: Range<usize> = Hasher::DIGEST_RANGE;

/// Number of needed to complete a single permutation.
///
/// This value is set to 7 to target 128-bit security level with 40% security margin.
pub const NUM_ROUNDS: usize = Hasher::NUM_ROUNDS;

/// Number of selector columns in the trace.
pub const NUM_SELECTORS: usize = 3;

/// Number of columns in Hasher execution trace. Additional two columns are for row address and
/// node index columns.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + STATE_WIDTH + 2;

/// Specifies a start of a new linear hash computation or absorption of new elements into an
/// executing linear hash computation. These selectors can also be used for a simple 2-to-1 hash
/// computation.
pub const LINEAR_HASH: Selectors = [Felt::ONE, Felt::ZERO, Felt::ZERO];

/// Specifies a start of Merkle path verification computation or absorption of a new path node
/// into the hasher state.
pub const MP_VERIFY: Selectors = [Felt::ONE, Felt::ZERO, Felt::ONE];

/// Specifies a start of Merkle path verification or absorption of a new path node into the hasher
/// state for the "old" node value during Merkle root update computation.
pub const MR_UPDATE_OLD: Selectors = [Felt::ONE, Felt::ONE, Felt::ZERO];

/// Specifies a start of Merkle path verification or absorption of a new path node into the hasher
/// state for the "new" node value during Merkle root update computation.
pub const MR_UPDATE_NEW: Selectors = [Felt::ONE, Felt::ONE, Felt::ONE];

/// Specifies a completion of a computation such that only the hash result (values in h0, h1, h2
/// h3) is returned.
pub const RETURN_HASH: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ZERO];

/// Specifies a completion of a computation such that the entire hasher state (values in h0 through
/// h11) is returned.
pub const RETURN_STATE: Selectors = [Felt::ZERO, Felt::ZERO, Felt::ONE];

// PASS-THROUGH FUNCTIONS
// ================================================================================================

/// Returns a hash of two digests. This method is intended for use in construction of Merkle trees.
#[inline(always)]
pub fn merge(values: &[Digest; 2]) -> Digest {
    Hasher::merge(values)
}

/// Returns a hash of the provided list of field elements.
#[inline(always)]
pub fn hash_elements(elements: &[Felt]) -> Digest {
    Hasher::hash_elements(elements)
}

/// Applies Rescue-XLIX round function to the provided state.
///
/// The function takes sponge state as an input and applies a single Rescue-XLIX round to it. The
/// round number must be specified via `round` parameter, which must be between 0 and 6 (both
/// inclusive).
#[inline(always)]
pub fn apply_round(state: &mut [Felt; STATE_WIDTH], round: usize) {
    Hasher::apply_round(state, round)
}

/// Applies Rescue-XLIX permutation (7 rounds) to the provided state.
#[inline(always)]
pub fn apply_permutation(state: &mut [Felt; STATE_WIDTH]) {
    Hasher::apply_permutation(state)
}
