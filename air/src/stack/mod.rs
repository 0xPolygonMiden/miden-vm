use vm_core::{utils::collections::Vec, StarkField, STACK_AUX_TRACE_OFFSET};

use super::{
    Assertion, AuxTraceRandElements, EvaluationFrame, Felt, FieldElement, MIN_STACK_DEPTH,
    STACK_TRACE_OFFSET,
};

pub mod op_flags;

// CONSTANTS
// ================================================================================================

const B0_COL_IDX: usize = STACK_TRACE_OFFSET + MIN_STACK_DEPTH;
const B1_COL_IDX: usize = B0_COL_IDX + 1;

// --- Main constraints ---------------------------------------------------------------------------

/// The number of boundary constraints required by the Stack, which is all stack positions for
/// inputs and outputs as well as the initial values of the bookkeeping columns.
pub const NUM_ASSERTIONS: usize = MIN_STACK_DEPTH + 2;

// --- Auxiliary column constraints ---------------------------------------------------------------

/// The number of auxiliary assertions.
pub const NUM_AUX_ASSERTIONS: usize = 1;

// BOUNDARY CONSTRAINTS
// ================================================================================================

// --- MAIN TRACE ---------------------------------------------------------------------------------

/// Returns the stack's boundary assertions for the main trace at the first step.
pub fn get_assertions_first_step(result: &mut Vec<Assertion<Felt>>, stack_inputs: &[Felt]) {
    // stack columns at the first step should be set to stack inputs, excluding overflow inputs.
    for (i, &value) in stack_inputs.iter().take(MIN_STACK_DEPTH).enumerate() {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, 0, value));
    }

    // if there are remaining slots on top of the stack without specified values, set them to ZERO.
    for i in stack_inputs.len()..MIN_STACK_DEPTH {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, 0, Felt::ZERO));
    }

    // get the initial values for the bookkeeping columns.
    let mut depth = MIN_STACK_DEPTH;
    let mut overflow_addr = Felt::ZERO;
    if stack_inputs.len() > MIN_STACK_DEPTH {
        depth = stack_inputs.len();
        overflow_addr = Felt::new(Felt::MODULUS - 1);
    }

    // b0 should be initialized to the depth of the stack.
    result.push(Assertion::single(B0_COL_IDX, 0, Felt::new(depth as u64)));

    // b1 should be initialized to the address of the last row in the overflow table, which is 0
    // when the overflow table is empty and -1 mod p when the stack is initialized with overflow.
    result.push(Assertion::single(B1_COL_IDX, 0, overflow_addr));
}

/// Returns the stack's boundary assertions for the main trace at the last step.
pub fn get_assertions_last_step(
    result: &mut Vec<Assertion<Felt>>,
    step: usize,
    stack_outputs: &[Felt],
) {
    // stack columns at the last step should be set to stack outputs, excluding overflow outputs
    for (i, &value) in stack_outputs.iter().take(MIN_STACK_DEPTH).enumerate() {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, step, value));
    }

    // if there are remaining slots on top of the stack without specified values, set them to ZERO.
    for i in stack_outputs.len()..MIN_STACK_DEPTH {
        result.push(Assertion::single(STACK_TRACE_OFFSET + i, step, Felt::ZERO));
    }
}

// --- AUXILIARY COLUMNS --------------------------------------------------------------------------

/// Returns the stack's boundary assertions for auxiliary columns at the first step.
pub fn get_aux_assertions_first_step<E: FieldElement>(
    result: &mut Vec<Assertion<E>>,
    alphas: &AuxTraceRandElements<E>,
    stack_inputs: &[Felt],
) where
    E: FieldElement<BaseField = Felt>,
{
    let step = 0;
    let value = if stack_inputs.len() > MIN_STACK_DEPTH {
        get_overflow_table_init(
            &stack_inputs[MIN_STACK_DEPTH..],
            alphas.get_segment_elements(0),
        )
    } else {
        E::ONE
    };

    result.push(Assertion::single(STACK_AUX_TRACE_OFFSET, step, value));
}

/// Returns the stack's boundary assertions for auxiliary columns at the last step.
pub fn get_aux_assertions_last_step<E: FieldElement>(_result: &mut [Assertion<E>], _step: usize) {}

// BOUNDARY CONSTRAINT HELPERS
// ================================================================================================

// --- AUX TRACE ----------------------------------------------------------------------------------

/// Gets the initial value of the overflow table auxiliary column  from the provided sets of initial
/// values and random elements.
fn get_overflow_table_init<E: FieldElement>(init_values: &[Felt], alphas: &[E]) -> E
where
    E: FieldElement<BaseField = Felt>,
{
    let mut value = E::ONE;
    let mut prev_clk = Felt::ZERO;
    let mut clk = Felt::from(Felt::MODULUS - init_values.len() as u64);

    // the values are in the overflow table in reverse order, since the deepest stack
    // value is added to the overflow table first.
    for &input in init_values.iter().rev() {
        value *= alphas[0]
            + alphas[1].mul_base(clk)
            + alphas[2].mul_base(input)
            + alphas[3].mul_base(prev_clk);
        prev_clk = clk;
        clk += Felt::ONE;
    }

    value
}
