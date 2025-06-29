use alloc::{collections::VecDeque, sync::Arc, vec::Vec};

use vm_core::{
    Felt, Word,
    crypto::merkle::MerklePath,
    mast::{MastForest, MastNodeId},
};

use crate::host::advice::AdviceSource;

pub struct ExternalNodeReplay {
    pub external_node_resolutions: VecDeque<(MastNodeId, Arc<MastForest>)>,
}

impl Default for ExternalNodeReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalNodeReplay {
    /// Creates a new ExternalNodeReplay with an empty resolution queue
    pub fn new() -> Self {
        Self {
            external_node_resolutions: VecDeque::new(),
        }
    }

    /// Records a resolution of an external node to a MastNodeId with its associated MastForest
    pub fn record_resolution(&mut self, node_id: MastNodeId, forest: Arc<MastForest>) {
        self.external_node_resolutions.push_back((node_id, forest));
    }

    /// Replays the next recorded external node resolution, returning both the node ID and forest
    pub fn replay_resolution(&mut self) -> (MastNodeId, Arc<MastForest>) {
        self.external_node_resolutions
            .pop_front()
            .expect("No external node resolutions recorded")
    }
}

/// Implements a shim for the memory chiplet, in which all elements read from memory during a given
/// fragment are recorded by the fast processor, and replayed by the main trace fragment generators.
///
/// This is used to simulate memory reads in parallel trace generation without needing to actually
/// access the memory chiplet. Writes are not recorded here, as they are not needed for the trace
/// generation process.
///
/// Elements/words read are stored with their addresses and are assumed to be read from the same
/// addresses that they were recorded at. This works naturally since the fast processor has exactly
/// the same access patterns as the main trace generators (which re-executes part of the program).
/// The read methods include debug assertions to verify address consistency.
pub struct MemoryReplay {
    pub elements_read: VecDeque<(Felt, Felt)>,
    pub words_read: VecDeque<(Felt, Word)>,
}

impl Default for MemoryReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryReplay {
    /// Creates a new MemoryReplay with empty read vectors
    pub fn new() -> Self {
        Self {
            elements_read: VecDeque::new(),
            words_read: VecDeque::new(),
        }
    }

    // MUTATIONS (populated by the fast processor)
    // --------------------------------------------------------------------------------

    /// Records a read element from memory
    pub fn record_element(&mut self, element: Felt, addr: Felt) {
        self.elements_read.push_back((addr, element));
    }

    /// Records a read word from memory
    pub fn record_word(&mut self, word: Word, addr: Felt) {
        self.words_read.push_back((addr, word));
    }

    // ACCESSORS
    // --------------------------------------------------------------------------------

    pub fn replay_read_element(&mut self, addr: Felt) -> Felt {
        let (stored_addr, element) =
            self.elements_read.pop_front().expect("No elements read from memory");
        debug_assert_eq!(stored_addr, addr, "Address mismatch: expected {addr}, got {stored_addr}");
        element
    }

    pub fn replay_read_word(&mut self, addr: Felt) -> Word {
        let (stored_addr, word) = self.words_read.pop_front().expect("No words read from memory");
        debug_assert_eq!(stored_addr, addr, "Address mismatch: expected {addr}, got {stored_addr}");
        word
    }
}

/// Implements a shim for the advice provider, in which all advice provider operations during a
/// given fragment are pre-recorded by the fast processor.
///
/// This is used to simulate advice provider interactions in parallel trace generation without
/// needing to actually access the advice provider. All advice provider operations are recorded
/// during fast execution and then replayed during parallel trace generation.
///
/// The shim records all operations with their parameters and results, and provides replay methods
/// that return the pre-recorded results. This works naturally since the fast processor has exactly
/// the same access patterns as the main trace generators (which re-executes part of the program).
/// The read methods include debug assertions to verify parameter consistency.
pub struct AdviceReplay {
    // Stack operations
    pub stack_pops: VecDeque<Felt>,
    pub stack_word_pops: VecDeque<Word>,
    pub stack_dword_pops: VecDeque<[Word; 2]>,
    pub stack_pushes: VecDeque<AdviceSource>,

    // Map operations - store the values separately so we can return references
    pub map_gets: VecDeque<(Word, bool)>,    // (key, found)
    pub map_get_values: VecDeque<Vec<Felt>>, // values for successful gets
    pub map_inserts: VecDeque<(Word, Vec<Felt>)>,

