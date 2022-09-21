use super::{AdviceSetError, CodeBlock, Digest, Felt};
use winterfell::ProverError;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    AdviceSetLookupFailed(AdviceSetError),
    AdviceSetNotFound([u8; 32]),
    AdviceSetUpdateFailed(AdviceSetError),
    CodeBlockNotFound(Digest),
    DivideByZero(u32),
    EmptyAdviceTape(u32),
    FailedAssertion(u32),
    InvalidFmpValue(Felt, Felt),
    InvalidStackDepthOnReturn(usize),
    NotBinaryValue(Felt),
    NotU32Value(Felt),
    ProverError(ProverError),
    UnexecutableCodeBlock(CodeBlock),
}
