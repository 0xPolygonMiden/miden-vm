//! TODO: add docs

use super::{create_range, Felt, Word, HASHER_AUX_TRACE_OFFSET, ONE, ZERO};
use core::ops::Range;

pub use crate::crypto::hash::{Rpo256 as Hasher, RpoDigest as Digest};

// TYPES ALIASES
// ================================================================================================

/// Type for Hasher trace selector. These selectors are used to define which transition function
/// is to be applied at a specific row of the hasher execution trace.
pub type Selectors = [Felt; NUM_SELECTORS];

/// Type for the Hasher's state.
pub type HasherState = [Felt; STATE_WIDTH];

// CONSTANTS
// ================================================================================================

/// Number of field element needed to represent the sponge state for the hash function.
///
/// This value is set to 12: 8 elements are reserved for rate and the remaining 4 elements are
/// reserved for capacity. This configuration enables computation of 2-to-1 hash in a single
/// permutation.
pub const STATE_WIDTH: usize = Hasher::STATE_WIDTH;

/// Index of the column holding row addresses in the trace.
pub const ROW_COL_IDX: usize = NUM_SELECTORS;

/// The hasher state portion of the execution trace, located in 4 .. 16 columns.
pub const STATE_COL_RANGE: Range<usize> = create_range(ROW_COL_IDX + 1, STATE_WIDTH);

/// Number of field elements in the capacity portion of the hasher's state.
pub const CAPACITY_LEN: usize = STATE_WIDTH - RATE_LEN;

/// The index of the capacity register where the domain is set when initializing the hasher.
pub const CAPACITY_DOMAIN_IDX: usize = 1;

/// The capacity portion of the hasher state in the execution trace, located in 4 .. 8 columns.
pub const CAPACITY_COL_RANGE: Range<usize> = Range {
    start: STATE_COL_RANGE.start,
    end: STATE_COL_RANGE.start + CAPACITY_LEN,
};

/// Number of field elements in the rate portion of the hasher's state.
pub const RATE_LEN: usize = 8;

/// The rate portion of the hasher state in the execution trace, located in 8 .. 16 columns.
pub const RATE_COL_RANGE: Range<usize> = Range {
    start: CAPACITY_COL_RANGE.end,
    end: CAPACITY_COL_RANGE.end + RATE_LEN,
};

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

/// The number of rows in the execution trace required to compute a permutation of Rescue Prime
/// Optimized. This is equal to 8.
pub const HASH_CYCLE_LEN: usize = NUM_ROUNDS.next_power_of_two();

/// Number of columns in Hasher execution trace. Additional two columns are for row address and
/// node index columns.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + STATE_WIDTH + 2;

// --- Transition selectors -----------------------------------------------------------------------

/// Specifies a start of a new linear hash computation or absorption of new elements into an
/// executing linear hash computation. These selectors can also be used for a simple 2-to-1 hash
/// computation.
pub const LINEAR_HASH: Selectors = [ONE, ZERO, ZERO];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// linear_hash selector=[0, 1, 0, 0] rev(selector)=[0, 0, 1, 0] +1=[0, 0, 1, 1]
pub const LINEAR_HASH_LABEL: u8 = 0b0011;

/// Specifies a start of Merkle path verification computation or absorption of a new path node
/// into the hasher state.
pub const MP_VERIFY: Selectors = [ONE, ZERO, ONE];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// mp_verify selector=[0, 1, 0, 1] rev(selector)=[1, 0, 1, 0] +1=[1, 0, 1, 1]
pub const MP_VERIFY_LABEL: u8 = 0b1011;

/// Specifies a start of Merkle path verification or absorption of a new path node into the hasher
/// state for the "old" node value during Merkle root update computation.
pub const MR_UPDATE_OLD: Selectors = [ONE, ONE, ZERO];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// mr_update selector=[0, 1, 1, 0] rev(selector)=[0, 1, 1, 0] +1=[0, 1, 1, 1]
pub const MR_UPDATE_OLD_LABEL: u8 = 0b0111;

/// Specifies a start of Merkle path verification or absorption of a new path node into the hasher
/// state for the "new" node value during Merkle root update computation.
pub const MR_UPDATE_NEW: Selectors = [ONE, ONE, ONE];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// mr_update selector=[0, 1, 1, 1] rev(selector)=[1, 1, 1, 0] +1=[1, 1, 1, 1]
pub const MR_UPDATE_NEW_LABEL: u8 = 0b1111;

