use super::{BaseElement, ExecutionError, Processor};

// INPUT OPERATIONS
// ================================================================================================

impl Processor {
    /// Pushes the provided value onto the stack.
    ///
    /// The original stack is shifted to the right by one item.
    pub(super) fn op_push(&mut self, value: BaseElement) -> Result<(), ExecutionError> {
        self.stack.set(0, value);
        self.stack.shift_right(0);
        Ok(())
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{
        super::{FieldElement, Operation},
        BaseElement, Processor,
    };

    #[test]
    fn op_push() {
        let mut processor = Processor::new_dummy();
        {
            assert_eq!(0, processor.stack.depth());
            assert_eq!(0, processor.stack.current_step());
            assert_eq!([BaseElement::ZERO; 16], processor.stack.trace_state());
        }

        // push one item onto the stack
        let op = Operation::Push(BaseElement::ONE);
        processor.execute_op(op).unwrap();
        {
            let mut expected = [BaseElement::ZERO; 16];
            expected[0] = BaseElement::ONE;

            assert_eq!(1, processor.stack.depth());
            assert_eq!(1, processor.stack.current_step());
            assert_eq!(expected, processor.stack.trace_state());
        }

        // push another item onto the stack
        let op = Operation::Push(BaseElement::new(3));
        processor.execute_op(op).unwrap();
        {
            let mut expected = [BaseElement::ZERO; 16];
            expected[0] = BaseElement::new(3);
            expected[1] = BaseElement::ONE;

            assert_eq!(2, processor.stack.depth());
            assert_eq!(2, processor.stack.current_step());
            assert_eq!(expected, processor.stack.trace_state());
        }
    }
}
