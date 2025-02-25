use alloc::{collections::BTreeMap, vec::Vec};
use core::cmp::min;

use miden_air::RowIndex;
use vm_core::{
    mast::{BasicBlockNode, MastForest, MastNode, MastNodeId},
    stack::MIN_STACK_DEPTH,
    utils::range,
    Felt, FieldElement, Operation, Program, StackOutputs, ONE, WORD_SIZE, ZERO,
};

use crate::{
    operations::utils::assert_binary,
    system::FMP_MAX,
    utils::{resolve_external_node, split_element},
    ContextId, ExecutionError, Host, FMP_MIN,
};

// temporary module to
pub mod experiments;

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

    /// A map from (context_id, word_address) to the word stored starting at that memory location.
    memory: BTreeMap<(ContextId, u32), [Felt; WORD_SIZE]>,
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
            memory: BTreeMap::new(),
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

        match node {
            MastNode::Block(basic_block_node) => {
                // start basic block
                self.clk += 1_u32;

                self.execute_basic_block_node(basic_block_node, program)?;

                // end basic block
                self.clk += 1_u32;

                Ok(())
            },
            MastNode::Join(join_node) => {
                // start join node
                self.clk += 1_u32;

                self.execute_mast_node(join_node.first(), program, host)?;
                self.execute_mast_node(join_node.second(), program, host)?;

                // end join node
                self.clk += 1_u32;

                Ok(())
            },
            MastNode::Split(split_node) => {
                let condition = self.stack[0];

                // TODO(plafer): test this - specifically if bounds check is updated such that the
                // next stack operation should fail

                // drop the condition from the stack
                self.decrement_stack_size();
                self.clk += 1_u32;
                self.update_bounds_check_counter();

                // execute the appropriate branch
                if condition == ONE {
                    self.execute_mast_node(split_node.on_true(), program, host)?;
                } else if condition == ZERO {
                    self.execute_mast_node(split_node.on_false(), program, host)?;
                } else {
                    return Err(ExecutionError::NotBinaryValue(condition));
                }

                // end join node
                self.clk += 1_u32;

                Ok(())
            },
            MastNode::Loop(_loop_node) => todo!(),
            MastNode::Call(_call_node) => todo!(),
            MastNode::Dyn(_dyn_node) => todo!(),
            MastNode::External(external_node) => {
                let (root_id, mast_forest) = resolve_external_node(external_node, host)?;

                self.execute_mast_node(root_id, &mast_forest, host)
            },
        }
    }

    // Note: when executing individual ops, we do not increment the clock by 1 at every iteration
    // for performance reasons (~25% performance drop). Hence, `self.clk` cannot be used directly to
    // determine the number of operations executed in a program.
    fn execute_basic_block_node(
        &mut self,
        basic_block_node: &BasicBlockNode,
        _program: &MastForest,
    ) -> Result<(), ExecutionError> {
        // TODO(plafer): this enumerate results in a ~5% performance drop (or about 25 MHz on the
        // fibonacci benchmark). We should find a better way to know the operation index when we
        // need to know the clock. For example, if the operations were stored in a Vec, we could
        // just use the index of the operation in the Vec. If still is still too slow, we could use
        // pointer arithmetic (comparing the address of the operation to the address of the first
        // operation in the Vec).
        for (op_idx, operation) in basic_block_node.operations().enumerate() {
            self.execute_op(operation, op_idx, _program)?;
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
            Operation::Assert(err_code) => {
                // TODO(plafer): delegate to the host when we have one
                if self.stack[self.stack_top_idx - 1] != ONE {
                    return Err(ExecutionError::FailedAssertion {
                        clk: self.clk + op_idx,
                        err_code: *err_code,
                        err_msg: None,
                    });
                }

                self.decrement_stack_size();
            },
            Operation::FmpAdd => {
                let top = &mut self.stack[self.stack_top_idx - 1];
                *top += self.fmp;
            },
            Operation::FmpUpdate => {
                let top = self.stack[self.stack_top_idx - 1];

                let new_fmp = self.fmp + top;
                if new_fmp.as_int() < FMP_MIN || new_fmp.as_int() > FMP_MAX {
                    return Err(ExecutionError::InvalidFmpValue(self.fmp, new_fmp));
                }

                self.fmp = new_fmp;
                self.decrement_stack_size();
            },
            Operation::SDepth => {
                let depth = (self.stack_top_idx - self.stack_bot_idx) as u32;
                self.stack[self.stack_top_idx] = depth.into();
                self.increment_stack_size();
            },
            Operation::Caller => todo!(),
            Operation::Clk => todo!(),
            Operation::Emit(_) => todo!(),

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
            Operation::Add => {
                self.pop2_applyfn_push(|a, b| Ok(a + b))?;
            },
            Operation::Neg => {
                self.stack[self.stack_top_idx - 1] = -self.stack[self.stack_top_idx - 1];
            },
            Operation::Mul => {
                self.pop2_applyfn_push(|a, b| Ok(a * b))?;
            },
            Operation::Inv => {
                let top = &mut self.stack[self.stack_top_idx - 1];
                if (*top) == ZERO {
                    return Err(ExecutionError::DivideByZero(self.clk + op_idx));
                }
                *top = top.inv();
            },
            Operation::Incr => {
                self.stack[self.stack_top_idx - 1] += ONE;
            },
            // TODO(plafer): test all cases (0,0), (0,1), (1,0), (1,1)
            Operation::And => {
                self.pop2_applyfn_push(|a, b| {
                    assert_binary(b)?;
                    assert_binary(a)?;

                    if a == ONE && b == ONE {
                        Ok(ONE)
                    } else {
                        Ok(ZERO)
                    }
                })?;
            },
            Operation::Or => {
                self.pop2_applyfn_push(|a, b| {
                    assert_binary(b)?;
                    assert_binary(a)?;

                    if a == ONE || b == ONE {
                        Ok(ONE)
                    } else {
                        Ok(ZERO)
                    }
                })?;
            },
            Operation::Not => {
                let top = &mut self.stack[self.stack_top_idx - 1];
                assert_binary(*top)?;
                *top = ONE - *top;
            },
            Operation::Eq => {
                self.pop2_applyfn_push(|a, b| if a == b { Ok(ONE) } else { Ok(ZERO) })?;
            },
            Operation::Eqz => {
                let top = &mut self.stack[self.stack_top_idx - 1];
                if (*top) == ZERO {
                    *top = ONE;
                } else {
                    *top = ZERO;
                }
            },
            Operation::Expacc => todo!(),

            // ----- ext2 operations --------------------------------------------------------------
            Operation::Ext2Mul => todo!(),

            // ----- u32 operations ---------------------------------------------------------------
            Operation::U32split => {
                let top = self.stack[self.stack_top_idx - 1];
                let (hi, lo) = split_element(top);

                self.stack[self.stack_top_idx - 1] = lo;
                self.stack[self.stack_top_idx] = hi;
                self.increment_stack_size();
            },
            // TODO(plafer): test the error cases where x>u32::max
            Operation::U32add => self.u32_pop2_applyfn_push_lowhigh(|a, b| a + b)?,
            Operation::U32add3 => self.op_u32_add3()?,
            Operation::U32sub => self.u32_pop2_applyfn_push_results(0, |a, b| {
                let result = a.wrapping_sub(b);
                let first = result >> 63;
                let second = result & u32::MAX as u64;

                Ok((first, second))
            })?,
            Operation::U32mul => self.u32_pop2_applyfn_push_lowhigh(|a, b| a * b)?,
            Operation::U32madd => todo!(),
            // TODO(plafer): Make sure we test the case where b == 0, and some nice divisions like
            // 6/3
            Operation::U32div => {
                let clk = self.clk + op_idx;
                self.u32_pop2_applyfn_push_results(0, |a, b| {
                    if b == 0 {
                        return Err(ExecutionError::DivideByZero(clk));
                    }

                    // a/b = n*q + r for some n>=0 and 0<=r<b
                    let q = a / b;
                    let r = a - q * b;

                    Ok((r, q))
                })?
            },
            Operation::U32and => self.u32_pop2_applyfn_push(|a, b| a & b)?,
            Operation::U32xor => self.u32_pop2_applyfn_push(|a, b| a ^ b)?,
            // TODO(plafer): probably switch order of `(first, second)` so that this can be
            // implemented as `Ok((a, b))`?
            Operation::U32assert2(err_code) => {
                self.u32_pop2_applyfn_push_results(*err_code, |a, b| Ok((b, a)))?
            },

            // ----- stack manipulation -----------------------------------------------------------
            Operation::Pad => {
                self.stack[self.stack_top_idx] = ZERO;
                self.increment_stack_size();
            },
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
            Operation::Swap => self.stack.swap(self.stack_top_idx - 1, self.stack_top_idx - 2),
            Operation::SwapW => self.swapw_nth(1),
            Operation::SwapW2 => self.swapw_nth(2),
            Operation::SwapW3 => self.swapw_nth(3),
            Operation::SwapDW => {
                self.stack.swap(self.stack_top_idx - 1, self.stack_top_idx - 9);
                self.stack.swap(self.stack_top_idx - 2, self.stack_top_idx - 10);
                self.stack.swap(self.stack_top_idx - 3, self.stack_top_idx - 11);
                self.stack.swap(self.stack_top_idx - 4, self.stack_top_idx - 12);
                self.stack.swap(self.stack_top_idx - 5, self.stack_top_idx - 13);
                self.stack.swap(self.stack_top_idx - 6, self.stack_top_idx - 14);
                self.stack.swap(self.stack_top_idx - 7, self.stack_top_idx - 15);
                self.stack.swap(self.stack_top_idx - 8, self.stack_top_idx - 16);
            },
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
            Operation::CSwap => todo!(),
            Operation::CSwapW => todo!(),

            // ----- input / output ---------------------------------------------------------------
            Operation::Push(element) => {
                self.stack[self.stack_top_idx] = *element;
                self.increment_stack_size();
            },
            Operation::AdvPop => todo!(),
            Operation::AdvPopW => todo!(),
            Operation::MLoadW => {
                let addr = {
                    let addr: u64 = self.stack[self.stack_top_idx - 1].as_int();
                    let addr: u32 = addr
                        .try_into()
                        .map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

                    if addr % WORD_SIZE as u32 != 0 {
                        return Err(ExecutionError::MemoryUnalignedWordAccess {
                            addr,
                            ctx: self.ctx,
                            clk: Felt::from(self.clk + op_idx),
                        });
                    }
                    addr
                };

                let word = self.memory.get(&(self.ctx, addr)).copied().unwrap_or([ZERO; WORD_SIZE]);

                self.stack[range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)]
                    .copy_from_slice(&word);

                self.decrement_stack_size();
            },
            Operation::MStoreW => {
                let addr = {
                    let addr: u64 = self.stack[self.stack_top_idx - 1].as_int();
                    let addr: u32 = addr
                        .try_into()
                        .map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

                    if addr % WORD_SIZE as u32 != 0 {
                        return Err(ExecutionError::MemoryUnalignedWordAccess {
                            addr,
                            ctx: self.ctx,
                            clk: Felt::from(self.clk + op_idx),
                        });
                    }
                    addr
                };

                let word: [Felt; WORD_SIZE] = self.stack
                    [range(self.stack_top_idx - 1 - WORD_SIZE, WORD_SIZE)]
                .try_into()
                .unwrap();

                self.memory.insert((self.ctx, addr), word);

                self.decrement_stack_size();
            },
            // TODO(plafer): test this
            Operation::MLoad => {
                let (word_addr, idx) = {
                    let addr = self.stack[self.stack_top_idx - 1].as_int();
                    let addr: u32 = addr
                        .try_into()
                        .map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

                    let idx = addr % WORD_SIZE as u32;

                    (addr - idx, idx)
                };
                let word =
                    self.memory.get(&(self.ctx, word_addr)).copied().unwrap_or([ZERO; WORD_SIZE]);

                self.stack[self.stack_top_idx - 1] = word[idx as usize];
            },
            // TODO(plafer): test this
            Operation::MStore => {
                let (word_addr, idx) = {
                    let addr = self.stack[self.stack_top_idx - 1].as_int();
                    let addr: u32 = addr
                        .try_into()
                        .map_err(|_| ExecutionError::MemoryAddressOutOfBounds(addr))?;

                    let idx = addr % WORD_SIZE as u32;

                    (addr - idx, idx)
                };

                let value = self.stack[self.stack_top_idx - 2];

                self.memory
                    .entry((self.ctx, word_addr))
                    .and_modify(|word| {
                        word[idx as usize] = value;
                    })
                    .or_insert_with(|| {
                        let mut word = [ZERO; WORD_SIZE];
                        word[idx as usize] = value;
                        word
                    });

                self.decrement_stack_size();
            },
            Operation::MStream => todo!(),
            Operation::Pipe => todo!(),

            // ----- cryptographic operations -----------------------------------------------------
            Operation::HPerm => todo!(),
            Operation::MpVerify(_) => todo!(),
            Operation::MrUpdate => todo!(),
            Operation::FriE2F4 => todo!(),
            Operation::HornerBase => todo!(),
            Operation::HornerExt => todo!(),
        }

        Ok(())
    }

    // HELPERS
    // ----------------------------------------------------------------------------------------------

    /// Pops the top two elements from the stack, applies the given function to them, and pushes the
    /// result back onto the stack.
    ///
    /// The size of the stack is decremented by 1.
    #[inline(always)]
    fn pop2_applyfn_push(
        &mut self,
        f: impl FnOnce(Felt, Felt) -> Result<Felt, ExecutionError>,
    ) -> Result<(), ExecutionError> {
        let b = self.stack[self.stack_top_idx - 1];
        let a = self.stack[self.stack_top_idx - 2];

        self.stack[self.stack_top_idx - 2] = f(a, b)?;
        self.decrement_stack_size();

        Ok(())
    }

    /// Equivalent to `pop2_applyfn_push`, but for u32 values.
    fn u32_pop2_applyfn_push(
        &mut self,
        f: impl FnOnce(u64, u64) -> u64,
    ) -> Result<(), ExecutionError> {
        let b = self.stack[self.stack_top_idx - 1].as_int();
        let a = self.stack[self.stack_top_idx - 2].as_int();

        // Check that a and b are u32 values.
        if a > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(a), ZERO));
        }
        if b > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(b), ZERO));
        }

        let result = f(a, b);
        self.stack[self.stack_top_idx - 2] = Felt::new(result);
        self.decrement_stack_size();

        Ok(())
    }

    /// Pops 2 elements from the stack, applies the given function to them, and pushes the low/high
    /// u32 values of the result back onto the stack.
    ///
    /// Specifically, this function
    /// 1. pops the top two elements from the stack,
    /// 2. applies the given function to them,
    /// 3. splits the result into low/high u32 values, and
    /// 4. pushes the low/high values back onto the stack.
    ///
    /// The size of the stack doesn't change.
    #[inline(always)]
    fn u32_pop2_applyfn_push_lowhigh(
        &mut self,
        f: impl FnOnce(u64, u64) -> u64,
    ) -> Result<(), ExecutionError> {
        let b = self.stack[self.stack_top_idx - 1].as_int();
        let a = self.stack[self.stack_top_idx - 2].as_int();

        // Check that a and b are u32 values.
        if a > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(a), ZERO));
        }
        if b > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(b), ZERO));
        }

        let result = Felt::new(f(a, b));
        let (hi, lo) = split_element(result);

        self.stack[self.stack_top_idx - 2] = lo;
        self.stack[self.stack_top_idx - 1] = hi;
        Ok(())
    }

    /// Pops 2 elements from the stack, applies the given function to them, and pushes the resulting
    /// 2 u32 values back onto the stack, in the order (first, second).
    ///
    /// The size of the stack doesn't change.
    #[inline(always)]
    fn u32_pop2_applyfn_push_results(
        &mut self,
        err_code: u32,
        f: impl FnOnce(u64, u64) -> Result<(u64, u64), ExecutionError>,
    ) -> Result<(), ExecutionError> {
        let b = self.stack[self.stack_top_idx - 1].as_int();
        let a = self.stack[self.stack_top_idx - 2].as_int();

        // Check that a and b are u32 values.
        if a > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(a), Felt::from(err_code)));
        }
        if b > u32::MAX as u64 {
            return Err(ExecutionError::NotU32Value(Felt::new(b), Felt::from(err_code)));
        }

        let (first, second) = f(a, b)?;

        self.stack[self.stack_top_idx - 2] = Felt::new(second);
        self.stack[self.stack_top_idx - 1] = Felt::new(first);
        Ok(())
    }

    /// Pops three elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    ///
    /// The size of the stack is decremented by 1.
    fn op_u32_add3(&mut self) -> Result<(), ExecutionError> {
        let (sum_hi, sum_lo) = {
            let c = self.stack[self.stack_top_idx - 1].as_int();
            let b = self.stack[self.stack_top_idx - 2].as_int();
            let a = self.stack[self.stack_top_idx - 3].as_int();

            // Check that a, b, and c are u32 values.
            if a > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(a), ZERO));
            }
            if b > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(b), ZERO));
            }
            if c > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(c), ZERO));
            }
            let result = Felt::new(a + b + c);
            split_element(result)
        };

        // write the high 32 bits to the new top of the stack, and low 32 bits after
        self.stack[self.stack_top_idx - 2] = sum_hi;
        self.stack[self.stack_top_idx - 3] = sum_lo;

        self.decrement_stack_size();
        Ok(())
    }

    /// Rotates the top `n` elements of the stack to the left by 1.
    ///
    /// For example, if the stack is [a, b, c, d], with `d` at the top, then `rotate_left(3)` will
    /// result in the top 3 elements being rotated left: [a, c, d, b].
    ///
    /// This operation is useful for implementing the `movup` instructions.
    ///
    /// The stack size doesn't change.
    #[inline(always)]
    fn rotate_left(&mut self, n: usize) {
        let rotation_bot_index = self.stack_top_idx - n;
        let new_stack_top_element = self.stack[rotation_bot_index];

        // shift the top n elements down by 1, starting from the bottom of the rotation.
        for i in 0..n - 1 {
            self.stack[rotation_bot_index + i] = self.stack[rotation_bot_index + i + 1];
        }

        // Set the top element (which comes from the bottom of the rotation).
        self.stack[self.stack_top_idx - 1] = new_stack_top_element;
    }

    /// Rotates the top `n` elements of the stack to the right by 1.
    ///
    /// Analogous to `rotate_left`, but in the opposite direction.
    #[inline(always)]
    fn rotate_right(&mut self, n: usize) {
        let rotation_bot_index = self.stack_top_idx - n;
        let new_stack_bot_element = self.stack[self.stack_top_idx - 1];

        // shift the top n elements up by 1, starting from the top of the rotation.
        for i in 1..n {
            self.stack[self.stack_top_idx - i] = self.stack[self.stack_top_idx - i - 1];
        }

        // Set the bot element (which comes from the top of the rotation).
        self.stack[rotation_bot_index] = new_stack_bot_element;
    }

    /// Duplicates the n'th element from the top of the stack to the top of the stack.
    ///
    /// The size of the stack is incremented by 1.
    #[inline(always)]
    fn dup_nth(&mut self, n: usize) {
        let to_dup_index = self.stack_top_idx - n - 1;
        self.stack[self.stack_top_idx] = self.stack[to_dup_index];
        self.increment_stack_size();
    }

    /// Swaps the nth word from the top of the stack with the top word of the stack.
    ///
    /// Valid values of `n` are 1, 2, and 3.
    fn swapw_nth(&mut self, n: usize) {
        // For example, for n=3, the stack words and variables look like:
        //    3     2     1     0
        // | ... | ... | ... | ... |
        // ^                 ^
        // nth_word       top_word
        let (rest_of_stack, top_word) = self.stack.split_at_mut(self.stack_top_idx - WORD_SIZE);
        let (_, nth_word) = rest_of_stack.split_at_mut(rest_of_stack.len() - n * WORD_SIZE);

        nth_word[0..WORD_SIZE].swap_with_slice(&mut top_word[0..WORD_SIZE]);
    }

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
}
