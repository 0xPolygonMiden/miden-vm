use super::{
    crypto::MerkleError,
    system::{FMP_MAX, FMP_MIN},
    CodeBlock, Digest, Felt, QuadFelt, Word,
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
    AdviceStackReadFailed(u32),
    InvalidNodeIndex { depth: Felt, value: Felt },
    MerkleUpdateInPlace,
    MerkleStoreLookupFailed(MerkleError),
    MerkleStoreUpdateFailed(MerkleError),
    MerkleStoreMergeFailed(MerkleError),
    CodeBlockNotFound(Digest),
    CallerNotInSyscall,
    DivideByZero(u32),
    DuplicateAdviceKey(Word),
    FailedAssertion(u32),
    UninitializedMemoryAddress(u64),
    InvalidFmpValue(Felt, Felt),
    InvalidFriDomainSegment(u64),
    InvalidFriLayerFolding(QuadFelt, QuadFelt),
    InvalidStackDepthOnReturn(usize),
    NttDomainSizeTooSmall(u64),
    NttDomainSizeNotPowerOf2(u64),
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
                write!(fmt, "Can't push values onto the advice stack: value for key {hex} not present in the advice map.")
            }
            InvalidNodeIndex { depth, value } => write!(
                fmt,
                "The provided index {value} is out of bounds for a node at depth {depth}"
            ),
            MerkleUpdateInPlace => write!(fmt, "Update in place is not supported"),
            MerkleStoreLookupFailed(reason) => {
                write!(fmt, "Advice provider Merkle store backend lookup failed: {reason}")
            }
            MerkleStoreUpdateFailed(reason) => {
                write!(fmt, "Advice provider Merkle store backend update failed: {reason}")
            }
            MerkleStoreMergeFailed(reason) => {
                write!(fmt, "Advice provider Merkle store backend merge failed: {reason}")
            }
            AdviceStackReadFailed(step) => write!(fmt, "Advice stack read failed at step {step}"),
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
            InvalidFriDomainSegment(value) => {
                write!(fmt, "FRI domain segment value cannot exceed 3, but was {value}")
            }
            InvalidFriLayerFolding(expected, actual) => {
                write!(fmt, "Degree-respecting projection is inconsistent: expected {expected} but was {actual}")
            }
            NttDomainSizeTooSmall(v) => {
                write!(fmt, "Input NTT domain size ({v} elements) is too small")
            }
            NttDomainSizeNotPowerOf2(v) => {
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
