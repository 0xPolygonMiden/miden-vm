use alloc::vec::Vec;
use core::cmp::min;

use memory::Memory;
use miden_air::RowIndex;
use vm_core::{
    mast::{
        BasicBlockNode, CallNode, DynNode, ExternalNode, JoinNode, LoopNode, MastForest, MastNode,
        MastNodeId, OpBatch, SplitNode, OP_GROUP_SIZE,
    },
    stack::MIN_STACK_DEPTH,
    utils::range,
    Decorator, DecoratorIterator, Felt, Kernel, Operation, Program, StackOutputs, Word, EMPTY_WORD,
    ONE, WORD_SIZE, ZERO,
};

use crate::{
    operations::utils::assert_binary, utils::resolve_external_node, ContextId, ExecutionError,
    Host, ProcessState, FMP_MIN, SYSCALL_FMP_MIN,
};

// temporary module to
pub mod experiments;

mod memory;

// Ops
mod crypto_ops;
mod field_ops;
mod fri_ops;
mod horner_ops;
mod io_ops;
mod stack_ops;
mod sys_ops;
mod u32_ops;

#[cfg(test)]
mod tests;

/// The size of the stack buffer.
///
/// Note: This value is much larger than it needs to be for the majority of programs. However, some
/// existing programs need it (e.g. `std::math::secp256k1::group::gen_mul`), so we're forced to push
/// it up. At this high a value, we're starting to see some performance degradation on benchmarks.
/// For example, the blake3 benchmark went from 285 MHz to 250 MHz (~10% degradation). Perhaps a
/// better solution would be to make this value much smaller (~1000), and then fallback to a `Vec`
/// if the stack overflows.
const STACK_BUFFER_SIZE: usize = 6650;

/// The initial position of the top of the stack in the stack buffer.
///
/// We place this value close to 0 because if a program hits the limit, it's much more likely to hit
/// the upper bound than the lower bound, since hitting the lower bound only occurs when you drop
/// 0's that were generated automatically to keep the stack depth at 16. In practice, if this
/// occurs, it is most likely a bug.
const INITIAL_STACK_TOP_IDX: usize = 50;

/// A fast processor which doesn't generate any trace.
#[derive(Debug)]
pub struct FastProcessor {
    /// The stack is stored in reverse order, so that the last element is at the top of the stack.
    pub(super) stack: [Felt; STACK_BUFFER_SIZE],
    /// The index of the top of the stack.
    stack_top_idx: usize,
    /// The index of the bottom of the stack.
    stack_bot_idx: usize,
    /// The counter which keeps track of the number of instructions that we can execute without
    /// hitting the bounds of `stack`.
    bounds_check_counter: usize,

    /// The current clock cycle.
    pub(super) clk: RowIndex,

    /// The current context ID.
    pub(super) ctx: ContextId,

    /// The free memory pointer.
    pub(super) fmp: Felt,

    /// Whether we are currently in a syscall.
    in_syscall: bool,

    /// The hash of the function that called into the current context, or `[ZERO, ZERO, ZERO,
    /// ZERO]` if we are in the first context (i.e. when `call_stack` is empty).
    pub(super) fn_hash: Word,

    /// A map from (context_id, word_address) to the word stored starting at that memory location.
    pub(super) memory: Memory,

    /// The call stack is used when starting a new execution context (from a `call`, `syscall` or
    /// `dyncall`) to keep track of the information needed to return to the previous context upon
    /// return. It is a stack since calls can be nested.
    call_stack: Vec<ExecutionContextInfo>,

    /// Whether to enable debug statements and tracing.
    in_debug_mode: bool,
}

impl FastProcessor {
    // CONSTRUCTORS
    // ----------------------------------------------------------------------------------------------

