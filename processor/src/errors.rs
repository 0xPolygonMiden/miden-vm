use alloc::{boxed::Box, string::String, sync::Arc};
use core::error::Error;

use miden_air::RowIndex;
use miette::Diagnostic;
use vm_core::{
    debuginfo::{SourceFile, SourceManager, SourceSpan},
    mast::{BasicBlockNode, DecoratorId, MastForest, MastNodeExt, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::to_hex,
};
use winter_prover::{ProverError, math::FieldElement};

use super::{
    Digest, Felt, QuadFelt, Word,
    crypto::MerkleError,
    system::{FMP_MAX, FMP_MIN},
};
use crate::MemoryError;

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ExecutionError {
    #[error("value for key {} not present in the advice map", to_hex(Felt::elements_as_bytes(.0)))]
    AdviceMapKeyNotFound(Word),
    #[error("value for key {} already present in the advice map", to_hex(Felt::elements_as_bytes(.0)))]
    AdviceMapKeyAlreadyPresent(Word),
    #[error("advice stack read failed at step {0}")]
    AdviceStackReadFailed(RowIndex),
    #[error("illegal use of instruction {0} while inside a syscall")]
    CallInSyscall(&'static str),
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
    #[error("failed to execute Ext2Intt operation: {0}")]
    Ext2InttError(Ext2InttError),
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
    #[error(
        "Updating FMP register from {0} to {1} failed because {1} is outside of {FMP_MIN}..{FMP_MAX}"
    )]
    InvalidFmpValue(Felt, Felt),
    #[error("FRI domain segment value cannot exceed 3, but was {0}")]
    InvalidFriDomainSegment(u64),
    #[error("degree-respecting projection is inconsistent: expected {0} but was {1}")]
    InvalidFriLayerFolding(QuadFelt, QuadFelt),
    #[error(
        "when returning from a call or dyncall, stack depth must be {MIN_STACK_DEPTH}, but was {depth}"
    )]
    #[diagnostic()]
    InvalidStackDepthOnReturn {
        #[label("when returning from this call site")]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        depth: usize,
    },
    #[error(
        "provided merkle tree {depth} is out of bounds and cannot be represented as an unsigned 8-bit integer"
    )]
    InvalidMerkleTreeDepth { depth: Felt },
    #[error("provided node index {value} is out of bounds for a merkle tree node at depth {depth}")]
    InvalidMerkleTreeNodeIndex { depth: Felt, value: Felt },
    #[error("attempted to calculate integer logarithm with zero argument at clock cycle {0}")]
    LogArgumentZero(RowIndex),
    #[error("malformed signature key: {0}")]
    MalformedSignatureKey(&'static str),
    #[error(
        "MAST forest in host indexed by procedure root {root_digest} doesn't contain that root"
    )]
    MalformedMastForestInHost { root_digest: Digest },
    #[error("node id {node_id} does not exist in MAST forest")]
    MastNodeNotFoundInForest { node_id: MastNodeId },
    #[error(transparent)]
    #[diagnostic(transparent)]
    MemoryError(MemoryError),
    #[error("no MAST forest contains the procedure with root digest {root_digest}")]
    NoMastForestWithProcedure { root_digest: Digest },
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
    #[error("advice provider Merkle store backend lookup failed")]
    MerkleStoreLookupFailed(#[source] MerkleError),
    #[error("advice provider Merkle store backend merge failed")]
    MerkleStoreMergeFailed(#[source] MerkleError),
    #[error("advice provider Merkle store backend update failed")]
    MerkleStoreUpdateFailed(#[source] MerkleError),
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

impl From<Ext2InttError> for ExecutionError {
    fn from(value: Ext2InttError) -> Self {
        Self::Ext2InttError(value)
    }
}

impl ExecutionError {
    pub fn invalid_stack_depth_on_return(
        depth: usize,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::InvalidStackDepthOnReturn { label, source_file, depth }
    }
}

impl AsRef<dyn Diagnostic> for ExecutionError {
    fn as_ref(&self) -> &(dyn Diagnostic + 'static) {
        self
    }
}

// EXT2INTT ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
pub enum Ext2InttError {
    #[error("input domain size must be a power of two, but was {0}")]
    DomainSizeNotPowerOf2(u64),
    #[error("input domain size ({0} elements) is too small")]
    DomainSizeTooSmall(u64),
    #[error("address of the last input must be smaller than 2^32, but was {0}")]
    InputEndAddressTooBig(u64),
    #[error("input size must be smaller than 2^32, but was {0}")]
    InputSizeTooBig(u64),
    #[error("address of the first input must be smaller than 2^32, but was {0}")]
    InputStartAddressTooBig(u64),
    #[error("address of the first input is not word aligned: {0}")]
    InputStartNotWordAligned(u64),
    #[error("output size ({0}) cannot be greater than the input size ({1})")]
    OutputSizeTooBig(usize, usize),
    #[error("output size must be greater than 0")]
    OutputSizeIsZero,
    #[error("uninitialized memory at address {0}")]
    UninitializedMemoryAddress(u32),
}

// ERROR CONTEXT
// ===============================================================================================

/// Context information to be used when reporting errors.
#[derive(Debug)]
pub struct ErrorContext<'a, N: MastNodeExt>(Option<ErrorContextImpl<'a, N>>);

impl<'a, N: MastNodeExt> ErrorContext<'a, N> {
    /// Creates a new error context for the specified node and source manager.
    ///
    /// This method should be used for all nodes except basic block nodes.
    pub fn new(
        mast_forest: &'a MastForest,
        node: &'a N,
        source_manager: Arc<dyn SourceManager>,
    ) -> Self {
        Self(Some(ErrorContextImpl::new(mast_forest, node, source_manager)))
    }

    /// Creates a new error context for the specified node and source manager.
    ///
    /// This method should be used for basic block nodes.
    pub fn new_with_op_idx(
        mast_forest: &'a MastForest,
        node: &'a N,
        source_manager: Arc<dyn SourceManager>,
        op_idx: usize,
    ) -> Self {
        Self(Some(ErrorContextImpl::new_with_op_idx(
            mast_forest,
            node,
            source_manager,
            op_idx,
        )))
    }

    /// Creates a new empty error context.
    ///
    /// This error context will not provide any information about the source of the error.
    pub fn none() -> Self {
        Self(None)
    }

    /// Returns the label and source file associated with the error context, if any.
    ///
    /// Note that `SourceSpan::UNKNOWN` will be returned to indicate an empty span.
    pub fn label_and_source_file(&self) -> (SourceSpan, Option<Arc<SourceFile>>) {
        self.0
            .as_ref()
            .map_or((SourceSpan::UNKNOWN, None), |ctx| ctx.label_and_source_file())
    }
}

impl Default for ErrorContext<'_, BasicBlockNode> {
    fn default() -> Self {
        Self::none()
    }
}

#[derive(Debug)]
struct ErrorContextImpl<'a, N: MastNodeExt> {
    mast_forest: &'a MastForest,
    node: &'a N,
    source_manager: Arc<dyn SourceManager>,
    op_idx: Option<usize>,
}

