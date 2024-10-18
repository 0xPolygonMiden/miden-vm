use alloc::{collections::BTreeMap, vec::Vec};

use miden_crypto::hash::blake::Blake3Digest;

use crate::mast::{
    DecoratorId, EqHash, MastForest, MastForestDfsIter, MastForestError, MastNode, MastNodeId,
};

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
    node_id_by_hash: BTreeMap<EqHash, MastNodeId>,
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
        let mut node_id_remapping = ForestIdMap::new(other_forest.nodes.len());

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
        node_id_remapping: &mut ForestIdMap<MastNodeId>,
    ) -> Result<(), MastForestError> {
        for (merging_id, node) in MastForestDfsIter::new(other_forest) {
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

            match self.node_id_by_hash.get(&node_eq) {
                Some(existing_entry) => {
                    // We have to map any occurence of `merging_id` to `existing_node_id`.
                    node_id_remapping.insert(merging_id, *existing_entry);

                    // Replace the external node in the existing forest with the non-external node
                    // from the merging forest.
                    // Any node in the existing forest that pointed to the external node will
                    // have the same digest due to the semantics of external nodes.
                    //
                    // Note that the inverse case is handled implicitly, that is, an external node
                    // in the merging forest that already exists as an external or non-external
                    // node in the existing forest will not be added to the merged forest because it
                    // will be handled like any other duplicate.
                    if self.mast_forest[*existing_entry].is_external()
                        && !remapped_node.is_external()
                    {
                        self.mast_forest[*existing_entry] = remapped_node;
                    }
                },
                None => {
                    let new_node_id = self.mast_forest.add_node(remapped_node)?;
                    node_id_remapping.insert(merging_id, new_node_id);

                    // We need to update the indices with the newly inserted nodes
                    // since the EqHash computation requires all descendants of a node
                    // to be in this index. Hence when we encounter a node in the merging forest
                    // which has descendants (Call, Loop, Split, ...), then those need to be in the
                    // indices.
                    self.node_id_by_hash.insert(node_eq, new_node_id);
                    self.hash_by_node_id.insert(new_node_id, node_eq);
                },
            }
        }

        Ok(())
    }

    fn merge_roots(
        &mut self,
        other_forest: &MastForest,
        node_id_remapping: &ForestIdMap<MastNodeId>,
    ) -> Result<(), MastForestError> {
        for root_id in other_forest.roots.iter() {
            // Map the previous root to its possibly new id.
            let new_root = node_id_remapping.get(root_id);
            // This will take O(n) every time to check if the root already exists.
            // We could improve this by keeping a BTreeSet<MastNodeId> of existing roots during
            // merging for a faster check.
            self.mast_forest.make_root(new_root);
        }

        Ok(())
    }

    /// Remaps a nodes' potentially contained children and decorators to their new IDs according to
    /// the given maps.
    fn remap_node(
        &self,
        node: &MastNode,
        decorator_id_remapping: &ForestIdMap<DecoratorId>,
        node_id_remapping: &ForestIdMap<MastNodeId>,
    ) -> MastNode {
        let map_decorator_id =
            |decorator_id: &DecoratorId| decorator_id_remapping.get(decorator_id);
        let map_decorators =
            |decorators: &[DecoratorId]| decorators.iter().map(map_decorator_id).collect();
        let map_node_id = |node_id: MastNodeId| node_id_remapping.get(&node_id);

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
}