    /// Creates a new `FastProcessor` instance with the given stack inputs.
    ///
    /// The stack inputs are expected to be stored in reverse order. For example, if `stack_inputs =
    /// [1,2,3]`, then the stack will be initialized as `[3,2,1,0,0,...]`, with `3` being on
    /// top.
    pub fn new(stack_inputs: Vec<Felt>) -> Self {
        Self::initialize(stack_inputs, false)
    }

    /// Creates a new `FastProcessor` instance with the given stack inputs, set to debug mode.
    ///
    /// The stack inputs are expected to be stored in reverse order. For example, if `stack_inputs =
    /// [1,2,3]`, then the stack will be initialized as `[3,2,1,0,0,...]`, with `3` being on
    /// top.
    pub fn new_debug(stack_inputs: Vec<Felt>) -> Self {
        Self::initialize(stack_inputs, true)
    }

    fn initialize(stack_inputs: Vec<Felt>, in_debug_mode: bool) -> Self {
        assert!(stack_inputs.len() <= MIN_STACK_DEPTH);

        let stack_top_idx = INITIAL_STACK_TOP_IDX;
        let stack = {
            let mut stack = [ZERO; STACK_BUFFER_SIZE];
            let bottom_idx = stack_top_idx - stack_inputs.len();

            stack[bottom_idx..stack_top_idx].copy_from_slice(&stack_inputs);
            stack
        };

        let stack_bot_idx = stack_top_idx - MIN_STACK_DEPTH;

        let bounds_check_counter = stack_bot_idx;

        FastProcessor {
            stack,
            stack_top_idx,
            stack_bot_idx,
            bounds_check_counter,
            clk: 0_u32.into(),
            ctx: 0_u32.into(),
            fmp: Felt::new(FMP_MIN),
            in_syscall: false,
            fn_hash: EMPTY_WORD,
            memory: Memory::new(),
            call_stack: Vec::new(),
            in_debug_mode,
        }
    }

    // ACCESSORS
    // -------------------------------------------------------------------------------------------

    /// Returns the stack, such that the top of the stack is at the last index of the returned
    /// slice.
    pub fn stack(&self) -> &[Felt] {
        &self.stack[self.stack_bot_idx..self.stack_top_idx]
    }

    /// Returns the element on the stack at index `idx`.
    pub fn stack_get(&self, idx: usize) -> Felt {
        self.stack[self.stack_top_idx - idx - 1]
    }

    /// Returns the word on the stack at index `idx`.
    ///
    /// Specifically, word 0 is defined by the first 4 elements of the stack, word 1 is defined
    /// by the next 4 elements etc. Since the top of the stack contains 4 word, the highest valid
    /// word index is 3.
    ///
    /// The words are created in reverse order. For example, for word 0 the top element of the
    /// stack will be at the last position in the word.
    pub fn stack_get_word(&self, idx: usize) -> Word {
        let word_start_idx = self.stack_top_idx - (WORD_SIZE * (idx + 1));
        self.stack[range(word_start_idx, WORD_SIZE)].try_into().unwrap()
    }

    // EXECUTE
    // -------------------------------------------------------------------------------------------

    pub fn execute(
        mut self,
        program: &Program,
        host: &mut impl Host,
    ) -> Result<StackOutputs, ExecutionError> {
        self.execute_impl(program, host)
    }

    /// Executes the given program and returns the stack outputs.
    ///
    /// This function is mainly split out of `execute()` for testing purposes.
    fn execute_impl(
        &mut self,
        program: &Program,
        host: &mut impl Host,
    ) -> Result<StackOutputs, ExecutionError> {
        self.execute_mast_node(
            program.entrypoint(),
            program.mast_forest(),
            program.kernel(),
            host,
        )?;

        StackOutputs::new(
            self.stack[self.stack_bot_idx..self.stack_top_idx]
                .iter()
                .rev()
                .copied()
                .collect(),
        )
        .map_err(|_| {
            ExecutionError::OutputStackOverflow(
                self.stack_top_idx - self.stack_bot_idx - MIN_STACK_DEPTH,
            )
        })
    }

