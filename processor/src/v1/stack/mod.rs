use super::ExecutionError;
use core::{cmp, convert::TryInto};
use std::panic;
use vm_core::v1::{program::Operation, BaseElement, FieldElement};

mod field_ops;
mod io_ops;
mod stack_ops;

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

    #[allow(dead_code)]
    pub fn get_trace_state(&self, step: usize) -> [BaseElement; STACK_TOP_SIZE] {
        assert!(
            step <= self.step,
            "cannot get trace state for a future step"
        );
        let mut result = [BaseElement::ZERO; STACK_TOP_SIZE];
        for (result, column) in result.iter_mut().zip(self.trace.iter()) {
            *result = column[step];
        }
        result
    }

    // OPERATION EXECUTOR
    // --------------------------------------------------------------------------------------------

    /// TODO: add comments
    pub fn execute(&mut self, op: Operation) -> Result<(), ExecutionError> {
        // increment current step by one and make sure there is enough memory allocated to hold
        // the execution trace
        self.advance_step();

        match op {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => self.copy_state(0),
            Operation::Assert => unimplemented!(),

            // ----- flow control operations ------------------------------------------------------
            Operation::Join => unreachable!(),
            Operation::Split => unreachable!(),
            Operation::Loop => unreachable!(),
            Operation::Repeat => unreachable!(),
            Operation::Span => unreachable!(),
            Operation::Respan => unreachable!(),
            Operation::End => unreachable!(),

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
            Operation::U32split => unimplemented!(),
            Operation::U32add => unimplemented!(),
            Operation::U32addc => unimplemented!(),
            Operation::U32sub => unimplemented!(),
            Operation::U32mul => unimplemented!(),
            Operation::U32madd => unimplemented!(),
            Operation::U32div => unimplemented!(),

            Operation::U32and => unimplemented!(),
            Operation::U32or => unimplemented!(),
            Operation::U32xor => unimplemented!(),

            // ----- stack manipulation -----------------------------------------------------------
            Operation::Pad => self.op_pad()?,
            Operation::Drop => self.op_drop()?,

            Operation::Dup0 => unimplemented!(),
            Operation::Dup1 => unimplemented!(),
            Operation::Dup2 => unimplemented!(),
            Operation::Dup3 => unimplemented!(),
            Operation::Dup4 => unimplemented!(),
            Operation::Dup5 => unimplemented!(),
            Operation::Dup6 => unimplemented!(),
            Operation::Dup7 => unimplemented!(),
            Operation::Dup8 => unimplemented!(),
            Operation::Dup9 => unimplemented!(),
            Operation::Dup10 => unimplemented!(),
            Operation::Dup11 => unimplemented!(),
            Operation::Dup12 => unimplemented!(),
            Operation::Dup13 => unimplemented!(),
            Operation::Dup14 => unimplemented!(),
            Operation::Dup15 => unimplemented!(),

            Operation::Swap => unimplemented!(),
            Operation::SwapW => unimplemented!(),
            Operation::SwapW2 => unimplemented!(),
            Operation::SwapW3 => unimplemented!(),

            Operation::MovUp2 => unimplemented!(),
            Operation::MovUp3 => unimplemented!(),
            Operation::MovUp4 => unimplemented!(),
            Operation::MovUp8 => unimplemented!(),
            Operation::MovUp12 => unimplemented!(),

            Operation::MovDn2 => unimplemented!(),
            Operation::MovDn3 => unimplemented!(),
            Operation::MovDn4 => unimplemented!(),
            Operation::MovDn8 => unimplemented!(),
            Operation::MovDn12 => unimplemented!(),

            Operation::CSwap => unimplemented!(),
            Operation::CSwapW => unimplemented!(),

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

        Ok(())
    }

    // HELPER METHODS
    // --------------------------------------------------------------------------------------------

    /// Copies stack values starting with the specified position to the next step.
    fn copy_state(&mut self, start_pos: usize) {
        debug_assert!(
            start_pos < STACK_TOP_SIZE,
            "start cannot exceed stack top size"
        );
        let end_pos = cmp::min(self.depth, STACK_TOP_SIZE);
        for i in start_pos..end_pos {
            self.trace[i][self.step] = self.trace[i][self.step - 1];
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
                    self.trace[i - 1][self.step] = self.trace[i][self.step - 1];
                }
            }
            _ => {
                for i in start_pos..STACK_TOP_SIZE {
                    self.trace[i - 1][self.step] = self.trace[i][self.step - 1];
                }
                let from_overflow = self.overflow.pop().expect("overflow stack is empty");
                self.trace[STACK_TOP_SIZE - 1][self.step] = from_overflow;
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
                    self.trace[i + 1][self.step] = self.trace[i][self.step - 1];
                }
            }
            _ => {
                for i in start_pos..MAX_TOP_IDX {
                    self.trace[i + 1][self.step] = self.trace[i][self.step - 1];
                }
                let to_overflow = self.trace[MAX_TOP_IDX][self.step - 1];
                self.overflow.push(to_overflow)
            }
        }

        self.depth += 1;
    }

    /// Increments current step by one and makes sure there is enough memory allocated for the
    /// trace to accommodate the new row.
    ///
    /// Trace length is doubled every time it needs to be increased.
    fn advance_step(&mut self) {
        // increment step by 1
        self.step += 1;

        // make sure there is enough memory allocated for register traces
        if self.step >= self.trace_length() {
            let new_length = self.trace_length() * 2;
            for register in self.trace.iter_mut() {
                register.resize(new_length, BaseElement::ZERO);
            }
        }
    }
}