/// Specifies a completion of a computation such that only the hash result (values in h0, h1, h2
/// h3) is returned.
pub const RETURN_HASH: Selectors = [ZERO, ZERO, ZERO];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// return_hash selector=[0, 0, 0, 0] rev(selector)=[0, 0, 0, 0] +1=[0, 0, 0, 1]
pub const RETURN_HASH_LABEL: u8 = 0b0001;

/// Specifies a completion of a computation such that the entire hasher state (values in h0 through
/// h11) is returned.
pub const RETURN_STATE: Selectors = [ZERO, ZERO, ONE];
/// Unique label computed as 1 plus the full chiplet selector with the bits reversed.
/// return_state selector=[0, 0, 0, 1] rev(selector)=[1, 0, 0, 0] +1=[1, 0, 0, 1]
pub const RETURN_STATE_LABEL: u8 = 0b1001;

// --- Column accessors in the auxiliary trace ----------------------------------------------------

/// Index of the auxiliary trace column tracking the state of the sibling table.
pub const P1_COL_IDX: usize = HASHER_AUX_TRACE_OFFSET;

// PASS-THROUGH FUNCTIONS
// ================================================================================================

/// Returns a hash of two digests. This method is intended for use in construction of Merkle trees.
#[inline(always)]
pub fn merge(values: &[Digest; 2]) -> Digest {
    Hasher::merge(values)
}

/// Returns a hash of two digests with a specified domain.
#[inline(always)]
pub fn merge_in_domain(values: &[Digest; 2], domain: Felt) -> Digest {
    Hasher::merge_in_domain(values, domain)
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

// HASHER STATE MUTATORS
// ================================================================================================

/// Initializes hasher state with the first 8 elements to be absorbed. In accordance with the RPO
/// padding rule, the first capacity element is set with the provided padding flag, which is assumed
/// to be ZERO or ONE, depending on whether the number of elements to be absorbed is a multiple of
/// the rate or not. The remaining elements in the capacity portion of the state are set to ZERO.
#[inline(always)]
pub fn init_state(init_values: &[Felt; RATE_LEN], padding_flag: Felt) -> [Felt; STATE_WIDTH] {
    debug_assert!(
        padding_flag == ZERO || padding_flag == ONE,
        "first capacity element must be 0 or 1"
    );
    [
        padding_flag,
        ZERO,
        ZERO,
        ZERO,
        init_values[0],
        init_values[1],
        init_values[2],
        init_values[3],
        init_values[4],
        init_values[5],
        init_values[6],
        init_values[7],
    ]
}

/// Initializes hasher state with the elements from the provided words. Because the length of the
/// input is a multiple of the rate, all capacity elements are initialized to zero, as specified by
/// the Rescue Prime Optimized padding rule.
#[inline(always)]
pub fn init_state_from_words(w1: &Word, w2: &Word) -> [Felt; STATE_WIDTH] {
    init_state_from_words_with_domain(w1, w2, ZERO)
}

/// Initializes hasher state with elements from the provided words.  Sets the second element of the
/// capacity register to the provided domain.  All other elements of the capacity register are set to 0.
#[inline(always)]
pub fn init_state_from_words_with_domain(
    w1: &Word,
    w2: &Word,
    domain: Felt,
) -> [Felt; STATE_WIDTH] {
    [ZERO, domain, ZERO, ZERO, w1[0], w1[1], w1[2], w1[3], w2[0], w2[1], w2[2], w2[3]]
}

/// Absorbs the specified values into the provided state by overwriting the corresponding elements
/// in the rate portion of the state.
#[inline(always)]
pub fn absorb_into_state(state: &mut [Felt; STATE_WIDTH], values: &[Felt; RATE_LEN]) {
    state[4] = values[0];
    state[5] = values[1];
    state[6] = values[2];
    state[7] = values[3];
    state[8] = values[4];
    state[9] = values[5];
    state[10] = values[6];
    state[11] = values[7];
}

/// Returns elements representing the digest portion of the provided hasher's state.
pub fn get_digest(state: &[Felt; STATE_WIDTH]) -> [Felt; DIGEST_LEN] {
    state[DIGEST_RANGE].try_into().expect("failed to get digest from hasher state")
}
