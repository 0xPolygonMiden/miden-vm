use vm_core::{errors::AdviceSetError, program::blocks::CodeBlock, BaseElement};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    UnsupportedCodeBlock(CodeBlock),
    UnexecutableCodeBlock(CodeBlock),
    NotBinaryValue(BaseElement),
    StackUnderflow(&'static str, usize),
    DivideByZero(usize),
    FailedAssertion(usize),
    EmptyAdviceTape(usize),
    AdviceSetNotFound([u8; 32]),
    AdviseSetLookupFailed(AdviceSetError),
}
