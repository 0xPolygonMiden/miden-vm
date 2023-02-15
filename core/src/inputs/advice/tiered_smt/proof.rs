use crypto::hash::rpo::RpoDigest;

use super::{CanonicalWord, Leaf, LeafIndex, MerklePath, TieredSmt, Word, EMPTY_SUBTREES};

// LEAF PROOF
// ================================================================================================

/// Encapsulates the arguments to prove the membership of a leaf for a given tiered tree root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafProof {
    pub input: LeafProofInput,
    pub path: MerklePath,
}

impl LeafProof {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new instance of a membership proof.
    pub fn new(input: LeafProofInput, path: MerklePath) -> Self {
        Self { input, path }
    }

    // PROVIDERS
    // --------------------------------------------------------------------------------------------

    /// Verifies the membership of the key-value pair for the given Merkle root of a `[TieredSmt]`.
    pub fn verify_membership<K>(&self, key: K, value: Word, root: &Word) -> bool
    where
        CanonicalWord: From<K>,
    {
        let key = CanonicalWord::from(key);
        let candidate = Leaf::new(key, value);

        // compute the depth of the proof.
        let depth = self.path.len() as u8;
        let index = LeafIndex::from(&candidate).traverse_to(depth).index();

        // compute the input of the merkle opening.
        let node = match &self.input {
            LeafProofInput::Lower(leaves) => {
                if !leaves.contains(&candidate) {
                    return false;
                }
                TieredSmt::hash_ordered_leaves(leaves)
            }
            _ => candidate.hash(depth),
        };

        // execute the merkle opening verification.
        self.path.verify(index, node.into(), root)
    }

    /// Verifies the non-membership of the key-value pair for the given Merkle root of a
    /// `[TieredSmt]`.
    pub fn verify_non_membership<K>(&self, key: K, value: Word, root: &Word) -> bool
    where
        CanonicalWord: From<K>,
    {
        let key = CanonicalWord::from(key);
        let candidate = Leaf::new(key, value);

        // compute the depth of the proof.
        let depth = self.path.len() as u8;
        let index = LeafIndex::from(&candidate).traverse_to(depth).index();

        // compute the input of the merkle opening.
        let node = match &self.input {
            LeafProofInput::Empty => RpoDigest::new(EMPTY_SUBTREES[depth as usize]),
            LeafProofInput::Lower(leaves) => {
                if leaves.contains(&candidate) {
                    return false;
                }
                TieredSmt::hash_ordered_leaves(leaves)
            }
            LeafProofInput::Upper(leaf) => {
                if leaf == &candidate {
                    return false;
                }
                leaf.hash(depth)
            }
        };

        // execute the merkle opening verification.
        self.path.verify(index, node.into(), root)
    }
}

// LEAF PROOF INPUT
// ================================================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafProofInput {
    Empty,
    Lower(Vec<Leaf>),
    Upper(Leaf),
}

impl Default for LeafProofInput {
    fn default() -> Self {
        Self::Empty
    }
}