    // Merkle store operations
    pub tree_nodes: VecDeque<(Word, Felt, Felt, Word)>,
    pub merkle_paths: VecDeque<(Word, Felt, Felt, MerklePath)>,
    pub leaf_depths: VecDeque<(Word, Felt, Felt, u8)>,
    pub node_updates: VecDeque<(Word, Felt, Felt, Word, (MerklePath, Word))>,
    pub root_merges: VecDeque<(Word, Word, Word)>,
}

impl Default for AdviceReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl AdviceReplay {
    /// Creates a new AdviceReplay with empty operation vectors
    pub fn new() -> Self {
        Self {
            stack_pops: VecDeque::new(),
            stack_word_pops: VecDeque::new(),
            stack_dword_pops: VecDeque::new(),
            stack_pushes: VecDeque::new(),
            map_gets: VecDeque::new(),
            map_get_values: VecDeque::new(),
            map_inserts: VecDeque::new(),
            tree_nodes: VecDeque::new(),
            merkle_paths: VecDeque::new(),
            leaf_depths: VecDeque::new(),
            node_updates: VecDeque::new(),
            root_merges: VecDeque::new(),
        }
    }

    // MUTATIONS (populated by the fast processor)
    // --------------------------------------------------------------------------------

    /// Records the value returned by a pop_stack operation
    pub fn record_stack_pop(&mut self, value: Felt) {
        self.stack_pops.push_back(value);
    }

    /// Records the word returned by a pop_stack_word operation
    pub fn record_stack_word_pop(&mut self, word: Word) {
        self.stack_word_pops.push_back(word);
    }

    /// Records the double word returned by a pop_stack_dword operation
    pub fn record_stack_dword_pop(&mut self, dword: [Word; 2]) {
        self.stack_dword_pops.push_back(dword);
    }

    /// Records a successful push_stack operation with the given advice source
    pub fn record_stack_push(&mut self, source: AdviceSource) {
        self.stack_pushes.push_back(source);
    }

    /// Records a get_mapped_values operation and its result
    pub fn record_map_get(&mut self, key: Word, values: Option<&[Felt]>) {
        match values {
            Some(values) => {
                self.map_gets.push_back((key, true));
                self.map_get_values.push_back(values.to_vec());
            },
            None => {
                self.map_gets.push_back((key, false));
            },
        }
    }

    /// Records an insert_into_map operation with the given key and values
    pub fn record_map_insert(&mut self, key: Word, values: Vec<Felt>) {
        self.map_inserts.push_back((key, values));
    }

    /// Records a successful get_tree_node operation and the returned node value
    pub fn record_tree_node(&mut self, root: Word, depth: Felt, index: Felt, node: Word) {
        self.tree_nodes.push_back((root, depth, index, node));
    }

    /// Records a successful get_merkle_path operation and the returned path
    pub fn record_merkle_path(&mut self, root: Word, depth: Felt, index: Felt, path: MerklePath) {
        self.merkle_paths.push_back((root, depth, index, path));
    }

    /// Records a successful get_leaf_depth operation and the returned depth
    pub fn record_leaf_depth(&mut self, root: Word, tree_depth: Felt, index: Felt, leaf_depth: u8) {
        self.leaf_depths.push_back((root, tree_depth, index, leaf_depth));
    }

    /// Records a successful update_merkle_node operation and the returned path and new root
    pub fn record_node_update(
        &mut self,
        root: Word,
        depth: Felt,
        index: Felt,
        value: Word,
        path_and_new_root: (MerklePath, Word),
    ) {
        self.node_updates.push_back((root, depth, index, value, path_and_new_root));
    }

    /// Records a successful merge_roots operation and the returned merged root
    pub fn record_root_merge(&mut self, lhs: Word, rhs: Word, merged_root: Word) {
        self.root_merges.push_back((lhs, rhs, merged_root));
    }

    // ACCESSORS (used during parallel trace generation)
    // --------------------------------------------------------------------------------

    /// Replays a pop_stack operation, returning the previously recorded value
    pub fn replay_stack_pop(&mut self) -> Felt {
        self.stack_pops.pop_front().expect("No stack pop operations recorded")
    }

    /// Replays a pop_stack_word operation, returning the previously recorded word
    pub fn replay_stack_word_pop(&mut self) -> Word {
        self.stack_word_pops.pop_front().expect("No stack word pop operations recorded")
    }

