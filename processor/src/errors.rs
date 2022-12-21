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
    AdviceTapeReadFailed(u32),
    CodeBlockNotFound(Digest),
    CallerNotInSyscall,
    DivideByZero(u32),
    DuplicateAdviceKey(Word),
    FailedAssertion(u32),
    UninitializedMemoryAddress(u64),
    InvalidFmpValue(Felt, Felt),
    NttDomainSizeNotPowerof2(u64),
    InvalidStackDepthOnReturn(usize),
    NotBinaryValue(Felt),
    NotU32Value(Felt),
    ProverError(ProverError),
    SyscallTargetNotInKernel(Digest),
    UnexecutableCodeBlock(CodeBlock),
}
