#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::{sync::Arc, vec::Vec};
use core::fmt::{Display, LowerHex};

use miden_air::trace::{
    CHIPLETS_WIDTH, DECODER_TRACE_WIDTH, MIN_TRACE_LEN, RANGE_CHECK_TRACE_WIDTH, STACK_TRACE_WIDTH,
    SYS_TRACE_WIDTH,
};
pub use miden_air::{ExecutionOptions, ExecutionOptionsError, RowIndex};
use utils::resolve_external_node;
pub use vm_core::{
    AssemblyOp, EMPTY_WORD, Felt, Kernel, ONE, Operation, Program, ProgramInfo, QuadExtension,
    StackInputs, StackOutputs, Word, ZERO,
    crypto::merkle::SMT_DEPTH,
    debuginfo::{DefaultSourceManager, SourceManager, SourceSpan},
    errors::InputError,
    mast::{MastForest, MastNode, MastNodeId},
    sys_events::SystemEvent,
    utils::{DeserializationError, collections::KvMap},
};
use vm_core::{
    Decorator, DecoratorIterator, FieldElement, WORD_SIZE,
    mast::{
        BasicBlockNode, CallNode, DynNode, JoinNode, LoopNode, OP_GROUP_SIZE, OpBatch, SplitNode,
    },
};
pub use winter_prover::matrix::ColMatrix;

pub mod fast;
use fast::FastProcessState;

mod operations;

mod system;
use system::System;
pub use system::{ContextId, FMP_MIN, SYSCALL_FMP_MIN};

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

mod range;
use range::RangeChecker;

mod host;
pub use host::{
    Host, MastForestStore, MemMastForestStore,
    advice::{AdviceError, AdviceInputs, AdviceProvider},
    default::{DefaultDebugHandler, DefaultHost},
    handlers::{DebugHandler, EventError, EventHandler, EventHandlerRegistry},
    library::HostLibrary,
};

mod chiplets;
use chiplets::Chiplets;
pub use chiplets::MemoryError;

mod trace;
use trace::TraceFragment;
pub use trace::{ChipletsLengths, ExecutionTrace, NUM_RAND_ROWS, TraceLenSummary};

mod errors;
pub use errors::{ErrorContext, ExecutionError, Ext2InttError};

pub mod utils;

#[cfg(test)]
mod tests;

mod debug;
pub use debug::{AsmOpInfo, VmState, VmStateIterator};

// RE-EXPORTS
// ================================================================================================

pub mod math {
    pub use vm_core::{Felt, FieldElement, StarkField};
    pub use winter_prover::math::fft;
}

pub mod crypto {
    pub use vm_core::crypto::{
        hash::{Blake3_192, Blake3_256, ElementHasher, Hasher, Rpo256, Rpx256},
        merkle::{
            MerkleError, MerklePath, MerkleStore, MerkleTree, NodeIndex, PartialMerkleTree,
            SimpleSmt,
        },
        random::{RandomCoin, RpoRandomCoin, RpxRandomCoin, WinterRandomCoin},
    };
}

// TYPE ALIASES
// ================================================================================================

type QuadFelt = QuadExtension<Felt>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MemoryAddress(u32);

impl From<u32> for MemoryAddress {
    fn from(addr: u32) -> Self {
        MemoryAddress(addr)
    }
}

impl From<MemoryAddress> for u32 {
    fn from(value: MemoryAddress) -> Self {
        value.0
    }
}

impl Display for MemoryAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl LowerHex for MemoryAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl core::ops::Add<MemoryAddress> for MemoryAddress {
    type Output = Self;

    fn add(self, rhs: MemoryAddress) -> Self::Output {
        MemoryAddress(self.0 + rhs.0)
    }
}

impl core::ops::Add<u32> for MemoryAddress {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        MemoryAddress(self.0 + rhs)
    }
}

type SysTrace = [Vec<Felt>; SYS_TRACE_WIDTH];

pub struct DecoderTrace {
    trace: [Vec<Felt>; DECODER_TRACE_WIDTH],
    aux_builder: decoder::AuxTraceBuilder,
}

pub struct StackTrace {
    trace: [Vec<Felt>; STACK_TRACE_WIDTH],
}

