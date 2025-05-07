use alloc::{boxed::Box, sync::Arc, vec::Vec};
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
    #[error("value for key {} not present in the advice map", to_hex(Felt::elements_as_bytes(.key)))]
    #[diagnostic()]
    AdviceMapKeyNotFound {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        key: Word,
    },
    #[error("value for key {} already present in the advice map when loading MAST forest", to_hex(Felt::elements_as_bytes(.key)))]
    #[diagnostic(help(
        "previous values at key were '{prev_values:?}'. Operation would have replaced them with '{new_values:?}'",
    ))]
    AdviceMapKeyAlreadyPresent {
        key: Word,
        prev_values: Vec<Felt>,
        new_values: Vec<Felt>,
    },
    #[error("advice stack read failed at clock cycle {row}")]
    #[diagnostic()]
    AdviceStackReadFailed {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        row: RowIndex,
    },
    /// This error is caught by the assembler, so we don't need diagnostics here.
    #[error("illegal use of instruction {0} while inside a syscall")]
    CallInSyscall(&'static str),
    /// This error is caught by the assembler, so we don't need diagnostics here.
    #[error("instruction `caller` used outside of kernel context")]
    CallerNotInSyscall,
    #[error("external node with mast root {0} resolved to an external node")]
    CircularExternalNode(Digest),
    #[error("exceeded the allowed number of max cycles {0}")]
    CycleLimitExceeded(u32),
    #[error("decorator id {decorator_id} does not exist in MAST forest")]
    DecoratorNotFoundInForest { decorator_id: DecoratorId },
    #[error("division by zero at clock cycle {clk}")]
    #[diagnostic()]
    DivideByZero {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        clk: RowIndex,
    },
    #[error("failed to execute the dynamic code block provided by the stack with root 0x{hex}; the block could not be found",
      hex = to_hex(.digest.as_bytes())
    )]
    #[diagnostic()]
    DynamicNodeNotFound {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        digest: Digest,
    },
    #[error("error during processing of event in on_event handler")]
    #[diagnostic()]
    EventError {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        #[source]
        error: Box<dyn Error + Send + Sync + 'static>,
    },
    #[error("failed to execute Ext2Intt operation: {0}")]
    Ext2InttError(Ext2InttError),
    #[error("assertion failed at clock cycle {clk} with error {}",
      match err_msg {
        Some(msg) => format!("message: {msg}"),
        None => format!("code: {err_code}"),
      }
    )]
    #[diagnostic()]
    FailedAssertion {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        clk: RowIndex,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
    },
    #[error("failed to execute the program for internal reason: {0}")]
    FailedToExecuteProgram(&'static str),
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
    #[diagnostic()]
    InvalidMerkleTreeDepth {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        depth: Felt,
    },
    #[error("provided node index {index} is out of bounds for a merkle tree node at depth {depth}")]
    #[diagnostic()]
    InvalidMerkleTreeNodeIndex {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        depth: Felt,
        index: Felt,
    },
    #[error("attempted to calculate integer logarithm with zero argument at clock cycle {clk}")]
    #[diagnostic()]
    LogArgumentZero {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        clk: RowIndex,
    },
    #[error("malformed signature key: {key_type}")]
    #[diagnostic(help("the secret key associated with the provided public key is malformed"))]
    MalformedSignatureKey {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        key_type: &'static str,
    },
    #[error(
        "MAST forest in host indexed by procedure root {root_digest} doesn't contain that root"
    )]
    MalformedMastForestInHost {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        root_digest: Digest,
    },
    #[error("node id {node_id} does not exist in MAST forest")]
    MastNodeNotFoundInForest { node_id: MastNodeId },
    #[error(transparent)]
    #[diagnostic(transparent)]
    MemoryError(MemoryError),
    #[error("no MAST forest contains the procedure with root digest {root_digest}")]
    NoMastForestWithProcedure {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        root_digest: Digest,
    },
    #[error("merkle path verification failed for value {value} at index {index} in the Merkle tree with root {root} (error {err})",
      value = to_hex(Felt::elements_as_bytes(value)),
      root = to_hex(root.as_bytes()),
      err = match err_msg {
        Some(msg) => format!("message: {msg}"),
        None => format!("code: {err_code}"),
      }
    )]
    MerklePathVerificationFailed {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        value: Word,
        index: Felt,
        root: Digest,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
    },
    #[error("failed to lookup value in Merkle store")]
    MerkleStoreLookupFailed {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        #[source]
        err: MerkleError,
    },
    #[error("advice provider Merkle store backend merge failed")]
    MerkleStoreMergeFailed {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        #[source]
        err: MerkleError,
    },
    #[error("advice provider Merkle store backend update failed")]
    MerkleStoreUpdateFailed {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        #[source]
        err: MerkleError,
    },
    #[error("if statement expected a binary value on top of the stack, but got {value}")]
    #[diagnostic()]
    NotBinaryValueIf {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        value: Felt,
    },
    #[error("operation expected a binary value, but got {value}")]
    #[diagnostic()]
    NotBinaryValueOp {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        value: Felt,
    },
    #[error("loop condition must be a binary value, but got {value}")]
    #[diagnostic(help(
        "this could happen either when first entering the loop, or any subsequent iteration"
    ))]
    NotBinaryValueLoop {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        value: Felt,
    },
    #[error("operation expected a u32 value, but got {value} (error code: {err_code})")]
    NotU32Value {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        value: Felt,
        err_code: Felt,
    },
    #[error("stack should have at most {MIN_STACK_DEPTH} elements at the end of program execution, but had {} elements", MIN_STACK_DEPTH + .0)]
    OutputStackOverflow(usize),
    #[error("a program has already been executed in this process")]
    ProgramAlreadyExecuted,
    #[error("proof generation failed")]
    ProverError(#[source] ProverError),
    #[error("smt node {node_hex} not found", node_hex = to_hex(Felt::elements_as_bytes(node)))]
    SmtNodeNotFound {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        node: Word,
    },
    #[error("expected pre-image length of node {node_hex} to be a multiple of 8 but was {preimage_len}",
      node_hex = to_hex(Felt::elements_as_bytes(node)),
    )]
    SmtNodePreImageNotValid {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        node: Word,
        preimage_len: usize,
    },
    #[error("syscall failed: procedure with root {hex} was not found in the kernel",
      hex = to_hex(proc_root.as_bytes())
    )]
    SyscallTargetNotInKernel {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        proc_root: Digest,
    },
    #[error("failed to execute arithmetic circuit evaluation operation: {error}")]
    #[diagnostic()]
    AceChipError {
        #[label("this call failed")]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        error: AceError,
    },
}

