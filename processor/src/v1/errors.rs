use vm_core::v1::{program::blocks::CodeBlock, BaseElement};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    UnsupportedCodeBlock(CodeBlock),
    UnexecutableCodeBlock(CodeBlock),
    NotBinaryValue(BaseElement),
    StackUnderflow(&'static str, usize),
}
