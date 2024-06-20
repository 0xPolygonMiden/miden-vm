use core::{fmt, ops::Index};

use alloc::vec::Vec;
use miden_crypto::hash::rpo::RpoDigest;

mod node;
pub use node::{
    get_span_op_group_count, BasicBlockNode, CallNode, DynNode, JoinNode, LoopNode, MastNode,
    OpBatch, SplitNode, OP_BATCH_SIZE, OP_GROUP_SIZE,
};

use crate::Kernel;

#[cfg(test)]
mod tests;

/// Encapsulates the behavior that a [`MastNode`] (and all its variants) is expected to have.
pub trait MerkleTreeNode {
    fn digest(&self) -> RpoDigest;
    fn to_display<'a>(&'a self, mast_forest: &'a MastForest) -> impl fmt::Display + 'a;
}

// TODOP: Remove `PartialEq/Eq` impls
/// An opaque handle to a [`MastNode`] in some [`MastForest`]. It is the responsibility of the user
/// to use a given [`MastNodeId`] with the corresponding [`MastForest`].
///
/// Note that since a [`MastForest`] enforces the invariant that equal [`MastNode`]s MUST have equal
/// [`MastNodeId`]s, [`MastNodeId`] equality can be used to determine equality of the underlying
/// [`MastNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MastNodeId(u32);

impl fmt::Display for MastNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MastNodeId({})", self.0)
    }
}

// MAST FOREST
// ===============================================================================================

#[derive(Clone, Debug, Default)]
pub struct MastForest {
    /// All of the blocks local to the trees comprising the MAST forest.
    nodes: Vec<MastNode>,

    /// Roots of all procedures defined within this MAST forest.
    roots: Vec<MastNodeId>,

    // TODOP: Move fields to `Program`

    /// The "entrypoint", when set, is the root of the entire forest, i.e. a path exists from this
    /// node to all other roots in the forest. This corresponds to the executable entry point.
    /// Whether or not the entrypoint is set distinguishes a MAST which is executable, versus a
    /// MAST which represents a library.
    entrypoint: Option<MastNodeId>,
    kernel: Kernel,
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
    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
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

    // TODOP: Document
    pub fn ensure_root(&mut self, new_root_id: MastNodeId) {
        if !self.roots.contains(&new_root_id) {
            self.roots.push(new_root_id);
        }
    }

    /// Sets the kernel for this forest.
    ///
    /// The kernel MUST have been compiled using this [`MastForest`]; that is, all kernel procedures
    /// must be present in this forest.
    pub fn set_kernel(&mut self, kernel: Kernel) {
        #[cfg(debug_assertions)]
        for &proc_hash in kernel.proc_hashes() {
            assert!(self.find_root(proc_hash).is_some());
        }

        self.kernel = kernel;
    }

    /// Sets the entrypoint for this forest. This also ensures that the entrypoint is a root in the
    /// forest.
    pub fn set_entrypoint(&mut self, entrypoint: MastNodeId) {
        self.ensure_root(entrypoint);
        self.entrypoint = Some(entrypoint);
    }
}

/// Public accessors
impl MastForest {
    /// Returns the kernel associated with this forest.
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// Returns the entrypoint associated with this forest, if any.
    pub fn entrypoint(&self) -> Option<MastNodeId> {
        self.entrypoint
    }

    /// A convenience method that provides the hash of the entrypoint, if any.
    pub fn entrypoint_digest(&self) -> Option<RpoDigest> {
        self.entrypoint.map(|entrypoint| self[entrypoint].digest())
    }

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
    pub fn find_root(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.roots.iter().find(|&&root_id| self[root_id].digest() == digest).copied()
    }

    /// Returns an iterator over the digest of the procedures in this MAST forest.
    pub fn roots(&self) -> impl Iterator<Item = RpoDigest> + '_ {
        self.roots.iter().map(|&root_id| self[root_id].digest())
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