    /// Replays a pop_stack_dword operation, returning the previously recorded double word
    pub fn replay_stack_dword_pop(&mut self) -> [Word; 2] {
        self.stack_dword_pops
            .pop_front()
            .expect("No stack dword pop operations recorded")
    }

    /// Replays a push_stack operation, verifying the advice source matches the recorded one
    pub fn replay_stack_push(&mut self, source: AdviceSource) {
        let recorded_source =
            self.stack_pushes.pop_front().expect("No stack push operations recorded");
        debug_assert_eq!(
            recorded_source, source,
            "AdviceSource mismatch: expected {source:?}, got {recorded_source:?}"
        );
    }

    /// Replays a get_mapped_values operation, returning the previously recorded values if found.
    ///
    /// Returns None if the key wasn't found during the original operation, or Some containing
    /// the recorded values if it was found.
    ///
    /// Note: This method consumes the recorded value and cannot be called multiple times for
    /// the same operation, unlike the real AdviceProvider which allows multiple accesses to
    /// the same key.
    pub fn replay_map_get(&mut self, key: &Word) -> Option<Vec<Felt>> {
        let (recorded_key, was_found) =
            self.map_gets.pop_front().expect("No map get operations recorded");
        debug_assert_eq!(
            recorded_key, *key,
            "Map key mismatch: expected {key:?}, got {recorded_key:?}"
        );

        if was_found {
            Some(self.map_get_values.pop_front().expect("Map get values should be available"))
        } else {
            None
        }
    }

    /// Replays an insert_into_map operation
    pub fn replay_map_insert(&mut self, key: Word, values: Vec<Felt>) {
        let (recorded_key, recorded_values) =
            self.map_inserts.pop_front().expect("No map insert operations recorded");
        debug_assert_eq!(
            recorded_key, key,
            "Map insert key mismatch: expected {key:?}, got {recorded_key:?}"
        );
        debug_assert_eq!(
            recorded_values, values,
            "Map insert values mismatch: expected {values:?}, got {recorded_values:?}"
        );
    }

    /// Replays a get_tree_node operation, returning the previously recorded node value
    pub fn replay_tree_node(&mut self, root: Word, depth: &Felt, index: &Felt) -> Word {
        let (recorded_root, recorded_depth, recorded_index, node_value) =
            self.tree_nodes.pop_front().expect("No tree node operations recorded");
        debug_assert_eq!(
            recorded_root, root,
            "Tree node root mismatch: expected {root:?}, got {recorded_root:?}"
        );
        debug_assert_eq!(
            recorded_depth, *depth,
            "Tree node depth mismatch: expected {depth:?}, got {recorded_depth:?}"
        );
        debug_assert_eq!(
            recorded_index, *index,
            "Tree node index mismatch: expected {index:?}, got {recorded_index:?}"
        );
        node_value
    }

    /// Replays a get_merkle_path operation, returning the previously recorded path
    pub fn replay_merkle_path(&mut self, root: Word, depth: Felt, index: Felt) -> MerklePath {
        let (recorded_root, recorded_depth, recorded_index, merkle_path) =
            self.merkle_paths.pop_front().expect("No merkle path operations recorded");
        debug_assert_eq!(
            recorded_root, root,
            "Merkle path root mismatch: expected {root:?}, got {recorded_root:?}"
        );
        debug_assert_eq!(
            recorded_depth, depth,
            "Merkle path depth mismatch: expected {depth:?}, got {recorded_depth:?}"
        );
        debug_assert_eq!(
            recorded_index, index,
            "Merkle path index mismatch: expected {index:?}, got {recorded_index:?}"
        );
        merkle_path
    }

    /// Replays a get_leaf_depth operation, returning the previously recorded leaf depth
    pub fn replay_leaf_depth(&mut self, root: Word, tree_depth: &Felt, index: &Felt) -> u8 {
        let (recorded_root, recorded_tree_depth, recorded_index, leaf_depth) =
            self.leaf_depths.pop_front().expect("No leaf depth operations recorded");
        debug_assert_eq!(
            recorded_root, root,
            "Leaf depth root mismatch: expected {root:?}, got {recorded_root:?}"
        );
        debug_assert_eq!(
            recorded_tree_depth, *tree_depth,
            "Leaf depth tree_depth mismatch: expected {tree_depth:?}, got {recorded_tree_depth:?}"
        );
        debug_assert_eq!(
            recorded_index, *index,
            "Leaf depth index mismatch: expected {index:?}, got {recorded_index:?}"
        );
        leaf_depth
    }

