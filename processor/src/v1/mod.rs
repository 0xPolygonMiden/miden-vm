use vm_core::v1::{
    program::{
        blocks::{CodeBlock, Join, Loop, Span, Split},
        Operation, ProgramInputs, Script,
    },
    BaseElement, FieldElement, StarkField, STACK_TOP_SIZE,
};

mod operations;

mod decoder;
use decoder::Decoder;

mod stack;
use stack::Stack;

mod memory;
use memory::Memory;

mod trace;
pub use trace::ExecutionTrace;

mod errors;
pub use errors::ExecutionError;

#[cfg(test)]
mod tests;

// TYPE ALIASES
// ================================================================================================

type Word = [BaseElement; 4];
type StackTrace = [Vec<BaseElement>; STACK_TOP_SIZE];

// PROCESSOR
// ================================================================================================

pub struct Processor {
    step: usize,
    decoder: Decoder,
    stack: Stack,
    memory: Memory,
}

impl Processor {
    pub fn new(inputs: ProgramInputs) -> Self {
        Self {
            step: 0,
            decoder: Decoder::new(),
            stack: Stack::new(&inputs, 4),
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, script: &Script) -> Result<ExecutionTrace, ExecutionError> {
        self.execute_code_block(script.root())?;

        self.stack.finalize();

        // TODO: get rid of cloning
        let trace = ExecutionTrace::new(self.stack.trace().clone());

        Ok(trace)
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    // CODE BLOCK EXECUTORS
    // --------------------------------------------------------------------------------------------

    /// Executes the specified [CodeBlock].
    ///
    /// # Errors
    /// Returns an [ExecutionError] if executing the specified block fails for any reason.
    fn execute_code_block(&mut self, block: &CodeBlock) -> Result<(), ExecutionError> {
        match block {
            CodeBlock::Join(block) => self.execute_join_block(block),
            CodeBlock::Split(block) => self.execute_split_block(block),
            CodeBlock::Loop(block) => self.execute_loop_block(block),
            CodeBlock::Span(block) => self.execute_span_block(block),
            CodeBlock::Proxy(_) => Err(ExecutionError::UnexecutableCodeBlock(block.clone())),
            _ => Err(ExecutionError::UnsupportedCodeBlock(block.clone())),
        }
    }

    /// Executes the specified [Join] block.
    #[inline(always)]
    fn execute_join_block(&mut self, block: &Join) -> Result<(), ExecutionError> {
        // start JOIN block; state of the stack does not change
        self.decoder.start_join(block);
        self.execute_op(Operation::Noop)?;

        // execute first and then second child of the join block
        self.execute_code_block(block.first())?;
        self.execute_code_block(block.second())?;

        // end JOIN block; state of the stack does not change
        self.decoder.end_join(block);
        self.execute_op(Operation::Noop)?;

        Ok(())
    }

    /// Executes the specified [Split] block.
    #[inline(always)]
    fn execute_split_block(&mut self, block: &Split) -> Result<(), ExecutionError> {
        // start SPLIT block; this also removes the top stack item to determine which branch of
        // the block should be executed.
        let condition = self.stack.peek()?;
        self.decoder.start_split(block, condition);
        self.execute_op(Operation::Drop)?;

        // execute either the true or the false branch of the split block based on the condition
        // retrieved from the top of the stack
        if condition == BaseElement::ONE {
            self.execute_code_block(block.on_true())?;
        } else if condition == BaseElement::ZERO {
            self.execute_code_block(block.on_false())?;
        } else {
            return Err(ExecutionError::NotBinaryValue(condition));
        }

        // end SPLIT block; state of the stack does not change
        self.decoder.end_split(block);
        self.execute_op(Operation::Noop)?;

        Ok(())
    }

    /// Executes the specified [Loop] block.
    #[inline(always)]
    fn execute_loop_block(&mut self, block: &Loop) -> Result<(), ExecutionError> {
        // start LOOP block; this requires examining the top of the stack to determine whether
        // the loop's body should be executed.
        let condition = self.stack.peek()?;
        self.decoder.start_loop(block, condition);

        // if the top of the stack is ONE, execute the loop body; otherwise skip the loop body;
        // before we execute the loop body we drop the condition from the stack; when the loop
        // body is not executed, we keep the condition on the stack so that it can be dropped by
        // the END operation later.
        if condition == BaseElement::ONE {
            // drop the condition and execute the loop body at least once
            self.execute_op(Operation::Drop)?;
            self.execute_code_block(block.body())?;

            // keep executing the loop body until the condition on the top of the stack is no
            // longer ONE; each iteration of the loop is preceded by executing REPEAT operation
            // which drops the condition from the stack
            while self.stack.peek()? == BaseElement::ONE {
                self.execute_op(Operation::Drop)?;
                self.decoder.repeat(block);

                self.execute_code_block(block.body())?;
            }
        } else if condition == BaseElement::ZERO {
            self.execute_op(Operation::Noop)?
        } else {
            return Err(ExecutionError::NotBinaryValue(condition));
        }

        // execute END operation; this can be done only if the top of the stack is ZERO, in which
        // case the top of the stack is dropped
        if self.stack.peek()? == BaseElement::ZERO {
            self.execute_op(Operation::Drop)?;
        } else if condition == BaseElement::ONE {
            unreachable!("top of the stack should not be ONE");
        } else {
            return Err(ExecutionError::NotBinaryValue(self.stack.peek()?));
        }
        self.decoder.end_loop(block);

        Ok(())
    }

    /// Executes the specified [Span] block.
    #[inline(always)]
    fn execute_span_block(&mut self, block: &Span) -> Result<(), ExecutionError> {
        // start the SPAN block and get the first operation batch from it; when executing a SPAN
        // operation the state of the stack does not change
        self.decoder.start_span(block);
        self.execute_op(Operation::Noop)?;

        // execute the first operation batch
        for &op in block.op_batches()[0].ops() {
            self.execute_op(op)?;
            self.decoder.execute_user_op(op);
        }

        // if the span contains more operation batches, execute them. each additional batch is
        // preceded by a RESPAN operation; executing RESPAN operation does not change the state
        // of the stack
        for op_batch in block.op_batches().iter().skip(1) {
            self.decoder.respan(op_batch);
            self.execute_op(Operation::Noop)?;
            for &op in op_batch.ops() {
                self.execute_op(op)?;
                self.decoder.execute_user_op(op);
            }
        }

        // end the SPAN block; when executing an END operation the state of the stack does not
        // change
        self.decoder.end_span(block);
        self.execute_op(Operation::Noop)?;

        Ok(())
    }
}
