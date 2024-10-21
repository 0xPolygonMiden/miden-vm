use alloc::{collections::BTreeMap, vec::Vec};
use core::ops::ControlFlow;

use miden_crypto::hash::{blake::Blake3Digest, rpo::RpoDigest};

use crate::mast::{DecoratorId, EqHash, MastForest, MastForestError, MastNode, MastNodeId};

#[cfg(test)]
mod tests;

/// A type that allows merging [`MastForest`]s.
///
/// This functionality is exposed via [`MastForest::merge`]. See its documentation for more details.
pub(crate) struct MastForestMerger {
    mast_forest: MastForest,
    // Internal indices needed for efficient duplicate checking and EqHash computation.
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
    pub(crate) fn merge(
        &mut self,
        other_forest: &MastForest,
    ) -> Result<MastForestRootMap, MastForestError> {
        let mut decorator_id_remapping = DecoratorIdMap::new(other_forest.decorators.len());
        let mut node_id_remapping = MastForestNodeIdMap::new();

        self.merge_decorators(other_forest, &mut decorator_id_remapping)?;
        self.merge_nodes(other_forest, &decorator_id_remapping, &mut node_id_remapping)?;
        self.merge_roots(other_forest, &node_id_remapping)?;

        let root_map =
            MastForestRootMap::from_node_id_map(node_id_remapping, other_forest.roots.as_slice());

        Ok(root_map)
    }

    fn merge_decorators(
        &mut self,
        other_forest: &MastForest,
        decorator_id_remapping: &mut DecoratorIdMap,
    ) -> Result<(), MastForestError> {
        for (merging_id, merging_decorator) in other_forest.decorators.iter().enumerate() {
            let merging_decorator_hash = merging_decorator.eq_hash();
            let new_decorator_id = if let Some(existing_decorator) =
                self.decorators_by_hash.get(&merging_decorator_hash)
            {
                *existing_decorator
            } else {
                let new_decorator_id = self.mast_forest.add_decorator(merging_decorator.clone())?;
                self.decorators_by_hash.insert(merging_decorator_hash, new_decorator_id);
                new_decorator_id
            };

            let merging_id = DecoratorId::from_u32_safe(merging_id as u32, other_forest)
                .expect("the index should always be less than the number of decorators");
            decorator_id_remapping.insert(merging_id, new_decorator_id);
        }

        Ok(())
    }

