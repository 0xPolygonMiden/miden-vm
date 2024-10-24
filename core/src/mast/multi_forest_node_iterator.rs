use alloc::{collections::BTreeMap, vec::Vec};

use miden_crypto::hash::rpo::RpoDigest;

use crate::mast::{MastForest, MastForestError, MastNode, MastNodeId};

type ForestIndex = usize;

/// Depth First Search Iterator in Post Order for [`MastForest`]s.
///
/// This iterator iterates through all **reachable** nodes of all given forests exactly once.
///
/// Since a `MastForest` has multiple possible entrypoints in the form of its roots, a depth-first
/// search must visit all of those roots and the trees they form. This iterator's `Item` is
/// [`MultiMastForestIteratorItem`]. It contains either a [`MultiMastForestIteratorItem::Node`] of a
/// forest, or the replacement of an external node. This is returned if one forest contains an
/// External node with digest `foo` and another forest contains a non-external node with digest
/// `foo`. In such a case the `foo` node is yielded first (unless it was already visited) and
/// subsequently a "replacement signal" ([`MultiMastForestIteratorItem::ExternalNodeReplacement`])
/// for the external node is yielded to make the caller aware that this replacement has happened.
///
/// All of this is useful to ensure that children are always processed before their parents, even if
/// a child is an External node which is replaced by a node in another forest. This guarantees that
/// **all [`MastNodeId`]s of child nodes are strictly less than the [`MastNodeId`] of their
/// parents**.
///
/// For instance, consider these `MastForest`s being passed to this iterator with the `Call(0)`'s
/// digest being `qux`:
///
/// ```text
/// Forest A Nodes: [Block(foo), External(qux), Join(0, 1)]
/// Forest A Roots: [2]
/// Forest B Nodes: [Block(bar), Call(0)]
/// Forest B Roots: [0]
/// ```
///
/// The only root of A is the `Join` node at index 2. The first three nodes of the forest form a
/// tree, since the `Join` node references index 0 and 1. This tree is discovered by
/// starting at the root at index 2 and following all children until we reach terminal nodes (like
/// `Block`s) and building up a stack of the discovered nodes. The special case here is the
/// `External` node whose digest matches that of a node in forest B. Instead of the External
/// node being added to the stack, the tree of the Call node is added instead. The stack is built
/// such that popping elements off the stack (from the back) yields a postorder.
///
/// After the first tree is discovered, the stack looks like this:
/// ```text
/// [Node(forest_idx: 0, node_id: 2),
///  ExternalNodeReplacement(
///     replacement_forest_idx: 1, replacement_node_id: 1
///     replaced_forest_idx: 0, replaced_node_id: 1
///  ),
///  Node(forest_idx: 1, node_id: 1),
///  Node(forest_idx: 1, node_id: 0),
///  Node(forest_idx: 0, node_id: 0)]
/// ```
///
/// If the stack is exhausted we start another discovery if more unvisited roots exist. In this
/// example, the root of forest B was already visited due to the External node reference, so the
/// iteration is complete.
///
/// The iteration on a higher level thus consists of a back and forth between discovering trees and
/// returning nodes from the stack.
pub(crate) struct MultiMastForestNodeIter<'forest> {
    /// The forests that we're iterating.
    mast_forests: Vec<&'forest MastForest>,
    /// The index of the forest we're currently processing and discovering trees in.
    ///
    /// This value iterates through 0..mast_forests.len() which guarantees that we visit all
    /// forests once.
    current_forest_idx: ForestIndex,
    /// The procedure root index at which we last started a tree discovery in the
    /// current_forest_idx.
    ///
    /// This value iterates through 0..mast_forests[current_forest_idx].num_procedures() which
    /// guarantees that we visit all nodes reachable from all roots.
    current_procedure_root_idx: u32,
    /// A map of MAST roots of all non-external nodes in mast_forests to their forest and node
    /// indices.
    non_external_nodes: BTreeMap<RpoDigest, (ForestIndex, MastNodeId)>,
    /// Describes whether the node at some [forest_index][node_index] has already been visited.
    /// Note that this is set to true for all nodes that have been returned from the iterator.
    visited_nodes: Vec<Vec<bool>>,
    /// This stack always contains the discovered nodes.
    /// The stack might contain a node twice. However we only ever return a node once, which is
    /// checked in the `next` function.
    discovered_nodes: Vec<MultiMastForestIteratorItem>,
}

