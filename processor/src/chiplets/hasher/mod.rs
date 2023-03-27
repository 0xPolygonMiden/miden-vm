use super::{
    trace::LookupTableRow, BTreeMap, ColMatrix, Felt, FieldElement, HasherState, MerkleRootUpdate,
    OpBatch, StarkField, TraceFragment, Vec, Word, ZERO,
};
use vm_core::chiplets::hasher::{
    absorb_into_state, get_digest, init_state, init_state_from_words,
    init_state_from_words_with_domain, Digest, Selectors, HASH_CYCLE_LEN, LINEAR_HASH,
    LINEAR_HASH_LABEL, MP_VERIFY, MP_VERIFY_LABEL, MR_UPDATE_NEW, MR_UPDATE_NEW_LABEL,
    MR_UPDATE_OLD, MR_UPDATE_OLD_LABEL, RETURN_HASH, RETURN_HASH_LABEL, RETURN_STATE,
    RETURN_STATE_LABEL, STATE_WIDTH, TRACE_WIDTH,
};

mod lookups;
pub use lookups::HasherLookup;
use lookups::HasherLookupContext;

mod trace;
use trace::HasherTrace;

mod aux_trace;
pub use aux_trace::{AuxTraceBuilder, SiblingTableRow, SiblingTableUpdate};

#[cfg(test)]
mod tests;

// HASH PROCESSOR
// ================================================================================================

/// Hash chiplet for the VM.
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
///   column start at 1 and are incremented by one with every subsequent row.
/// * Hasher state columns h0 through h11 used to hold the hasher state for each round of hash
///   computation. The state is laid out as follows:
///   - The first four columns represent the capacity state of the sponge function.
///   - The next eight columns represent the rate elements of the state. These are used to absorb
///     the values to be hashed. Once a permutation is complete, hash output is located in the first
///     four rate columns (h4, h5, h6, h7).
/// * Node index column idx used to help with Merkle path verification and Merkle root update
///   computations. For all other computations the values in this column are set to 0s.
///
/// Each permutation of the hash function adds 8 rows to the execution trace. Thus, for Merkle
/// path verification, number of rows added to the trace is 8 * path.len(), and for Merkle root
/// update it is 16 * path.len(), since we need to perform two path verifications for each update.
///
/// In addition to the execution trace, the hash chiplet also maintains:
/// - an auxiliary trace builder, which can be used to construct a running product column describing
///   the state of the sibling table (used in Merkle root update operations).
/// - a vector of [HasherLookup]s, each of which specifies the data for one of the lookup rows which
///   are required for verification of the communication between the stack/decoder and the Hash
///   Chiplet via the Chiplets Bus.
/// - a map of memoized execution trace, which keeps track of start and end rows of the sections of
///   the trace of a control or span block that can be copied to be used later for program blocks
///   encountered with the same digest instead of building it from scratch everytime. The only
///   thing that changes in the copied trace are the `addr` column rows. The hash of the block
///   is used as the key here after converting it to a bytes array.
#[derive(Default)]
pub struct Hasher {
    trace: HasherTrace,
    aux_trace: AuxTraceBuilder,
    memoized_trace_map: BTreeMap<[u8; 32], (usize, usize)>,
}

impl Hasher {
    // STATE ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns current length of the execution trace stored in this hasher.
    pub(super) fn trace_len(&self) -> usize {
        self.trace.trace_len()
    }

    /// Returns the [HasherLookup] from the provided label, index and context inputs.
    #[inline(always)]
    fn get_lookup(&self, label: u8, index: Felt, context: HasherLookupContext) -> HasherLookup {
        let addr = match context {
            // when starting a new hash operation, lookups are added before the operation begins.
            HasherLookupContext::Start => self.trace.next_row_addr().as_int() as u32,
            // in all other cases, they are added after the hash operation has completed.
            _ => self.trace_len() as u32,
        };
        HasherLookup::new(label, addr, index, context)
    }

    // HASHING METHODS
    // --------------------------------------------------------------------------------------------