    fn merge_nodes(
        &mut self,
        other_forest: &MastForest,
        decorator_id_remapping: &DecoratorIdMap,
        node_id_remapping: &mut MastForestNodeIdMap,
    ) -> Result<(), MastForestError> {
        for (merging_id, node) in other_forest.iter_nodes() {
            // We need to remap the node prior to computing the EqHash.
            //
            // This is because the EqHash computation looks up its descendants and decorators in
            // the internal index, and if we were to pass the original node to that
            // computation, it would look up the incorrect descendants and decorators (since the
            // descendant's indices may have changed).
            //
            // Remapping at this point is guaranteed to be "complete", meaning all ids of children
            // will be present in `node_id_remapping` since the DFS iteration guarantees
            // that all children of this `node` have been processed before this node and
            // their indices have been added to the mappings.
            let remapped_node = self.remap_node(node, decorator_id_remapping, node_id_remapping)?;

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
        node_id_remapping: &MastForestNodeIdMap,
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
        node_id_remapping: &mut MastForestNodeIdMap,
        node_eq: EqHash,
    ) -> Result<(), MastForestError> {
        let new_node_id = self.mast_forest.add_node(node)?;
        node_id_remapping.insert(previous_id, new_node_id);

        // We need to update the indices with the newly inserted nodes
        // since the EqHash computation requires all descendants of a node
        // to be in this index. Hence when we encounter a node in the merging forest
        // which has descendants (Call, Loop, Split, ...), then their descendants need to be in the
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
    /// - The existing forest contains a node (external or non-external) with MAST root `foo` and
    ///   the merging External node refers to `foo`. In this case, the merging node will be mapped
    ///   to the existing node and dropped.
    /// - The existing forest contains an External nodes with a MAST root `foo` and the non-external
    ///   merging node's digest is `foo`. In this case, the existing external node will be replaced
    ///   by the merging node.
    ///
    /// Returns whether the caller should continue in their code path for this node or skip it.
    fn merge_external_nodes(
        &mut self,
        previous_id: MastNodeId,
        node_eq: &EqHash,
        remapped_node: &MastNode,
        node_id_remapping: &mut MastForestNodeIdMap,
    ) -> Result<ControlFlow<()>, MastForestError> {
        if remapped_node.is_external() {
            match self.lookup_node_by_root(&node_eq.mast_root) {
                // If there already is any node with the same MAST root, map the merging external
                // node to that existing one.
                // This code path is also entered if the fingerprints match, so we can skip the
                // general merging case by returning `Break`.
                Some((_, existing_external_node_id)) => {
                    node_id_remapping.insert(previous_id, *existing_external_node_id);
                    Ok(ControlFlow::Break(()))
                },
                // If no duplicate for the external node exists do nothing as `merge_nodes`
                // will simply add the node to the forest.
                None => Ok(ControlFlow::Continue(())),
            }
        } else {
            // Replace an external node in self with the given MAST root with the non-external
            // node from the merging forest.
            // Any node in the existing forest that pointed to the external node will
            // have the same MAST root due to the semantics of external nodes.
            match self.lookup_external_node_by_root(node_eq) {
                Some((_, external_node_id)) => {
                    self.mast_forest[external_node_id] = remapped_node.clone();
                    node_id_remapping.insert(previous_id, external_node_id);
                    // The other branch of this function guarantees that no external and
                    // non-external node with the same MAST root exist in the
                    // merged forest, so if we found an external node with a
                    // given MAST root, it must be the only one in the merged
                    // forest, so we can skip the remainder of the `merge_nodes` code path.
                    Ok(ControlFlow::Break(()))
                },
                // If we did not find a matching node, we can continue in the `merge_nodes` code
                // path.
                None => Ok(ControlFlow::Continue(())),
            }
        }
    }

    /// Remaps a nodes' potentially contained children and decorators to their new IDs according to
    /// the given maps.
    fn remap_node(
        &self,
        node: &MastNode,
        decorator_id_remapping: &DecoratorIdMap,
        node_id_remapping: &MastForestNodeIdMap,
    ) -> Result<MastNode, MastForestError> {
        let map_decorator_id = |decorator_id: &DecoratorId| {
            decorator_id_remapping.get(decorator_id).ok_or_else(|| {
                MastForestError::DecoratorIdOverflow(*decorator_id, decorator_id_remapping.len())
            })
        };
        let map_decorators = |decorators: &[DecoratorId]| -> Result<Vec<_>, MastForestError> {
            decorators.iter().map(map_decorator_id).collect()
        };

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
            MastNode::Block(basic_block_node) => {
                MastNode::new_basic_block(
                    basic_block_node.operations().copied().collect(),
                    // Operation Indices of decorators stay the same while decorator IDs need to be
                    // mapped.
                    Some(
                        basic_block_node
                            .decorators()
                            .iter()
                            .map(|(idx, decorator_id)| match map_decorator_id(decorator_id) {
                                Ok(mapped_decorator) => Ok((*idx, mapped_decorator)),
                                Err(err) => Err(err),
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                )
                .expect("previously valid BasicBlockNode should still be valid")
            },
            MastNode::Dyn(_) => MastNode::new_dyn(),
            MastNode::External(external_node) => MastNode::new_external(external_node.digest()),
        };

        // Decorators must be handled specially for basic block nodes.
        // For other node types we can handle it centrally.
        if !mapped_node.is_basic_block() {
            mapped_node.set_before_enter(map_decorators(node.before_enter())?);
            mapped_node.set_after_exit(map_decorators(node.after_exit())?);
        }

        Ok(mapped_node)
    }

    // HELPERS
    // ================================================================================================

    fn lookup_node_by_fingerprint(&self, eq_hash: &EqHash) -> Option<&(EqHash, MastNodeId)> {
        self.node_id_by_hash.get(&eq_hash.mast_root).and_then(|node_ids| {
            node_ids.iter().find(|(node_fingerprint, _)| node_fingerprint == eq_hash)
        })
    }

    fn lookup_node_by_root(&self, mast_root: &RpoDigest) -> Option<&(EqHash, MastNodeId)> {
        self.node_id_by_hash.get(mast_root).and_then(|node_ids| node_ids.first())
    }

    fn lookup_external_node_by_root(&self, fingerprint: &EqHash) -> Option<(EqHash, MastNodeId)> {
        self.node_id_by_hash.get(&fingerprint.mast_root).and_then(|ids| {
            let mut iterator = ids
                .iter()
                .filter(|(_, node_id)| self.mast_forest[*node_id].is_external())
                .copied();
            let external_node = iterator.next();
            // The merging implementation should guarantee that no two external nodes with the same
            // MAST root exist.
            debug_assert!(iterator.next().is_none());
            external_node
        })
    }
}

impl From<MastForestMerger> for MastForest {
    fn from(merger: MastForestMerger) -> Self {
        merger.mast_forest
    }
}

// MAST FOREST ROOT MAP
// ================================================================================================

/// A mapping for the new location of the roots of a [`MastForest`] after a merge.
///
/// It maps the roots ([`MastNodeId`]s) of a forest to their new [`MastNodeId`] in the merged
/// forest. See [`MastForest::merge`] for more details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MastForestRootMap {
    root_map: BTreeMap<MastNodeId, MastNodeId>,
}

impl MastForestRootMap {
    fn from_node_id_map(id_map: MastForestNodeIdMap, roots: &[MastNodeId]) -> Self {
        let mut root_map = BTreeMap::new();

        for root in roots {
            let new_id =
                id_map.get(root).copied().expect("every node id should be mapped to its new id");
            root_map.insert(*root, new_id);
        }

        Self { root_map }
    }

    /// Maps the given root to its new location in the merged forest, if such a mapping exists.
    ///
    /// It is guaranteed that every root of the map's corresponding forest is contained in the map.
    pub fn map_root(&self, root: &MastNodeId) -> Option<MastNodeId> {
        self.root_map.get(root).copied()
    }
}

// DECORATOR ID MAP
// ================================================================================================

/// A specialized map from [`DecoratorId`] -> [`DecoratorId`].
///
/// When mapping Decorator IDs during merging, we always map all IDs of the merging
/// forest to new ids. Hence it is more efficient to use a `Vec` instead of, say, a `BTreeMap`.
///
/// In other words, this type is similar to `BTreeMap<ID, ID>` but takes advantage of the fact that
/// the keys are contiguous.
///
/// This type is meant to encapsulates some guarantees:
///
/// - Indexing into the vector for any ID is safe if that ID is valid for the corresponding forest,
///   which is enforced in the `from_u32_safe` functions (as long as they are used with the correct
///   forest). Despite that, we still cannot index unconditionally in case node with invalid
///   [`DecoratorId`]s is passed to `merge`.
/// - The entry itself can be either None or Some. However:
///   - For `DecoratorId`s we iterate and insert all decorators into this map before retrieving any
///     entry, so all entries contain `Some`. Because of this, we can use `expect` in `get` for the
///     `Option` value.
/// - Similarly, inserting any ID from the corresponding forest is safe as the map contains a
///   pre-allocated `Vec` of the appropriate size.
struct DecoratorIdMap {
    inner: Vec<Option<DecoratorId>>,
}

impl DecoratorIdMap {
    fn new(num_ids: usize) -> Self {
        Self { inner: vec![None; num_ids] }
    }

    /// Maps the given key to the given value.
    ///
    /// It is the caller's responsibility to only pass keys that belong to the forest for which this
    /// map was originally created.
    fn insert(&mut self, key: DecoratorId, value: DecoratorId) {
        self.inner[key.as_usize()] = Some(value);
    }

    /// Retrieves the value for the given key.
    fn get(&self, key: &DecoratorId) -> Option<DecoratorId> {
        self.inner
            .get(key.as_usize())
            .map(|id| id.expect("every id should have a Some entry in the map when calling get"))
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// A type definition for increased readability in function signatures.
type MastForestNodeIdMap = BTreeMap<MastNodeId, MastNodeId>;
