use core::ops::Index;

use alloc::{collections::BTreeMap, vec::Vec};
use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MastForestError, MastNode, MastNodeId},
    DecoratorList, Operation,
};

/// Builder for a [`MastForest`].
#[derive(Clone, Debug, Default)]
pub struct MastForestBuilder {
    mast_forest: MastForest,
    node_id_by_hash: BTreeMap<RpoDigest, MastNodeId>,
}

impl MastForestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MastForest {
        self.mast_forest
    }
}

/// Accessors
impl MastForestBuilder {
    /// Returns the underlying [`MastForest`] being built
    pub fn forest(&self) -> &MastForest {
        &self.mast_forest
    }

    /// Returns the [`MastNodeId`] of the procedure associated with a given digest, if any.
    #[inline(always)]
    pub fn find_procedure_root(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.mast_forest.find_procedure_root(digest)
    }
}

/// Mutators
impl MastForestBuilder {
    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
    ///
    /// If a [`MastNode`] which is equal to the current node was previously added, the previously
    /// returned [`MastNodeId`] will be returned. This enforces this invariant that equal
    /// [`MastNode`]s have equal [`MastNodeId`]s.
    fn ensure_node(&mut self, node: MastNode) -> Result<MastNodeId, MastForestError> {
        let node_digest = node.digest();

        if let Some(node_id) = self.node_id_by_hash.get(&node_digest) {
            // node already exists in the forest; return previously assigned id
            Ok(*node_id)
        } else {
            let new_node_id = self.mast_forest.add_node(node)?;
            self.node_id_by_hash.insert(node_digest, new_node_id);

            Ok(new_node_id)
        }
    }

    /// Adds a basic block node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_block(
        &mut self,
        operations: Vec<Operation>,
        decorators: Option<DecoratorList>,
    ) -> Result<MastNodeId, MastForestError> {
        match decorators {
            Some(decorators) => {
                self.ensure_node(MastNode::new_basic_block_with_decorators(operations, decorators))
            }
            None => self.ensure_node(MastNode::new_basic_block(operations)),
        }
    }

    /// Adds a join node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_join(
        &mut self,
        left_child: MastNodeId,
        right_child: MastNodeId,
    ) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_join(left_child, right_child, &self.mast_forest))
    }

    /// Adds a split node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_split(
        &mut self,
        if_branch: MastNodeId,
        else_branch: MastNodeId,
    ) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_split(if_branch, else_branch, &self.mast_forest))
    }

    /// Adds a loop node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_loop(&mut self, body: MastNodeId) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_loop(body, &self.mast_forest))
    }

    /// Adds a call node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_call(&mut self, callee: MastNodeId) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_call(callee, &self.mast_forest))
    }

    /// Adds a syscall node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_syscall(&mut self, callee: MastNodeId) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_syscall(callee, &self.mast_forest))
    }

    /// Adds a dynexec node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_dyn(&mut self) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_dyn())
    }

    /// Adds an external node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_external(&mut self, mast_root: RpoDigest) -> Result<MastNodeId, MastForestError> {
        self.ensure_node(MastNode::new_external(mast_root))
    }

    /// Marks the given [`MastNodeId`] as being the root of a procedure.
    pub fn make_root(&mut self, new_root_id: MastNodeId) {
        self.mast_forest.make_root(new_root_id)
    }
}

impl Index<MastNodeId> for MastForestBuilder {
    type Output = MastNode;

    #[inline(always)]
    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        &self.mast_forest[node_id]
    }
}
