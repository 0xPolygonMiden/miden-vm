use super::FastProcessor;
use crate::{ExecutionError, chiplets::eval_circuit_fast, errors::ErrorContext};

impl FastProcessor {
    /// Checks that the evaluation of an arithmetic circuit is equal to zero.
    ///
    /// The inputs are composed of:
    ///
    /// 1. a pointer to the memory region containing the arithmetic circuit description, which
    ///    itself is arranged as:
    ///
    ///    a. `Read` section:
    ///       1. Inputs to the circuit which are elements in the quadratic extension field,
    ///       2. Constants of the circuit which are elements in the quadratic extension field,
    ///
    ///    b. `Eval` section, which contains the encodings of the evaluation gates of the circuit.
    ///    Each gate is encoded as a single base field element.
    /// 2. the number of rows in the `READ` section,
    /// 3. the number of rows in the `EVAL` section,
    ///
    /// Stack transition:
    /// [ptr, num_read_rows, num_eval_rows, ...] -> [...]
    pub fn arithmetic_circuit_eval(&mut self, op_idx: usize) -> Result<(), ExecutionError> {
        let num_eval_rows = self.stack_get(2);
        let num_read_rows = self.stack_get(1);
        let ptr = self.stack_get(0);
        let ctx = self.ctx;
        let clk = self.clk;
        let eval_context = eval_circuit_fast(
            ctx,
            ptr,
            clk,
            num_read_rows,
            num_eval_rows,
            &mut self.memory,
            op_idx,
            &ErrorContext::default(),
        )?;
        self.ace.add_eval_context(clk, eval_context);

        Ok(())
    }
}
