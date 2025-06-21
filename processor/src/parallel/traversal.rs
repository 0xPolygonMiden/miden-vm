use alloc::{sync::Arc, vec::Vec};

use vm_core::mast::{MastForest, MastNode, MastNodeId};

/// ExecutionTraversal is a structure that allows traversing the MastForest
/// in the order of execution, starting from the root node.
pub struct ExecutionTraversal {
    call_stack: Vec<MastNodeId>,
    mast_forest: Arc<MastForest>,
}

impl ExecutionTraversal {
    /// Creates a new ExecutionTraversal starting from the given entrypoint node.
    pub fn new_from_entrypoint(mast_forest: Arc<MastForest>, entrypoint: MastNodeId) -> Self {
        Self {
            call_stack: vec![entrypoint],
            mast_forest,
        }
    }

    /// Creates a new ExecutionTraversal from an existing call stack.
    ///
    /// # Panics if the call stack is empty.
    pub fn new_from_call_stack(mast_forest: Arc<MastForest>, call_stack: Vec<MastNodeId>) -> Self {
        assert!(!call_stack.is_empty(), "Call stack cannot be empty");
        Self { call_stack, mast_forest }
    }

    /// Returns the next node in the execution order without advancing the traversal.
    pub fn peek(&self) -> Option<MastNodeId> {
        self.call_stack.last().copied()
    }

    /// Advances to the next node in the execution order, where the node on top of the call stack is
    /// considered to be fully executed. Returns the next node ID if available, or None if the
    /// traversal is complete.
    pub fn advance(&mut self) -> Option<MastNodeId> {
        // Get the current node (without popping it yet)
        let current_node_id = self.call_stack.last().copied()?;

        self.call_stack.pop();
        self.advance_from_parent(current_node_id)
    }