impl<'forest> MultiMastForestNodeIter<'forest> {
    /// Builds a map of MAST roots to non-external nodes in any of the given forests to initialize
    /// the iterator. This enables an efficient check whether for any encountered External node
    /// referencing digest `foo` a node with digest `foo` already exists in any forest.
    pub(crate) fn new(mast_forests: Vec<&'forest MastForest>) -> Self {
        let visited_nodes = mast_forests
            .iter()
            .map(|forest| vec![false; forest.num_nodes() as usize])
            .collect();

        let mut non_external_nodes = BTreeMap::new();

        for (forest_idx, forest) in mast_forests.iter().enumerate() {
            for (node_idx, node) in forest.nodes().iter().enumerate() {
                // SAFETY: The passed id comes from the iterator over the nodes, so we never exceed
                // the forest's number of nodes.
                let node_id = MastNodeId::new_unchecked(node_idx as u32);
                if !node.is_external() {
                    non_external_nodes.insert(node.digest(), (forest_idx, node_id));
                }
            }
        }

        Self {
            mast_forests,
            current_forest_idx: 0,
            current_procedure_root_idx: 0,
            non_external_nodes,
            visited_nodes,
            discovered_nodes: Vec::new(),
        }
    }

    /// Pushes the given node, uniquely identified by the forest and node index onto the stack
    /// even if the node was already discovered before.
    ///
    /// It's the callers responsibility to only pass valid indices.
    fn push_node(&mut self, forest_idx: usize, node_id: MastNodeId) {
        self.discovered_nodes
            .push(MultiMastForestIteratorItem::Node { forest_idx, node_id });
    }

    /// Discovers a tree starting at the given forest index and node id.
    ///
    /// It's the callers responsibility to only pass valid indices.
    fn discover_tree(
        &mut self,
        forest_idx: ForestIndex,
        node_id: MastNodeId,
    ) -> Result<(), MastForestError> {
        let current_node =
            &self.mast_forests[forest_idx].nodes.get(node_id.as_usize()).ok_or_else(|| {
                MastForestError::NodeIdOverflow(
                    node_id,
                    self.mast_forests[forest_idx].num_nodes() as usize,
                )
            })?;

        // Note that the order in which we add or discover nodes is the reverse of postorder, since
        // we're pushing them onto a stack, which reverses the order itself. Hence, reversing twice
        // gives us the actual postorder we want.
        match current_node {
            MastNode::Block(_) => {
                self.push_node(forest_idx, node_id);
            },
            MastNode::Join(join_node) => {
                self.push_node(forest_idx, node_id);
                self.discover_tree(forest_idx, join_node.second())?;
                self.discover_tree(forest_idx, join_node.first())?;
            },
            MastNode::Split(split_node) => {
                self.push_node(forest_idx, node_id);
                self.discover_tree(forest_idx, split_node.on_false())?;
                self.discover_tree(forest_idx, split_node.on_true())?;
            },
            MastNode::Loop(loop_node) => {
                self.push_node(forest_idx, node_id);
                self.discover_tree(forest_idx, loop_node.body())?;
            },
            MastNode::Call(call_node) => {
                self.push_node(forest_idx, node_id);
                self.discover_tree(forest_idx, call_node.callee())?;
            },
            MastNode::Dyn(_) => {
                self.push_node(forest_idx, node_id);
            },
            MastNode::External(external_node) => {
                // When we encounter an external node referencing digest `foo` there are two cases:
                // - If there exists a node `replacement` in any forest with digest `foo`, we want
                //   to replace the external node with that node, which we do in two steps.
                //   - Discover the `replacement`'s tree and add it to the stack.
                //     - If `replacement` was already visited before, it won't actually be returned.
                //     - In any case this means: The `replacement` node is processed before the
                //       replacement signal we're adding next.
                //   - Add a replacement signal to the stack, signaling that the `replacement`
                //     replaced the external node.
                //   - Note that the order of these operations in code is reversed, since the stack
                //     we're pushing the operations onto reverses the order once more.
                // - If no replacement exists, yield the External Node as a regular `Node`.
                if let Some((other_forest_idx, other_node_id)) =
                    self.non_external_nodes.get(&external_node.digest()).copied()
                {
                    self.discovered_nodes.push(
                        MultiMastForestIteratorItem::ExternalNodeReplacement {
                            replacement_forest_idx: other_forest_idx,
                            replacement_mast_node_id: other_node_id,
                            replaced_forest_idx: forest_idx,
                            replaced_mast_node_id: node_id,
                        },
                    );

                    self.discover_tree(other_forest_idx, other_node_id)?;
                } else {
                    self.push_node(forest_idx, node_id);
                }
            },
        }

        Ok(())
    }

