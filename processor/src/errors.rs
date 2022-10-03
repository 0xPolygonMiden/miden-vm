use super::{AdviceSetError, CodeBlock, Digest, Felt, Word};
use winterfell::ProverError;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    AdviceKeyNotFound(Word),
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
    SyscallTargetNotInKernel(Digest),
    UnexecutableCodeBlock(CodeBlock),
}
