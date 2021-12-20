use vm_core::{v1::program::Operation, BaseElement};

use super::ExecutionError;

pub struct Stack {}

impl Stack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, _op: Operation) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub fn noop(&mut self) {}

    pub fn drop(&mut self) -> Result<(), ExecutionError> {
        unimplemented!()
    }

    pub fn peek(&self) -> Result<BaseElement, ExecutionError> {
        unimplemented!()
    }
}
