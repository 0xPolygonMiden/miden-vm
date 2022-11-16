use vm_core::Decorator;

use super::{AssemblerError, Borrow, CodeBlock, DecoratorList, Operation, Vec};

// SPAN BUILDER
// ================================================================================================

#[derive(Default)]
pub struct SpanBuilder {
    ops: Vec<Operation>,
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

    pub fn push_decorator(&mut self, decorator: Decorator) {
        self.decorators.push((self.ops.len(), decorator));
    }

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
}
