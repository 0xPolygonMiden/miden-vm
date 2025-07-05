use alloc::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    vec::Vec,
};
use core::ops::{Index, IndexMut};

use miden_core::{
    AdviceMap, Decorator, DecoratorList, Felt, Operation, Word,
    mast::{
        DecoratorFingerprint, DecoratorId, MastForest, MastNode, MastNodeFingerprint, MastNodeId,
        Remapping, SubtreeIterator,
    },
};

use super::{GlobalProcedureIndex, LinkerError, Procedure};
use crate::{
    Library,
    diagnostics::{IntoDiagnostic, Report, WrapErr},
    report,
};

// CONSTANTS
// ================================================================================================

/// Constant that decides how many operation batches disqualify a procedure from inlining.
const PROCEDURE_INLINING_THRESHOLD: usize = 32;

// MAST FOREST BUILDER
// ================================================================================================

/// Builder for a [`MastForest`].
///
/// The purpose of the builder is to ensure that the underlying MAST forest contains as little
/// information as possible needed to adequately describe the logical MAST forest. Specifically:
/// - The builder ensures that only one copy of nodes that have the same MAST root and decorators is
///   added to the MAST forest (i.e., two nodes that have the same MAST root and decorators will
///   have the same [`MastNodeId`]).
/// - The builder tries to merge adjacent basic blocks and eliminate the source block whenever this
///   does not have an impact on other nodes in the forest.
#[derive(Clone, Debug, Default)]
pub struct MastForestBuilder {
    /// The MAST forest being built by this builder; this MAST forest is up-to-date - i.e., all
    /// nodes added to the MAST forest builder are also immediately added to the underlying MAST
    /// forest.
    mast_forest: MastForest,
    /// A map of all procedures added to the MAST forest indexed by their global procedure ID.
    /// This includes all local, exported, and re-exported procedures. In case multiple procedures
    /// with the same digest are added to the MAST forest builder, only the first procedure is
    /// added to the map, and all subsequent insertions are ignored.
    procedures: BTreeMap<GlobalProcedureIndex, Procedure>,
    /// A map from procedure MAST root to its global procedure index. Similar to the `procedures`
    /// map, this map contains only the first inserted procedure for procedures with the same MAST
    /// root.
    proc_gid_by_mast_root: BTreeMap<Word, GlobalProcedureIndex>,
    /// A map of MAST node fingerprints to their corresponding positions in the MAST forest.
    node_id_by_fingerprint: BTreeMap<MastNodeFingerprint, MastNodeId>,
    /// The reverse mapping of `node_id_by_fingerprint`. This map caches the fingerprints of all
    /// nodes (for performance reasons).
    hash_by_node_id: BTreeMap<MastNodeId, MastNodeFingerprint>,
    /// A map of decorator fingerprints to their corresponding positions in the MAST forest.
    decorator_id_by_fingerprint: BTreeMap<DecoratorFingerprint, DecoratorId>,
    /// A set of IDs for basic blocks which have been merged into a bigger basic blocks. This is
    /// used as a candidate set of nodes that may be eliminated if the are not referenced by any
    /// other node in the forest and are not a root of any procedure.
    merged_basic_block_ids: BTreeSet<MastNodeId>,
    /// A MastForest that contains the MAST of all statically-linked libraries, it's used to find
    /// precompiled procedures and copy their subtrees instead of inserting external nodes.
    statically_linked_mast: Arc<MastForest>,
    /// Keeps track of the new ids assigned to nodes that are copied from the MAST of
    /// statically-linked libraries.
    statically_linked_mast_remapping: Remapping,
}