impl From<Ext2InttError> for ExecutionError {
    fn from(value: Ext2InttError) -> Self {
        Self::Ext2InttError(value)
    }
}

impl ExecutionError {
    pub fn advice_map_key_not_found(
        key: Word,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::AdviceMapKeyNotFound { label, source_file, key }
    }

    pub fn advice_stack_read_failed(
        row: RowIndex,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::AdviceStackReadFailed { label, source_file, row }
    }

    pub fn divide_by_zero(clk: RowIndex, err_ctx: &ErrorContext<'_, impl MastNodeExt>) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::DivideByZero { clk, label, source_file }
    }

    pub fn dynamic_node_not_found(
        digest: Digest,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::DynamicNodeNotFound { label, source_file, digest }
    }

    pub fn event_error(
        error: Box<dyn Error + Send + Sync + 'static>,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::EventError { label, source_file, error }
    }

    pub fn failed_assertion(
        clk: RowIndex,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::FailedAssertion {
            label,
            source_file,
            clk,
            err_code,
            err_msg,
        }
    }

    pub fn invalid_merkle_tree_depth(
        depth: Felt,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::InvalidMerkleTreeDepth { label, source_file, depth }
    }

    pub fn invalid_merkle_tree_node_index(
        depth: Felt,
        index: Felt,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::InvalidMerkleTreeNodeIndex { label, source_file, depth, index }
    }

