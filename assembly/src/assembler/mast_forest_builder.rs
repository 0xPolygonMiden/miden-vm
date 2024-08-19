use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use core::ops::Index;

use vm_core::{
    crypto::hash::RpoDigest,
    mast::{MastForest, MastNode, MastNodeId},
    DecoratorList, Operation,
};

use super::{GlobalProcedureIndex, Procedure};
use crate::AssemblyError;

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
        if self.procedures.contains_key(&gid) {
            // The global procedure index and the MAST root resolve to an already cached version of
            // this procedure, or an alias of it, nothing to do.
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
            // Handle the case where a procedure with no locals is lowered to a MastForest
            // consisting only of an `External` node to another procedure which has one or more
            // locals. This will result in the calling procedure having the same digest as the
            // callee, but the two procedures having mismatched local counts. When this occurs,
            // we want to use the procedure with non-zero local count as the definition, and treat
            // the other procedure as an alias, which can be referenced like any other procedure,
            // but the MAST returned for it will be that of the "real" definition.
            let cached_locals = cached.num_locals();
            let procedure_locals = procedure.num_locals();
            let mismatched_locals = cached_locals != procedure_locals;
            let is_valid =
                !mismatched_locals || core::cmp::min(cached_locals, procedure_locals) == 0;
            if !is_valid {
                return Err(AssemblyError::ConflictingDefinitions {
                    first: cached.fully_qualified_name().clone(),
                    second: procedure.fully_qualified_name().clone(),
                });
            }
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

    /// Builds a tree of `JOIN` operations to combine all the top-level MAST node IDs of the
    /// procedure body.
    pub fn join_mast_node_ids(
        &mut self,
        mast_node_ids: Vec<MastNodeId>,
    ) -> Result<MastNodeId, AssemblyError> {
        debug_assert!(!mast_node_ids.is_empty(), "cannot combine empty MAST node id list");

        let mut mast_node_ids = self.merge_contiguous_basic_blocks(mast_node_ids)?;

        // build a binary tree of blocks joining them using JOIN blocks
        while mast_node_ids.len() > 1 {
            let last_mast_node_id = if mast_node_ids.len() % 2 == 0 {
                None
            } else {
                mast_node_ids.pop()
            };

            let mut source_mast_node_ids = Vec::new();
            core::mem::swap(&mut mast_node_ids, &mut source_mast_node_ids);

            let mut source_mast_node_iter = source_mast_node_ids.drain(0..);
            while let (Some(left), Some(right)) =
                (source_mast_node_iter.next(), source_mast_node_iter.next())
            {
                let join_mast_node_id = self.ensure_join(left, right)?;

                mast_node_ids.push(join_mast_node_id);
            }
            if let Some(mast_node_id) = last_mast_node_id {
                mast_node_ids.push(mast_node_id);
            }
        }

        Ok(mast_node_ids.remove(0))
    }

    /// Returns a list of [`MastNodeId`]s built from merging the contiguous basic blocks
    /// found in the provided list of [`MastNodeId`]s.
    fn merge_contiguous_basic_blocks(
        &mut self,
        mast_node_ids: Vec<MastNodeId>,
    ) -> Result<Vec<MastNodeId>, AssemblyError> {
        let mut merged_mast_node_ids = Vec::with_capacity(mast_node_ids.len());
        let mut contiguous_basic_block_ids: Vec<MastNodeId> = Vec::new();

        for mast_node_id in mast_node_ids {
            if self[mast_node_id].is_basic_block() {
                contiguous_basic_block_ids.push(mast_node_id);
            } else {
                if let Some(merged_basic_block_id) =
                    self.merge_basic_blocks(&contiguous_basic_block_ids)?
                {
                    merged_mast_node_ids.push(merged_basic_block_id)
                }
                contiguous_basic_block_ids.clear();

                merged_mast_node_ids.push(mast_node_id);
            }
        }

        if let Some(merged_basic_block_id) = self.merge_basic_blocks(&contiguous_basic_block_ids)? {
            merged_mast_node_ids.push(merged_basic_block_id)
        }

        Ok(merged_mast_node_ids)
    }

    /// Creates a new basic block by appending all operations and decorators in the provided list of
    /// basic blocks (which are assumed to be contiguous).
    ///
    /// # Panics
    /// - Panics if a provided [`MastNodeId`] doesn't refer to a basic block node.
    fn merge_basic_blocks(
        &mut self,
        contiguous_basic_block_ids: &[MastNodeId],
    ) -> Result<Option<MastNodeId>, AssemblyError> {
        if contiguous_basic_block_ids.is_empty() {
            return Ok(None);
        }
        if contiguous_basic_block_ids.len() == 1 {
            return Ok(Some(contiguous_basic_block_ids[0]));
        }

        let mut operations: Vec<Operation> = Vec::new();
        let mut decorators = DecoratorList::new();

        for &basic_block_node_id in contiguous_basic_block_ids {
            // It is safe to unwrap here, since we already checked that all IDs in
            // `contiguous_basic_block_ids` are `BasicBlockNode`s
            let basic_block_node = self[basic_block_node_id].get_basic_block().unwrap();

            for (op_idx, decorator) in basic_block_node.decorators() {
                decorators.push((*op_idx + operations.len(), decorator.clone()));
            }
            for batch in basic_block_node.op_batches() {
                operations.extend_from_slice(batch.ops());
            }
        }

        let merged_basic_block = self.ensure_block(operations, Some(decorators))?;
        Ok(Some(merged_basic_block))
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
        let block = MastNode::new_basic_block(operations, decorators)?;
        self.ensure_node(block)
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
