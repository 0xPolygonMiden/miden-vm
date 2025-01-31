use math::FieldElement;
use miden_crypto::{merkle::MerkleError, Felt, Word};
use miden_formatting::hex::to_hex;

#[derive(Debug, thiserror::Error)]
pub enum AdviceProviderError {
    #[error("value for key {} not present in the advice map", to_hex(Felt::elements_as_bytes(.0)))]
    AdviceMapKeyNotFound(Word),
    #[error("value for key {} already present in the advice map", to_hex(Felt::elements_as_bytes(.0)))]
    AdviceMapKeyAlreadyPresent(Word),
    #[error("advice stack read failed due to empty stack")]
    AdviceStackReadFailed,
    #[error("provided merkle tree {depth} is out of bounds and cannot be represented as an unsigned 8-bit integer")]
    InvalidMerkleTreeDepth { depth: Felt },
    #[error(
        "provided node index {value} is out of bounds for a merkle tree node at depth {depth}"
    )]
    InvalidMerkleTreeNodeIndex { depth: Felt, value: Felt },
    #[error("advice provider Merkle store backend lookup failed")]
    MerkleStoreLookupFailed(#[source] MerkleError),
    #[error("advice provider Merkle store backend merge failed")]
    MerkleStoreMergeFailed(#[source] MerkleError),
    #[error("advice provider Merkle store backend update failed")]
    MerkleStoreUpdateFailed(#[source] MerkleError),
}