pub struct RangeCheckTrace {
    trace: [Vec<Felt>; RANGE_CHECK_TRACE_WIDTH],
    aux_builder: range::AuxTraceBuilder,
}

pub struct ChipletsTrace {
    trace: [Vec<Felt>; CHIPLETS_WIDTH],
    aux_builder: chiplets::AuxTraceBuilder,
}

// EXECUTORS
// ================================================================================================

/// Returns an execution trace resulting from executing the provided program against the provided
/// inputs.
///
/// The `host` parameter is used to provide the external environment to the program being executed,
/// such as access to the advice provider and libraries that the program depends on.
#[tracing::instrument("execute_program", skip_all)]
pub fn execute(
    program: &Program,
    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,
    host: &mut impl Host,
    options: ExecutionOptions,
    source_manager: Arc<dyn SourceManager>,
) -> Result<ExecutionTrace, ExecutionError> {
    let mut process = Process::new(program.kernel().clone(), stack_inputs, advice_inputs, options)
        .with_source_manager(source_manager);
    let stack_outputs = process.execute(program, host)?;
    let trace = ExecutionTrace::new(process, stack_outputs);
    assert_eq!(&program.hash(), trace.program_hash(), "inconsistent program hash");
    Ok(trace)
}

/// Returns an iterator which allows callers to step through the execution and inspect VM state at
/// each execution step.
pub fn execute_iter(
    program: &Program,
    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,
    host: &mut impl Host,
    source_manager: Arc<dyn SourceManager>,
) -> VmStateIterator {
    let mut process = Process::new_debug(program.kernel().clone(), stack_inputs, advice_inputs)
        .with_source_manager(source_manager);
    let result = process.execute(program, host);
    if result.is_ok() {
        assert_eq!(
            program.hash(),
            process.decoder.program_hash().into(),
            "inconsistent program hash"
        );
    }
    VmStateIterator::new(process, result)
}

// PROCESS
// ================================================================================================

/// A [Process] is the underlying execution engine for a Miden [Program].
///
/// Typically, you do not need to worry about, or use [Process] directly, instead you should prefer
/// to use either [execute] or [execute_iter], which also handle setting up the process state,
/// inputs, as well as compute the [ExecutionTrace] for the program.
///
/// However, for situations in which you want finer-grained control over those steps, you will need
/// to construct an instance of [Process] using [Process::new], invoke [Process::execute], and then
/// get the execution trace using [ExecutionTrace::new] using the outputs produced by execution.
#[cfg(not(any(test, feature = "testing")))]
pub struct Process {
    advice: AdviceProvider,
    system: System,
    decoder: Decoder,
    stack: Stack,
    range: RangeChecker,
    chiplets: Chiplets,
    max_cycles: u32,
    enable_tracing: bool,
    source_manager: Arc<dyn SourceManager>,
}

#[cfg(any(test, feature = "testing"))]
pub struct Process {
    pub advice: AdviceProvider,
    pub system: System,
    pub decoder: Decoder,
    pub stack: Stack,
    pub range: RangeChecker,
    pub chiplets: Chiplets,
    pub max_cycles: u32,
    pub enable_tracing: bool,
    pub source_manager: Arc<dyn SourceManager>,
}

impl Process {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Creates a new process with the provided inputs.
    pub fn new(
        kernel: Kernel,
        stack_inputs: StackInputs,
        advice_inputs: AdviceInputs,
        execution_options: ExecutionOptions,
    ) -> Self {
        Self::initialize(kernel, stack_inputs, advice_inputs, execution_options)
    }

    /// Creates a new process with provided inputs and debug options enabled.
    pub fn new_debug(
        kernel: Kernel,
        stack_inputs: StackInputs,
        advice_inputs: AdviceInputs,
    ) -> Self {
        Self::initialize(
            kernel,
            stack_inputs,
            advice_inputs,
            ExecutionOptions::default().with_tracing().with_debugging(true),
        )
    }

