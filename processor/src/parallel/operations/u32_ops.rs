use vm_core::{Felt, ZERO};

use super::MainTraceFragmentGenerator;

// Helper function to split a field element into high and low 32-bit values
fn split_element(value: Felt) -> (Felt, Felt) {
    let value_u64 = value.as_int();
    let hi = Felt::new(value_u64 >> 32);
    let lo = Felt::new(value_u64 & 0xffffffff);
    (hi, lo)
}

impl MainTraceFragmentGenerator {
    /// Pops an element off the stack, splits it into upper and lower 32-bit values, and pushes
    /// these values back onto the stack.
    pub(crate) fn u32split(&mut self) {
        let a = self.stack_get(0);
        let (hi, lo) = split_element(a);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
    }

    /// Pops two elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    pub(crate) fn u32add(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        let result = Felt::new(a.as_int() + b.as_int());
        let (hi, lo) = split_element(result);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
    }

    /// Pops three elements off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    pub(crate) fn u32add3(&mut self) {
        let c = self.stack_get(0);
        let b = self.stack_get(1);
        let a = self.stack_get(2);

        let result = Felt::new(a.as_int() + b.as_int() + c.as_int());
        let (hi, lo) = split_element(result);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
        self.stack_shift_left(1);
    }

    /// Pops two elements off the stack, subtracts the top element from the second element,
    /// splits the result into low and high 32-bit values, and pushes these values back onto the
    /// stack.
    pub(crate) fn u32sub(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        let result = if a.as_int() >= b.as_int() {
            Felt::new(a.as_int() - b.as_int())
        } else {
            // Handle underflow by adding 2^64
            Felt::new((1u64 << 32) + a.as_int() - b.as_int())
        };
        let (hi, lo) = split_element(result);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
    }

    /// Pops two elements off the stack, multiplies them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    pub(crate) fn u32mul(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        let result = Felt::new(a.as_int() * b.as_int());
        let (hi, lo) = split_element(result);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
    }

    /// Pops three elements off the stack, multiplies the first two and adds the third,
    /// splits the result into low and high 32-bit values, and pushes these values back onto the
    /// stack.
    pub(crate) fn u32madd(&mut self) {
        let c = self.stack_get(0);
        let b = self.stack_get(1);
        let a = self.stack_get(2);

        let result = Felt::new(a.as_int() * b.as_int() + c.as_int());
        let (hi, lo) = split_element(result);

        self.stack_set(0, hi);
        self.stack_set(1, lo);
        self.stack_shift_left(1);
    }

    /// Pops two elements off the stack, performs integer division, and pushes the quotient and
    /// remainder back onto the stack.
    pub(crate) fn u32div(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        if b == ZERO {
            panic!("Division by zero at clock {}", self.state.system.clk);
        }

        let quotient = Felt::new(a.as_int() / b.as_int());
        let remainder = Felt::new(a.as_int() % b.as_int());

        self.stack_set(0, quotient);
        self.stack_set(1, remainder);
    }

    /// Pops two elements off the stack, performs bitwise AND, and pushes the result back onto the
    /// stack.
    pub(crate) fn u32and(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        let result = Felt::new(a.as_int() & b.as_int());

        self.stack_set(0, result);
        self.stack_shift_left(1);
    }

    /// Pops two elements off the stack, performs bitwise XOR, and pushes the result back onto the
    /// stack.
    pub(crate) fn u32xor(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        let result = Felt::new(a.as_int() ^ b.as_int());

        self.stack_set(0, result);
        self.stack_shift_left(1);
    }

    /// Pops two elements off the stack and verifies they are valid 32-bit values.
    pub(crate) fn u32assert2(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        if a.as_int() > u32::MAX as u64 {
            panic!(
                "Value {} is not a valid 32-bit integer at clock {}",
                a.as_int(),
                self.state.system.clk
            );
        }
        if b.as_int() > u32::MAX as u64 {
            panic!(
                "Value {} is not a valid 32-bit integer at clock {}",
                b.as_int(),
                self.state.system.clk
            );
        }

        // Elements remain on stack unchanged
    }
}
