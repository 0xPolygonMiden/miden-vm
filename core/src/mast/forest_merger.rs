use alloc::{collections::BTreeMap, vec::Vec};
use std::println;

use miden_crypto::hash::blake::Blake3Digest;

use crate::mast::{DecoratorId, MastForest, MastForestError, MastNode, MastNodeEq, MastNodeId};

pub struct MastForestMerger {
    forest: MastForest,
    node_id_by_hash: BTreeMap<MastNodeEq, MastForestIndexEntry>,
    hash_by_node_id: BTreeMap<MastNodeId, MastNodeEq>,
    decorators: BTreeMap<Blake3Digest<32>, DecoratorId>,
}

impl MastForestMerger {
    pub fn new(forest: MastForest) -> Self {
        let mut forest = Self {
            node_id_by_hash: BTreeMap::new(),
            hash_by_node_id: BTreeMap::new(),
            decorators: BTreeMap::new(),
            forest,
        };

        forest.build_index();

        forest
    }

    pub fn merge(&mut self, other_forest: MastForest) -> Result<(), MastForestError> {
        // TODO: Use Vecs for both of these if we actually map all IDs.
        let mut decorator_id_remapping = BTreeMap::new();
        let mut node_id_remapping = BTreeMap::new();

        for (merging_id, merging_decorator) in other_forest.decorators.iter().enumerate() {
            let new_decorator_id = if let Some(existing_decorator) =
                self.decorators.get(&merging_decorator.eq_hash())
            {
                *existing_decorator
            } else {
                self.forest.add_decorator(merging_decorator.clone())?
            };

            let merging_id =
                DecoratorId::from_u32_safe(merging_id as u32, &other_forest).expect("TODO");
            decorator_id_remapping.insert(merging_id, new_decorator_id);
        }

        for (merging_id, node) in ForestDfsPostOrder::new(&other_forest) {
            // We need to remap the node prior to computing the MastNodeEq.
            // This is because the MastNodeEq computation looks up its descendants and decorators in
            // the internal index, and if we were to pass the original node to that
            // computation, it would look up the incorrect descendants and decorators.
            let remapped_node =
                Self::remap_node(node, &decorator_id_remapping, &node_id_remapping, &self.forest);

            let node_eq =
                MastNodeEq::from_mast_node(&self.forest, &self.hash_by_node_id, &remapped_node);

            match self.node_id_by_hash.get_mut(&node_eq) {
                Some(existing_entry) => {
                    println!(
                        "node {} already exists in original forest\n{:?}\n{:?}",
                        merging_id, node, self.forest[existing_entry.node_id]
                    );
                    // We have to map any occurence of `merging_id` to `existing_node_id`.
                    node_id_remapping.insert(merging_id, existing_entry.node_id);

                    // Replace the external node in the existing forest with the non-external node
                    // from the merging forest.
                    // Any node in the existing forest that pointed to the external node will
                    // have the same digest due to the semantics of external nodes.
                    //
                    // Note that the inverse case is handled implicitly, that is, an external node
                    // in the merging forest that already exists as an external or non-external
                    // node in the existing forest will not be added to the merged forest because it
                    // will be handled like any other duplicate.
                    if existing_entry.is_external && !remapped_node.is_external() {
                        self.forest.nodes[existing_entry.node_id.as_usize()] = remapped_node;
                        // Change the flag in the index since we just replaced the external with a
                        // non-external node.
                        existing_entry.is_external = false;
                    }
                },
                None => {
                    let is_external = remapped_node.is_external();
                    let new_node_id = self.forest.add_node(remapped_node)?;
                    node_id_remapping.insert(merging_id, new_node_id);

                    // We need to update the indices with the newly inserted nodes
                    // since the MastNodeEq computation requires all descendants of a node
                    // to be in this index. Hence when we encounter a node in the merging forest
                    // which has descendants (Call, Loop, Split, ...), then those need to be in the
                    // indices.
                    self.node_id_by_hash.insert(
                        node_eq,
                        MastForestIndexEntry { node_id: new_node_id, is_external },
                    );
                    self.hash_by_node_id.insert(new_node_id, node_eq);
                },
            }
        }

        for root_id in other_forest.roots {
            // Map the previous root to its possibly new id.
            let new_root = node_id_remapping.get(&root_id).copied().unwrap_or(root_id);
            // This will take O(n) every time to check if the root already exists.
            // We could this by keeping a BTreeSet<MastNodeId> of existing roots during merging for
            // a faster check.
            self.forest.make_root(new_root);
        }

        Ok(())
    }