    fn initialize(
        kernel: Kernel,
        stack: StackInputs,
        advice_inputs: AdviceInputs,
        execution_options: ExecutionOptions,
    ) -> Self {
        let in_debug_mode = execution_options.enable_debugging();
        let source_manager = Arc::new(DefaultSourceManager::default());
        Self {
            advice: advice_inputs.into(),
            system: System::new(execution_options.expected_cycles() as usize),
            decoder: Decoder::new(in_debug_mode),
            stack: Stack::new(&stack, execution_options.expected_cycles() as usize, in_debug_mode),
            range: RangeChecker::new(),
            chiplets: Chiplets::new(kernel),
            max_cycles: execution_options.max_cycles(),
            enable_tracing: execution_options.enable_tracing(),
            source_manager,
        }
    }

    /// Set the internal source manager to an externally initialized one.
    pub fn with_source_manager(mut self, source_manager: Arc<dyn SourceManager>) -> Self {
        self.source_manager = source_manager;
        self
    }

    // PROGRAM EXECUTOR
    // --------------------------------------------------------------------------------------------

    /// Executes the provided [`Program`] in this process.
    pub fn execute(
        &mut self,
        program: &Program,
        host: &mut impl Host,
    ) -> Result<StackOutputs, ExecutionError> {
        if self.system.clk() != 0 {
            return Err(ExecutionError::ProgramAlreadyExecuted);
        }

        self.advice
            .merge_advice_map(program.mast_forest().advice_map())
            .map_err(|err| ExecutionError::advice_error(err, RowIndex::from(0), &()))?;

        self.execute_mast_node(program.entrypoint(), &program.mast_forest().clone(), host)?;

        self.stack.build_stack_outputs()
    }

    // NODE EXECUTORS
    // --------------------------------------------------------------------------------------------

    fn execute_mast_node(
        &mut self,
        node_id: MastNodeId,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let node = program
            .get_node_by_id(node_id)
            .ok_or(ExecutionError::MastNodeNotFoundInForest { node_id })?;

        for &decorator_id in node.before_enter() {
            self.execute_decorator(&program[decorator_id], host)?;
        }

        match node {
            MastNode::Block(node) => self.execute_basic_block_node(node, program, host)?,
            MastNode::Join(node) => self.execute_join_node(node, program, host)?,
            MastNode::Split(node) => self.execute_split_node(node, program, host)?,
            MastNode::Loop(node) => self.execute_loop_node(node, program, host)?,
            MastNode::Call(node) => {
                let err_ctx = err_ctx!(program, node, self.source_manager.clone());
                add_error_ctx_to_external_error(
                    self.execute_call_node(node, program, host),
                    err_ctx,
                )?
            },
            MastNode::Dyn(node) => {
                let err_ctx = err_ctx!(program, node, self.source_manager.clone());
                add_error_ctx_to_external_error(
                    self.execute_dyn_node(node, program, host),
                    err_ctx,
                )?
            },
            MastNode::External(external_node) => {
                let (root_id, mast_forest) =
                    resolve_external_node(external_node, &mut self.advice, host)?;

                self.execute_mast_node(root_id, &mast_forest, host)?;
            },
        }

        for &decorator_id in node.after_exit() {
            self.execute_decorator(&program[decorator_id], host)?;
        }

        Ok(())
    }

    /// Executes the specified [JoinNode].
    #[inline(always)]
    fn execute_join_node(
        &mut self,
        node: &JoinNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        self.start_join_node(node, program, host)?;

        // execute first and then second child of the join block
        self.execute_mast_node(node.first(), program, host)?;
        self.execute_mast_node(node.second(), program, host)?;

        self.end_join_node(node, program, host)
    }

    /// Executes the specified [SplitNode].
    #[inline(always)]
    fn execute_split_node(
        &mut self,
        node: &SplitNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // start the SPLIT block; this also pops the stack and returns the popped element
        let condition = self.start_split_node(node, program, host)?;

        // execute either the true or the false branch of the split block based on the condition
        if condition == ONE {
            self.execute_mast_node(node.on_true(), program, host)?;
        } else if condition == ZERO {
            self.execute_mast_node(node.on_false(), program, host)?;
        } else {
            let err_ctx = err_ctx!(program, node, self.source_manager.clone());
            return Err(ExecutionError::not_binary_value_if(condition, &err_ctx));
        }

        self.end_split_node(node, program, host)
    }