    /// Applies a single permutation of the hash function to the provided state and records the
    /// execution trace of this computation as well as the lookups required for verifying the
    /// correctness of the permutation so that they can be provided to the Chiplets Bus.
    ///
    /// The returned tuple contains the hasher state after the permutation and the row address of
    /// the execution trace at which the permutation started.
    pub(super) fn permute(
        &mut self,
        mut state: HasherState,
        lookups: &mut Vec<HasherLookup>,
    ) -> (Felt, HasherState) {
        let addr = self.trace.next_row_addr();

        // add the lookup for the hash initialization.
        let lookup = self.get_lookup(LINEAR_HASH_LABEL, ZERO, HasherLookupContext::Start);
        lookups.push(lookup);

        // perform the hash.
        self.trace.append_permutation(&mut state, LINEAR_HASH, RETURN_STATE);

        // add the lookup for the hash result.
        let lookup = self.get_lookup(RETURN_STATE_LABEL, ZERO, HasherLookupContext::Return);
        lookups.push(lookup);

        (addr, state)
    }

    /// Computes the hash of the control block by computing hash(h1, h2) and returns the result.
    /// It also records the execution trace of this computation as well as the lookups required for
    /// verifying its correctness so that they can be provided to the Chiplets Bus.
    ///
    /// The returned tuple also contains the row address of the execution trace at which the hash
    /// computation started.
    pub(super) fn hash_control_block(
        &mut self,
        h1: Word,
        h2: Word,
        domain: Felt,
        expected_hash: Digest,
        lookups: &mut Vec<HasherLookup>,
    ) -> (Felt, Word) {
        let addr = self.trace.next_row_addr();
        let mut state = init_state_from_words_with_domain(&h1, &h2, domain);

        // add the lookup for the hash initialization.
        let lookup = self.get_lookup(LINEAR_HASH_LABEL, ZERO, HasherLookupContext::Start);
        lookups.push(lookup);

        if let Some((start_row, end_row)) = self.get_memoized_trace(expected_hash) {
            // copy the trace of a block with same hash instead of building it again.
            self.trace.copy_trace(&mut state, *start_row..*end_row);
        } else {
            // perform the hash.
            self.trace.append_permutation(&mut state, LINEAR_HASH, RETURN_HASH);

            self.insert_to_memoized_trace_map(addr, expected_hash);
        };

        // add the lookup for the hash result.
        let lookup = self.get_lookup(RETURN_HASH_LABEL, ZERO, HasherLookupContext::Return);
        lookups.push(lookup);

        let result = get_digest(&state);

        (addr, result)
    }