    fn execute_mast_node(
        &mut self,
        node_id: MastNodeId,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let node = program
            .get_node_by_id(node_id)
            .ok_or(ExecutionError::MastNodeNotFoundInForest { node_id })?;

        // Corresponds to the row inserted for the node added to the trace (e.g. JOIN, SPLIT, etc).
        // `External` is the only node that doesn't insert a row in the trace.
        if !matches!(node, MastNode::External(_)) {
            self.clk += 1_u32;
        }

        match node {
            MastNode::Block(basic_block_node) => {
                self.execute_basic_block_node(basic_block_node, program, host)?
            },
            MastNode::Join(join_node) => {
                self.execute_join_node(join_node, program, kernel, host)?
            },
            MastNode::Split(split_node) => {
                self.execute_split_node(split_node, program, kernel, host)?
            },
            MastNode::Loop(loop_node) => {
                self.execute_loop_node(loop_node, program, kernel, host)?
            },
            MastNode::Call(call_node) => {
                self.execute_call_node(call_node, program, kernel, host)?
            },
            MastNode::Dyn(dyn_node) => self.execute_dyn_node(dyn_node, program, kernel, host)?,
            MastNode::External(external_node) => {
                self.execute_external_node(external_node, kernel, host)?
            },
        }

        // Corresponds to the row inserted for the `END` added to the trace. `External` is the only
        // node that doesn't insert a corresponding `END` row in the trace.
        if !matches!(node, MastNode::External(_)) {
            self.clk += 1_u32;
        }

        Ok(())
    }

    fn execute_join_node(
        &mut self,
        join_node: &JoinNode,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        self.execute_mast_node(join_node.first(), program, kernel, host)?;
        self.execute_mast_node(join_node.second(), program, kernel, host)
    }

    fn execute_split_node(
        &mut self,
        split_node: &SplitNode,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let condition = self.stack[self.stack_top_idx - 1];

        // drop the condition from the stack
        self.decrement_stack_size();

        // execute the appropriate branch
        if condition == ONE {
            self.execute_mast_node(split_node.on_true(), program, kernel, host)
        } else if condition == ZERO {
            self.execute_mast_node(split_node.on_false(), program, kernel, host)
        } else {
            Err(ExecutionError::NotBinaryValue(condition))
        }
    }

    fn execute_loop_node(
        &mut self,
        loop_node: &LoopNode,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // The loop condition is checked after the loop body is executed.
        let mut condition = self.stack[self.stack_top_idx - 1];

        // drop the condition from the stack
        self.decrement_stack_size();

        // execute the loop body as long as the condition is true
        while condition == ONE {
            self.execute_mast_node(loop_node.body(), program, kernel, host)?;

            // check the loop condition, and drop it from the stack
            condition = self.stack[self.stack_top_idx - 1];
            self.decrement_stack_size();

            // this clock increment is for the row inserted for the `REPEAT` node added to
            // the trace on each iteration. It needs to be at the end of this loop (instead
            // of at the beginning), otherwise we get an off-by-one error when comparing
            // with [crate::Process].
            if condition == ONE {
                self.clk += 1;
            }
        }

        if condition == ZERO {
            Ok(())
        } else {
            Err(ExecutionError::NotBinaryValue(condition))
        }
    }

