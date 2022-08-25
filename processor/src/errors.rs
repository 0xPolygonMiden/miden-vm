use super::{AdviceSetError, CodeBlock, Digest, Felt};
use winterfell::ProverError;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    AdviceSetLookupFailed(AdviceSetError),
    AdviceSetNotFound([u8; 32]),
    AdviceSetUpdateFailed(AdviceSetError),
    DivideByZero(u32),
    EmptyAdviceTape(u32),
    FailedAssertion(u32),
    InvalidFmpValue(Felt, Felt),
    InvalidPowerOfTwo(Felt),
    NotBinaryValue(Felt),
    NotU32Value(Felt),
    ProverError(ProverError),
    TooManyStackOutputs(usize),
    UnexecutableCodeBlock(CodeBlock),
    CodeBlockNotFound(Digest),
}