impl MastForestBuilder {
    /// Creates a new builder which will transitively include the MAST of any procedures referenced
    /// in the provided set of statically-linked libraries.
    ///
    /// In all other cases, references to procedures not present in the main MastForest are assumed
    /// to be dynamically-linked, and are inserted as an external node. Dynamically-linked libraries
    /// must be provided separately to the processor at runtime.
    pub fn new<'a>(
        static_libraries: impl IntoIterator<Item = &'a Library>,
    ) -> Result<Self, Report> {
        // All statically-linked libraries are merged into a single MastForest.
        let forests = static_libraries.into_iter().map(|lib| lib.mast_forest().as_ref());
        let (statically_linked_mast, _remapping) = MastForest::merge(forests).into_diagnostic()?;
        // The AdviceMap of the statically-linkeed forest is copied to the forest being built.
        //
        // This might include excess advice map data in the built MastForest, but we currently do
        // not do any analysis to determine what advice map data is actually required by parts of
        // the library(s) that are actually linked into the output.
        let mut mast_forest = MastForest::default();
        *mast_forest.advice_map_mut() = statically_linked_mast.advice_map().clone();
        Ok(MastForestBuilder {
            mast_forest,
            statically_linked_mast: Arc::new(statically_linked_mast),
            ..Self::default()
        })
    }

    /// Removes the unused nodes that were created as part of the assembly process, and returns the
    /// resulting MAST forest.
    ///
    /// It also returns the map from old node IDs to new node IDs. Any [`MastNodeId`] used in
    /// reference to the old [`MastForest`] should be remapped using this map.
    pub fn build(mut self) -> (MastForest, BTreeMap<MastNodeId, MastNodeId>) {
        let nodes_to_remove = get_nodes_to_remove(self.merged_basic_block_ids, &self.mast_forest);
        let id_remappings = self.mast_forest.remove_nodes(&nodes_to_remove);

        (self.mast_forest, id_remappings)
    }
}

/// Takes the set of MAST node ids (all basic blocks) that were merged as part of the assembly
/// process (i.e. they were contiguous and were merged into a single basic block), and returns the
/// subset of nodes that can be removed from the MAST forest.
///
/// Specifically, MAST node ids can be reused, so merging a basic block doesn't mean it should be
/// removed (specifically in the case where another node refers to it). Hence, we cycle through all
/// nodes of the forest and only mark for removal those nodes that are not referenced by any node.
/// We also ensure that procedure roots are not removed.
fn get_nodes_to_remove(
    merged_node_ids: BTreeSet<MastNodeId>,
    mast_forest: &MastForest,
) -> BTreeSet<MastNodeId> {
    // make sure not to remove procedure roots
    let mut nodes_to_remove: BTreeSet<MastNodeId> = merged_node_ids
        .iter()
        .filter(|&&mast_node_id| !mast_forest.is_procedure_root(mast_node_id))
        .copied()
        .collect();

    for node in mast_forest.nodes() {
        match node {
            MastNode::Join(node) => {
                if nodes_to_remove.contains(&node.first()) {
                    nodes_to_remove.remove(&node.first());
                }
                if nodes_to_remove.contains(&node.second()) {
                    nodes_to_remove.remove(&node.second());
                }
            },
            MastNode::Split(node) => {
                if nodes_to_remove.contains(&node.on_true()) {
                    nodes_to_remove.remove(&node.on_true());
                }
                if nodes_to_remove.contains(&node.on_false()) {
                    nodes_to_remove.remove(&node.on_false());
                }
            },
            MastNode::Loop(node) => {
                if nodes_to_remove.contains(&node.body()) {
                    nodes_to_remove.remove(&node.body());
                }
            },
            MastNode::Call(node) => {
                if nodes_to_remove.contains(&node.callee()) {
                    nodes_to_remove.remove(&node.callee());
                }
            },
            MastNode::Block(_) | MastNode::Dyn(_) | MastNode::External(_) => (),
        }
    }

    nodes_to_remove
}

