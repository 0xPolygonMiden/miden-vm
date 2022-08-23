use super::{super::validate_operation, AssemblyError, Felt, Operation, Token, Vec};
use vm_core::{utils::PushMany, AdviceInjector, Decorator, DecoratorList};

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
///
/// # Errors
/// Returns an AssemblyError if:
/// - the operation is malformed.
/// - an unrecognized operation is received (anything other than rphash).
pub(super) fn parse_rphash(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "rphash", 0);

    // Add 4 elements to the stack to prepare the capacity portion for the Rescue Prime permutation
    // The capacity should start at stack[8], and the number of elements to be hashed should
    // be deepest in the stack at stack[11]
    span_ops.push(Operation::Push(Felt::new(RPHASH_NUM_ELEMENTS)));
    span_ops.push_many(Operation::Pad, 3);
    span_ops.push(Operation::SwapW2);
    // restore the order of the top 2 words to be hashed
    span_ops.push(Operation::SwapW);

    // Do the Rescue Prime permutation on the top 12 elements in the stack
    span_ops.push(Operation::RpPerm);

    // Drop 4 elements (the part of the rate that doesn't have our result)
    span_ops.push_many(Operation::Drop, 4);

    // Move the top word (our result) down the stack
    span_ops.push(Operation::SwapW);

    // Drop 4 elements (the capacity portion)
    span_ops.push_many(Operation::Drop, 4);

    Ok(())
}

