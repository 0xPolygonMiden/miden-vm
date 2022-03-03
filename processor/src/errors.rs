use super::{AdviceSetError, CodeBlock, Felt};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    UnsupportedCodeBlock(CodeBlock),
    UnexecutableCodeBlock(CodeBlock),
    NotBinaryValue(Felt),
    StackUnderflow(&'static str, usize),
    DivideByZero(usize),
    FailedAssertion(usize),
    EmptyAdviceTape(usize),
    AdviceSetNotFound([u8; 32]),
    AdviceSetLookupFailed(AdviceSetError),
    AdviceSetUpdateFailed(AdviceSetError),
    InvalidFmpValue(Felt, Felt),
    NotU32Value(Felt),
}
