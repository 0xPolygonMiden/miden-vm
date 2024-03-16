use super::{
    crypto::MerkleError,
    system::{FMP_MAX, FMP_MIN},
    CodeBlock, Digest, Felt, QuadFelt, Word,
};
use alloc::string::String;
use core::fmt::{Display, Formatter};
use vm_core::{stack::STACK_TOP_SIZE, utils::to_hex};
use winter_prover::{math::FieldElement, ProverError};

#[cfg(feature = "std")]
use std::error::Error;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionError {
    AdviceMapKeyNotFound(Word),
    AdviceStackReadFailed(u32),
    CallerNotInSyscall,
    CodeBlockNotFound(Digest),
    CycleLimitExceeded(u32),
    DivideByZero(u32),
    DynamicCodeBlockNotFound(Digest),
    EventError(String),
    Ext2InttError(Ext2InttError),
    FailedAssertion {
        clk: u32,
        err_code: u32,
        err_msg: Option<String>,
    },
    FailedSignatureGeneration(&'static str),
    InvalidFmpValue(Felt, Felt),
    InvalidFriDomainSegment(u64),
    InvalidFriLayerFolding(QuadFelt, QuadFelt),
    InvalidMemoryRange {
        start_addr: u64,
        end_addr: u64,
    },
    InvalidStackDepthOnReturn(usize),
    InvalidStackWordOffset(usize),
    InvalidTreeDepth {
        depth: Felt,
    },
    InvalidTreeNodeIndex {
        depth: Felt,
        value: Felt,
    },
    LogArgumentZero(u32),
    MalformedSignatureKey(&'static str),
    MemoryAddressOutOfBounds(u64),
    MerklePathVerificationFailed {
        value: Word,
        index: Felt,
        root: Digest,
    },
    MerkleStoreLookupFailed(MerkleError),
    MerkleStoreMergeFailed(MerkleError),
    MerkleStoreUpdateFailed(MerkleError),
    NotBinaryValue(Felt),
    NotU32Value(Felt, Felt),
    ProverError(ProverError),
    SmtNodeNotFound(Word),
    SmtNodePreImageNotValid(Word, usize),
    SyscallTargetNotInKernel(Digest),
    UnexecutableCodeBlock(CodeBlock),
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        use ExecutionError::*;

        match self {
            AdviceMapKeyNotFound(key) => {
                let hex = to_hex(Felt::elements_as_bytes(key))?;
                write!(f, "Value for key {hex} not present in the advice map")
            }
            AdviceStackReadFailed(step) => write!(f, "Advice stack read failed at step {step}"),
            CallerNotInSyscall => {
                write!(f, "Instruction `caller` used outside of kernel context")
            }
            CodeBlockNotFound(digest) => {
                let hex = to_hex(&digest.as_bytes())?;
                write!(
                    f,
                    "Failed to execute code block with root {hex}; the block could not be found"
                )
            }
            CycleLimitExceeded(max_cycles) => {
                write!(f, "Exceeded the allowed number of cycles (max cycles = {max_cycles})")
            }
            DivideByZero(clk) => write!(f, "Division by zero at clock cycle {clk}"),
            DynamicCodeBlockNotFound(digest) => {
                let hex = to_hex(&digest.as_bytes())?;
                write!(
                    f,
                    "Failed to execute the dynamic code block provided by the stack with root {hex}; the block could not be found"
                )
            }
            EventError(error) => write!(f, "Failed to process event - {error}"),
            Ext2InttError(err) => write!(f, "Failed to execute Ext2Intt operation: {err}"),
            FailedAssertion {
                clk,
                err_code,
                err_msg,
            } => {
                if let Some(err_msg) = err_msg {
                    write!(
                        f,
                        "Assertion failed at clock cycle {clk} with error code {err_code}: {err_msg}"
                    )
                } else {
                    write!(f, "Assertion failed at clock cycle {clk} with error code {err_code}")
                }
            }
            FailedSignatureGeneration(signature) => {
                write!(f, "Failed to generate signature: {signature}")
            }
            InvalidFmpValue(old, new) => {
                write!(f, "Updating FMP register from {old} to {new} failed because {new} is outside of {FMP_MIN}..{FMP_MAX}")
            }
            InvalidFriDomainSegment(value) => {
                write!(f, "FRI domain segment value cannot exceed 3, but was {value}")
            }
            InvalidFriLayerFolding(expected, actual) => {
                write!(f, "Degree-respecting projection is inconsistent: expected {expected} but was {actual}")
            }
            InvalidMemoryRange {
                start_addr,
                end_addr,
            } => {
                write!(f, "Memory range start address cannot exceed end address, but was ({start_addr}, {end_addr})")
            }
            InvalidStackDepthOnReturn(depth) => {
                write!(f, "When returning from a call, stack depth must be {STACK_TOP_SIZE}, but was {depth}")
            }
            InvalidStackWordOffset(offset) => {
                write!(f, "Stack word offset cannot exceed 12, but was {offset}")
            }
            InvalidTreeDepth { depth } => {
                write!(f, "The provided {depth} is out of bounds and cannot be represented as an unsigned 8-bits integer")
            }
            InvalidTreeNodeIndex { depth, value } => {
                write!(f, "The provided index {value} is out of bounds for a node at depth {depth}")
            }
            LogArgumentZero(clk) => {
                write!(
                    f,
                    "Calculating of the integer logarithm with zero argument at clock cycle {clk}"
                )
            }
            MalformedSignatureKey(signature) => write!(f, "Malformed signature key: {signature}"),
            MemoryAddressOutOfBounds(addr) => {
                write!(f, "Memory address cannot exceed 2^32 but was {addr}")
            }
            MerklePathVerificationFailed { value, index, root } => {
                let value = to_hex(Felt::elements_as_bytes(value))?;
                let root = to_hex(&root.as_bytes())?;
                write!(f, "Merkle path verification failed for value {value} at index {index}, in the Merkle tree with root {root}")
            }
            MerkleStoreLookupFailed(reason) => {
                write!(f, "Advice provider Merkle store backend lookup failed: {reason}")
            }
            MerkleStoreMergeFailed(reason) => {
                write!(f, "Advice provider Merkle store backend merge failed: {reason}")
            }
            MerkleStoreUpdateFailed(reason) => {
                write!(f, "Advice provider Merkle store backend update failed: {reason}")
            }
            NotBinaryValue(v) => {
                write!(f, "An operation expected a binary value, but received {v}")
            }
            NotU32Value(v, err_code) => {
                write!(
                    f,
                    "An operation expected a u32 value, but received {v} (error code: {err_code})"
                )
            }
            SmtNodeNotFound(node) => {
                let node_hex = to_hex(Felt::elements_as_bytes(node))?;
                write!(f, "Smt node {node_hex} not found")
            }
            SmtNodePreImageNotValid(node, preimage_len) => {
                let node_hex = to_hex(Felt::elements_as_bytes(node))?;
                write!(f, "Invalid pre-image for node {node_hex}. Expected pre-image length to be a multiple of 8, but was {preimage_len}")
            }
            ProverError(error) => write!(f, "Proof generation failed: {error}"),
            SyscallTargetNotInKernel(proc) => {
                let hex = to_hex(&proc.as_bytes())?;
                write!(f, "Syscall failed: procedure with root {hex} was not found in the kernel")
            }
            UnexecutableCodeBlock(block) => {
                write!(f, "Execution reached unexecutable code block {block:?}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for ExecutionError {}

impl From<Ext2InttError> for ExecutionError {
    fn from(value: Ext2InttError) -> Self {
        Self::Ext2InttError(value)
    }
}

// EXT2INTT ERROR
// ================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum Ext2InttError {
    DomainSizeNotPowerOf2(u64),
    DomainSizeTooSmall(u64),
    InputEndAddressTooBig(u64),
    InputSizeTooBig(u64),
    InputStartAddressTooBig(u64),
    OutputSizeTooBig(usize, usize),
    OutputSizeIsZero,
    UninitializedMemoryAddress(u32),
}

impl Display for Ext2InttError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        use Ext2InttError::*;

        match self {
            DomainSizeNotPowerOf2(v) => {
                write!(f, "input domain size must be a power of two, but was {v}")
            }
            DomainSizeTooSmall(v) => {
                write!(f, "input domain size ({v} elements) is too small")
            }
            InputEndAddressTooBig(addr) => {
                write!(f, "address of the last input must be smaller than 2^32, but was {addr}")
            }
            InputSizeTooBig(size) => {
                write!(f, "input size must be smaller than 2^32, but was {size}")
            }
            InputStartAddressTooBig(addr) => {
                write!(f, "address of the first input must be smaller than 2^32, but was {addr}")
            }
            OutputSizeIsZero => {
                write!(f, "output size must be greater than 0")
            }
            OutputSizeTooBig(output_size, input_size) => {
                write!(
                    f,
                    "output size ({output_size}) cannot be greater than the input size ({input_size})"
                )
            }

            UninitializedMemoryAddress(address) => {
                write!(f, "uninitialized memory at address {address}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for Ext2InttError {}