impl From<MastForestMerger> for MastForest {
    fn from(merger: MastForestMerger) -> Self {
        merger.mast_forest
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

impl ForestId for MastNodeId {
    fn as_usize(&self) -> usize {
        MastNodeId::as_usize(self)
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

#[cfg(test)]
mod tests {
    use miden_crypto::{hash::rpo::RpoDigest, ONE};

    use super::*;
    use crate::{Decorator, Operation};

    fn block_foo() -> MastNode {
        MastNode::new_basic_block(vec![Operation::Mul, Operation::Add], None).unwrap()
    }

    fn block_bar() -> MastNode {
        MastNode::new_basic_block(vec![Operation::And, Operation::Eq], None).unwrap()
    }

    fn block_qux() -> MastNode {
        MastNode::new_basic_block(vec![Operation::Swap, Operation::Push(ONE)], None).unwrap()
    }

    fn assert_contains_node_once(forest: &MastForest, digest: RpoDigest) {
        assert_eq!(forest.nodes.iter().filter(|node| node.digest() == digest).count(), 1);
    }

    /// Tests that Call(bar) still correctly calls the remapped bar block.
    ///
    /// [Block(foo), Call(foo)]
    /// +
    /// [Block(bar), Call(bar)]
    /// =
    /// [Block(foo), Call(foo), Block(bar), Call(bar)]
    #[test]
    fn mast_forest_merge_remap() {
        let mut forest_a = MastForest::new();
        let id_foo = forest_a.add_node(block_foo()).unwrap();
        let id_call_a = forest_a.add_call(id_foo).unwrap();
        forest_a.make_root(id_call_a);

        let mut forest_b = MastForest::new();
        let id_bar = forest_b.add_node(block_bar()).unwrap();
        let id_call_b = forest_b.add_call(id_bar).unwrap();
        forest_b.make_root(id_call_b);

        let merged = forest_a.merge(&forest_b).unwrap();

        assert_eq!(merged.nodes().len(), 4);
        assert_eq!(merged.nodes()[0], block_foo());
        assert_matches!(&merged.nodes()[1], MastNode::Call(call_node) if call_node.callee().as_u32() == 0);
        assert_eq!(merged.nodes()[2], block_bar());
        assert_matches!(&merged.nodes()[3], MastNode::Call(call_node) if call_node.callee().as_u32() == 2);
    }

    /// Tests that Forest_A + Forest_A = Forest_A (i.e. duplicates are removed).
    #[test]
    fn mast_forest_merge_duplicate() {
        let mut forest_a = MastForest::new();
        forest_a.add_decorator(Decorator::Debug(crate::DebugOptions::MemAll)).unwrap();
        forest_a.add_decorator(Decorator::Trace(25)).unwrap();

        let id_external = forest_a.add_external(block_bar().digest()).unwrap();
        let id_foo = forest_a.add_node(block_foo()).unwrap();
        let id_call = forest_a.add_call(id_foo).unwrap();
        let id_loop = forest_a.add_loop(id_external).unwrap();
        forest_a.make_root(id_call);
        forest_a.make_root(id_loop);

        let merged = forest_a.merge(&forest_a).unwrap();

        for merged_root in merged.procedure_digests() {
            forest_a.procedure_digests().find(|root| root == &merged_root).unwrap();
        }

        for merged_node in merged.nodes().iter().map(MastNode::digest) {
            forest_a.nodes.iter().find(|node| node.digest() == merged_node).unwrap();
        }

        for merged_decorator in merged.decorators.iter() {
            assert!(forest_a.decorators.contains(merged_decorator));
        }
    }

    /// Tests that External(foo) is replaced by Block(foo) whether it is in forest A or B, and the
    /// duplicate Call is removed.
    ///
    /// [External(foo), Call(foo)]
    /// +
    /// [Block(foo), Call(foo)]
    /// =
    /// [Block(foo), Call(foo)]
    /// +
    /// [External(foo), Call(foo)]
    /// =
    /// [Block(foo), Call(foo)]
    #[test]
    fn mast_forest_merge_replace_external() {
        let mut forest_a = MastForest::new();
        let id_foo_a = forest_a.add_external(block_foo().digest()).unwrap();
        let id_call_a = forest_a.add_call(id_foo_a).unwrap();
        forest_a.make_root(id_call_a);

        let mut forest_b = MastForest::new();
        let id_foo_b = forest_b.add_node(block_foo()).unwrap();
        let id_call_b = forest_b.add_call(id_foo_b).unwrap();
        forest_b.make_root(id_call_b);

        let merged_ab = forest_a.merge(&forest_b).unwrap();
        let merged_ba = forest_b.merge(&forest_a).unwrap();

        for merged in [merged_ab, merged_ba] {
            assert_eq!(merged.nodes().len(), 2);
            assert_eq!(merged.nodes()[0], block_foo());
            assert_matches!(&merged.nodes()[1], MastNode::Call(call_node) if call_node.callee().as_u32() == 0);
        }
    }

    /// Test that roots are preserved and deduplicated if appropriate.
    ///
    /// Nodes: [Block(foo), Call(foo)]
    /// Roots: [Call(foo)]
    /// +
    /// Nodes: [Block(foo), Block(bar), Call(foo)]
    /// Roots: [Block(bar), Call(foo)]
    /// =
    /// Nodes: [Block(foo), Block(bar), Call(foo)]
    /// Roots: [Block(bar), Call(foo)]
    #[test]
    fn mast_forest_merge_roots() {
        let mut forest_a = MastForest::new();
        let id_foo_a = forest_a.add_node(block_foo()).unwrap();
        let call_a = forest_a.add_call(id_foo_a).unwrap();
        forest_a.make_root(call_a);

        let mut forest_b = MastForest::new();
        let id_foo_b = forest_b.add_node(block_foo()).unwrap();
        let id_bar_b = forest_b.add_node(block_bar()).unwrap();
        let call_b = forest_b.add_call(id_foo_b).unwrap();
        forest_b.make_root(id_bar_b);
        forest_b.make_root(call_b);

        let root_digest_call_a = forest_a.get_node_by_id(call_a).unwrap().digest();
        let root_digest_bar_b = forest_b.get_node_by_id(id_bar_b).unwrap().digest();
        let root_digest_call_b = forest_b.get_node_by_id(call_b).unwrap().digest();

        let merged = forest_a.merge(&forest_b).unwrap();

        // Asserts (together with the other assertions) that the duplicate Call(foo) roots have been
        // deduplicated.
        assert_eq!(merged.procedure_roots().len(), 2);

        // Assert that all root digests from A an B are still roots in the merged forest.
        let root_digests = merged.procedure_digests().collect::<Vec<_>>();
        assert!(root_digests.contains(&root_digest_call_a));
        assert!(root_digests.contains(&root_digest_bar_b));
        assert!(root_digests.contains(&root_digest_call_b));
    }

    /// Test that multiple trees can be merged when the same merger is reused.
    ///
    /// Nodes: [Block(foo), Call(foo)]
    /// Roots: [Call(foo)]
    /// +
    /// Nodes: [Block(foo), Block(bar), Call(foo)]
    /// Roots: [Block(bar), Call(foo)]
    /// +
    /// Nodes: [Block(foo), Block(qux), Call(foo)]
    /// Roots: [Block(qux), Call(foo)]
    /// =
    /// Nodes: [Block(foo), Block(bar), Block(qux), Call(foo)]
    /// Roots: [Block(bar), Block(qux), Call(foo)]
    #[test]
    fn mast_forest_merge_multiple() {
        let mut forest_a = MastForest::new();
        let id_foo_a = forest_a.add_node(block_foo()).unwrap();
        let call_a = forest_a.add_call(id_foo_a).unwrap();
        forest_a.make_root(call_a);

        let mut forest_b = MastForest::new();
        let id_foo_b = forest_b.add_node(block_foo()).unwrap();
        let id_bar_b = forest_b.add_node(block_bar()).unwrap();
        let call_b = forest_b.add_call(id_foo_b).unwrap();
        forest_b.make_root(id_bar_b);
        forest_b.make_root(call_b);

        let mut forest_c = MastForest::new();
        let id_foo_c = forest_c.add_node(block_foo()).unwrap();
        let id_qux_c = forest_c.add_node(block_qux()).unwrap();
        let call_c = forest_c.add_call(id_foo_c).unwrap();
        forest_c.make_root(id_qux_c);
        forest_c.make_root(call_c);

        let merged = forest_a.merge_multiple(&[&forest_b, &forest_c]).unwrap();

        let block_foo_digest = forest_b.get_node_by_id(id_foo_b).unwrap().digest();
        let block_bar_digest = forest_b.get_node_by_id(id_bar_b).unwrap().digest();
        let call_foo_digest = forest_b.get_node_by_id(call_b).unwrap().digest();
        let block_qux_digest = forest_c.get_node_by_id(id_qux_c).unwrap().digest();

        assert_eq!(merged.procedure_roots().len(), 3);

        let root_digests = merged.procedure_digests().collect::<Vec<_>>();
        assert!(root_digests.contains(&call_foo_digest));
        assert!(root_digests.contains(&block_bar_digest));
        assert!(root_digests.contains(&block_qux_digest));

        assert_contains_node_once(&merged, block_foo_digest);
        assert_contains_node_once(&merged, block_bar_digest);
        assert_contains_node_once(&merged, block_qux_digest);
        assert_contains_node_once(&merged, call_foo_digest);
    }

    /// Tests that decorators are merged and that nodes who are identical except for their
    /// decorators are not deduplicated.
    ///
    /// Note in particular that the `Loop` nodes only differ in their decorator which ensures that
    /// the merging takes decorators into account.
    ///
    /// Nodes: [Block(foo, [Trace(1), Trace(2)]), Loop(foo, [Trace(0), Trace(2)])]
    /// Decorators: [Trace(0), Trace(1), Trace(2)]
    /// +
    /// Nodes: [Block(foo, [Trace(1), Trace(2)]), Loop(foo, [Trace(1), Trace(3)])]
    /// Decorators: [Trace(1), Trace(2), Trace(3)]
    /// =
    /// Nodes: [
    ///   Block(foo, [Trace(1), Trace(2)]),
    ///   Loop(foo, [Trace(0), Trace(2)]),
    ///   Loop(foo, [Trace(1), Trace(3)]),
    /// ]
    /// Decorators: [Trace(0), Trace(1), Trace(2), Trace(3)]
    #[test]
    fn mast_forest_merge_decorators() {
        let mut forest_a = MastForest::new();
        let trace0 = Decorator::Trace(0);
        let trace1 = Decorator::Trace(1);
        let trace2 = Decorator::Trace(2);
        let trace3 = Decorator::Trace(3);

        // Build Forest A
        let deco0_a = forest_a.add_decorator(trace0.clone()).unwrap();
        let deco1_a = forest_a.add_decorator(trace1.clone()).unwrap();
        let deco2_a = forest_a.add_decorator(trace2.clone()).unwrap();

        let mut foo_node_a = block_foo();
        foo_node_a.set_before_enter(vec![deco1_a, deco2_a]);
        let id_foo_a = forest_a.add_node(foo_node_a).unwrap();

        let mut loop_node_a = MastNode::new_loop(id_foo_a, &forest_a).unwrap();
        loop_node_a.set_after_exit(vec![deco0_a, deco2_a]);
        let id_loop_a = forest_a.add_node(loop_node_a).unwrap();

        forest_a.make_root(id_loop_a);

        // Build Forest B
        let mut forest_b = MastForest::new();
        let deco1_b = forest_b.add_decorator(trace1.clone()).unwrap();
        let deco2_b = forest_b.add_decorator(trace2.clone()).unwrap();
        let deco3_b = forest_b.add_decorator(trace3.clone()).unwrap();

        // This foo node is identical to the one in A, including its decorators.
        let mut foo_node_b = block_foo();
        foo_node_b.set_before_enter(vec![deco1_b, deco2_b]);
        let id_foo_b = forest_b.add_node(foo_node_b).unwrap();

        // This loop node's decorators are different from the loop node in a.
        let mut loop_node_b = MastNode::new_loop(id_foo_b, &forest_b).unwrap();
        loop_node_b.set_after_exit(vec![deco1_b, deco3_b]);
        let id_loop_b = forest_b.add_node(loop_node_b).unwrap();

        forest_b.make_root(id_loop_b);

        let merged = forest_a.merge(&forest_b).unwrap();

        // There are 4 unique decorators across both forests.
        assert_eq!(merged.decorators.len(), 4);
        assert!(merged.decorators.contains(&trace0));
        assert!(merged.decorators.contains(&trace1));
        assert!(merged.decorators.contains(&trace2));
        assert!(merged.decorators.contains(&trace3));

        let find_decorator_id = |deco: &Decorator| {
            let idx = merged
                .decorators
                .iter()
                .enumerate()
                .find_map(
                    |(deco_id, forest_deco)| if forest_deco == deco { Some(deco_id) } else { None },
                )
                .unwrap();
            DecoratorId::from_u32_safe(idx as u32, &merged).unwrap()
        };

        let merged_deco0 = find_decorator_id(&trace0);
        let merged_deco1 = find_decorator_id(&trace1);
        let merged_deco2 = find_decorator_id(&trace2);
        let merged_deco3 = find_decorator_id(&trace3);

        assert_eq!(merged.nodes.len(), 3);

        let merged_foo_block = merged.nodes.iter().find(|node| node.is_basic_block()).unwrap();
        let MastNode::Block(merged_foo_block) = merged_foo_block else {
            panic!("expected basic block node");
        };

        assert_eq!(
            merged_foo_block.decorators().as_slice(),
            &[(0, merged_deco1), (0, merged_deco2)]
        );

        // Asserts that there exists exactly one Loop Node with the given decorators.
        assert_eq!(
            merged
                .nodes
                .iter()
                .filter(|node| {
                    if let MastNode::Loop(loop_node) = node {
                        loop_node.after_exit() == [merged_deco0, merged_deco2]
                    } else {
                        false
                    }
                })
                .count(),
            1
        );

        // Asserts that there exists exactly one Loop Node with the given decorators.
        assert_eq!(
            merged
                .nodes
                .iter()
                .filter(|node| {
                    if let MastNode::Loop(loop_node) = node {
                        loop_node.after_exit() == [merged_deco1, merged_deco3]
                    } else {
                        false
                    }
                })
                .count(),
            1
        );
    }
}
