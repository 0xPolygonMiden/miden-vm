use super::{Felt, FieldElement, StarkField, TraceFragment, Word};
use core::ops::Range;
use vm_core::hasher::{
    Selectors, LINEAR_HASH, MP_VERIFY, MR_UPDATE_NEW, MR_UPDATE_OLD, RETURN_HASH, RETURN_STATE,
    STATE_WIDTH, TRACE_WIDTH,
};

mod trace;
use trace::HasherTrace;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

/// Elements of the hasher state which are returned as hash result.
const HASH_RESULT_RANGE: Range<usize> = Range { start: 4, end: 8 };

// TYPE ALIASES
// ================================================================================================

type HasherState = [Felt; STATE_WIDTH];

// HASH PROCESSOR
// ================================================================================================

/// Hash processor for the VM.
///
/// This component is responsible for performing all hash-related computations for the VM, as well
/// as building an execution trace for these computations. These computations include:
/// * Linear hashes, including simple 2-to-1 hashes, single and multiple permutations.
/// * Merkle path verification.
/// * Merkle root updates.
///
/// ## Execution trace
/// Hasher execution trace consists of 17 columns as illustrated below:
///
///   s0   s1   s2   addr   h0   h1   h2   h3   h4   h5   h6   h7   h8   h9   h10   h11   idx
/// ├────┴────┴────┴──────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴─────┴─────┴─────┤
///
/// In the above, the meaning of the columns is as follows:
/// * Selector columns s0, s1, and s2 used to help select transition function for a given row.
/// * Row address column addr used to uniquely identify each row in the table. Values in this
///   column start at 0 and are incremented by one with every subsequent row.
/// * Hasher state columns h0 through h11 used to hold the hasher state for each round of hash
///   computation. The state is laid out as follows:
///   - The first four columns are reserved for capacity elements of the state. When the state
///     is initialized for hash computations, h0 should be set to the number of elements to be
///     hashed. All other capacity elements should be set to 0s.
///   - The next eight columns are reserved for the rate elements of the state. These are used
///     to absorb the values to be hashed. Once a permutation is complete, hash output is located
///     in the first four rate columns (h4, h5, h6, h7).
/// * Node index column idx used to help with Merkle path verification and Merkle root update
///   computations. For all other computations the values in this column are set to 0s.
///
/// Each permutation of the hash function adds 8 rows to the execution trace. Thus, for Merkle
/// path verification, number of rows added to the trace is 8 * path.len(), and for Merkle root
/// update it is 16 * path.len(), since we need to perform two path verifications for each update.
pub struct Hasher {
    trace: HasherTrace,
}

