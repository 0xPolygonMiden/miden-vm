use super::{BaseElement, ExecutionError, Stack};

// INPUT OPERATIONS
// ================================================================================================

impl Stack {
    /// Pushes the provided value onto the stack. The original stack is shifted to the right by
    /// one item.
    pub(super) fn op_push(&mut self, value: BaseElement) -> Result<(), ExecutionError> {
        self.trace[0][self.step + 1] = value;
        self.shift_right(0);
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{FieldElement, Operation},
        BaseElement, Stack,
    };

    #[test]
    fn op_push() {
        let mut stack = Stack::new(2);
        assert_eq!(0, stack.depth());
        assert_eq!(0, stack.current_step());
        assert_eq!([BaseElement::ZERO; 16], stack.trace_state());

        // push one item onto the stack
        stack.execute(Operation::Push(BaseElement::ONE)).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::ONE;

        assert_eq!(1, stack.depth());
        assert_eq!(1, stack.current_step());
        assert_eq!(expected, stack.trace_state());

        // push another item onto the stack
        stack.execute(Operation::Push(BaseElement::new(3))).unwrap();
        let mut expected = [BaseElement::ZERO; 16];
        expected[0] = BaseElement::new(3);
        expected[1] = BaseElement::ONE;

        assert_eq!(2, stack.depth());
        assert_eq!(2, stack.current_step());
        assert_eq!(expected, stack.trace_state());
    }
}
