use super::{AssemblyError, CodeBlock, Operation::*, SpanBuilder};
use vm_core::{AdviceInjector, Decorator};

// HASHING
// ================================================================================================

/// Appends HPERM and stack manipulation operations to the span block as required to compute a
/// 1-to-1 hash. The top of the stack is expected to be arranged with 1 word (4 elements) to be
/// hashed: [A, ...]. The resulting stack will contain the 1-to-1 hash result: [C, ...].
///
/// This assembly instruction uses the VM operation HPERM at its core, which permutes the top 12
/// elements of the stack.
///
/// To perform the operation we do the following:
/// 1. Prepare the stack with 12 elements for HPERM by pushing 4 more elements for the capacity,
///    then reordering the stack and pushing an additional 4 elements so that the stack looks
///    like: [0, 0, 0, 1, a3, a2, a1, a0, 0, 0, 0, 1, ...].  The first capacity element is set to
///    ONE as we are hashing a number of elements which is not a multiple of the rate width. We
///    also set the next element in the rate after `A` to ONE.  All other capacity and rate
///    elements are set to ZERO, in accordance with the RPO rules.
/// 2. Append the HPERM operation, which performs a permutation of RPO on the top 12 elements and
///    leaves the an output of [D, C, B, ...] on the stack.  C is our 1-to-1 has result.
/// 3. Drop D and B to achieve our result [C, ...]
///
/// This operation takes 20 VM cycles.
pub(super) fn hash(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        // add 4 elements to the stack to be used as the capacity elements for the RPO permutation
        Pad, Incr, Pad, Pad, Pad,

        // swap capacity elements such that they are below the elements to be hashed
        SwapW,

        // Duplicate capacity elements in the rate portion of the stack
        Dup7, Dup7, Dup7, Dup7,

        // Apply a hashing permutation on the top 12 elements in the stack
        HPerm,

        // Drop 4 elements (the part of the rate that doesn't have our result)
        Drop, Drop, Drop, Drop,

        // Move the top word (our result) down the stack
        SwapW,

        // Drop 4 elements (the capacity portion)
        Drop, Drop, Drop, Drop,
    ];
    span.add_ops(ops)
}

