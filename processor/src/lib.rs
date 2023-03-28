#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

pub use vm_core::{
    chiplets::hasher::Digest, errors::InputError, utils::DeserializationError, AssemblyOp, Kernel,
    Operation, Program, ProgramInfo, QuadExtension, StackInputs, StackOutputs, Word,
};
use vm_core::{
    code_blocks::{
        Call, CodeBlock, Join, Loop, OpBatch, Span, Split, OP_BATCH_SIZE, OP_GROUP_SIZE,
    },
    utils::collections::{BTreeMap, Vec},
    AdviceInjector, CodeBlockTable, Decorator, DecoratorIterator, Felt, FieldElement,
    StackTopState, StarkField, CHIPLETS_WIDTH, DECODER_TRACE_WIDTH, MIN_TRACE_LEN, ONE,
    RANGE_CHECK_TRACE_WIDTH, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH, ZERO,
};
use winter_prover::ColMatrix;

mod decorators;
mod operations;

mod system;
use system::System;
pub use system::{FMP_MIN, SYSCALL_FMP_MIN};

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

mod range;
use range::RangeChecker;

mod advice;
pub use advice::{AdviceInputs, AdviceProvider, AdviceSource, MemAdviceProvider};

mod chiplets;
use chiplets::Chiplets;

mod trace;
pub use trace::ExecutionTrace;
use trace::TraceFragment;

mod errors;
pub use errors::ExecutionError;

pub mod utils;

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
        hash::{Blake3_192, Blake3_256, ElementHasher, Hasher, Rpo256},
        merkle::{MerkleError, MerkleStore, MerkleTree, SimpleSmt},
        random::{RandomCoin, RpoRandomCoin, WinterRandomCoin},
    };
}

// TYPE ALIASES
// ================================================================================================

type QuadFelt = QuadExtension<Felt>;

type SysTrace = [Vec<Felt>; SYS_TRACE_WIDTH];

pub struct DecoderTrace {
    trace: [Vec<Felt>; DECODER_TRACE_WIDTH],
    aux_trace_hints: decoder::AuxTraceHints,
}

pub struct StackTrace {
    trace: [Vec<Felt>; STACK_TRACE_WIDTH],
    aux_builder: stack::AuxTraceBuilder,
}

pub struct RangeCheckTrace {
    trace: [Vec<Felt>; RANGE_CHECK_TRACE_WIDTH],
    aux_builder: range::AuxTraceBuilder,
}

pub struct ChipletsTrace {
    trace: [Vec<Felt>; CHIPLETS_WIDTH],
    hasher_aux_builder: chiplets::HasherAuxTraceBuilder,
    aux_builder: chiplets::AuxTraceBuilder,
}

// EXECUTORS
// ================================================================================================

/// Returns an execution trace resulting from executing the provided program against the provided
/// inputs.
pub fn execute<A>(
    program: &Program,
    stack_inputs: StackInputs,
    advice_provider: A,
) -> Result<ExecutionTrace, ExecutionError>
where
    A: AdviceProvider,
{
    let mut process = Process::new(program.kernel().clone(), stack_inputs, advice_provider);
    let stack_outputs = process.execute(program)?;
    let trace = ExecutionTrace::new(process, stack_outputs);
    assert_eq!(&program.hash(), trace.program_hash(), "inconsistent program hash");
    Ok(trace)
}

