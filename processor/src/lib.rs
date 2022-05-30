use vm_core::{
    errors::AdviceSetError,
    hasher::Digest,
    program::{
        blocks::{CodeBlock, Join, Loop, OpBatch, Span, Split, OP_BATCH_SIZE, OP_GROUP_SIZE},
        Script,
    },
    AdviceInjector, DebugOptions, Felt, FieldElement, Operation, ProgramInputs, StackTopState,
    StarkField, Word, AUX_TRACE_WIDTH, DECODER_TRACE_WIDTH, MIN_STACK_DEPTH, MIN_TRACE_LEN,
    NUM_STACK_HELPER_COLS, RANGE_CHECK_TRACE_WIDTH, STACK_TRACE_WIDTH, SYS_TRACE_WIDTH,
};

mod operations;

mod system;
use system::System;
pub use system::FMP_MIN;

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

mod range;
use range::RangeChecker;

mod hasher;
use hasher::Hasher;

mod bitwise;
use bitwise::Bitwise;

mod memory;
use memory::Memory;

mod advice;
use advice::AdviceProvider;

mod aux_table;
use aux_table::AuxTable;

mod trace;
pub use trace::ExecutionTrace;
use trace::TraceFragment;

mod errors;
pub use errors::ExecutionError;

mod debug;
pub use debug::{VmState, VmStateIterator};

// TYPE ALIASES
// ================================================================================================

type SysTrace = [Vec<Felt>; SYS_TRACE_WIDTH];
type DecoderTrace = [Vec<Felt>; DECODER_TRACE_WIDTH];
type StackTrace = [Vec<Felt>; STACK_TRACE_WIDTH];
type RangeCheckTrace = [Vec<Felt>; RANGE_CHECK_TRACE_WIDTH];
type AuxTableTrace = [Vec<Felt>; AUX_TRACE_WIDTH]; // TODO: potentially rename to AuxiliaryTrace

// EXECUTOR
// ================================================================================================

/// Returns an execution trace resulting from executing the provided script against the provided
/// inputs.
pub fn execute(script: &Script, inputs: &ProgramInputs) -> Result<ExecutionTrace, ExecutionError> {
    let mut process = Process::new(inputs.clone());
    process.execute_code_block(script.root())?;
    // TODO: make sure program hash from script and trace are the same
    Ok(ExecutionTrace::new(process, *script.hash()))
}

/// Returns an iterator that allows callers to step through each execution and inspect
/// vm state information along side.
pub fn execute_iter(script: &Script, inputs: &ProgramInputs) -> VmStateIterator {
    let mut process = Process::new_debug(inputs.clone());
    let result = process.execute_code_block(script.root());
    VmStateIterator::new(process, result)
}

// PROCESS
// ================================================================================================

pub struct Process {
    system: System,
    decoder: Decoder,
    stack: Stack,
    range: RangeChecker,
    hasher: Hasher,
    bitwise: Bitwise,
    memory: Memory,
    advice: AdviceProvider,
}

impl Process {
    fn initialize(inputs: ProgramInputs, keep_overflow_trace: bool) -> Self {
        Self {
            system: System::new(MIN_TRACE_LEN),
            decoder: Decoder::new(),
            stack: Stack::new(&inputs, MIN_TRACE_LEN, keep_overflow_trace),
            range: RangeChecker::new(),
            hasher: Hasher::new(),
            bitwise: Bitwise::new(),
            memory: Memory::new(),
            advice: AdviceProvider::new(inputs),
        }
    }

    /// Creates a new process with the provided inputs.
    pub fn new(inputs: ProgramInputs) -> Self {
        Self::initialize(inputs, false)
    }

    /// Creates a new process with provided inputs and debug options enabled.
    pub fn new_debug(inputs: ProgramInputs) -> Self {
        Self::initialize(inputs, true)
    }

    // CODE BLOCK EXECUTORS
    // --------------------------------------------------------------------------------------------

