use alloc::{boxed::Box, string::String};
use core::error::Error;

use miden_air::RowIndex;
use vm_core::{
    mast::{DecoratorId, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::to_hex,
    AdviceProviderError,
};
use winter_prover::{math::FieldElement, ProverError};

use super::{
    system::{FMP_MAX, FMP_MIN},
    Digest, Felt, QuadFelt, Word,
};
use crate::ContextId;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("advice provider error")]
    AdviceProviderError(
        #[from]
        #[source]
        AdviceProviderError,
    ),
    #[error("instruction `caller` used outside of kernel context")]
    CallerNotInSyscall,
    #[error("external node with mast root {0} resolved to an external node")]
    CircularExternalNode(Digest),
    #[error("exceeded the allowed number of max cycles {0}")]
    CycleLimitExceeded(u32),
    #[error("decorator id {decorator_id} does not exist in MAST forest")]
    DecoratorNotFoundInForest { decorator_id: DecoratorId },
    #[error("division by zero at clock cycle {0}")]
    DivideByZero(RowIndex),
    #[error("failed to execute the dynamic code block provided by the stack with root {hex}; the block could not be found",
      hex = to_hex(.0.as_bytes())
    )]
    DynamicNodeNotFound(Digest),
    #[error("error during processing of event in on_event handler")]
    EventError(#[source] Box<dyn Error + Send + Sync + 'static>),
    #[error("event handler not found for event {event_id} at clock cycle {clk}")]
    EventHandlerNotFound { event_id: u32, clk: RowIndex },
    #[error("an event handler for event id '{event_id}' is already registered")]
    EventHandlerAlreadyRegistered { event_id: u32 },
    #[error("assertion failed at clock cycle {clk} with error code {err_code}{}",
      match err_msg {
        Some(msg) => format!(": {msg}"),
        None => "".into()
      }
    )]
    FailedAssertion {
        clk: RowIndex,
        err_code: u32,
        err_msg: Option<String>,
    },
    #[error("memory address {addr} in context {ctx} was read and written, or written twice, in the same clock cycle {clk}")]
    IllegalMemoryAccess { ctx: ContextId, addr: u32, clk: Felt },
    #[error("Updating FMP register from {0} to {1} failed because {1} is outside of {FMP_MIN}..{FMP_MAX}")]
    InvalidFmpValue(Felt, Felt),
    #[error("FRI domain segment value cannot exceed 3, but was {0}")]
    InvalidFriDomainSegment(u64),
    #[error("degree-respecting projection is inconsistent: expected {0} but was {1}")]
    InvalidFriLayerFolding(QuadFelt, QuadFelt),
    #[error(
        "memory range start address cannot exceed end address, but was ({start_addr}, {end_addr})"
    )]
    InvalidMemoryRange { start_addr: u64, end_addr: u64 },
    #[error("when returning from a call, stack depth must be {MIN_STACK_DEPTH}, but was {0}")]
    InvalidStackDepthOnReturn(usize),
    #[error("attempted to calculate integer logarithm with zero argument at clock cycle {0}")]
    LogArgumentZero(RowIndex),
    #[error(
        "MAST forest in host indexed by procedure root {root_digest} doesn't contain that root"
    )]
    MalformedMastForestInHost { root_digest: Digest },
    #[error("node id {node_id} does not exist in MAST forest")]
    MastNodeNotFoundInForest { node_id: MastNodeId },
    #[error("no MAST forest contains the procedure with root digest {root_digest}")]
    NoMastForestWithProcedure { root_digest: Digest },
    #[error("memory address cannot exceed 2^32 but was {0}")]
    MemoryAddressOutOfBounds(u64),
    #[error(
        "word memory access at address {addr} in context {ctx} is unaligned at clock cycle {clk}"
    )]
    MemoryUnalignedWordAccess { addr: u32, ctx: ContextId, clk: Felt },
    // Note: we need this version as well because to handle advice provider calls, which don't
    // have access to the clock.
    #[error("word access at memory address {addr} in context {ctx} is unaligned")]
    MemoryUnalignedWordAccessNoClk { addr: u32, ctx: ContextId },
    #[error("merkle path verification failed for value {value} at index {index} in the Merkle tree with root {root} (error code: {err_code})", 
      value = to_hex(Felt::elements_as_bytes(value)),
      root = to_hex(root.as_bytes()),
    )]
    MerklePathVerificationFailed {
        value: Word,
        index: Felt,
        root: Digest,
        err_code: u32,
    },
    #[error("an operation expected a binary value, but received {0}")]
    NotBinaryValue(Felt),
    #[error("an operation expected a u32 value, but received {0} (error code: {1})")]
    NotU32Value(Felt, Felt),
    #[error("stack should have at most {MIN_STACK_DEPTH} elements at the end of program execution, but had {} elements", MIN_STACK_DEPTH + .0)]
    OutputStackOverflow(usize),
    #[error("a program has already been executed in this process")]
    ProgramAlreadyExecuted,
    #[error("proof generation failed")]
    ProverError(#[source] ProverError),
    #[error("smt node {node_hex} not found", node_hex = to_hex(Felt::elements_as_bytes(.0)))]
    SmtNodeNotFound(Word),
    #[error("expected pre-image length of node {node_hex} to be a multiple of 8 but was {preimage_len}",
      node_hex = to_hex(Felt::elements_as_bytes(.0)),
      preimage_len = .1
    )]
    SmtNodePreImageNotValid(Word, usize),
    #[error("syscall failed: procedure with root {hex} was not found in the kernel",
      hex = to_hex(.0.as_bytes())
    )]
    SyscallTargetNotInKernel(Digest),
}

#[cfg(test)]
mod error_assertions {
    use super::*;

    /// Asserts at compile time that the passed error has Send + Sync + 'static bounds.
    fn _assert_error_is_send_sync_static<E: core::error::Error + Send + Sync + 'static>(_: E) {}

    fn _assert_execution_error_bounds(err: ExecutionError) {
        _assert_error_is_send_sync_static(err);
    }
}
