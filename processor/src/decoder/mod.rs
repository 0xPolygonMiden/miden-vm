use vm_core::FieldElement;

use super::{ExecutionError, Felt, Join, Loop, OpBatch, Operation, Process, Span, Split};

// DECODER PROCESS EXTENSION
// ================================================================================================

impl Process {
    // JOIN BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        // TODO: get address from hasher
        let addr = Felt::ZERO;
        self.decoder.start_join(block, addr);

        Ok(())
    }

    pub(super) fn end_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        self.decoder.end_join(block);

        Ok(())
    }

    // SPLIT BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        let condition = self.stack.peek();
        self.execute_op(Operation::Drop)?;

        // TODO: get address from hasher
        let addr = Felt::ZERO;
        self.decoder.start_split(block, addr, condition);

        Ok(())
    }

    pub(super) fn end_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        self.decoder.end_split(block);

        Ok(())
    }

    // SPAN BLOCK
    // --------------------------------------------------------------------------------------------

    pub(super) fn start_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        // TODO: get address from hasher
        let addr = Felt::ZERO;
        self.decoder.start_span(block, addr);

        Ok(())
    }

    pub(super) fn end_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        self.execute_op(Operation::Noop)?;

        self.decoder.end_span(block);

        Ok(())
    }
}

// DECODER
// ================================================================================================
pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    // JOIN BLOCK
    // --------------------------------------------------------------------------------------------

    pub fn start_join(&mut self, _block: &Join, _addr: Felt) {}

    pub fn end_join(&mut self, _block: &Join) {}

    // SPLIT BLOCK
    // --------------------------------------------------------------------------------------------

    pub fn start_split(&mut self, _block: &Split, _addr: Felt, _condition: Felt) {}

    pub fn end_split(&mut self, _block: &Split) {}

    // LOOP BLOCK
    // --------------------------------------------------------------------------------------------

    pub fn start_loop(&mut self, _block: &Loop, _condition: Felt) {}

    pub fn repeat(&mut self, _block: &Loop) {}

    pub fn end_loop(&mut self, _block: &Loop) {}

    // SPAN BLOCK
    // --------------------------------------------------------------------------------------------
    pub fn start_span(&mut self, _block: &Span, _addr: Felt) {}

    pub fn respan(&mut self, _op_batch: &OpBatch) {}

    pub fn execute_user_op(&mut self, _op: Operation) {}

    pub fn end_span(&mut self, _block: &Span) {}
}