    pub fn into_forest(self) -> MastForest {
        self.forest
    }

    fn remap_node(
        node: &MastNode,
        decorator_id_remapping: &BTreeMap<DecoratorId, DecoratorId>,
        node_id_remapping: &BTreeMap<MastNodeId, MastNodeId>,
        mast_forest: &MastForest,
    ) -> MastNode {
        let map_decorator_id = |decorator_id: &DecoratorId| {
            decorator_id_remapping.get(decorator_id).copied().expect("TODO")
        };
        let map_decorators =
            |decorators: &[DecoratorId]| decorators.iter().map(map_decorator_id).collect();

        // TODO: Is the unwrap_or necessary?
        let map_node_id_or_previous =
            |node_id: MastNodeId| node_id_remapping.get(&node_id).copied().unwrap_or(node_id);

        let mut mapped_node = match node {
            MastNode::Join(join_node) => {
                let new_first = map_node_id_or_previous(join_node.first());
                let new_second = map_node_id_or_previous(join_node.second());
                MastNode::new_join(new_first, new_second, mast_forest)
                    .expect("mapping of JoinNode should succeed")
            },
            MastNode::Split(split_node) => {
                let if_branch = map_node_id_or_previous(split_node.on_true());
                let else_branch = map_node_id_or_previous(split_node.on_false());

                MastNode::new_split(if_branch, else_branch, mast_forest)
                    .expect("mapping of SplitNode should succeed")
            },
            MastNode::Loop(loop_node) => {
                let body = map_node_id_or_previous(loop_node.body());
                MastNode::new_loop(body, mast_forest).expect("mapping of LoopNode should succeed")
            },
            MastNode::Call(call_node) => {
                let callee = map_node_id_or_previous(call_node.callee());
                MastNode::new_call(callee, mast_forest).expect("mapping of CallNode should succeed")
            },
            // Other nodes are simply copied.
            MastNode::Block(basic_block_node) => {
                let remapped_block_node = MastNode::new_basic_block(
                    basic_block_node.operations().copied().collect(),
                    Some(
                        basic_block_node
                            .decorators()
                            .iter()
                            .map(|(idx, decorator_id)| (*idx, map_decorator_id(decorator_id)))
                            .collect(),
                    ),
                )
                .expect("mapping of BasicBlockNode should succeed");

                remapped_block_node
            },
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

    /// Helper function to build the initial index of the forest in which others are going to be
    /// merged.
    fn build_index(&mut self) {
        let dfs = ForestDfsPostOrder::new(&self.forest);
        for (id, node) in dfs {
            // SAFETY: This is safe as MastForests are constrained to less than 2^32 nodes.
            let node_eq = MastNodeEq::from_mast_node(&self.forest, &self.hash_by_node_id, node);
            self.hash_by_node_id.insert(id, node_eq);
            self.node_id_by_hash.insert(
                node_eq,
                MastForestIndexEntry {
                    node_id: id,
                    is_external: node.is_external(),
                },
            );
        }

        for (id, decorator) in self.forest.decorators.iter().enumerate() {
            self.decorators.insert(
                decorator.eq_hash(),
                DecoratorId::from_u32_safe(id as u32, &self.forest).expect("TODO"),
            );
        }
    }
}

struct MastForestIndexEntry {
    node_id: MastNodeId,
    is_external: bool,
}

struct ForestDfsPostOrder<'forest> {
    pub mast_forest: &'forest MastForest,
    pub last_visited_idx: usize,
    pub node_visited: Vec<bool>,
    // TODO: Invariant: unvisited nodes && visited[id] = true.
    pub node_stack: Vec<MastNodeId>,
}

impl<'forest> ForestDfsPostOrder<'forest> {
    fn new(mast_forest: &'forest MastForest) -> Self {
        let visited = vec![false; mast_forest.num_nodes() as usize];
        Self {
            mast_forest,
            last_visited_idx: 0,
            node_visited: visited,
            node_stack: Vec::new(),
        }
    }