// ------------------------------------------------------------------------------------------------
/// Public accessors
impl MastForestBuilder {
    /// Returns a reference to the procedure with the specified [`GlobalProcedureIndex`], or None
    /// if such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn get_procedure(&self, gid: GlobalProcedureIndex) -> Option<&Procedure> {
        self.procedures.get(&gid)
    }

    /// Returns a reference to the procedure with the specified MAST root, or None
    /// if such a procedure is not present in this MAST forest builder.
    #[inline(always)]
    pub fn find_procedure_by_mast_root(&self, mast_root: &Word) -> Option<&Procedure> {
        self.proc_gid_by_mast_root
            .get(mast_root)
            .and_then(|gid| self.get_procedure(*gid))
    }

    /// Returns the [`MastNode`] for the provided MAST node ID, or None if a node with this ID is
    /// not present in this MAST forest builder.
    pub fn get_mast_node(&self, id: MastNodeId) -> Option<&MastNode> {
        self.mast_forest.get_node_by_id(id)
    }
}

// ------------------------------------------------------------------------------------------------
/// Procedure insertion
impl MastForestBuilder {
    /// Inserts a procedure into this MAST forest builder.
    ///
    /// If the procedure with the same ID already exists in this forest builder, this will have
    /// no effect.
    pub fn insert_procedure(
        &mut self,
        gid: GlobalProcedureIndex,
        procedure: Procedure,
    ) -> Result<(), Report> {
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
        if let Some(cached) = self.find_procedure_by_mast_root(&procedure.mast_root()) {
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
                let first = cached.fully_qualified_name();
                let second = procedure.fully_qualified_name();
                return Err(report!(
                    "two procedures found with same mast root, but conflicting definitions ('{}' and '{}')",
                    first,
                    second
                ));
            }
        }

        self.mast_forest.make_root(procedure.body_node_id());
        self.proc_gid_by_mast_root.insert(procedure.mast_root(), gid);
        self.procedures.insert(gid, procedure);

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
/// Joining nodes
impl MastForestBuilder {
    /// Builds a tree of `JOIN` operations to combine the provided MAST node IDs.
    pub fn join_nodes(&mut self, node_ids: Vec<MastNodeId>) -> Result<MastNodeId, Report> {
        debug_assert!(!node_ids.is_empty(), "cannot combine empty MAST node id list");

        let mut node_ids = self.merge_contiguous_basic_blocks(node_ids)?;

        // build a binary tree of blocks joining them using JOIN blocks
        while node_ids.len() > 1 {
            let last_mast_node_id = if node_ids.len() % 2 == 0 { None } else { node_ids.pop() };

            let mut source_node_ids = Vec::new();
            core::mem::swap(&mut node_ids, &mut source_node_ids);

            let mut source_mast_node_iter = source_node_ids.drain(0..);
            while let (Some(left), Some(right)) =
                (source_mast_node_iter.next(), source_mast_node_iter.next())
            {
                let join_mast_node_id = self.ensure_join(left, right)?;

                node_ids.push(join_mast_node_id);
            }
            if let Some(mast_node_id) = last_mast_node_id {
                node_ids.push(mast_node_id);
            }
        }

        Ok(node_ids.remove(0))
    }

    /// Returns a list of [`MastNodeId`]s built from merging the contiguous basic blocks
    /// found in the provided list of [`MastNodeId`]s.
    fn merge_contiguous_basic_blocks(
        &mut self,
        node_ids: Vec<MastNodeId>,
    ) -> Result<Vec<MastNodeId>, Report> {
        let mut merged_node_ids = Vec::with_capacity(node_ids.len());
        let mut contiguous_basic_block_ids: Vec<MastNodeId> = Vec::new();

        for mast_node_id in node_ids {
            if self.mast_forest[mast_node_id].is_basic_block() {
                contiguous_basic_block_ids.push(mast_node_id);
            } else {
                merged_node_ids.extend(self.merge_basic_blocks(&contiguous_basic_block_ids)?);
                contiguous_basic_block_ids.clear();

                merged_node_ids.push(mast_node_id);
            }
        }

        merged_node_ids.extend(self.merge_basic_blocks(&contiguous_basic_block_ids)?);

        Ok(merged_node_ids)
    }

