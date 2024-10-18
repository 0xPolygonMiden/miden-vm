use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::ControlFlow;

use miden_crypto::hash::{blake::Blake3Digest, rpo::RpoDigest};

use crate::mast::{DecoratorId, EqHash, MastForest, MastForestError, MastNode, MastNodeId};

#[cfg(test)]
mod tests;

/// A type that allows merging [`MastForest`]s.
///
/// Merging two forests means combining all their constituent parts, i.e. [`MastNode`]s,
/// [`Decorator`](crate::mast::Decorator)s and roots. During this process, any duplicate or
/// unreachable nodes are removed. Additionally, [`MastNodeId`]s of nodes as well as
/// [`DecoratorId`]s of decorators may change and references to them are remapped to their new
/// location.
///
/// For example, consider this representation of a forest's nodes and all of these nodes being
/// roots:
///
/// ```text
/// [Block(foo), Block(bar)]
/// ```
///
/// If we merge another forest into it:
///
/// ```text
/// [Block(bar), Call(0)]
/// ```
///
/// then we would expect this forest:
///
/// ```text
/// [Block(foo), Block(bar), Call(1)]
/// ```
///
/// - The `Call` to the `bar` block was remapped to its new index (now 1, previously 0).
/// - The `Block(bar)` was deduplicated any only exists once in the merged forest.
///
/// If any forest being merged contains an `External(qux)` node and another forest contains a node
/// whose digest is `qux`, then the external node will be replaced with the `qux` node, which is
/// effectively deduplication.
///
/// Note that there are convenience methods for merging on [`MastForest`] itself:
/// - [`MastForest::merge`]
/// - [`MastForest::merge_multiple`]
pub(crate) struct MastForestMerger {
    mast_forest: MastForest,
    // Internal indices needed for efficient duplicate checking.
    node_id_by_hash: BTreeMap<RpoDigest, Vec<(EqHash, MastNodeId)>>,
    hash_by_node_id: BTreeMap<MastNodeId, EqHash>,
    decorators_by_hash: BTreeMap<Blake3Digest<32>, DecoratorId>,
}

impl MastForestMerger {
    /// Creates a new merger which creates a new internal, empty forest into which other
    /// [`MastForest`]s are merged.
    pub(crate) fn new() -> Self {
        Self {
            node_id_by_hash: BTreeMap::new(),
            hash_by_node_id: BTreeMap::new(),
            decorators_by_hash: BTreeMap::new(),
            mast_forest: MastForest::new(),
        }
    }

    /// Merges `other_forest` into the forest contained in self.
    pub(crate) fn merge(&mut self, other_forest: &MastForest) -> Result<(), MastForestError> {
        let mut decorator_id_remapping = ForestIdMap::new(other_forest.decorators.len());
        let mut node_id_remapping = MastForestIdMap::new();

        self.merge_decorators(other_forest, &mut decorator_id_remapping)?;
        self.merge_nodes(other_forest, &decorator_id_remapping, &mut node_id_remapping)?;
        self.merge_roots(other_forest, &node_id_remapping)?;

        Ok(())
    }

    fn merge_decorators(
        &mut self,
        other_forest: &MastForest,
        decorator_id_remapping: &mut ForestIdMap<DecoratorId>,
    ) -> Result<(), MastForestError> {
        for (merging_id, merging_decorator) in other_forest.decorators.iter().enumerate() {
            let merging_decorator_hash = merging_decorator.eq_hash();
            let new_decorator_id = if let Some(existing_decorator) =
                self.decorators_by_hash.get(&merging_decorator_hash)
            {
                *existing_decorator
            } else {
                self.mast_forest.add_decorator(merging_decorator.clone())?
            };

            let merging_id = DecoratorId::from_u32_safe(merging_id as u32, other_forest)
                .expect("the index should always be less than the number of decorators");
            decorator_id_remapping.insert(merging_id, new_decorator_id);
            self.decorators_by_hash.insert(merging_decorator_hash, new_decorator_id);
        }

        Ok(())
    }

