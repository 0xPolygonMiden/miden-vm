use vm_core::{v1::program::blocks::CodeBlock, BaseElement};

// EXECUTION ERROR
// ================================================================================================

pub enum ExecutionError {
    UnsupportedCodeBlock(CodeBlock),
    UnexecutableCodeBlock(CodeBlock),
    NotBinaryValue(BaseElement),
}
