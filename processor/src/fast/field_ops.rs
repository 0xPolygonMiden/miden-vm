use vm_core::{Felt, FieldElement, ONE, ZERO};

use super::{assert_binary, ExecutionError, FastProcessor};

const TWO: Felt = Felt::new(2);

impl FastProcessor {
    pub fn op_add(&mut self) -> Result<(), ExecutionError> {
        self.pop2_applyfn_push(|a, b| Ok(a + b))
    }

    pub fn op_neg(&mut self) -> Result<(), ExecutionError> {
        self.stack[self.stack_top_idx - 1] = -self.stack[self.stack_top_idx - 1];
        Ok(())
    }

    pub fn op_mul(&mut self) -> Result<(), ExecutionError> {
        self.pop2_applyfn_push(|a, b| Ok(a * b))
    }

    pub fn op_inv(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        let top = &mut self.stack[self.stack_top_idx - 1];
        if (*top) == ZERO {
            return Err(ExecutionError::DivideByZero(self.clk + op_idx));
        }
        *top = top.inv();
        Ok(())
    }

    pub fn op_incr(&mut self) -> Result<(), ExecutionError> {
        self.stack[self.stack_top_idx - 1] += ONE;
        Ok(())
    }

    pub fn op_and(&mut self) -> Result<(), ExecutionError> {
        self.pop2_applyfn_push(|a, b| {
            assert_binary(b)?;
            assert_binary(a)?;

            if a == ONE && b == ONE {
                Ok(ONE)
            } else {
                Ok(ZERO)
            }
        })
    }

    pub fn op_or(&mut self) -> Result<(), ExecutionError> {
        self.pop2_applyfn_push(|a, b| {
            assert_binary(b)?;
            assert_binary(a)?;

            if a == ONE || b == ONE {
                Ok(ONE)
            } else {
                Ok(ZERO)
            }
        })
    }

    pub fn op_not(&mut self) -> Result<(), ExecutionError> {
        let top = &mut self.stack[self.stack_top_idx - 1];
        assert_binary(*top)?;
        *top = ONE - *top;
        Ok(())
    }

    pub fn op_eq(&mut self) -> Result<(), ExecutionError> {
        self.pop2_applyfn_push(|a, b| if a == b { Ok(ONE) } else { Ok(ZERO) })
    }

    pub fn op_eqz(&mut self) -> Result<(), ExecutionError> {
        let top = &mut self.stack[self.stack_top_idx - 1];
        if (*top) == ZERO {
            *top = ONE;
        } else {
            *top = ZERO;
        }
        Ok(())
    }

    pub fn op_expacc(&mut self) {
        let old_base = self.stack[self.stack_top_idx - 2];
        let old_acc = self.stack[self.stack_top_idx - 3];
        let old_exp_int = self.stack[self.stack_top_idx - 4].as_int();

        // Compute new exponent.
        let new_exp = Felt::new(old_exp_int >> 1);

        // Compute new accumulator. We update the accumulator only when the least significant bit of
        // the exponent is 1.
        let exp_lsb = old_exp_int & 1;
        let acc_update_val = if exp_lsb == 1 { old_base } else { ONE };
        let new_acc = old_acc * acc_update_val;

        // Compute the new base.
        let new_base = old_base * old_base;

        self.stack[self.stack_top_idx - 1] = Felt::new(exp_lsb);
        self.stack[self.stack_top_idx - 2] = new_base;
        self.stack[self.stack_top_idx - 3] = new_acc;
        self.stack[self.stack_top_idx - 4] = new_exp;
    }

    /// Gets the top four values from the stack [b1, b0, a1, a0], where a = (a1, a0) and
    /// b = (b1, b0) are elements of the extension field, and outputs the product c = (c1, c0)
    /// where c0 = b0 * a0 - 2 * b1 * a1 and c1 = (b0 + b1) * (a1 + a0) - b0 * a0. It pushes 0 to
    /// the first and second positions on the stack, c1 and c2 to the third and fourth positions,
    /// and leaves the rest of the stack unchanged.
    pub fn op_ext2mul(&mut self) {
        let b1 = self.stack[self.stack_top_idx - 1];
        let b0 = self.stack[self.stack_top_idx - 2];
        let a1 = self.stack[self.stack_top_idx - 3];
        let a0 = self.stack[self.stack_top_idx - 4];

        /* top 2 elements remain unchanged */

        let b0_times_a0 = b0 * a0;
        self.stack[self.stack_top_idx - 3] = (b0 + b1) * (a1 + a0) - b0_times_a0;
        self.stack[self.stack_top_idx - 4] = b0_times_a0 - TWO * b1 * a1;
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
}