    /// Helper method to advance after completing execution of a child node
    fn advance_from_parent(&mut self, executed_node_id: MastNodeId) -> Option<MastNodeId> {
        // If no parent, traversal is complete
        let parent_node_id = self.call_stack.last().copied()?;

        let parent_node = self
            .mast_forest
            .get_node_by_id(parent_node_id)
            .expect("Parent node should exist in the forest");

        match parent_node {
            MastNode::Block(_) => unreachable!(
                "Basic blocks are always leaf nodes, and therefore cannot have children."
            ),
            MastNode::Join(join_node) => {
                if executed_node_id == join_node.first() {
                    // If we just executed the first node of the join, execute the second node
                    self.call_stack.push(join_node.second());
                    Some(join_node.second())
                } else {
                    // If we executed the second node, we are done with this join - pop it and
                    // continue
                    self.call_stack.pop();
                    self.advance_from_parent(parent_node_id)
                }
            },
            MastNode::Split(_)
            | MastNode::Loop(_)
            | MastNode::Call(_)
            | MastNode::Dyn(_)
            | MastNode::External(_) => {
                // These nodes complete immediately after their children - pop and continue
                self.call_stack.pop();
                self.advance_from_parent(parent_node_id)
            },
        }
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;

    use vm_core::{
        Felt, Operation,
        mast::{MastForest, MastNode},
    };

    use super::*;

    /// Helper function to create a basic block node with simple operations
    fn create_basic_block(operations: Vec<Operation>) -> MastNode {
        MastNode::new_basic_block(operations, None).unwrap()
    }

    #[test]
    fn test_basic_block_traversal() {
        let mut forest = MastForest::new();

        // Create a simple basic block
        let block = create_basic_block(vec![Operation::Add, Operation::Mul]);
        let block_id = forest.add_node(block).unwrap();
        forest.make_root(block_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), block_id);

        // Test peek - should return the block itself
        assert_eq!(traversal.peek(), Some(block_id));

        // Test advance - should return None since basic blocks are leaf nodes
        assert_eq!(traversal.advance(), None);

        // After advance, peek should return None
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_join_node_traversal() {
        let mut forest = MastForest::new();

        // Create two basic blocks
        let block1 = create_basic_block(vec![Operation::Add]);
        let block2 = create_basic_block(vec![Operation::Mul]);

        let block1_id = forest.add_node(block1).unwrap();
        let block2_id = forest.add_node(block2).unwrap();

        // Create a join node
        let join_node = MastNode::new_join(block1_id, block2_id, &forest).unwrap();
        let join_id = forest.add_node(join_node).unwrap();
        forest.make_root(join_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), join_id);

        // Initial peek should return the join node
        assert_eq!(traversal.peek(), Some(join_id));

        // When advance() is called on a join node, it means the join (including both children)
        // has been fully executed, so the traversal should be complete
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_nested_join_nodes() {
        let mut forest = MastForest::new();

        // Create basic blocks
        let block1 = create_basic_block(vec![Operation::Add]);
        let block2 = create_basic_block(vec![Operation::Mul]);
        let block3 = create_basic_block(vec![Operation::Drop]);

        let block1_id = forest.add_node(block1).unwrap();
        let block2_id = forest.add_node(block2).unwrap();
        let block3_id = forest.add_node(block3).unwrap();

        // Create nested join: join1(block1, block2), join2(join1, block3)
        let join1 = MastNode::new_join(block1_id, block2_id, &forest).unwrap();
        let join1_id = forest.add_node(join1).unwrap();

        let join2 = MastNode::new_join(join1_id, block3_id, &forest).unwrap();
        let join2_id = forest.add_node(join2).unwrap();
        forest.make_root(join2_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), join2_id);

        // With corrected semantics, advance() means "fully executed"
        // So the traversal only surfaces the root join node
        assert_eq!(traversal.peek(), Some(join2_id));

        // When advance() is called, join2 (and all its children) are considered fully executed
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_split_node_traversal() {
        let mut forest = MastForest::new();

        // Create two basic blocks
        let true_block = create_basic_block(vec![Operation::Add]);
        let false_block = create_basic_block(vec![Operation::Mul]);

        let true_block_id = forest.add_node(true_block).unwrap();
        let false_block_id = forest.add_node(false_block).unwrap();

        // Create a split node
        let split_node = MastNode::new_split(true_block_id, false_block_id, &forest).unwrap();
        let split_id = forest.add_node(split_node).unwrap();
        forest.make_root(split_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), split_id);

        // Initial peek should return the split node
        assert_eq!(traversal.peek(), Some(split_id));

        // Since split nodes don't have predetermined execution order,
        // advance should complete immediately
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_loop_node_traversal() {
        let mut forest = MastForest::new();

        // Create a basic block for the loop body
        let body_block = create_basic_block(vec![Operation::Add, Operation::Drop]);
        let body_id = forest.add_node(body_block).unwrap();

        // Create a loop node
        let loop_node = MastNode::new_loop(body_id, &forest).unwrap();
        let loop_id = forest.add_node(loop_node).unwrap();
        forest.make_root(loop_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), loop_id);

        // Initial peek should return the loop node
        assert_eq!(traversal.peek(), Some(loop_id));

        // Since loop execution is runtime-dependent, advance should complete immediately
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_call_node_traversal() {
        let mut forest = MastForest::new();

        // Create a basic block to call
        let callee_block = create_basic_block(vec![Operation::Swap, Operation::Drop]);
        let callee_id = forest.add_node(callee_block).unwrap();

        // Create a call node
        let call_node = MastNode::new_call(callee_id, &forest).unwrap();
        let call_id = forest.add_node(call_node).unwrap();
        forest.make_root(call_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), call_id);

        // Initial peek should return the call node
        assert_eq!(traversal.peek(), Some(call_id));

        // Since call execution involves context switching, advance should complete immediately
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_dyn_node_traversal() {
        let mut forest = MastForest::new();

        // Create a dyn node
        let dyn_node = MastNode::new_dyn();
        let dyn_id = forest.add_node(dyn_node).unwrap();
        forest.make_root(dyn_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), dyn_id);

        // Initial peek should return the dyn node
        assert_eq!(traversal.peek(), Some(dyn_id));

        // Since dyn execution is runtime-dependent, advance should complete immediately
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_external_node_traversal() {
        let mut forest = MastForest::new();

        // Create an external node with a dummy digest
        use vm_core::crypto::hash::RpoDigest;
        let dummy_digest = RpoDigest::default();
        let external_node = MastNode::new_external(dummy_digest);
        let external_id = forest.add_node(external_node).unwrap();
        forest.make_root(external_id);

        let forest = Arc::new(forest);
        let mut traversal =
            ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), external_id);