    fn merge_nodes(
        &mut self,
        other_forest: &MastForest,
        decorator_id_remapping: &ForestIdMap<DecoratorId>,
        node_id_remapping: &mut MastForestIdMap,
    ) -> Result<(), MastForestError> {
        for (merging_id, node) in other_forest.iter_nodes() {
            // We need to remap the node prior to computing the EqHash.
            //
            // This is because the EqHash computation looks up its descendants and decorators in
            // the internal index, and if we were to pass the original node to that
            // computation, it would look up the incorrect descendants and decorators.
            //
            // Remapping at this point is going to be "complete", meaning all ids of children will
            // be remapped since the DFS iteration guarantees that all children of this `node` have
            // been processed before this node and their indices have been added to the
            // mappings.
            let remapped_node = self.remap_node(node, decorator_id_remapping, node_id_remapping);

            let node_eq =
                EqHash::from_mast_node(&self.mast_forest, &self.hash_by_node_id, &remapped_node);

            match self.merge_external_nodes(
                merging_id,
                &node_eq,
                &remapped_node,
                node_id_remapping,
            )? {
                // Continue is interpreted as doing nothing.
                ControlFlow::Continue(_) => (),
                // Break is interpreted as continue in the loop sense.
                ControlFlow::Break(_) => continue,
            }

            // If an external node was previously replaced by the remapped node, this will detect
            // them as duplicates here if their fingerprints match exactly and add the appropriate
            // mapping from the merging id to the existing id.
            match self.lookup_node_by_fingerprint(&node_eq) {
                Some((_, existing_node_id)) => {
                    // We have to map any occurence of `merging_id` to `existing_node_id`.
                    node_id_remapping.insert(merging_id, *existing_node_id);
                },
                None => {
                    self.add_merged_node(merging_id, remapped_node, node_id_remapping, node_eq)?;
                },
            }
        }

        Ok(())
    }

    fn merge_roots(
        &mut self,
        other_forest: &MastForest,
        node_id_remapping: &MastForestIdMap,
    ) -> Result<(), MastForestError> {
        for root_id in other_forest.roots.iter() {
            // Map the previous root to its possibly new id.
            let new_root =
                node_id_remapping.get(root_id).expect("all node ids should have an entry");
            // This will take O(n) every time to check if the root already exists.
            // We could improve this by keeping a BTreeSet<MastNodeId> of existing roots during
            // merging for a faster check.
            self.mast_forest.make_root(*new_root);
        }

        Ok(())
    }

    fn add_merged_node(
        &mut self,
        previous_id: MastNodeId,
        node: MastNode,
        node_id_remapping: &mut MastForestIdMap,
        node_eq: EqHash,
    ) -> Result<(), MastForestError> {
        let new_node_id = self.mast_forest.add_node(node)?;
        node_id_remapping.insert(previous_id, new_node_id);

        // We need to update the indices with the newly inserted nodes
        // since the EqHash computation requires all descendants of a node
        // to be in this index. Hence when we encounter a node in the merging forest
        // which has descendants (Call, Loop, Split, ...), then those need to be in the
        // indices.
        self.node_id_by_hash
            .entry(node_eq.mast_root)
            .and_modify(|node_ids| node_ids.push((node_eq, new_node_id)))
            .or_insert_with(|| vec![(node_eq, new_node_id)]);

        self.hash_by_node_id.insert(new_node_id, node_eq);

        Ok(())
    }

