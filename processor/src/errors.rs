use super::{AdviceSetError, BaseElement, CodeBlock};

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
    AdviceSetLookupFailed(AdviceSetError),
    AdviceSetUpdateFailed(AdviceSetError),
}