    /// Executes the specified [LoopNode].
    #[inline(always)]
    fn execute_loop_node(
        &mut self,
        node: &LoopNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // start the LOOP block; this also pops the stack and returns the popped element
        let condition = self.start_loop_node(node, program, host)?;

        // if the top of the stack is ONE, execute the loop body; otherwise skip the loop body
        if condition == ONE {
            // execute the loop body at least once
            self.execute_mast_node(node.body(), program, host)?;

            // keep executing the loop body until the condition on the top of the stack is no
            // longer ONE; each iteration of the loop is preceded by executing REPEAT operation
            // which drops the condition from the stack
            while self.stack.peek() == ONE {
                self.decoder.repeat();
                self.execute_op(Operation::Drop, program, host)?;
                self.execute_mast_node(node.body(), program, host)?;
            }

            if self.stack.peek() != ZERO {
                let err_ctx = err_ctx!(program, node, self.source_manager.clone());
                return Err(ExecutionError::not_binary_value_loop(self.stack.peek(), &err_ctx));
            }

            // end the LOOP block and drop the condition from the stack
            self.end_loop_node(node, true, program, host)
        } else if condition == ZERO {
            // end the LOOP block, but don't drop the condition from the stack because it was
            // already dropped when we started the LOOP block
            self.end_loop_node(node, false, program, host)
        } else {
            let err_ctx = err_ctx!(program, node, self.source_manager.clone());
            Err(ExecutionError::not_binary_value_loop(condition, &err_ctx))
        }
    }

    /// Executes the specified [CallNode].
    #[inline(always)]
    fn execute_call_node(
        &mut self,
        call_node: &CallNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // call or syscall are not allowed inside a syscall
        if self.system.in_syscall() {
            let instruction = if call_node.is_syscall() { "syscall" } else { "call" };
            return Err(ExecutionError::CallInSyscall(instruction));
        }

        // if this is a syscall, make sure the call target exists in the kernel
        if call_node.is_syscall() {
            let callee = program.get_node_by_id(call_node.callee()).ok_or_else(|| {
                ExecutionError::MastNodeNotFoundInForest { node_id: call_node.callee() }
            })?;
            let err_ctx = err_ctx!(program, call_node, self.source_manager.clone());
            self.chiplets.kernel_rom.access_proc(callee.digest(), &err_ctx)?;
        }
        let err_ctx = err_ctx!(program, call_node, self.source_manager.clone());

        self.start_call_node(call_node, program, host)?;
        self.execute_mast_node(call_node.callee(), program, host)?;
        self.end_call_node(call_node, program, host, &err_ctx)
    }

    /// Executes the specified [vm_core::mast::DynNode].
    ///
    /// The MAST root of the callee is assumed to be at the top of the stack, and the callee is
    /// expected to be either in the current `program` or in the host.
    #[inline(always)]
    fn execute_dyn_node(
        &mut self,
        node: &DynNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // dyn calls are not allowed inside a syscall
        if node.is_dyncall() && self.system.in_syscall() {
            return Err(ExecutionError::CallInSyscall("dyncall"));
        }

        let err_ctx = err_ctx!(program, node, self.source_manager.clone());

        let callee_hash = if node.is_dyncall() {
            self.start_dyncall_node(node, &err_ctx)?
        } else {
            self.start_dyn_node(node, program, host, &err_ctx)?
        };

        // if the callee is not in the program's MAST forest, try to find a MAST forest for it in
        // the host (corresponding to an external library loaded in the host); if none are
        // found, return an error.
        match program.find_procedure_root(callee_hash) {
            Some(callee_id) => self.execute_mast_node(callee_id, program, host)?,
            None => {
                let mast_forest = host
                    .get_mast_forest(&callee_hash)
                    .ok_or_else(|| ExecutionError::dynamic_node_not_found(callee_hash, &err_ctx))?;

                // We limit the parts of the program that can be called externally to procedure
                // roots, even though MAST doesn't have that restriction.
                let root_id = mast_forest
                    .find_procedure_root(callee_hash)
                    .ok_or(ExecutionError::malfored_mast_forest_in_host(callee_hash, &()))?;

                self.execute_mast_node(root_id, &mast_forest, host)?
            },
        }

        if node.is_dyncall() {
            self.end_dyncall_node(node, program, host, &err_ctx)
        } else {
            self.end_dyn_node(node, program, host)
        }
    }

