use super::{AssemblyError, CodeBlock, Operation::*, SpanBuilder};
use vm_core::{AdviceInjector, Decorator, Felt};

// HASHING
// ================================================================================================
// The number of elements to be hashed by the rphash operation
const RPHASH_NUM_ELEMENTS: u64 = 8;

/// Appends RPPERM and stack manipulation operations to the span block as required to compute a
/// 2-to-1 Rescue Prime hash. The top of the stack is expected to be arranged with 2 words
/// (8 elements) to be hashed: [B, A, ...]. The resulting stack will contain the 2-to-1 hash result
/// [E, ...].
///
/// This assembly operation uses the VM operation RPPERM at its core, which permutes the top 12
/// elements of the stack.
///
/// To perform the operation, we do the following:
/// 1. Prepare the stack with 12 elements for RPPERM by pushing 4 more elements for the capacity,
///    including the number of elements to be hashed (8), so the stack looks like [C, B, A, ...]
///    where C is the capacity, and the number of elements is the deepest element in C.
/// 2. Reorder the stack so the capacity is deepest in the stack [B, A, C, ...]
/// 3. Append the RPPERM operation, which performs a Rescue Prime permutation on the top 12
///    elements and leaves an output of [F, E, D, ...] on the stack. E is our 2-to-1 hash result.
/// 4. Drop F and D to return our result [E, ...].
///
/// This operation takes 16 VM cycles.
pub(super) fn rphash(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // Add 4 elements to the stack to prepare the capacity portion for the Rescue Prime permutation
    // The capacity should start at stack[8], and the number of elements to be hashed should
    // be deepest in the stack at stack[11]
    span.push_op(Push(Felt::new(RPHASH_NUM_ELEMENTS)));
    span.push_op_many(Pad, 3);
    span.push_op(SwapW2);
    // restore the order of the top 2 words to be hashed
    span.push_op(SwapW);

    // Do the Rescue Prime permutation on the top 12 elements in the stack
    span.push_op(RpPerm);

    // Drop 4 elements (the part of the rate that doesn't have our result)
    span.push_op_many(Drop, 4);

    // Move the top word (our result) down the stack
    span.push_op(SwapW);

    // Drop 4 elements (the capacity portion)
    span.push_op_many(Drop, 4);

    Ok(None)
}

// MERKLE TREES
// ================================================================================================

/// Appends the MPVERIFY op and stack manipulations to the span block as required to verify that a
/// Merkle tree with root R opens to node V at depth d and index i. The stack is expected to be
/// arranged as follows (from the top):
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - current root of the tree, 4 elements
///
/// After the operations are executed, the stack will be arranged as follows:
/// - node V, 4 elements
/// - root of the tree, 4 elements.
///
/// This operation takes 9 VM cycles.
pub(super) fn mtree_get(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // stack: [d, i, R, ...]
    // inject the node value we're looking for at the head of the advice tape
    read_mtree_node(span);

    // verify the node V for root R with depth d and index i
    // => [V, d, i, R, ...]
    span.push_op(MpVerify);

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [V, R, ...]
    span.push_op(MovUp4);
    span.push_op(Drop);
    span.push_op(MovUp4);
    span.push_op(Drop);

    Ok(None)
}

/// Appends the MRUPDATE op with a parameter of "false" and stack manipulations to the span block
/// as required to update a node in the Merkle tree with root R at depth d and index i to value V.
/// The stack is expected to be arranged as follows (from the top):
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - current root of the tree, 4 elements
/// - new value of the node, 4 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - new root of the tree after the update, 4 elements
/// - new value of the node, 4 elements
///
/// This operation takes 14 VM cycles.
pub(super) fn mtree_set(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // Inject the old node value onto the stack for the call to MRUPDATE.
    // [d, i, R, V_new, ...] => [V_old, d, i, R, V_new, ...]
    read_mtree_node(span);

    // Update the Merkle tree with the new value without copying the old tree. This replaces the
    // old node value with the computed new Merkle root.
    // => [R_new, d, i, R, V_new, ...]
    span.push_op(MrUpdate(false));

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [R_new, R, V_new, ...]
    span.push_op(MovUp4);
    span.push_op(Drop);
    span.push_op(MovUp4);
    span.push_op(Drop);

    // Move the old root to the top of the stack => [R, R_new, V_new, ...]
    span.push_op(SwapW);

    // Drop old root from the stack => [R_new, V_new ...]
    span.push_op_many(Drop, 4);

    Ok(None)
}

/// Appends the MRUPDATE op with a parameter of "true" and stack manipulations to the span block as
/// required to copy a Merkle tree with root R and update the node in the copied tree at depth d
/// and index i to value V. The stack is expected to be arranged as follows (from the top):
/// - depth of the node, 1 element; this is expected to be the depth of the Merkle tree
/// - index of the node, 1 element
/// - current root of the tree, 4 elements
/// - new value of the node, 4 element
///
/// After the operations are executed, the stack will be arranged as follows:
/// - new root of the tree after the update, 4 elements
/// - new value of the node, 4 elements
/// - root of the old tree which was copied, 4 elements
///
/// This operation takes 12 VM cycles.
pub(super) fn mtree_cwm(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // Inject the old node value onto the stack for the call to MRUPDATE.
    // [d, i, R, V_new, ...] => [V_old, d, i, R, V_new, ...]
    read_mtree_node(span);

    // update the Merkle tree with the new value and copy the old tree. This replaces the
    // old node value with the computed new Merkle root.
    // => [R_new, d, i, R, V_new, ...]
    span.push_op(MrUpdate(true));

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [R_new, R, V_new, ...]
    span.push_op(MovUp4);
    span.push_op(Drop);
    span.push_op(MovUp4);
    span.push_op(Drop);

    // Move the new value to the top => [R_new, V_new, R ...]
    span.push_op(SwapW);
    span.push_op(SwapW2);
    span.push_op(SwapW);

    Ok(None)
}

/// This is a helper function for assembly operations that fetches the node value from the
/// Merkle tree using decorators and pushes it onto the stack. It prepares the stack with the
/// elements expected by the VM's MPVERIFY & MRUPDATE operations.
/// The stack is expected to be arranged as follows (from the top):
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - root of the Merkle tree, 4 elements
/// - new value of the node, 4 elements (only in the case of mtree_set and mtree_cwm)
///
/// After the operations are executed, the stack will be arranged as follows:
/// - old value of the node, 4 elements
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - root of the Merkle tree, 4 elements
/// - new value of the node, 4 elements (only in the case of mtree_set and mtree_cwm)
///
/// This operation takes 4 VM cycles.
fn read_mtree_node(span: &mut SpanBuilder) {
    // The stack should be arranged in the following way: [d, i, R, ...] so that the decorator
    // can fetch the node value from the root. In the `mtree.get` operation we have the stack in
    // the following format: [d, i, R], whereas in the case of `mtree.set` and `mtree.cwm` we
    // would also have the new node value post the tree root: [d, i, R, V_new].
    //
    // inject the node value we're looking for at the head of the advice tape.
    span.push_decorator(Decorator::Advice(AdviceInjector::MerkleNode));

    // read old node value from advice tape => MPVERIFY: [V_old, d, i, R, ...]
    // MRUPDATE: [V_old, d, i, R, V_new, ...]
    span.push_op_many(Read, 4);
}
