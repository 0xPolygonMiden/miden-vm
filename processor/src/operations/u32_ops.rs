use super::{utils::assert_binary, ExecutionError, Felt, FieldElement, Process, StarkField};

impl Process {
    // CASTING OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops the top element off the stack, splits it into low and high 32-bit values, and pushes
    /// these values back onto the stack.
    pub(super) fn op_u32split(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        let (lo, hi) = split_element(a);

        self.add_range_checks(lo, Some(hi));

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.shift_right(1);
        Ok(())
    }

    /// Pops top two element off the stack, splits both into low and high 32-bit values, checks if both
    /// high are equal to 0, if it passes, put both of them onto the stack, else throws an execution error
    pub(super) fn op_u32assert2(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack.get(0);
        let b = self.stack.get(1);

        let (lo_a, hi_a) = split_element(a);
        let (lo_b, hi_b) = split_element(b);

        if hi_a != Felt::ZERO {
            return Err(ExecutionError::NotU32Value(a));
        }

        if hi_b != Felt::ZERO {
            return Err(ExecutionError::NotU32Value(b));
        }

        self.add_range_checks(lo_a, Some(hi_a));
        self.add_range_checks(lo_b, Some(hi_b));

        self.stack.copy_state(0);
        Ok(())
    }

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    pub(super) fn op_u32add(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        let result = a + b;
        let (lo, hi) = split_element(result);

        self.add_range_checks(lo, None);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops three elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the third element from the top fo the stack is not a binary value.
    pub(super) fn op_u32addc(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let c = assert_binary(self.stack.get(2))?.as_int();
        let result = Felt::new(a + b + c);
        let (lo, hi) = split_element(result);

        self.add_range_checks(lo, None);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.shift_left(3);
        Ok(())
    }

    /// Pops two elements off the stack, subtracts the top element from the second element, and
    /// pushes the result as well as a flag indicating whether there was underflow back onto the
    /// stack.
    pub(super) fn op_u32sub(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = a.wrapping_sub(b);
        let d = Felt::new(result >> 63);
        let c = Felt::new((result as u32) as u64);

        self.add_range_checks(c, None);

        self.stack.set(0, d);
        self.stack.set(1, c);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops two elements off the stack, multiplies them, splits the result into low and high
    /// 32-bit values, and pushes these values back onto the stack.
    pub(super) fn op_u32mul(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = Felt::new(a * b);
        let (lo, hi) = split_element(result);

        self.add_range_checks(lo, Some(hi));

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops three elements off the stack, multiplies the first two and adds the third element to
    /// the result, splits the result into low and high 32-bit values, and pushes these values
    /// back onto the stack.
    pub(super) fn op_u32madd(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let c = self.stack.get(2).as_int();
        let result = Felt::new(a * b + c);
        let (lo, hi) = split_element(result);

        self.add_range_checks(lo, Some(hi));

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.shift_left(3);
        Ok(())
    }

    /// Pops two elements off the stack, divides the second element by the top element, and pushes
    /// the quotient and the remainder back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the divisor is ZERO.
    pub(super) fn op_u32div(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();

        if b == 0 {
            return Err(ExecutionError::DivideByZero(self.system.clk()));
        }

        let q = a / b;
        let r = a - q * b;

        // These range checks help enforce that q <= a.
        let lo = Felt::new(a - q);
        // These range checks help enforce that r < b.
        let hi = Felt::new(b - r - 1);
        self.add_range_checks(lo, Some(hi));

        self.stack.set(0, Felt::new(r));
        self.stack.set(1, Felt::new(q));
        self.stack.copy_state(2);
        Ok(())
    }

    // BITWISE OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two elements off the stack, computes their bitwise AND, and pushes the result back
    /// onto the stack.
    pub(super) fn op_u32and(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        let result = self.bitwise.u32and(a, b)?;

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two elements off the stack, computes their bitwise OR, and pushes the result back onto
    /// the stack.
    pub(super) fn op_u32or(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        let result = self.bitwise.u32or(a, b)?;

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two elements off the stack, computes their bitwise XOR, and pushes the result back onto
    /// the stack.
    pub(super) fn op_u32xor(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack.get(0);
        let a = self.stack.get(1);
        let result = self.bitwise.u32xor(a, b)?;

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Adds 16-bit range checks to the RangeChecker for the high and low 16-bit limbs of one or two
    /// field elements which are assumed to have 32-bit integer values.
    fn add_range_checks(&mut self, lo: Felt, hi: Option<Felt>) {
        let (t0, t1) = split_element_to_u16(lo);
        self.range.add_value(t0);
        self.range.add_value(t1);

        if let Some(hi) = hi {
            let (t2, t3) = split_element_to_u16(hi);
            self.range.add_value(t2);
            self.range.add_value(t3);
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
fn split_element(value: Felt) -> (Felt, Felt) {
    let value = value.as_int();
    let lo = (value as u32) as u64;
    let hi = value >> 32;
    (Felt::new(lo), Felt::new(hi))
}

/// Splits an element into two 16 bit integer limbs. It assumes that the field element contains a
/// valid 32-bit integer value.
fn split_element_to_u16(value: Felt) -> (u16, u16) {
    let value = value.as_int() as u32;
    let lo = value as u16;
    let hi = (value >> 16) as u16;
    (lo, hi)
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{init_stack_with, Felt, FieldElement, Operation},
        Process,
    };
    use rand_utils::rand_value;

    // CASTING OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_u32split() {
        // --- test a random value ---------------------------------------------
        let a: u64 = rand_value();
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[a]);
        let hi = a >> 32;
        let lo = (a as u32) as u64;

        process.execute_op(Operation::U32split).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::new(hi);
        expected[1] = Felt::new(lo);
        assert_eq!(expected, process.stack.trace_state());

        // --- test the rest of the stack is not modified -----------------------
        let b: u64 = rand_value();
        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[a, b]);
        let hi = b >> 32;
        let lo = (b as u32) as u64;

        process.execute_op(Operation::U32split).unwrap();
        let mut expected = [Felt::ZERO; 16];
        expected[0] = Felt::new(hi);
        expected[1] = Felt::new(lo);
        expected[2] = Felt::new(a);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_u32assert2() {
        // --- test random values ensuring other elements are still values are still intact ----------
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);

        process.execute_op(Operation::U32assert2).unwrap();
        let expected = build_expected(&[a, b, c, d]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_u32add() {
        // --- test random values ---------------------------------------------
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);
        let (result, over) = a.overflowing_add(b);

        process.execute_op(Operation::U32add).unwrap();
        let expected = build_expected(&[over as u32, result, c, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test overflow --------------------------------------------------
        let a = u32::MAX - 1;
        let b = 2u32;

        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[a as u64, b as u64]);
        let (result, over) = a.overflowing_add(b);

        process.execute_op(Operation::U32add).unwrap();
        let expected = build_expected(&[over as u32, result]);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_u32addc() {
        // --- test c = 1 -----------------------------------------------------
        let a = (rand_value::<u64>() as u32) as u64;
        let b = (rand_value::<u64>() as u32) as u64;
        let c = 1u64;
        let d = (rand_value::<u64>() as u32) as u64;

        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[d, c, b, a]);

        let result = a + b + c;
        let hi = (result >> 32) as u32;
        let lo = result as u32;
        assert!(hi <= 1);

        process.execute_op(Operation::U32addc).unwrap();
        let expected = build_expected(&[hi, lo, d as u32]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test c > 1 -----------------------------------------------------
        let a = (rand_value::<u64>() as u32) as u64;
        let b = (rand_value::<u64>() as u32) as u64;
        let c = 2u64;

        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[c, b, a]);
        assert!(process.execute_op(Operation::U32addc).is_err());

        // --- test with minimum stack depth ----------------------------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::U32addc).is_ok());
    }

    #[test]
    fn op_u32sub() {
        // --- test random values ---------------------------------------------
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);
        let (result, under) = b.overflowing_sub(a);

        process.execute_op(Operation::U32sub).unwrap();
        let expected = build_expected(&[under as u32, result, c, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test underflow -------------------------------------------------
        let a = 10u32;
        let b = 11u32;

        let mut process = Process::new_dummy();
        init_stack_with(&mut process, &[a as u64, b as u64]);
        let (result, under) = a.overflowing_sub(b);

        process.execute_op(Operation::U32sub).unwrap();
        let expected = build_expected(&[under as u32, result]);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_u32mul() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);
        let result = (a as u64) * (b as u64);
        let hi = (result >> 32) as u32;
        let lo = result as u32;

        process.execute_op(Operation::U32mul).unwrap();
        let expected = build_expected(&[hi, lo, c, d]);
        assert_eq!(expected, process.stack.trace_state());
    }

    #[test]
    fn op_u32madd() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);
        let result = (a as u64) * (b as u64) + (c as u64);
        let hi = (result >> 32) as u32;
        let lo = result as u32;

        process.execute_op(Operation::U32madd).unwrap();
        let expected = build_expected(&[hi, lo, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test with minimum stack depth ----------------------------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::U32madd).is_ok());
    }

    #[test]
    fn op_u32div() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);
        let q = b / a;
        let r = b % a;

        process.execute_op(Operation::U32div).unwrap();
        let expected = build_expected(&[r, q, c, d]);
        assert_eq!(expected, process.stack.trace_state());
    }

    // BITWISE OPERATIONS
    // --------------------------------------------------------------------------------------------

    #[test]
    fn op_u32and() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);

        process.execute_op(Operation::U32and).unwrap();
        let expected = build_expected(&[a & b, c, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test with minimum stack depth ----------------------------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::U32and).is_ok());
    }

    #[test]
    fn op_u32or() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);

