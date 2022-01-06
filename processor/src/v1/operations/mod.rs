use super::{BaseElement, ExecutionError, FieldElement, Operation, Process, StarkField};

mod field_ops;
mod io_ops;
mod stack_ops;
mod sys_ops;
mod u32_ops;
mod utils;

// OPERATION DISPATCHER
// ================================================================================================

impl Process {
    /// Executes the specified operation.
    pub(super) fn execute_op(&mut self, op: Operation) -> Result<(), ExecutionError> {
        // make sure there is enough memory allocated to hold the execution trace
        self.ensure_trace_capacity();

        // execute the operation
        match op {
            // ----- system operations ------------------------------------------------------------
            Operation::Noop => self.stack.copy_state(0),
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
            Operation::Eqw => self.op_eqw()?,

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
            Operation::Dup9 => self.op_dup(9)?,
            Operation::Dup11 => self.op_dup(11)?,
            Operation::Dup13 => self.op_dup(13)?,
            Operation::Dup15 => self.op_dup(15)?,

            Operation::Swap => self.op_swap()?,
            Operation::SwapW => self.op_swapw()?,
            Operation::SwapW2 => self.op_swapw2()?,
            Operation::SwapW3 => self.op_swapw3()?,

            Operation::MovUp2 => self.op_movup2()?,
            Operation::MovUp3 => self.op_movup3()?,
            Operation::MovUp4 => self.op_movup4()?,
            Operation::MovUp5 => self.op_movup5()?,
            Operation::MovUp6 => self.op_movup6()?,
            Operation::MovUp7 => self.op_movup7()?,
            Operation::MovUp9 => self.op_movup9()?,
            Operation::MovUp11 => self.op_movup11()?,
            Operation::MovUp13 => self.op_movup13()?,
            Operation::MovUp15 => self.op_movup15()?,

            Operation::MovDn2 => self.op_movdn2()?,
            Operation::MovDn3 => self.op_movdn3()?,
            Operation::MovDn4 => self.op_movdn4()?,
            Operation::MovDn5 => self.op_movdn5()?,
            Operation::MovDn6 => self.op_movdn6()?,
            Operation::MovDn7 => self.op_movdn7()?,
            Operation::MovDn9 => self.op_movdn9()?,
            Operation::MovDn11 => self.op_movdn11()?,
            Operation::MovDn13 => self.op_movdn13()?,
            Operation::MovDn15 => self.op_movdn15()?,

            Operation::CSwap => self.op_cswap()?,
            Operation::CSwapW => self.op_cswapw()?,

            // ----- input / output ---------------------------------------------------------------
            Operation::Push(value) => self.op_push(value)?,

            Operation::Read => unimplemented!(),
            Operation::ReadW => unimplemented!(),

            Operation::LoadW => self.op_loadw()?,
            Operation::StoreW => self.op_storew()?,

            Operation::SDepth => unimplemented!(),

            // ----- cryptographic operations -----------------------------------------------------
            Operation::RpHash => unimplemented!(),
            Operation::RpPerm => unimplemented!(),
        }

        // increment the clock cycle
        self.advance_clock();

        Ok(())
    }

    /// Increments the clock cycle for all components of the processor.
    fn advance_clock(&mut self) {
        self.step += 1;
        self.stack.advance_clock();
        self.memory.advance_clock();
    }

    /// Makes sure there is enough memory allocated for the trace to accommodate a new clock cycle.
    fn ensure_trace_capacity(&mut self) {
        self.stack.ensure_trace_capacity();
    }

    // TEST METHODS
    // --------------------------------------------------------------------------------------------

    /// Instantiates a new processor for testing purposes.
    #[cfg(test)]
    fn new_dummy() -> Self {
        Self::new(super::ProgramInputs::none())
    }
}

// TEST HELPERS
// ================================================================================================

/// Pushes proved values onto the stack of the specified process. The values are pushed in the
/// order in which they are provided.
#[cfg(test)]
fn init_stack_with(process: &mut Process, values: &[u64]) {
    let mut result = Vec::with_capacity(values.len());
    for value in values.iter().map(|&v| BaseElement::new(v)) {
        process.execute_op(Operation::Push(value)).unwrap();
        result.push(value);
    }
}