    /// Replays an update_merkle_node operation
    pub fn replay_node_update(
        &mut self,
        root: Word,
        depth: &Felt,
        index: &Felt,
        value: Word,
    ) -> (MerklePath, Word) {
        let (recorded_root, recorded_depth, recorded_index, recorded_value, result) =
            self.node_updates.pop_front().expect("No node update operations recorded");
        debug_assert_eq!(
            recorded_root, root,
            "Node update root mismatch: expected {root:?}, got {recorded_root:?}"
        );
        debug_assert_eq!(
            recorded_depth, *depth,
            "Node update depth mismatch: expected {depth:?}, got {recorded_depth:?}"
        );
        debug_assert_eq!(
            recorded_index, *index,
            "Node update index mismatch: expected {index:?}, got {recorded_index:?}"
        );
        debug_assert_eq!(
            recorded_value, value,
            "Node update value mismatch: expected {value:?}, got {recorded_value:?}"
        );
        result
    }

    /// Replays a merge_roots operation
    pub fn replay_root_merge(&mut self, lhs: Word, rhs: Word) -> Word {
        let (recorded_lhs, recorded_rhs, result) =
            self.root_merges.pop_front().expect("No root merge operations recorded");
        debug_assert_eq!(
            recorded_lhs, lhs,
            "Root merge lhs mismatch: expected {lhs:?}, got {recorded_lhs:?}"
        );
        debug_assert_eq!(
            recorded_rhs, rhs,
            "Root merge rhs mismatch: expected {rhs:?}, got {recorded_rhs:?}"
        );
        result
    }
}

/// Implements a shim for the hasher chiplet, in which all hasher operations during a given
/// fragment are pre-recorded by the fast processor.
///
/// This is used to simulate hasher operations in parallel trace generation without needing
/// to actually perform hash computations. All hasher operations are recorded during fast
/// execution and then replayed during parallel trace generation.
#[derive(Debug)]
pub struct HasherReplay {
    /// Recorded hasher addresses from operations like hash_control_block, hash_basic_block, etc.
    pub block_addresses: VecDeque<Felt>,

    /// Recorded hasher operations from permutation operations (HPerm)
    /// Each entry contains (address, output_state)
    pub permutation_operations: VecDeque<(Felt, [Felt; 12])>,

    /// Recorded hasher operations from Merkle path verification operations
    /// Each entry contains (address, computed_root)
    pub build_merkle_root_operations: VecDeque<(Felt, Word)>,

    /// Recorded hasher operations from Merkle root update operations
    /// Each entry contains (address, old_root, new_root)
    pub mrupdate_operations: VecDeque<(Felt, Word, Word)>,
}

impl Default for HasherReplay {
    fn default() -> Self {
        Self::new()
    }
}

impl HasherReplay {
    pub fn new() -> Self {
        Self {
            block_addresses: VecDeque::new(),
            permutation_operations: VecDeque::new(),
            build_merkle_root_operations: VecDeque::new(),
            mrupdate_operations: VecDeque::new(),
        }
    }

    // MUTATIONS (populated by the fast processor)
    // --------------------------------------------------------------------------------

    /// Records a hasher address from a block hash operation
    pub fn record_block_address(&mut self, addr: Felt) {
        self.block_addresses.push_back(addr);
    }

    /// Records a permutation operation with its address and result
    pub fn record_permutation(&mut self, addr: Felt, output_state: [Felt; 12]) {
        self.permutation_operations.push_back((addr, output_state));
    }

    /// Records a Merkle path verification with its address and computed root
    pub fn record_build_merkle_root(&mut self, addr: Felt, computed_root: Word) {
        self.build_merkle_root_operations.push_back((addr, computed_root));
    }

    /// Records a Merkle root update with its address, old root, and new root
    pub fn record_mrupdate(&mut self, addr: Felt, old_root: Word, new_root: Word) {
        self.mrupdate_operations.push_back((addr, old_root, new_root));
    }

    // ACCESSORS (used by parallel trace generators)
    // --------------------------------------------------------------------------------

    /// Replays a block hash operation, returning the pre-recorded address
    pub fn replay_block_address(&mut self) -> Felt {
        self.block_addresses.pop_front().expect("No block address operations recorded")
    }

