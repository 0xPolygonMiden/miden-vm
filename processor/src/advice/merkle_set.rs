use super::{MerkleError, MerklePath, MerklePathSet, MerkleTree, NodeIndex, SimpleSmt, Vec, Word};

// MERKLE SET
// ================================================================================================

/// TODO: add docs
#[derive(Clone, Debug)]
pub enum MerkleSet {
    MerkleTree(MerkleTree),
    SparseMerkleTree(SimpleSmt),
    MerklePathSet(MerklePathSet),
}

impl MerkleSet {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new [MerkleSet] instantiated as a Merkle tree from the provided leaves.
    ///
    /// For more information, check `[MerkleTree::new]`.
    pub fn new_merkle_tree(leaves: Vec<Word>) -> Result<Self, MerkleError> {
        MerkleTree::new(leaves).map(Self::MerkleTree)
    }

    /// Returns a new [MerkleSet] instantiated as a Sparse Merkle tree from the provided leaves.
    ///
    /// For more information, check `[SimpleSmt::new]`.
    pub fn new_sparse_merkle_tree(
        keys: Vec<u64>,
        values: Vec<Word>,
        depth: u8,
    ) -> Result<Self, MerkleError> {
        SimpleSmt::new(depth)?
            .with_leaves(keys.into_iter().zip(values.into_iter()))
            .map(Self::SparseMerkleTree)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a root of this merkle set.
    pub fn root(&self) -> Word {
        match self {
            Self::MerkleTree(tree) => tree.root(),
            Self::SparseMerkleTree(tree) => tree.root(),
            Self::MerklePathSet(set) => set.root(),
        }
    }

    /// Returns the maximum depth of this merkle set.
    pub fn depth(&self) -> u8 {
        match self {
            Self::MerkleTree(tree) => tree.depth(),
            Self::SparseMerkleTree(tree) => tree.depth(),
            Self::MerklePathSet(set) => set.depth(),
        }
    }

    /// Returns a node located at the specified index.
    ///
    /// For more information, check the following concrete implementations:
    /// - `[MerkleTree::get_node]`
    /// - `[SimpleSmt::get_node]`
    /// - `[MerklePathSet::get_node]`
    pub fn get_node(&self, index: NodeIndex) -> Result<Word, MerkleError> {
        match self {
            Self::MerkleTree(tree) => tree.get_node(index),
            Self::SparseMerkleTree(tree) => tree.get_node(&index),
            Self::MerklePathSet(set) => set.get_node(index),
        }
    }

    /// Returns a Merkle path to a node located at the specified index. The node itself is not
    /// included in the path.
    ///
    /// For more information, check the following concrete implementations:
    /// - `[MerkleTree::get_path]`
    /// - `[SimpleSmt::get_path]`
    /// - `[MerklePathSet::get_path]`
    pub fn get_path(&self, index: NodeIndex) -> Result<MerklePath, MerkleError> {
        match self {
            Self::MerkleTree(tree) => tree.get_path(index),
            Self::SparseMerkleTree(tree) => tree.get_path(index),
            Self::MerklePathSet(set) => set.get_path(index),
        }
    }

    // DATA MUTATORS
    // --------------------------------------------------------------------------------------------

    /// Replaces the leaf at the specified index with the provided value.
    ///
    /// For more information, check the following concrete implementations:
    /// - `[MerkleTree::update_leaf]`
    /// - `[SimpleSmt::update_leaf]`
    /// - `[MerklePathSet::update_leaf]`
    pub fn update_leaf(&mut self, index: u64, value: Word) -> Result<(), MerkleError> {
        match self {
            Self::MerkleTree(tree) => tree.update_leaf(index, value),
            Self::SparseMerkleTree(tree) => tree.update_leaf(index, value),
            Self::MerklePathSet(set) => set.update_leaf(index, value),
        }
    }
}
