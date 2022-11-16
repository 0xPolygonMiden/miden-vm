use super::{AssemblerError, Borrow, CodeBlock, DecoratorList, Operation, Vec};

// SPAN BUILDER
// ================================================================================================

#[derive(Default)]
pub struct SpanBuilder {
    ops: Vec<Operation>,
    #[allow(dead_code)]
    decorators: DecoratorList,
}

impl SpanBuilder {
    pub fn add_op(&mut self, op: Operation) -> Result<Option<CodeBlock>, AssemblerError> {
        self.ops.push(op);
        Ok(None)
    }

    pub fn add_ops<I, O>(&mut self, ops: I) -> Result<Option<CodeBlock>, AssemblerError>
    where
        I: IntoIterator<Item = O>,
        O: Borrow<Operation>,
    {
        self.ops.extend(ops.into_iter().map(|o| *o.borrow()));
        Ok(None)
    }

    #[allow(dead_code)]
    pub fn push_op(&mut self, op: Operation) {
        self.ops.push(op);
    }

    #[allow(dead_code)]
    pub fn has_ops(&self) -> bool {
        self.ops.is_empty()
    }

    pub fn extract_span_into(&mut self, target: &mut Vec<CodeBlock>) {
        if !self.ops.is_empty() {
            let ops: Vec<_> = self.ops.drain(..).collect();
            target.push(CodeBlock::new_span(ops));
        }
    }
}