    fn execute_call_node(
        &mut self,
        call_node: &CallNode,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // call or syscall are not allowed inside a syscall
        if self.in_syscall {
            let instruction = if call_node.is_syscall() { "syscall" } else { "call" };
            return Err(ExecutionError::CallInSyscall(instruction));
        }

        let callee_hash = program
            .get_node_by_id(call_node.callee())
            .ok_or(ExecutionError::MastNodeNotFoundInForest { node_id: call_node.callee() })?
            .digest()
            .into();

        self.save_context_and_truncate_stack();

        if call_node.is_syscall() {
            // check if the callee is in the kernel
            let callee_digest = program
                .get_node_by_id(call_node.callee())
                .ok_or_else(|| ExecutionError::MastNodeNotFoundInForest {
                    node_id: call_node.callee(),
                })?
                .digest();
            if !kernel.contains_proc(callee_digest) {
                return Err(ExecutionError::SyscallTargetNotInKernel(callee_digest));
            }

            // set the system registers to the syscall context
            self.ctx = ContextId::root();
            self.fmp = SYSCALL_FMP_MIN.into();
            self.in_syscall = true;
        } else {
            // set the system registers to the callee context
            self.ctx = self.clk.into();
            self.fmp = Felt::new(FMP_MIN);
            self.fn_hash = callee_hash;
        }

        // Execute the callee.
        self.execute_mast_node(call_node.callee(), program, kernel, host)?;

        // when returning from a function call or a syscall, restore the context of the
        // system registers and the operand stack to what it was prior to
        // the call.
        self.restore_context()
    }

    fn execute_dyn_node(
        &mut self,
        dyn_node: &DynNode,
        program: &MastForest,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        // dyn calls are not allowed inside a syscall
        if dyn_node.is_dyncall() && self.in_syscall {
            return Err(ExecutionError::CallInSyscall("dyncall"));
        }

        // Retrieve callee hash from memory, using stack top as the memory address.
        let callee_hash = {
            let mem_addr = self.stack[self.stack_top_idx - 1];
            self.memory.read_word(self.ctx, mem_addr, self.clk).copied()?
        };

        // Drop the memory address from the stack. This needs to be done BEFORE saving the context,
        // because the next instruction starts with a "shifted left" stack.
        self.decrement_stack_size();

        // For dyncall, save the context and reset it.
        if dyn_node.is_dyncall() {
            self.save_context_and_truncate_stack();
            self.ctx = self.clk.into();
            self.fmp = Felt::new(FMP_MIN);
            self.fn_hash = callee_hash;
        };

        // if the callee is not in the program's MAST forest, try to find a MAST forest for it in
        // the host (corresponding to an external library loaded in the host); if none are
        // found, return an error.
        match program.find_procedure_root(callee_hash.into()) {
            Some(callee_id) => self.execute_mast_node(callee_id, program, kernel, host)?,
            None => {
                let mast_forest = host
                    .get_mast_forest(&callee_hash.into())
                    .ok_or_else(|| ExecutionError::DynamicNodeNotFound(callee_hash.into()))?;

                // We limit the parts of the program that can be called externally to procedure
                // roots, even though MAST doesn't have that restriction.
                let root_id = mast_forest.find_procedure_root(callee_hash.into()).ok_or(
                    ExecutionError::MalformedMastForestInHost { root_digest: callee_hash.into() },
                )?;

                self.execute_mast_node(root_id, &mast_forest, kernel, host)?
            },
        }

        // For dyncall, restore the context.
        if dyn_node.is_dyncall() {
            self.restore_context()?;
        }

        Ok(())
    }

    fn execute_external_node(
        &mut self,
        external_node: &ExternalNode,
        kernel: &Kernel,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let (root_id, mast_forest) = resolve_external_node(external_node, host)?;

        self.execute_mast_node(root_id, &mast_forest, kernel, host)
    }

