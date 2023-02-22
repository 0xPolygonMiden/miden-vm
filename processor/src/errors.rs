use super::{
    system::{FMP_MAX, FMP_MIN},
    CodeBlock, Digest, Felt, MerkleError, Word,
};
use core::fmt::{Display, Formatter};
use vm_core::{stack::STACK_TOP_SIZE, utils::to_hex};
use winter_prover::{math::FieldElement, ProverError};

#[cfg(feature = "std")]
use std::error::Error;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ExecutionError {
    AdviceKeyNotFound(Word),
    AdviceTapeReadFailed(u32),
    MerkleSetLookupFailed(MerkleError),
    MerkleSetNotFound([u8; 32]),
    MerkleSetUpdateFailed(MerkleError),
    CodeBlockNotFound(Digest),
    CallerNotInSyscall,
    DivideByZero(u32),
    DuplicateAdviceKey(Word),
    FailedAssertion(u32),
    UninitializedMemoryAddress(u64),
    InvalidFmpValue(Felt, Felt),
    NttDomainSizeTooSmall(u64),
    NttDomainSizeNotPowerof2(u64),
    InvalidStackDepthOnReturn(usize),
    InterpolationResultSizeTooBig(usize, usize),
    NotBinaryValue(Felt),
    NotU32Value(Felt),
    ProverError(ProverError),
    SyscallTargetNotInKernel(Digest),
    UnexecutableCodeBlock(CodeBlock),
}

impl Display for ExecutionError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        use ExecutionError::*;

        match self {
            AdviceKeyNotFound(key) => {
                let hex = to_hex(Felt::elements_as_bytes(key))?;
                write!(fmt, "Can't write to advice tape: value for key {hex} not present in the advice map.")
            }
            MerkleSetLookupFailed(reason) => write!(fmt, "Advice set lookup failed: {reason}"),
            MerkleSetNotFound(root) => write!(fmt, "Advice set with root {root:x?} not found"),
            MerkleSetUpdateFailed(reason) => write!(fmt, "Advice set update failed: {reason}"),
            AdviceTapeReadFailed(step) => write!(fmt, "Advice tape read failed at step {step}"),
            CodeBlockNotFound(digest) => {
                let hex = to_hex(&digest.as_bytes())?;
                write!(
                    fmt,
                    "Failed to execute code block with root {hex}; the block could not be found"
                )
            }
            CallerNotInSyscall => {
                write!(
                    fmt,
                    "Instruction `caller` used outside of kernel context, this is not supported"
                )
            }
            DivideByZero(clk) => write!(fmt, "Division by zero at clock cycle {clk}"),
            DuplicateAdviceKey(key) => {
                let hex = to_hex(Felt::elements_as_bytes(key))?;
                write!(fmt, "Insertion into advice map failed because {hex} already exists")
            }
            FailedAssertion(clk) => write!(fmt, "Assertion failed at clock cycle {clk}"),
            UninitializedMemoryAddress(address) => {
                write!(fmt, "Ext2INTT referenced unintialized memory at address {address}")
            }
            InvalidFmpValue(old, new) => {
                write!(fmt, "Updating FMP register from {old} to {new} failed because {new} is outside of {FMP_MIN}..{FMP_MAX}")
            }
            NttDomainSizeTooSmall(v) => {
                write!(fmt, "Input NTT domain size ({v} elements) is too small")
            }
            NttDomainSizeNotPowerof2(v) => {
                write!(fmt, "Input NTT domain size must be a power of two, but was {v}")
            }
            InvalidStackDepthOnReturn(depth) => {
                write!(fmt, "When returning from a call, stack depth must be {STACK_TOP_SIZE}, but was {depth}")
            }
            InterpolationResultSizeTooBig(output_len, input_len) => {
                write!(
                    fmt,
                    "Interpolation output length ({output_len}) cannot be greater than the input length ({input_len})"
                )
            }
            NotBinaryValue(v) => {
                write!(
                    fmt,
                    "Execution failed: an operation expected a binary value, but received {v}"
                )
            }
            NotU32Value(v) => {
                write!(fmt, "Execution failed: an operation expected a u32 value, but received {v}")
            }
            ProverError(error) => write!(fmt, "Proof generation failed: {error}"),
            SyscallTargetNotInKernel(proc) => {
                let hex = to_hex(&proc.as_bytes())?;
                write!(fmt, "Syscall failed: procedure with root {hex} was not found in the kernel")
            }
            UnexecutableCodeBlock(block) => {
                write!(fmt, "Execution reached unexecutable code block {block:?}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for ExecutionError {}
