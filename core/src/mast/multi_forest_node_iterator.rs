use alloc::{collections::BTreeMap, vec::Vec};

use miden_crypto::hash::rpo::RpoDigest;

use crate::mast::{MastForest, MastForestError, MastNode, MastNodeId};

type ForestIndex = usize;

/// Depth First Search Iterator in Post Order for [`MastForest`]s.
///
/// This iterator iterates through all **reachable** nodes of a forest exactly once.
///
/// Since a `MastForest` has multiple possible entrypoints in the form of its roots, a depth-first
/// search must visit all of those roots and the trees they form.
///
/// For instance, consider this `MastForest`:
///
/// ```text
/// Nodes: [Block(foo), Block(bar), Join(0, 1), External(qux)]
/// Roots: [2]
/// ```
///
/// The only root is the `Join` node at index 2. The first three nodes of the forest form a
/// tree, since the `Join` node references index 0 and 1. This tree is discovered by
/// starting at the root at index 2 and following all children until we reach terminal nodes (like
/// `Block`s) and build up a stack of the discovered, but unvisited nodes. The stack is
/// built such that popping elements off the stack (from the back) yields a postorder.
///
/// After the first tree is discovered, the stack looks like this: `[2, 1, 0]`. On each
/// call to `next` one element is popped off this stack and returned.
///
/// If the stack is exhausted we start another discovery if more unvisited roots exist. Since the
/// `External` node is not a root and not referenced by any other tree in the forest, it will not be
/// visited.
///
/// The iteration on a high-level thus consists of a constant back and forth between discovering
/// trees and returning nodes from the stack.
///
/// Note: This type could be made more general to implement pre-order or in-order iteration too.
pub(crate) struct MultiMastForestNodeIter<'forest> {
    /// The forest that we're iterating.
    mast_forests: Vec<&'forest MastForest>,
    /// The procedure root index at which we last started a tree discovery.
    ///
    /// This value iterates through 0..mast_forest.num_procedures() which guarantees that we visit
    /// all nodes reachable from all roots.
    last_forest_idx: usize,
    last_procedure_root_idx: u32,
    non_external_nodes: BTreeMap<RpoDigest, (usize, MastNodeId)>,
    /// Describes whether the node at some index has already been visited. Note that this is set to
    /// true for all nodes on the stack, even if the caller of the iterator has not yet seen the
    /// node. See [`Self::visit_later`] for more details.
    node_visited: Vec<Vec<bool>>,
    /// This stack always contains the discovered but unvisited nodes.
    /// For any id store on the stack it holds that `node_visited[id] = true`.
    unvisited_node_stack: Vec<MultiMastForestIteratorItem>,
}

impl<'forest> MultiMastForestNodeIter<'forest> {
    pub(crate) fn new(mast_forests: Vec<&'forest MastForest>) -> Self {
        let visited = mast_forests
            .iter()
            .map(|forest| vec![false; forest.num_nodes() as usize])
            .collect();

        let mut non_external_nodes = BTreeMap::new();

        for (forest_idx, forest) in mast_forests.iter().enumerate() {
            for (node_idx, node) in forest.nodes().iter().enumerate() {
                let node_id = MastNodeId::from_u32_safe(node_idx as u32, mast_forests[forest_idx])
                    .expect("the passed id should be a valid node in the forest");
                if !node.is_external() {
                    non_external_nodes.insert(node.digest(), (forest_idx, node_id));
                }
            }
        }

        Self {
            mast_forests,
            last_forest_idx: 0,
            last_procedure_root_idx: 0,
            non_external_nodes,
            node_visited: visited,
            unvisited_node_stack: Vec::new(),
        }
    }

