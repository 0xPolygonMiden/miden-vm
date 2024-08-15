use alloc::vec::Vec;
use core::{fmt, ops::Index};

use miden_crypto::hash::rpo::RpoDigest;

mod node;
pub use node::{
    BasicBlockNode, CallNode, DynNode, ExternalNode, JoinNode, LoopNode, MastNode, OpBatch,
    OperationOrDecorator, SplitNode, OP_BATCH_SIZE, OP_GROUP_SIZE,
};
use winter_utils::DeserializationError;

use crate::{DecoratorList, Operation};

mod serialization;

#[cfg(test)]
mod tests;

// MAST FOREST
// ================================================================================================

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

// ------------------------------------------------------------------------------------------------
/// Constructors
impl MastForest {
    /// Creates a new empty [`MastForest`].
    pub fn new() -> Self {
        Self::default()
    }
}

// ------------------------------------------------------------------------------------------------
/// State mutators
impl MastForest {
    /// The maximum number of nodes that can be stored in a single MAST forest.
    const MAX_NODES: usize = (1 << 30) - 1;

    /// Adds a node to the forest, and returns the associated [`MastNodeId`].
    ///
    /// Adding two duplicate nodes will result in two distinct returned [`MastNodeId`]s.
    pub fn add_node(&mut self, node: MastNode) -> Result<MastNodeId, MastForestError> {
        if self.nodes.len() == Self::MAX_NODES {
            return Err(MastForestError::TooManyNodes);
        }

        let new_node_id = MastNodeId(self.nodes.len() as u32);
        self.nodes.push(node);

        Ok(new_node_id)
    }

    /// Adds a basic block node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_block(
        &mut self,
        operations: Vec<Operation>,
        decorators: Option<DecoratorList>,
    ) -> Result<MastNodeId, MastForestError> {
        match decorators {
            Some(decorators) => {
                self.add_node(MastNode::new_basic_block_with_decorators(operations, decorators))
            },
            None => self.add_node(MastNode::new_basic_block(operations)),
        }
    }

    /// Adds a join node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_join(
        &mut self,
        left_child: MastNodeId,
        right_child: MastNodeId,
    ) -> Result<MastNodeId, MastForestError> {
        let join = MastNode::new_join(left_child, right_child, self)?;
        self.add_node(join)
    }

    /// Adds a split node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_split(
        &mut self,
        if_branch: MastNodeId,
        else_branch: MastNodeId,
    ) -> Result<MastNodeId, MastForestError> {
        let split = MastNode::new_split(if_branch, else_branch, self)?;
        self.add_node(split)
    }

    /// Adds a loop node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_loop(&mut self, body: MastNodeId) -> Result<MastNodeId, MastForestError> {
        let loop_node = MastNode::new_loop(body, self)?;
        self.add_node(loop_node)
    }

    /// Adds a call node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_call(&mut self, callee: MastNodeId) -> Result<MastNodeId, MastForestError> {
        let call = MastNode::new_call(callee, self)?;
        self.add_node(call)
    }

    /// Adds a syscall node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_syscall(&mut self, callee: MastNodeId) -> Result<MastNodeId, MastForestError> {
        let syscall = MastNode::new_syscall(callee, self)?;
        self.add_node(syscall)
    }

    /// Adds a dyn node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_dyn(&mut self) -> Result<MastNodeId, MastForestError> {
        self.add_node(MastNode::new_dyn())
    }

    /// Adds an external node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn add_external(&mut self, mast_root: RpoDigest) -> Result<MastNodeId, MastForestError> {
        self.add_node(MastNode::new_external(mast_root))
    }

    /// Marks the given [`MastNodeId`] as being the root of a procedure.
    ///
    /// # Panics
    /// - if `new_root_id`'s internal index is larger than the number of nodes in this forest (i.e.
    ///   clearly doesn't belong to this MAST forest).
    pub fn make_root(&mut self, new_root_id: MastNodeId) {
        assert!((new_root_id.0 as usize) < self.nodes.len());

        if !self.roots.contains(&new_root_id) {
            self.roots.push(new_root_id);
        }
    }
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl MastForest {
    /// Returns the [`MastNode`] associated with the provided [`MastNodeId`] if valid, or else
    /// `None`.
    ///
    /// This is the failable version of indexing (e.g. `mast_forest[node_id]`).
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

    /// Returns true if a node with the specified ID is a root of a procedure in this MAST forest.
    pub fn is_procedure_root(&self, node_id: MastNodeId) -> bool {
        self.roots.contains(&node_id)
    }

    /// Returns an iterator over the digest of the procedures in this MAST forest.
    pub fn procedure_digests(&self) -> impl Iterator<Item = RpoDigest> + '_ {
        self.roots.iter().map(|&root_id| self[root_id].digest())
    }

    /// Returns an iterator over the IDs of the procedures in this MAST forest.
    pub fn procedure_roots(&self) -> &[MastNodeId] {
        &self.roots
    }

    /// Returns the number of procedures in this MAST forest.
    pub fn num_procedures(&self) -> u32 {
        self.roots
            .len()
            .try_into()
            .expect("MAST forest contains more than 2^32 procedures.")
    }

    /// Returns the number of nodes in this MAST forest.
    pub fn num_nodes(&self) -> u32 {
        self.nodes.len() as u32
    }
}

/// Destructors
impl MastForest {
    pub fn into_parts(self) -> (Vec<MastNode>, Vec<MastNodeId>) {
        let Self { nodes, roots } = self;

        (nodes, roots)
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

// MAST NODE ID
// ================================================================================================

/// An opaque handle to a [`MastNode`] in some [`MastForest`]. It is the responsibility of the user
/// to use a given [`MastNodeId`] with the corresponding [`MastForest`].
///
/// Note that the [`MastForest`] does *not* ensure that equal [`MastNode`]s have equal
/// [`MastNodeId`] handles. Hence, [`MastNodeId`] equality must not be used to test for equality of
/// the underlying [`MastNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MastNodeId(u32);

impl MastNodeId {
    /// Returns a new `MastNodeId` with the provided inner value, or an error if the provided
    /// `value` is greater than the number of nodes in the forest.
    ///
    /// For use in deserialization.
    pub fn from_u32_safe(
        value: u32,
        mast_forest: &MastForest,
    ) -> Result<Self, DeserializationError> {
        if (value as usize) < mast_forest.nodes.len() {
            Ok(Self(value))
        } else {
            Err(DeserializationError::InvalidValue(format!(
                "Invalid deserialized MAST node ID '{}', but only {} nodes in the forest",
                value,
                mast_forest.nodes.len(),
            )))
        }
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<MastNodeId> for usize {
    fn from(value: MastNodeId) -> Self {
        value.0 as usize
    }
}

impl From<MastNodeId> for u32 {
    fn from(value: MastNodeId) -> Self {
        value.0
    }
}

impl From<&MastNodeId> for u32 {
    fn from(value: &MastNodeId) -> Self {
        value.0
    }
}

impl From<u32> for MastNodeId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl fmt::Display for MastNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MastNodeId({})", self.0)
    }
}

// MAST FOREST ERROR
// ================================================================================================

/// Represents the types of errors that can occur when dealing with MAST forest.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MastForestError {
    #[error(
        "invalid node count: MAST forest exceeds the maximum of {} nodes",
        MastForest::MAX_NODES
    )]
    TooManyNodes,
    #[error("node id: {0} is greater than or equal to forest length: {1}")]
    NodeIdOverflow(MastNodeId, usize),
}
