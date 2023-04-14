use super::{Felt, FieldElement, StarkField, Vec, Word};

// RE-EXPORTS
// ================================================================================================

pub use vm_core::crypto::{
    hash::{Rpo256, RpoDigest},
    merkle::{
        EmptySubtreeRoots, MerkleError, MerklePath, MerklePathSet, MerkleStore, MerkleTree, Mmr,
        MmrPeaks, NodeIndex,
    },
};

pub use winter_prover::crypto::{BatchMerkleProof, ElementHasher, Hasher};

// CRYPTO HELPER FUNCTIONS
// ================================================================================================

pub fn init_merkle_store(values: &[u64]) -> (Vec<Word>, MerkleStore) {
    let leaves = init_merkle_leaves(values);
    let store = MerkleStore::new().with_merkle_tree(leaves.clone()).unwrap();
    (leaves, store)
}

pub fn init_merkle_leaves(values: &[u64]) -> Vec<Word> {
    values.iter().map(|&v| init_merkle_leaf(v)).collect()
}

pub fn init_merkle_leaf(value: u64) -> Word {
    [Felt::new(value), Felt::ZERO, Felt::ZERO, Felt::ZERO]
}

/// Returns a remaining path key for a Sparse Merkle tree
pub fn get_smt_remaining_key(mut key: Word, depth: u8) -> Word {
    key[3] = Felt::new(match depth {
        16 | 32 | 48 => (key[3].as_int() << depth) >> depth,
        64 => 0,
        _ => unreachable!(),
    });
    key
}