    /// Pushes the given index onto the stack unless the index was already visited.
    fn mark_for_visit(&mut self, forest_idx: usize, node_id: MastNodeId) {
        // SAFETY: The node_visited Vec's len is equal to the number of forest nodes
        // so any `MastNodeId` from that forest is safe to use.
        let node_visited_mut = self.node_visited[forest_idx]
            .get_mut(node_id.as_usize())
            .expect("node_visited can be safely indexed by any valid MastNodeId");

        if !*node_visited_mut {
            self.unvisited_node_stack
                .push(MultiMastForestIteratorItem::Regular { forest_idx, node_id });
            // Set nodes added to the stack as visited even though we have not technically visited
            // them. This is however important to avoid visiting nodes twice that appear
            // in the same tree. If we were to add all nodes to the stack that we
            // discovered, then we would have duplicate ids on the stack. Marking them
            // as visited immediately when adding them avoid this issue.
            *node_visited_mut = true;
        }
    }

    /// Discovers a tree starting at the given root index.
    fn discover_tree(
        &mut self,
        forest_idx: usize,
        root_idx: MastNodeId,
    ) -> Result<(), MastForestError> {
        let current_node =
            &self.mast_forests[forest_idx].nodes.get(root_idx.as_usize()).ok_or_else(|| {
                MastForestError::NodeIdOverflow(
                    root_idx,
                    self.mast_forests[forest_idx].num_nodes() as usize,
                )
            })?;

        // Note that the order in which we add or discover nodes is the reverse of postorder, since
        // we're pushing them onto a stack, which reverses the order itself. Hence, reversing twice
        // gives us the actual postorder we want.
        match current_node {
            MastNode::Block(_) => {
                self.mark_for_visit(forest_idx, root_idx);
            },
            MastNode::Join(join_node) => {
                self.mark_for_visit(forest_idx, root_idx);
                self.discover_tree(forest_idx, join_node.second())?;
                self.discover_tree(forest_idx, join_node.first())?;
            },
            MastNode::Split(split_node) => {
                self.mark_for_visit(forest_idx, root_idx);
                self.discover_tree(forest_idx, split_node.on_false())?;
                self.discover_tree(forest_idx, split_node.on_true())?;
            },
            MastNode::Loop(loop_node) => {
                self.mark_for_visit(forest_idx, root_idx);
                self.discover_tree(forest_idx, loop_node.body())?;
            },
            MastNode::Call(call_node) => {
                self.mark_for_visit(forest_idx, root_idx);
                self.discover_tree(forest_idx, call_node.callee())?;
            },
            MastNode::Dyn(_) => {
                self.mark_for_visit(forest_idx, root_idx);
            },
            MastNode::External(external_node) => {
                if let Some((other_forest_idx, other_node_id)) =
                    self.non_external_nodes.get(&external_node.digest()).copied()
                {
                    let visited = self.node_visited[forest_idx]
                        .get(root_idx.as_usize())
                        .expect("node_visited can be safely indexed by any valid MastNodeId");
                    if !visited {
                        self.unvisited_node_stack.push(
                            MultiMastForestIteratorItem::ExternalNodeReplacement {
                                replacement_forest_idx: other_forest_idx,
                                replacement_mast_node_id: other_node_id,
                                replaced_forest_idx: forest_idx,
                                replaced_mast_node_id: root_idx,
                            },
                        );
                    }

                    self.discover_tree(other_forest_idx, other_node_id)?;

                    // Skip external node.
                    *self.node_visited[forest_idx]
                        .get_mut(root_idx.as_usize())
                        .expect("node_visited can be safely indexed by any valid MastNodeId") =
                        true;
                } else {
                    self.mark_for_visit(forest_idx, root_idx);
                }
            },
        }

        Ok(())
    }

    /// Finds the next unvisited procedure root and discovers a tree from it.
    ///
    /// If the unvisited node stack is empty after calling this function, the iteration is complete.
    fn discover_nodes(&mut self) {
        'forest_loop: while self.last_forest_idx < self.mast_forests.len()
            && self.unvisited_node_stack.is_empty()
        {
            if self.mast_forests.is_empty() {
                return;
            }
            if self.mast_forests[self.last_forest_idx].num_procedures() == 0 {
                self.last_forest_idx += 1;
                continue;
            }

            let procedure_roots = self.mast_forests[self.last_forest_idx].procedure_roots();
            let node_visited = &self.node_visited[self.last_forest_idx];
            // Find the next unvisited procedure root.
            while node_visited[procedure_roots[self.last_procedure_root_idx as usize].as_usize()] {
                if self.last_procedure_root_idx + 1
                    >= self.mast_forests[self.last_forest_idx].num_procedures()
                {
                    self.last_procedure_root_idx = 0;
                    self.last_forest_idx += 1;
                    continue 'forest_loop;
                }
                self.last_procedure_root_idx += 1;
            }

            let tree_root_id = procedure_roots[self.last_procedure_root_idx as usize];
            self.discover_tree(self.last_forest_idx, tree_root_id)
                .expect("we should only pass root indices that are valid for the forest");
        }
    }
}