        // Initial peek should return the external node
        assert_eq!(traversal.peek(), Some(external_id));

        // Since external nodes reference external procedures, advance should complete immediately
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_complex_nested_structure() {
        let mut forest = MastForest::new();

        // Create several basic blocks
        let block_a = create_basic_block(vec![Operation::Add]);
        let block_b = create_basic_block(vec![Operation::Mul]);
        let block_c = create_basic_block(vec![Operation::Drop]);
        let block_d = create_basic_block(vec![Operation::Swap]);

        let block_a_id = forest.add_node(block_a).unwrap();
        let block_b_id = forest.add_node(block_b).unwrap();
        let block_c_id = forest.add_node(block_c).unwrap();
        let block_d_id = forest.add_node(block_d).unwrap();

        // Create a complex structure: join(join(a, b), split(c, d))
        let join_ab = MastNode::new_join(block_a_id, block_b_id, &forest).unwrap();
        let join_ab_id = forest.add_node(join_ab).unwrap();

        let split_cd = MastNode::new_split(block_c_id, block_d_id, &forest).unwrap();
        let split_cd_id = forest.add_node(split_cd).unwrap();

        let root_join = MastNode::new_join(join_ab_id, split_cd_id, &forest).unwrap();
        let root_id = forest.add_node(root_join).unwrap();
        forest.make_root(root_id);

        let forest = Arc::new(forest);
        // Start execution with root_id, join_ab_id, and block_b_id on the call stack
        let mut traversal = ExecutionTraversal::new_from_call_stack(
            Arc::clone(&forest),
            vec![root_id, join_ab_id, block_b_id],
        );

        // Initial peek should return block b (top of call stack)
        assert_eq!(traversal.peek(), Some(block_b_id));

        // When advance() is called on block b, it should:
        // 1. Pop block_b (completing join_ab since block_b is the second child)
        // 2. Pop join_ab (since it's now complete)
        // 3. Since join_ab was the first child of root_join, push split_cd (the second child)
        assert_eq!(traversal.advance(), Some(split_cd_id));
        assert_eq!(traversal.peek(), Some(split_cd_id));

        // When advance() is called on split_cd, the traversal should complete
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_empty_traversal() {
        let mut forest = MastForest::new();

        // Create a basic block but don't add it to initialize traversal
        let block = create_basic_block(vec![Operation::Noop]);
        let block_id = forest.add_node(block).unwrap();
        forest.make_root(block_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), block_id);

        // Consume the traversal
        assert_eq!(traversal.peek(), Some(block_id));
        assert_eq!(traversal.advance(), None);

        // Test that subsequent operations work correctly on empty traversal
        assert_eq!(traversal.peek(), None);
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }

    #[test]
    fn test_single_node_forest() {
        let mut forest = MastForest::new();

        // Create a forest with just one node
        let single_block = create_basic_block(vec![Operation::Push(Felt::new(42))]);
        let block_id = forest.add_node(single_block).unwrap();
        forest.make_root(block_id);

        let forest = Arc::new(forest);
        let mut traversal = ExecutionTraversal::new_from_entrypoint(Arc::clone(&forest), block_id);

        // Test the complete lifecycle
        assert_eq!(traversal.peek(), Some(block_id));
        assert_eq!(traversal.advance(), None);
        assert_eq!(traversal.peek(), None);
    }
}
