use super::Felt;
use vm_core::{crypto::merkle::MerkleStore, FieldElement, Word};

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
