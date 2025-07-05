use alloc::{boxed::Box, sync::Arc};
use core::error::Error;

use miden_air::RowIndex;
use miette::Diagnostic;
use vm_core::{
    debuginfo::{SourceFile, SourceManager, SourceSpan},
    mast::{DecoratorId, MastForest, MastNodeExt, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::to_hex,
};
use winter_prover::ProverError;

use super::{
    Felt, QuadFelt, Word,
    system::{FMP_MAX, FMP_MIN},
};
use crate::{MemoryError, host::advice::AdviceError};

// EXECUTION ERROR
// ================================================================================================

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ExecutionError {
    #[error("advice provider error at clock cycle {clk}")]
    #[diagnostic()]
    AdviceError {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        clk: RowIndex,
        #[source]
        #[diagnostic_source]
        err: AdviceError,
    },
    /// This error is caught by the assembler, so we don't need diagnostics here.
    #[error("illegal use of instruction {0} while inside a syscall")]
    CallInSyscall(&'static str),
    /// This error is caught by the assembler, so we don't need diagnostics here.
    #[error("instruction `caller` used outside of kernel context")]
    CallerNotInSyscall,
    #[error("external node with mast root {0} resolved to an external node")]
    CircularExternalNode(Word),
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
    #[error("failed to execute the dynamic code block provided by the stack with root {hex}; the block could not be found",
      hex = .digest.to_hex()
    )]
    #[diagnostic()]
    DynamicNodeNotFound {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        digest: Word,
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
        root_digest: Word,
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
        root_digest: Word,
    },
    #[error("merkle path verification failed for value {value} at index {index} in the Merkle tree with root {root} (error {err})",
      value = to_hex(value.as_bytes()),
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
        root: Word,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
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
    #[error(
        "Operand stack input is {input} but it is expected to fit in a u32 at clock cycle {clk}"
    )]
    #[diagnostic()]
    NotU32StackValue {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        clk: RowIndex,
        input: u64,
    },
    #[error("stack should have at most {MIN_STACK_DEPTH} elements at the end of program execution, but had {} elements", MIN_STACK_DEPTH + .0)]
    OutputStackOverflow(usize),
    #[error("a program has already been executed in this process")]
    ProgramAlreadyExecuted,
    #[error("proof generation failed")]
    ProverError(#[source] ProverError),
    #[error("smt node {node_hex} not found", node_hex = to_hex(node.as_bytes()))]
    SmtNodeNotFound {
        #[label]
        label: SourceSpan,
        #[source_code]
        source_file: Option<Arc<SourceFile>>,
        node: Word,
    },
    #[error("expected pre-image length of node {node_hex} to be a multiple of 8 but was {preimage_len}",
      node_hex = to_hex(node.as_bytes()),
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
        proc_root: Word,
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

impl ExecutionError {
    pub fn advice_error(
        err: AdviceError,
        clk: RowIndex,
        err_ctx: &impl ErrorContext,
    ) -> ExecutionError {
        let (label, source_file) = err_ctx.label_and_source_file();
        ExecutionError::AdviceError { label, source_file, err, clk }
    }

    pub fn divide_by_zero(clk: RowIndex, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::DivideByZero { clk, label, source_file }
    }

    pub fn input_not_u32(clk: RowIndex, input: u64, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotU32StackValue { clk, input, label, source_file }
    }

    pub fn dynamic_node_not_found(digest: Word, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::DynamicNodeNotFound { label, source_file, digest }
    }

    pub fn event_error(
        error: Box<dyn Error + Send + Sync + 'static>,
        err_ctx: &impl ErrorContext,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();

        Self::EventError { label, source_file, error }
    }

    pub fn failed_assertion(
        clk: RowIndex,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
        err_ctx: &impl ErrorContext,
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

    pub fn invalid_stack_depth_on_return(depth: usize, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::InvalidStackDepthOnReturn { label, source_file, depth }
    }

    pub fn log_argument_zero(clk: RowIndex, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::LogArgumentZero { label, source_file, clk }
    }

    pub fn malfored_mast_forest_in_host(root_digest: Word, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MalformedMastForestInHost { label, source_file, root_digest }
    }

    pub fn malformed_signature_key(key_type: &'static str, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::MalformedSignatureKey { label, source_file, key_type }
    }

    pub fn merkle_path_verification_failed(
        value: Word,
        index: Felt,
        root: Word,
        err_code: Felt,
        err_msg: Option<Arc<str>>,
        err_ctx: &impl ErrorContext,
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

    pub fn no_mast_forest_with_procedure(root_digest: Word, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NoMastForestWithProcedure { label, source_file, root_digest }
    }

    pub fn not_binary_value_if(value: Felt, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueIf { label, source_file, value }
    }

    pub fn not_binary_value_op(value: Felt, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueOp { label, source_file, value }
    }

    pub fn not_binary_value_loop(value: Felt, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotBinaryValueLoop { label, source_file, value }
    }

    pub fn not_u32_value(value: Felt, err_code: Felt, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::NotU32Value { label, source_file, value, err_code }
    }

    pub fn smt_node_not_found(node: Word, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SmtNodeNotFound { label, source_file, node }
    }

    pub fn smt_node_preimage_not_valid(
        node: Word,
        preimage_len: usize,
        err_ctx: &impl ErrorContext,
    ) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SmtNodePreImageNotValid { label, source_file, node, preimage_len }
    }

    pub fn syscall_target_not_in_kernel(proc_root: Word, err_ctx: &impl ErrorContext) -> Self {
        let (label, source_file) = err_ctx.label_and_source_file();
        Self::SyscallTargetNotInKernel { label, source_file, proc_root }
    }

    pub fn failed_arithmetic_evaluation(err_ctx: &impl ErrorContext, error: AceError) -> Self {
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
    #[error("num of wires must be less than 2^30 but was {0}")]
    TooManyWires(u64),
}

// ERROR CONTEXT
// ===============================================================================================

/// Constructs an error context for the given node in the MAST forest.
///
/// When the `no_err_ctx` feature is disabled, this macro returns a proper error context; otherwise,
/// it returns `()`. That is, this macro is designed to be zero-cost when the `no_err_ctx` feature
/// is enabled.
///
/// Usage:
/// - `err_ctx!(mast_forest, node, source_manager)` - creates basic error context
/// - `err_ctx!(mast_forest, node, source_manager, op_idx)` - creates error context with operation
///   index
#[cfg(not(feature = "no_err_ctx"))]
#[macro_export]
macro_rules! err_ctx {
    ($mast_forest:expr, $node:expr, $source_manager:expr) => {
        $crate::errors::ErrorContextImpl::new($mast_forest, $node, $source_manager)
    };
    ($mast_forest:expr, $node:expr, $source_manager:expr, $op_idx:expr) => {
        $crate::errors::ErrorContextImpl::new_with_op_idx(
            $mast_forest,
            $node,
            $source_manager,
            $op_idx,
        )
    };
}

/// Constructs an error context for the given node in the MAST forest.
///
/// When the `no_err_ctx` feature is disabled, this macro returns a proper error context; otherwise,
/// it returns `()`. That is, this macro is designed to be zero-cost when the `no_err_ctx` feature
/// is enabled.
///
/// Usage:
/// - `err_ctx!(mast_forest, node, source_manager)` - creates basic error context
/// - `err_ctx!(mast_forest, node, source_manager, op_idx)` - creates error context with operation
///   index
#[cfg(feature = "no_err_ctx")]
#[macro_export]
macro_rules! err_ctx {
    ($mast_forest:expr, $node:expr, $source_manager:expr) => {{ () }};
    ($mast_forest:expr, $node:expr, $source_manager:expr, $op_idx:expr) => {{ () }};
}

/// Trait defining the interface for error context providers.
///
/// This trait contains the same methods as `ErrorContext` to provide a common
/// interface for error context functionality.
pub trait ErrorContext {
    /// Returns the label and source file associated with the error context, if any.
    ///
    /// Note that `SourceSpan::UNKNOWN` will be returned to indicate an empty span.
    fn label_and_source_file(&self) -> (SourceSpan, Option<Arc<SourceFile>>);
}

/// Context information to be used when reporting errors.
#[derive(Debug)]
pub struct ErrorContextImpl<'a, N: MastNodeExt> {
    mast_forest: &'a MastForest,
    node: &'a N,
    source_manager: Arc<dyn SourceManager>,
    op_idx: Option<usize>,
}

impl<'a, N: MastNodeExt> ErrorContextImpl<'a, N> {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
                        self.source_manager.get_by_uri(&location.uri),
                    )
                },
            )
    }
}

impl<'a, N: MastNodeExt> ErrorContext for ErrorContextImpl<'a, N> {
    fn label_and_source_file(&self) -> (SourceSpan, Option<Arc<SourceFile>>) {
        self.label_and_source_file()
    }
}

impl ErrorContext for () {
    fn label_and_source_file(&self) -> (SourceSpan, Option<Arc<SourceFile>>) {
        (SourceSpan::UNKNOWN, None)
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