    /// Finds the next unvisited procedure root and discovers a tree from it.
    ///
    /// If the unvisited node stack is empty after calling this function, the iteration is complete.
    ///
    /// This function basically consists of two loops:
    /// - The outer loop iterates over all forest indices.
    /// - The inner loop iterates over all procedure root indices for the current forest.
    fn discover_nodes(&mut self) {
        'forest_loop: while self.current_forest_idx < self.mast_forests.len()
            && self.discovered_nodes.is_empty()
        {
            // If we don't have any forests, there is nothing to do.
            if self.mast_forests.is_empty() {
                return;
            }

            // If the current forest doesn't have roots, advance to the next one.
            if self.mast_forests[self.current_forest_idx].num_procedures() == 0 {
                self.current_forest_idx += 1;
                continue;
            }

            let procedure_roots = self.mast_forests[self.current_forest_idx].procedure_roots();
            let visited_nodes = &self.visited_nodes[self.current_forest_idx];

            // Find the next unvisited procedure root for the current forest by incrementing the
            // current procedure root until we find one that was not yet visited.
            while visited_nodes
                [procedure_roots[self.current_procedure_root_idx as usize].as_usize()]
            {
                // If we have reached the end of the procedure roots for the current forest,
                // continue searching in the next forest.
                if self.current_procedure_root_idx + 1
                    >= self.mast_forests[self.current_forest_idx].num_procedures()
                {
                    // Reset current procedure root.
                    self.current_procedure_root_idx = 0;
                    // Increment forest index.
                    self.current_forest_idx += 1;

                    continue 'forest_loop;
                }

                // Since the current procedure root was already visited, check the next one.
                self.current_procedure_root_idx += 1;
            }

            // We exited the loop, so the current procedure root is unvisited and so we can start
            // a discovery from that root. Since that root is unvisited and undiscovered, it is
            // guaranteed that after this discovery the stack will be non-empty.
            let tree_root_id = procedure_roots[self.current_procedure_root_idx as usize];
            self.discover_tree(self.current_forest_idx, tree_root_id)
                .expect("we should only pass root indices that are valid for the forest");
        }
    }
}

impl Iterator for MultiMastForestNodeIter<'_> {
    type Item = MultiMastForestIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(stack_item) = self.discovered_nodes.pop() {
            // Get the forest and node index of the node being processed on the stack so we can
            // check if it has already been visited.
            let (forest_idx, node_id) = match &stack_item {
                MultiMastForestIteratorItem::Node { forest_idx, node_id } => (forest_idx, node_id),
                MultiMastForestIteratorItem::ExternalNodeReplacement {
                    replaced_forest_idx,
                    replaced_mast_node_id,
                    ..
                } => (replaced_forest_idx, replaced_mast_node_id),
            };

            let is_node_visited_mut = self.visited_nodes[*forest_idx]
                .get_mut(node_id.as_usize())
                .expect("visited_nodes can be safely indexed by any valid MastNodeId");
            if *is_node_visited_mut {
                continue;
            } else {
                *is_node_visited_mut = true;
            }

            return Some(stack_item);
        }

        self.discover_nodes();

        if !self.discovered_nodes.is_empty() {
            self.next()
        } else {
            // If the stack is empty after tree discovery, all (reachable) nodes have been
            // discovered and visited.
            None
        }
    }
}