    /// Computes a sequential hash of all operation batches in the list and returns the result. It
    /// also records the execution trace of this computation, as well as the lookups required for
    /// verifying its correctness so that they can be provided to the Chiplets Bus.
    ///
    /// The returned tuple also contains the row address of the execution trace at which the hash
    /// computation started.
    pub(super) fn hash_span_block(
        &mut self,
        op_batches: &[OpBatch],
        expected_hash: Digest,
        lookups: &mut Vec<HasherLookup>,
    ) -> (Felt, Word) {
        const START: Selectors = LINEAR_HASH;
        const START_LABEL: u8 = LINEAR_HASH_LABEL;
        const RETURN: Selectors = RETURN_HASH;
        const RETURN_LABEL: u8 = RETURN_HASH_LABEL;
        // absorb selectors are the same as linear hash selectors, but absorb selectors are
        // applied on the last row of a permutation cycle, while linear hash selectors are
        // applied on the first row of a permutation cycle.
        const ABSORB: Selectors = LINEAR_HASH;
        const ABSORB_LABEL: u8 = LINEAR_HASH_LABEL;
        // to continue linear hash we need retain the 2nd and 3rd selector flags and set the
        // 1st flag to ZERO.
        const CONTINUE: Selectors = [ZERO, LINEAR_HASH[1], LINEAR_HASH[2]];

        let addr = self.trace.next_row_addr();

        // initialize the state and absorb the first operation batch into it
        let mut state = init_state(op_batches[0].groups(), ZERO);

        // add the lookup for the hash initialization.
        let lookup = self.get_lookup(START_LABEL, ZERO, HasherLookupContext::Start);
        lookups.push(lookup);

        // check if a span block with same hash has been encountered before in which case we can
        // directly copy it's trace.
        let (start_row, end_row, is_memoized) =
            if let Some((start_row, end_row)) = self.get_memoized_trace(expected_hash) {
                (*start_row, *end_row, true)
            } else {
                (0, 0, false)
            };

        let num_batches = op_batches.len();

        // if the span block is encountered for the first time and it's trace is not memoized,
        // we need to build the trace from scratch.
        if !is_memoized {
            if num_batches == 1 {
                // if there is only one batch to hash, we need only one permutation
                self.trace.append_permutation(&mut state, START, RETURN);
            } else {
                // if there is more than one batch, we need to process the first, the last, and the
                // middle permutations a bit differently. Specifically, selector flags for the
                // permutations need to be set as follows:
                // - first permutation: init linear hash on the first row, and absorb the next
                //   operation batch on the last row.
                // - middle permutations: continue hashing on the first row, and absorb the next
                //   operation batch on the last row.
                // - last permutation: continue hashing on the first row, and return the result
                //   on the last row.
                self.trace.append_permutation(&mut state, START, ABSORB);

                for batch in op_batches.iter().take(num_batches - 1).skip(1) {
                    absorb_into_state(&mut state, batch.groups());

                    // add the lookup for absorbing the next operation batch.
                    let lookup = self.get_lookup(ABSORB_LABEL, ZERO, HasherLookupContext::Absorb);
                    lookups.push(lookup);

                    self.trace.append_permutation(&mut state, CONTINUE, ABSORB);
                }

                absorb_into_state(&mut state, op_batches[num_batches - 1].groups());

                // add the lookup for absorbing the final operation batch.
                let lookup = self.get_lookup(ABSORB_LABEL, ZERO, HasherLookupContext::Absorb);
                lookups.push(lookup);

                self.trace.append_permutation(&mut state, CONTINUE, RETURN);
            }
            self.insert_to_memoized_trace_map(addr, expected_hash);
        } else if num_batches == 1 {
            self.trace.copy_trace(&mut state, start_row..end_row);
        } else {
            for i in 1..num_batches {
                // add the lookup for absorbing the next operation batch. Here we add the
                // lookups before actually copying the memoized trace.
                let lookup_addr = self.trace_len() + i * HASH_CYCLE_LEN;
                let lookup = HasherLookup::new(
                    ABSORB_LABEL,
                    lookup_addr as u32,
                    ZERO,
                    HasherLookupContext::Absorb,
                );
                lookups.push(lookup);
            }

            self.trace.copy_trace(&mut state, start_row..end_row);
        }

        // add the lookup for the hash result.
        let lookup = self.get_lookup(RETURN_LABEL, ZERO, HasherLookupContext::Return);
        lookups.push(lookup);

        let result = get_digest(&state);

        (addr, result)
    }

    /// Performs Merkle path verification computation and records its execution trace, as well as
    /// the lookups required for verifying its correctness so that they can be provided to the
    /// Chiplets Bus.
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
    pub(super) fn build_merkle_root(
        &mut self,
        value: Word,
        path: &[Word],
        index: Felt,
        lookups: &mut Vec<HasherLookup>,
    ) -> (Felt, Word) {
        let addr = self.trace.next_row_addr();

        let root = self.verify_merkle_path(
            value,
            path,
            index.as_int(),
            MerklePathContext::MpVerify,
            lookups,
        );

        (addr, root)
    }

    /// Performs Merkle root update computation and records its execution trace, as well as the
    /// lookups required for verifying its correctness so that they can be provided to the Chiplets
    /// Bus.
    ///
    /// The computation consists of two Merkle path verifications, one for the old value of the
    /// node (value before the update), and another for the new value (value after the update).
    ///
    /// # Panics
    /// Panics if:
    /// - The provided path does not contain any nodes.
    /// - The provided index is out of range for the specified path.
    pub(super) fn update_merkle_root(
        &mut self,
        old_value: Word,
        new_value: Word,
        path: &[Word],
        index: Felt,
        lookups: &mut Vec<HasherLookup>,
    ) -> MerkleRootUpdate {
        let address = self.trace.next_row_addr();
        let index = index.as_int();

        let old_root = self.verify_merkle_path(
            old_value,
            path,
            index,
            MerklePathContext::MrUpdateOld,
            lookups,
        );
        let new_root = self.verify_merkle_path(
            new_value,
            path,
            index,
            MerklePathContext::MrUpdateNew,
            lookups,
        );

        MerkleRootUpdate {
            address,
            old_root,
            new_root,
        }
    }

