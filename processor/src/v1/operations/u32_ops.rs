use super::{utils::assert_binary, BaseElement, ExecutionError, Process, StarkField};

impl Process {
    // CASTING OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops the top element off the stack, splits it into low and high 32-bit values, and pushes
    /// these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    pub(super) fn op_u32split(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(1, "U32SPLIT")?;

        let a = self.stack.get(0);
        let (lo, hi) = split_element(a);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.shift_right(1);
        Ok(())
    }

    // ARITHMETIC OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two element off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32add(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32ADD")?;

        let b = self.stack.get(0);
        let a = self.stack.get(1);
        let result = a + b;
        let (lo, hi) = split_element(result);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops three element off the stack, adds them, splits the result into low and high 32-bit
    /// values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if:
    /// * The stack contains fewer than three elements.
    /// * The third element from the top fo the stack is not a binary value.
    pub(super) fn op_u32addc(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(3, "U32ADDC")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let c = assert_binary(self.stack.get(2))?.as_int();
        let result = BaseElement::new(a + b + c);
        let (lo, hi) = split_element(result);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops two elements off the stack, subtracts the top element from the second element, and
    /// pushes the result as well as a flag indicating whether there was underflow back onto the
    /// stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32sub(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32SUB")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = a - b;

        self.stack.set(0, BaseElement::new(result >> 63));
        self.stack.set(1, BaseElement::new((result as u32) as u64));
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops two element off the stack, multiplies them, splits the result into low and high
    /// 32-bit values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32mul(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32MUL")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = BaseElement::new(a * b);
        let (lo, hi) = split_element(result);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops two elements off the stack, divides the second element by the top element, and pushes
    /// the quotient and the remainder back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32madd(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(3, "U32MADD")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let c = self.stack.get(2).as_int();
        let result = BaseElement::new(a * b + c);
        let (lo, hi) = split_element(result);

        self.stack.set(0, hi);
        self.stack.set(1, lo);
        self.stack.copy_state(2);
        Ok(())
    }

    /// Pops three element off the stack, multiplies the first two and adds the third element to
    /// the result, splits the result into low and high 32-bit values, and pushes these values
    /// back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than three elements.
    pub(super) fn op_u32div(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32DIV")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let q = a / b;
        let r = a - q * b;

        self.stack.set(0, BaseElement::new(r));
        self.stack.set(1, BaseElement::new(q));
        self.stack.copy_state(2);
        Ok(())
    }

    // BITWISE OPERATIONS
    // --------------------------------------------------------------------------------------------

    /// Pops two element off the stack, computes their bitwise AND, splits the result into low and
    /// high 32-bit values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32and(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32AND")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = BaseElement::new(a & b);

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two element off the stack, computes their bitwise OR, splits the result into low and
    /// high 32-bit values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32or(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32OR")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = BaseElement::new(a | b);

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }

    /// Pops two element off the stack, computes their bitwise XOR, splits the result into low and
    /// high 32-bit values, and pushes these values back onto the stack.
    ///
    /// # Errors
    /// Returns an error if the stack contains fewer than two elements.
    pub(super) fn op_u32xor(&mut self) -> Result<(), ExecutionError> {
        self.stack.check_depth(2, "U32XOR")?;

        let b = self.stack.get(0).as_int();
        let a = self.stack.get(1).as_int();
        let result = BaseElement::new(a ^ b);

        self.stack.set(0, result);
        self.stack.shift_left(2);
        Ok(())
    }
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline(always)]
fn split_element(value: BaseElement) -> (BaseElement, BaseElement) {
    let value = value.as_int();
    let lo = (value as u32) as u64;
    let hi = value >> 32;
    (BaseElement::new(lo), BaseElement::new(hi))
}