impl Hasher {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a [Hasher] instantiated with an empty execution trace.
    pub fn new() -> Self {
        Self {
            trace: HasherTrace::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns current length of the execution trace stored in this hasher.
    #[allow(dead_code)]
    pub fn trace_len(&self) -> usize {
        self.trace.trace_len()
    }

    // HASHING METHODS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn hash(&mut self, mut state: HasherState) -> (Felt, HasherState) {
        let addr = self.trace.next_row_addr();
        self.trace
            .append_permutation(&mut state, LINEAR_HASH, RETURN_HASH, Felt::ZERO, Felt::ZERO);
        (addr, state)
    }

    /// Applies a single permutation of the hash function to the provided state and records the
    /// execution trace of this computation.
    ///
    /// The returned tuple contains hasher state after the permutation and the row address of the
    /// execution trace at which the permutation started.
    pub fn permute(&mut self, mut state: HasherState) -> (Felt, HasherState) {
        let addr = self.trace.next_row_addr();
        self.trace.append_permutation(
            &mut state,
            LINEAR_HASH,
            RETURN_STATE,
            Felt::ZERO,
            Felt::ZERO,
        );
        (addr, state)
    }

    /// Performs Merkle path verification computation and records its execution trace.
    ///
    /// The computation consists of computing a Merkle root of the specified path for a node with
    /// the specified value, located at the specified index.
    ///
    /// The returned tuple contains the root of the Merkle path and the row address of the
    /// execution trace at which the computation started.
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    pub fn build_merkle_root(&mut self, value: Word, path: &[Word], index: Felt) -> (Felt, Word) {
        let addr = self.trace.next_row_addr();
        let root =
            self.verify_merkle_path(value, path, index.as_int(), MerklePathContext::MpVerify);
        (addr, root)
    }

    /// Performs Merkle root update computation and records its execution trace.
    ///
    /// The computation consists of two Merkle path verification procedures for a node at the
    /// specified index. The procedures compute Merkle roots for the specified path for the old
    /// value of the node (value before the update), and the new value of the node (value after
    /// the update).
    ///
    /// The returned tuple contains these roots, as well as the row address of the execution trace
    /// at which the computation started.
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    pub fn update_merkle_root(
        &mut self,
        old_value: Word,
        new_value: Word,
        path: &[Word],
        index: Felt,
    ) -> (Felt, Word, Word) {
        let addr = self.trace.next_row_addr();
        let index = index.as_int();
        let old_root =
            self.verify_merkle_path(old_value, path, index, MerklePathContext::MrUpdateOld);
        let new_root =
            self.verify_merkle_path(new_value, path, index, MerklePathContext::MrUpdateNew);
        (addr, old_root, new_root)
    }

    // TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace fragment with trace data from this hasher trace instance.
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        self.trace.fill_trace(trace)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Computes a root of the provided Merkle path in the specified context. The path is assumed
    /// to be for a node with the specified value at the specified index.
    ///
    /// This also records the execution trace of the Merkle path computation.
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    fn verify_merkle_path(
        &mut self,
        value: Word,
        path: &[Word],
        mut index: u64,
        context: MerklePathContext,
    ) -> Word {
        assert!(!path.is_empty(), "path is empty");
        assert!(index >> path.len() == 0, "invalid index for the path");
        let mut root = value;

        // determine selectors for the specified context
        let main_selectors = context.main_selectors();
        let part_selectors = context.part_selectors();

        if path.len() == 1 {
            // handle path of length 1 separately because pattern for init and final selectors
            // is different from other cases
            self.verify_mp_leg(root, path[0], &mut index, main_selectors, RETURN_HASH)
        } else {
            // process the first node of the path; for this node, init and final selectors are
            // the same
            let sibling = path[0];
            root = self.verify_mp_leg(root, sibling, &mut index, main_selectors, main_selectors);

            // process all other nodes, except for the last one
            for &sibling in &path[1..path.len() - 1] {
                root =
                    self.verify_mp_leg(root, sibling, &mut index, part_selectors, main_selectors);
            }

            // process the last node
            let sibling = path[path.len() - 1];
            self.verify_mp_leg(root, sibling, &mut index, part_selectors, RETURN_HASH)
        }
    }

    /// Verifies a single leg of a Merkle path.
    ///
    /// This function does the following:
    /// - Builds the initial hasher state based on the least significant bit of the index.
    /// - Applies a permutation to this state and records the resulting trace.
    /// - Returns the result of the permutation and updates the index by removing its least
    ///   significant bit.
    fn verify_mp_leg(
        &mut self,
        root: Word,
        sibling: Word,
        index: &mut u64,
        init_selectors: Selectors,
        final_selectors: Selectors,
    ) -> Word {
        // build the hasher state based on the value of the least significant bit of the index
        let index_bit = *index & 1;
        let mut state = build_merge_state(&root, &sibling, index_bit);

        // determine values for the node index column for this permutation. if the first selector
        // of init_selectors is not ZERO (i.e., we are processing the first leg of the Merkle
        // path), the index for the first row is different from the index for the other rows;
        // otherwise, indexes are the same.
        let (init_index, rest_index) = if init_selectors[0] == Felt::ZERO {
            (Felt::new(*index >> 1), Felt::new(*index >> 1))
        } else {
            (Felt::new(*index), Felt::new(*index >> 1))
        };

        // apply the permutation to the state and record its trace
        self.trace.append_permutation(
            &mut state,
            init_selectors,
            final_selectors,
            init_index,
            rest_index,
        );

        // remove the least significant bit from the index and return hash result
        *index >>= 1;
        state[HASH_RESULT_RANGE]
            .try_into()
            .expect("failed to get result from hasher state")
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

// MERKLE PATH CONTEXT
// ================================================================================================

/// Specifies the context of a Merkle path computation.
enum MerklePathContext {
    /// The computation is for verifying a Merkle path (MPVERIFY).
    MpVerify,
    /// The computation is for verifying a Merkle path to an old node during Merkle root update
    /// procedure (MRUPDATE).
    MrUpdateOld,
    /// The computation is for verifying a Merkle path to a new node during Merkle root update
    /// procedure (MRUPDATE).
    MrUpdateNew,
}

impl MerklePathContext {
    /// Returns selector values for this context.
    pub fn main_selectors(&self) -> Selectors {
        match self {
            Self::MpVerify => MP_VERIFY,
            Self::MrUpdateOld => MR_UPDATE_OLD,
            Self::MrUpdateNew => MR_UPDATE_NEW,
        }
    }

    /// Returns partial selector values for this context. Partial selector values are derived
    /// from selector values by replacing the first selector with ZERO.
    pub fn part_selectors(&self) -> Selectors {
        let selectors = self.main_selectors();
        [Felt::ZERO, selectors[1], selectors[2]]
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Combines two words into a hasher state for Merkle path computation.
///
/// If index_bit = 0, the words are combined in the order (a, b), if index_bit = 1, the words are
/// combined in the order (b, a), otherwise, the function panics.
///
/// This also sets the capacity elements of the state to 8 (the number of elements to be hashed),
/// followed by 3 zeros.
fn build_merge_state(a: &Word, b: &Word, index_bit: u64) -> HasherState {
    const EIGHT: Felt = Felt::new(8);
    const ZERO: Felt = Felt::ZERO;
    match index_bit {
        0 => [
            EIGHT, ZERO, ZERO, ZERO, a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3],
        ],
        1 => [
            EIGHT, ZERO, ZERO, ZERO, b[0], b[1], b[2], b[3], a[0], a[1], a[2], a[3],
        ],
        _ => panic!("index bit is not a binary value"),
    }
}