    fn visit_later(&mut self, idx: MastNodeId) {
        if !self.node_visited[idx.as_usize()] {
            self.node_stack.push(idx);
            self.node_visited[idx.as_usize()] = true;
        }
    }

    // TODO: Point out reverse order.
    fn discover_subtree(&mut self, idx: MastNodeId) {
        let current_node = &self.mast_forest.nodes[idx.as_usize()];
        match current_node {
            MastNode::Block(_) => {
                self.visit_later(idx);
            },
            MastNode::Join(join_node) => {
                self.visit_later(idx);
                self.discover_subtree(join_node.second());
                self.discover_subtree(join_node.first());
            },
            MastNode::Split(split_node) => {
                self.visit_later(idx);
                self.discover_subtree(split_node.on_false());
                self.discover_subtree(split_node.on_true());
            },
            MastNode::Loop(loop_node) => {
                self.visit_later(idx);
                self.discover_subtree(loop_node.body());
            },
            MastNode::Call(call_node) => {
                self.visit_later(idx);
                self.discover_subtree(call_node.callee());
            },
            MastNode::Dyn(_) => {
                self.visit_later(idx);
            },
            MastNode::External(_) => {
                self.visit_later(idx);
            },
        }
    }

    fn discover_nodes(&mut self) {
        while self.node_visited[self.last_visited_idx] {
            if self.last_visited_idx + 1 >= self.mast_forest.num_nodes() as usize {
                return;
            }
            self.last_visited_idx += 1;
        }

        self.discover_subtree(
            MastNodeId::from_u32_safe(self.last_visited_idx as u32, self.mast_forest)
                .expect("todo"),
        );
    }
}

impl<'forest> Iterator for ForestDfsPostOrder<'forest> {
    type Item = (MastNodeId, &'forest MastNode);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_node_id) = self.node_stack.pop() {
            // SAFETY: We only add valid ids to the stack so it's fine to index the forest nodes
            // directly.
            let node = &self.mast_forest.nodes[next_node_id.as_usize()];
            return Some((next_node_id, node));
        }

        self.discover_nodes();

        if !self.node_stack.is_empty() {
            self.next()
        } else {
            debug_assert!(self.node_visited.iter().all(|visited| *visited));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use miden_crypto::hash::rpo::RpoDigest;

    use super::*;
    use crate::{Decorator, Operation};

    impl MastForest {
        fn debug_print(&self) {
            for (idx, node) in self.nodes().iter().enumerate() {
                std::println!("Node {idx}\n{}\n", node.to_display(self));
            }
        }
    }

    fn random_digest() -> RpoDigest {
        RpoDigest::new([rand_utils::rand_value(); 4])
    }

    fn block_foo() -> MastNode {
        MastNode::new_basic_block(vec![Operation::Mul, Operation::Add], None).unwrap()
    }

    fn block_bar() -> MastNode {
        MastNode::new_basic_block(vec![Operation::And, Operation::Eq], None).unwrap()
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
        forest_a.add_call(id_foo).unwrap();

        let mut forest_b = MastForest::new();
        let id_bar = forest_b.add_node(block_bar()).unwrap();
        forest_b.add_call(id_bar).unwrap();

        forest_a.debug_print();
        forest_b.debug_print();

        forest_a.merge(forest_b).unwrap();
        let merged = forest_a;

        merged.debug_print();

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
        let id_external = forest_a.add_external(random_digest()).unwrap();
        let id_foo = forest_a.add_node(block_foo()).unwrap();
        forest_a.add_call(id_foo).unwrap();
        forest_a.add_loop(id_external).unwrap();

        let original = forest_a.clone();

        forest_a.merge(forest_a.clone()).unwrap();

        assert_eq!(forest_a, original);
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
        forest_a.add_call(id_foo_a).unwrap();

        let mut forest_b = MastForest::new();
        let id_foo_b = forest_b.add_node(block_foo()).unwrap();
        forest_b.add_call(id_foo_b).unwrap();

        let mut merged_ab = forest_a.clone();
        let mut merged_ba = forest_b.clone();

        merged_ab.merge(forest_b).unwrap();
        merged_ba.merge(forest_a).unwrap();

        for merged in [merged_ab, merged_ba] {
            assert_eq!(merged.nodes().len(), 2);
            assert_eq!(merged.nodes()[0], block_foo());
            assert_matches!(&merged.nodes()[1], MastNode::Call(call_node) if call_node.callee().as_u32() == 0);
        }
    }

    /// Test that roots are preserved and deduplicated if appropriate.
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

        forest_a.merge(forest_b).unwrap();

        assert_eq!(forest_a.procedure_roots().len(), 2);

        let root_digests = forest_a.procedure_digests().collect::<Vec<_>>();
        assert!(root_digests.contains(&root_digest_call_a));
        assert!(root_digests.contains(&root_digest_bar_b));
        assert!(root_digests.contains(&root_digest_call_b));
    }

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
        forest_a.add_node(loop_node_a).unwrap();

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
        forest_b.add_node(loop_node_b).unwrap();