    /// Creates a new basic block by appending all operations and decorators in the provided list of
    /// basic blocks (which are assumed to be contiguous).
    ///
    /// # Panics
    /// - Panics if a provided [`MastNodeId`] doesn't refer to a basic block node.
    fn merge_basic_blocks(
        &mut self,
        contiguous_basic_block_ids: &[MastNodeId],
    ) -> Result<Vec<MastNodeId>, Report> {
        if contiguous_basic_block_ids.is_empty() {
            return Ok(Vec::new());
        }
        if contiguous_basic_block_ids.len() == 1 {
            return Ok(contiguous_basic_block_ids.to_vec());
        }

        let mut operations: Vec<Operation> = Vec::new();
        let mut decorators = DecoratorList::new();

        let mut merged_basic_blocks: Vec<MastNodeId> = Vec::new();

        for &basic_block_id in contiguous_basic_block_ids {
            // It is safe to unwrap here, since we already checked that all IDs in
            // `contiguous_basic_block_ids` are `BasicBlockNode`s
            let basic_block_node =
                self.mast_forest[basic_block_id].get_basic_block().unwrap().clone();

            // check if the block should be merged with other blocks
            if should_merge(
                self.mast_forest.is_procedure_root(basic_block_id),
                basic_block_node.num_op_batches(),
            ) {
                for &(op_idx, decorator) in basic_block_node.decorators() {
                    decorators.push((op_idx + operations.len(), decorator));
                }
                for batch in basic_block_node.op_batches() {
                    operations.extend_from_slice(batch.ops());
                }
            } else {
                // if we don't want to merge this block, we flush the buffer of operations into a
                // new block, and add the un-merged block after it
                if !operations.is_empty() {
                    let block_ops = core::mem::take(&mut operations);
                    let block_decorators = core::mem::take(&mut decorators);
                    let merged_basic_block_id =
                        self.ensure_block(block_ops, Some(block_decorators))?;

                    merged_basic_blocks.push(merged_basic_block_id);
                }
                merged_basic_blocks.push(basic_block_id);
            }
        }

        // Mark the removed basic blocks as merged
        self.merged_basic_block_ids.extend(contiguous_basic_block_ids.iter());

        if !operations.is_empty() || !decorators.is_empty() {
            let merged_basic_block = self.ensure_block(operations, Some(decorators))?;
            merged_basic_blocks.push(merged_basic_block);
        }

        Ok(merged_basic_blocks)
    }
}

// ------------------------------------------------------------------------------------------------
/// Node inserters
impl MastForestBuilder {
    /// Adds a decorator to the forest, and returns the [`Decorator`] associated with it.
    pub fn ensure_decorator(&mut self, decorator: Decorator) -> Result<DecoratorId, Report> {
        let decorator_hash = decorator.fingerprint();

        if let Some(decorator_id) = self.decorator_id_by_fingerprint.get(&decorator_hash) {
            // decorator already exists in the forest; return previously assigned id
            Ok(*decorator_id)
        } else {
            let new_decorator_id = self
                .mast_forest
                .add_decorator(decorator)
                .into_diagnostic()
                .wrap_err("assembler failed to add new decorator")?;
            self.decorator_id_by_fingerprint.insert(decorator_hash, new_decorator_id);

            Ok(new_decorator_id)
        }
    }

    /// Adds a node to the forest, and returns the [`MastNodeId`] associated with it.
    ///
    /// Note that only one copy of nodes that have the same MAST root and decorators is added to the
    /// MAST forest; two nodes that have the same MAST root and decorators will have the same
    /// [`MastNodeId`].
    pub fn ensure_node(&mut self, node: MastNode) -> Result<MastNodeId, Report> {
        let node_fingerprint = self.fingerprint_for_node(&node);

        if let Some(node_id) = self.node_id_by_fingerprint.get(&node_fingerprint) {
            // node already exists in the forest; return previously assigned id
            Ok(*node_id)
        } else {
            let new_node_id = self
                .mast_forest
                .add_node(node)
                .into_diagnostic()
                .wrap_err("assembler failed to add new node")?;
            self.node_id_by_fingerprint.insert(node_fingerprint, new_node_id);
            self.hash_by_node_id.insert(new_node_id, node_fingerprint);

            Ok(new_node_id)
        }
    }