/// The iterator item for [`MultiMastForestNodeIter`]. See its documentation for details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MultiMastForestIteratorItem {
    /// A regular node discovered by the iterator.
    Node {
        forest_idx: ForestIndex,
        node_id: MastNodeId,
    },
    /// Signals a replacement of an external node by some other node.
    ExternalNodeReplacement {
        replacement_forest_idx: usize,
        replacement_mast_node_id: MastNodeId,
        replaced_forest_idx: usize,
        replaced_mast_node_id: MastNodeId,
    },
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use miden_crypto::hash::rpo::RpoDigest;

    use super::*;
    use crate::Operation;

    fn random_digest() -> RpoDigest {
        RpoDigest::new([rand_utils::rand_value(); 4])
    }

    #[test]
    fn multi_mast_forest_dfs_empty() {
        let forest = MastForest::new();
        let mut iterator = MultiMastForestNodeIter::new(vec![&forest]);
        assert!(iterator.next().is_none());
    }

    #[test]
    fn multi_mast_forest_multiple_forests_dfs() {
        let nodea0_digest = random_digest();
        let nodea1_digest = random_digest();
        let nodea2_digest = random_digest();
        let nodea3_digest = random_digest();

        let nodeb0_digest = random_digest();

        let mut forest_a = MastForest::new();
        forest_a.add_external(nodea0_digest).unwrap();
        let id1 = forest_a.add_external(nodea1_digest).unwrap();
        let id2 = forest_a.add_external(nodea2_digest).unwrap();
        let id3 = forest_a.add_external(nodea3_digest).unwrap();
        let id_split = forest_a.add_split(id2, id3).unwrap();
        let id_join = forest_a.add_join(id2, id_split).unwrap();

        forest_a.make_root(id_join);
        forest_a.make_root(id1);

        let mut forest_b = MastForest::new();
        let id_ext_b = forest_b.add_external(nodeb0_digest).unwrap();
        let id_block_b = forest_b.add_block(vec![Operation::Eqz], None).unwrap();
        let id_split_b = forest_b.add_split(id_ext_b, id_block_b).unwrap();

        forest_b.make_root(id_split_b);

        // Note that the node at index 0 is not visited because it is not reachable from any root
        // and is not a root itself.
        let nodes = MultiMastForestNodeIter::new(vec![&forest_a, &forest_b]).collect::<Vec<_>>();

        assert_eq!(nodes.len(), 8);
        assert_eq!(nodes[0], MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: id2 });
        assert_eq!(nodes[1], MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: id3 });
        assert_eq!(
            nodes[2],
            MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: id_split }
        );
        assert_eq!(nodes[3], MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: id_join });
        assert_eq!(nodes[4], MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: id1 });
        assert_eq!(
            nodes[5],
            MultiMastForestIteratorItem::Node { forest_idx: 1, node_id: id_ext_b }
        );
        assert_eq!(
            nodes[6],
            MultiMastForestIteratorItem::Node { forest_idx: 1, node_id: id_block_b }
        );
        assert_eq!(
            nodes[7],
            MultiMastForestIteratorItem::Node { forest_idx: 1, node_id: id_split_b }
        );
    }

    #[test]
    fn multi_mast_forest_external_dependencies() {
        let block_foo = MastNode::new_basic_block(vec![Operation::Drop], None).unwrap();
        let mut forest_a = MastForest::new();
        let id_foo_a = forest_a.add_external(block_foo.digest()).unwrap();
        let id_call_a = forest_a.add_call(id_foo_a).unwrap();
        forest_a.make_root(id_call_a);

        let mut forest_b = MastForest::new();
        let id_ext_b = forest_b.add_external(forest_a[id_call_a].digest()).unwrap();
        let id_call_b = forest_b.add_call(id_ext_b).unwrap();
        forest_b.add_node(block_foo).unwrap();
        forest_b.make_root(id_call_b);

        let nodes = MultiMastForestNodeIter::new(vec![&forest_a, &forest_b]).collect::<Vec<_>>();

        assert_eq!(nodes.len(), 5);

        // The replacement for the external node from forest A.
        assert_eq!(
            nodes[0],
            MultiMastForestIteratorItem::Node {
                forest_idx: 1,
                node_id: MastNodeId::new_unchecked(2)
            }
        );
        // The external node replaced by the block foo from forest B.
        assert_eq!(
            nodes[1],
            MultiMastForestIteratorItem::ExternalNodeReplacement {
                replacement_forest_idx: 1,
                replacement_mast_node_id: MastNodeId::new_unchecked(2),
                replaced_forest_idx: 0,
                replaced_mast_node_id: MastNodeId::new_unchecked(0)
            }
        );
        // The call from forest A.
        assert_eq!(
            nodes[2],
            MultiMastForestIteratorItem::Node {
                forest_idx: 0,
                node_id: MastNodeId::new_unchecked(1)
            }
        );
        // The replacement for the external node that is replaced by the Call in forest A.
        assert_eq!(
            nodes[3],
            MultiMastForestIteratorItem::ExternalNodeReplacement {
                replacement_forest_idx: 0,
                replacement_mast_node_id: MastNodeId::new_unchecked(1),
                replaced_forest_idx: 1,
                replaced_mast_node_id: MastNodeId::new_unchecked(0)
            }
        );
        // The call from forest B.
        assert_eq!(
            nodes[4],
            MultiMastForestIteratorItem::Node {
                forest_idx: 1,
                node_id: MastNodeId::new_unchecked(1)
            }
        );
    }

    /// Tests that a node which appears twice in a tree is returned in the required order.
    ///
    /// In this test we have a MastForest with a tree like this:
    ///
    ///     3 <- Split Node
    ///    / \
    ///   1   2
    ///  /     \
    /// 0       0
    ///
    /// In a previous implementation we marked the nodes that we discovered as visited immediately
    /// and did not add them again when encountering them again. This was not correct because of
    /// this example. In the iterator, we need to discover nodes in reverse so we can later pop
    /// them off the stack in postorder. So we first descend down child 2 of the split node.
    /// This means we would mark 0 as visited and then add 2 to the stack. Then we descend down
    /// child 1 but do not add 0 again because it is already visited and we add 1. So we end up
    /// with this stack for the tree: [3, 2, 0, 1]. This means when we pop off nodes from the
    /// stack we get to 1 before we get to 0 and that violates the guarantees from this
    /// iterator.
    ///
    /// Hence this test to ensure that we do return nodes in the right order.
    ///
    /// This test and example is essentially a copy from a part of the MastForest of the Miden
    /// Stdlib where this error occured.
    #[test]
    fn multi_mast_forest_child_duplicate() {
        let block_foo = MastNode::new_basic_block(vec![Operation::Drop], None).unwrap();
        let mut forest = MastForest::new();
        let id_foo = forest.add_external(block_foo.digest()).unwrap();
        let id_call1 = forest.add_call(id_foo).unwrap();
        let id_call2 = forest.add_call(id_foo).unwrap();
        let id_split = forest.add_split(id_call1, id_call2).unwrap();
        forest.make_root(id_split);

        let nodes = MultiMastForestNodeIter::new(vec![&forest]).collect::<Vec<_>>();

        // The foo node should be yielded first and it should not be yielded twice.
        for (i, expected_node_id) in [id_foo, id_call1, id_call2, id_split].into_iter().enumerate()
        {
            assert_eq!(
                nodes[i],
                MultiMastForestIteratorItem::Node { forest_idx: 0, node_id: expected_node_id }
            );
        }
    }
}