/// Appends HPERM and stack manipulation operations to the span block as required to compute a
/// 2-to-1 Rescue Prime Optimized hash. The top of the stack is expected to be arranged with 2 words
/// (8 elements) to be hashed: [B, A, ...]. The resulting stack will contain the 2-to-1 hash result
/// [E, ...].
///
/// This assembly operation uses the VM operation HPERM at its core, which permutes the top 12
/// elements of the stack.
///
/// To perform the operation, we do the following:
/// 1. Prepare the stack with 12 elements for HPERM by pushing 4 more elements for the capacity,
///    then reordering so the stack looks like [A, B, C, ...] where C is the capacity. All capacity
///    elements are set to ZERO, in accordance with the RPO padding rule for when the input length
///    is a multiple of the rate.
/// 2. Reorder the top 2 words to restore the order of the elements to be hashed to [B, A, C, ...].
/// 3. Append the HPERM operation, which performs a permutation of Rescue Prime Optimized on the
///    top 12 elements and leaves an output of [F, E, D, ...] on the stack. E is our 2-to-1 hash
///    result.
/// 4. Drop F and D to return our result [E, ...].
///
/// This operation takes 16 VM cycles.
pub(super) fn hmerge(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    #[rustfmt::skip]
    let ops = [
        // Add 4 elements to the stack to prepare the capacity portion for the RPO permutation
        // The capacity should start at stack[8], and the number of elements to be hashed should
        // be deepest in the stack at stack[11]
        Pad, Pad, Pad, Pad, SwapW2,

        // restore the order of the top 2 words to be hashed
        SwapW,

        // Do the RPO permutation on the top 12 elements in the stack
        HPerm,

        // Drop 4 elements (the part of the rate that doesn't have our result)
        Drop, Drop, Drop, Drop,

        // Move the top word (our result) down the stack
        SwapW,

        // Drop 4 elements (the capacity portion)
        Drop, Drop, Drop, Drop,
    ];
    span.add_ops(ops)
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
    // pops the value of the node we are looking for from the advice stack
    read_mtree_node(span);
    #[rustfmt::skip]
    let ops = [
        // verify the node V for root R with depth d and index i
        // => [V, d, i, R, ...]
        MpVerify,

        // move d, i back to the top of the stack and are dropped since they are
        // no longer needed => [V, R, ...]
        MovUp4, Drop, MovUp4, Drop,
    ];
    span.add_ops(ops)
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
/// - old value of the node, 4 elements
/// - new root of the tree after the update, 4 elements
///
/// This operation takes 29 VM cycles.
pub(super) fn mtree_set(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // stack: [d, i, R_old, V_new, ...]

    // stack: [V_old, R_new, ...] (29 cycles)
    update_mtree(span)
}

/// Creates a new Merkle tree in the advice provider by combining trees with the specified roots.
/// The stack is expected to be arranged as follows (from the top):
/// - root of the right tree, 4 elements
/// - root of the left tree, 4 elements
///
/// The operation will merge the Merkle trees with the provided roots, producing a new merged root
/// with incremented depth. After the operations are executed, the stack will be arranged as
/// follows:
/// - merged root, 4 elements
///
/// This operation will fail if either of the input roots doesn't exist as Merkle tree in the
/// advice provider.
///
/// This operation takes 16 VM cycles.
pub(super) fn mtree_merge(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // stack input:  [R_rhs, R_lhs, ...]
    // stack output: [R_merged, ...]

    // invoke the advice provider function to merge 2 Merkle trees defined by the roots on the top
    // of the operand stack
    span.push_decorator(Decorator::Advice(AdviceInjector::MerkleMerge));

    // perform the `hmerge`, updating the operand stack
    hmerge(span)
}

// MERKLE TREES - HELPERS
// ================================================================================================

/// This is a helper function for assembly operations that fetches the node value from the
/// Merkle tree using decorators and pushes it onto the stack. It prepares the stack with the
/// elements expected by the VM's MPVERIFY & MRUPDATE operations.
/// The stack is expected to be arranged as follows (from the top):
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - root of the Merkle tree, 4 elements
/// - new value of the node, 4 elements (only in the case of mtree_set)
///
/// After the operations are executed, the stack will be arranged as follows:
/// - old value of the node, 4 elements
/// - depth of the node, 1 element
/// - index of the node, 1 element
/// - root of the Merkle tree, 4 elements
/// - new value of the node, 4 elements (only in the case of mtree_set)
///
/// This operation takes 4 VM cycles.
fn read_mtree_node(span: &mut SpanBuilder) {
    // The stack should be arranged in the following way: [d, i, R, ...] so that the decorator
    // can fetch the node value from the root. In the `mtree.get` operation we have the stack in
    // the following format: [d, i, R], whereas in the case of `mtree.set` we would also have the
    // new node value post the tree root: [d, i, R, V_new]
    //
    // pops the value of the node we are looking for from the advice stack
    span.push_decorator(Decorator::Advice(AdviceInjector::MerkleNode));

    // pops the old node value from advice the stack => MPVERIFY: [V_old, d, i, R, ...]
    // MRUPDATE: [V_old, d, i, R, V_new, ...]
    span.push_op_many(AdvPop, 4);
}

/// Update a node in the merkle tree. This operation will always copy the tree into a new instance,
/// and perform the mutation on the copied tree.
///
/// This operation takes 29 VM cycles.
fn update_mtree(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblyError> {
    // stack: [d, i, R_old, V_new, ...]
    // output: [R_new, R_old, V_new, V_old, ...]

    // Inject the old node value onto the stack for the call to MRUPDATE.
    // stack: [V_old, d, i, R_old, V_new, ...] (4 cycles)
    read_mtree_node(span);

    #[rustfmt::skip]
    let ops = [
        // Note: The stack is 14 elements deep already. The existing ops manipulate up to depth 16,
        // so it's only possible to copy 2-elements at the time.

        // COPY V_old
        // These instructions will push the current copy of V_old down, and create a new word with
        // the same elements on the top of the stack.
        //
        // ========================================================================================
        // Renamed V_old => o<pos>, R_old => r<pos>, V_new => n<pos>
        // stack: [[o3, o2, o1, o0], [d, i, r3, r2], [r1, r0, n3, n2], n1, n0, ...]

        // Renamed V_old => o<pos>, R_old => r<pos>, V_new => n<pos>
        // stack: [[o3, o2, o1, o0], [d, i, r3, r2], [r1, r0, n3, n2], n1, n0, ...]

        // Move i then d up
        // stack: [[d, i, o3, o2], [o1, o0, r3, r2], [r1, r0, n3, n2], n1, n0, ...]
        MovUp5, MovUp5,

        // Copy half of the word, o0 then o1
        // stack: [[o1, o0, d, i], [o3, o2, o1, o0], [r3, r2, r1, r0], [n3, n2, n1, n0], ...]
        Dup5, Dup5,

        // Move the data down
        // stack: [[o1, o0, d, i], [r3, r2, r1, r0], [n3, n2, n1, n0], [o3, o2, o1, o0], ...]
        SwapDW, SwapW, SwapW2,

        // Copy the other half of the word, o2 then o3
        // stack: [[o3, o2, o1, o0], [d, i, r3, r2], [r1, r0, n3, n2,] [n1, n0, o3, o2], o1, o0, ...]
        Dup13, Dup13,

        // Update the Merkle tree
        // ========================================================================================

        // Update the node at depth `d` and position `i`. It will always copy the Merkle tree.
        // stack: [R_new, d, i, R_old, V_new, V_old, ...]
        MrUpdate,

        // Drop unecessary values
        // ========================================================================================

        // drop d and i since they are no longer needed
        // stack: [R_new, R_old, V_new, V_old, ...]
        MovUp4, Drop, MovUp4, Drop,

        // drop old Merkle root from the stack
        // stack: [R_new, V_new, V_old, ...]
        SwapW, Drop, Drop, Drop, Drop,

        // drop new value from stack
        // stack: [R_new, V_old, ...]
        SwapW, Drop, Drop, Drop, Drop,

        // move the V_old to the front
        // stack: [V_old, R_new, ...]
        SwapW
    ];

    // stack: [V_old, R_new, ...] (25 cycles)
    span.add_ops(ops)
}
