use alloc::vec::Vec;
use core::cmp::min;

use memory::Memory;
use miden_air::RowIndex;
use vm_core::{
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::range,
    Felt, Operation, Program, StackOutputs, Word, EMPTY_WORD, ONE, WORD_SIZE, ZERO,
};

use crate::{
    operations::utils::assert_binary, utils::resolve_external_node, ContextId, ExecutionError,
    Host, FMP_MIN, SYSCALL_FMP_MIN,
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

/// A fast processor which doesn't generate any trace.
#[derive(Debug)]
pub struct SpeedyGonzales<const N: usize> {
    /// The stack is stored in reverse order, so that the last element is at the top of the stack.
    stack: [Felt; N],
    /// The index of the top of the stack.
    stack_top_idx: usize,
    /// The index of the bottom of the stack.
    stack_bot_idx: usize,
    /// The counter which keeps track of the number of instructions that we can execute without
    /// hitting the bounds of `stack`.
    bounds_check_counter: usize,

    // TODO(plafer): add a bunch of tests to make sure that with all control flow operations we
    // keep track of the clk correctly. Specifically re: the `execute_op(Noop)` pattern.
    clk: RowIndex,

    /// The current context ID.
    ctx: ContextId,

    /// The free memory pointer.
    fmp: Felt,

    /// Whether we are currently in a syscall.
    in_syscall: bool,

    /// The hash of the function that called into the current context, or `[ZERO, ZERO, ZERO,
    /// ZERO]` if we are in the first context (i.e. when `call_stack` is empty).
    fn_hash: Word,

    /// A map from (context_id, word_address) to the word stored starting at that memory location.
    memory: Memory,

    /// The call stack is used when starting a new execution context (from a `call`, `syscall` or
    /// `dyncall`) to keep track of the information needed to return to the previous context upon
    /// return. It is a stack since calls can be nested.
    call_stack: Vec<ExecutionContextInfo>,
}

impl<const N: usize> SpeedyGonzales<N> {
    /// Creates a new `SpeedyGonzales` instance with the given stack inputs.
    ///
    /// The stack inputs are expected to be stored in reverse order. For example, if `stack_inputs =
    /// [1,2,3]`, then the stack will be initialized as `[3,2,1,0,0,...]`, with `3` being on
    /// top.
    pub fn new(stack_inputs: Vec<Felt>) -> Self {
        assert!(stack_inputs.len() <= MIN_STACK_DEPTH);

        // The stack buffer initially looks like
        //
        // | ---x--- | stack_bot_idx | ---16--- | stack_top_idx | ---x--- |
        //
        // That is, we place the middle of the buffer between the 7th and 8th elements of the stack,
        // such that `x = N/2 - 8`. This maximizes the value of `bounds_check_counter`.
        let stack_top_idx = N / 2 + MIN_STACK_DEPTH / 2;
        let stack = {
            let mut stack = [ZERO; N];
            let bottom_idx = stack_top_idx - stack_inputs.len();

            stack[bottom_idx..stack_top_idx].copy_from_slice(&stack_inputs);
            stack
        };

        let stack_bot_idx = stack_top_idx - MIN_STACK_DEPTH;

        let bounds_check_counter = stack_bot_idx;

        SpeedyGonzales {
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
        }
    }

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
        self.execute_mast_node(program.entrypoint(), program.mast_forest(), host)?;

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
                self.execute_basic_block_node(basic_block_node, program)?;
            },
            MastNode::Join(join_node) => {
                self.execute_mast_node(join_node.first(), program, host)?;
                self.execute_mast_node(join_node.second(), program, host)?;
            },
            MastNode::Split(split_node) => {
                let condition = self.stack[self.stack_top_idx - 1];

                // TODO(plafer): test this - specifically if bounds check is updated such that the
                // next stack operation should fail

                // drop the condition from the stack
                self.decrement_stack_size();
                self.update_bounds_check_counter();

                // execute the appropriate branch
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program, host)?;
                } else if condition == ZERO {
                    self.execute_mast_node(split_node.on_false(), program, host)?;
                } else {
                    return Err(ExecutionError::NotBinaryValue(condition));
                }
            },
            MastNode::Loop(loop_node) => {
                // The loop condition is checked after the loop body is executed.
                let mut condition = self.stack[self.stack_top_idx - 1];

                // drop the condition from the stack
                self.decrement_stack_size();

                // execute the loop body as long as the condition is true
                while condition == ONE {
                    self.execute_mast_node(loop_node.body(), program, host)?;

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

                if condition != ZERO {
                    return Err(ExecutionError::NotBinaryValue(condition));
                }
            },
            MastNode::Call(call_node) => {
                // call or syscall are not allowed inside a syscall
                if self.in_syscall {
                    let instruction = if call_node.is_syscall() { "syscall" } else { "call" };
                    return Err(ExecutionError::CallInSyscall(instruction));
                }

                let callee_hash = program
                    .get_node_by_id(call_node.callee())
                    .ok_or(ExecutionError::MastNodeNotFoundInForest {
                        node_id: call_node.callee(),
                    })?
                    .digest()
                    .into();

                self.save_context_and_truncate_stack();

                if call_node.is_syscall() {
                    // TODO(plafer): check if target exists in kernel
                    self.ctx = ContextId::root();
                    self.fmp = SYSCALL_FMP_MIN.into();
                    self.in_syscall = true;
                } else {
                    self.ctx = (self.clk + 1).into();
                    self.fmp = Felt::new(FMP_MIN);
                    self.fn_hash = callee_hash;
                }

                // Execute the callee.
                self.execute_mast_node(call_node.callee(), program, host)?;

                // when a CALL node ends, stack depth must be exactly 16.
                if self.stack_size() > MIN_STACK_DEPTH {
                    return Err(ExecutionError::InvalidStackDepthOnReturn(self.stack_size()));
                }

                // when returning from a function call or a syscall, restore the context of the
                // system registers and the operand stack to what it was prior to
                // the call.
                self.restore_context()?;
            },
            MastNode::Dyn(_dyn_node) => todo!(),
            MastNode::External(external_node) => {
                let (root_id, mast_forest) = resolve_external_node(external_node, host)?;

                self.execute_mast_node(root_id, &mast_forest, host)?;
            },
        }

        // Corresponds to the row inserted for the `END` added to the trace. `External` is the only
        // node that doesn't insert a corresponding `END` row in the trace.
        if !matches!(node, MastNode::External(_)) {
            self.clk += 1_u32;
        }

        Ok(())
    }

    // Note: when executing individual ops, we do not increment the clock by 1 at every iteration
    // for performance reasons (~25% performance drop). Hence, `self.clk` cannot be used directly to
    // determine the number of operations executed in a program.
    fn execute_basic_block_node(
        &mut self,
        basic_block_node: &BasicBlockNode,
        program: &MastForest,
    ) -> Result<(), ExecutionError> {
        // TODO(plafer): this enumerate results in a ~5% performance drop (or about 25 MHz on the
        // fibonacci benchmark). We should find a better way to know the operation index when we
        // need to know the clock. For example, if the operations were stored in a Vec, we could
        // just use the index of the operation in the Vec. If still is still too slow, we could use
        // pointer arithmetic (comparing the address of the operation to the address of the first
        // operation in the Vec).
        for (op_idx, operation) in basic_block_node.operations().enumerate() {
            self.execute_op(operation, op_idx, program)?;
        }

        self.clk += basic_block_node.num_operations();

        Ok(())
    }

    fn execute_op(
        &mut self,
        operation: &Operation,
        op_idx: usize,
        _program: &MastForest,
    ) -> Result<(), ExecutionError> {
        if self.bounds_check_counter == 0 {
            return Err(ExecutionError::FailedToExecuteProgram("stack overflow"));
        }

        match operation {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => {
                // do nothing
            },
            Operation::Assert(err_code) => self.op_assert(*err_code, op_idx)?,
            Operation::FmpAdd => self.op_fmp_add(),
            Operation::FmpUpdate => self.op_fmp_update()?,
            Operation::SDepth => self.op_sdepth(),
            Operation::Caller => self.op_caller()?,
            Operation::Clk => self.op_clk(op_idx)?,
            Operation::Emit(event_id) => self.op_emit(*event_id)?,

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
            Operation::AdvPop => self.adv_pop()?,
            Operation::AdvPopW => self.adv_popw()?,
            Operation::MLoadW => self.op_mloadw(op_idx)?,
            Operation::MStoreW => self.op_mstorew(op_idx)?,
            Operation::MLoad => self.op_mload()?,
            Operation::MStore => self.op_mstore()?,
            Operation::MStream => self.op_mstream(op_idx)?,
            Operation::Pipe => self.op_pipe()?,

            // ----- cryptographic operations -----------------------------------------------------
            Operation::HPerm => self.op_hperm(),
            Operation::MpVerify(_) => todo!(),
            Operation::MrUpdate => todo!(),
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

    /// TODO(plafer): add docs
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
                min(self.stack_top_idx - MIN_STACK_DEPTH, N - self.stack_top_idx);
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
    ///  buffer.
    fn restore_context(&mut self) -> Result<(), ExecutionError> {
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
