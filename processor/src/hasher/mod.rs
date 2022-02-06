use super::{Felt, FieldElement, StarkField, TraceFragment, Word};
use vm_core::hasher::STATE_WIDTH;

mod selectors;

mod trace;
use trace::HasherTrace;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const NUM_SELECTORS: usize = 3;

/// Number of columns in Hasher execution trace. Additional two columns are for row address and
/// index columns.
const TRACE_WIDTH: usize = NUM_SELECTORS + STATE_WIDTH + 2;

// TYPE ALIASES
// ================================================================================================

type HasherState = [Felt; STATE_WIDTH];
type Selectors = [Felt; NUM_SELECTORS];

// HASH PROCESSOR
// ================================================================================================

/// TODO: add docs
pub struct Hasher {
    trace: HasherTrace,
}

impl Hasher {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add docs
    pub fn new() -> Self {
        Self {
            trace: HasherTrace::new(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn trace_len(&self) -> usize {
        self.trace.trace_len()
    }

    // HASHING METHODS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn permute(&mut self, mut state: HasherState) -> (Felt, HasherState) {
        let addr = self.trace.next_row_addr();

        self.trace.append_permutation(
            &mut state,
            selectors::LINEAR_HASH,
            selectors::RETURN_STATE,
            Felt::ZERO,
            Felt::ZERO,
        );

        (addr, state)
    }

    /// TODO: add docs
    pub fn build_merkle_root(&mut self, value: Word, path: &[Word], index: Felt) -> (Felt, Word) {
        let addr = self.trace.next_row_addr();
        let root = self.verify_merkle_path(value, path, index.as_int(), MerklePathType::MpVerify);
        (addr, root)
    }

    pub fn update_merkle_root(
        &mut self,
        old_value: Word,
        new_value: Word,
        path: &[Word],
        index: Felt,
    ) -> (Felt, Word, Word) {
        let addr = self.trace.next_row_addr();
        let index = index.as_int();

        let old_root = self.verify_merkle_path(old_value, path, index, MerklePathType::MrUpdateOld);
        let new_root = self.verify_merkle_path(new_value, path, index, MerklePathType::MrUpdateNew);

        (addr, old_root, new_root)
    }

    // TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provide trace fragment with trace data from this bitwise helper instance.
    #[cfg(test)]
    pub fn fill_trace(self, trace: &mut TraceFragment) {
        self.trace.fill_trace(trace)
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    fn verify_merkle_path(
        &mut self,
        value: Word,
        path: &[Word],
        mut index: u64,
        mp_type: MerklePathType,
    ) -> Word {
        let mut root = value;

        let main_selectors = mp_type.main_selectors();
        let part_selectors = mp_type.part_selectors();
        const RETURN_HASH: Selectors = selectors::RETURN_HASH;

        if path.len() == 1 {
            assert!(index <= 1, "");
            self.verify_mp_leg(root, path[0], &mut index, main_selectors, RETURN_HASH)
        } else {
            // first node
            let sibling = path[0];
            root = self.verify_mp_leg(root, sibling, &mut index, main_selectors, main_selectors);

            for &sibling in &path[1..path.len() - 1] {
                root =
                    self.verify_mp_leg(root, sibling, &mut index, part_selectors, main_selectors);
            }

            // last node
            let sibling = path[path.len() - 1];
            self.verify_mp_leg(root, sibling, &mut index, part_selectors, RETURN_HASH)
        }
    }

    fn verify_mp_leg(
        &mut self,
        root: Word,
        sibling: Word,
        index: &mut u64,
        init_selectors: Selectors,
        final_selectors: Selectors,
    ) -> Word {
        let index_bit = *index & 1;
        let mut state = build_merge_state(&root, &sibling, index_bit);

        let (init_index, rest_index) = if init_selectors[0] == Felt::ZERO {
            (Felt::new(*index >> 1), Felt::new(*index >> 1))
        } else {
            (Felt::new(*index), Felt::new(*index >> 1))
        };

        self.trace.append_permutation(
            &mut state,
            init_selectors,
            final_selectors,
            init_index,
            rest_index,
        );

        *index >>= 1;
        [state[4], state[5], state[6], state[7]]
    }
}

// MERKLE PATH TYPE
// ================================================================================================

enum MerklePathType {
    MpVerify,
    MrUpdateOld,
    MrUpdateNew,
}

impl MerklePathType {
    pub fn main_selectors(&self) -> Selectors {
        match self {
            Self::MpVerify => selectors::MP_VERIFY,
            Self::MrUpdateOld => selectors::MR_UPDATE_OLD,
            Self::MrUpdateNew => selectors::MR_UPDATE_NEW,
        }
    }

    pub fn part_selectors(&self) -> Selectors {
        match self {
            Self::MpVerify => selectors::CONTINUE_MP_VERIFY,
            Self::MrUpdateOld => selectors::CONTINUE_MR_UPDATE_OLD,
            Self::MrUpdateNew => selectors::CONTINUE_MR_UPDATE_NEW,
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn build_merge_state(a: &Word, b: &Word, index_bit: u64) -> HasherState {
    match index_bit {
        0 => [
            Felt::new(8),
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            a[0],
            a[1],
            a[2],
            a[3],
            b[0],
            b[1],
            b[2],
            b[3],
        ],
        1 => [
            Felt::new(8),
            Felt::ZERO,
            Felt::ZERO,
            Felt::ZERO,
            b[0],
            b[1],
            b[2],
            b[3],
            a[0],
            a[1],
            a[2],
            a[3],
        ],
        _ => panic!(""),
    }
}