    /// Adds a basic block node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_block(
        &mut self,
        operations: Vec<Operation>,
        decorators: Option<DecoratorList>,
    ) -> Result<MastNodeId, Report> {
        let block = MastNode::new_basic_block(operations, decorators)
            .into_diagnostic()
            .wrap_err("assembler failed to add new basic block node")?;
        self.ensure_node(block)
    }

    /// Adds a join node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_join(
        &mut self,
        left_child: MastNodeId,
        right_child: MastNodeId,
    ) -> Result<MastNodeId, Report> {
        let join = MastNode::new_join(left_child, right_child, &self.mast_forest)
            .into_diagnostic()
            .wrap_err("assembler failed to add new join node")?;
        self.ensure_node(join)
    }

    /// Adds a split node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_split(
        &mut self,
        if_branch: MastNodeId,
        else_branch: MastNodeId,
    ) -> Result<MastNodeId, Report> {
        let split = MastNode::new_split(if_branch, else_branch, &self.mast_forest)
            .into_diagnostic()
            .wrap_err("assembler failed to add new split node")?;
        self.ensure_node(split)
    }

    /// Adds a loop node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_loop(&mut self, body: MastNodeId) -> Result<MastNodeId, Report> {
        let loop_node = MastNode::new_loop(body, &self.mast_forest)
            .into_diagnostic()
            .wrap_err("assembler failed to add new loop node")?;
        self.ensure_node(loop_node)
    }

    /// Adds a call node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_call(&mut self, callee: MastNodeId) -> Result<MastNodeId, Report> {
        let call = MastNode::new_call(callee, &self.mast_forest)
            .into_diagnostic()
            .wrap_err("assembler failed to add new call node")?;
        self.ensure_node(call)
    }

    /// Adds a syscall node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_syscall(&mut self, callee: MastNodeId) -> Result<MastNodeId, Report> {
        let syscall = MastNode::new_syscall(callee, &self.mast_forest)
            .into_diagnostic()
            .wrap_err("assembler failed to add new syscall node")?;
        self.ensure_node(syscall)
    }

    /// Adds a dyn node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_dyn(&mut self) -> Result<MastNodeId, Report> {
        self.ensure_node(MastNode::new_dyn())
    }

    /// Adds a dyncall node to the forest, and returns the [`MastNodeId`] associated with it.
    pub fn ensure_dyncall(&mut self) -> Result<MastNodeId, Report> {
        self.ensure_node(MastNode::new_dyncall())
    }

    /// Adds a node corresponding to the given MAST root, according to how it is linked.
    ///
    /// * If statically-linked, then the entire subtree is copied, and the MastNodeId of the root of
    ///   the inserted subtree is returned.
    /// * If dynamically-linked, then an external node is inserted, and its MastNodeId is returned
    pub fn ensure_external_link(&mut self, mast_root: Word) -> Result<MastNodeId, Report> {
        if let Some(root_id) = self.statically_linked_mast.find_procedure_root(mast_root) {
            for old_id in SubtreeIterator::new(&root_id, &self.statically_linked_mast.clone()) {
                let node = self.statically_linked_mast[old_id]
                    .remap_children(&self.statically_linked_mast_remapping);
                let new_id = self.ensure_node(node)?;
                self.statically_linked_mast_remapping.insert(old_id, new_id);
            }
            Ok(root_id.remap(&self.statically_linked_mast_remapping))
        } else {
            self.ensure_node(MastNode::new_external(mast_root))
        }
    }

    /// Adds a list of decorators to the provided node to be executed before the node executes.
    ///
    /// If other decorators are already present, the new decorators are added to the end of the
    /// list.
    pub fn append_before_enter(&mut self, node_id: MastNodeId, decorator_ids: &[DecoratorId]) {
        self.mast_forest[node_id].append_before_enter(decorator_ids);

        let new_node_fingerprint = self.fingerprint_for_node(&self[node_id]);
        self.hash_by_node_id.insert(node_id, new_node_fingerprint);
    }

    pub fn append_after_exit(&mut self, node_id: MastNodeId, decorator_ids: &[DecoratorId]) {
        self.mast_forest[node_id].append_after_exit(decorator_ids);

        let new_node_fingerprint = self.fingerprint_for_node(&self[node_id]);
        self.hash_by_node_id.insert(node_id, new_node_fingerprint);
    }
}