/// Appends an RPPERM operation to the span block, which performs a Rescue Prime permutation on the
/// top 12 elements of the stack.
///
/// This operation takes 1 VM cycle.
///
/// # Errors
/// Returns an AssemblyError if:
/// - the operation is malformed.
/// - an unrecognized operation is received (anything other than rpperm).
pub(super) fn parse_rpperm(span_ops: &mut Vec<Operation>, op: &Token) -> Result<(), AssemblyError> {
    validate_operation!(op, "rpperm", 0);

    // append the machine op to the span block
    span_ops.push(Operation::RpPerm);

    Ok(())
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
///
/// # Errors:
/// Returns an AssemblyError if the operation is malformed.
pub(super) fn parse_mtree_get(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    validate_operation!(op, "mtree_get", 0);

    // stack: [d, i, R, ...]
    // inject the node value we're looking for at the head of the advice tape
    read_mtree_node(span_ops, decorators);

    // verify the node V for root R with depth d and index i
    // => [V, d, i, R, ...]
    span_ops.push(Operation::MpVerify);

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [V, R, ...]
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);

    Ok(())
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
///
/// # Errors:
/// Returns an AssemblyError if the operation is malformed.
pub(super) fn parse_mtree_set(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    validate_operation!(op, "mtree_set", 0);

    // Inject the old node value onto the stack for the call to MRUPDATE.
    // [d, i, R, V_new, ...] => [V_old, d, i, R, V_new, ...]
    read_mtree_node(span_ops, decorators);

    // Update the Merkle tree with the new value without copying the old tree. This replaces the
    // old node value with the computed new Merkle root.
    // => [R_new, d, i, R, V_new, ...]
    span_ops.push(Operation::MrUpdate(false));

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [R_new, R, V_new, ...]
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);

    // Move the old root to the top of the stack => [R, R_new, V_new, ...]
    span_ops.push(Operation::SwapW);

    // Drop old root from the stack => [R_new, V_new ...]
    span_ops.push_many(Operation::Drop, 4);

    Ok(())
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
///
/// # Errors:
/// Returns an AssemblyError if the operation is malformed.
pub(super) fn parse_mtree_cwm(
    span_ops: &mut Vec<Operation>,
    op: &Token,
    decorators: &mut DecoratorList,
) -> Result<(), AssemblyError> {
    validate_operation!(op, "mtree_cwm", 0);

    // Inject the old node value onto the stack for the call to MRUPDATE.
    // [d, i, R, V_new, ...] => [V_old, d, i, R, V_new, ...]
    read_mtree_node(span_ops, decorators);

    // update the Merkle tree with the new value and copy the old tree. This replaces the
    // old node value with the computed new Merkle root.
    // => [R_new, d, i, R, V_new, ...]
    span_ops.push(Operation::MrUpdate(true));

    // move d, i back to the top of the stack and are dropped since they are
    // no longer needed => [R_new, R, V_new, ...]
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);
    span_ops.push(Operation::MovUp4);
    span_ops.push(Operation::Drop);

    // Move the new value to the top => [R_new, V_new, R ...]
    span_ops.push(Operation::SwapW);
    span_ops.push(Operation::SwapW2);
    span_ops.push(Operation::SwapW);

    Ok(())
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
fn read_mtree_node(span_ops: &mut Vec<Operation>, decorators: &mut DecoratorList) {
    // The stack should be arranged in the following way: [d, i, R, ...] so that the decorator
    // can fetch the node value from the root. In the `mtree.get` operation we have the stack in
    // the following format: [d, i, R], whereas in the case of `mtree.set` and `mtree.cwm` we
    // would also have the new node value post the tree root: [d, i, R, V_new].
    // 
    // inject the node value we're looking for at the head of the advice tape.
    decorators.push((
        span_ops.len(),
        Decorator::Advice(AdviceInjector::MerkleNode),
    ));

    // read old node value from advice tape => MPVERIFY: [V_old, d, i, R, ...]
    // MRUPDATE: [V_old, d, i, R, V_new, ...]
    span_ops.push_many(Operation::Read, 4);
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpperm() {
        let mut span_ops: Vec<Operation> = Vec::new();
        let op = Token::new("rpperm", 0);
        let expected = vec![Operation::RpPerm];

        parse_rpperm(&mut span_ops, &op).expect("Failed to parse rpperm");

        assert_eq!(span_ops, expected);
    }

    #[test]
    fn rpperm_invalid() {
        // parse_rpperm should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("rpperm.12", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_rpperm(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("rphash", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "rpperm");
        assert_eq!(
            parse_rpperm(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }

    #[test]
    fn rphash_invalid() {
        // parse_rphash should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let op_pos = 0;

        let op_too_long = Token::new("rphash.12", op_pos);
        let expected = AssemblyError::extra_param(&op_too_long);
        assert_eq!(
            parse_rphash(&mut span_ops, &op_too_long).unwrap_err(),
            expected
        );

        let op_mismatch = Token::new("rpperm", op_pos);
        let expected = AssemblyError::unexpected_token(&op_mismatch, "rphash");
        assert_eq!(
            parse_rphash(&mut span_ops, &op_mismatch).unwrap_err(),
            expected
        );
    }

    #[test]
    fn mtree_invalid() {
        // parse_mtree should return an error if called with an invalid or incorrect operation
        let mut span_ops: Vec<Operation> = Vec::new();
        let mut decorators = DecoratorList::new();
        let op_pos = 0;

        let op_mismatch = Token::new("mtree", op_pos);
        assert_eq!(
            parse_mtree_get(&mut span_ops, &op_mismatch, &mut decorators).unwrap_err(),
            AssemblyError::unexpected_token(&op_mismatch, "mtree_get")
        );
        assert_eq!(
            parse_mtree_set(&mut span_ops, &op_mismatch, &mut decorators).unwrap_err(),
            AssemblyError::unexpected_token(&op_mismatch, "mtree_set")
        );
        assert_eq!(
            parse_mtree_cwm(&mut span_ops, &op_mismatch, &mut decorators).unwrap_err(),
            AssemblyError::unexpected_token(&op_mismatch, "mtree_cwm")
        );

        let op_too_long1 = Token::new("mtree_get.12", op_pos);
        assert_eq!(
            parse_mtree_get(&mut span_ops, &op_too_long1, &mut decorators).unwrap_err(),
            AssemblyError::extra_param(&op_too_long1)
        );

        let op_too_long2 = Token::new("mtree_set.12", op_pos);
        assert_eq!(
            parse_mtree_set(&mut span_ops, &op_too_long2, &mut decorators).unwrap_err(),
            AssemblyError::extra_param(&op_too_long2)
        );

        let op_too_long3 = Token::new("mtree_cwm.12", op_pos);
        assert_eq!(
            parse_mtree_cwm(&mut span_ops, &op_too_long3, &mut decorators).unwrap_err(),
            AssemblyError::extra_param(&op_too_long3)
        );
    }
}
