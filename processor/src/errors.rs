use super::{AdviceSetError, CodeBlock, Felt};
use winterfell::ProverError;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    UnsupportedCodeBlock(CodeBlock),
    UnexecutableCodeBlock(CodeBlock),
    NotBinaryValue(Felt),
    DivideByZero(usize),
    FailedAssertion(usize),
    EmptyAdviceTape(usize),
    AdviceSetNotFound([u8; 32]),
    AdviceSetLookupFailed(AdviceSetError),
    AdviceSetUpdateFailed(AdviceSetError),
    InvalidFmpValue(Felt, Felt),
    NotU32Value(Felt),
    ProverError(ProverError),
    TooManyStackOutputs(usize),
}