    /// Executes the specified [BasicBlockNode].
    #[inline(always)]
    fn execute_basic_block_node(
        &mut self,
        basic_block: &BasicBlockNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        self.start_basic_block_node(basic_block, program, host)?;

        let mut op_offset = 0;
        let mut decorator_ids = basic_block.decorator_iter();

        // execute the first operation batch
        self.execute_op_batch(
            basic_block,
            &basic_block.op_batches()[0],
            &mut decorator_ids,
            op_offset,
            program,
            host,
        )?;
        op_offset += basic_block.op_batches()[0].ops().len();

        // if the span contains more operation batches, execute them. each additional batch is
        // preceded by a RESPAN operation; executing RESPAN operation does not change the state
        // of the stack
        for op_batch in basic_block.op_batches().iter().skip(1) {
            self.respan(op_batch);
            self.execute_op(Operation::Noop, program, host)?;
            self.execute_op_batch(
                basic_block,
                op_batch,
                &mut decorator_ids,
                op_offset,
                program,
                host,
            )?;
            op_offset += op_batch.ops().len();
        }

        self.end_basic_block_node(basic_block, program, host)?;

        // execute any decorators which have not been executed during span ops execution; this
        // can happen for decorators appearing after all operations in a block. these decorators
        // are executed after SPAN block is closed to make sure the VM clock cycle advances beyond
        // the last clock cycle of the SPAN block ops.
        for &decorator_id in decorator_ids {
            let decorator = program
                .get_decorator_by_id(decorator_id)
                .ok_or(ExecutionError::DecoratorNotFoundInForest { decorator_id })?;
            self.execute_decorator(decorator, host)?;
        }

        Ok(())
    }

    /// Executes all operations in an [OpBatch]. This also ensures that all alignment rules are
    /// satisfied by executing NOOPs as needed. Specifically:
    /// - If an operation group ends with an operation carrying an immediate value, a NOOP is
    ///   executed after it.
    /// - If the number of groups in a batch is not a power of 2, NOOPs are executed (one per group)
    ///   to bring it up to the next power of two (e.g., 3 -> 4, 5 -> 8).
    #[inline(always)]
    fn execute_op_batch(
        &mut self,
        basic_block: &BasicBlockNode,
        batch: &OpBatch,
        decorators: &mut DecoratorIterator,
        op_offset: usize,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let op_counts = batch.op_counts();
        let mut op_idx = 0;
        let mut group_idx = 0;
        let mut next_group_idx = 1;

        // round up the number of groups to be processed to the next power of two; we do this
        // because the processor requires the number of groups to be either 1, 2, 4, or 8; if
        // the actual number of groups is smaller, we'll pad the batch with NOOPs at the end
        let num_batch_groups = batch.num_groups().next_power_of_two();

        // execute operations in the batch one by one
        for (i, &op) in batch.ops().iter().enumerate() {
            while let Some(&decorator_id) = decorators.next_filtered(i + op_offset) {
                let decorator = program
                    .get_decorator_by_id(decorator_id)
                    .ok_or(ExecutionError::DecoratorNotFoundInForest { decorator_id })?;
                self.execute_decorator(decorator, host)?;
            }

            // decode and execute the operation
            let err_ctx =
                err_ctx!(program, basic_block, self.source_manager.clone(), i + op_offset);
            self.decoder.execute_user_op(op, op_idx);
            self.execute_op_with_error_ctx(op, program, host, &err_ctx)?;

            // if the operation carries an immediate value, the value is stored at the next group
            // pointer; so, we advance the pointer to the following group
            let has_imm = op.imm_value().is_some();
            if has_imm {
                next_group_idx += 1;
            }

            // determine if we've executed all non-decorator operations in a group
            if op_idx == op_counts[group_idx] - 1 {
                // if we are at the end of the group, first check if the operation carries an
                // immediate value
                if has_imm {
                    // an operation with an immediate value cannot be the last operation in a group
                    // so, we need execute a NOOP after it. the assert also makes sure that there
                    // is enough room in the group to execute a NOOP (if there isn't, there is a
                    // bug somewhere in the assembler)
                    debug_assert!(op_idx < OP_GROUP_SIZE - 1, "invalid op index");
                    self.decoder.execute_user_op(Operation::Noop, op_idx + 1);
                    self.execute_op(Operation::Noop, program, host)?;
                }

                // then, move to the next group and reset operation index
                group_idx = next_group_idx;
                next_group_idx += 1;
                op_idx = 0;

                // if we haven't reached the end of the batch yet, set up the decoder for
                // decoding the next operation group
                if group_idx < num_batch_groups {
                    self.decoder.start_op_group(batch.groups()[group_idx]);
                }
            } else {
                // if we are not at the end of the group, just increment the operation index
                op_idx += 1;
            }
        }

        // make sure we execute the required number of operation groups; this would happen when
        // the actual number of operation groups was not a power of two
        for group_idx in group_idx..num_batch_groups {
            self.decoder.execute_user_op(Operation::Noop, 0);
            self.execute_op(Operation::Noop, program, host)?;

            // if we are not at the last group yet, set up the decoder for decoding the next
            // operation groups. the groups were are processing are just NOOPs - so, the op group
            // value is ZERO
            if group_idx < num_batch_groups - 1 {
                self.decoder.start_op_group(ZERO);
            }
        }

        Ok(())
    }

