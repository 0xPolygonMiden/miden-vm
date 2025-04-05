use crate::chiplets::ace::trace::EvaluationContext;
use crate::chiplets::memory::Memory;
use crate::errors::{AceError, ErrorContext};
use crate::trace::TraceFragment;
use crate::{ContextId, ExecutionError, Felt, QuadFelt};
use miden_air::RowIndex;
use std::collections::BTreeMap;
use std::prelude::rust_2024::Vec;
use trace::NUM_COLS;
use vm_core::mast::MastNodeExt;
use vm_core::{FieldElement, ZERO};

mod circuit;
mod encoder;
#[cfg(test)]
mod tests;
mod trace;

/// An `EncodedCircuit` represents a `Circuit` as a list of field elements, containing both
/// constants and instructions.
#[derive(Debug)]
struct EncodedCircuit {
    num_vars: usize,
    num_eval: usize,
    encoded_circuit: Vec<Felt>,
}

#[derive(Debug, Default)]
pub struct Ace {
    circuit_evaluations: BTreeMap<u32, EvaluationContext>,
}
impl Ace {
    pub(crate) fn trace_len(&self) -> usize {
        self.circuit_evaluations.iter().fold(0, |acc, term| acc + term.1.num_rows())
    }

    pub(crate) fn fill_trace(self, trace: &mut TraceFragment) {
        // make sure fragment dimensions are consistent with the dimensions of this trace
        debug_assert_eq!(self.trace_len(), trace.len(), "inconsistent trace lengths");
        debug_assert_eq!(NUM_COLS, trace.width(), "inconsistent trace widths");

        let mut gen_trace: [Vec<Felt>; NUM_COLS] = (0..NUM_COLS)
            .map(|_| vec![ZERO; self.trace_len()])
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to convert vector to array");

        let mut offset = 0;
        for eval_ctx in self.circuit_evaluations.into_values() {
            eval_ctx.fill(offset, &mut gen_trace);
            offset += eval_ctx.num_rows()
        }

        for (out_column, column) in trace.columns().zip(gen_trace) {
            out_column.copy_from_slice(&column);
        }
    }

    pub(crate) fn add_eval_context(&mut self, clk: RowIndex, eval_context: EvaluationContext) {
        self.circuit_evaluations.insert(clk.as_u32(), eval_context);
    }
}

pub fn eval_circuit(
    ctx: ContextId,
    ptr: Felt,
    clk: RowIndex,
    num_vars: Felt,
    num_eval: Felt,
    mem: &mut Memory,
    error_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<EvaluationContext, ExecutionError> {
    let num_vars = num_vars.as_int();
    let num_eval = num_eval.as_int();

    // Ensure vars and instructions are word-aligned and non-empty
    if num_vars % 2 != 0 || num_vars == 0 {
        return Err(ExecutionError::AceError(AceError::NumVarIsNotWordAlignedOrIsEmpty(num_vars)));
    }
    if num_eval % 4 != 0 || num_eval == 0 {
        return Err(ExecutionError::AceError(AceError::NumEvalIsNotWordAlignedOrIsEmpty(num_eval)));
    }

    // Ensure instructions are word-aligned and non-empty
    let num_read_rows = num_vars as u32 / 2;
    let num_eval_rows = num_eval as u32;

    let mut evaluation_context = EvaluationContext::new(ctx, clk, num_read_rows, num_eval_rows);

    let mut ptr = ptr;
    // perform READ operations
    for _ in 0..num_read_rows {
        let word = mem.read_word(ctx, ptr, clk, error_ctx).map_err(ExecutionError::MemoryError)?;
        ptr = evaluation_context.do_read(ptr, word)?;
    }
    // perform EVAL operations
    for _ in 0..num_eval_rows {
        let instruction =
            mem.read(ctx, ptr, clk, error_ctx).map_err(ExecutionError::MemoryError)?;
        ptr = evaluation_context.do_eval(ptr, instruction)?;
    }

    // Ensure the circuit evaluated to zero.
    if !evaluation_context.output_value().is_some_and(|eval| eval == QuadFelt::ZERO) {
        return Err(ExecutionError::AceError(AceError::CircuitNotEvaluateZero));
    }

    Ok(evaluation_context)
}