    // TRACE GENERATION
    // --------------------------------------------------------------------------------------------

    /// Fills the provided trace fragment with trace data from this hasher trace instance. This
    /// also returns the trace builder for hasher-related auxiliary trace columns.
    pub(super) fn fill_trace(self, trace: &mut TraceFragment) -> AuxTraceBuilder {
        self.trace.fill_trace(trace);

        self.aux_trace
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Computes a root of the provided Merkle path in the specified context. The path is assumed
    /// to be for a node with the specified value at the specified index.
    ///
    /// This also records the execution trace of the Merkle path computation and all lookups
    /// required for verifying its correctness.
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
        lookups: &mut Vec<HasherLookup>,
    ) -> Word {
        assert!(!path.is_empty(), "path is empty");
        assert!(index >> path.len() == 0, "invalid index for the path");
        let mut root = value;
        let mut depth = path.len() - 1;

        // determine selectors for the specified context
        let main_selectors = context.main_selectors();
        let part_selectors = context.part_selectors();

        if path.len() == 1 {
            // handle path of length 1 separately because pattern for init and final selectors
            // is different from other cases
            self.update_sibling_hints(context, index, path[0], depth);
            self.verify_mp_leg(root, path[0], &mut index, main_selectors, RETURN_HASH, lookups)
        } else {
            // process the first node of the path; for this node, init and final selectors are
            // the same
            let sibling = path[0];
            self.update_sibling_hints(context, index, sibling, depth);
            root = self.verify_mp_leg(
                root,
                sibling,
                &mut index,
                main_selectors,
                main_selectors,
                lookups,
            );
            depth -= 1;

            // process all other nodes, except for the last one
            for &sibling in &path[1..path.len() - 1] {
                self.update_sibling_hints(context, index, sibling, depth);
                root = self.verify_mp_leg(
                    root,
                    sibling,
                    &mut index,
                    part_selectors,
                    main_selectors,
                    lookups,
                );
                depth -= 1;
            }

            // process the last node
            let sibling = path[path.len() - 1];
            self.update_sibling_hints(context, index, sibling, depth);
            self.verify_mp_leg(root, sibling, &mut index, part_selectors, RETURN_HASH, lookups)
        }
    }

    /// Verifies a single leg of a Merkle path.
    ///
    /// This function does the following:
    /// - Builds the initial hasher state based on the least significant bit of the index.
    /// - Records the lookup required for verification of the hash initialization if the
    ///   `init_selectors` indicate that it is the beginning of the Merkle path verification.
    /// - Applies a permutation to this state and records the resulting trace.
    /// - Records the lookup required for verification of the hash result if the `final_selectors`
    ///   indicate that it is the end of the Merkle path verification.
    /// - Returns the result of the permutation and updates the index by removing its least
    ///   significant bit.
    fn verify_mp_leg(
        &mut self,
        root: Word,
        sibling: Word,
        index: &mut u64,
        init_selectors: Selectors,
        final_selectors: Selectors,
        lookups: &mut Vec<HasherLookup>,
    ) -> Word {
        // build the hasher state based on the value of the least significant bit of the index
        let index_bit = *index & 1;
        let mut state = build_merge_state(&root, &sibling, index_bit);

        // add the lookup for the hash initialization if this is the beginning.
        let context = HasherLookupContext::Start;
        if let Some(label) = get_selector_context_label(init_selectors, context) {
            let lookup = self.get_lookup(label, Felt::new(*index), context);
            lookups.push(lookup);
        }

        // determine values for the node index column for this permutation. if the first selector
        // of init_selectors is not ZERO (i.e., we are processing the first leg of the Merkle
        // path), the index for the first row is different from the index for the other rows;
        // otherwise, indexes are the same.
        let (init_index, rest_index) = if init_selectors[0] == ZERO {
            (Felt::new(*index >> 1), Felt::new(*index >> 1))
        } else {
            (Felt::new(*index), Felt::new(*index >> 1))
        };

        // apply the permutation to the state and record its trace
        self.trace.append_permutation_with_index(
            &mut state,
            init_selectors,
            final_selectors,
            init_index,
            rest_index,
        );

        // remove the least significant bit from the index and return hash result
        *index >>= 1;

        // add the lookup for the hash result if this is the end.
        let context = HasherLookupContext::Return;
        if let Some(label) = get_selector_context_label(final_selectors, context) {
            let lookup = self.get_lookup(label, Felt::new(*index), context);
            lookups.push(lookup);
        }

        get_digest(&state)
    }