    pub fn invalid_stack_depth_on_return(
        depth: usize,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::InvalidStackDepthOnReturn { label, source_file, depth }
    }

    pub fn log_argument_zero(clk: RowIndex, err_ctx: &ErrorContext<'_, impl MastNodeExt>) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::LogArgumentZero { label, source_file, clk }
    }

    pub fn malfored_mast_forest_in_host(
        root_digest: Digest,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MalformedMastForestInHost { label, source_file, root_digest }
    }

    pub fn malformed_signature_key(
        key_type: &'static str,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MalformedSignatureKey { label, source_file, key_type }
    }

    pub fn merkle_path_verification_failed(
        value: Word,
        index: Felt,
        root: Digest,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::MerklePathVerificationFailed {
            label,
            source_file,
            value,
            index,
            root,
            err_code,
            err_msg,
        }
    }

    pub fn merkle_store_lookup_failed(
        err: MerkleError,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MerkleStoreLookupFailed { label, source_file, err }
    }

    /// Note: This error currently never occurs, since `MerkleStore::merge_roots()` never fails.
    pub fn merkle_store_merge_failed(
        err: MerkleError,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MerkleStoreMergeFailed { label, source_file, err }
    }

    pub fn merkle_store_update_failed(
        err: MerkleError,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MerkleStoreUpdateFailed { label, source_file, err }
    }

    pub fn no_mast_forest_with_procedure(
        root_digest: Digest,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NoMastForestWithProcedure { label, source_file, root_digest }
    }

    pub fn not_binary_value_if(value: Felt, err_ctx: &ErrorContext<'_, impl MastNodeExt>) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueIf { label, source_file, value }
    }

    pub fn not_binary_value_op(value: Felt, err_ctx: &ErrorContext<'_, impl MastNodeExt>) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueOp { label, source_file, value }
    }

    pub fn not_binary_value_loop(
        value: Felt,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueLoop { label, source_file, value }
    }

    pub fn not_u32_value(
        value: Felt,
        err_code: Felt,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotU32Value { label, source_file, value, err_code }
    }

    pub fn smt_node_not_found(node: Word, err_ctx: &ErrorContext<'_, impl MastNodeExt>) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SmtNodeNotFound { label, source_file, node }
    }

    pub fn smt_node_preimage_not_valid(
        node: Word,
        preimage_len: usize,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SmtNodePreImageNotValid { label, source_file, node, preimage_len }
    }

    pub fn syscall_target_not_in_kernel(
        proc_root: Digest,
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SyscallTargetNotInKernel { label, source_file, proc_root }
    }

    pub fn failed_arithmetic_evaluation(
        err_ctx: &ErrorContext<'_, impl MastNodeExt>,
        error: AceError,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::AceChipError { label, source_file, error }
    }
}

impl AsRef<dyn Diagnostic> for ExecutionError {
    fn as_ref(&self) -> &(dyn Diagnostic + 'static) {
        self
    }
}

// ACE ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error)]
pub enum AceError {
    #[error("num of variables should be word aligned and non-zero but was {0}")]
    NumVarIsNotWordAlignedOrIsEmpty(u64),
    #[error("num of evaluation gates should be word aligned and non-zero but was {0}")]
    NumEvalIsNotWordAlignedOrIsEmpty(u64),
    #[error("circuit does not evaluate to zero")]
    CircuitNotEvaluateZero,
    #[error("failed to read from memory")]
    FailedMemoryRead,
    #[error("failed to decode instruction")]
    FailedDecodeInstruction,
    #[error("failed to read from the wiring bus")]
    FailedWireBusRead,
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
