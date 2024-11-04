//! TODO: add docs

use core::ops::Range;

pub use vm_core::crypto::hash::{Rpo256 as Hasher, RpoDigest as Digest};

use super::{create_range, Felt, ONE, ZERO};

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

/// The hasher state portion of the execution trace, located in columns 3..15.
pub const STATE_COL_RANGE: Range<usize> = create_range(NUM_SELECTORS, STATE_WIDTH);

/// Number of field elements in the capacity portion of the hasher's state.
pub const CAPACITY_LEN: usize = STATE_WIDTH - RATE_LEN;

/// The index of the capacity register where the domain is set when initializing the hasher.
pub const CAPACITY_DOMAIN_IDX: usize = 1;

/// The capacity portion of the hasher state in the execution trace, located in columns 3..7.
pub const CAPACITY_COL_RANGE: Range<usize> = Range {
    start: STATE_COL_RANGE.start,
    end: STATE_COL_RANGE.start + CAPACITY_LEN,
};

/// Number of field elements in the rate portion of the hasher's state.
pub const RATE_LEN: usize = 8;

/// The rate portion of the hasher state in the execution trace, located in columns 7..15.
pub const RATE_COL_RANGE: Range<usize> = Range {
    start: CAPACITY_COL_RANGE.end,
    end: CAPACITY_COL_RANGE.end + RATE_LEN,
};

// The length of the output portion of the hash state.
pub const DIGEST_LEN: usize = 4;

/// The output portion of the hash state, located in state elements 3, 4, 5, and 6.
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

/// Number of columns in Hasher execution trace. There is one additional column for the node index.
pub const TRACE_WIDTH: usize = NUM_SELECTORS + STATE_WIDTH + 1;

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
