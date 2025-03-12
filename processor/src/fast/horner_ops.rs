use vm_core::Felt;

use super::FastProcessor;
use crate::{ExecutionError, QuadFelt};

// CONSTANTS
// ================================================================================================

const ALPHA_ADDR_INDEX: usize = 13;
const ACC_HIGH_INDEX: usize = 14;
const ACC_LOW_INDEX: usize = 15;

impl FastProcessor {
    /// Mirrors the implementation of `Process::op_horner_eval_base`.
    pub fn op_horner_eval_base(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        // read the values of the coefficients, over the base field, from the stack
        let coef = self.get_coeff_as_base_elements();

        // read the evaluation point alpha from memory
        let alpha = self.get_evaluation_point(op_idx)?;

        // compute the updated accumulator value
        let (acc_new1, acc_new0) = {
            let acc_old = self.get_accumulator();
            let acc_new =
                coef.iter().rev().fold(acc_old, |acc, coef| QuadFelt::from(*coef) + alpha * acc);

            let acc_new_base_elements = acc_new.to_base_elements();

            (acc_new_base_elements[1], acc_new_base_elements[0])
        };

        // Update the accumulator values
        self.stack[self.stack_top_idx - 1 - ACC_HIGH_INDEX] = acc_new1;
        self.stack[self.stack_top_idx - 1 - ACC_LOW_INDEX] = acc_new0;

        Ok(())
    }

    /// Mirrors the implementation of `Process::op_horner_eval_ext`.
    pub fn op_horner_eval_ext(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        // read the values of the coefficients, over the base field, from the stack
        let coef = self.get_coeff_as_quad_ext_elements();

        // read the evaluation point alpha from memory
        let alpha = self.get_evaluation_point(op_idx)?;

        // compute the updated accumulator value
        let (acc_new1, acc_new0) = {
            let acc_old = self.get_accumulator();
            let acc_new = coef.iter().rev().fold(acc_old, |acc, coef| *coef + alpha * acc);

            let acc_new_base_elements = acc_new.to_base_elements();

            (acc_new_base_elements[1], acc_new_base_elements[0])
        };

        // Update the accumulator values
        self.stack[self.stack_top_idx - 1 - ACC_HIGH_INDEX] = acc_new1;
        self.stack[self.stack_top_idx - 1 - ACC_LOW_INDEX] = acc_new0;

        Ok(())
    }

    // HELPER METHODS
    //// ------------------------------------------------------------------------------------------

    /// Returns the top 8 elements of the operand stack, such that the element on top of the stack
    /// appears first in the return array.
    fn get_coeff_as_base_elements(&self) -> [Felt; 8] {
        let c0 = self.stack[self.stack_top_idx - 1];
        let c1 = self.stack[self.stack_top_idx - 2];
        let c2 = self.stack[self.stack_top_idx - 3];
        let c3 = self.stack[self.stack_top_idx - 4];
        let c4 = self.stack[self.stack_top_idx - 5];
        let c5 = self.stack[self.stack_top_idx - 6];
        let c6 = self.stack[self.stack_top_idx - 7];
        let c7 = self.stack[self.stack_top_idx - 8];

        [c0, c1, c2, c3, c4, c5, c6, c7]
    }

    /// Returns the top 8 elements of the operand stack.
    fn get_coeff_as_quad_ext_elements(&self) -> [QuadFelt; 4] {
        let c0_1 = self.stack[self.stack_top_idx - 1];
        let c0_0 = self.stack[self.stack_top_idx - 2];
        let c1_1 = self.stack[self.stack_top_idx - 3];
        let c1_0 = self.stack[self.stack_top_idx - 4];
        let c2_1 = self.stack[self.stack_top_idx - 5];
        let c2_0 = self.stack[self.stack_top_idx - 6];
        let c3_1 = self.stack[self.stack_top_idx - 7];
        let c3_0 = self.stack[self.stack_top_idx - 8];

        [
            QuadFelt::new(c0_0, c0_1),
            QuadFelt::new(c1_0, c1_1),
            QuadFelt::new(c2_0, c2_1),
            QuadFelt::new(c3_0, c3_1),
        ]
    }

    /// Returns the evaluation point.
    fn get_evaluation_point(&mut self, op_idx: usize) -> Result<QuadFelt, ExecutionError> {
        let (alpha_0, alpha_1) = {
            let addr = self.stack[self.stack_top_idx - 1 - ALPHA_ADDR_INDEX];
            let word = self.memory.read_word(self.ctx, addr, self.clk + op_idx)?;

            (word[0], word[1])
        };

        Ok(QuadFelt::new(alpha_0, alpha_1))
    }

    /// Reads the accumulator values.
    fn get_accumulator(&self) -> QuadFelt {
        let acc1 = self.stack[self.stack_top_idx - 1 - ACC_HIGH_INDEX];
        let acc0 = self.stack[self.stack_top_idx - 1 - ACC_LOW_INDEX];

        QuadFelt::new(acc0, acc1)
    }
}