    /// Executes the specified [CodeBlock].
    ///
    /// # Errors
    /// Returns an [ExecutionError] if executing the specified block fails for any reason.
    pub fn execute_code_block(&mut self, block: &CodeBlock) -> Result<(), ExecutionError> {
        match block {
            CodeBlock::Join(block) => self.execute_join_block(block),
            CodeBlock::Split(block) => self.execute_split_block(block),
            CodeBlock::Loop(block) => self.execute_loop_block(block),
            CodeBlock::Span(block) => self.execute_span_block(block),
            CodeBlock::Proxy(_) => Err(ExecutionError::UnexecutableCodeBlock(block.clone())),
            _ => Err(ExecutionError::UnsupportedCodeBlock(block.clone())),
        }
    }

    /// Executes the specified [Join] block.
    #[inline(always)]
    fn execute_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        self.start_join_block(block)?;

        // execute first and then second child of the join block
        self.execute_code_block(block.first())?;
        self.execute_code_block(block.second())?;

        self.end_join_block(block)
    }

    /// Executes the specified [Split] block.
    #[inline(always)]
    fn execute_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        // start the SPLIT block; this also pops the stack and returns the popped element
        let condition = self.start_split_block(block)?;

        // execute either the true or the false branch of the split block based on the condition
        if condition == Felt::ONE {
            self.execute_code_block(block.on_true())?;
        } else if condition == Felt::ZERO {
            self.execute_code_block(block.on_false())?;
        } else {
            return Err(ExecutionError::NotBinaryValue(condition));
        }

        self.end_split_block(block)
    }

    /// Executes the specified [Loop] block.
    #[inline(always)]
    fn execute_loop_block(&mut self, block: &Loop) -> Result<(), ExecutionError> {
        // start the LOOP block; this also pops the stack and returns the popped element
        let condition = self.start_loop_block(block)?;

        // if the top of the stack is ONE, execute the loop body; otherwise skip the loop body
        if condition == Felt::ONE {
            // execute the loop body at least once
            self.execute_code_block(block.body())?;

            // keep executing the loop body until the condition on the top of the stack is no
            // longer ONE; each iteration of the loop is preceded by executing REPEAT operation
            // which drops the condition from the stack
            while self.stack.peek() == Felt::ONE {
                self.decoder.repeat();
                self.execute_op(Operation::Drop)?;
                self.execute_code_block(block.body())?;
            }

            // end the LOOP block and drop the condition from the stack
            self.end_loop_block(block, true)
        } else if condition == Felt::ZERO {
            // end the LOOP block, but don't drop the condition from the stack because it was
            // already dropped when we started the LOOP block
            self.end_loop_block(block, false)
        } else {
            Err(ExecutionError::NotBinaryValue(condition))
        }
    }

    /// Executes the specified [Span] block.
    #[inline(always)]
    fn execute_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.start_span_block(block)?;

        // execute the first operation batch
        self.execute_op_batch(&block.op_batches()[0])?;

        // if the span contains more operation batches, execute them. each additional batch is
        // preceded by a RESPAN operation; executing RESPAN operation does not change the state
        // of the stack
        for op_batch in block.op_batches().iter().skip(1) {
            self.decoder.respan(op_batch);
            self.execute_op(Operation::Noop)?;
            self.execute_op_batch(op_batch)?;
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
    fn execute_op_batch(&mut self, batch: &OpBatch) -> Result<(), ExecutionError> {
        let op_counts = batch.op_counts();
        let mut op_idx = 0;
        let mut group_idx = 0;
        let mut next_group_idx = 1;

        // round up the number of groups to be processed to the next power of two; we do this
        // because the processor requires the number of groups to be either 1, 2, 4, or 8; if
        // the actual number of groups is smaller, we'll pad the batch with NOOPs at the end
        let num_batch_groups = batch.num_groups().next_power_of_two();

        // execute operations in the batch one by one
        for &op in batch.ops() {
            if op.is_decorator() {
                // if the operation is a decorator, it has no side effects and, thus, we don't
                // need to decode it
                self.execute_op(op)?;
                continue;
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
                self.decoder.start_op_group(Felt::ZERO);
            }
        }

        Ok(())
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn get_memory_value(&self, addr: u64) -> Option<Word> {
        self.memory.get_value(addr)
    }

    pub fn to_components(self) -> (System, Decoder, Stack, RangeChecker, AuxTable) {
        let aux_table = AuxTable::new(self.hasher, self.bitwise, self.memory);
        (self.system, self.decoder, self.stack, self.range, aux_table)
    }
}
