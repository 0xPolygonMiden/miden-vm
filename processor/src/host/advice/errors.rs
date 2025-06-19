use vm_core::utils::to_hex;

use crate::{Felt, FieldElement, Word, crypto::MerkleError};

#[derive(Debug, thiserror::Error)]
pub enum AdviceProviderError {
    #[error("value for key {} not present in the advice map", to_hex(Felt::elements_as_bytes(.key)))]
    AdviceMapKeyNotFound { key: Word },
    #[error("advice stack read failed")]
    AdviceStackReadFailed,
    #[error(
        "provided merkle tree {depth} is out of bounds and cannot be represented as an unsigned 8-bit integer"
    )]
    InvalidMerkleTreeDepth { depth: Felt },
    #[error("provided node index {index} is out of bounds for a merkle tree node at depth {depth}")]
    InvalidMerkleTreeNodeIndex { depth: Felt, index: Felt },
    #[error("failed to lookup value in Merkle store")]
    MerkleStoreLookupFailed(#[source] MerkleError),
    /// Note: This error currently never occurs, since `MerkleStore::merge_roots()` never fails.
    #[error("advice provider Merkle store backend merge failed")]
    MerkleStoreMergeFailed(#[source] MerkleError),
    #[error("advice provider Merkle store backend update failed")]
    MerkleStoreUpdateFailed(#[source] MerkleError),
}
