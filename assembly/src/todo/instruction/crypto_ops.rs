use core::iter;

use vm_core::{code_blocks::CodeBlock, AdviceInjector, Decorator, Operation::*};

use crate::{todo::span_builder::SpanBuilder, AssemblerError};

// MERKLE TREES
// ================================================================================================

pub(super) fn mtree_get(span: &mut SpanBuilder) -> Result<Option<CodeBlock>, AssemblerError> {
    // The stack should be arranged in the following way: [d, i, R, ...] so that the decorator
    // can fetch the node value from the root. In the `mtree.get` operation we have the stack in
    // the following format: [d, i, R], whereas in the case of `mtree.set` and `mtree.cwm` we
    // would also have the new node value post the tree root: [d, i, R, V_new].
    //
    // inject the node value we're looking for at the head of the advice tape.
    span.push_decorator(Decorator::Advice(AdviceInjector::MerkleNode));

    // read old node value from advice tape => MPVERIFY: [V_old, d, i, R, ...]
    // MRUPDATE: [V_old, d, i, R, V_new, ...]
    let pre = iter::repeat(Read).take(4);

    let post = [
        // verify the node V for root R with depth d and index i
        // => [V, d, i, R, ...]
        MpVerify,
        // move d, i back to the top of the stack and are dropped since they are
        // no longer needed => [V, R, ...]
        MovUp4, Drop, MovUp4, Drop,
    ]
    .into_iter();

    span.add_ops(pre.chain(post))
}
