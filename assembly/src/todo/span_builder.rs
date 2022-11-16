use super::{
    AssemblerError, BodyWrapper, Borrow, CodeBlock, Decorator, DecoratorList, Operation, Vec,
};

// SPAN BUILDER
// ================================================================================================

#[derive(Default)]
pub struct SpanBuilder {
    ops: Vec<Operation>,
    decorators: DecoratorList,
    epilogue: Vec<Operation>,
}

impl SpanBuilder {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [SpanBuilder] instantiated with the specified optional wrapper.
    ///
    /// If the wrapper is provided, the prologue of the wrapper is immediately appended to the
    /// vector of span operations. The epilogue of the wrapper is appended to the list of
    /// operations upon consumption of the builder via `extract_final_span_into()` method.
    pub(super) fn new(wrapper: Option<BodyWrapper>) -> Self {
        match wrapper {
            Some(wrapper) => Self {
                ops: wrapper.prologue,
                decorators: Vec::new(),
                epilogue: wrapper.epilogue,
            },
            None => Self::default(),
        }
    }

    // STATE MUTATORS
    // --------------------------------------------------------------------------------------------

    /// TODO: add docs
    pub fn add_op(&mut self, op: Operation) -> Result<Option<CodeBlock>, AssemblerError> {
        self.ops.push(op);
        Ok(None)
    }

    /// TODO: add docs
    pub fn add_ops<I, O>(&mut self, ops: I) -> Result<Option<CodeBlock>, AssemblerError>
    where
        I: IntoIterator<Item = O>,
        O: Borrow<Operation>,
    {
        self.ops.extend(ops.into_iter().map(|o| *o.borrow()));
        Ok(None)
    }

    /// TODO: add docs
    #[allow(dead_code)]
    pub fn push_op(&mut self, op: Operation) {
        self.ops.push(op);
    }

    /// TODO: add docs
    pub fn push_decorator(&mut self, decorator: Decorator) {
        self.decorators.push((self.ops.len(), decorator));
    }

    // SPAN CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Creates a new SPAN block from the operations and decorators currently in this builder and
    /// appends the block to the provided target.
    ///
    /// This consumes all operations and decorators in the builder, but does not touch the
    /// operations in the epilogue of the builder.
    pub fn extract_span_into(&mut self, target: &mut Vec<CodeBlock>) {
        if !self.ops.is_empty() {
            let ops = self.ops.drain(..).collect();
            let decorators = self.decorators.drain(..).collect();
            target.push(CodeBlock::new_span_with_decorators(ops, decorators));
        } else if !self.decorators.is_empty() {
            // this is a bug in the assembler. we shouldn't have decorators added without their
            // associated operations
            unreachable!()
        }
    }

    /// Creates a new SPAN block from the operations and decorators currently in this builder and
    /// appends the block to the provided target.
    ///
    /// The main differences from the `extract_span_int()` method above are:
    /// - Operations contained in the epilogue of the span builder are appended to the list of
    ///   ops which go into the new SPAN block.
    /// - The span builder is consumed in the process.
    pub fn extract_final_span_into(mut self, target: &mut Vec<CodeBlock>) {
        self.ops.append(&mut self.epilogue);
        self.extract_span_into(target);
    }
}
