use super::ExecutionError;
use core::{cmp, convert::TryInto};
use std::panic;
use vm_core::v1::{program::Operation, BaseElement, FieldElement};

mod field_ops;
mod io_ops;
mod stack_ops;
mod u32_ops;

// CONSTANT
// ================================================================================================

/// Specifies the number of stack registers which can be accesses by the VM directly.
const STACK_TOP_SIZE: usize = 16;

// TYPES ALIASES
// ================================================================================================

type StackTrace = [Vec<BaseElement>; STACK_TOP_SIZE];

// STACK
// ================================================================================================

pub struct Stack {
    step: usize,
    trace: StackTrace,
    overflow: Vec<BaseElement>,
    depth: usize,
}

impl Stack {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// TODO: add comments
    pub fn new(init_trace_length: usize) -> Self {
        Self {
            step: 0,
            trace: (0..STACK_TOP_SIZE)
                .map(|_| vec![BaseElement::ZERO; init_trace_length])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            overflow: Vec::new(),
            depth: 0,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns depth of the stack at the current step.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the current step of the execution trace.
    #[allow(dead_code)]
    pub fn current_step(&self) -> usize {
        self.step
    }

    /// Returns execution trace length for this stack.
    pub fn trace_length(&self) -> usize {
        self.trace[0].len()
    }

    /// Returns a copy of the item currently at the top of the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub fn peek(&self) -> Result<BaseElement, ExecutionError> {
        if self.depth == 0 {
            return Err(ExecutionError::StackUnderflow("peek", self.step));
        }

        Ok(self.trace[0][self.step])
    }

    /// Returns trace state at the current step.
    ///
    /// Trace state is always 16 elements long and contains the top 16 values of the stack. When
    /// the stack depth is less than 16, the un-used slots contain ZEROs.
    #[allow(dead_code)]
    pub fn trace_state(&self) -> [BaseElement; STACK_TOP_SIZE] {
        let mut result = [BaseElement::ZERO; STACK_TOP_SIZE];
        for (result, column) in result.iter_mut().zip(self.trace.iter()) {
            *result = column[self.step];
        }
        result
    }

    // OPERATION EXECUTOR
    // --------------------------------------------------------------------------------------------

    /// TODO: add comments
    pub fn execute(&mut self, op: Operation) -> Result<(), ExecutionError> {
        // make sure there is enough memory allocated to hold the execution trace
        self.ensure_trace_capacity();

        // execute the operation
        match op {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => self.copy_state(0),
            Operation::Assert => self.op_assert()?,

            // ----- flow control operations ------------------------------------------------------
            // control flow operations are never executed directly
            Operation::Join => unreachable!("control flow operation"),
            Operation::Split => unreachable!("control flow operation"),
            Operation::Loop => unreachable!("control flow operation"),
            Operation::Repeat => unreachable!("control flow operation"),
            Operation::Span => unreachable!("control flow operation"),
            Operation::Respan => unreachable!("control flow operation"),
            Operation::End => unreachable!("control flow operation"),

            // ----- field operations -------------------------------------------------------------
            Operation::Add => self.op_add()?,
            Operation::Neg => self.op_neg()?,
            Operation::Mul => self.op_mul()?,
            Operation::Inv => self.op_inv()?,
            Operation::Incr => self.op_incr()?,

            Operation::And => self.op_and()?,
            Operation::Or => self.op_or()?,
            Operation::Not => self.op_not()?,

            Operation::Eq => self.op_eq()?,
            Operation::Eqz => self.op_eqz()?,

            // ----- u32 operations ---------------------------------------------------------------
            Operation::U32split => self.op_u32split()?,
            Operation::U32add => self.op_u32add()?,
            Operation::U32addc => self.op_u32addc()?,
            Operation::U32sub => self.op_u32sub()?,
            Operation::U32mul => self.op_u32mul()?,
            Operation::U32madd => self.op_u32madd()?,
            Operation::U32div => self.op_u32div()?,

            Operation::U32and => self.op_u32and()?,
            Operation::U32or => self.op_u32or()?,
            Operation::U32xor => self.op_u32xor()?,

            // ----- stack manipulation -----------------------------------------------------------
            Operation::Pad => self.op_pad()?,
            Operation::Drop => self.op_drop()?,

            Operation::Dup0 => self.op_dup(0)?,
            Operation::Dup1 => self.op_dup(1)?,
            Operation::Dup2 => self.op_dup(2)?,
            Operation::Dup3 => self.op_dup(3)?,
            Operation::Dup4 => self.op_dup(4)?,
            Operation::Dup5 => self.op_dup(5)?,
            Operation::Dup6 => self.op_dup(6)?,
            Operation::Dup7 => self.op_dup(7)?,
            Operation::Dup8 => self.op_dup(8)?,
            Operation::Dup9 => self.op_dup(9)?,
            Operation::Dup10 => self.op_dup(10)?,
            Operation::Dup11 => self.op_dup(11)?,
            Operation::Dup12 => self.op_dup(12)?,
            Operation::Dup13 => self.op_dup(13)?,
            Operation::Dup14 => self.op_dup(14)?,
            Operation::Dup15 => self.op_dup(15)?,

            Operation::Swap => self.op_swap()?,
            Operation::SwapW => self.op_swapw()?,
            Operation::SwapW2 => self.op_swapw2()?,
            Operation::SwapW3 => self.op_swapw3()?,

            Operation::MovUp2 => self.op_movup2()?,
            Operation::MovUp3 => self.op_movup3()?,
            Operation::MovUp4 => self.op_movup4()?,
            Operation::MovUp8 => self.op_movup8()?,
            Operation::MovUp12 => self.op_movup12()?,

            Operation::MovDn2 => self.op_movdn2()?,
            Operation::MovDn3 => self.op_movdn3()?,
            Operation::MovDn4 => self.op_movdn4()?,
            Operation::MovDn8 => self.op_movdn8()?,
            Operation::MovDn12 => self.op_movdn12()?,

            Operation::CSwap => self.op_cswap()?,
            Operation::CSwapW => self.op_cswapw()?,

            // ----- input / output ---------------------------------------------------------------
            Operation::Push(value) => self.op_push(value)?,

            Operation::Read => unimplemented!(),
            Operation::ReadW => unimplemented!(),

            Operation::LoadW => unimplemented!(),
            Operation::StoreW => unimplemented!(),

            // ----- cryptographic operations -----------------------------------------------------
            Operation::RpHash => unimplemented!(),
            Operation::RpPerm => unimplemented!(),
        }

        // increment step by 1
        self.step += 1;

        Ok(())
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// TODO: add comments
    fn op_assert(&mut self) -> Result<(), ExecutionError> {
        if self.depth == 0 {
            return Err(ExecutionError::StackUnderflow("ASSERT", self.step));
        }
        if self.trace[0][self.step] != BaseElement::ONE {
            return Err(ExecutionError::FailedAssertion(self.step));
        }
        self.shift_left(1);
        Ok(())
    }

    /// Copies stack values starting with the specified position to the next step.
    fn copy_state(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start cannot exceed stack top size"
        );
        let end_pos = cmp::min(self.depth, STACK_TOP_SIZE);
        for i in start_pos..end_pos {
            self.trace[i][self.step + 1] = self.trace[i][self.step];
        }
    }

    /// Shifts the stack by one item to the left starting from the specified position.
    ///
    /// If the stack depth is greater than 16, items are moved from the overflow stack to the
    /// stack top.
    ///
    /// # Panics
    /// Panics if the stack is empty.
    fn shift_left(&mut self, start_pos: usize) {
        debug_assert!(start_pos > 0, "start position must be greater than 0");
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start position cannot exceed stack top size"
        );
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        match self.depth {
            0 => unreachable!("stack underflow"),
            1..=16 => {
                for i in start_pos..self.depth {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
            }
            _ => {
                for i in start_pos..STACK_TOP_SIZE {
                    self.trace[i - 1][self.step + 1] = self.trace[i][self.step];
                }
                let from_overflow = self.overflow.pop().expect("overflow stack is empty");
                self.trace[STACK_TOP_SIZE - 1][self.step + 1] = from_overflow;
            }
        }

        self.depth -= 1;
    }

    /// Shifts the stack one item to the right starting from the specified position.
    ///
    /// If stack depth grows beyond 16 items, additional items are pushed into the overflow stack.
    fn shift_right(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start position cannot exceed stack top size"
        );
        debug_assert!(
            start_pos <= self.depth,
            "start position cannot exceed current depth"
        );

        const MAX_TOP_IDX: usize = STACK_TOP_SIZE - 1;
        match self.depth {
            0 => {} // if the stack is empty, do nothing
            1..=MAX_TOP_IDX => {
                for i in start_pos..self.depth {
                    self.trace[i + 1][self.step + 1] = self.trace[i][self.step];
                }
            }
            _ => {
                for i in start_pos..MAX_TOP_IDX {
                    self.trace[i + 1][self.step + 1] = self.trace[i][self.step];
                }
                let to_overflow = self.trace[MAX_TOP_IDX][self.step];
                self.overflow.push(to_overflow)
            }
        }

        self.depth += 1;
    }

    /// Makes sure there is enough memory allocated for the trace to accommodate a new row.
    ///
    /// Trace length is doubled every time it needs to be increased.
    fn ensure_trace_capacity(&mut self) {
        if self.step + 1 >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            for register in self.trace.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
        }
    }
}
