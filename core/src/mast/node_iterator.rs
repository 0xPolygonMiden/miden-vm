use alloc::vec::Vec;

use crate::mast::{MastForest, MastNode, MastNodeId};

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
pub(crate) struct MastForestNodeIter<'forest> {
    /// The forest that we're iterating.
    pub mast_forest: &'forest MastForest,
    /// The procedure root index at which we last started a tree discovery.
    ///
    /// This value iterates through 0..mast_forest.num_procedures() which guarantees that we visit
    /// all nodes reachable from all roots.
    pub last_procedure_root_idx: u32,
    /// Describes whether the node at some index has already been visited. Note that this is set to
    /// true for all nodes on the stack, even if the caller of the iterator has not yet seen the
    /// node. See [`Self::visit_later`] for more details.
    pub node_visited: Vec<bool>,
    /// This stack always contains the discovered but unvisited nodes.
    /// For any id store on the stack it holds that `node_visited[id] = true`.
    pub unvisited_node_stack: Vec<MastNodeId>,
}

impl<'forest> MastForestNodeIter<'forest> {
    pub(crate) fn new(mast_forest: &'forest MastForest) -> Self {
        let visited = vec![false; mast_forest.num_nodes() as usize];

        Self {
            mast_forest,
            last_procedure_root_idx: 0,
            node_visited: visited,
            unvisited_node_stack: Vec::new(),
        }
    }

    /// Pushes the given index onto the stack unless the index was already visited.
    fn mark_for_visit(&mut self, node_id: MastNodeId) {
        // SAFETY: The node_visited Vec's len is equal to the number of forest nodes
        // so any `MastNodeId` from that forest is safe to use.
        let node_visited_mut = self
            .node_visited
            .get_mut(node_id.as_usize())
            .expect("node_visited can be safely indexed by any valid MastNodeId");

        if !*node_visited_mut {
            self.unvisited_node_stack.push(node_id);
            // Set nodes added to the stack as visited even though we have not technically visited
            // them. This is however important to avoid visiting nodes twice that appear
            // in the same tree. If we were to add all nodes to the stack that we
            // discovered, then we would have duplicate ids on the stack. Marking them
            // as visited immediately when adding them avoid this issue.
            *node_visited_mut = true;
        }
    }

    /// Discovers a tree starting at the given root index.
    fn discover_tree(&mut self, root_idx: MastNodeId) {
        let current_node = &self.mast_forest.nodes[root_idx.as_usize()];
        // Note that the order in which we add or discover nodes is the reverse of postorder, since
        // we're pushing them onto a stack, which reverses the order itself. Hence, reversing twice
        // gives us the actual postorder we want.
        match current_node {
            MastNode::Block(_) => {
                self.mark_for_visit(root_idx);
            },
            MastNode::Join(join_node) => {
                self.mark_for_visit(root_idx);
                self.discover_tree(join_node.second());
                self.discover_tree(join_node.first());
            },
            MastNode::Split(split_node) => {
                self.mark_for_visit(root_idx);
                self.discover_tree(split_node.on_false());
                self.discover_tree(split_node.on_true());
            },
            MastNode::Loop(loop_node) => {
                self.mark_for_visit(root_idx);
                self.discover_tree(loop_node.body());
            },
            MastNode::Call(call_node) => {
                self.mark_for_visit(root_idx);
                self.discover_tree(call_node.callee());
            },
            MastNode::Dyn(_) => {
                self.mark_for_visit(root_idx);
            },
            MastNode::External(_) => {
                self.mark_for_visit(root_idx);
            },
        }
    }

    /// Finds the next unvisited procedure root and discovers a tree from it.
    ///
    /// If the unvisited node stack is empty after calling this function, the iteration is complete.
    fn discover_nodes(&mut self) {
        if self.mast_forest.num_procedures() == 0 {
            return;
        }

        let procedure_roots = self.mast_forest.procedure_roots();
        // Find the next unvisited procedure root.
        while self.node_visited[procedure_roots[self.last_procedure_root_idx as usize].as_usize()] {
            if self.last_procedure_root_idx + 1 >= self.mast_forest.num_procedures() {
                return;
            }

            self.last_procedure_root_idx += 1;
        }

        let tree_root_id = procedure_roots[self.last_procedure_root_idx as usize];
        self.discover_tree(tree_root_id);
    }
}

impl<'forest> Iterator for MastForestNodeIter<'forest> {
    type Item = (MastNodeId, &'forest MastNode);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_node_id) = self.unvisited_node_stack.pop() {
            // SAFETY: We only add valid ids to the stack so it's fine to index the forest nodes
            // directly.
            let node = &self.mast_forest.nodes[next_node_id.as_usize()];
            return Some((next_node_id, node));
        }

        self.discover_nodes();

        if !self.unvisited_node_stack.is_empty() {
            self.next()
        } else {
            // If the stack is empty after tree discovery, all (reachable) nodes have been visited.
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use miden_crypto::hash::rpo::RpoDigest;

    use super::*;

    fn random_digest() -> RpoDigest {
        RpoDigest::new([rand_utils::rand_value(); 4])
    }

    #[test]
    fn mast_forest_dfs_empty() {
        let forest = MastForest::new();
        let mut iterator = forest.iter_nodes();
        assert!(iterator.next().is_none());
    }

    #[test]
    fn mast_forest_dfs() {
        let node0_digest = random_digest();
        let node1_digest = random_digest();
        let node2_digest = random_digest();
        let node3_digest = random_digest();

        let mut forest = MastForest::new();
        forest.add_external(node0_digest).unwrap();
        let id1 = forest.add_external(node1_digest).unwrap();
        let id2 = forest.add_external(node2_digest).unwrap();
        let id3 = forest.add_external(node3_digest).unwrap();
        let id_split = forest.add_split(id2, id3).unwrap();
        let id_join = forest.add_join(id2, id_split).unwrap();

        forest.make_root(id_join);
        forest.make_root(id1);

        // Note that the node at index 0 is not visited because it is not reachable from any root
        // and is not a root itself.
        let mut iterator = forest.iter_nodes();
        // Node at id2 should only be visited once.
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node2_digest && id == id2);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node3_digest && id == id3);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::Split(_)) if id == id_split);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::Join(_)) if id == id_join);
        assert_matches!(iterator.next().unwrap(), (id, MastNode::External(digest)) if digest.digest() == node1_digest&& id == id1);
        assert!(iterator.next().is_none());
    }
}