        // forest_a.debug_print();
        // println!("{:#?}", forest_a);
        // println!("FOREST B");
        // println!("{:#?}", forest_b);
        // forest_b.debug_print();

        forest_a.merge(forest_b).unwrap();

        // There are 4 unique decorators across both forests.
        assert_eq!(forest_a.decorators.len(), 4);
        assert!(forest_a.decorators.contains(&trace0));
        assert!(forest_a.decorators.contains(&trace1));
        assert!(forest_a.decorators.contains(&trace2));
        assert!(forest_a.decorators.contains(&trace3));

        let find_decorator_id = |deco: &Decorator| {
            let idx = forest_a
                .decorators
                .iter()
                .enumerate()
                .find_map(
                    |(deco_id, forest_deco)| if forest_deco == deco { Some(deco_id) } else { None },
                )
                .unwrap();
            DecoratorId::from_u32_safe(idx as u32, &forest_a).unwrap()
        };

        let merged_deco0 = find_decorator_id(&trace0);
        let merged_deco1 = find_decorator_id(&trace1);
        let merged_deco2 = find_decorator_id(&trace2);
        let merged_deco3 = find_decorator_id(&trace3);

        assert_eq!(forest_a.nodes.len(), 3);

        let merged_foo_block = forest_a.nodes.iter().find(|node| node.is_basic_block()).unwrap();
        let MastNode::Block(merged_foo_block) = merged_foo_block else {
            panic!("expected basic block node");
        };

        println!("{forest_a:#?}");
        forest_a.debug_print();

        assert_eq!(
            merged_foo_block.decorators().as_slice(),
            &[(0, merged_deco1), (0, merged_deco2)]
        );

        assert_eq!(
            forest_a
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

        assert_eq!(
            forest_a
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

    #[test]
    fn mast_forest_dfs() {
        let node5_digest = random_digest();
        let node1_digest = random_digest();
        let node2_digest = random_digest();
        let node3_digest = random_digest();

        let mut forest = MastForest::new();
        // This will become the node at index 5 later.
        let mut id5 = forest.add_external(node5_digest).unwrap();
        let id1 = forest.add_external(node1_digest).unwrap();
        let id2 = forest.add_external(node2_digest).unwrap();
        let id3 = forest.add_external(node3_digest).unwrap();
        let id_split = forest.add_split(id2, id3).unwrap();
        let mut id_join = forest.add_join(id2, id_split).unwrap();

        // Swap the join node to the first position so we visit a node first whose descendants have
        // not been visited yet.
        forest.nodes.swap(id5.as_usize(), id_join.as_usize());
        std::mem::swap(&mut id5, &mut id_join);

        let mut iterator = ForestDfsPostOrder::new(&forest);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node2_digest && id == id2);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node3_digest && id == id3);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::Split(_)) if id == id_split);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::Join(_)) if id == id_join);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node1_digest&& id == id1);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node5_digest&& id == id5);
        assert!(iterator.next().is_none());
    }
}