/// Returns an iterator which allows callers to step through the execution and inspect VM state at
/// each execution step.
pub fn execute_iter<A>(
    program: &Program,
    stack_inputs: StackInputs,
    advice_provider: A,
) -> VmStateIterator
where
    A: AdviceProvider,
{
    let mut process = Process::new_debug(program.kernel().clone(), stack_inputs, advice_provider);
    let result = process.execute(program);
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

#[cfg(not(any(test, feature = "internals")))]
struct Process<A>
where
    A: AdviceProvider,
{
    system: System,
    decoder: Decoder,
    stack: Stack,
    range: RangeChecker,
    chiplets: Chiplets,
    advice_provider: A,
}

impl<A> Process<A>
where
    A: AdviceProvider,
{
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------
    /// Creates a new process with the provided inputs.
    pub fn new(kernel: Kernel, stack_inputs: StackInputs, advice_provider: A) -> Self {
        Self::initialize(kernel, stack_inputs, advice_provider, false)
    }

    /// Creates a new process with provided inputs and debug options enabled.
    pub fn new_debug(kernel: Kernel, stack_inputs: StackInputs, advice_provider: A) -> Self {
        Self::initialize(kernel, stack_inputs, advice_provider, true)
    }

    fn initialize(
        kernel: Kernel,
        stack: StackInputs,
        advice_provider: A,
        in_debug_mode: bool,
    ) -> Self {
        Self {
            system: System::new(MIN_TRACE_LEN),
            decoder: Decoder::new(in_debug_mode),
            stack: Stack::new(&stack, MIN_TRACE_LEN, in_debug_mode),
            range: RangeChecker::new(),
            chiplets: Chiplets::new(kernel),
            advice_provider,
        }
    }

    // PROGRAM EXECUTOR
    // --------------------------------------------------------------------------------------------

    /// Executes the provided [Program] in this process.
    pub fn execute(&mut self, program: &Program) -> Result<StackOutputs, ExecutionError> {
        assert_eq!(self.system.clk(), 0, "a program has already been executed in this process");
        self.execute_code_block(program.root(), program.cb_table())?;

        Ok(self.stack.build_stack_outputs())
    }

    // CODE BLOCK EXECUTORS
    // --------------------------------------------------------------------------------------------

    /// Executes the specified [CodeBlock].
    ///
    /// # Errors
    /// Returns an [ExecutionError] if executing the specified block fails for any reason.
    fn execute_code_block(
        &mut self,
        block: &CodeBlock,
        cb_table: &CodeBlockTable,
    ) -> Result<(), ExecutionError> {
        match block {
            CodeBlock::Join(block) => self.execute_join_block(block, cb_table),
            CodeBlock::Split(block) => self.execute_split_block(block, cb_table),
            CodeBlock::Loop(block) => self.execute_loop_block(block, cb_table),
            CodeBlock::Call(block) => self.execute_call_block(block, cb_table),
            CodeBlock::Span(block) => self.execute_span_block(block),
            CodeBlock::Proxy(_) => Err(ExecutionError::UnexecutableCodeBlock(block.clone())),
        }
    }

    /// Executes the specified [Join] block.
    #[inline(always)]
    fn execute_join_block(
        &mut self,
        block: &Join,
        cb_table: &CodeBlockTable,
    ) -> Result<(), ExecutionError> {
        self.start_join_block(block)?;

        // execute first and then second child of the join block
        self.execute_code_block(block.first(), cb_table)?;
        self.execute_code_block(block.second(), cb_table)?;

        self.end_join_block(block)
    }

    /// Executes the specified [Split] block.
    #[inline(always)]
    fn execute_split_block(
        &mut self,
        block: &Split,
        cb_table: &CodeBlockTable,
    ) -> Result<(), ExecutionError> {
        // start the SPLIT block; this also pops the stack and returns the popped element
        let condition = self.start_split_block(block)?;

        // execute either the true or the false branch of the split block based on the condition
        if condition == ONE {
            self.execute_code_block(block.on_true(), cb_table)?;
        } else if condition == ZERO {
            self.execute_code_block(block.on_false(), cb_table)?;
        } else {
            return Err(ExecutionError::NotBinaryValue(condition));
        }

        self.end_split_block(block)
    }

    /// Executes the specified [Loop] block.
    #[inline(always)]
    fn execute_loop_block(
        &mut self,
        block: &Loop,
        cb_table: &CodeBlockTable,
    ) -> Result<(), ExecutionError> {
        // start the LOOP block; this also pops the stack and returns the popped element
        let condition = self.start_loop_block(block)?;

        // if the top of the stack is ONE, execute the loop body; otherwise skip the loop body
        if condition == ONE {
            // execute the loop body at least once
            self.execute_code_block(block.body(), cb_table)?;

            // keep executing the loop body until the condition on the top of the stack is no
            // longer ONE; each iteration of the loop is preceded by executing REPEAT operation
            // which drops the condition from the stack
            while self.stack.peek() == ONE {
                self.decoder.repeat();
                self.execute_op(Operation::Drop)?;
                self.execute_code_block(block.body(), cb_table)?;
            }

            // end the LOOP block and drop the condition from the stack
            self.end_loop_block(block, true)
        } else if condition == ZERO {
            // end the LOOP block, but don't drop the condition from the stack because it was
            // already dropped when we started the LOOP block
            self.end_loop_block(block, false)
        } else {
            Err(ExecutionError::NotBinaryValue(condition))
        }
    }

    /// Executes the specified [Call] block.
    #[inline(always)]
    fn execute_call_block(
        &mut self,
        block: &Call,
        cb_table: &CodeBlockTable,
    ) -> Result<(), ExecutionError> {
        // if this is a syscall, make sure the call target exists in the kernel
        if block.is_syscall() {
            self.chiplets.access_kernel_proc(block.fn_hash())?;
        }

        self.start_call_block(block)?;

        // get function body from the code block table and execute it
        let fn_body = cb_table
            .get(block.fn_hash())
            .ok_or_else(|| ExecutionError::CodeBlockNotFound(block.fn_hash()))?;
        self.execute_code_block(fn_body, cb_table)?;

        self.end_call_block(block)
    }

    /// Executes the specified [Span] block.
    #[inline(always)]
    fn execute_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.start_span_block(block)?;

        let mut op_offset = 0;
        let mut decorators = block.decorator_iter();

        // execute the first operation batch
        self.execute_op_batch(&block.op_batches()[0], &mut decorators, op_offset)?;
        op_offset += block.op_batches()[0].ops().len();

        // if the span contains more operation batches, execute them. each additional batch is
        // preceded by a RESPAN operation; executing RESPAN operation does not change the state
        // of the stack
        for op_batch in block.op_batches().iter().skip(1) {
            self.respan(op_batch);
            self.execute_op(Operation::Noop)?;
            self.execute_op_batch(op_batch, &mut decorators, op_offset)?;
            op_offset += op_batch.ops().len();
        }

        self.end_span_block(block)
    }

    /// Executes all operations in an [OpBatch]. This also ensures that all alignment rules are
    /// satisfied by executing NOOPs as needed. Specifically:
    /// - If an operation group ends with an operation carrying an immediate value, a NOOP is
    ///   executed after it.
    /// - If the number of groups in a batch is not a power of 2, NOOPs are executed (one per
    ///   group) to bring it up to the next power of two (e.g., 3 -> 4, 5 -> 8).
    #[inline(always)]
    fn execute_op_batch(
        &mut self,
        batch: &OpBatch,
        decorators: &mut DecoratorIterator,
        op_offset: usize,
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
            while let Some(decorator) = decorators.next(i + op_offset) {
                self.execute_decorator(decorator)?;
            }

            // decode and execute the operation
            self.decoder.execute_user_op(op, op_idx);
            self.execute_op(op)?;

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
                    self.execute_op(Operation::Noop)?;
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
            self.execute_op(Operation::Noop)?;

            // if we are not at the last group yet, set up the decoder for decoding the next
            // operation groups. the groups were are processing are just NOOPs - so, the op group
            // value is ZERO
            if group_idx < num_batch_groups - 1 {
                self.decoder.start_op_group(ZERO);
            }
        }

        Ok(())
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    pub const fn kernel(&self) -> &Kernel {
        self.chiplets.kernel()
    }

    pub fn get_memory_value(&self, ctx: u32, addr: u64) -> Option<Word> {
        self.chiplets.get_mem_value(ctx, addr)
    }

    pub fn into_parts(self) -> (System, Decoder, Stack, RangeChecker, Chiplets, A) {
        (
            self.system,
            self.decoder,
            self.stack,
            self.range,
            self.chiplets,
            self.advice_provider,
        )
    }
}

// INTERNALS
// ================================================================================================

#[cfg(any(test, feature = "internals"))]
pub struct Process<A>
where
    A: AdviceProvider,
{
    pub system: System,
    pub decoder: Decoder,
    pub stack: Stack,
    pub range: RangeChecker,
    pub chiplets: Chiplets,
    pub advice_provider: A,
}
