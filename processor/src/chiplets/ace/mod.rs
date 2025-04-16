#![allow(dead_code)]

use crate::chiplets::ace::trace::EvaluationContext;
use crate::chiplets::memory::Memory;
use crate::errors::{AceError, ErrorContext};
use crate::trace::TraceFragment;
use crate::{ContextId, ExecutionError, Felt, QuadFelt};
use miden_air::RowIndex;
use miden_air::trace::main_trace::MainTrace;
use std::collections::BTreeMap;
use std::prelude::rust_2024::Vec;
use trace::NUM_COLS;
use vm_core::mast::MastNodeExt;
use vm_core::{FieldElement, ZERO};

mod trace;

#[cfg(test)]
mod tests;
mod encoded_circuit;

pub use trace::{NUM_ACE_LOGUP_FRACTIONS_EVAL, NUM_ACE_LOGUP_FRACTIONS_READ};


#[derive(Debug, Default)]
pub struct Ace {
    circuit_evaluations: BTreeMap<u32, EvaluationContext>,
    sections_info: Vec<AceSection>,
}

impl Ace {
    pub(crate) fn trace_len(&self) -> usize {
        self.circuit_evaluations.iter().fold(0, |acc, term| acc + term.1.num_rows())
    }

    pub(crate) fn fill_trace(mut self, trace: &mut TraceFragment) -> Vec<AceSection> {
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
            offset += eval_ctx.num_rows();
            let section = AceSection::from_evaluation_context(&eval_ctx);
            self.sections_info.push(section);
        }

        for (out_column, column) in trace.columns().zip(gen_trace) {
            out_column.copy_from_slice(&column);
        }

        self.sections_info
    }

    pub(crate) fn add_eval_context(&mut self, clk: RowIndex, eval_context: EvaluationContext) {
        self.circuit_evaluations.insert(clk.as_u32(), eval_context);
    }
}

#[derive(Debug, Default)]
pub struct AceSection {
    ctx: u32,
    clk: u32,
    num_vars: u32,
    num_evals: u32,
}

impl AceSection {
    pub fn clk(&self) -> u32 {
        self.clk
    }

    pub fn ctx(&self) -> u32 {
        self.ctx
    }

    pub fn num_vars(&self) -> u32 {
        self.num_vars
    }

    pub fn num_evals(&self) -> u32 {
        self.num_evals
    }

    fn from_evaluation_context(eval_ctx: &EvaluationContext) -> AceSection {
        AceSection {
            ctx: eval_ctx.ctx(),
            clk: eval_ctx.clk(),
            num_vars: eval_ctx.num_read_rows(),
            num_evals: eval_ctx.num_eval_rows(),
        }
    }
}
#[derive(Debug, Default)]
pub struct AceHints {
    offset_chiplet_trace: usize,
    pub sections: Vec<AceSection>,
}

impl AceHints {
    pub fn new(offset_chiplet_trace: usize, sections: Vec<AceSection>) -> Self {
        Self { offset_chiplet_trace, sections }
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset_chiplet_trace
    }