    /// This will handle two cases:
    ///
    /// - The existing forest contains a node with MAST root `foo` and the merging External node
    ///   refers to `foo` and their fingerprints do not match. If their fingerprints match this is
    ///   the general case of merging (or deduplicating to be precise) and is handled in
    ///   `merge_nodes` already, so it is not duplicated here.
    /// - The existing forest contains _one or more_ External nodes with a MAST root `foo` and the
    ///   merging node's digest is `foo`.
    ///
    /// Importantly, this does not handle the case where the fingerprints match as that is the
    /// general case of merging and is handled in `merge_nodes` already, so it is not duplicated
    /// here.
    fn merge_external_nodes(
        &mut self,
        previous_id: MastNodeId,
        node_eq: &EqHash,
        remapped_node: &MastNode,
        node_id_remapping: &mut MastForestIdMap,
    ) -> Result<ControlFlow<()>, MastForestError> {
        if remapped_node.is_external() {
            // If any non-external node exists, use it and drop the external node.
            match self.lookup_non_external_node_by_root(node_eq) {
                Some((_, referenced_node_id)) => {
                    node_id_remapping.insert(previous_id, *referenced_node_id);
                    Ok(ControlFlow::Break(()))
                },
                // If no replacement for the external node exists do nothing as `merge_nodes` will
                // simply add the node to the forest.
                None => Ok(ControlFlow::Continue(())),
            }
        } else {
            // Replace all external nodes in self with the given MAST root with the non-external
            // node from the merging forest.
            // Any node in the existing forest that pointed to the external node will
            // have the same MAST root due to the semantics of external nodes.
            //
            // By default we assume that no external node will be replaced in which case we want to
            // `Continue`, otherwis we `Break`.
            let mut control_flow = ControlFlow::Continue(());

            for (_, external_node_id) in self.lookup_all_external_nodes_by_root(node_eq).into_iter()
            {
                self.mast_forest[external_node_id] = remapped_node.clone();
                node_id_remapping.insert(previous_id, external_node_id);
                control_flow = ControlFlow::Break(());
            }

            Ok(control_flow)
        }
    }

    /// Remaps a nodes' potentially contained children and decorators to their new IDs according to
    /// the given maps.
    fn remap_node(
        &self,
        node: &MastNode,
        decorator_id_remapping: &ForestIdMap<DecoratorId>,
        node_id_remapping: &MastForestIdMap,
    ) -> MastNode {
        let map_decorator_id =
            |decorator_id: &DecoratorId| decorator_id_remapping.get(decorator_id);
        let map_decorators =
            |decorators: &[DecoratorId]| decorators.iter().map(map_decorator_id).collect();
        let map_node_id = |node_id: MastNodeId| {
            node_id_remapping
                .get(&node_id)
                .copied()
                .expect("every node id should have an entry")
        };

        // Due to DFS postorder iteration all children of node's should have been inserted before
        // their parents which is why we can `expect` the constructor calls here.
        let mut mapped_node = match node {
            MastNode::Join(join_node) => {
                let first = map_node_id(join_node.first());
                let second = map_node_id(join_node.second());

                MastNode::new_join(first, second, &self.mast_forest)
                    .expect("JoinNode children should have been mapped to a lower index")
            },
            MastNode::Split(split_node) => {
                let if_branch = map_node_id(split_node.on_true());
                let else_branch = map_node_id(split_node.on_false());

                MastNode::new_split(if_branch, else_branch, &self.mast_forest)
                    .expect("SplitNode children should have been mapped to a lower index")
            },
            MastNode::Loop(loop_node) => {
                let body = map_node_id(loop_node.body());
                MastNode::new_loop(body, &self.mast_forest)
                    .expect("LoopNode children should have been mapped to a lower index")
            },
            MastNode::Call(call_node) => {
                let callee = map_node_id(call_node.callee());
                MastNode::new_call(callee, &self.mast_forest)
                    .expect("CallNode children should have been mapped to a lower index")
            },
            // Other nodes are simply copied.
            MastNode::Block(basic_block_node) => MastNode::new_basic_block(
                basic_block_node.operations().copied().collect(),
                // Operation Indices of decorators stay the same while decorator IDs need to be
                // mapped.
                Some(
                    basic_block_node
                        .decorators()
                        .iter()
                        .map(|(idx, decorator_id)| (*idx, map_decorator_id(decorator_id)))
                        .collect(),
                ),
            )
            .expect("previously valid BasicBlockNode should still be valid"),
            MastNode::Dyn(_) => MastNode::new_dyn(),
            MastNode::External(external_node) => MastNode::new_external(external_node.digest()),
        };

        // Decorators must be handled specially for basic block nodes.
        // For other node types we can handle it centrally.
        if !mapped_node.is_basic_block() {
            mapped_node.set_before_enter(map_decorators(node.before_enter()));
            mapped_node.set_after_exit(map_decorators(node.after_exit()));
        }

        mapped_node
    }