    // Note: when executing individual ops, we do not increment the clock by 1 at every iteration
    // for performance reasons (~25% performance drop). Hence, `self.clk` cannot be used directly to
    // determine the number of operations executed in a program.
    fn execute_basic_block_node(
        &mut self,
        basic_block_node: &BasicBlockNode,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let mut batch_offset_in_block = 0;
        let mut op_batches = basic_block_node.op_batches().iter();
        let mut decorator_ids = basic_block_node.decorator_iter();

        // execute first op batch
        if let Some(first_op_batch) = op_batches.next() {
            self.execute_op_batch(
                first_op_batch,
                &mut decorator_ids,
                batch_offset_in_block,
                program,
                host,
            )?;
            batch_offset_in_block += first_op_batch.ops().len();
        }

        // execute the rest of the op batches
        for op_batch in op_batches {
            // increment clock to account for `RESPAN`
            self.clk += 1;

            self.execute_op_batch(
                op_batch,
                &mut decorator_ids,
                batch_offset_in_block,
                program,
                host,
            )?;
            batch_offset_in_block += op_batch.ops().len();
        }

        // update clock with all the operations that executed
        self.clk += batch_offset_in_block as u32;

        // execute any decorators which have not been executed during span ops execution; this can
        // happen for decorators appearing after all operations in a block. these decorators are
        // executed after SPAN block is closed to make sure the VM clock cycle advances beyond the
        // last clock cycle of the SPAN block ops.
        //
        // Note: we pass in `op_idx_in_batch = 1` as a hack, since in `Process`, those decorators
        // are executed after the `END` block (which we increment the `clk` outside of this
        // function). Setting `op_idx_in_batch = 1` accounts for this discrepancy.
        for &decorator_id in decorator_ids {
            let decorator = program
                .get_decorator_by_id(decorator_id)
                .ok_or(ExecutionError::DecoratorNotFoundInForest { decorator_id })?;
            self.execute_decorator(decorator, 1, host)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn execute_op_batch(
        &mut self,
        batch: &OpBatch,
        decorators: &mut DecoratorIterator,
        batch_offset_in_block: usize,
        program: &MastForest,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        let op_counts = batch.op_counts();
        let mut op_idx_in_group = 0;
        let mut group_idx = 0;
        let mut next_group_idx = 1;

        // round up the number of groups to be processed to the next power of two; we do this
        // because the processor requires the number of groups to be either 1, 2, 4, or 8; if
        // the actual number of groups is smaller, we'll pad the batch with NOOPs at the end
        let num_batch_groups = batch.num_groups().next_power_of_two();

        // execute operations in the batch one by one
        for (op_idx_in_batch, op) in batch.ops().iter().enumerate() {
            while let Some(&decorator_id) =
                decorators.next_filtered(batch_offset_in_block + op_idx_in_batch)
            {
                let decorator = program
                    .get_decorator_by_id(decorator_id)
                    .ok_or(ExecutionError::DecoratorNotFoundInForest { decorator_id })?;
                self.execute_decorator(decorator, op_idx_in_batch, host)?;
            }

            // decode and execute the operation
            self.execute_op(op, batch_offset_in_block + op_idx_in_batch, host)?;

            // if the operation carries an immediate value, the value is stored at the next group
            // pointer; so, we advance the pointer to the following group
            let has_imm = op.imm_value().is_some();
            if has_imm {
                next_group_idx += 1;
            }

            // determine if we've executed all non-decorator operations in a group
            if op_idx_in_group == op_counts[group_idx] - 1 {
                // if we are at the end of the group, first check if the operation carries an
                // immediate value
                if has_imm {
                    // an operation with an immediate value cannot be the last operation in a group
                    // so, we need execute a NOOP after it. In this processor, we increment the
                    // clock to account for the NOOP.
                    debug_assert!(op_idx_in_group < OP_GROUP_SIZE - 1, "invalid op index");
                    self.clk += 1;
                }

                // then, move to the next group and reset operation index
                group_idx = next_group_idx;
                next_group_idx += 1;
                op_idx_in_group = 0;
            } else {
                // TODO(plafer): this is probably slow - see if we can avoid it.
                // if we are not at the end of the group, just increment the operation index
                op_idx_in_group += 1;
            }
        }

        // make sure we execute the required number of operation groups; this would happen when the
        // actual number of operation groups was not a power of two. In this processor, this
        // corresponds to incrementing the clock by the number of empty op groups (i.e. 1 NOOP
        // executed per missing op group).

        self.clk += (num_batch_groups - group_idx) as u32;

        Ok(())
    }

    /// Executes the specified decorator
    fn execute_decorator(
        &mut self,
        decorator: &Decorator,
        op_idx_in_batch: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        match decorator {
            Decorator::Debug(options) => {
                if self.in_debug_mode {
                    host.on_debug(ProcessState::new_fast(self, op_idx_in_batch), options)?;
                }
            },
            Decorator::AsmOp(_assembly_op) => {
                // do nothing
            },
            Decorator::Trace(id) => {
                host.on_trace(ProcessState::new_fast(self, op_idx_in_batch), *id)?;
            },
        };
        Ok(())
    }

    fn execute_op(
        &mut self,
        operation: &Operation,
        op_idx: usize,
        host: &mut impl Host,
    ) -> Result<(), ExecutionError> {
        if self.bounds_check_counter == 0 {
            let err_str = if self.stack_top_idx - MIN_STACK_DEPTH == 0 {
                "stack underflow"
            } else {
                "stack overflow"
            };
            return Err(ExecutionError::FailedToExecuteProgram(err_str));
        }

        match operation {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => {
                // do nothing
            },
            Operation::Assert(err_code) => self.op_assert(*err_code, op_idx, host)?,
            Operation::FmpAdd => self.op_fmp_add(),
            Operation::FmpUpdate => self.op_fmp_update()?,
            Operation::SDepth => self.op_sdepth(),
            Operation::Caller => self.op_caller()?,
            Operation::Clk => self.op_clk(op_idx)?,
            Operation::Emit(event_id) => self.op_emit(*event_id, op_idx, host)?,

            // ----- flow control operations ------------------------------------------------------
            // control flow operations are never executed directly
            Operation::Join => unreachable!("control flow operation"),
            Operation::Split => unreachable!("control flow operation"),
            Operation::Loop => unreachable!("control flow operation"),
            Operation::Call => unreachable!("control flow operation"),
            Operation::SysCall => unreachable!("control flow operation"),
            Operation::Dyn => unreachable!("control flow operation"),
            Operation::Dyncall => unreachable!("control flow operation"),
            Operation::Span => unreachable!("control flow operation"),
            Operation::Repeat => unreachable!("control flow operation"),
            Operation::Respan => unreachable!("control flow operation"),
            Operation::End => unreachable!("control flow operation"),
            Operation::Halt => unreachable!("control flow operation"),

            // ----- field operations -------------------------------------------------------------
            Operation::Add => self.op_add()?,
            Operation::Neg => self.op_neg()?,
            Operation::Mul => self.op_mul()?,
            Operation::Inv => self.op_inv(op_idx)?,
            Operation::Incr => self.op_incr()?,
            Operation::And => self.op_and()?,
            Operation::Or => self.op_or()?,
            Operation::Not => self.op_not()?,
            Operation::Eq => self.op_eq()?,
            Operation::Eqz => self.op_eqz()?,
            Operation::Expacc => self.op_expacc(),
            Operation::Ext2Mul => self.op_ext2mul(),

            // ----- u32 operations ---------------------------------------------------------------
            Operation::U32split => self.u32_split()?,
            Operation::U32add => self.u32_add()?,
            Operation::U32add3 => self.u32_add3()?,
            Operation::U32sub => self.u32_sub(op_idx)?,
            Operation::U32mul => self.u32_mul()?,
            Operation::U32madd => self.u32_madd()?,
            Operation::U32div => self.u32_div(op_idx)?,
            Operation::U32and => self.u32_and()?,
            Operation::U32xor => self.u32_xor()?,
            Operation::U32assert2(err_code) => self.u32_assert2(*err_code)?,

            // ----- stack manipulation -----------------------------------------------------------
            Operation::Pad => self.op_pad(),
            Operation::Drop => self.decrement_stack_size(),
            Operation::Dup0 => self.dup_nth(0),
            Operation::Dup1 => self.dup_nth(1),
            Operation::Dup2 => self.dup_nth(2),
            Operation::Dup3 => self.dup_nth(3),
            Operation::Dup4 => self.dup_nth(4),
            Operation::Dup5 => self.dup_nth(5),
            Operation::Dup6 => self.dup_nth(6),
            Operation::Dup7 => self.dup_nth(7),
            Operation::Dup9 => self.dup_nth(9),
            Operation::Dup11 => self.dup_nth(11),
            Operation::Dup13 => self.dup_nth(13),
            Operation::Dup15 => self.dup_nth(15),
            Operation::Swap => self.op_swap(),
            Operation::SwapW => self.swapw_nth(1),
            Operation::SwapW2 => self.swapw_nth(2),
            Operation::SwapW3 => self.swapw_nth(3),
            Operation::SwapDW => self.op_swap_double_word(),
            Operation::MovUp2 => self.rotate_left(3),
            Operation::MovUp3 => self.rotate_left(4),
            Operation::MovUp4 => self.rotate_left(5),
            Operation::MovUp5 => self.rotate_left(6),
            Operation::MovUp6 => self.rotate_left(7),
            Operation::MovUp7 => self.rotate_left(8),
            Operation::MovUp8 => self.rotate_left(9),
            Operation::MovDn2 => self.rotate_right(3),
            Operation::MovDn3 => self.rotate_right(4),
            Operation::MovDn4 => self.rotate_right(5),
            Operation::MovDn5 => self.rotate_right(6),
            Operation::MovDn6 => self.rotate_right(7),
            Operation::MovDn7 => self.rotate_right(8),
            Operation::MovDn8 => self.rotate_right(9),
            Operation::CSwap => self.op_cswap()?,
            Operation::CSwapW => self.op_cswapw()?,

            // ----- input / output ---------------------------------------------------------------
            Operation::Push(element) => self.op_push(*element),
            Operation::AdvPop => self.op_advpop(op_idx, host)?,
            Operation::AdvPopW => self.op_advpopw(op_idx, host)?,
            Operation::MLoadW => self.op_mloadw(op_idx)?,
            Operation::MStoreW => self.op_mstorew(op_idx)?,
            Operation::MLoad => self.op_mload()?,
            Operation::MStore => self.op_mstore()?,
            Operation::MStream => self.op_mstream(op_idx)?,
            Operation::Pipe => self.op_pipe(op_idx, host)?,

            // ----- cryptographic operations -----------------------------------------------------
            Operation::HPerm => self.op_hperm(),
            Operation::MpVerify(err_code) => self.op_mpverify(*err_code, host)?,
            Operation::MrUpdate => self.op_mrupdate(host)?,
            Operation::FriE2F4 => self.op_fri_ext2fold4()?,
            Operation::HornerBase => self.op_horner_eval_base(op_idx)?,
            Operation::HornerExt => self.op_horner_eval_ext(op_idx)?,
        }

        Ok(())
    }

    // HELPERS
    // ----------------------------------------------------------------------------------------------

    /// Increments the stack top pointer by 1.
    ///
    /// The bottom of the stack is never affected by this operation.
    #[inline(always)]
    fn increment_stack_size(&mut self) {
        self.stack_top_idx += 1;
        self.update_bounds_check_counter();
    }

    /// Decrements the stack top pointer by 1.
    ///
    /// The bottom of the stack is only decremented in cases where the stack depth would become less
    /// than 16.
    #[inline(always)]
    fn decrement_stack_size(&mut self) {
        self.stack_top_idx -= 1;
        self.stack_bot_idx = min(self.stack_bot_idx, self.stack_top_idx - MIN_STACK_DEPTH);
        self.update_bounds_check_counter();
    }

    /// Returns the size of the stack.
    #[inline(always)]
    fn stack_size(&self) -> usize {
        self.stack_top_idx - self.stack_bot_idx
    }

    /// Updates the bounds check counter.
    ///
    /// The bounds check counter is decremented by 1. If it reaches 0, it is reset to the minimum of
    /// the stack depth from the low end and the high end of the stack buffer.
    ///
    /// The purpose of the bounds check counter is to ensure that we never access the stack buffer
    /// at an out-of-bounds index.
    #[inline(always)]
    fn update_bounds_check_counter(&mut self) {
        self.bounds_check_counter -= 1;

        if self.bounds_check_counter == 0 {
            // We will need to check the bounds either because we reach the low end or the high end
            // of the stack buffer. There are two worst cases that we are concerned about:
            // - we only execute instructions that decrease stack depth
            // - we only execute instructions that increase stack depth
            //
            // In the first case, we will hit the low end of the stack buffer; in the second case,
            // we will hit the high end of the stack buffer. We set the number of instructions that
            // is safe to execute to be the minimum of these two worst cases.

            self.bounds_check_counter =
                min(self.stack_top_idx - MIN_STACK_DEPTH, STACK_BUFFER_SIZE - self.stack_top_idx);
        }
    }

    /// Saves the current execution context and truncates the stack to 16 elements in preparation to
    /// start a new execution context.
    fn save_context_and_truncate_stack(&mut self) {
        let overflow_stack = if self.stack_size() > MIN_STACK_DEPTH {
            // save the overflow stack, and zero out the buffer
            let overflow_stack =
                self.stack[self.stack_bot_idx..self.stack_top_idx - MIN_STACK_DEPTH].to_vec();
            self.stack[self.stack_bot_idx..self.stack_top_idx - MIN_STACK_DEPTH].fill(ZERO);

            overflow_stack
        } else {
            Vec::new()
        };

        self.stack_bot_idx = self.stack_top_idx - MIN_STACK_DEPTH;

        self.call_stack.push(ExecutionContextInfo {
            overflow_stack,
            ctx: self.ctx,
            fn_hash: self.fn_hash,
            fmp: self.fmp,
        });
    }

    /// Restores the execution context to the state it was in before the last `call`, `syscall` or
    /// `dyncall`.
    ///
    /// This includes restoring the overflow stack and the system parameters.
    ///
    /// # Errors
    /// - Returns an error if the overflow stack is larger than the space available in the stack
    ///   buffer.
    fn restore_context(&mut self) -> Result<(), ExecutionError> {
        // when a call/dyncall/syscall node ends, stack depth must be exactly 16.
        if self.stack_size() > MIN_STACK_DEPTH {
            return Err(ExecutionError::InvalidStackDepthOnReturn(self.stack_size()));
        }

        let ctx_info = self
            .call_stack
            .pop()
            .expect("execution context stack should never be empty when restoring context");

        // restore the overflow stack
        {
            let overflow_len = ctx_info.overflow_stack.len();
            if overflow_len > self.stack_bot_idx {
                return Err(ExecutionError::FailedToExecuteProgram(
                    "stack underflow when restoring context",
                ));
            }

            self.stack[range(self.stack_bot_idx - overflow_len, overflow_len)]
                .copy_from_slice(&ctx_info.overflow_stack);
            self.stack_bot_idx -= overflow_len;
        }

        // restore system parameters
        self.ctx = ctx_info.ctx;
        self.fmp = ctx_info.fmp;
        self.in_syscall = false;
        self.fn_hash = ctx_info.fn_hash;

        Ok(())
    }
}

// EXECUTION CONTEXT INFO
// ===============================================================================================

/// Information about the execution context.
///
/// This struct is used to keep track of the information needed to return to the previous context
/// upon return from a `call`, `syscall` or `dyncall`.
#[derive(Debug)]
struct ExecutionContextInfo {
    /// This stores all the elements on the stack at the call site, excluding the top 16 elements.
    /// This corresponds to the overflow table in [crate::Process].
    overflow_stack: Vec<Felt>,
    ctx: ContextId,
    fn_hash: Word,
    fmp: Felt,
}