    /// Records an update hint in the auxiliary trace builder to indicate whether the sibling was
    /// consumed as a part of computing the new or the old Merkle root. This is relevant only for
    /// the Merkle root update computation.
    fn update_sibling_hints(
        &mut self,
        context: MerklePathContext,
        index: u64,
        sibling: Word,
        depth: usize,
    ) {
        let step = self.trace.trace_len() as u32;
        match context {
            MerklePathContext::MrUpdateOld => {
                self.aux_trace.sibling_added(step, Felt::new(index), sibling);
            }
            MerklePathContext::MrUpdateNew => {
                // we use node depth as row offset here because siblings are added to the table
                // in reverse order of their depth (i.e., the sibling with the greatest depth is
                // added first). thus, when removing siblings from the table, we can find the right
                // entry by looking at the n-th entry from the end of the table, where n is the
                // node's depth (e.g., an entry for the sibling with depth 2, would be in the
                // second entry from the end of the table).
                self.aux_trace.sibling_removed(step, depth);
            }
            _ => (),
        }
    }

    /// Checks if a trace for a program block already exists and returns the start and end rows
    /// of the memoized trace. Returns None otherwise.
    fn get_memoized_trace(&self, hash: Digest) -> Option<&(usize, usize)> {
        let key: [u8; 32] = hash.into();
        self.memoized_trace_map.get(&key)
    }

    /// Inserts start and end rows of trace for a program block to the memoized_trace_map.
    fn insert_to_memoized_trace_map(&mut self, addr: Felt, hash: Digest) {
        let key: [u8; 32] = hash.into();
        let start_row = addr.as_int() as usize - 1;
        let end_row = self.trace.next_row_addr().as_int() as usize - 1;
        self.memoized_trace_map.insert(key, (start_row, end_row));
    }
}

// MERKLE PATH CONTEXT
// ================================================================================================

/// Specifies the context of a Merkle path computation.
#[derive(Debug, Clone, Copy)]
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
        [ZERO, selectors[1], selectors[2]]
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Combines two words into a hasher state for Merkle path computation.
///
/// If index_bit = 0, the words are combined in the order (a, b), if index_bit = 1, the words are
/// combined in the order (b, a), otherwise, the function panics.
#[inline(always)]
fn build_merge_state(a: &Word, b: &Word, index_bit: u64) -> HasherState {
    match index_bit {
        0 => init_state_from_words(a, b),
        1 => init_state_from_words(b, a),
        _ => panic!("index bit is not a binary value"),
    }
}

/// Gets the label for the hash operation from the provided selectors and the specified context.
pub fn get_selector_context_label(
    selectors: Selectors,
    context: HasherLookupContext,
) -> Option<u8> {
    match context {
        HasherLookupContext::Start => {
            if selectors == LINEAR_HASH {
                Some(LINEAR_HASH_LABEL)
            } else if selectors == MP_VERIFY {
                Some(MP_VERIFY_LABEL)
            } else if selectors == MR_UPDATE_OLD {
                Some(MR_UPDATE_OLD_LABEL)
            } else if selectors == MR_UPDATE_NEW {
                Some(MR_UPDATE_NEW_LABEL)
            } else {
                None
            }
        }
        HasherLookupContext::Return => {
            if selectors == RETURN_HASH {
                Some(RETURN_HASH_LABEL)
            } else if selectors == RETURN_STATE {
                Some(RETURN_STATE_LABEL)
            } else {
                None
            }
        }
        _ => {
            if selectors == LINEAR_HASH {
                Some(LINEAR_HASH_LABEL)
            } else {
                None
            }
        }
    }
}