    /// Executes the specified decorator
    fn execute_decorator(
        &mut self,
        decorator: &Decorator,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        match decorator {
            Decorator::Debug(options) => {
                if self.decoder.in_debug_mode() {
                    let process = &mut self.state();
                    host.on_debug(process, options)?;
                }
            },
            Decorator::AsmOp(assembly_op) => {
                if self.decoder.in_debug_mode() {
                    self.decoder.append_asmop(self.system.clk(), assembly_op.clone());
                }
            },
            Decorator::Trace(id) => {
                if self.enable_tracing {
                    let process = &mut self.state();
                    host.on_trace(process, *id)?;
                }
            },
        };
        Ok(())
    }

    // PUBLIC ACCESSORS
    // ================================================================================================

    pub const fn kernel(&self) -> &Kernel {
        self.chiplets.kernel_rom.kernel()
    }

    pub fn into_parts(self) -> (System, Decoder, Stack, RangeChecker, Chiplets) {
        (self.system, self.decoder, self.stack, self.range, self.chiplets)
    }
}

#[derive(Debug)]
pub struct SlowProcessState<'a> {
    advice: &'a mut AdviceProvider,
    system: &'a System,
    stack: &'a Stack,
    chiplets: &'a Chiplets,
}

// PROCESS STATE
// ================================================================================================

#[derive(Debug)]
pub enum ProcessState<'a> {
    Slow(SlowProcessState<'a>),
    Fast(FastProcessState<'a>),
}

impl Process {
    #[inline(always)]
    fn state(&mut self) -> ProcessState<'_> {
        ProcessState::Slow(SlowProcessState {
            advice: &mut self.advice,
            system: &self.system,
            stack: &self.stack,
            chiplets: &self.chiplets,
        })
    }
}

impl<'a> ProcessState<'a> {
    /// Returns a reference to the advice provider.
    #[inline(always)]
    pub fn advice_provider(&self) -> &AdviceProvider {
        match self {
            ProcessState::Slow(state) => state.advice,
            ProcessState::Fast(state) => &state.processor.advice,
        }
    }

    /// Returns a mutable reference to the advice provider.
    #[inline(always)]
    pub fn advice_provider_mut(&mut self) -> &mut AdviceProvider {
        match self {
            ProcessState::Slow(state) => state.advice,
            ProcessState::Fast(state) => &mut state.processor.advice,
        }
    }

    /// Returns the current clock cycle of a process.
    #[inline(always)]
    pub fn clk(&self) -> RowIndex {
        match self {
            ProcessState::Slow(state) => state.system.clk(),
            ProcessState::Fast(state) => state.processor.clk + state.op_idx,
        }
    }

    /// Returns the current execution context ID.
    #[inline(always)]
    pub fn ctx(&self) -> ContextId {
        match self {
            ProcessState::Slow(state) => state.system.ctx(),
            ProcessState::Fast(state) => state.processor.ctx,
        }
    }

    /// Returns the current value of the free memory pointer.
    #[inline(always)]
    pub fn fmp(&self) -> u64 {
        match self {
            ProcessState::Slow(state) => state.system.fmp().as_int(),
            ProcessState::Fast(state) => state.processor.fmp.as_int(),
        }
    }