    // HELPERS
    // ================================================================================================

    fn lookup_node_by_fingerprint(&self, eq_hash: &EqHash) -> Option<&(EqHash, MastNodeId)> {
        self.node_id_by_hash.get(&eq_hash.mast_root).and_then(|node_ids| {
            node_ids.iter().find(|(node_fingerprint, _)| node_fingerprint == eq_hash)
        })
    }

    fn lookup_non_external_node_by_root(
        &self,
        fingerprint: &EqHash,
    ) -> Option<&(EqHash, MastNodeId)> {
        self.node_id_by_hash.get(&fingerprint.mast_root).and_then(|node_ids| {
            node_ids.iter().find(|(node_fingerprint, node_id)| {
                !self.mast_forest[*node_id].is_external() && node_fingerprint != fingerprint
            })
        })
    }

    fn lookup_all_external_nodes_by_root(&self, fingerprint: &EqHash) -> Vec<(EqHash, MastNodeId)> {
        self.node_id_by_hash
            .get(&fingerprint.mast_root)
            .map(|ids| {
                ids.iter()
                    .filter(|(_, node_id)| self.mast_forest[*node_id].is_external())
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl From<MastForestMerger> for MastForest {
    fn from(merger: MastForestMerger) -> Self {
        merger.mast_forest
    }
}

// MAST FOREST ID MAP
// ================================================================================================

pub struct MastForestIdMap {
    map: BTreeMap<MastNodeId, MastNodeId>,
}

impl MastForestIdMap {
    pub(crate) fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    pub(crate) fn insert(&mut self, key: MastNodeId, value: MastNodeId) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &MastNodeId) -> Option<&MastNodeId> {
        self.map.get(key)
    }
}

// MAST FOREST ID MAP
// ================================================================================================

/// A specialized map from ID -> ID meant to be used with [`DecoratorId`] or [`MastNodeId`].
///
/// When mapping Decorator or Mast Node IDs during merging, we always map all IDs of the merging
/// forest to new ids. Hence it is more efficient to use a `Vec` instead of, say, a `BTreeMap`.
///
/// In other words, this type is similar to `BTreeMap<ID, ID>` but takes advantage of the fact that
/// the keys are contiguous.
///
/// This type is meant to encapsulates some guarantees:
///
/// - Indexing into the vector for any ID is safe if that ID is valid for the corresponding forest,
///   which is enforced in the `from_u32_safe` functions (as long as they are used with the correct
///   forest).
/// - The entry itself can be either None or Some. However:
///   - For `DecoratorId`s we process them before retrieving any entry, so all entries contain
///     `Some`.
///   - For `MastNodeId`s we only `get` those node IDs that we have previously visited due to the
///     guarantees of DFS iteration, which is why we always find a `Some` entry as well.
///   - Because of this, we can use `expect` in `get`.
/// - Similarly, inserting any ID is safe as the map contains a pre-allocated `Vec` of the
///   appropriate size.
struct ForestIdMap<T: ForestId> {
    inner: Vec<Option<T>>,
}

trait ForestId: Clone + Copy {
    fn as_usize(&self) -> usize;
}

impl ForestId for DecoratorId {
    fn as_usize(&self) -> usize {
        DecoratorId::as_usize(self)
    }
}

impl<T: ForestId> ForestIdMap<T> {
    fn new(num_ids: usize) -> Self {
        Self { inner: vec![None; num_ids] }
    }

    /// Maps the given key to the given value.
    ///
    /// It is the caller's responsibility to only pass keys that belong to the forest for which this
    /// map was originally created.
    fn insert(&mut self, key: T, value: T) {
        self.inner[key.as_usize()] = Some(value);
    }

    /// Retrieves the value for the given key.
    ///
    /// It is the caller's responsibility to only pass keys that belong to the forest for which this
    /// map was originally created.
    fn get(&self, key: &T) -> T {
        self.inner[key.as_usize()]
            .expect("every id should have a Some entry in the map when calling get")
    }
}
