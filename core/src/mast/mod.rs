use core::{fmt, ops::Index};

use alloc::vec::Vec;
use miden_crypto::hash::rpo::RpoDigest;

mod node;
pub use node::{
    get_span_op_group_count, BasicBlockNode, CallNode, DynNode, JoinNode, LoopNode, MastNode,
    OpBatch, SplitNode, OP_BATCH_SIZE, OP_GROUP_SIZE,
};

#[cfg(test)]
mod tests;

/// Encapsulates the behavior that a [`MastNode`] (and all its variants) is expected to have.
pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a;
}

/// An opaque handle to a [`MastNode`] in some [`MastForest`]. It is the responsibility of the user
/// to use a given [`MastNodeId`] with the corresponding [`MastForest`].
///
/// Note that the [`MastForest`] does *not* ensure that equal [`MastNode`]s have equal
/// [`MastNodeId`] handles. Hence, [`MastNodeId`] equality must not be used to test for equality of
/// the underlying [`MastNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MastNodeId(u32);

impl fmt::Display for MastNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MastNodeId({})", self.0)
    }
}

// MAST FOREST
// ===============================================================================================

/// Represents one or more procedures, represented as a collection of [`MastNode`]s.
///
/// A [`MastForest`] does not have an entrypoint, and hence is not executable. A [`crate::Program`]
/// can be built from a [`MastForest`] to specify an entrypoint.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MastForest {
    /// All of the nodes local to the trees comprising the MAST forest.
    nodes: Vec<MastNode>,

    /// Roots of procedures defined within this MAST forest.
    roots: Vec<MastNodeId>,
}

/// Constructors
impl MastForest {
    /// Creates a new empty [`MastForest`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Mutators
impl MastForest {
    /// Adds a node to the forest, and returns the associated [`MastNodeId`].
    ///
    /// Adding two duplicate nodes will result in two distinct returned [`MastNodeId`]s.
    pub fn add_node(&mut self, node: MastNode) -> MastNodeId {
        let new_node_id = MastNodeId(
            self.nodes
                .len()
                .try_into()
                .expect("invalid node id: exceeded maximum number of nodes in a single forest"),
        );

        self.nodes.push(node);

        new_node_id
    }

    /// Marks the given [`MastNodeId`] as being the root of a procedure.
    pub fn make_root(&mut self, new_root_id: MastNodeId) {
        if !self.roots.contains(&new_root_id) {
            self.roots.push(new_root_id);
        }
    }
}

/// Public accessors
impl MastForest {
    /// Returns the [`MastNode`] associated with the provided [`MastNodeId`] if valid, or else
    /// `None`.
    ///
    /// This is the faillible version of indexing (e.g. `mast_forest[node_id]`).
    #[inline(always)]
    pub fn get_node_by_id(&self, node_id: MastNodeId) -> Option<&MastNode> {
        let idx = node_id.0 as usize;

        self.nodes.get(idx)
    }

    /// Returns the [`MastNodeId`] of the procedure associated with a given digest, if any.
    #[inline(always)]
    pub fn find_procedure_root(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.roots.iter().find(|&&root_id| self[root_id].digest() == digest).copied()
    }

    /// Returns an iterator over the digest of the procedures in this MAST forest.
    pub fn procedure_roots(&self) -> impl Iterator<Item = RpoDigest> + '_ {
        self.roots.iter().map(|&root_id| self[root_id].digest())
    }

    /// Returns the number of procedures in this MAST forest.
    pub fn num_procedures(&self) -> u32 {
        self.roots
            .len()
            .try_into()
            .expect("MAST forest contains more than 2^32 procedures.")
    }
}

impl Index<MastNodeId> for MastForest {
    type Output = MastNode;

    #[inline(always)]
    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        let idx = node_id.0 as usize;

        &self.nodes[idx]
    }
}
