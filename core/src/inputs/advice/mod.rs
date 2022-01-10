use super::BaseElement;

mod merkle_tree;
use merkle_tree::MerkleTree;

// TYPE ALIASES
// ================================================================================================

type Node = [BaseElement; 4];

// MERKLE SET
// ================================================================================================

/// TODO: add docs
#[derive(Clone, Debug)]
pub enum AdviceSet {
    MerkleTree(MerkleTree),
}

impl AdviceSet {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [AdviceSet] instantiated as a Merkle tree from the provided leaves.
    ///
    /// # Errors
    /// TODO: Returns an error of the number of leaves is not a power of two.
    pub fn new_merkle_tree(leaves: Vec<Node>) -> Self {
        Self::MerkleTree(MerkleTree::new(leaves))
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a root of this Merkle set.
    pub fn root(&self) -> Node {
        match self {
            Self::MerkleTree(tree) => tree.root(),
        }
    }

    pub fn depth(&self) -> u32 {
        match self {
            Self::MerkleTree(tree) => tree.depth(),
        }
    }

    pub fn get_node(&self, depth: u32, index: u64) -> Node {
        match self {
            Self::MerkleTree(tree) => tree.get_node(depth, index),
        }
    }

    pub fn get_path(&self, depth: u32, index: u64) -> Vec<Node> {
        match self {
            Self::MerkleTree(tree) => tree.get_path(depth, index),
        }
    }
}
