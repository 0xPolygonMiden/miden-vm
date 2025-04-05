use crate::chiplets::ace::trace::EvaluationContext;
use crate::chiplets::memory::Memory;
use crate::errors::ErrorContext;
use crate::{ContextId, Felt, QuadFelt};
use miden_air::RowIndex;
use std::prelude::rust_2024::Vec;
use vm_core::FieldElement;
use vm_core::mast::MastNodeExt;

mod circuit;
mod encoder;
#[cfg(test)]
mod tests;
mod trace;

pub fn eval_circuit(
    ctx: ContextId,
    ptr: Felt,
    clk: RowIndex,
    num_vars: Felt,
    num_eval: Felt,
    mem: &mut Memory,
    error_ctx: &ErrorContext<'_, impl MastNodeExt>,
) -> Result<(), ()> {
    let num_vars = num_vars.as_int();
    let num_eval = num_eval.as_int();

    // Ensure vars and instructions are word-aligned and non-empty
    if num_vars % 2 != 0 || num_vars == 0 {
        return Err(());
    }
    if num_eval % 4 != 0 || num_eval == 0 {
        return Err(());
    }

    // Ensure instructions are word-aligned and non-empty
    let num_read_rows = num_vars as u32 / 2;
    let num_eval_rows = num_eval as u32;

    let mut evaluation_context = EvaluationContext::new(ctx, clk, num_read_rows, num_eval_rows);

    let mut ptr = ptr;
    // perform READ operations
    for _ in 0..num_read_rows {
        let word = mem.read_word(ctx, ptr, clk, error_ctx).expect("TODO");
        ptr = evaluation_context.do_read(ptr, word)?;
    }
    // perform EVAL operations
    for _ in 0..num_eval_rows {
        let instruction = mem.read(ctx, ptr, clk, error_ctx).expect("TODO");
        ptr = evaluation_context.do_eval(ptr, instruction)?;
    }

    // Ensure the circuit evaluated to zero.
    if !evaluation_context.output_value().is_some_and(|eval| eval == QuadFelt::ZERO) {
        return Err(());
    }

    Ok(())
}

/// An `EncodedCircuit` represents a `Circuit` as a list of field elements, containing both
/// constants and instructions.
struct EncodedCircuit {
    num_vars: usize,
    num_eval: usize,
    encoded_circuit: Vec<Felt>,
}