impl<'a, N: MastNodeExt> ErrorContextImpl<'a, N> {
    pub fn new(
        mast_forest: &'a MastForest,
        node: &'a N,
        source_manager: Arc<dyn SourceManager>,
    ) -> Self {
        Self {
            mast_forest,
            node,
            source_manager,
            op_idx: None,
        }
    }

    pub fn new_with_op_idx(
        mast_forest: &'a MastForest,
        node: &'a N,
        source_manager: Arc<dyn SourceManager>,
        op_idx: usize,
    ) -> Self {
        Self {
            mast_forest,
            node,
            source_manager,
            op_idx: Some(op_idx),
        }
    }

    pub fn label_and_source_file(&self) -> (SourceSpan, Option<Arc<SourceFile>>) {
        self.node
            .get_assembly_op(self.mast_forest, self.op_idx)
            .and_then(|assembly_op| assembly_op.location())
            .map_or_else(
                || (SourceSpan::UNKNOWN, None),
                |location| {
                    (
                        self.source_manager.location_to_span(location.clone()).unwrap_or_default(),
                        self.source_manager.get_by_path(&location.path),
                    )
                },
            )
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod error_assertions {
    use super::*;

    /// Asserts at compile time that the passed error has Send + Sync + 'static bounds.
    fn _assert_error_is_send_sync_static<E: core::error::Error + Send + Sync + 'static>(_: E) {}

    fn _assert_execution_error_bounds(err: ExecutionError) {
        _assert_error_is_send_sync_static(err);
    }
}