        process.execute_op(Operation::U32or).unwrap();
        let expected = build_expected(&[a | b, c, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test with minimum stack depth ----------------------------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::U32or).is_ok());
    }

    #[test]
    fn op_u32xor() {
        let mut process = Process::new_dummy();
        let (a, b, c, d) = init_stack_rand(&mut process);

        process.execute_op(Operation::U32xor).unwrap();
        let expected = build_expected(&[a ^ b, c, d]);
        assert_eq!(expected, process.stack.trace_state());

        // --- test with minimum stack depth ----------------------------------
        let mut process = Process::new_dummy();
        assert!(process.execute_op(Operation::U32xor).is_ok());
    }

    // HELPER FUNCTIONS
    // --------------------------------------------------------------------------------------------

    fn init_stack_rand(process: &mut Process) -> (u32, u32, u32, u32) {
        // push values a and b onto the stack
        let a = rand_value::<u64>() as u32;
        let b = rand_value::<u64>() as u32;
        let c = rand_value::<u64>() as u32;
        let d = rand_value::<u64>() as u32;
        init_stack_with(process, &[a as u64, b as u64, c as u64, d as u64]);
        (d, c, b, a)
    }

    fn build_expected(values: &[u32]) -> [Felt; 16] {
        let mut expected = [Felt::ZERO; 16];
        for (&value, result) in values.iter().zip(expected.iter_mut()) {
            *result = Felt::new(value as u64);
        }
        expected
    }
}