impl MastForestBuilder {
    fn fingerprint_for_node(&self, node: &MastNode) -> MastNodeFingerprint {
        MastNodeFingerprint::from_mast_node(&self.mast_forest, &self.hash_by_node_id, node)
            .expect("hash_by_node_id should contain the fingerprints of all children of `node`")
    }

    /// Registers an error message in the MAST Forest and returns the
    /// corresponding error code as a Felt.
    pub fn register_error(&mut self, msg: Arc<str>) -> Felt {
        self.mast_forest.register_error(msg)
    }
}

impl Index<MastNodeId> for MastForestBuilder {
    type Output = MastNode;

    #[inline(always)]
    fn index(&self, node_id: MastNodeId) -> &Self::Output {
        &self.mast_forest[node_id]
    }
}

impl Index<DecoratorId> for MastForestBuilder {
    type Output = Decorator;

    #[inline(always)]
    fn index(&self, decorator_id: DecoratorId) -> &Self::Output {
        &self.mast_forest[decorator_id]
    }
}

impl IndexMut<DecoratorId> for MastForestBuilder {
    #[inline(always)]
    fn index_mut(&mut self, decorator_id: DecoratorId) -> &mut Self::Output {
        &mut self.mast_forest[decorator_id]
    }
}

// ------------------------------------------------------------------------------------------------

impl MastForestBuilder {
    /// Merges an AdviceMap into the one being built within the MAST Forest.
    ///
    /// # Errors
    ///
    /// Returns `AdviceMapKeyCollisionOnMerge` if any of the keys of the AdviceMap being merged
    /// are already present with a different value in the AdviceMap of the Mast Forest. In
    /// case of error the AdviceMap of the Mast Forest remains unchanged.
    pub fn merge_advice_map(&mut self, other: &AdviceMap) -> Result<(), Report> {
        self.mast_forest
            .advice_map_mut()
            .merge_advice_map(other)
            .map_err(|((key, prev_values), new_values)| LinkerError::AdviceMapKeyAlreadyPresent {
                key: key.into(),
                prev_values,
                new_values,
            })
            .into_diagnostic()
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Determines if we want to merge a block with other blocks. Currently, this works as follows:
/// - If the block is a procedure, we merge it only if the number of operation batches is smaller
///   then the threshold (currently set at 32). The reasoning is based on an estimate of the the
///   runtime penalty of not inlining the procedure. We assume that this penalty is roughly 3 extra
///   nodes in the MAST and so would require 3 additional hashes at runtime. Since hashing each
///   operation batch requires 1 hash, this basically implies that if the runtime penalty is more
///   than 10%, we inline the block, but if it is less than 10% we accept the penalty to make
///   deserialization faster.
/// - If the block is not a procedure, we always merge it because: (1) if it is a large block, it is
///   likely to be unique and, thus, the original block will be orphaned and removed later; (2) if
///   it is a small block, there is a large run-time benefit for inlining it.
fn should_merge(is_procedure: bool, num_op_batches: usize) -> bool {
    if is_procedure {
        num_op_batches < PROCEDURE_INLINING_THRESHOLD
    } else {
        true
    }
}