    /// Replays a permutation operation, returning the pre-recorded address and result
    pub fn replay_permutation(&mut self) -> (Felt, [Felt; 12]) {
        self.permutation_operations
            .pop_front()
            .expect("No permutation operations recorded")
    }

    /// Replays a Merkle path verification, returning the pre-recorded address and computed root
    pub fn replay_build_merkle_root(&mut self) -> (Felt, Word) {
        self.build_merkle_root_operations
            .pop_front()
            .expect("No build merkle root operations recorded")
    }

    /// Replays a Merkle root update, returning the pre-recorded address, old root, and new root
    pub fn replay_mrupdate(&mut self) -> (Felt, Word, Word) {
        self.mrupdate_operations.pop_front().expect("No mrupdate operations recorded")
    }
}

// pub struct HasherReplay {
//     // Permutation operations
//     pub permutations: VecDeque<(HasherState, Felt, HasherState)>, // (input_state, addr,
// output_state)

//     // Control block hashing operations
//     pub control_block_hashes: VecDeque<(Word, Word, Felt, RpoDigest, Felt, Word)>, // (h1, h2,
// domain, expected_hash, addr, result)

//     // Basic block hashing operations
//     pub basic_block_hashes: VecDeque<(usize, RpoDigest, Felt, Word)>, // (num_batches,
// expected_hash, addr, result)

//     // Merkle path verification operations
//     pub merkle_path_verifications: VecDeque<(Word, usize, Felt, Felt, Word)>, // (value,
// path_len, index, addr, root)

//     // Merkle root update operations
//     pub merkle_root_updates: VecDeque<(Word, Word, usize, Felt, MerkleRootUpdate)>, //
// (old_value, new_value, path_len, index, result) }

// impl HasherReplay {
//     /// Creates a new HasherReplay with empty operation vectors
//     pub fn new() -> Self {
//         Self {
//             permutations: VecDeque::new(),
//             control_block_hashes: VecDeque::new(),
//             basic_block_hashes: VecDeque::new(),
//             merkle_path_verifications: VecDeque::new(),
//             merkle_root_updates: VecDeque::new(),
//         }
//     }

//     // MUTATIONS (populated by the fast processor)
//     // --------------------------------------------------------------------------------

//     /// Records a permutation operation and its result
//     pub fn record_permutation(&mut self, input_state: HasherState, addr: Felt, output_state:
// HasherState) {         self.permutations.push_back((input_state, addr, output_state));
//     }

//     /// Records a control block hash operation and its result
//     pub fn record_control_block_hash(
//         &mut self,
//         h1: Word,
//         h2: Word,
//         domain: Felt,
//         expected_hash: RpoDigest,
//         addr: Felt,
//         result: Word,
//     ) {
//         self.control_block_hashes.push_back((h1, h2, domain, expected_hash, addr, result));
//     }

//     /// Records a basic block hash operation and its result
//     pub fn record_basic_block_hash(
//         &mut self,
//         num_batches: usize,
//         expected_hash: RpoDigest,
//         addr: Felt,
//         result: Word,
//     ) {
//         self.basic_block_hashes.push_back((num_batches, expected_hash, addr, result));
//     }

//     /// Records a Merkle path verification operation and its result
//     pub fn record_merkle_path_verification(
//         &mut self,
//         value: Word,
//         path_len: usize,
//         index: Felt,
//         addr: Felt,
//         root: Word,
//     ) {
//         self.merkle_path_verifications.push_back((value, path_len, index, addr, root));
//     }

//     /// Records a Merkle root update operation and its result
//     pub fn record_merkle_root_update(
//         &mut self,
//         old_value: Word,
//         new_value: Word,
//         path_len: usize,
//         index: Felt,
//         result: MerkleRootUpdate,
//     ) {
//         self.merkle_root_updates.push_back((old_value, new_value, path_len, index, result));
//     }

//     // ACCESSORS (used during parallel trace generation)
//     // --------------------------------------------------------------------------------

//     /// Replays a permutation operation, returning the previously recorded result
//     pub fn replay_permutation(&mut self, input_state: HasherState) -> (Felt, HasherState) {
//         let (recorded_input, addr, output_state) =
//             self.permutations.pop_front().expect("No permutation operations recorded");
//         debug_assert_eq!(
//             recorded_input, input_state,
//             "Permutation input state mismatch: expected {:?}, got {:?}",
//             input_state, recorded_input
//         );
//         (addr, output_state)
//     }