    pub(crate) fn build_divisors<E: FieldElement<BaseField = Felt>>(
        &self,
        main_trace: &MainTrace,
        alphas: &[E],
    ) -> Vec<E> {
        let num_fractions = self.num_fractions();
        let mut total_values = vec![E::ZERO; num_fractions];
        let mut total_inv_values = vec![E::ZERO; num_fractions];

        let mut chiplet_offset = self.offset_chiplet_trace;
        let mut values_offset = 0;
        let mut acc = E::ONE;
        for section in self.sections.iter() {
            let clk = section.clk();
            let ctx = section.ctx();

            let values = &mut total_values[values_offset
                ..values_offset + NUM_ACE_LOGUP_FRACTIONS_READ * section.num_vars() as usize];
            let inv_values = &mut total_inv_values[values_offset
                ..values_offset + NUM_ACE_LOGUP_FRACTIONS_READ * section.num_vars() as usize];

            // read section
            for (i, (value, inv_value)) in values
                .chunks_mut(NUM_ACE_LOGUP_FRACTIONS_READ)
                .zip(inv_values.chunks_mut(NUM_ACE_LOGUP_FRACTIONS_READ))
                .enumerate()
            {
                let trace_row = i + chiplet_offset;

                let wire_0 = main_trace.chiplet_ace_wire_0(trace_row.into());
                let wire_1 = main_trace.chiplet_ace_wire_1(trace_row.into());

                let value_0 = alphas[0]
                    + alphas[1].mul_base(Felt::from(clk))
                    + alphas[2].mul_base(Felt::from(ctx))
                    + alphas[3].mul_base(wire_0[0])
                    + alphas[4].mul_base(wire_0[1])
                    + alphas[5].mul_base(wire_0[2]);
                let value_1 = alphas[0]
                    + alphas[1].mul_base(Felt::from(clk))
                    + alphas[2].mul_base(Felt::from(ctx))
                    + alphas[3].mul_base(wire_1[0])
                    + alphas[4].mul_base(wire_1[1])
                    + alphas[5].mul_base(wire_1[2]);

                value[0] = value_0;
                value[1] = value_1;
                inv_value[0] = acc;
                acc *= value_0;
                inv_value[1] = acc;
                acc *= value_1;
            }

            chiplet_offset += section.num_vars() as usize;
            values_offset += NUM_ACE_LOGUP_FRACTIONS_READ * section.num_vars() as usize;

            // eval section
            let values = &mut total_values[values_offset
                ..values_offset + NUM_ACE_LOGUP_FRACTIONS_EVAL * section.num_evals() as usize];
            let inv_values = &mut total_inv_values[values_offset
                ..values_offset + NUM_ACE_LOGUP_FRACTIONS_EVAL * section.num_evals() as usize];
            for (i, (value, inv_value)) in values
                .chunks_mut(NUM_ACE_LOGUP_FRACTIONS_EVAL)
                .zip(inv_values.chunks_mut(NUM_ACE_LOGUP_FRACTIONS_EVAL))
                .enumerate()
            {
                let trace_row = i + chiplet_offset;

                let wire_0 = main_trace.chiplet_ace_wire_0(trace_row.into());
                let wire_1 = main_trace.chiplet_ace_wire_1(trace_row.into());
                let wire_2 = main_trace.chiplet_ace_wire_2(trace_row.into());

                let value_0 = alphas[0]
                    + alphas[1].mul_base(Felt::from(clk))
                    + alphas[2].mul_base(Felt::from(ctx))
                    + alphas[3].mul_base(wire_0[0])
                    + alphas[4].mul_base(wire_0[1])
                    + alphas[5].mul_base(wire_0[2]);

                let value_1 = alphas[0]
                    + alphas[1].mul_base(Felt::from(clk))
                    + alphas[2].mul_base(Felt::from(ctx))
                    + alphas[3].mul_base(wire_1[0])
                    + alphas[4].mul_base(wire_1[1])
                    + alphas[5].mul_base(wire_1[2]);

                let value_2 = alphas[0]
                    + alphas[1].mul_base(Felt::from(clk))
                    + alphas[2].mul_base(Felt::from(ctx))
                    + alphas[3].mul_base(wire_2[0])
                    + alphas[4].mul_base(wire_2[1])
                    + alphas[5].mul_base(wire_2[2]);

                value[0] = value_0;
                value[1] = value_1;
                value[2] = value_2;
                inv_value[0] = acc;
                acc *= value_0;
                inv_value[1] = acc;
                acc *= value_1;
                inv_value[2] = acc;
                acc *= value_2;
            }

            chiplet_offset += section.num_evals() as usize;
            values_offset += NUM_ACE_LOGUP_FRACTIONS_EVAL * section.num_evals() as usize;
        }

        // invert the accumulated product
        acc = acc.inv();

        for i in (0..total_values.len()).rev() {
            total_inv_values[i] *= acc;
            acc *= total_values[i];
        }

        total_inv_values
    }

    fn num_fractions(&self) -> usize {
        self.sections
            .iter()
            .fold(0, |acc, term| acc + 2 * term.num_vars + 3 * term.num_evals) as usize
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
