use core::ops::Index;

use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MastNode, MastNodeId},
    DecoratorList, Operation,
};

use crate::AssemblyError;

use super::{GlobalProcedureIndex, Procedure};

// MAST FOREST BUILDER
// ================================================================================================

/// Builder for a [`MastForest`].
#[derive(Clone, Debug, Default)]
pub struct MastForestBuilder {
    mast_forest: MastForest,
    node_id_by_hash: BTreeMap<RpoDigest, MastNodeId>,
    procedures: BTreeMap<GlobalProcedureIndex, Arc<Procedure>>,
    procedure_hashes: BTreeMap<GlobalProcedureIndex, RpoDigest>,
    proc_gid_by_hash: BTreeMap<RpoDigest, GlobalProcedureIndex>,
}

impl MastForestBuilder {
    pub fn build(self) -> MastForest {
        self.mast_forest
    }
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl MastForestBuilder {
    /// Returns a reference to the procedure with the specified [`GlobalProcedureIndex`], or None
    /// if such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn get_procedure(&self, gid: GlobalProcedureIndex) -> Option<Arc<Procedure>> {
        self.procedures.get(&gid).cloned()
    }

    /// Returns the hash of the procedure with the specified [`GlobalProcedureIndex`], or None if
    /// such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn get_procedure_hash(&self, gid: GlobalProcedureIndex) -> Option<RpoDigest> {
        self.procedure_hashes.get(&gid).cloned()
    }

    /// Returns a reference to the procedure with the specified MAST root, or None
    /// if such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn find_procedure(&self, mast_root: &RpoDigest) -> Option<Arc<Procedure>> {
        self.proc_gid_by_hash.get(mast_root).and_then(|gid| self.get_procedure(*gid))
    }

    /// Returns the [`MastNodeId`] of the procedure associated with a given MAST root, or None
    /// if such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn find_procedure_node_id(&self, digest: RpoDigest) -> Option<MastNodeId> {
        self.mast_forest.find_procedure_root(digest)
    }

    /// Returns the [`MastNode`] for the provided MAST node ID, or None if a node with this ID is
    /// not present in this MAST forest builder.
    pub fn get_mast_node(&self, id: MastNodeId) -> Option<&MastNode> {
        self.mast_forest.get_node_by_id(id)
    }
}

impl MastForestBuilder {
    pub fn insert_procedure_hash(
        &mut self,
        gid: GlobalProcedureIndex,
        proc_hash: RpoDigest,
    ) -> Result<(), AssemblyError> {
        // TODO(plafer): Check if exists
        self.procedure_hashes.insert(gid, proc_hash);

        Ok(())
    }

    /// Inserts a procedure into this MAST forest builder.
    ///
    /// If the procedure with the same ID already exists in this forest builder, this will have
    /// no effect.
    pub fn insert_procedure(
        &mut self,
        gid: GlobalProcedureIndex,
        procedure: Procedure,
    ) -> Result<(), AssemblyError> {
        let proc_root = self.mast_forest[procedure.body_node_id()].digest();

        // Check if an entry is already in this cache slot.
        //
        // If there is already a cache entry, but it conflicts with what we're trying to cache,
        // then raise an error.
        if let Some(cached) = self.procedures.get(&gid) {
            let cached_root = self.mast_forest[cached.body_node_id()].digest();
            if cached_root != proc_root || cached.num_locals() != procedure.num_locals() {
                return Err(AssemblyError::ConflictingDefinitions {
                    first: cached.fully_qualified_name().clone(),
                    second: procedure.fully_qualified_name().clone(),
                });
            }

            // The global procedure index and the MAST root resolve to an already cached version of
            // this procedure, nothing to do.
            //
            // TODO: We should emit a warning for this, because while it is not an error per se, it
            // does reflect that we're doing work we don't need to be doing. However, emitting a
            // warning only makes sense if this is controllable by the user, and it isn't yet
            // clear whether this edge case will ever happen in practice anyway.
            return Ok(());
        }

        // We don't have a cache entry yet, but we do want to make sure we don't have a conflicting
        // cache entry with the same MAST root:
        if let Some(cached) = self.find_procedure(&proc_root) {
            if cached.num_locals() != procedure.num_locals() {
                return Err(AssemblyError::ConflictingDefinitions {
                    first: cached.fully_qualified_name().clone(),
                    second: procedure.fully_qualified_name().clone(),
                });
            }

            // We have a previously cached version of an equivalent procedure, just under a
            // different [GlobalProcedureIndex], so insert the cached procedure into the slot for
            // `id`, but skip inserting a record in the MAST root lookup table
            self.make_root(procedure.body_node_id());
            self.insert_procedure_hash(gid, procedure.mast_root())?;
            self.procedures.insert(gid, Arc::new(procedure));
            return Ok(());
        }

        self.make_root(procedure.body_node_id());
        self.proc_gid_by_hash.insert(proc_root, gid);
        self.insert_procedure_hash(gid, procedure.mast_root())?;
        self.procedures.insert(gid, Arc::new(procedure));

        Ok(())
    }

    /// Marks the given [`MastNodeId`] as being the root of a procedure.
    pub fn make_root(&mut self, new_root_id: MastNodeId) {
        self.mast_forest.make_root(new_root_id)
    }
}

// ------------------------------------------------------------------------------------------------
/// Node inserters
impl MastForestBuilder {
    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
    ///
    /// If a [`MastNode`] which is equal to the current node was previously added, the previously
    /// returned [`MastNodeId`] will be returned. This enforces this invariant that equal
    /// [`MastNode`]s have equal [`MastNodeId`]s.
    fn ensure_node(&mut self, node: MastNode) -> Result<MastNodeId, AssemblyError> {
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
    ) -> Result<MastNodeId, AssemblyError> {
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
    ) -> Result<MastNodeId, AssemblyError> {
        let join = MastNode::new_join(left_child, right_child, &self.mast_forest)?;
        self.ensure_node(join)
    }

    /// Adds a split node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_split(
        &mut self,
        if_branch: MastNodeId,
        else_branch: MastNodeId,
    ) -> Result<MastNodeId, AssemblyError> {
        let split = MastNode::new_split(if_branch, else_branch, &self.mast_forest)?;
        self.ensure_node(split)
    }

    /// Adds a loop node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_loop(&mut self, body: MastNodeId) -> Result<MastNodeId, AssemblyError> {
        let loop_node = MastNode::new_loop(body, &self.mast_forest)?;
        self.ensure_node(loop_node)
    }

    /// Adds a call node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_call(&mut self, callee: MastNodeId) -> Result<MastNodeId, AssemblyError> {
        let call = MastNode::new_call(callee, &self.mast_forest)?;
        self.ensure_node(call)
    }

    /// Adds a syscall node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_syscall(&mut self, callee: MastNodeId) -> Result<MastNodeId, AssemblyError> {
        let syscall = MastNode::new_syscall(callee, &self.mast_forest)?;
        self.ensure_node(syscall)
    }

    /// Adds a dyn node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_dyn(&mut self) -> Result<MastNodeId, AssemblyError> {
        self.ensure_node(MastNode::new_dyn())
    }

    /// Adds an external node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_external(&mut self, mast_root: RpoDigest) -> Result<MastNodeId, AssemblyError> {
        self.ensure_node(MastNode::new_external(mast_root))
    }
}

impl Index<MastNodeId> for MastForestBuilder {
    type Output = MastNode;

    #[inline(always)]
    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        &self.mast_forest[node_id]
    }
}