//     /// Replays a control block hash operation, returning the previously recorded result
//     pub fn replay_control_block_hash(
//         &mut self,
//         h1: Word,
//         h2: Word,
//         domain: Felt,
//         expected_hash: RpoDigest,
//     ) -> (Felt, Word) {
//         let (recorded_h1, recorded_h2, recorded_domain, recorded_hash, addr, result) =
//             self.control_block_hashes.pop_front().expect("No control block hash operations
// recorded");         debug_assert_eq!(
//             recorded_h1, h1,
//             "Control block hash h1 mismatch: expected {:?}, got {:?}",
//             h1, recorded_h1
//         );
//         debug_assert_eq!(
//             recorded_h2, h2,
//             "Control block hash h2 mismatch: expected {:?}, got {:?}",
//             h2, recorded_h2
//         );
//         debug_assert_eq!(
//             recorded_domain, domain,
//             "Control block hash domain mismatch: expected {:?}, got {:?}",
//             domain, recorded_domain
//         );
//         debug_assert_eq!(
//             recorded_hash, expected_hash,
//             "Control block hash expected_hash mismatch: expected {:?}, got {:?}",
//             expected_hash, recorded_hash
//         );
//         (addr, result)
//     }

//     /// Replays a basic block hash operation, returning the previously recorded result
//     pub fn replay_basic_block_hash(
//         &mut self,
//         num_batches: usize,
//         expected_hash: RpoDigest,
//     ) -> (Felt, Word) {
//         let (recorded_num_batches, recorded_hash, addr, result) =
//             self.basic_block_hashes.pop_front().expect("No basic block hash operations
// recorded");         debug_assert_eq!(
//             recorded_num_batches, num_batches,
//             "Basic block hash num_batches mismatch: expected {}, got {}",
//             num_batches, recorded_num_batches
//         );
//         debug_assert_eq!(
//             recorded_hash, expected_hash,
//             "Basic block hash expected_hash mismatch: expected {:?}, got {:?}",
//             expected_hash, recorded_hash
//         );
//         (addr, result)
//     }

//     /// Replays a Merkle path verification operation, returning the previously recorded result
//     pub fn replay_merkle_path_verification(
//         &mut self,
//         value: Word,
//         path_len: usize,
//         index: Felt,
//     ) -> (Felt, Word) {
//         let (recorded_value, recorded_path_len, recorded_index, addr, root) =
//             self.merkle_path_verifications.pop_front().expect("No Merkle path verification
// operations recorded");         debug_assert_eq!(
//             recorded_value, value,
//             "Merkle path verification value mismatch: expected {:?}, got {:?}",
//             value, recorded_value
//         );
//         debug_assert_eq!(
//             recorded_path_len, path_len,
//             "Merkle path verification path_len mismatch: expected {}, got {}",
//             path_len, recorded_path_len
//         );
//         debug_assert_eq!(
//             recorded_index, index,
//             "Merkle path verification index mismatch: expected {:?}, got {:?}",
//             index, recorded_index
//         );
//         (addr, root)
//     }

//     /// Replays a Merkle root update operation, returning the previously recorded result
//     pub fn replay_merkle_root_update(
//         &mut self,
//         old_value: Word,
//         new_value: Word,
//         path_len: usize,
//         index: Felt,
//     ) -> MerkleRootUpdate {
//         let (recorded_old_value, recorded_new_value, recorded_path_len, recorded_index, result) =
//             self.merkle_root_updates.pop_front().expect("No Merkle root update operations
// recorded");         debug_assert_eq!(
//             recorded_old_value, old_value,
//             "Merkle root update old_value mismatch: expected {:?}, got {:?}",
//             old_value, recorded_old_value
//         );
//         debug_assert_eq!(
//             recorded_new_value, new_value,
//             "Merkle root update new_value mismatch: expected {:?}, got {:?}",
//             new_value, recorded_new_value
//         );
//         debug_assert_eq!(
//             recorded_path_len, path_len,
//             "Merkle root update path_len mismatch: expected {}, got {}",
//             path_len, recorded_path_len
//         );
//         debug_assert_eq!(
//             recorded_index, index,
//             "Merkle root update index mismatch: expected {:?}, got {:?}",
//             index, recorded_index
//         );
//         result
//     }
// }
