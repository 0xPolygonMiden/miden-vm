use vm_core::{code_blocks::CodeBlock, Felt, FieldElement, Operation::*};

use crate::{todo::SpanBuilder, AssemblerError};

// ARITHMETIC OPERATIONS
// ================================================================================================

pub(super) fn add_imm(
    imm: &Felt,
    span: &mut SpanBuilder,
) -> Result<Option<CodeBlock>, AssemblerError> {
    if imm == &Felt::ONE {
        span.add_op(Incr)
    } else {
        span.add_ops(&[Push(*imm), Add])
    }
}
