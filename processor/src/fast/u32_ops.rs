use vm_core::{Felt, ZERO};

use super::FastProcessor;
use crate::{utils::split_element, ExecutionError};

impl FastProcessor {
    pub fn u32_split(&mut self) -> Result<(), ExecutionError> {
        let top = self.stack[self.stack_top_idx - 1];
        let (hi, lo) = split_element(top);

        self.stack[self.stack_top_idx - 1] = lo;
        self.stack[self.stack_top_idx] = hi;
        self.increment_stack_size();
        Ok(())
    }

    // TODO(plafer): test the error cases where x>u32::max
    pub fn u32_add(&mut self) -> Result<(), ExecutionError> {
        self.u32_pop2_applyfn_push_lowhigh(|a, b| a + b)
    }

    /// Pops three elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    ///
    /// The size of the stack is decremented by 1.
    pub fn u32_add3(&mut self) -> Result<(), ExecutionError> {
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

    pub fn u32_sub(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        self.u32_pop2_applyfn_push_results(op_idx as u32, |a, b| {
            let result = a.wrapping_sub(b);
            let first = result >> 63;
            let second = result & u32::MAX as u64;

            Ok((first, second))
        })
    }

    pub fn u32_mul(&mut self) -> Result<(), ExecutionError> {
        self.u32_pop2_applyfn_push_lowhigh(|a, b| a * b)
    }

    /// Pops three elements off the stack, multiplies the first two and adds the third element to
    /// the result, splits the result into low and high 32-bit values, and pushes these values
    /// back onto the stack.
    pub fn u32_madd(&mut self) -> Result<(), ExecutionError> {
        let (result_hi, result_lo) = {
            let b = self.stack[self.stack_top_idx - 1].as_int();
            let a = self.stack[self.stack_top_idx - 2].as_int();
            let c = self.stack[self.stack_top_idx - 3].as_int();

            // Check that a, b, and c are u32 values.
            if b > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(a), ZERO));
            }
            if a > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(b), ZERO));
            }
            if c > u32::MAX as u64 {
                return Err(ExecutionError::NotU32Value(Felt::new(c), ZERO));
            }
            let result = Felt::new(a * b + c);
            split_element(result)
        };

        // write the high 32 bits to the new top of the stack, and low 32 bits after
        self.stack[self.stack_top_idx - 2] = result_hi;
        self.stack[self.stack_top_idx - 3] = result_lo;

        self.decrement_stack_size();
        Ok(())
    }

    // TODO(plafer): Make sure we test the case where b == 0, and some nice divisions like
    // 6/3
    pub fn u32_div(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        let clk = self.clk + op_idx;
        self.u32_pop2_applyfn_push_results(0, |a, b| {
            if b == 0 {
                return Err(ExecutionError::DivideByZero(clk));
            }

            // a/b = n*q + r for some n>=0 and 0<=r<b
            let q = a / b;
            let r = a - q * b;

            Ok((r, q))
        })
    }

    pub fn u32_and(&mut self) -> Result<(), ExecutionError> {
        self.u32_pop2_applyfn_push(|a, b| a & b)
    }

    pub fn u32_xor(&mut self) -> Result<(), ExecutionError> {
        self.u32_pop2_applyfn_push(|a, b| a ^ b)
    }

    pub fn u32_assert2(&mut self, err_code: u32) -> Result<(), ExecutionError> {
        // TODO(plafer): probably switch order of `(first, second)` so that this can be
        // implemented as `Ok((a, b))`?
        self.u32_pop2_applyfn_push_results(err_code, |a, b| Ok((b, a)))
    }

    // HELPERS
    // ----------------------------------------------------------------------------------------------

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
}
