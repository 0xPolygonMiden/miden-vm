use vm_core::{FieldElement, ONE, ZERO};

use super::MainTraceFragmentGenerator;

impl MainTraceFragmentGenerator {
    /// Pops two elements off the stack, adds them together, and pushes the result back onto the
    /// stack.
    pub(crate) fn op_add(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);
        self.stack_set(0, a + b);
        self.stack_shift_left(1);
    }

    /// Pops an element off the stack, computes its additive inverse, and pushes the result back
    /// onto the stack.
    pub(crate) fn op_neg(&mut self) {
        let a = self.stack_get(0);
        self.stack_set(0, -a);
    }

    /// Pops two elements off the stack, multiplies them, and pushes the result back onto the
    /// stack.
    pub(crate) fn op_mul(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);
        self.stack_set(0, a * b);
        self.stack_shift_left(1);
    }

    /// Pops an element off the stack, computes its multiplicative inverse, and pushes the result
    /// back onto the stack.
    pub(crate) fn op_inv(&mut self) {
        let a = self.stack_get(0);
        if a == ZERO {
            // In parallel execution, we don't use ErrorContext, so we'll just panic on division by
            // zero
            panic!("Division by zero in op_inv at clock {}", self.state.system.clk);
        }

        self.stack_set(0, a.inv());
    }

    /// Pops an element off the stack, adds ONE to it, and pushes the result back onto the stack.
    pub(crate) fn op_incr(&mut self) {
        let a = self.stack_get(0);
        self.stack_set(0, a + ONE);
    }

    /// Pops two elements off the stack, computes their boolean AND, and pushes the result back
    /// onto the stack.
    pub(crate) fn op_and(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        // Boolean AND: result is ONE only if both operands are ONE
        if a == ONE && b == ONE {
            self.stack_set(0, ONE);
        } else {
            self.stack_set(0, ZERO);
        }
        self.stack_shift_left(1);
    }

    /// Pops two elements off the stack, computes their boolean OR, and pushes the result back
    /// onto the stack.
    pub(crate) fn op_or(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        // Boolean OR: result is ONE if at least one operand is ONE
        if a == ONE || b == ONE {
            self.stack_set(0, ONE);
        } else {
            self.stack_set(0, ZERO);
        }
        self.stack_shift_left(1);
    }

    /// Pops an element off the stack, computes its boolean NOT, and pushes the result back onto
    /// the stack.
    pub(crate) fn op_not(&mut self) {
        let a = self.stack_get(0);

        // Boolean NOT: ONE becomes ZERO, ZERO becomes ONE
        if a == ZERO {
            self.stack_set(0, ONE);
        } else {
            self.stack_set(0, ZERO);
        }
    }

    /// Pops two elements off the stack, tests whether they are equal, and pushes ONE onto the
    /// stack if they are equal, otherwise pushes ZERO onto the stack.
    pub(crate) fn op_eq(&mut self) {
        let b = self.stack_get(0);
        let a = self.stack_get(1);

        if a == b {
            self.stack_set(0, ONE);
        } else {
            self.stack_set(0, ZERO);
        }
        self.stack_shift_left(1);
    }

    /// Pops an element off the stack, tests whether it is zero, and pushes ONE onto the stack if
    /// it is zero, otherwise pushes ZERO onto the stack.
    pub(crate) fn op_eqz(&mut self) {
        let a = self.stack_get(0);

        if a == ZERO {
            self.stack_set(0, ONE);
        } else {
            self.stack_set(0, ZERO);
        }
    }

    /// Exponent accumulation used to keep track of a bit decomposition.
    /// Takes the next exponent bit, the base, and the current accumulator value. It computes
    /// the new accumulator value by adding the next bit times the current base to the current
    /// accumulator, and then updates the base to be the current base squared.
    pub(crate) fn op_expacc(&mut self) {
        // For parallel trace generation, we assume the logic is already validated
        let bit = self.stack_get(0);
        let base = self.stack_get(1);
        let acc = self.stack_get(2);

        let new_acc = acc + bit * base;
        let new_base = base * base;

        self.stack_set(0, ZERO); // next exponent bit
        self.stack_set(1, new_base);
        self.stack_set(2, new_acc);
        self.stack_shift_left(1);
    }
}