impl Iterator for MultiMastForestNodeIter<'_> {
    type Item = MultiMastForestIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(stack_item) = self.unvisited_node_stack.pop() {
            // SAFETY: We only add valid ids to the stack so it's fine to index the forest nodes
            // directly.
            // let node = &self.mast_forests[stack_item.forest_idx].nodes[next_node_id.as_usize()];

            return Some(stack_item);
        }

        self.discover_nodes();

        if !self.unvisited_node_stack.is_empty() {
            self.next()
        } else {
            // If the stack is empty after tree discovery, all (reachable) nodes have been
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MultiMastForestIteratorItem {
    Regular {
        forest_idx: ForestIndex,
        node_id: MastNodeId,
    },
    ExternalNodeReplacement {
        replacement_forest_idx: usize,
        replacement_mast_node_id: MastNodeId,
        replaced_forest_idx: usize,
        replaced_mast_node_id: MastNodeId,
    },
}

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
        assert_eq!(nodes[0], MultiMastForestIteratorItem::Regular { forest_idx: 0, node_id: id2 });
        assert_eq!(nodes[1], MultiMastForestIteratorItem::Regular { forest_idx: 0, node_id: id3 });
        assert_eq!(
            nodes[2],
            MultiMastForestIteratorItem::Regular { forest_idx: 0, node_id: id_split }
        );
        assert_eq!(
            nodes[3],
            MultiMastForestIteratorItem::Regular { forest_idx: 0, node_id: id_join }
        );
        assert_eq!(nodes[4], MultiMastForestIteratorItem::Regular { forest_idx: 0, node_id: id1 });
        assert_eq!(
            nodes[5],
            MultiMastForestIteratorItem::Regular { forest_idx: 1, node_id: id_ext_b }
        );
        assert_eq!(
            nodes[6],
            MultiMastForestIteratorItem::Regular { forest_idx: 1, node_id: id_block_b }
        );
        assert_eq!(
            nodes[7],
            MultiMastForestIteratorItem::Regular { forest_idx: 1, node_id: id_split_b }
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
            MultiMastForestIteratorItem::Regular {
                forest_idx: 1,
                node_id: MastNodeId::new_unsafe(2)
            }
        );
        // The external node replaced by the block foo from forest B.
        assert_eq!(
            nodes[1],
            MultiMastForestIteratorItem::ExternalNodeReplacement {
                replacement_forest_idx: 1,
                replacement_mast_node_id: MastNodeId::new_unsafe(2),
                replaced_forest_idx: 0,
                replaced_mast_node_id: MastNodeId::new_unsafe(0)
            }
        );
        // The call from forest A.
        assert_eq!(
            nodes[2],
            MultiMastForestIteratorItem::Regular {
                forest_idx: 0,
                node_id: MastNodeId::new_unsafe(1)
            }
        );
        // The replacement for the external node that is replaced by the Call in forest A.
        assert_eq!(
            nodes[3],
            MultiMastForestIteratorItem::ExternalNodeReplacement {
                replacement_forest_idx: 0,
                replacement_mast_node_id: MastNodeId::new_unsafe(1),
                replaced_forest_idx: 1,
                replaced_mast_node_id: MastNodeId::new_unsafe(0)
            }
        );
        // The call from forest B.
        assert_eq!(
            nodes[4],
            MultiMastForestIteratorItem::Regular {
                forest_idx: 1,
                node_id: MastNodeId::new_unsafe(1)
            }
        );
    }
}