    /// Returns the value located at the specified position on the stack at the current clock cycle.
    #[inline(always)]
    pub fn get_stack_item(&self, pos: usize) -> Felt {
        match self {
            ProcessState::Slow(state) => state.stack.get(pos),
            ProcessState::Fast(state) => state.processor.stack_get(pos),
        }
    }

    /// Returns a word located at the specified word index on the stack.
    ///
    /// Specifically, word 0 is defined by the first 4 elements of the stack, word 1 is defined
    /// by the next 4 elements etc. Since the top of the stack contains 4 word, the highest valid
    /// word index is 3.
    ///
    /// The words are created in reverse order. For example, for word 0 the top element of the
    /// stack will be at the last position in the word.
    ///
    /// Creating a word does not change the state of the stack.
    #[inline(always)]
    pub fn get_stack_word(&self, word_idx: usize) -> Word {
        match self {
            ProcessState::Slow(state) => state.stack.get_word(word_idx),
            ProcessState::Fast(state) => state.processor.stack_get_word(word_idx * WORD_SIZE),
        }
    }

    /// Returns stack state at the current clock cycle. This includes the top 16 items of the
    /// stack + overflow entries.
    #[inline(always)]
    pub fn get_stack_state(&self) -> Vec<Felt> {
        match self {
            ProcessState::Slow(state) => state.stack.get_state_at(state.system.clk()),
            ProcessState::Fast(state) => state.processor.stack().iter().rev().copied().collect(),
        }
    }

    /// Returns the element located at the specified context/address, or None if the address hasn't
    /// been accessed previously.
    #[inline(always)]
    pub fn get_mem_value(&self, ctx: ContextId, addr: u32) -> Option<Felt> {
        match self {
            ProcessState::Slow(state) => state.chiplets.memory.get_value(ctx, addr),
            ProcessState::Fast(state) => state.processor.memory.read_element_impl(ctx, addr),
        }
    }

    /// Returns the batch of elements starting at the specified context/address.
    ///
    /// # Errors
    /// - If the address is not word aligned.
    #[inline(always)]
    pub fn get_mem_word(&self, ctx: ContextId, addr: u32) -> Result<Option<Word>, MemoryError> {
        match self {
            ProcessState::Slow(state) => state.chiplets.memory.get_word(ctx, addr),
            ProcessState::Fast(state) => {
                state.processor.memory.read_word_impl(ctx, addr, None, &())
            },
        }
    }

    /// Returns the entire memory state for the specified execution context at the current clock
    /// cycle.
    ///
    /// The state is returned as a vector of (address, value) tuples, and includes addresses which
    /// have been accessed at least once.
    #[inline(always)]
    pub fn get_mem_state(&self, ctx: ContextId) -> Vec<(MemoryAddress, Felt)> {
        match self {
            ProcessState::Slow(state) => {
                state.chiplets.memory.get_state_at(ctx, state.system.clk())
            },
            ProcessState::Fast(state) => state.processor.memory.get_memory_state(ctx),
        }
    }
}

// HELPERS
// ================================================================================================

/// For errors generated from processing an `ExternalNode`, returns the same error except with
/// proper error context.
pub(crate) fn add_error_ctx_to_external_error(
    result: Result<(), ExecutionError>,
    err_ctx: impl ErrorContext,
) -> Result<(), ExecutionError> {
    match result {
        Ok(_) => Ok(()),
        // Add context information to any errors coming from executing an `ExternalNode`
        Err(err) => match err {
            ExecutionError::NoMastForestWithProcedure { label, source_file: _, root_digest }
            | ExecutionError::MalformedMastForestInHost { label, source_file: _, root_digest } => {
                if label == SourceSpan::UNKNOWN {
                    let err_with_ctx =
                        ExecutionError::no_mast_forest_with_procedure(root_digest, &err_ctx);
                    Err(err_with_ctx)
                } else {
                    // If the source span was already populated, just return the error as-is. This
                    // would occur when a call deeper down the call stack was responsible for the
                    // error.
                    Err(err)
                }
            },

            _ => {
                // do nothing
                Err(err)
            },
        },
    }
}
